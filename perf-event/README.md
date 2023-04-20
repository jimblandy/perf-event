## perf-event: a Rust interface to Linux performance monitoring

This crate is a wrapper around the Linux [`perf_event_open`][man] API. It
allows you to access a wide variety of the performance monitoring counters that
are available in Linux.

> This crate is a fork of Jim Blandy's [perf-event crate][jb-perf-event] that
> has been updated to support more features of the [`perf_event_open`][man]
> API.

[man]: http://man7.org/linux/man-pages/man2/perf_event_open.2.html
[jb-perf-event]: https://github.com/jimblandy/perf-event

## Getting Started

Add the following to your `Cargo.toml`
```toml
perf-event2 = "0.5"
```

Use `Builder` to create a perf counter, then use `enable` and `disable` to stop
and start countinng. Call `read` to get your count. If you need to use multiple
counters at once you can use `Group`. If you want to sample events from the
kernel, check out `Sampler`.

## Examples

For example, this counts the number of cycles used by the call to `println!`.
Try adjusting the length of the vector to see the cycle count change.

```rust
use perf_event::Builder;
use perf_event::events::Hardware;

fn main() -> std::io::Result<()> {
    let mut counter = Builder::new(Hardware::INSTRUCTIONS).build()?;

    let vec = (0..=51).collect::<Vec<_>>();

    counter.enable()?;
    println!("{:?}", vec);
    counter.disable()?;

    println!("{} instructions retired", counter.read()?);

    Ok(())
}
```

The `examples` directory includes programs that count other sorts of events.

## Differences between perf-event2 and perf-event
`perf-event2` supports all the same features that `perf-event` does but it also
supports the following:
- All counter options introduced up to linux kernel 6.0.
- Creating a `Group` to monitor anything other than all threads in the current
  process.
- Sampled events via the kernel (e.g. gathering stack traces, etc.). Parsing
  the records emitted by the kernel is still not supported by this crate.
- Direct access to the underlying `perf_event_attr` struct exposed by the
  kernel.

## Migrating From perf-event
`perf-event2` v0.4.8 is exactly the same as `perf-event` v0.4.8. You should be
to just replace `perf-event -> perf_event2` in your `Cargo.toml` and continue
going with no changes. To get the new features, however, you will need to
upgrade to v0.5.

The main change to be aware of is that `Builder::new` now takes an event
directly. Where you would previously do this
```rust,ignore
let counter = Builder::new()
    .kind(some_event)
    // ...
    .build()?
```
you will now need to do this instead
```rust,ignore
let counter = Builder::new(some_event)
    // ...
    .build()?;
```
Note that if you didn't call `kind` the default event type was `Hardware::INSTRUCTIONS`.

## See also

- The original [`perf-event`][jb-perf-event] crate is still perfectly workable
  depending on your use case.
- Markus Stange's [`linux-perf-event-reader`][lper] allows you to parse records
  emitted by perf and should allow you to parse those emitted directly by the
  kernel as well.
- The [`perfcnt`] crate provides similar coverage of the `perf_event_open` API and
  appears to support parsing samples emitted by the kernel as well.
- The [`not-perf`] project is a rewrite of `perf` in Rust, and has a bunch of
  code for dealing with the Linux perf API.

[`perfcnt`]: https://crates.io/crates/perfcnt
[lper]: https://crates.io/crates/linux-perf-event-reader
[`not-perf`]: https://github.com/koute/not-perf

