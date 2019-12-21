//! A Rust API for Linux performance monitoring.
//!
//! This crate lets you access counters provided by the processor and kernel for
//! things like instruction completions, cache references and misses, branch
//! predictions and misses, and so on. The kernel also maintains counters for
//! its own internal events like context switches, page faults, etc.
//!
//! For example, the following code calls `println!`, and then prints hit ratio
//! of the level 1 cache:
//!
//!     use perf_event::{Builder, Group};
//!     use perf_event::events::{Cache, CacheOp, CacheResult, WhichCache};
//!
//!     fn main() -> std::io::Result<()> {
//!         const ACCESS: Cache = Cache {
//!             which: WhichCache::L1D,
//!             operation: CacheOp::READ,
//!             result: CacheResult::ACCESS,
//!         };
//!         const MISS: Cache = Cache { result: CacheResult::MISS, ..ACCESS };
//!
//!         let mut group = Group::new()?;
//!         let access_counter = Builder::new().group(&group).kind(ACCESS).build()?;
//!         let miss_counter = Builder::new().group(&group).kind(MISS).build()?;
//!
//!         let vec = (0..=51).collect::<Vec<_>>();
//!
//!         group.enable()?;
//!         println!("{:?}", vec);
//!         group.disable()?;
//!
//!         let counts = group.read()?;
//!         println!("L1D cache misses/references: {} / {} ({:.0}%)",
//!                  counts[&miss_counter],
//!                  counts[&access_counter],
//!                  (counts[&miss_counter] as f64 / counts[&access_counter] as f64) * 100.0);
//!
//!         println!("{:?}", counts);
//!
//!         Ok(())
//!     }



//! to help programmers assess the performance of their code.

//! To help programmers understand the behavior of their code, modern processors
//! have various counters built into the chip for events like 

use events::Event;
use libc::pid_t;
use perf_event_open_sys as sys;
use std::fs::File;
use std::io::{self, Read};
use std::os::raw::{c_int, c_ulong};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::sync::atomic::{AtomicUsize, Ordering};

pub mod events;

pub struct Counter {
    /// The file descriptor for this counter, returned by `perf_event_open`.
    ///
    /// When a `Counter` is dropped, this `File` is dropped, and the kernel
    /// removes the counter from any group it belongs to.
    file: File,

    /// The unique id assigned to this counter by the kernel.
    id: u64,
}

pub struct Builder<'a> {
    who: EventPid<'a>,
    cpu: Option<usize>,
    kind: Event,
    group: Option<&'a Group>,
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

pub struct Group {
    /// The file descriptor for this counter, returned by `perf_event_open`.
    /// This counter itself is for the dummy software event, so it's not
    /// interesting.
    file: File,

    /// The unique id assigned to this group by the kernel. We only use this for
    /// assertions.
    id: u64,

    /// An upper bound on the number of Counters in this group. This lets us
    /// allocate buffers of sufficient size for for PERF_FORMAT_GROUP reads.
    ///
    /// There's no way to ask the kernel how many members a group has. And if we
    /// pass a group read a buffer that's too small, the kernel won't just
    /// return a truncated result; it returns ENOSPC and leaves the buffer
    /// untouched. So the buffer just has to be large enough.
    ///
    /// Since we're borrowed while building group members, adding members can
    /// increment this counter. But it's harder to decrement it when a member
    /// gets dropped: we don't require that a Group outlive its members, so they
    /// can't necessarily update their `Group`'s count from a `Drop` impl. So we
    /// just increment, giving us an overestimate, and then correct the count
    /// when we actually do a read.
    max_members: AtomicUsize,
}

pub struct Counts {
    // Raw results from the `read`.
    data: Vec<u64>
}

impl<'a> EventPid<'a> {
    // Return the `pid` arg and the `flags` bits representing `self`.
    fn as_args(&self) -> (pid_t, u32) {
        match self {
            EventPid::ThisProcess => (0, 0),
            EventPid::Other(pid) => (*pid, 0),
            EventPid::CGroup(file) =>
                (file.as_raw_fd(), sys::bindings::PERF_FLAG_PID_CGROUP),
        }
    }
}

impl<'a> Default for Builder<'a> {
    fn default() -> Builder<'a> {
        Builder {
            who: EventPid::ThisProcess,
            cpu: None,
            kind: Event::Hardware(events::Hardware::INSTRUCTIONS),
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

    pub fn kind<K: Into<Event>>(mut self, kind: K) -> Builder<'a> {
        self.kind = kind.into();
        self
    }

    pub fn group(mut self, group: &'a Group) -> Builder<'a> {
        self.group = Some(group);
        self
    }

    pub fn build(self) -> std::io::Result<Counter> {
        let cpu = match self.cpu {
            Some(cpu) => cpu as c_int,
            None => -1,
        };
        let (pid, flags) = self.who.as_args();
        let group_fd = match self.group {
            Some(g) => {
                g.max_members.fetch_add(1, Ordering::SeqCst);
                g.file.as_raw_fd() as c_int
            }
            None => -1,
        };

        let mut attrs = sys::bindings::perf_event_attr::default();
        attrs.type_ = self.kind.as_type();
        attrs.size = std::mem::size_of::<sys::bindings::perf_event_attr>() as u32;
        attrs.config = self.kind.as_config();
        attrs.set_disabled(1);
        attrs.set_exclude_kernel(1);
        attrs.set_exclude_hv(1);

        let file = unsafe {
            File::from_raw_fd(check_syscall(|| {
                sys::perf_event_open(&mut attrs, pid, cpu, group_fd, flags as c_ulong)
            })?)
        };

        // If we're going to be part of a Group, retrieve the ID the kernel
        // assigned us, so we can find our results in a Counts structure. Even
        // if we're not part of a group, we'll use it in `Debug` output.
        let mut id = 0_64;
        check_syscall(|| unsafe {
            sys::ioctls::ID(file.as_raw_fd(), &mut id)
        })?;

        Ok(Counter { file, id, })
    }
}

impl Counter {
    pub fn id(&self) -> u64 {
        self.id
    }

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

impl std::fmt::Debug for Counter {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "Counter {{ fd: {}, id: {} }}",
               self.file.as_raw_fd(), self.id)
    }
}

impl Group {
    #[allow(unused_parens)]
    pub fn new() -> io::Result<Group> {
        // Open a placeholder perf counter that we can add other events to.
        let mut attrs = sys::bindings::perf_event_attr::default();
        attrs.type_ = sys::bindings::perf_type_id_PERF_TYPE_SOFTWARE;
        attrs.size = std::mem::size_of::<sys::bindings::perf_event_attr>() as u32;
        attrs.config = sys::bindings::perf_sw_ids_PERF_COUNT_SW_DUMMY as u64;
        attrs.set_disabled(1);
        attrs.set_exclude_kernel(1);
        attrs.set_exclude_hv(1);

        // Arrange to be able to identify the counters we read back.
        attrs.read_format = (sys::bindings::perf_event_read_format_PERF_FORMAT_ID |
                             sys::bindings::perf_event_read_format_PERF_FORMAT_GROUP) as u64;

        let file = unsafe {
            File::from_raw_fd(check_syscall(|| {
                sys::perf_event_open(&mut attrs, 0, -1, -1, 0)
            })?)
        };

        // Retrieve the ID the kernel assigned us.
        let mut id = 0_64;
        check_syscall(|| unsafe {
            sys::ioctls::ID(file.as_raw_fd(), &mut id)
        })?;

        let max_members = AtomicUsize::new(0);

        Ok(Group { file, id, max_members })
    }

    pub fn enable(&mut self) -> io::Result<()> {
        check_syscall(|| unsafe {
            sys::ioctls::ENABLE(self.file.as_raw_fd(),
                                sys::bindings::perf_event_ioc_flags_PERF_IOC_FLAG_GROUP)
        }).map(|_| ())
    }

    pub fn disable(&mut self) -> io::Result<()> {
        check_syscall(|| unsafe {
            sys::ioctls::DISABLE(self.file.as_raw_fd(),
                                 sys::bindings::perf_event_ioc_flags_PERF_IOC_FLAG_GROUP)
        }).map(|_| ())
    }

    pub fn read(&mut self) -> io::Result<Counts> {
        // Since we passed PERF_FORMAT_ID | PERF_FORMAT_GROUP, the data we'll
        // read has the form:
        //
        //     struct read_format {
        //         u64 nr;            /* The number of events */
        //         u64 time_enabled;  /* if PERF_FORMAT_TOTAL_TIME_ENABLED */
        //         u64 time_running;  /* if PERF_FORMAT_TOTAL_TIME_RUNNING */
        //         struct {
        //             u64 value;     /* The value of the event */
        //             u64 id;        /* if PERF_FORMAT_ID */
        //         } values[nr];
        //     };
        let mut data = vec![0_u64; 3 + 2 * self.max_members.load(Ordering::SeqCst)];
        self.file.read(u64::slice_as_bytes_mut(&mut data))?;

        // CountsIter assumes that the group's dummy count appears first.
        assert_eq!(data[2], self.id);

        Ok(Counts { data })
    }
}

impl std::fmt::Debug for Group {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "Group {{ fd: {}, id: {} }}",
               self.file.as_raw_fd(), self.id)
    }
}

impl Counts {
    fn len(&self) -> usize {
        self.data[0] as usize
    }

    fn nth_ref(&self, n: usize) -> (u64, &u64) {
        assert!(n < self.len());
        // (id, &value)
        (self.data[1 + 2 * n + 1],
         &self.data[1 + 2 * n])
    }
}

pub struct CountsIter<'c> {
    counts: &'c Counts,
    next: usize
}

impl<'c> Iterator for CountsIter<'c> {
    type Item = (u64, &'c u64);
    fn next(&mut self) -> Option<(u64, &'c u64)> {
        if self.next >= self.counts.len() {
            return None;
        }
        let result = self.counts.nth_ref(self.next);
        self.next += 1;
        return Some(result);
    }
}

impl<'c> IntoIterator for &'c Counts {
    type Item = (u64, &'c u64);
    type IntoIter = CountsIter<'c>;
    fn into_iter(self) -> CountsIter<'c> {
        CountsIter {
            counts: self,
            next: 1, // skip the `Group` itself, it's just a dummy.
        }
    }
}

impl Counts {
    pub fn get(&self, member: &Counter) -> Option<&u64> {
        self.into_iter()
            .find(|&(id, _)| id == member.id)
            .map(|(_, value)| value)
    }

    pub fn iter(&self) -> CountsIter {
        <&Counts as IntoIterator>::into_iter(self)
    }
}

impl std::ops::Index<&Counter> for Counts {
    type Output = u64;
    fn index(&self, index: &Counter) -> &u64 {
        self.get(index).unwrap()
    }
}

impl std::fmt::Debug for Counts {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_map().entries(self.into_iter()).finish()
    }
}

unsafe trait SliceAsBytesMut: Sized {
    fn slice_as_bytes_mut(slice: &mut [Self]) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(slice.as_mut_ptr() as *mut u8,
                                           std::mem::size_of_val(slice))
        }
    }
}

unsafe impl SliceAsBytesMut for u64 { }

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
