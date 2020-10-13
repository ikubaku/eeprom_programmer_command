#[cfg(feature = "serial")]
use nb::block;

#[cfg(feature = "buffer")]
use core::convert::TryFrom;
#[cfg(feature = "buffer")]
use arrayvec::ArrayVec;
#[cfg(feature = "buffer")]
use std::convert::Infallible;

#[cfg(feature = "buffer")]
const BUFFER_READER_SIZE: usize = 32;

pub trait Reader {
    fn read(&mut self) -> Option<u8>;
}

#[cfg(feature = "std")]
pub struct StandardReader<R> {
    reader: R
}

#[cfg(feature = "std")]
impl<R> StandardReader<R>
    where R: std::io::Read {
    pub fn new(reader: R) -> StandardReader<R> {
        StandardReader {
            reader
        }
    }
    pub fn destroy(self) -> R {
        self.reader
    }
}

#[cfg(feature = "std")]
impl<R> Reader for StandardReader<R>
where R: std::io::Read {
    fn read(&mut self) -> Option<u8> {
        let mut c: [u8; 1] = [0];
        if self.reader.read_exact(&mut c).is_err() {
            return None
        }

        Some(c[0])
    }
}

#[cfg(feature = "serial")]
pub struct SerialReader<R> {
    reader: R
}

#[cfg(feature = "serial")]
impl<R> SerialReader<R>
where R: embedded_hal::serial::Read<u8> {
    pub fn new(reader: R) -> SerialReader<R> {
        SerialReader {
            reader
        }
    }
    pub fn destroy(self) -> R {
        self.reader
    }
}

#[cfg(feature = "serial")]
impl<R> Reader for SerialReader<R>
where R: embedded_hal::serial::Read<u8> {
    fn read(&mut self) -> Option<u8> {
        match block!(self.reader.read()) {
            Ok(c) => Some(c),
            Err(_) => None,
        }
    }
}

#[cfg(feature = "buffer")]
pub struct BufferReader<A> {
    reader: arrayvec::ArrayVec<A>,
}

#[cfg(feature = "buffer")]
impl<A> BufferReader<A>
where A: arrayvec::Array {
    pub fn try_new(buffer: &[A::Item]) -> Result<Self, ()> {
        // We need to push data into the arrayvec reversed because the ArrayVec::pop() works like a stack operation
        match arrayvec::ArrayVec::<[A; BUFFER_READER_SIZE]>::try_from(buffer.iter().rev().collect::<&[A::Item]>()) {
            Ok(av) => Ok(BufferReader { reader: av }),
            Err(_) => Err(()),
        }
    }
}

#[cfg(feature = "buffer")]
impl<A> Reader for BufferReader<A> {
    fn read(&mut self) -> Option<A> {
        self.reader.iter().next()
    }
}
