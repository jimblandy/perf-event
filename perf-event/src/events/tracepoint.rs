use std::num::ParseIntError;
use std::path::{Path, PathBuf};
use std::{fmt, io};

use perf_event_open_sys::bindings;

use crate::events::Event;

/// Kernel tracepoint event.
///
/// Tracepoints allow you to dynamically insert breakpoints into specific hook
/// points defined by the kernel. These can be used to count function executions
/// or to attach eBPF programs that run during those breakpoints.
///
/// Tracepoints are similar to kprobes. The difference, however, is that
/// tracepoints are stable and documented. On the other hand, kprobes can be
/// inserted (almost) anywhere within the kernel whereas tracepoints are
/// restricted to only the locations at which they have been defined.
///
/// Note that it is possible to create tracepoints from kprobes by using
/// [`perf probe`].
///
/// [`perf probe`]: https://man7.org/linux/man-pages/man1/perf-probe.1.html
#[derive(Clone, Copy, Debug)]
pub struct Tracepoint {
    id: u64,
}

impl Tracepoint {
    /// Create a tracepoint directly from its raw ID.
    ///
    /// Usually you will have to look within debugfs to get this ID.
    /// [`with_name`](Tracepoint::with_name) is a helper to do this by looking
    /// up the event ID in the debugfs instance mounted at `/sys/kernel/debug`.
    pub fn with_id(id: u64) -> Self {
        Self { id }
    }

    /// Create a tracepoint by looking up its ID within `/sys/kernel/debug`.
    ///
    /// Event names are listed under `/sys/kernel/debug/tracing/events`. All
    /// this method does is read the file at
    /// `/sys/kernel/debug/tracing/events/<name>/id` and use the contents of the
    /// `id` file as the tracepoint id.
    ///
    /// Note that `/sys/kernel/debug` is only accessible if running as root or
    /// if the process has `CAP_SYS_ADMIN`.
    ///
    /// # Example
    /// Create a tracepoint event for the `sched_switch` tracepoint.
    /// ```
    /// # fn run() -> std::io::Result<()> {
    /// let tracepoint = Tracepoint::with_name("sched/sched_switch")?;
    /// # Ok(())
    /// # }
    /// # let _ = run();
    /// ```
    pub fn with_name(name: impl AsRef<Path>) -> io::Result<Self> {
        let mut path = PathBuf::from("/sys/kernel/debug/tracing/events");
        path.push(name.as_ref());
        path.push("id");

        let id = std::fs::read_to_string(&path)?
            .trim_end()
            .parse()
            .map_err(move |e| {
                io::Error::new(io::ErrorKind::Other, UnparseableIdFile::new(path, e))
            })?;

        Ok(Self::with_id(id))
    }

    /// Get the id of this tracepoint.
    pub fn id(&self) -> u64 {
        self.id
    }
}

impl Event for Tracepoint {
    fn update_attrs(self, attr: &mut bindings::perf_event_attr) {
        attr.type_ = bindings::PERF_TYPE_TRACEPOINT;
        attr.config = self.id;
    }
}

#[derive(Debug)]
struct UnparseableIdFile {
    path: PathBuf,
    source: ParseIntError,
}

impl UnparseableIdFile {
    fn new(path: PathBuf, source: ParseIntError) -> Self {
        Self { path, source }
    }
}

impl fmt::Display for UnparseableIdFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "unparseable tracepoint id file `{}`",
            self.path.display()
        ))
    }
}

impl std::error::Error for UnparseableIdFile {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        Some(&self.source)
    }
}
