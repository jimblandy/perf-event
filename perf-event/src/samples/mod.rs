//! TODO

#![warn(missing_docs)]

use bitflags::bitflags;
use bytes::Buf;
use perf_event_open_sys::bindings::{self, perf_event_attr, perf_event_header};
use std::fmt;

bitflags! {
    /// Specifies which fields to include in the sample.
    ///
    /// These fields will be recorded in the [`Sampler`] output buffer.
    ///
    /// [`Sampler`]: crate::Sampler
    pub struct Sample : u64 {
        /// Record the instruction pointer.
        const IP = bindings::PERF_SAMPLE_IP;

        /// Record the process and thread IDs.
        const TID = bindings::PERF_SAMPLE_TID;

        /// Record a timestamp.
        const TIME = bindings::PERF_SAMPLE_TIME;

        /// Record an address, if applicable.
        const ADDR = bindings::PERF_SAMPLE_ADDR;

        /// Record counter values for all events in a group, not just the
        /// group leader.
        const READ = bindings::PERF_SAMPLE_READ;

        /// Record a unique ID for the opened event's group leader.
        const CALLCHAIN = bindings::PERF_SAMPLE_CALLCHAIN;

        /// Record CPU number.
        const ID = bindings::PERF_SAMPLE_ID;

        /// Record the callchain (stack backtrace).
        const CPU = bindings::PERF_SAMPLE_CPU;

        /// Record the current sampling period.
        const PERIOD = bindings::PERF_SAMPLE_PERIOD;

        /// Record a unique ID for the opened event. Unlike [`ID`] an actual ID
        /// is returned, not the group leader.
        ///
        /// This ID is the same as the one returned by [`Counter::id`]
        ///
        /// [`ID`]: Self::ID
        /// [`Counter::id`]: crate::Counter::id
        const STREAM_ID = bindings::PERF_SAMPLE_STREAM_ID;

        /// Record additional data, if applicable. Usually returned by
        /// tracepoint events.
        const RAW = bindings::PERF_SAMPLE_RAW;

        /// Provides a record of recent branches, as provided by CPU branch
        /// sampling hardware (such as Intel Last Branch Record). Not all
        /// hardware supports this feature.
        ///
        /// Available since Linux 3.4.
        const BRANCH_STACK = bindings::PERF_SAMPLE_BRANCH_STACK;

        /// Record the current user-level CPU register state (the values in the
        /// process before the kernel was called).
        ///
        /// Available since Linux 3.7.
        const REGS_USER = bindings::PERF_SAMPLE_REGS_USER;

        /// Record the user level stack, allowing stack unwinding.
        ///
        /// Available since Linux 3.7.
        const STACK_USER = bindings::PERF_SAMPLE_STACK_USER;

        /// Record a hardware provided weight value that expresses how costly
        /// the sampled event was. This allows the hardware to highlight
        /// expensive events in a profile.
        ///
        /// Available since Linux 3.10.
        const WEIGHT = bindings::PERF_SAMPLE_WEIGHT;

        /// Record the data source: where in the memory hierarchy the data
        /// associated with the sampled instruction came from. This is
        /// available only if the underlying hardware supports this feature.
        ///
        /// Available since Linux 3.10.
        const DATA_SRC = bindings::PERF_SAMPLE_DATA_SRC;

        /// Places the [`ID`] value in a fixed position in the record, either
        /// at the beginning (for sample events) or at the end (if a non-sample
        /// event).
        ///
        /// This was necessary because a sample stream may have records from
        /// various different event sources with different `sample_type`
        /// settings. Parsing the event stream properly was not possible
        /// because the format of the record was needed to find the `SAMPLE_ID`,
        /// but the format could not be found without knowing what event the
        /// sample belonged to (causing a circular dependency).
        ///
        /// The `IDENTIFIER` setting makes the event stream always parsable by
        /// putting `ID` in a fixed location, even though it means having
        /// duplicate `ID` fields in records.
        const IDENTIFIER = bindings::PERF_SAMPLE_IDENTIFIER;

        /// Record reasons for transactional memory abort events (for example,
        /// from Intel TSX transactional memory support).
        ///
        /// The `precise_ip` setting must be greater than 0 and a transactional
        /// memory abort must be measured or no values will be recorded. Also
        /// note that some perf_event measurements, such as sampled cycle
        /// counting, may cause extraneous aborts (by causing an interrupt
        /// during a transaction).
        const TRANSACTION = bindings::PERF_SAMPLE_TRANSACTION;

        /// Record a subset of the current CPU register state as specified by
        /// `sample_regs_intr`. Unlike [`REGS_USER`] the register values will
        /// return kernel register state if the overflow happened while kernel
        /// code was running. If the CPU supports hardware sampling of register
        /// state (e.e. PEBS on Intel x86) and `precise_ip` is set higher than
        /// zero then the register values returned are those captured by the
        /// hardware at the time of the sampled instruction's retirement.
        ///
        /// [`REGS_USER`]: Self::REGS_USER
        const REGS_INTR = bindings::PERF_SAMPLE_REGS_INTR;

        /// Record the physical address of data like in [`Sample::ADDR`].
        const PHYS_ADDR = bindings::PERF_SAMPLE_PHYS_ADDR;

        /// Record the perf_event cgroup ID of the process. This corresponds
        /// to the `id` field of the `CGROUP` event.
        const CGROUP = bindings::PERF_SAMPLE_CGROUP;

        // Don't clobber unknown flags when constructing the bitflag struct.
        #[doc(hidden)]
        const _ALLOW_ALL_FLAGS = !0;
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
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct RecordCpuMode(pub u16);

impl RecordCpuMode {
    /// Unknown CPU mode.
    pub const UNKNOWN: Self = Self(bindings::PERF_RECORD_MISC_CPUMODE_UNKNOWN as _);

    /// The sample happened in the kernel.
    pub const KERNEL: Self = Self(bindings::PERF_RECORD_MISC_KERNEL as _);

    /// The sample happened in user code.
    pub const USER: Self = Self(bindings::PERF_RECORD_MISC_USER as _);

    /// The sample happened in the hypervisor.
    pub const HYPERVISOR: Self = Self(bindings::PERF_RECORD_MISC_HYPERVISOR as _);

    /// The sample happened in the guest kernel (since Linux 2.6.35).
    pub const GUEST_KERNEL: Self = Self(bindings::PERF_RECORD_MISC_GUEST_KERNEL as _);

    /// The sample happened in guest user code (since Linux 2.6.35).
    pub const GUEST_USER: Self = Self(bindings::PERF_RECORD_MISC_GUEST_USER as _);
}

bitflags! {
    /// Additional flags about the record event.
    ///
    /// Not all of these can be for each sample.
    #[derive(Default)]
    pub struct RecordMiscFlags : u16 {
        /// The first 3 bits of the misc flags actually contain an enum that
        /// describes in what cpu mode the sample was collected from.
        ///
        /// To access this, use the [`cpumode`][Self::cpumode] function.
        const CPUMODE_MASK = bindings::PERF_RECORD_MISC_CPUMODE_MASK as _;

        /// Indicates that the mapping is not executable; otherwise the mapping
        /// is executable.
        ///
        /// This flag only applies to MMAP and MMAP2 records.
        const MMAP_DATA = bindings::PERF_RECORD_MISC_MMAP_DATA as _;

        /// Indicates that the process name change was caused by an
        /// [`execve(2)`] system call. Only emitted on kernels more recent than
        /// Linux 3.16.
        ///
        /// This flag only applies to COMM records.
        ///
        /// [`execve(2)`]: https://man7.org/linux/man-pages/man2/execve.2.html
        const COMM_EXEC = bindings::PERF_RECORD_MISC_COMM_EXEC as _;

        /// When a SWITCH or SWITCH_CPU_WIDE record is generated, this bit
        /// indicates that the context switch is away from the current process
        /// (instead of into the current process).
        ///
        /// This flag only applies to SWITCH and SWITCH_CPU_WIDE records.
        const SWITCH_OUT = bindings::PERF_RECORD_MISC_SWITCH_OUT as _;

        /// Indicates that sampled ip address within the record points to the
        /// actual instruction that triggered the event.
        const EXACT_IP = bindings::PERF_RECORD_MISC_EXACT_IP as _;

        /// Indicates that there is extended data available (since Linux 2.6.35).
        /// This flag is currently not used.
        const EXT_RESERVED = bindings::PERF_RECORD_MISC_EXT_RESERVED as _;

        // New flags will likely be added to the perf_event_open interface in
        // the future. In that case we would like to avoid deleting those flags.
        // This field will ensure that the bitflags crate does not truncate any
        // flags when we construct a RecordMiscFlags instance.
        #[doc(hidden)]
        const _ALLOW_ALL_FLAGS = u16::MAX;
    }
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

    /// An ID which uniquely identifies the counter. If the counter is a member
    /// of an event group then the group leader ID is returned instead.
    pub id: Option<u64>,

    /// An ID which uniquely identifies the counter. Unlike `id`, if the
    /// counter is a member of a group then the counter's ID is returned and
    /// not the group leader's.
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
    /// An event was generated but `perf-event` was not able to parse it.
    /// Instead, the bytes making up the event are available here.
    Unknown(Vec<u8>),
}

#[allow(missing_docs)]
pub enum ParseError {
    UnexpectedEof,
    UnexpectedRemainder,
    Unsupported(&'static str),
}

impl From<u32> for RecordType {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl fmt::Debug for RecordType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut st = f.debug_tuple("RecordType");

        match *self {
            Self::MMAP => st.field(&"MMAP"),
            Self::LOST => st.field(&"LOST"),
            Self::COMM => st.field(&"COMM"),
            Self::EXIT => st.field(&"EXIT"),
            Self::THROTTLE => st.field(&"THROTTLE"),
            Self::UNTHROTTLE => st.field(&"UNTHROTTLE"),
            Self::FORK => st.field(&"FORK"),
            Self::READ => st.field(&"READ"),
            Self::SAMPLE => st.field(&"SAMPLE"),
            Self::MMAP2 => st.field(&"MMAP2"),
            Self::AUX => st.field(&"AUX"),
            Self::ITRACE_START => st.field(&"ITRACE_START"),
            Self::LOST_SAMPLES => st.field(&"LOST_SAMPLES"),
            Self::SWITCH => st.field(&"SWITCH"),
            Self::SWITCH_CPU_WIDE => st.field(&"SWITCH_CPU_WIDE"),
            Self::NAMESPACES => st.field(&"NAMESPACES"),
            Self::KSYMBOL => st.field(&"KSYMBOL"),
            Self::BPF_EVENT => st.field(&"BPF_EVENT"),
            Self::CGROUP => st.field(&"CGROUP"),
            Self::TEXT_POKE => st.field(&"TEXT_POKE"),
            Self(value) => st.field(&value),
        };

        st.finish()
    }
}

impl From<u16> for RecordCpuMode {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl fmt::Debug for RecordCpuMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut st = f.debug_tuple("RecordCpuMode");

        match *self {
            Self::UNKNOWN => st.field(&"UNKNOWN"),
            Self::KERNEL => st.field(&"KERNEL"),
            Self::USER => st.field(&"USER"),
            Self::HYPERVISOR => st.field(&"HYPERVISOR"),
            Self::GUEST_KERNEL => st.field(&"GUEST_KERNEL"),
            Self::GUEST_USER => st.field(&"GUEST_USER"),
            Self(value) => st.field(&value)
        };

        st.finish()
    }
}

impl RecordMiscFlags {
    /// Returns the CPU mode bits.
    pub fn cpumode(&self) -> RecordCpuMode {
        (*self & Self::CPUMODE_MASK).bits().into()
    }
}

impl ParseError {
    pub(crate) fn unexpected_eof() -> Self {
        Self::UnexpectedEof
    }

    pub(crate) fn unexpected_remaining_input() -> Self {
        Self::UnexpectedRemainder
    }

    /// The message here is to force documenting _why_ whatever we're trying to
    /// do is unsupported so that it's there when someone goes to fix it.
    #[allow(dead_code)]
    pub(crate) fn unsupported(message: &'static str) -> Self {
        Self::Unsupported(message)
    }
}

/// Trait for types which are parseable given the necessary configuration
/// context.
pub(crate) trait Parse {
    fn parse<B: Buf>(
        attr: &perf_event_attr,
        header: &perf_event_header,
        buf: &mut B,
    ) -> Result<Self, ParseError>
    where
        Self: Sized;
}

impl Parse for Record {
    fn parse<B: Buf>(
        attr: &perf_event_attr,
        header: &perf_event_header,
        buf: &mut B,
    ) -> Result<Self, ParseError> {
        let sample_id_len = SampleId::expected_size(attr);
        let mut limited = buf.take(buf.remaining() - sample_id_len);

        let event = match header.type_ {
            _ => RecordEvent::Unknown({
                let remaining = limited.remaining();
                limited.parse_vec(remaining)?
            }),
        };

        if limited.remaining() != 0 {
            return Err(ParseError::unexpected_remaining_input());
        }

        let sample_id = SampleId::parse(attr, header, buf)?;

        Ok(Self {
            ty: header.type_.into(),
            misc: RecordMiscFlags::from_bits_truncate(header.misc),
            event,
            sample_id,
        })
    }
}

impl SampleId {
    fn expected_size(attr: &perf_event_attr) -> usize {
        if attr.sample_id_all() == 0 {
            return 0;
        }

        let mut len = 0;

        if contains(attr.sample_type, bindings::PERF_SAMPLE_TID) {
            len += std::mem::size_of::<u64>();
        }

        if contains(attr.sample_type, bindings::PERF_SAMPLE_TIME) {
            len += std::mem::size_of::<u64>();
        }

        if contains(attr.sample_type, bindings::PERF_SAMPLE_ID) {
            len += std::mem::size_of::<u64>();
        }

        if contains(attr.sample_type, bindings::PERF_SAMPLE_STREAM_ID) {
            len += std::mem::size_of::<u64>();
        }

        if contains(attr.sample_type, bindings::PERF_SAMPLE_CPU) {
            len += std::mem::size_of::<u64>();
        }

        if contains(attr.sample_type, bindings::PERF_SAMPLE_IDENTIFIER) {
            len += std::mem::size_of::<u64>();
        }

        len
    }
}

impl Parse for SampleId {
    fn parse<B: Buf>(
        attr: &perf_event_attr,
        _: &perf_event_header,
        buf: &mut B,
    ) -> Result<Self, ParseError> {
        if attr.sample_id_all() == 0 {
            return Ok(Self::default());
        }

        let mut sample = Self::default();
        if contains(attr.sample_type, bindings::PERF_SAMPLE_TID) {
            sample.pid = Some(buf.parse()?);
            sample.tid = Some(buf.parse()?);
        }

        if contains(attr.sample_type, bindings::PERF_SAMPLE_TIME) {
            sample.time = Some(buf.parse()?);
        }

        if contains(attr.sample_type, bindings::PERF_SAMPLE_ID) {
            sample.id = Some(buf.parse()?);
        }

        if contains(attr.sample_type, bindings::PERF_SAMPLE_STREAM_ID) {
            sample.stream_id = Some(buf.parse()?);
        }

        if contains(attr.sample_type, bindings::PERF_SAMPLE_CPU) {
            sample.cpu = Some(buf.parse()?);
            let _ = buf.parse::<u32>()?; // res
        }

        if contains(attr.sample_type, bindings::PERF_SAMPLE_IDENTIFIER) {
            sample.id = Some(buf.parse()?);
        }

        Ok(sample)
    }
}

/// Utility trait for parsing data out of a [`Buf`] without panicking.
pub(crate) trait ParseBuf: Buf {
    fn parse_vec(&mut self, mut len: usize) -> Result<Vec<u8>, ParseError> {
        if self.remaining() < len {
            return Err(ParseError::unexpected_eof());
        }

        let mut bytes = Vec::with_capacity(len);
        while len > 0 {
            let chunk = self.chunk();
            let chunk = &chunk[..len.min(chunk.len())];
            bytes.extend_from_slice(chunk);
            len -= chunk.len();
            self.advance(chunk.len());
        }

        Ok(bytes)
    }

    fn parse_bytes<const N: usize>(&mut self) -> Result<[u8; N], ParseError> {
        if self.remaining() < N {
            return Err(ParseError::unexpected_eof());
        }

        let mut bytes = [0; N];
        self.copy_to_slice(&mut bytes);
        Ok(bytes)
    }

    fn parse_header(&mut self) -> Result<bindings::perf_event_header, ParseError> {
        let bytes = self.parse_bytes::<{ std::mem::size_of::<perf_event_header>() }>()?;
        Ok(unsafe { std::mem::transmute(bytes) })
    }

    fn parse<P: Parseable>(&mut self) -> Result<P, ParseError> {
        P::parse(self)
    }
}

impl<B: Buf> ParseBuf for B {
    fn parse_vec(&mut self, mut len: usize) -> Result<Vec<u8>, ParseError> {
        if self.remaining() < len {
            return Err(ParseError::unexpected_eof());
        }

        let mut bytes = Vec::with_capacity(len);
        while len > 0 {
            let chunk = self.chunk();
            let chunk = &chunk[..len.min(chunk.len())];
            bytes.extend_from_slice(chunk);
            len -= chunk.len();
            self.advance(chunk.len());
        }

        Ok(bytes)
    }

    fn parse_bytes<const N: usize>(&mut self) -> Result<[u8; N], ParseError> {
        if self.remaining() < N {
            return Err(ParseError::unexpected_eof());
        }

        let mut bytes = [0; N];
        self.copy_to_slice(&mut bytes);
        Ok(bytes)
    }
}

/// Utility trait for [`ParseBuf::parse`].
pub(crate) trait Parseable: Sized {
    fn parse<B>(buf: &mut B) -> Result<Self, ParseError>
    where
        B: Buf + ?Sized;
}

macro_rules! parse_impl {
    ($ty:ty) => {
        impl Parseable for $ty {
            fn parse<B>(mut buf: &mut B) -> Result<Self, ParseError>
            where
                B: Buf + ?Sized,
            {
                buf.parse_bytes().map(Self::from_ne_bytes)
            }
        }
    };
}

parse_impl!(u8);
parse_impl!(u16);
parse_impl!(u32);
parse_impl!(u64);

/// Small utility method to mask some of the arithmetic away.
fn contains(sample: u64, flag: u64) -> bool {
    sample & flag != 0
}
