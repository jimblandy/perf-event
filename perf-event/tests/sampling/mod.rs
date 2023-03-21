use std::fmt;

mod mmap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Hex<T>(T);

impl<T: fmt::UpperHex> fmt::Display for Hex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: fmt::UpperHex> fmt::Debug for Hex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("0x")?;
        self.0.fmt(f)
    }
}
