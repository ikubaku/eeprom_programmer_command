const SCANNED_STRING_BUFFER_SIZE: usize = 16;

pub struct Scanner {
    state: ScannerState,
    scanned_string: [u8; SCANNED_STRING_BUFFER_SIZE],
    scanned_number: i32,
    scanned_number_sign: Sign,
}

pub enum ScannerState {
    Initial,
    Identifier,
    Finish,
    String,
    NumberWithSign,
    AnyNumber,
    Escape,
    StringEnd,
    DecimalNumber,
    BinaryNumber,
    OctalNumber,
    HexadecimalNumber,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier,
    String,
    Number,
    Finish,
    Invalid,
}

#[derive(PartialEq)]
enum Sign {
    Positive,
    Negative,
}

impl Scanner {
    pub fn default() -> Scanner {
        Scanner {
            state: ScannerState::Initial,
            scanned_string: [0; SCANNED_STRING_BUFFER_SIZE],
            scanned_number: 0,
            scanned_number_sign: Sign::Positive
        }
    }

    pub fn scan_command(self: &mut Scanner, c: u8) -> Option<Token> {
        match self.state {
            ScannerState::Initial => self.scan_when_initial(c),
            ScannerState::Identifier => self.scan_when_identifier(c),
            ScannerState::Finish => self.scan_when_finish(c),
            ScannerState::String => self.scan_when_string(c),
            ScannerState::NumberWithSign => self.scan_when_number_with_sign(c),
            ScannerState::AnyNumber => self.scan_when_any_number(c),
            ScannerState::Escape => self.scan_when_escape(c),
            ScannerState::StringEnd => self.scan_when_string_end(c),
            ScannerState::DecimalNumber => self.scan_when_decimal_number(c),
            ScannerState::BinaryNumber => self.scan_when_binary_number(c),
            ScannerState::OctalNumber => self.scan_when_octal_number(c),
            ScannerState::HexadecimalNumber => self.scan_when_hexadecimal_number(c),
        }
    }

    fn clear_scanned_number(self: &mut Scanner) {
        self.scanned_number = 0;
        self.scanned_number_sign = Sign::Positive;
    }

    fn clear_scanned_string(self: &mut Scanner) {
        self.scanned_string = [0; SCANNED_STRING_BUFFER_SIZE];
    }

    fn push_digit(self: &mut Scanner, d: u8, radix: u8) -> Result<(), ()> {
        match self.scanned_number.checked_mul(radix as i32)
            .and_then(|r| r.checked_add(d as i32)) {
            Some(new_value) => {
                self.scanned_number = new_value;
                Ok(())
            },
            None => Err(()),
        }
    }

    fn finalize_scanned_number(self: &mut Scanner) -> Result<(), ()> {
        if self.scanned_number_sign == Sign::Negative {
            match self.scanned_number.checked_mul(-1) {
                Some(new_value) => {
                    self.scanned_number = new_value;
                    Ok(())
                },
                None => Err(()),
            }
        } else {
            Ok(())
        }
    }

    fn push_char(self: &mut Scanner, c: u8) -> Result<(), ()> {
        for i in 0..SCANNED_STRING_BUFFER_SIZE-1 {
            if self.scanned_string[i] == b'\0' {
                self.scanned_string[i] = c;
                return Ok(());
            }
        }

        Err(())
    }

    fn scan_when_initial(self: &mut Scanner, c: u8) -> Option<Token> {
        if c == b' ' {
            None
        } else if c == b'\r' {
            self.state = ScannerState::Finish;
            None
        } else if c == b'-' {
            self.clear_scanned_number();
            self.scanned_number_sign = Sign::Negative;

            self.state = ScannerState::NumberWithSign;
            None
        } else if c == b'0' {
            self.clear_scanned_number();

            self.state = ScannerState::AnyNumber;
            None
        } else if b'1' <= c && c <= b'9' {
            self.clear_scanned_number();
            if self.push_digit(c - b'0', 10).is_err() {
                return Some(Token::Invalid);
            }

            self.state = ScannerState::DecimalNumber;
            None
        } else if c == b'\'' {
            self.clear_scanned_string();

            self.state = ScannerState::String;
            None
        } else if (b'a' <= c && c <= b'z') || (b'A' <= c && c <= b'Z') {
            self.clear_scanned_string();
            if self.push_char(c).is_err() {
                return Some(Token::Invalid);
            }

            self.state = ScannerState::Identifier;
            None
        } else {
            Some(Token::Invalid)
        }
    }

    fn scan_when_identifier(self: &mut Scanner, c: u8) -> Option<Token> {
        if c == b' ' {
            self.state = ScannerState::Initial;
            Some(Token::Identifier)
        } else if c == b'\r' {
            self.state = ScannerState::Finish;
            Some(Token::Identifier)
        } else if c == b'_' || (b'a' <= c && c <= b'z') || (b'A' <= c && c <= b'Z') || (b'0' <= c && c <= b'9') {
            if self.push_char(c).is_err() {
                return Some(Token::Invalid);
            }

            None
        } else {
            Some(Token::Invalid)
        }
    }

    fn scan_when_finish(self: &mut Scanner, c: u8) -> Option<Token> {
        if c == b'\n' {
            self.state = ScannerState::Initial;
            Some(Token::Finish)
        } else {
            Some(Token::Invalid)
        }
    }

    fn scan_when_string(self: &mut Scanner, c: u8) -> Option<Token> {
        if c == b'\\' {
            self.state = ScannerState::Escape;
            None
        } else if c == b'\'' {
            self.state = ScannerState::StringEnd;
            None
        } else if 0x20 <= c && c <= 0x7E {
            if self.push_char(c).is_err() {
                return Some(Token::Invalid);
            }

            None
        } else {
            Some(Token::Invalid)
        }
    }

    fn scan_when_number_with_sign(self: &mut Scanner, c: u8) -> Option<Token> {
        if c == b'0' {
            self.state = ScannerState::AnyNumber;
            None
        } else if b'0' <= c && c <= b'9' {
            if self.push_digit(c - b'0', 10).is_err() {
                return Some(Token::Invalid);
            }

            self.state = ScannerState::DecimalNumber;
            None
        } else {
            Some(Token::Invalid)
        }
    }

    fn scan_when_any_number(self: &mut Scanner, c: u8) -> Option<Token> {
        if c == b' ' {
            if self.finalize_scanned_number().is_err() {
                return Some(Token::Invalid);
            }

            self.state = ScannerState::Initial;
            Some(Token::Number)
        } else if c == b'\r' {
            if self.finalize_scanned_number().is_err() {
                return Some(Token::Invalid);
            }

            self.state = ScannerState::Finish;
            Some(Token::Number)
        } else if c == b'b' {
            self.state = ScannerState::BinaryNumber;
            None
        } else if c == b'o' {
            self.state = ScannerState::OctalNumber;
            None
        } else if c == b'x' {
            self.state = ScannerState::HexadecimalNumber;
            None
        } else if b'0' <= c && c <= b'9' {
            if self.push_digit(c - b'0', 10).is_err() {
                return Some(Token::Invalid);
            }

            self.state = ScannerState::DecimalNumber;
            None
        } else if c == b'd' {
            self.state = ScannerState::DecimalNumber;
            None
        } else {
            Some(Token::Invalid)
        }
    }

    fn scan_when_escape(self: &mut Scanner, c: u8) -> Option<Token> {
        if 0x20 <= c && c <= 0x7E {
            if self.push_char(c).is_err() {
                return Some(Token::Invalid);
            }

            self.state = ScannerState::String;
            None
        } else {
            Some(Token::Invalid)
        }
    }

    fn scan_when_string_end(self: &mut Scanner, c: u8) -> Option<Token> {
        if c == b' ' {
            self.state = ScannerState::Initial;
            Some(Token::String)
        } else if c == b'\r' {
            self.state = ScannerState::Finish;
            Some(Token::String)
        } else {
            Some(Token::Invalid)
        }
    }

    fn scan_when_decimal_number(self: &mut Scanner, c: u8) -> Option<Token> {
        if c == b' ' {
            if self.finalize_scanned_number().is_err() {
                return Some(Token::Invalid);
            }

            self.state = ScannerState::Initial;
            Some(Token::Number)
        } else if c == b'\r' {
            if self.finalize_scanned_number().is_err() {
                return Some(Token::Invalid);
            }

            self.state = ScannerState::Finish;
            Some(Token::Number)
        } else if b'0' <= c && c <= b'9' {
            if self.push_digit(c - b'0', 10).is_err() {
                return Some(Token::Invalid);
            }

            None
        } else {
            Some(Token::Invalid)
        }
    }

    fn scan_when_binary_number(self: &mut Scanner, c: u8) -> Option<Token> {
        if c == b' ' {
            if self.finalize_scanned_number().is_err() {
                return Some(Token::Invalid);
            }

            self.state = ScannerState::Initial;
            Some(Token::Number)
        } else if c == b'\r' {
            if self.finalize_scanned_number().is_err() {
                return Some(Token::Invalid);
            }

            self.state = ScannerState::Finish;
            Some(Token::Number)
        } else if c == b'0' || c == b'1' {
            if self.push_digit(c - b'0', 2).is_err() {
                return Some(Token::Invalid);
            }

            None
        } else {
            Some(Token::Invalid)
        }
    }

    fn scan_when_octal_number(self: &mut Scanner, c: u8) -> Option<Token> {
        if c == b' ' {
            if self.finalize_scanned_number().is_err() {
                return Some(Token::Invalid);
            }

            self.state = ScannerState::Initial;
            Some(Token::Number)
        } else if c == b'\r' {
            if self.finalize_scanned_number().is_err() {
                return Some(Token::Invalid);
            }

            self.state = ScannerState::Finish;
            Some(Token::Number)
        } else if b'0' <= c && c <= b'7' {
            if self.push_digit(c - b'0', 8).is_err() {
                return Some(Token::Invalid);
            }

            None
        } else {
            Some(Token::Invalid)
        }
    }

    fn scan_when_hexadecimal_number(self: &mut Scanner, c: u8) -> Option<Token> {
        if c == b' ' {
            if self.finalize_scanned_number().is_err() {
                return Some(Token::Invalid);
            }

            self.state = ScannerState::Initial;
            Some(Token::Number)
        } else if c == b'\r' {
            if self.finalize_scanned_number().is_err() {
                return Some(Token::Invalid);
            }

            self.state = ScannerState::Finish;
            Some(Token::Number)
        } else if b'0' <= c && c <= b'9' {
            if self.push_digit(c - b'0', 16).is_err() {
                return Some(Token::Invalid);
            }

            None
        } else if b'a' <= c && c <= b'f' {
            if self.push_digit(c - b'a' + 10, 16).is_err() {
                return Some(Token::Invalid);
            }

            None
        } else if b'A' <= c && c <= b'F' {
            if self.push_digit(c - b'A' + 10, 16).is_err() {
                return Some(Token::Invalid);
            }

            None
        } else {
            Some(Token::Invalid)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::scanner::{Scanner, Token};

    fn expect_first_token(scanner: &mut Scanner, input: &str, expected: Token) {
        for c in input.as_bytes() {
            let res = scanner.scan_command(*c);

            if res.is_some() {
                assert_eq!(res.unwrap(), expected);
                return;
            }
        }

        panic!("Should yield at least one token!");
    }

    fn expect_scanned_string(scanner: &Scanner, expected: &str) {
        let expected_slice = expected.as_bytes();
        for i in 0..expected_slice.len() {
            assert_eq!(scanner.scanned_string[i], expected_slice[i]);
        }
        assert_eq!(scanner.scanned_string[expected_slice.len()], b'\0');
    }

    #[test]
    fn scan_binary_number() {
        let mut scanner = Scanner::default();
        expect_first_token(&mut scanner, "0b1010\r\n", Token::Number);
        assert_eq!(scanner.scanned_number, 0b1010);
    }

    #[test]
    fn scan_octal_number() {
        let mut scanner = Scanner::default();
        expect_first_token(&mut scanner, "0o755\r\n", Token::Number);
        assert_eq!(scanner.scanned_number, 0o755);
    }

    #[test]
    fn scan_hexadecimal_number() {
        let mut scanner = Scanner::default();
        expect_first_token(&mut scanner, "0x1234ABCD\r\n", Token::Number);
        assert_eq!(scanner.scanned_number, 0x1234ABCD);
    }

    #[test]
    fn scan_decimal_number_0() {
        let mut scanner = Scanner::default();
        expect_first_token(&mut scanner, "123\r\n", Token::Number);
        assert_eq!(scanner.scanned_number, 123);
    }

    #[test]
    fn scan_decimal_number_1() {
        let mut scanner = Scanner::default();
        expect_first_token(&mut scanner, "0109\r\n", Token::Number);
        assert_eq!(scanner.scanned_number, 109);
    }

    #[test]
    fn scan_decimal_number_2() {
        let mut scanner = Scanner::default();
        expect_first_token(&mut scanner, "0\r\n", Token::Number);
        assert_eq!(scanner.scanned_number, 0);
    }

    #[test]
    fn scan_decimal_number_3() {
        let mut scanner = Scanner::default();
        expect_first_token(&mut scanner, "3\r\n", Token::Number);
        assert_eq!(scanner.scanned_number, 3);
    }

    #[test]
    fn scan_decimal_number_4() {
        let mut scanner = Scanner::default();
        expect_first_token(&mut scanner, "-128\r\n", Token::Number);
        assert_eq!(scanner.scanned_number, -128);
    }

    #[test]
    fn scan_negative_number() {
        let mut scanner = Scanner::default();
        expect_first_token(&mut scanner, "-0x42\r\n", Token::Number);
        assert_eq!(scanner.scanned_number, -0x42);
    }

    #[test]
    fn scan_identifier_0() {
        let mut scanner = Scanner::default();
        expect_first_token(&mut scanner, "abcde\r\n", Token::Identifier);
        expect_scanned_string(&scanner, "abcde");
    }

    #[test]
    fn scan_identifier_1() {
        let mut scanner = Scanner::default();
        expect_first_token(&mut scanner, "hoge_\r\n", Token::Identifier);
        expect_scanned_string(&scanner, "hoge_");
    }

    #[test]
    fn scan_finish_0() {
        let mut scanner = Scanner::default();
        expect_first_token(&mut scanner, "\r\n\r\n", Token::Finish);
    }

    #[test]
    fn scan_string_0() {
        let mut scanner = Scanner::default();
        expect_first_token(&mut scanner, "'Hello, world!'\r\n", Token::String);
        expect_scanned_string(&scanner, "Hello, world!");
    }

    #[test]
    fn scan_string_1() {
        let mut scanner = Scanner::default();
        expect_first_token(&mut scanner, "'That\\'s Right!'\r\n", Token::String);
        expect_scanned_string(&scanner, "That's Right!");
    }
}
