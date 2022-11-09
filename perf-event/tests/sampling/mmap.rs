use perf_event::events::Software;
use perf_event::samples::RecordEvent;
use perf_event::Builder;
use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Hex<T>(T);

impl<T: fmt::UpperHex> fmt::Display for Hex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: fmt::UpperHex> fmt::Debug for Hex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("0x")?;
        self.0.fmt(f)
    }
}

#[test]
fn record_executable_mmap() {
    // Use the pagesize as the mmap size so that the recorded length in the
    // record and the length we allocate are the same.
    let pagesize = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
    assert!(pagesize >= 0);
    let pagesize = pagesize as usize;

    let mut sampler = Builder::new()
        .kind(Software::DUMMY)
        .mmap(true)
        .build_sampler(4096)
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

    let record = sampler.next().expect("Sampler did not record any events");
    let record = match record.event {
        RecordEvent::Mmap(mmap) => mmap,
        _ => panic!("expected a MMAP record, got {:?} instead", record.ty),
    };

    eprintln!("record: {:#?}", record);

    assert_eq!(Hex(record.addr), Hex(mmap.as_ptr() as usize as _));
    assert_eq!(Hex(record.len), Hex(mmap.len() as _));
    assert_eq!(Hex(record.pid), Hex(unsafe { libc::getpid() as _ }));
    assert_eq!(Hex(record.tid), Hex(unsafe { libc::gettid() as _ }));
}
