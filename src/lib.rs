//! A Rust API for Linux performance monitoring

use event_kind::EventKind;
use libc::pid_t;
use perf_event_open_sys as sys;
use std::fs::File;
use std::io::{self, Read};
use std::os::raw::{c_int, c_ulong};
use std::os::unix::io::{AsRawFd, FromRawFd};

pub mod event_kind;

pub struct Counter {
    file: File,
}

pub struct Builder<'a> {
    who: EventPid<'a>,
    cpu: Option<usize>,
    kind: EventKind,
    group: Option<&'a Counter>,
}

#[derive(Debug)]
pub enum EventPid<'a> {
    /// Monitor the calling process.
    ThisProcess,

    /// Monitor the given pid.
    Other(pid_t),

    /// Monitor members of the given cgroup.
    CGroup(&'a File),
}

impl<'a> EventPid<'a> {
    // Return the `pid` arg and the `flags` bits representing `self`.
    fn as_args(&self) -> (pid_t, u32) {
        match self {
            EventPid::ThisProcess => (0, sys::bindings::PERF_FLAG_FD_NO_GROUP),
            EventPid::Other(pid) => (*pid, sys::bindings::PERF_FLAG_FD_NO_GROUP),
            EventPid::CGroup(file) =>
                (file.as_raw_fd(), 0),
        }
    }
}

impl<'a> Default for Builder<'a> {
    fn default() -> Builder<'a> {
        Builder {
            who: EventPid::ThisProcess,
            cpu: None,
            kind: EventKind::Hardware(event_kind::Hardware::INSTRUCTIONS),
            group: None,
        }
    }
}

impl<'a> Builder<'a> {
    pub fn new() -> Builder<'a> {
        Builder::default()
    }

    pub fn observe_self(mut self) -> Builder<'a> {
        self.who = EventPid::ThisProcess;
        self
    }

    pub fn observe_pid(mut self, pid: pid_t) -> Builder<'a> {
        self.who = EventPid::Other(pid);
        self
    }

    // Ugly that this takes `cgroup` by value...
    pub fn observe_cgroup(mut self, cgroup: &'a File) -> Builder<'a> {
        self.who = EventPid::CGroup(cgroup);
        self
    }

    pub fn one_cpu(mut self, cpu: usize) -> Builder<'a> {
        self.cpu = Some(cpu);
        self
    }


    pub fn any_cpu(mut self) -> Builder<'a> {
        self.cpu = None;
        self
    }

    pub fn kind<K: Into<EventKind>>(mut self, kind: K) -> Builder<'a> {
        self.kind = kind.into();
        self
    }

    pub fn event_group(mut self, event: &'a Counter) -> Builder<'a> {
        self.group = Some(event);
        self
    }

    pub fn build(self) -> std::io::Result<Counter> {
        let mut attrs = sys::bindings::perf_event_attr::default();

        let cpu = self.cpu;
        let (pid, flags) = self.who.as_args();
        let group_fd = self.group.map(|e| e.file.as_raw_fd() as c_int).unwrap_or(-1);

        attrs.type_ = self.kind.as_type();
        attrs.size = std::mem::size_of::<sys::bindings::perf_event_attr>() as u32;
        attrs.config = self.kind.as_config();
        attrs.set_disabled(1);
        attrs.set_exclude_kernel(1);
        attrs.set_exclude_hv(1);

        let fd = check_syscall(|| unsafe {
            sys::perf_event_open(&mut attrs,
                                 pid,
                                 cpu.map(|u| u as c_int).unwrap_or(-1 as c_int),
                                 group_fd,
                                 flags as c_ulong)
        })?;

        Ok(Counter {
            file: unsafe {
                File::from_raw_fd(fd)
            }
        })
    }
}

impl Counter {
    pub fn enable(&mut self) -> io::Result<()> {
        check_syscall(|| unsafe {
            sys::ioctls::ENABLE(self.file.as_raw_fd(), 0)
        }).map(|_| ())
    }

    pub fn disable(&mut self) -> io::Result<()> {
        check_syscall(|| unsafe {
            sys::ioctls::DISABLE(self.file.as_raw_fd(), 0)
        }).map(|_| ())
    }

    pub fn read(&mut self) -> io::Result<u64> {
        let mut buf = [0_u8; 8];
        self.file.read_exact(&mut buf)?;
        Ok(u64::from_ne_bytes(buf))
    }
}

fn check_syscall<F, R>(f: F) -> io::Result<R>
where F: FnOnce() -> R,
      R: PartialOrd + Default
{
    let result = f();
    if result < R::default() {
        Err(io::Error::last_os_error())
    } else {
        Ok(result)
    }
}

#[test]
fn simple_build() {
    Builder::new().build().expect("Couldn't build default Counter");
}
