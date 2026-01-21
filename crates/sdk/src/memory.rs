use anyhow::{bail, Result};

#[derive(Clone, Copy, Debug)]
pub enum Endian {
    Little,
}

pub trait MemoryReader {
    fn read_bytes(&self, address: u64, len: usize) -> Result<Vec<u8>>;

    fn read_u8(&self, address: u64) -> Result<u8> {
        Ok(self.read_bytes(address, 1)?[0])
    }

    fn read_u16(&self, address: u64, endian: Endian) -> Result<u16> {
        let buf = self.read_bytes(address, 2)?;
        Ok(match endian {
            Endian::Little => u16::from_le_bytes([buf[0], buf[1]]),
        })
    }

    fn read_u32(&self, address: u64, endian: Endian) -> Result<u32> {
        let buf = self.read_bytes(address, 4)?;
        Ok(match endian {
            Endian::Little => u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]),
        })
    }

    fn read_u64(&self, address: u64, endian: Endian) -> Result<u64> {
        let buf = self.read_bytes(address, 8)?;
        Ok(match endian {
            Endian::Little => u64::from_le_bytes([
                buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7],
            ]),
        })
    }

    fn read_ptr(&self, address: u64, pointer_size: u8, endian: Endian) -> Result<u64> {
        match pointer_size {
            4 => Ok(self.read_u32(address, endian)? as u64),
            8 => Ok(self.read_u64(address, endian)?),
            _ => bail!("unsupported pointer size {pointer_size}"),
        }
    }
}

pub trait MemoryWriter {
    fn write_bytes(&self, address: u64, data: &[u8]) -> Result<()>;
}

#[derive(Clone, Debug)]
pub struct MemoryView<'a, R: MemoryReader> {
    pub reader: &'a R,
    pub pointer_size: u8,
    pub endian: Endian,
}

impl<'a, R: MemoryReader> MemoryView<'a, R> {
    pub fn new(reader: &'a R, pointer_size: u8) -> Self {
        Self {
            reader,
            pointer_size,
            endian: Endian::Little,
        }
    }

    pub fn read_ptr(&self, address: u64) -> Result<u64> {
        self.reader.read_ptr(address, self.pointer_size, self.endian)
    }
}

#[derive(Clone, Debug)]
pub struct MockMemory {
    pub base: u64,
    pub data: Vec<u8>,
}

impl MockMemory {
    pub fn new(base: u64, data: Vec<u8>) -> Self {
        Self { base, data }
    }

    fn range(&self, address: u64, len: usize) -> Result<std::ops::Range<usize>> {
        if address < self.base {
            bail!("address below mock base");
        }
        let start = (address - self.base) as usize;
        let end = start.checked_add(len).ok_or_else(|| anyhow::anyhow!("overflow"))?;
        if end > self.data.len() {
            bail!("read out of bounds");
        }
        Ok(start..end)
    }
}

impl MemoryReader for MockMemory {
    fn read_bytes(&self, address: u64, len: usize) -> Result<Vec<u8>> {
        let range = self.range(address, len)?;
        Ok(self.data[range].to_vec())
    }
}

#[cfg(windows)]
mod win {
    use super::{Endian, MemoryReader};
    use anyhow::{bail, Result};
    use std::ptr;

    #[derive(Clone, Debug)]
    pub struct InProcessMemory;

    impl InProcessMemory {
        pub fn new() -> Self {
            Self
        }
    }

    impl MemoryReader for InProcessMemory {
        fn read_bytes(&self, address: u64, len: usize) -> Result<Vec<u8>> {
            if address == 0 {
                bail!("null address");
            }
            let mut out = vec![0u8; len];
            unsafe {
                let src = address as *const u8;
                ptr::copy_nonoverlapping(src, out.as_mut_ptr(), len);
            }
            Ok(out)
        }
    }

    impl InProcessMemory {
        pub fn read_u64_le(&self, address: u64) -> Result<u64> {
            self.read_u64(address, Endian::Little)
        }
    }

    pub use InProcessMemory as DefaultMemory;
}

#[cfg(windows)]
pub use win::DefaultMemory;
