use super::Hex;
use nix::unistd::SysconfVar;
use perf_event::events::Software;
use perf_event::samples::RecordEvent;
use perf_event::Builder;

#[test]
fn record_executable_mmap() {
    // Use the pagesize as the mmap size so that the recorded length in the
    // record and the length we allocate are the same.
    let pagesize = nix::unistd::sysconf(SysconfVar::PAGE_SIZE)
        .expect("Unable to get page size")
        .expect("No page size returned") as usize;

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
    assert_eq!(record.len, mmap.len() as _);
    assert_eq!(record.pid, nix::unistd::getpid().as_raw() as _);
    assert_eq!(record.tid, nix::unistd::gettid().as_raw() as _);
}
