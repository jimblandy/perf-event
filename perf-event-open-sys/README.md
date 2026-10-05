## Direct, unsafe Rust bindings for Linux's `perf_event_open` system call

This crate exports `unsafe` Rust wrappers for Linux system calls for accessing
performance monitoring counters and tracing facilities. This includes:

- the processor's own performance monitoring registers
- kernel counters for things like context switches and page faults
- kernel tracepoints, kprobe, and uprobes
- processor tracing facilities like Intel's Branch Trace Store (BTS)
- hardware breakpoints

Specifically, this crate provides:

- a Rust wrapper the Linux `perf_event_open` system call

- Rust wrappers for the ioctls you can apply to a file descriptor returned by
  `perf_event_open`

- bindings for `perf_event_open`'s associated header files, generated from the C
  headers by `bindgen`

All functions are direct, `unsafe` wrappers for the underlying calls. They
operate on raw pointers and raw file descriptors.

For a type-safe API for basic functionality, see the [perf-event] crate.

[perf-event]: https://crates.io/crates/perf-event

### Supported architectures

Almost all of the `perf_event_open` API is independent of the program's
instruction set, but there are a few details that vary from one architecture to
another, like the format in which CPU register values are saved when the
`PERF_SAMPLE_REGS_USER` sample type is selected. (This is actually the only
thing we are aware of that varies, aside from the layout of bitfields, which
differs on big-endian and little-endian machines.)

The exact API this crate offers depends on rustc's [`target_arch`] configuration
option. The following architectures are supported:

- `target_arch = "aarch64"` (Linux kernel: `arm64`)
- `target_arch = "x86"` and `target_arch = "x86_64"` (Linux kernel: `x86_64`)
- `target_arch = "riscv64"` (Linux kernel: `riscv`)
- `target_arch = "powerpc64"` (Linux kernel: `powerpc`)
- `target_arch = "loongarch64` (Linux kernel: `loongarch`)

[`target_arch`]: https://doc.rust-lang.org/reference/conditional-compilation.html#r-cfg.target_arch

### Using perf types on other platforms

Even though Windows and Mac don't have the `perf_event_open` system
call, the `perf_event_open_sys` crate still builds on those platforms:
the type definitions in the `bindings` module can be useful for code
that needs to parse perf-related data produced on Linux or Android
systems. The syscall and ioctl wrapper functions are not available.

### Updating the System Call Bindings

The `bindings` module defines Rust equivalents for the types and constants used
by the Linux `perf_event_open` system call and its related ioctls. These are
generated automatically from the kernel's C header files, using [bindgen]. Both
the interface and the underlying functionality are quite complex, and new
features are added at a steady pace.

To update the generated bindings, consult the checklist in `../checklists.org`.

### Deciding when a major release is necessary.

A release that introduces [non-semver-compatible][compat] changes to
the crate's API must assign the crate a new major version number.

You can use [`cargo semver-checks`] to check whether the resulting changes are
semver-compatible. We don't always literally follow the standards set by
cargo-semver-checks but it is generally a good starting point for determining
whether the change is a breaking one. Changes we would tolerate:

- Changes to the signatures of `new_bitfield_N` functions. These
  take an argument for every bitfield in the container member they
  construct, so they are affected any time a bitfield is added.
  These functions are unweildy and rarely used.

Beware! `cargo semver-checks` does not detect all breaking changes!
For example, it did not report any incompatibility when [#79] changed
the type of `perf_event_open_sys::ioctl::PERIOD`'s second argument
from `u64` to `*const u64`. This is a known limitation in `cargo
semver-checks`; see [cargo-semver-checks#637].

[bindgen]: https://crates.io/crates/bindgen
[cargo semver-checks]: https://github.com/obi1kenobi/cargo-semver-checks
[compat]: https://doc.rust-lang.org/cargo/reference/semver.html
[#79]: https://github.com/jimblandy/perf-event/pull/79
[cargo-semver-checks#637]: https://github.com/obi1kenobi/cargo-semver-checks/issues/637
