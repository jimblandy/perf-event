use std::fmt;
use std::iter::FusedIterator;
use std::time::Duration;

use crate::{Builder, Counter, Group, ReadFormat};

const SKIP_GROUP: ReadFormat = ReadFormat::from_bits_retain(1 << (u64::BITS - 1));

used_in_docs!(Group);
used_in_docs!(Builder);

/// A collection of counts from a group of counters.
///
/// This is the type returned by [`Counter::read_group`] and [`Group::read`].
/// You can index it with a reference to a specific [`Counter`]:
///
/// ```
/// use perf_event::events::Hardware;
/// use perf_event::{Builder, Group};
///
/// let mut group = Group::new()?;
/// let cycles = group.add(&Builder::new(Hardware::CPU_CYCLES))?;
/// let insns = group.add(&Builder::new(Hardware::INSTRUCTIONS))?;
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
/// Or you can iterate over the results it contains:
///
/// ```
/// # fn main() -> std::io::Result<()> {
/// # use perf_event::Group;
/// # let counts = Group::new()?.read()?;
/// for entry in &counts {
///     println!("Counter id {} has value {}", entry.id(), entry.value());
/// }
/// # Ok(())
/// # }
/// ```
///
/// The `id` values produced by this iteration are internal identifiers assigned
/// by the kernel. You can use the [`Counter::id`] method to find a
/// specific counter's id.
///
/// For some kinds of events, the kernel may use timesharing to give all
/// counters access to scarce hardware registers. You can see how long a group
/// was actually running versus the entire time it was enabled using the
/// `time_enabled` and `time_running` methods:
///
/// ```
/// # use perf_event::{Builder, Group};
/// # use perf_event::events::Software;
/// # let mut group = Group::new()?;
/// # let insns = group.add(&Builder::new(Software::DUMMY))?;
/// # let counts = group.read()?;
/// let scale =
///     counts.time_enabled().unwrap().as_secs_f64() / counts.time_running().unwrap().as_secs_f64();
/// for entry in &counts {
///     let value = entry.value() as f64 * scale;
///
///     print!("Counter id {} has value {}", entry.id(), value as u64);
///     if scale > 1.0 {
///         print!(" (estimated)");
///     }
///     println!();
/// }
/// # std::io::Result::Ok(())
/// ```
pub struct GroupData {
    // Raw results from the `read`.
    data: Vec<u64>,
    read_format: ReadFormat,
}

impl GroupData {
    pub(crate) fn new(data: Vec<u64>, read_format: ReadFormat) -> Self {
        Self { data, read_format }
    }

    /// Return the number of counters this `Counts` holds results for.
    pub fn len(&self) -> usize {
        let len = self.data[0] as usize;

        if self.should_skip() {
            len - 1
        } else {
            len
        }
    }

    /// Whether this `GroupData` is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The duration for which the group was enabled.
    ///
    /// This will only be present if [`TOTAL_TIME_ENABLED`] was passed to
    /// [`read_format`].
    ///
    /// [`TOTAL_TIME_ENABLED`]: ReadFormat::TOTAL_TIME_ENABLED
    /// [`read_format`]: Builder::read_format
    pub fn time_enabled(&self) -> Option<Duration> {
        self.prefix_offset_of(ReadFormat::TOTAL_TIME_ENABLED)
            .map(|idx| self.data[idx])
            .map(Duration::from_nanos)
    }

    /// The duration for which the group was scheduled on the CPU.
    ///
    /// This will only be present if [`TOTAL_TIME_RUNNING`] was passed to
    /// [`read_format`].
    ///
    /// [`TOTAL_TIME_RUNNING`]: ReadFormat::TOTAL_TIME_RUNNING
    /// [`read_format`]: Builder::read_format
    pub fn time_running(&self) -> Option<Duration> {
        self.prefix_offset_of(ReadFormat::TOTAL_TIME_RUNNING)
            .map(|idx| self.data[idx])
            .map(Duration::from_nanos)
    }

    /// Get the entry for `member` in `self`, or `None` if `member` is not
    /// present.
    ///
    /// `member` can be either a `Counter` or a `Group`.
    ///
    /// If you know the counter is in the group then you can access the count
    /// via indexing.
    /// ```
    /// use perf_event::events::Hardware;
    /// use perf_event::{Builder, Group};
    ///
    /// let mut group = Group::new()?;
    /// let instrs = Builder::new(Hardware::INSTRUCTIONS).build_with_group(&mut group)?;
    /// let cycles = Builder::new(Hardware::CPU_CYCLES).build_with_group(&mut group)?;
    /// group.enable()?;
    /// // ...
    /// let counts = group.read()?;
    /// let instrs = counts[&instrs];
    /// let cycles = counts[&cycles];
    /// # std::io::Result::Ok(())
    /// ```
    pub fn get(&self, member: &Counter) -> Option<GroupEntry> {
        self.iter_with_group()
            .find(|entry| entry.id() == member.id())
    }

    /// Return an iterator over all entries in `self`.
    ///
    /// For compatibility reasons, if the [`Group`] this was
    ///
    /// # Example
    /// ```
    /// # use perf_event::Group;
    /// # let mut group = Group::new()?;
    /// let data = group.read()?;
    /// for entry in &data {
    ///     println!("Counter with id {} has value {}", entry.id(), entry.value());
    /// }
    /// # std::io::Result::Ok(())
    /// ```
    pub fn iter(&self) -> GroupIter {
        let mut iter = self.iter_with_group();
        if self.should_skip() {
            let _ = iter.next();
        }
        iter
    }

    fn iter_with_group(&self) -> GroupIter {
        GroupIter::new(
            self.read_format,
            &self.data[self.read_format.prefix_len()..],
        )
    }

    fn should_skip(&self) -> bool {
        self.read_format.contains(SKIP_GROUP)
    }

    /// Mark that the first counter in this group is a `Group` and should not be
    /// included when iterating over this `GroupData` instance.
    pub(crate) fn skip_group(&mut self) {
        self.read_format |= SKIP_GROUP;
    }

    fn prefix_offset_of(&self, flag: ReadFormat) -> Option<usize> {
        debug_assert_eq!(flag.bits().count_ones(), 1);

        let read_format =
            self.read_format & (ReadFormat::TOTAL_TIME_ENABLED | ReadFormat::TOTAL_TIME_RUNNING);

        if !self.read_format.contains(flag) {
            return None;
        }

        Some((read_format.bits() & (flag.bits() - 1)).count_ones() as _)
    }
}

impl std::ops::Index<&Counter> for GroupData {
    type Output = u64;

    fn index(&self, ctr: &Counter) -> &u64 {
        let data = self
            .iter_with_group()
            .iter
            .find(|data| {
                let entry = GroupEntry::new(self.read_format, *data);
                entry.id() == ctr.id()
            })
            .unwrap_or_else(|| panic!("group contained no counter with id {}", ctr.id()));

        &data[0]
    }
}

impl fmt::Debug for GroupData {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        struct GroupEntries<'a>(&'a GroupData);

        impl fmt::Debug for GroupEntries<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_list().entries(self.0.iter()).finish()
            }
        }

        let mut dbg = fmt.debug_struct("GroupData");

        if let Some(time_enabled) = self.time_enabled() {
            dbg.field("time_enabled", &time_enabled.as_nanos());
        }

        if let Some(time_running) = self.time_running() {
            dbg.field("time_running", &time_running.as_nanos());
        }

        dbg.field("entries", &GroupEntries(self));
        dbg.finish()
    }
}

impl<'a> IntoIterator for &'a GroupData {
    type IntoIter = GroupIter<'a>;
    type Item = <GroupIter<'a> as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Individual entry for a counter returned by [`Group::read`].
#[derive(Copy, Clone)]
pub struct GroupEntry {
    // Note: Make sure to update the Debug impl below when adding a field here.
    read_format: ReadFormat,
    value: u64,
    id: u64,
    lost: u64,
}

impl GroupEntry {
    fn new(read_format: ReadFormat, data: &[u64]) -> Self {
        Self {
            read_format,
            value: data[0],
            id: data[1],
            lost: data.get(2).copied().unwrap_or(0),
        }
    }

    /// TODO
    pub fn value(&self) -> u64 {
        self.value
    }

    /// TODO
    pub fn id(&self) -> u64 {
        self.id
    }

    /// TODO
    pub fn lost(&self) -> Option<u64> {
        self.read_format
            .contains(ReadFormat::LOST)
            .then_some(self.lost)
    }
}

impl fmt::Debug for GroupEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dbg = f.debug_struct("GroupEntry");
        dbg.field("value", &self.value());
        dbg.field("id", &self.id());

        if let Some(lost) = self.lost() {
            dbg.field("lost", &lost);
        }

        dbg.finish_non_exhaustive()
    }
}

/// Iterator over the entries contained within [`GroupData`].
#[derive(Clone)]
pub struct GroupIter<'a> {
    read_format: ReadFormat,
    iter: std::slice::ChunksExact<'a, u64>,
}

impl<'a> GroupIter<'a> {
    fn new(read_format: ReadFormat, data: &'a [u64]) -> Self {
        Self {
            read_format,
            iter: data.chunks_exact(read_format.element_len()),
        }
    }
}

impl<'a> Iterator for GroupIter<'a> {
    type Item = GroupEntry;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|chunk| GroupEntry::new(self.read_format, chunk))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn count(self) -> usize {
        self.iter.count()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|chunk| GroupEntry::new(self.read_format, chunk))
    }

    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

impl<'a> DoubleEndedIterator for GroupIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter
            .next_back()
            .map(|chunk| GroupEntry::new(self.read_format, chunk))
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth_back(n)
            .map(|chunk| GroupEntry::new(self.read_format, chunk))
    }
}

impl<'a> ExactSizeIterator for GroupIter<'a> {}
impl<'a> FusedIterator for GroupIter<'a> {}
