use super::Hex;
use nix::unistd::SysconfVar;
use perf_event::events::Software;
use perf_event::Builder;
use perf_event_open_sys::bindings;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MmapRecord {
    pub pid: u32,
    pub tid: u32,
    pub addr: u64,
    pub len: u64,
    pub pgoff: u64,
}

impl MmapRecord {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut buf = [0; std::mem::size_of::<Self>()];
        buf.copy_from_slice(bytes);
        unsafe { std::mem::transmute(buf) }
    }
}

#[test]
fn record_executable_mmap() {
    // Use the pagesize as the mmap size so that the recorded length in the
    // record and the length we allocate are the same.
    let pagesize = nix::unistd::sysconf(SysconfVar::PAGE_SIZE)
        .expect("Unable to get page size")
        .expect("No page size returned") as usize;

    let mut sampler = Builder::new(Software::DUMMY)
        .mmap(true)
        .build()
        .expect("Failed to build counter")
        .sampled(4096)
        .expect("Failed to build sampler");

    sampler.enable().expect("Failed to enable sampler");

    let mmap = memmap2::MmapOptions::new()
        .len(pagesize)
        .map_anon()
        .expect("Failed to create anonymous memory map");

    // This should cause the sampler to record a MMAP event
    let mmap = mmap
        .make_exec()
        .expect("Failed to transition memory mapping to be executable");

    sampler.disable().expect("Failed to disable sampler");

    let record = sampler
        .next_record()
        .expect("Sampler did not record any events");

    assert_eq!(record.ty(), bindings::PERF_RECORD_MMAP);

    let bytes = record.to_contiguous();
    let record = MmapRecord::from_bytes(&bytes[..std::mem::size_of::<MmapRecord>()]);

    eprintln!("record: {:#?}", record);

    assert_eq!(Hex(record.addr), Hex(mmap.as_ptr() as usize as _));
    assert_eq!(record.len, mmap.len() as _);
    assert_eq!(record.pid, nix::unistd::getpid().as_raw() as _);
    assert_eq!(record.tid, nix::unistd::gettid().as_raw() as _);
}
