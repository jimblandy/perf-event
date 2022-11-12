use bytes::Buf;
use std::ffi::OsString;
use std::os::unix::prelude::OsStringExt;

use super::{Parse, ParseBuf, ParseConfig, RecordEvent};

/// MMAP events record memory mappings.
/// 
/// See the [manpage] for more documentation here.
///
/// [manpage]: http://man7.org/linux/man-pages/man2/perf_event_open.2.html
#[derive(Clone, Debug)]
pub struct Mmap {
    /// The process ID.
    pub pid: u32,

    /// The thread ID.
    pub tid: u32,

    /// The address of the memory mapping.
    pub addr: u64,

    /// The length of the memory mapping.
    pub len: u64,

    /// The page offset of the memory mapping.
    pub pgoff: u64,

    /// A string describing the backing of the memory mapping.
    ///
    /// For mappings of files this will be a file path.
    pub filename: OsString,
}

impl Parse for Mmap {
    fn parse<B: Buf>(_: &ParseConfig, buf: &mut B) -> Self
    where
        Self: Sized,
    {
        Self {
            pid: buf.get_u32_ne(),
            tid: buf.get_u32_ne(),
            addr: buf.get_u64_ne(),
            len: buf.get_u64_ne(),
            pgoff: buf.get_u64_ne(),
            filename: {
                let mut vec = buf.parse_remainder();

                // Remove padding nul bytes from the entry
                while let Some(b'\0') = vec.last() {
                    vec.pop();
                }

                OsString::from_vec(vec)
            },
        }
    }
}

impl From<Mmap> for RecordEvent {
    fn from(mmap: Mmap) -> Self {
        RecordEvent::Mmap(mmap)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(not(target_endian = "little"), ignore)]
    fn test_parse() {
        let mut bytes: &[u8] = &[
            10, 100, 0, 0, 11, 100, 0, 0, 0, 160, 118, 129, 189, 127, 0, 0, 0, 16, 0, 0, 0, 0, 0,
            0, 0, 160, 118, 129, 189, 127, 0, 0, 47, 47, 97, 110, 111, 110, 0, 0,
        ];

        let mmap = Mmap::parse(&ParseConfig::default(), &mut bytes);

        assert_eq!(mmap.pid, 25610);
        assert_eq!(mmap.tid, 25611);
        assert_eq!(mmap.addr, 0x7FBD8176A000);
        assert_eq!(mmap.len, 4096);
        assert_eq!(mmap.pgoff, 0x7FBD8176A000);
        assert_eq!(mmap.filename, "//anon");
    }
}
