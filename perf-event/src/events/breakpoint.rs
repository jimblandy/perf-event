use bitflags::bitflags;
use perf_event_open_sys::bindings;

use crate::events::Event;

bitflags! {
    /// Memory access mask for a hardware data breakpoint.
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    pub struct BreakpointAccess : u32 {
        /// Count when we read the memory location.
        const READ = bindings::HW_BREAKPOINT_R;

        /// Count when we write the memory location.
        const WRITE = bindings::HW_BREAKPOINT_W;

        /// Count when we read or write the memory location.
        const READ_WRITE = Self::READ.union(Self::WRITE).bits();
    }
}

/// A hardware breakpoint.
///
/// A hardware breakpoint watches a region of memory for accesses. It has three
/// parameters:
/// - the address that is being watched (`addr`)
/// - the number of bytes that breakpoint covers (`len`)
/// - which type of memory accesses we care about (`ty`)
///
/// Note that both number of bytes that can be watched as well as the number of
/// breakpoints that is allowed to be active at any given time is limited.
///
/// # Execute Breakpoint
/// We can use a breakpoint to count the number of times that a function gets
/// called, as long as the compiler does not optimize the function away.
///
/// ```
/// # use perf_event::Builder;
/// # use perf_event::events::Breakpoint;
/// #[inline(never)]
/// fn do_some_things() {
///     // ...
///     # println!("test println so the function doesn't get removed")
/// }
///
/// let fnptr = do_some_things as fn() as usize;
/// let mut counter = Builder::new(Breakpoint::execute(fnptr as u64)).build()?;
/// counter.enable()?;
///
/// for _ in 0..500 {
///     do_some_things();
/// }
///
/// counter.disable()?;
/// assert_eq!(counter.read()?, 500);
/// # std::io::Result::Ok(())
/// ```
///
/// # Data Breakpoint
/// We can also use a breakpoint to count the number of times that a memory
/// location is accessed.
/// ```
/// # use perf_event::Builder;
/// # use perf_event::events::Breakpoint;
/// #
/// let mut data: Vec<u64> = (0..1024).rev().collect();
///
/// let breakpoint = Breakpoint::read_write(&data[20] as *const _ as usize as u64, 8);
/// let mut counter = Builder::new(breakpoint).build()?;
/// counter.enable()?;
/// data.sort();
/// counter.disable()?;
///
/// println!("Position 20 accessed {} times", counter.read()?);
/// # std::io::Result::Ok(())
/// ```
///
/// # Usage Notes
/// - Some systems do not support creating read-only or write-only breakpoints.
///   If you are getting `EINVAL` errors while trying to build such a counter
///   using a read-write breakpoint might work instead.
///
/// - The valid values of len are quite limited. The [`perf_event_open`][man]
///   manpage indicates that the only valid values for `bp_len` are 1, 2, 4, and
///   8.
///
/// [man]: http://man7.org/linux/man-pages/man2/perf_event_open.2.html
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Breakpoint {
    /// Data breakpoint. Triggers when code reads or writes to the memory area
    /// as configured by the parameters below.
    Data {
        /// Bitfield containing the types of accesses we want the breakpoint to
        /// trigger on.
        access: BreakpointAccess,

        /// The address of the memory location on which the breakpoint should
        /// trigger.
        addr: u64,

        /// The length of the breakpoint being measured.
        ///
        /// There are a limited number of valid values for this field.
        /// Basically, the options are 1, 2, 4, and 8. Setting this
        /// field to anything else will cause counter creation to fail
        /// with an error.
        len: u64,
    },

    /// Code breakpoint. Triggers when the code at the address is executed.
    Code {
        /// The address that the breakpoint is monitoring.
        addr: u64,
    },
}

impl Breakpoint {
    /// Create a code execution breakpoint, that counts the number of
    /// times the instruction at the provided address was executed.
    pub const fn execute(addr: u64) -> Self {
        Self::Code { addr }
    }

    /// Create a memory read breakpoint, that counts the number of
    /// times we read from the provided memory location.
    ///
    /// See the struct field docs for valid values of `len`.
    pub const fn read(addr: u64, len: u64) -> Self {
        Self::Data {
            access: BreakpointAccess::READ,
            addr,
            len,
        }
    }

    /// Create a memory write breakpoint, that counts the number of
    /// times we write to the provided memory location.
    ///
    /// See the struct field docs for valid values of `len`.
    pub const fn write(addr: u64, len: u64) -> Self {
        Self::Data {
            access: BreakpointAccess::WRITE,
            addr,
            len,
        }
    }

    /// Create a memory access breakpoint, that counts the number of
    /// times we either read from or write to the provided memory
    /// location.
    ///
    /// See the struct field docs for valid values of `len`.
    pub const fn read_write(addr: u64, len: u64) -> Self {
        Self::Data {
            access: BreakpointAccess::READ_WRITE,
            addr,
            len,
        }
    }
}

impl Event for Breakpoint {
    fn update_attrs(self, attr: &mut bindings::perf_event_attr) {
        attr.type_ = bindings::PERF_TYPE_BREAKPOINT;
        attr.config = 0;

        match self {
            Self::Data { access, addr, len } => {
                attr.bp_type = access.bits();
                attr.__bindgen_anon_3.bp_addr = addr;
                attr.__bindgen_anon_4.bp_len = len;
            }
            Self::Code { addr } => {
                attr.bp_type = bindings::HW_BREAKPOINT_X;
                attr.__bindgen_anon_3.bp_addr = addr;
                // According to the perf_event_open man page, execute breakpoints
                // should set len to sizeof(long).
                attr.__bindgen_anon_4.bp_len = std::mem::size_of::<libc::c_long>() as _;
            }
        }
    }
}
