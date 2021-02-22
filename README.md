## perf-event: a Rust interface to Linux performance monitoring

*This is a nascent project. Tests are lacking. The design may change.*

This uses the Linux [`perf_event_open`][man] API to access performance monitoring
hardware and software. Use `Builder` to create a perf event counter, then use
`enable` and `disable` to start and stop counting. Call `read` to get your
count.

For example, this counts the number of cycles used by the call to `println!`.
Try adjusting the length of the vector to see the cycle count change.

    use perf_event::Builder;

    fn main() -> std::io::Result<()> {
        let mut counter = Builder::new().build()?;

        let vec = (0..=51).collect::<Vec<_>>();

        counter.enable()?;
        println!("{:?}", vec);
        counter.disable()?;

        println!("{} instructions retired", counter.read()?);

        Ok(())
    }

Since we don't specify what sort of event we want to count, `Builder` defaults
to `PERF_COUNT_HW_INSTRUCTIONS` events, whose documentation says:

> Retired instructions. Be careful, these can be affected by various issues,
> most notably hardware interrupt counts.

The `examples` directory includes programs that count other sorts of events.

[man]: http://man7.org/linux/man-pages/man2/perf_event_open.2.html

## See also

The [`perfcnt`] crate provides more extensive coverage of the Linux
`perf_event_open` API than this crate. However, `perfcnt` does not build on
stable Rust.

[`perfcnt`]: https://crates.io/crates/perfcnt
