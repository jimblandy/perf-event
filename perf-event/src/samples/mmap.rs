use bytes::Buf;
use std::ffi::OsString;
use std::os::unix::prelude::OsStringExt;

use super::{Parse, ParseBuf, ParseConfig, ParseError};

/// MMAP events record memory mappings.
/// 
/// This allows us to correlate user-space IPs to code.
#[derive(Clone, Debug)]
pub struct Mmap {
    /// The process ID.
    pub pid: u32,

    /// The thread ID.
    pub tid: u32,

    /// The address of the allocated memory.
    pub addr: u64,

    /// The length of the allocated memory.
    pub len: u64,

    /// The page offset of the allocated memory.
    pub pgoff: u64,

    /// A string describing the backing of the allocated memory.
    /// 
    /// For mappings of files this will be a file path.
    pub filename: OsString,
}

impl Parse for Mmap {
    fn parse<B: Buf>(_: &ParseConfig, buf: &mut B) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        Ok(Self {
            pid: buf.parse()?,
            tid: buf.parse()?,
            addr: buf.parse()?,
            len: buf.parse()?,
            pgoff: buf.parse()?,
            filename: OsString::from_vec(buf.parse_remainder()?),
        })
    }
}
