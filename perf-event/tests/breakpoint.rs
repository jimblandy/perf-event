use perf_event::{events, Builder};

// Need a function that will not be optimized away or inlined by the compiler.
#[inline(never)]
fn use_data(data: &[u8]) {
    for byte in data {
        // Use a volatile read here to ensure that the resulting program
        // actually does the read from data and it doesn't get optimized away.
        unsafe { std::ptr::read_volatile(byte) };
    }
}

#[test]
fn data() {
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
        use_data(&data);
    }

    counter.disable().unwrap();
    assert_eq!(counter.read().unwrap(), 1000);
}

#[test]
fn execute() {
    let data = b"TEST DATA".to_vec();
    let fnptr = use_data as fn(_) -> _;

    let mut counter = Builder::new()
        .kind(events::Breakpoint::execute(fnptr as usize as u64))
        .observe_self()
        .build()
        .expect("Unable to build performance counter");
    counter.enable().unwrap();

    for _ in 0..1000 {
        use_data(&data);
    }

    counter.disable().unwrap();
    assert_eq!(counter.read().unwrap(), 1000);
}
