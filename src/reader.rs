#[cfg(feature = "serial")]
use nb::block;

#[cfg(feature = "buffer")]
use arrayvec::ArrayVec;
#[cfg(feature = "buffer")]
use core::convert::Infallible;
#[cfg(feature = "buffer")]
use core::convert::TryFrom;

#[cfg(feature = "buffer")]
const BUFFER_READER_SIZE: usize = 32;

pub trait Reader {
    fn read(&mut self) -> Option<u8>;
}

#[cfg(feature = "std")]
pub struct StandardReader<R> {
    reader: R,
}

#[cfg(feature = "std")]
impl<R> StandardReader<R>
where
    R: std::io::Read,
{
    pub fn new(reader: R) -> StandardReader<R> {
        StandardReader { reader }
    }
    pub fn destroy(self) -> R {
        self.reader
    }
}

#[cfg(feature = "std")]
impl<R> Reader for StandardReader<R>
where
    R: std::io::Read,
{
    fn read(&mut self) -> Option<u8> {
        let mut c: [u8; 1] = [0];
        if self.reader.read_exact(&mut c).is_err() {
            return None;
        }

        Some(c[0])
    }
}

#[cfg(feature = "serial")]
pub struct SerialReader<R> {
    reader: R,
}

#[cfg(feature = "serial")]
impl<R> SerialReader<R>
where
    R: embedded_hal::serial::Read<u8>,
{
    pub fn new(reader: R) -> SerialReader<R> {
        SerialReader { reader }
    }
    pub fn destroy(self) -> R {
        self.reader
    }
}

#[cfg(feature = "serial")]
impl<R> Reader for SerialReader<R>
where
    R: embedded_hal::serial::Read<u8>,
{
    fn read(&mut self) -> Option<u8> {
        match block!(self.reader.read()) {
            Ok(c) => Some(c),
            Err(_) => None,
        }
    }
}

#[cfg(feature = "buffer")]
pub struct BufferReader {
    reader: arrayvec::ArrayVec<[u8; 32]>,
}

#[cfg(feature = "buffer")]
impl BufferReader {
    pub fn try_new(buffer: &[u8]) -> Result<Self, ()> {
        // We need to push data into the arrayvec reversed because the ArrayVec::pop() works like a stack operation
        if buffer.len() > BUFFER_READER_SIZE {
            Err(())
        } else {
            let mut av = arrayvec::ArrayVec::<[u8; BUFFER_READER_SIZE]>::new();
            for c in buffer.iter().rev() {
                av.push(*c);
            }
            Ok(BufferReader { reader: av })
        }
    }
}

#[cfg(feature = "buffer")]
impl Reader for BufferReader {
    fn read(&mut self) -> Option<u8> {
        self.reader.pop()
    }
}
