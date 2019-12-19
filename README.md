## perf-event: a Rust interface to Linux performance monitoring

*This is a nascent project. Tests and docs are lacking. The design may change.*

This uses the Linux `perf_event_open` API to access performance monitoring
hardware and software. Use `Builder` to create a perf event counter, then use
`enable` and `disable` to start and stop counting. Call `read` to get your
count.

For example, this counts the number of cycles used by the call to `println!`.
Try adjusting the length of the vector to see the cycle count change.

    fn main() -> std::io::Result<()> {
        let mut counter = Builder::new().build()?;

        let vec = (0..=51).collect::<Vec<_>>();

        counter.enable()?;
        println!("{:?}", vec);
        counter.disable()?;

        println!("{} instructions retired", counter.read()?);

        Ok(())
    }

At present, `Builder` measures the `PERF_COUNT_HW_INSTRUCTIONS` counter. Its
description from the `perf_event_open(2)` man page:

> Retired instructions. Be careful, these can be affected by various issues,
> most notably hardware interrupt counts.

The `examples` directory includes programs that count other sorts of events.
