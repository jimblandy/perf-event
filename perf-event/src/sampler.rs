use std::borrow::Cow;
use std::os::fd::{AsRawFd, IntoRawFd, RawFd};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use crate::sys::bindings::{perf_event_header, perf_event_mmap_page};
use crate::{check_errno_syscall, Counter};

/// A sampled perf event.
///
/// A sampler for a sampler perf event consists of two things: a [`Counter`],
/// and a memory-mapped ring buffer into which the kernel periodically writes
/// events. The specific event is configured on construction and can vary from
/// changes to the memory mapping associated with a process, to sampling call
/// stacks, to getting the output from a bpf program running in the kernel, and
/// more.
///
/// This sampler type provides direct access to the bytes written by the kernel
/// without doing any parsing of the emitted records. To actually read the
/// involved fields you will need to parse them yourself. See the
/// [`perf_event_open` man page][0] for documentation on how the sample records
/// are represented in memory.
///
/// [0]: https://man7.org/linux/man-pages/man2/perf_event_open.2.html
pub struct Sampler {
    // Used by the crate::counter_impl! macro
    pub(crate) counter: Counter,
    mmap: memmap2::MmapRaw,
}

/// A view into a [`Sampler`]'s ring buffer for a single kernel event record.
///
/// When dropped, this type will advance the tail pointer in the ringbuffer of
/// the [`Sampler`] that it references. To avoid this, you can use
/// [`std::mem::forget`] so the next call to `Sampler::next_record` will return
/// the same record again.
pub struct Record<'a> {
    page: *const perf_event_mmap_page,
    header: perf_event_header,
    data: ByteBuffer<'a>,
}

/// A `Buf` that can be either a single byte slice or two disjoint byte
/// slices.
#[derive(Copy, Clone)]
enum ByteBuffer<'a> {
    Single(&'a [u8]),
    Split([&'a [u8]; 2]),
}

impl Sampler {
    pub(crate) fn new(counter: Counter, mmap: memmap2::MmapRaw) -> Self {
        Self { counter, mmap }
    }

    /// Convert this sampler back into a counter.
    ///
    /// This will close the ringbuffer associated with the sampler.
    pub fn into_counter(self) -> Counter {
        self.counter
    }

    /// Access the underlying counter for this sampler.
    pub fn as_counter(&self) -> &Counter {
        &self.counter
    }

    /// Mutably access the underlying counter for this sampler.
    pub fn as_counter_mut(&mut self) -> &mut Counter {
        &mut self.counter
    }

    /// Read the next record from the ring buffer.
    ///
    /// This method does not block. If you want blocking behaviour, use
    /// [`next_blocking`] instead.
    ///
    /// It is possible to get readiness notifications for when events are
    /// present in the ring buffer (e.g. for async code). See the documentation
    /// on the [`perf_event_open`][man] manpage for details on how to do this.
    ///
    /// [`next_blocking`]: Self::next_blocking
    /// [man]: https://man7.org/linux/man-pages/man2/perf_event_open.2.html
    pub fn next_record(&mut self) -> Option<Record> {
        use memoffset::raw_field;
        use std::{mem, ptr, slice};

        let page = self.page();

        // SAFETY:
        // - page points to a valid instance of perf_event_mmap_page.
        // - data_tail is only written by the user side so it is safe to do a
        //   non-atomic read here.
        let tail = unsafe { ptr::read(raw_field!(page, perf_event_mmap_page, data_tail)) };
        // ATOMICS:
        // - The acquire load here syncronizes with the release store in the
        //   kernel and ensures that all the data written to the ring buffer
        //   before data_head is visible to this thread.
        // SAFETY:
        // - page points to a valid instance of perf_event_mmap_page.
        let head = unsafe {
            atomic_load(
                raw_field!(page, perf_event_mmap_page, data_head),
                Ordering::Acquire,
            )
        };

        if tail == head {
            return None;
        }

        // SAFETY: (for both statements)
        // - page points to a valid instance of perf_event_mmap_page.
        // - neither of these fields are written to except before the map is
        //   created so reading from them non-atomically is safe.
        let data_size = unsafe { ptr::read(raw_field!(page, perf_event_mmap_page, data_size)) };
        let data_offset = unsafe { ptr::read(raw_field!(page, perf_event_mmap_page, data_offset)) };

        let mod_tail = (tail % data_size) as usize;
        let mod_head = (head % data_size) as usize;

        // SAFETY:
        // - perf_event_open guarantees that page.data_offset is within the
        //   memory mapping.
        let data_start = unsafe { self.mmap.as_ptr().add(data_offset as usize) };
        // SAFETY:
        // - data_start is guaranteed to be valid for at least data_size bytes.
        let tail_start = unsafe { data_start.add(mod_tail) };

        let mut buffer = if mod_head > mod_tail {
            ByteBuffer::Single(unsafe { slice::from_raw_parts(tail_start, mod_head - mod_tail) })
        } else {
            ByteBuffer::Split([
                unsafe { slice::from_raw_parts(tail_start, data_size as usize - mod_tail) },
                unsafe { slice::from_raw_parts(data_start, mod_head) },
            ])
        };

        let header = buffer.parse_header();
        assert!(header.size as usize >= mem::size_of::<perf_event_header>());
        buffer.truncate(header.size as usize - mem::size_of::<perf_event_header>());

        Some(Record {
            page: self.page(),
            header,
            data: buffer,
        })
    }

    /// Read the next record from the ring buffer. This method will block (with
    /// an optional timeout) until a new record is available.
    ///
    /// If this sampler is only enabled for a single process and that process
    /// exits, this method will return `None` even if no timeout is passed.
    /// Note that this only works on Linux 3.18 and above.
    ///
    /// # Panics
    /// This method will panic if an unexpected error is returned from
    /// `libc::poll`. There are only two cases where this can happen:
    /// - the current process has run out of file descriptors, or,
    /// - the kernel couldn't allocate memory for internal poll datastructures.
    pub fn next_blocking(&mut self, timeout: Option<Duration>) -> Option<Record> {
        let deadline = timeout.map(|timeout| Instant::now() + timeout);

        loop {
            if let Some(record) = self.next_record() {
                // This is a workaround for a known limitation of NLL in rustc.
                // If it worked, we could do
                //    return Some(record);
                // but currently that extends the lifetime for the &mut self
                // borrow to cover the whole function and that causes conflicts
                // with other borrows further down.
                //
                // Fixing this is tracked in the following rustc issue
                // https://github.com/rust-lang/rust/issues/51132
                //
                // You can verify that the code above should, in fact, pass the
                // borrow checker by removing the line below, uncommenting the
                // line above, and checking it via
                //     cargo +nightly rustc -- -Zpolonius
                return Some(unsafe { std::mem::transmute::<Record, Record>(record) });
            }

            let timeout = match deadline {
                Some(deadline) => deadline
                    .checked_duration_since(Instant::now())?
                    .as_millis()
                    .min(libc::c_int::MAX as u128) as libc::c_int,
                None => -1,
            };

            let mut pollfd = libc::pollfd {
                fd: self.as_raw_fd(),
                events: libc::POLLIN,
                revents: 0,
            };

            match check_errno_syscall(|| unsafe { libc::poll(&mut pollfd, 1, timeout) }) {
                // poll timed out.
                Ok(0) => return None,
                // The sampler was tracking a single other process and that
                // process has exited.
                //
                // However, there may still be events in the ring buffer in this case so
                // we still need to check.
                Ok(_) if pollfd.revents & libc::POLLHUP != 0 => return self.next_record(),
                // Must be POLLIN, there should be an event ready.
                Ok(_) => continue,
                Err(e) => match e.raw_os_error() {
                    Some(libc::EINTR) => continue,
                    // The only other possible kernel errors here are so rare
                    // that it doesn't make sense to make this API have a
                    // result because of them. To whit, they are:
                    // - EINVAL - the process ran out of file descriptors
                    // - ENOMEM - the kernel couldn't allocate memory for the
                    //            poll datastructures.
                    // In this case, we panic.
                    _ => panic!(
                        "polling a perf-event fd returned an unexpected error: {}",
                        e
                    ),
                },
            }
        }
    }

    fn page(&self) -> *const perf_event_mmap_page {
        self.mmap.as_ptr() as *const _
    }
}

impl std::convert::AsRef<Counter> for Sampler {
    fn as_ref(&self) -> &Counter {
        &self.counter
    }
}

impl std::convert::AsMut<Counter> for Sampler {
    fn as_mut(&mut self) -> &mut Counter {
        &mut self.counter
    }
}

impl AsRawFd for Sampler {
    fn as_raw_fd(&self) -> RawFd {
        self.counter.as_raw_fd()
    }
}

impl IntoRawFd for Sampler {
    fn into_raw_fd(self) -> RawFd {
        self.counter.into_raw_fd()
    }
}

impl<'s> Record<'s> {
    /// Access the `type` field of the kernel record header.
    ///
    /// This indicates the type of the record emitted by the kernel.
    pub fn ty(&self) -> u32 {
        self.header.type_
    }

    /// Access the `misc` field of the kernel record header.
    ///
    /// This contains a set of flags carry some additional metadata on the
    /// record being emitted by the kernel.
    pub fn misc(&self) -> u16 {
        self.header.misc
    }

    /// Get the total length, in bytes, of this record.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Access the bytes of this record.
    ///
    /// Since the underlying buffer is a ring buffer the bytes of the record
    /// may end up wrapping around the end of the buffer. That gets exposed
    /// here as data returning either one or two byte slices. If there is no
    /// wrap-around then one slice will be returned here, otherwise, two will
    /// be returned.
    pub fn data(&self) -> &[&[u8]] {
        match &self.data {
            ByteBuffer::Single(buf) => std::slice::from_ref(buf),
            ByteBuffer::Split(bufs) => &bufs[..],
        }
    }

    /// Copy the bytes of this record to an owned [`Vec`].
    pub fn to_vec(&self) -> Vec<u8> {
        self.to_contiguous().into_owned()
    }

    /// Get the bytes of this record as a single contiguous slice.
    ///
    /// For most records this is effectively free but if the record wraps
    /// around the end of the ringbuffer then it will be copied to a vector.
    pub fn to_contiguous(&self) -> Cow<[u8]> {
        match self.data {
            ByteBuffer::Single(data) => Cow::Borrowed(data),
            ByteBuffer::Split([a, b]) => {
                let mut vec = Vec::with_capacity(a.len() + b.len());
                vec.extend_from_slice(a);
                vec.extend_from_slice(b);
                Cow::Owned(vec)
            }
        }
    }
}

impl<'s> Drop for Record<'s> {
    fn drop(&mut self) {
        use memoffset::raw_field;
        use std::ptr;

        unsafe {
            // SAFETY:
            // - page points to a valid instance of perf_event_mmap_page
            // - data_tail is only written on our side so it is safe to do a
            //   non-atomic read here.
            let tail = ptr::read(raw_field!(self.page, perf_event_mmap_page, data_tail));

            // ATOMICS:
            // - The release store here prevents the compiler from re-ordering
            //   any reads past the store to data_tail.
            // SAFETY:
            // - page points to a valid instance of perf_event_mmap_page
            atomic_store(
                raw_field!(self.page, perf_event_mmap_page, data_tail),
                tail + (self.header.size as u64),
                Ordering::Release,
            );
        }
    }
}

// Record contains a pointer which prevents it from implementing Send or Sync
// by default. It is, however, valid to send it across threads and it has no
// interior mutability so we implement Send and Sync here manually.
unsafe impl<'s> Sync for Record<'s> {}
unsafe impl<'s> Send for Record<'s> {}

impl<'a> ByteBuffer<'a> {
    /// Parse an instance of `perf_event_header` out of the start of this
    /// byte buffer.
    fn parse_header(&mut self) -> perf_event_header {
        let mut bytes = [0; std::mem::size_of::<perf_event_header>()];
        self.copy_to_slice(&mut bytes);
        // SAFETY: perf_event_header is a packed C struct so it is valid to
        //         copy arbitrary initialized memory into it.
        unsafe { std::mem::transmute(bytes) }
    }

    fn len(&self) -> usize {
        match self {
            Self::Single(buf) => buf.len(),
            Self::Split([a, b]) => a.len() + b.len(),
        }
    }

    /// Shorten this byte buffer to only include the first `new_len` bytes.
    ///
    /// # Panics
    /// Panics if `new_len > self.len()`.
    fn truncate(&mut self, new_len: usize) {
        assert!(new_len <= self.len());

        *self = match *self {
            Self::Single(buf) => Self::Single(&buf[..new_len]),
            Self::Split([a, b]) => {
                if a.len() <= new_len {
                    Self::Single(&a[..new_len])
                } else {
                    Self::Split([a, &b[..new_len - a.len()]])
                }
            }
        }
    }

    /// Copy bytes from within this byte buffer to the provided slice.
    ///
    /// This will also remove those same bytes from the front of this byte
    /// buffer.
    ///
    /// # Panics
    /// Panics if `self.len() < dst.len()`
    fn copy_to_slice(&mut self, dst: &mut [u8]) {
        assert!(self.len() >= dst.len());

        match self {
            Self::Single(buf) => {
                let (head, rest) = buf.split_at(dst.len());
                dst.copy_from_slice(head);
                *buf = rest;
            }
            Self::Split([buf, _]) if buf.len() >= dst.len() => {
                let (head, rest) = buf.split_at(dst.len());
                dst.copy_from_slice(head);
                *buf = rest;
            }
            &mut Self::Split([a, b]) => {
                let (d_head, d_rest) = dst.split_at_mut(a.len());
                let (b_head, b_rest) = b.split_at(d_rest.len());

                d_head.copy_from_slice(a);
                d_rest.copy_from_slice(b_head);
                *self = Self::Single(b_rest);
            }
        }
    }
}

/// Do an atomic write to the value stored at `ptr`.
///
/// # Safety
/// - `ptr` must be valid for writes.
/// - `ptr` must be properly aligned.
unsafe fn atomic_store(ptr: *const u64, val: u64, order: Ordering) {
    (*(ptr as *const AtomicU64)).store(val, order)
}

/// Perform an atomic read from the value stored at `ptr`.
///
/// # Safety
/// - `ptr` must be valid for reads.
/// - `ptr` must be properly aligned.
unsafe fn atomic_load(ptr: *const u64, order: Ordering) -> u64 {
    (*(ptr as *const AtomicU64)).load(order)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buf_copy_over_split() {
        let mut out = [0; 7];
        let mut buf = ByteBuffer::Split([b"aaaaaa", b"bbbbb"]);
        buf.copy_to_slice(&mut out);
        assert_eq!(&out, b"aaaaaab");
        assert_eq!(buf.len(), 4);
    }

    #[test]
    fn buf_copy_to_split() {
        let mut out = [0; 6];
        let mut buf = ByteBuffer::Split([b"aaaaaa", b"bbbbb"]);
        buf.copy_to_slice(&mut out);

        assert_eq!(&out, b"aaaaaa");
        assert_eq!(buf.len(), 5);
    }

    #[test]
    fn buf_copy_before_split() {
        let mut out = [0; 5];
        let mut buf = ByteBuffer::Split([b"aaaaaa", b"bbbbb"]);
        buf.copy_to_slice(&mut out);

        assert_eq!(&out, b"aaaaa");
        assert_eq!(buf.len(), 6);
    }
}
