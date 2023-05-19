use perf_event_open_sys::bindings::perf_event_attr;
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::{fmt, io};

use crate::events::{Event, EventData};

// 0 will never be the PMU value for kprobe or uprobe.
// We use it as a flag value to indicate that this has not been initialized.
static KPROBE_TYPE: AtomicU32 = AtomicU32::new(0);
static UPROBE_TYPE: AtomicU32 = AtomicU32::new(0);

#[derive(Clone, Debug)]
enum ProbeTarget {
    Func { name: CString, offset: u64 },
    Addr(u64),
}

#[derive(Clone, Debug)]
struct Probe {
    ty: u32,
    retprobe: bool,
    target: ProbeTarget,
}

impl Probe {
    fn kprobe_type() -> io::Result<u32> {
        match KPROBE_TYPE.load(Ordering::Relaxed) {
            0 => {
                let text = std::fs::read_to_string("/sys/bus/event_source/devices/kprobe/type")?;
                let ty = text
                    .trim_end()
                    .parse()
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                KPROBE_TYPE.store(ty, Ordering::Relaxed);
                Ok(ty)
            }
            ty => Ok(ty),
        }
    }

    fn uprobe_type() -> io::Result<u32> {
        match UPROBE_TYPE.load(Ordering::Relaxed) {
            0 => {
                let text = std::fs::read_to_string("/sys/bus/event_source/devices/uprobe/type")?;
                let ty = text
                    .trim_end()
                    .parse()
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                UPROBE_TYPE.store(ty, Ordering::Relaxed);
                Ok(ty)
            }
            ty => Ok(ty),
        }
    }
}

impl Event for Probe {
    fn update_attrs(self, _: &mut perf_event_attr) {
        unimplemented!("probes require storing data within the Builder")
    }

    fn update_attrs_with_data(self, attr: &mut perf_event_attr) -> Option<Arc<dyn EventData>> {
        attr.type_ = self.ty;
        attr.config = self.retprobe.into();
        match self.target {
            ProbeTarget::Addr(addr) => {
                attr.__bindgen_anon_3.kprobe_func = 0;
                attr.__bindgen_anon_4.kprobe_addr = addr;
                None
            }
            ProbeTarget::Func { name, offset } => {
                attr.__bindgen_anon_3.kprobe_func = name.as_ptr() as usize as u64;
                attr.__bindgen_anon_4.probe_offset = offset;
                Some(Arc::new(name))
            }
        }
    }
}

/// Kernel-space probe event.
///
/// Kprobes allow you to dynamically insert breakpoints into kernel functions.
/// This can be used to count function executions or to attach eBPF programs
/// that run during those breakpoints.
///
/// There are two types of kprobes:
/// - [`kprobe`](KProbe::probe)s trigger when the relevant function is called
///   (or, potentially, executed at an offset within that function).
/// - [`kretprobe`](KProbe::retprobe)s trigger just before the relevant function
///   returns.
///
/// Kprobes can be create either for a named function or at a raw address in
/// kernel space.
///
/// The internal documentation on how kprobes work is available [here][kdoc].
///
/// [kdoc]: https://www.kernel.org/doc/Documentation/kprobes.txt
#[derive(Clone)]
pub struct KProbe(Probe);

impl KProbe {
    /// Create a kprobe or kretprobe for a named kernel function.
    ///
    /// # Errors
    /// This will attempt to read the kprobe PMU type from
    /// `/sys/bus/event_source`. It will return an error if the kprobe PMU is
    /// not available or the filesystem exposed by the kernel there is otherwise
    /// unparseable.
    pub fn for_function(retprobe: bool, func: CString, offset: u64) -> io::Result<Self> {
        Ok(Self(Probe {
            ty: Probe::kprobe_type()?,
            retprobe,
            target: ProbeTarget::Func { name: func, offset },
        }))
    }

    /// Create a kprobe or kretprobe for a kernel address.
    ///
    /// # Errors
    /// This will attempt to read the kprobe PMU type from
    /// `/sys/bus/event_source`. It will return an error if the kprobe PMU is
    /// not available or the filesystem exposed by the kernel there is otherwise
    /// unparseable.
    pub fn for_addr(retprobe: bool, addr: u64) -> io::Result<Self> {
        Ok(Self(Probe {
            ty: Probe::kprobe_type()?,
            retprobe,
            target: ProbeTarget::Addr(addr),
        }))
    }

    fn new_generic(retprobe: bool, func: impl AsRef<[u8]>, offset: u64) -> io::Result<Self> {
        let func = CString::new(func.as_ref())
            .expect("kprobe function target contained an internal nul byte");
        Self::for_function(retprobe, func, offset)
    }

    /// Create a kprobe on the given function at `offset`.
    ///
    /// # Errors
    /// This will attempt to read the kprobe PMU type from
    /// `/sys/bus/event_source`. It will return an error if the kprobe PMU is
    /// not available or the filesystem exposed by the kernel there is otherwise
    /// unparseable.
    ///
    /// # Panics
    /// Panics if `func` contains a nul byte other than at the very end.
    pub fn probe(func: impl AsRef<[u8]>, offset: u64) -> io::Result<Self> {
        Self::new_generic(false, func, offset)
    }

    /// Create a kretprobe on the given function at `offset`.
    ///
    /// # Errors
    /// This will attempt to read the kprobe PMU type from
    /// `/sys/bus/event_source`. It will return an error if the kprobe PMU is
    /// not available or the filesystem exposed by the kernel there is otherwise
    /// unparseable.
    ///
    /// # Panics
    /// Panics if `func` contains a nul byte other than at the very end.
    pub fn retprobe(func: impl AsRef<[u8]>, offset: u64) -> io::Result<Self> {
        Self::new_generic(true, func, offset)
    }
}

impl Event for KProbe {
    fn update_attrs(self, attr: &mut perf_event_attr) {
        self.0.update_attrs(attr);
    }

    fn update_attrs_with_data(self, attr: &mut perf_event_attr) -> Option<Arc<dyn EventData>> {
        self.0.update_attrs_with_data(attr)
    }
}

impl fmt::Debug for KProbe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dbg = f.debug_struct("KProbe");
        dbg.field("type", &self.0.ty);
        dbg.field("retprobe", &self.0.retprobe);

        match &self.0.target {
            ProbeTarget::Addr(addr) => dbg.field("addr", addr),
            ProbeTarget::Func { name, offset } => dbg.field("func", name).field("offset", offset),
        };

        dbg.finish()
    }
}

/// User-space probe event.
///
/// Uprobes allow you to dynamically insert tracepoints within user-space
/// processes. This allows you to gather metrics on how many times a function
/// is called (e.g. malloc) or attach eBPF programs to run when the tracepoint
/// is triggered.
///
/// There are two types of kprobes:
/// - [`uprobe`](UProbe::probe)s trigger when the relevant function is called
///   (or, potentially, executed at an offset within that function).
/// - [`uretprobe`](UProbe::retprobe)s trigger just before the relevant function
///   returns.
///
/// To create a uprobe you will need to provide both a path to a binary and
/// the offset within that binary at which you want to insert the probe.
/// Discovering the offset that corresponds to a given function is up to you.
#[derive(Clone)]
pub struct UProbe(Probe);

impl UProbe {
    /// Create a new uprobe from a path string and offset.
    ///
    /// # Errors
    /// This will attempt to read the kprobe PMU type from
    /// `/sys/bus/event_source`. It will return an error if the uprobe PMU is
    /// not available or the filesystem exposed by the kernel there is otherwise
    /// unparseable.
    pub fn new(retprobe: bool, path: CString, offset: u64) -> io::Result<Self> {
        Ok(Self(Probe {
            ty: Probe::uprobe_type()?,
            retprobe,
            target: ProbeTarget::Func { name: path, offset },
        }))
    }

    fn new_generic(retprobe: bool, path: impl AsRef<Path>, offset: u64) -> io::Result<Self> {
        let path = CString::new(path.as_ref().as_os_str().as_bytes())
            .expect("uprobe path contained an internal nul byte");
        Self::new(retprobe, path, offset)
    }

    /// Create a new uprobe from a path and an offset within that file.
    ///
    /// # Errors
    /// This will attempt to read the kprobe PMU type from
    /// `/sys/bus/event_source`. It will return an error if the uprobe PMU is
    /// not available or the filesystem exposed by the kernel there is otherwise
    /// unparseable.
    pub fn probe(path: impl AsRef<Path>, offset: u64) -> io::Result<Self> {
        Self::new_generic(false, path, offset)
    }

    /// Create a new uretprobe from a path and an offset within that file.
    ///
    /// # Errors
    /// This will attempt to read the kprobe PMU type from
    /// `/sys/bus/event_source`. It will return an error if the uprobe PMU is
    /// not available or the filesystem exposed by the kernel there is otherwise
    /// unparseable.
    pub fn retprobe(path: impl AsRef<Path>, offset: u64) -> io::Result<Self> {
        Self::new_generic(true, path, offset)
    }
}

impl fmt::Debug for UProbe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dbg = f.debug_struct("UProbe");
        dbg.field("type", &self.0.ty);
        dbg.field("retprobe", &self.0.retprobe);

        match &self.0.target {
            ProbeTarget::Addr(addr) => dbg.field("addr", addr),
            ProbeTarget::Func { name, offset } => dbg.field("path", name).field("offset", offset),
        };

        dbg.finish()
    }
}
