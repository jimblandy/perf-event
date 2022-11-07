use perf_event::{events, Builder};
use std::fs::OpenOptions;
use std::io::Write;

// Need a function that will not be optimized away or inlined by the compiler.
#[inline(never)]
fn copy_and_fwrite(data: &[u8], file: &mut std::fs::File) -> std::io::Result<()> {
    // This guarantees that the memory within data is read so that we can test
    // read breakpoints.
    let copy = data.to_vec();

    file.write(&copy)?;
    Ok(())
}

#[test]
fn data() {
    let mut file = OpenOptions::new()
        .write(true)
        .open("/dev/null")
        .expect("Unable to open /dev/null for writing");
    let data = b"TEST DATA".to_vec();

    let mut counter = Builder::new()
        .kind(events::Breakpoint::read_write(
            data.as_ptr() as usize as _,
            1,
        ))
        .observe_self()
        .build()
        .expect("Unable to build performance counter");
    counter.enable().unwrap();

    for _ in 0..1000 {
        copy_and_fwrite(&data, &mut file).unwrap();
    }

    counter.disable().unwrap();
    assert_eq!(counter.read().unwrap(), 1000);
}

#[test]
fn execute() {
    let mut file = OpenOptions::new()
        .write(true)
        .open("/dev/null")
        .expect("Unable to open /dev/null for writing");
    let data = b"TEST DATA".to_vec();
    let fnptr = copy_and_fwrite as fn(_, _) -> _;

    let mut counter = Builder::new()
        .kind(events::Breakpoint::execute(fnptr as usize as u64))
        .observe_self()
        .build()
        .expect("Unable to build performance counter");
    counter.enable().unwrap();

    for _ in 0..1000 {
        copy_and_fwrite(&data, &mut file).unwrap();
    }

    counter.disable().unwrap();
    assert_eq!(counter.read().unwrap(), 1000);
}
