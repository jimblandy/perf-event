use std::io;

use crate::events::Software;
use crate::{Builder, Counter, GroupData, ReadFormat};

/// A group of counters that can be managed as a unit.
///
/// A `Group` represents a group of [`Counter`]s that can be enabled,
/// disabled, reset, or read as a single atomic operation. This is necessary if
/// you want to compare counter values, produce ratios, and so on, since those
/// operations are only meaningful on counters that cover exactly the same
/// period of execution.
///
/// A `Counter` is placed in a group when it is created via the
/// [`Builder::build_with_group`] method. A `Group`'s [`read`] method returns
/// values of all its member counters at once as a [`GroupData`] value, which
/// can be indexed by `Counter` to retrieve a specific value.
///
/// The lifetime of a `Group` and its associated `Counter`s are independent:
/// you can drop them in any order and they will continue to work. A `Counter`
/// will continue to work after the `Group` is dropped. If a `Counter` is
/// dropped first then it will simply be removed from the `Group`.
///
/// Enabling or disabling a `Group` affects each `Counter` that belongs to it.
/// Subsequent reads from the `Counter` will not reflect activity while the
/// `Group` was disabled, unless the `Counter` is re-enabled individually.
///
/// ## Limits on group size
///
/// Hardware counters are implemented using special-purpose registers on the
/// processor, of which there are only a fixed number. (For example, an Intel
/// high-end laptop processor from 2015 has four such registers per virtual
/// processor.) Without using groups, if you request more hardware counters than
/// the processor can actually support, a complete count isn't possible, but the
/// kernel will rotate the processor's real registers amongst the measurements
/// you've requested to at least produce a sample.
///
/// But since the point of a counter group is that its members all cover exactly
/// the same period of time, this tactic can't be applied to support large
/// groups. If the kernel cannot schedule a group, its counters remain zero. I
/// think you can detect this situation by comparing the group's
/// [`time_enabled`] and [`time_running`] values. If the [`pinned`] option is
/// set then you will also be able to detect this by [`read`] returning an error
/// with kind [`UnexpectedEof`].
///
/// According to the `perf_list(1)` man page, you may be able to free up a
/// hardware counter by disabling the kernel's NMI watchdog, which reserves one
/// for detecting kernel hangs:
///
/// ```text
/// $ echo 0 > /proc/sys/kernel/nmi_watchdog
/// ```
///
/// You can reenable the watchdog when you're done like this:
///
/// ```text
/// $ echo 1 > /proc/sys/kernel/nmi_watchdog
/// ```
///
/// [`read`]: Self::read
/// [`pinned`]: Builder::pinned
/// [`UnexpectedEof`]: io::ErrorKind::UnexpectedEof
///
/// # Examples
/// Compute the average cycles-per-instruction (CPI) for a call to `println!`:
/// ```
/// use perf_event::events::Hardware;
/// use perf_event::{Builder, Group};
///
/// let mut group = Group::new()?;
/// let cycles = group.add(&Builder::new(Hardware::CPU_CYCLES))?;
/// let insns = group.add(&Builder::new(Hardware::INSTRUCTIONS))?;
///
/// let vec = (0..=51).collect::<Vec<_>>();
///
/// group.enable()?;
/// println!("{:?}", vec);
/// group.disable()?;
///
/// let counts = group.read()?;
/// println!(
///     "cycles / instructions: {} / {} ({:.2} cpi)",
///     counts[&cycles],
///     counts[&insns],
///     (counts[&cycles] as f64 / counts[&insns] as f64)
/// );
/// # std::io::Result::Ok(())
/// ```
///
/// [`read`]: Group::read
/// [`time_enabled`]: GroupData::time_enabled
/// [`time_running`]: GroupData::time_running
pub struct Group(pub(crate) Counter);

impl Group {
    /// Construct a new, empty `Group`.
    ///
    /// The resulting `Group` is only suitable for observing the current process
    /// on any CPU. If you need to build a `Group` with different settings you
    /// will need to use [`Builder::build_group`].
    pub fn new() -> io::Result<Group> {
        Builder::new(Software::DUMMY)
            .read_format(
                ReadFormat::GROUP
                    | ReadFormat::TOTAL_TIME_ENABLED
                    | ReadFormat::TOTAL_TIME_RUNNING
                    | ReadFormat::ID,
            )
            .build_group()
    }

    /// Access the internal counter for this group.
    pub fn as_counter(&self) -> &Counter {
        &self.0
    }

    /// Mutably access the internal counter for this group.
    pub fn as_counter_mut(&mut self) -> &mut Counter {
        &mut self.0
    }

    /// Convert this `Group` into its internal counter.
    pub fn into_counter(self) -> Counter {
        self.0
    }

    /// Return this group's kernel-assigned unique id.
    pub fn id(&self) -> u64 {
        self.0.id()
    }

    /// Enable all counters in this `Group`.
    pub fn enable(&mut self) -> io::Result<()> {
        self.0.enable_group()
    }

    /// Disable all counters in this `Group`
    pub fn disable(&mut self) -> io::Result<()> {
        self.0.disable_group()
    }

    /// Reset the value of all counters in this `Group` to zero.
    pub fn reset(&mut self) -> io::Result<()> {
        self.0.reset_group()
    }

    /// Construct a new counter as a part of this group.
    ///
    /// # Example
    /// ```
    /// use perf_event::events::Hardware;
    /// use perf_event::{Builder, Group};
    ///
    /// let mut group = Group::new()?;
    /// let counter = group.add(&Builder::new(Hardware::INSTRUCTIONS).any_cpu());
    /// #
    /// # std::io::Result::Ok(())
    /// ```
    pub fn add(&mut self, builder: &Builder) -> io::Result<Counter> {
        builder.build_with_group(self)
    }

    /// Return the values of all the `Counter`s in this `Group` as a
    /// [`GroupData`] value.
    ///
    /// A [`GroupData`] value is a map from specific `Counter`s to their values.
    /// You can find a specific `Counter`'s value by indexing:
    ///
    /// ```
    /// # use perf_event::events::Software;
    /// # const RHOMBUS_INCLINATIONS: Software = Software::DUMMY;
    /// # const TAXI_MEDALLIONS: Software = Software::DUMMY;
    /// #
    /// use perf_event::{Builder, Group};
    ///
    /// let mut group = Group::new()?;
    /// let counter1 = Builder::new(RHOMBUS_INCLINATIONS).build_with_group(&mut group)?;
    /// let counter2 = Builder::new(TAXI_MEDALLIONS).build_with_group(&mut group)?;
    /// // ...
    /// let counts = group.read()?;
    /// println!(
    ///     "Rhombus inclinations per taxi medallion: {} / {} ({:.0}%)",
    ///     counts[&counter1],
    ///     counts[&counter2],
    ///     (counts[&counter1] as f64 / counts[&counter2] as f64) * 100.0
    /// );
    /// # std::io::Result::Ok(())
    /// ```
    pub fn read(&mut self) -> io::Result<GroupData> {
        let mut data = self.0.read_group()?;
        data.skip_group();
        Ok(data)
    }
}

impl AsRef<Counter> for &'_ Group {
    fn as_ref(&self) -> &Counter {
        &self.0
    }
}

impl AsMut<Counter> for &'_ mut Group {
    fn as_mut(&mut self) -> &mut Counter {
        &mut self.0
    }
}
