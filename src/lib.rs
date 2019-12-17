//! A Rust API for Linux performance monitoring

use bindings::__u32;
use event_kind::EventKind;
use libc::pid_t;
use std::fs::File;
use std::io::{self, Read};
use std::os::raw::{c_int, c_ulong};
use std::os::unix::io::AsRawFd;

mod bindings;
pub mod event_kind;
mod syscalls;

pub struct Event {
    file: File,
}

pub struct Builder {
    who: EventPid,
    cpu: Option<usize>,
    kind: EventKind,
}

#[derive(Debug)]
pub enum EventPid {
    /// Monitor the calling process.
    ThisProcess,

    /// Monitor the given pid.
    Other(pid_t),

    /// Monitor members of the given cgroup.
    CGroup(File),
}

impl EventPid {
    // Return the `pid` arg and the `flags` bits representing `self`.
    fn as_args(&self) -> (bindings::__kernel_pid_t, u32) {
        match self {
            EventPid::ThisProcess => (0, bindings::PERF_FLAG_FD_NO_GROUP),
            EventPid::Other(pid) => (*pid, bindings::PERF_FLAG_FD_NO_GROUP),
            EventPid::CGroup(file) =>
                (file.as_raw_fd(), 0),
        }
    }
}

impl Default for Builder {
    fn default() -> Builder {
        Builder {
            who: EventPid::ThisProcess,
            cpu: None,
            kind: EventKind::Hardware(event_kind::Hardware::INSTRUCTIONS),
        }
    }
}

impl Builder {
    pub fn new() -> Builder {
        Builder::default()
    }

    pub fn observe_self(mut self) -> Builder {
        self.who = EventPid::ThisProcess;
        self
    }

    pub fn observe_pid(mut self, pid: pid_t) -> Builder {
        self.who = EventPid::Other(pid);
        self
    }

    // Ugly that this takes `cgroup` by value...
    pub fn observe_cgroup(mut self, cgroup: File) -> Builder {
        self.who = EventPid::CGroup(cgroup);
        self
    }

    pub fn one_cpu(mut self, cpu: usize) -> Builder {
        self.cpu = Some(cpu);
        self
    }


    pub fn any_cpu(mut self) -> Builder {
        self.cpu = None;
        self
    }

    pub fn kind<K: Into<EventKind>>(mut self, kind: K) -> Builder {
        self.kind = kind.into();
        self
    }

    pub fn build(self) -> std::io::Result<Event> {
        let mut attrs = bindings::perf_event_attr::default();

        attrs.type_ = self.kind.as_type();
        attrs.size = std::mem::size_of::<bindings::perf_event_attr>() as __u32;
        attrs.config = self.kind.as_config();
        attrs.set_disabled(1);
        attrs.set_exclude_kernel(1);
        attrs.set_exclude_hv(1);

        let (pid, flags) = self.who.as_args();

        Ok(Event {
            file: syscalls::perf_event_open(&attrs,
                                            pid,
                                            self.cpu.map(|u| u as c_int).unwrap_or(-1 as c_int),
                                            -1,
                                            flags as c_ulong)?,
        })
    }
}

impl Event {
    pub fn enable(&mut self) -> io::Result<()> {
        unsafe {
            syscalls::ioctls::ENABLE(&self.file, 0).map(|_| ())
        }
    }

    pub fn disable(&mut self) -> io::Result<()> {
        unsafe {
            syscalls::ioctls::DISABLE(&self.file, 0).map(|_| ())
        }
    }

    pub fn read(&mut self) -> io::Result<u64> {
        let mut buf = [0_u8; 8];
        self.file.read_exact(&mut buf)?;
        Ok(u64::from_ne_bytes(buf))
    }
}

#[test]
fn simple_build() {
    Builder::new().build().expect("Couldn't build default Event");
}
