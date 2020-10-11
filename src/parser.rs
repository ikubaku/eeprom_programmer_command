use crate::scanner::Token;

#[derive(PartialEq, Debug)]
pub enum Command {
    ReadByte(u32),
    WriteByte(u32, u8),
    ReadData(u32, u32),
    WritePage(u16),
    SetDevice(DeviceName),
}

#[derive(PartialEq, Debug)]
pub enum DeviceName {
    X00,
    X01,
    X02,
    X04,
    X08,
    X16,
    X32,
    X64,
    X128,
    X256,
    X512,
    XM01,
    XM02,
}

pub struct Parser<R> {
    reader: R,
    scanner: crate::scanner::Scanner,
}

impl<R> Parser<R>
where R: crate::reader::Reader {
    pub fn new(reader: R) -> Parser<R> {
        Parser {
            reader,
            scanner: crate::scanner::Scanner::default(),
        }
    }

    fn get_token(&mut self) -> Result<Token, ()> {
        loop {
            let c = self.reader.read();
            if c.is_none() {
                return Err(());
            }

            let res = self.scanner.scan_command(c.unwrap());
            if res.is_some() {
                return Ok(res.unwrap());
            }
        }
    }

    pub fn destroy(self: Parser<R>) -> R {
        self.reader
    }

    pub fn parse_command(&mut self) -> Result<Command, ()> {
        let cmd = self.get_token()?;
        if cmd != Token::Identifier {
            Err(())
        } else {
            let cmd_str = self.scanner.scanned_string;
            if crate::util::u8_str_equal(&cmd_str, "rb\0".as_bytes()) {
                self.parse_read_byte()
            } else if crate::util::u8_str_equal(&cmd_str, "wb\0".as_bytes()) {
                self.parse_write_byte()
            } else if crate::util::u8_str_equal(&cmd_str, "rd\0".as_bytes()) {
                self.parse_read_data()
            } else if crate::util::u8_str_equal(&cmd_str, "wp\0".as_bytes()) {
                self.parse_write_page()
            } else if crate::util::u8_str_equal(&cmd_str, "sd\0".as_bytes()) {
                self.parse_set_device()
            } else {
                Err(())
            }
        }
    }

    fn parse_address(&mut self) -> Result<u32, ()> {
        let arg = self.get_token()?;
        if arg != Token::Number {
            Err(())
        } else {
            let addr = self.scanner.scanned_number;
            if addr < 0 {
                Err(())
            } else {
                Ok(addr as u32)
            }
        }
    }

    fn parse_data(&mut self) -> Result<u8, ()> {
        let arg = self.get_token()?;
        if arg != Token::Number {
            Err(())
        } else {
            let data = self.scanner.scanned_number;
            if data < 0 {
                if data >= -128 {
                    Ok((0x100 + data) as u8)
                } else {
                    Err(())
                }
            } else {
                if data < 256 {
                    Ok(data as u8)
                } else {
                    Err(())
                }
            }
        }
    }

    fn parse_length(&mut self) -> Result<u32, ()> {
        let arg = self.get_token()?;
        if arg != Token::Number {
            Err(())
        } else {
            let addr = self.scanner.scanned_number;
            if addr < 0 {
                Err(())
            } else {
                Ok(addr as u32)
            }
        }
    }

    fn parse_page(&mut self) -> Result<u16, ()> {
        let arg = self.get_token()?;
        if arg != Token::Number {
            Err(())
        } else {
            let page = self.scanner.scanned_number;
            if page < 0 || 1023 < page {
                Err(())
            } else {
                Ok(page as u16)
            }
        }
    }

    fn parse_device_name(&mut self) -> Result<DeviceName, ()> {
        let arg = self.get_token()?;
        if arg != Token::Identifier {
            Err(())
        } else {
            let device_name_str = self.scanner.scanned_string;
            if crate::util::u8_str_equal(&device_name_str, "x00\0".as_bytes()) {
                Ok(DeviceName::X00)
            } else if crate::util::u8_str_equal(&device_name_str, "x01\0".as_bytes()) {
                Ok(DeviceName::X01)
            } else if crate::util::u8_str_equal(&device_name_str, "x02\0".as_bytes()) {
                Ok(DeviceName::X02)
            } else if crate::util::u8_str_equal(&device_name_str, "x04\0".as_bytes()) {
                Ok(DeviceName::X04)
            } else if crate::util::u8_str_equal(&device_name_str, "x08\0".as_bytes()) {
                Ok(DeviceName::X08)
            } else if crate::util::u8_str_equal(&device_name_str, "x16\0".as_bytes()) {
                Ok(DeviceName::X16)
            } else if crate::util::u8_str_equal(&device_name_str, "x32\0".as_bytes()) {
                Ok(DeviceName::X32)
            } else if crate::util::u8_str_equal(&device_name_str, "x64\0".as_bytes()) {
                Ok(DeviceName::X64)
            } else if crate::util::u8_str_equal(&device_name_str, "x128\0".as_bytes()) {
                Ok(DeviceName::X128)
            } else if crate::util::u8_str_equal(&device_name_str, "x256\0".as_bytes()) {
                Ok(DeviceName::X256)
            } else if crate::util::u8_str_equal(&device_name_str, "x512\0".as_bytes()) {
                Ok(DeviceName::X512)
            } else if crate::util::u8_str_equal(&device_name_str, "xm01\0".as_bytes()) {
                Ok(DeviceName::XM01)
            } else if crate::util::u8_str_equal(&device_name_str, "xm02\0".as_bytes()) {
                Ok(DeviceName::XM02)
            } else {
                Err(())
            }
        }
    }

    fn parse_read_byte(&mut self) -> Result<Command, ()> {
        let addr = self.parse_address()?;
        if self.get_token()? == Token::Finish {
            Ok(Command::ReadByte(addr))
        } else {
            Err(())
        }
    }

    fn parse_write_byte(&mut self) -> Result<Command, ()> {
        let addr = self.parse_address()?;
        let data = self.parse_data()?;
        if self.get_token()? == Token::Finish {
            Ok(Command::WriteByte(addr, data))
        } else {
            Err(())
        }
    }

    fn parse_read_data(&mut self) -> Result<Command, ()> {
        let addr = self.parse_address()?;
        let len = self.parse_length()?;
        if self.get_token()? == Token::Finish {
            Ok(Command::ReadData(addr, len))
        } else {
            Err(())
        }
    }

    fn parse_write_page(&mut self) -> Result<Command, ()> {
        let page = self.parse_page()?;
        if self.get_token()? == Token::Finish {
            Ok(Command::WritePage(page))
        } else {
            Err(())
        }
    }

    fn parse_set_device(&mut self) -> Result<Command, ()> {
        let device_name = self.parse_device_name()?;
        if self.get_token()? == Token::Finish {
            Ok(Command::SetDevice(device_name))
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{Parser, DeviceName, Command};
    use crate::reader::StandardReader;

    #[test]
    fn parse_read_byte() {
        let command = "rb 0x000E3B41\r\n";
        let reader = StandardReader::new(command.as_bytes());
        let mut parser = Parser::new(reader);
        let res = parser.parse_command();

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Command::ReadByte(0x000E3B41));
    }

    #[test]
    fn parse_write_byte() {
        let command = "wb 0x00012000 0x42\r\n";
        let reader = StandardReader::new(command.as_bytes());
        let mut parser = Parser::new(reader);
        let res = parser.parse_command();

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Command::WriteByte(0x00012000, 0x42));
    }

    #[test]
    fn parse_read_data() {
        let command = "rd 0x00000010 32\r\n";
        let reader = StandardReader::new(command.as_bytes());
        let mut parser = Parser::new(reader);
        let res = parser.parse_command();

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Command::ReadData(0x00000010, 32));
    }

    #[test]
    fn parse_write_page() {
        let command = "wp 0x0F\r\n";
        let reader = StandardReader::new(command.as_bytes());
        let mut parser = Parser::new(reader);
        let res = parser.parse_command();

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Command::WritePage(0x0F));
    }

    #[test]
    fn parse_set_device() {
        let command = "sd xm01\r\n";
        let reader = StandardReader::new(command.as_bytes());
        let mut parser = Parser::new(reader);
        let res = parser.parse_command();

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Command::SetDevice(DeviceName::XM01));
    }
}
