#[cfg(feature = "std")]
use std::io::Read;

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
        match self.reader.read() {
            Ok(c) => Some(c),
            Err(_) => None,
        }
    }
}
