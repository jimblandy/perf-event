//! Samples that the kernel can generate.
//!
//! This module contains bindings for samples emitted by the kernel into the
//! ringbuffer generated by `perf_event_open`. For authoritative documentation
//! on what each record means see the [`perf_event_open` manpage][man].
//!
//! The main type that you will need to use is the [`Record`] struct. It is
//! created by [`Sampler::next`] and represents a single event as generated by
//! the kernel.
//!
//! [`Sampler::next`]: crate::Sampler::next
//! [man]: https://man7.org/linux/man-pages/man2/perf_event_open.2.html

use bitflags::bitflags;
use bytes::Buf;
use perf_event_open_sys::bindings::{self, perf_event_attr, perf_event_header};
use std::fmt;

mod mmap;

pub use self::bitflags_defs::{RecordMiscFlags, SampleType};
pub use self::mmap::Mmap;

// Need a module here to avoid the allow applying to everything.
#[allow(missing_docs)]
mod bitflags_defs {
    use super::*;

    bitflags::bitflags! {
        /// Specifies which fields to include in the sample.
        ///
        /// These values correspond to `PERF_SAMPLE_x` values. See the
        /// [manpage] for documentation on what they mean.
        ///
        /// [`Sampler`]: crate::Sampler
        /// [manpage]: http://man7.org/linux/man-pages/man2/perf_event_open.2.html
        #[derive(Default)]
        pub struct SampleType : u64 {
            const IP = bindings::PERF_SAMPLE_IP;
            const TID = bindings::PERF_SAMPLE_TID;
            const TIME = bindings::PERF_SAMPLE_TIME;
            const ADDR = bindings::PERF_SAMPLE_ADDR;
            const READ = bindings::PERF_SAMPLE_READ;
            const CALLCHAIN = bindings::PERF_SAMPLE_CALLCHAIN;
            const ID = bindings::PERF_SAMPLE_ID;
            const CPU = bindings::PERF_SAMPLE_CPU;
            const PERIOD = bindings::PERF_SAMPLE_PERIOD;
            const STREAM_ID = bindings::PERF_SAMPLE_STREAM_ID;
            const RAW = bindings::PERF_SAMPLE_RAW;
            const BRANCH_STACK = bindings::PERF_SAMPLE_BRANCH_STACK;
            const REGS_USER = bindings::PERF_SAMPLE_REGS_USER;
            const STACK_USER = bindings::PERF_SAMPLE_STACK_USER;
            const WEIGHT = bindings::PERF_SAMPLE_WEIGHT;
            const DATA_SRC = bindings::PERF_SAMPLE_DATA_SRC;
            const IDENTIFIER = bindings::PERF_SAMPLE_IDENTIFIER;
            const TRANSACTION = bindings::PERF_SAMPLE_TRANSACTION;
            const REGS_INTR = bindings::PERF_SAMPLE_REGS_INTR;
            const PHYS_ADDR = bindings::PERF_SAMPLE_PHYS_ADDR;
            const CGROUP = bindings::PERF_SAMPLE_CGROUP;

            // Don't clobber unknown flags when constructing the bitflag struct.
            #[doc(hidden)]
            const _ALLOW_ALL_FLAGS = !0;
        }
    }

    bitflags! {
        /// Additional flags about the record event.
        ///
        /// Not all of these apply for every record type and in certain cases the
        /// same bit is reused to mean different things for different record types.
        ///
        /// See the [manpage] for documentation on what each flag means.
        ///
        /// [manpage]: http://man7.org/linux/man-pages/man2/perf_event_open.2.html
        #[derive(Default)]
        pub struct RecordMiscFlags : u16 {
            /// The first few bytes of these flags actually contain an enum value.
            ///
            /// Use [`cpumode`](Self::cpumode) to access them.
            const CPUMODE_MASK = bindings::PERF_RECORD_MISC_CPUMODE_MASK as _;

            /// Indicates that the associated [`Mmap`] or [`Mmap2`] record is
            /// for a non-executable memory mapping.
            const MMAP_DATA = bindings::PERF_RECORD_MISC_MMAP_DATA as _;

            /// Indicates that the [`Comm`] record is due to an `exec` syscall.
            const COMM_EXEC = bindings::PERF_RECORD_MISC_COMM_EXEC as _;

            /// Indicates that the context switch event was away from the
            /// process ID contained within the sample.
            const SWITCH_OUT = bindings::PERF_RECORD_MISC_SWITCH_OUT as _;

            /// Indicates that the contents of [`Sample::ip`] points to the
            /// exact instruction that generated the event.
            const EXACT_IP = bindings::PERF_RECORD_MISC_EXACT_IP as _;

            const EXT_RESERVED = bindings::PERF_RECORD_MISC_EXT_RESERVED as _;

            // New flags will likely be added to the perf_event_open interface in
            // the future. In that case we would like to avoid deleting those flags.
            // This field will ensure that the bitflags crate does not truncate any
            // flags when we construct a RecordMiscFlags instance.
            #[doc(hidden)]
            const _ALLOW_ALL_FLAGS = u16::MAX;
        }
    }

    impl SampleType {
        /// Create a sample from the underlying bits.
        pub const fn new(bits: u64) -> Self {
            Self { bits }
        }
    }

    impl RecordMiscFlags {
        /// Create a set of flags from the underlying bits.
        pub const fn new(bits: u16) -> Self {
            Self { bits }
        }
    }
}

/// The type of the record as communicated by the kernel.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct RecordType(pub u32);

// Note: If you add a new value here make sure to also add it to the debug impl.
#[allow(missing_docs)]
impl RecordType {
    pub const MMAP: Self = Self(bindings::PERF_RECORD_MMAP);
    pub const LOST: Self = Self(bindings::PERF_RECORD_LOST);
    pub const COMM: Self = Self(bindings::PERF_RECORD_COMM);
    pub const EXIT: Self = Self(bindings::PERF_RECORD_EXIT);
    pub const THROTTLE: Self = Self(bindings::PERF_RECORD_THROTTLE);
    pub const UNTHROTTLE: Self = Self(bindings::PERF_RECORD_UNTHROTTLE);
    pub const FORK: Self = Self(bindings::PERF_RECORD_FORK);
    pub const READ: Self = Self(bindings::PERF_RECORD_READ);
    pub const SAMPLE: Self = Self(bindings::PERF_RECORD_SAMPLE);
    pub const MMAP2: Self = Self(bindings::PERF_RECORD_MMAP2);
    pub const AUX: Self = Self(bindings::PERF_RECORD_AUX);
    pub const ITRACE_START: Self = Self(bindings::PERF_RECORD_ITRACE_START);
    pub const LOST_SAMPLES: Self = Self(bindings::PERF_RECORD_LOST_SAMPLES);
    pub const SWITCH: Self = Self(bindings::PERF_RECORD_SWITCH);
    pub const SWITCH_CPU_WIDE: Self = Self(bindings::PERF_RECORD_SWITCH_CPU_WIDE);
    pub const NAMESPACES: Self = Self(bindings::PERF_RECORD_NAMESPACES);
    pub const KSYMBOL: Self = Self(bindings::PERF_RECORD_KSYMBOL);
    pub const BPF_EVENT: Self = Self(bindings::PERF_RECORD_BPF_EVENT);
    pub const CGROUP: Self = Self(bindings::PERF_RECORD_CGROUP);
    pub const TEXT_POKE: Self = Self(bindings::PERF_RECORD_TEXT_POKE);
}

/// Indicates the CPU mode in which the sample was collected.
///
/// See the [manpage] for the documentation of what each value means.
///
/// [manpage]: http://man7.org/linux/man-pages/man2/perf_event_open.2.html
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct RecordCpuMode(pub u16);

#[allow(missing_docs)]
impl RecordCpuMode {
    pub const UNKNOWN: Self = Self(bindings::PERF_RECORD_MISC_CPUMODE_UNKNOWN as _);
    pub const KERNEL: Self = Self(bindings::PERF_RECORD_MISC_KERNEL as _);
    pub const USER: Self = Self(bindings::PERF_RECORD_MISC_USER as _);
    pub const HYPERVISOR: Self = Self(bindings::PERF_RECORD_MISC_HYPERVISOR as _);
    pub const GUEST_KERNEL: Self = Self(bindings::PERF_RECORD_MISC_GUEST_KERNEL as _);
    pub const GUEST_USER: Self = Self(bindings::PERF_RECORD_MISC_GUEST_USER as _);
}

/// An event emitted by the kernel.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Record {
    /// Indicates which type of event was emitted by the kernel.
    ///
    /// Most of the time you will not need to use this. However, if you run
    /// into events which are not supported by perf-event then this should
    /// give you the ability to parse them from the [`RecordEvent::Unknown`]
    /// variant.
    pub ty: RecordType,

    /// Contains additional inforamtion about the sample.
    pub misc: RecordMiscFlags,

    /// The actual event as emitted by `perf_event_open`.
    pub event: RecordEvent,

    /// If `sample_id_all` is set when creating the sampler then this field
    /// will contain a subset of the selected sample fields.
    pub sample_id: SampleId,
}

/// A subset of the sample fields attached to every event.
///
/// If `sample_id_all` is set when creating the [`Sampler`][crate::Sampler]
/// instance then this struct will contain selected fields related to where
/// and when an event took place.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct SampleId {
    /// The process ID of the process which generated the event.
    pub pid: Option<u32>,

    /// The thread ID of the thread which generated the event.
    pub tid: Option<u32>,

    /// The time at which the event was generated.
    pub time: Option<u64>,

    /// An ID which uniquely identifies the counter.
    ///
    /// If the counter that generated this event was a member of a group, then
    /// this will be the ID of the group leader instead.
    pub id: Option<u64>,

    /// An ID which uniquely identifies the counter.
    ///
    /// If the counter that generated this event is a member of a group, then
    /// this will still be the member of the counter and not the group leader.
    pub stream_id: Option<u64>,

    /// The CPU on which the event was generated.
    pub cpu: Option<u32>,
}

/// The data specific to the record event type.
///
/// If the event type is not supported by `perf-event` then it will return
/// [`RecordEvent::Unknown`].
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum RecordEvent {
    /// Record of a new memory map.
    Mmap(Mmap),

    /// An event was generated but `perf-event` was not able to parse it.
    ///
    /// Instead, the bytes making up the event are available here.
    Unknown(Vec<u8>),
}

/// All the config info needed to parse a record from the perf ring buffer.
///
/// If you need something new, add it here!
#[derive(Default)]
pub(crate) struct ParseConfig {
    sample_type: SampleType,
    sample_id_all: bool,
}

impl Record {
    pub(crate) fn parse<B>(config: &ParseConfig, header: &perf_event_header, buf: &mut B) -> Self
    where
        B: Buf,
    {
        let ty = header.type_.into();
        let sample_id_len = match ty {
            // MMAP and SAMPLE do not include the sample_id trailer
            RecordType::MMAP | RecordType::SAMPLE => None,
            _ => Some(SampleId::expected_size(config)),
        };

        let mut limited = buf.take(buf.remaining() - sample_id_len.unwrap_or(0));
        let event = match ty {
            RecordType::MMAP => Mmap::parse(config, &mut limited).into(),
            _ => RecordEvent::Unknown(limited.parse_remainder()),
        };

        limited.advance(limited.remaining());

        let sample_id = match sample_id_len {
            Some(_) => SampleId::parse(config, buf),
            // Fill in some fields from the record in cases where there is no
            // sample_id encoded with the record.
            None => match &event {
                RecordEvent::Mmap(mmap) => SampleId {
                    pid: Some(mmap.pid),
                    tid: Some(mmap.tid),
                    ..Default::default()
                },
                _ => SampleId::default(),
            },
        };

        Self {
            ty,
            misc: RecordMiscFlags::new(header.misc),
            event,
            sample_id,
        }
    }
}

impl SampleId {
    fn expected_size(config: &ParseConfig) -> usize {
        if config.sample_id_all {
            return 0;
        }

        let configs = [
            config.sample_type.contains(SampleType::TID),
            config.sample_type.contains(SampleType::TIME),
            config.sample_type.contains(SampleType::ID),
            config.sample_type.contains(SampleType::STREAM_ID),
            config.sample_type.contains(SampleType::CPU),
            config.sample_type.contains(SampleType::IDENTIFIER),
        ];

        configs.iter().copied().filter(|&x| x).count() * std::mem::size_of::<u64>()
    }
}

impl From<&'_ perf_event_attr> for ParseConfig {
    fn from(attr: &perf_event_attr) -> Self {
        Self {
            sample_type: SampleType::new(attr.sample_type),
            sample_id_all: attr.sample_id_all() != 0,
        }
    }
}

impl From<perf_event_attr> for ParseConfig {
    fn from(attr: perf_event_attr) -> Self {
        Self::from(&attr)
    }
}

impl From<u32> for RecordType {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl fmt::Debug for RecordType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const NAME: &str = "RecordType";

        match *self {
            Self::MMAP => write!(f, "{NAME}::MMAP"),
            Self::LOST => write!(f, "{NAME}::LOST"),
            Self::COMM => write!(f, "{NAME}::COMM"),
            Self::EXIT => write!(f, "{NAME}::EXIT"),
            Self::THROTTLE => write!(f, "{NAME}::THROTTLE"),
            Self::UNTHROTTLE => write!(f, "{NAME}::UNTHROTTLE"),
            Self::FORK => write!(f, "{NAME}::FORK"),
            Self::READ => write!(f, "{NAME}::READ"),
            Self::SAMPLE => write!(f, "{NAME}::SAMPLE"),
            Self::MMAP2 => write!(f, "{NAME}::MMAP2"),
            Self::AUX => write!(f, "{NAME}::AUX"),
            Self::ITRACE_START => write!(f, "{NAME}::ITRACE_START"),
            Self::LOST_SAMPLES => write!(f, "{NAME}::LOST_SAMPLES"),
            Self::SWITCH => write!(f, "{NAME}::SWITCH"),
            Self::SWITCH_CPU_WIDE => write!(f, "{NAME}::SWITCH_CPU_WIDE"),
            Self::NAMESPACES => write!(f, "{NAME}::NAMESPACES"),
            Self::KSYMBOL => write!(f, "{NAME}::KSYMBOL"),
            Self::BPF_EVENT => write!(f, "{NAME}::BPF_EVENT"),
            Self::CGROUP => write!(f, "{NAME}::CGROUP"),
            Self::TEXT_POKE => write!(f, "{NAME}::TEXT_POKE"),
            Self(value) => f.debug_tuple(NAME).field(&value).finish(),
        }
    }
}

impl From<u16> for RecordCpuMode {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl fmt::Debug for RecordCpuMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const NAME: &str = "RecordCpuMode";

        match *self {
            Self::UNKNOWN => write!(f, "{NAME}::UNKNOWN"),
            Self::KERNEL => write!(f, "{NAME}::KERNEL"),
            Self::USER => write!(f, "{NAME}::USER"),
            Self::HYPERVISOR => write!(f, "{NAME}::HYPERVISOR"),
            Self::GUEST_KERNEL => write!(f, "{NAME}::GUEST_KERNEL"),
            Self::GUEST_USER => write!(f, "{NAME}::GUEST_USER"),
            Self(value) => f.debug_tuple(NAME).field(&value).finish(),
        }
    }
}

impl RecordMiscFlags {
    /// Returns the CPU mode bits.
    pub fn cpumode(&self) -> RecordCpuMode {
        (*self & Self::CPUMODE_MASK).bits().into()
    }
}

/// Trait for types which are parseable given the necessary configuration
/// context.
pub(crate) trait Parse {
    fn parse<B: Buf>(config: &ParseConfig, buf: &mut B) -> Self
    where
        Self: Sized;
}

impl Parse for SampleId {
    fn parse<B: Buf>(config: &ParseConfig, buf: &mut B) -> Self {
        if config.sample_id_all {
            return Self::default();
        }

        let mut sample = Self::default();
        if config.sample_type.contains(SampleType::TID) {
            sample.pid = Some(buf.get_u32_ne());
            sample.tid = Some(buf.get_u32_ne());
        }

        if config.sample_type.contains(SampleType::TIME) {
            sample.time = Some(buf.get_u64_ne());
        }

        if config.sample_type.contains(SampleType::ID) {
            sample.id = Some(buf.get_u64_ne());
        }

        if config.sample_type.contains(SampleType::STREAM_ID) {
            sample.stream_id = Some(buf.get_u64_ne());
        }

        if config.sample_type.contains(SampleType::CPU) {
            sample.cpu = Some(buf.get_u32_ne());
            let _ = buf.get_u32_ne(); // res
        }

        if config.sample_type.contains(SampleType::IDENTIFIER) {
            sample.id = Some(buf.get_u64_ne());
        }

        sample
    }
}

/// Utility trait for parsing data out of a [`Buf`] without panicking.
pub(crate) trait ParseBuf: Buf {
    fn parse_vec(&mut self, mut len: usize) -> Vec<u8> {
        assert!(len <= self.remaining());

        let mut vec = Vec::with_capacity(len);

        while len > 0 {
            let chunk = self.chunk();
            let chunk = &chunk[..len.min(chunk.len())];
            vec.extend_from_slice(chunk);
            len -= chunk.len();
            self.advance(chunk.len());
        }

        vec
    }

    /// Parse the remaining bytes within the buffer to a Vec.
    fn parse_remainder(&mut self) -> Vec<u8> {
        self.parse_vec(self.remaining())
    }

    /// Parse a constant number of bytes to an array.
    fn parse_bytes<const N: usize>(&mut self) -> [u8; N] {
        assert!(N <= self.remaining());

        let mut bytes = [0; N];
        self.copy_to_slice(&mut bytes);
        bytes
    }

    fn parse_header(&mut self) -> bindings::perf_event_header {
        let bytes = self.parse_bytes::<{ std::mem::size_of::<perf_event_header>() }>();
        unsafe { std::mem::transmute(bytes) }
    }
}

impl<B: Buf> ParseBuf for B {}
