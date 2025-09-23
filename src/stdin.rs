use std::{
    collections::VecDeque,
    io::{self, BufRead, BufReader, Read},
};

use itertools::Itertools;

///Checks if the given byte represents an ASCII character (not a part of a multibyte Unicode character).
//ref: |https://stackoverflow.com/questions/5012803/test-if-char-string-contains-multibyte-characters|
fn is_ascii(c: u8) -> bool {
    (c >> 7) == 0b0
}

/// Stdin reader which can read a single Unicode character.
pub struct Stdin {
    is_eof: bool,
    stdin: Box<dyn BufRead>, //`Box` is for dependency injection.
}

impl Stdin {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            is_eof: false,
            stdin: Box::new(BufReader::new(io::stdin())),
        }
    }

    //for dependency injection
    pub fn new_with_string(s: &str) -> Self {
        Self {
            is_eof: false,
            stdin: Box::new(VecDeque::from(s.to_string().into_bytes())),
        }
    }

    /// Peeks the next byte without consuming it.
    /// `None` is returned if EOF.
    ///
    /// This is also useful for testing.
    fn peek(&mut self) -> Option<u8> {
        if self.is_eof {
            return None;
        }

        let internal_buffer = self.stdin.as_mut().fill_buf().unwrap();
        if internal_buffer.is_empty() {
            self.is_eof = true;
            return None;
        }

        Some(internal_buffer[0])
    }

    /// Peeks the next byte from `stdin` and consumes it if it is an ASCII whitespace,
    /// in which case the consumed whitespace character is returned.
    ///
    /// `None` is returned if EOF or the next byte is not an ASCII whitespace.
    fn consume_next_whitespace(&mut self) -> Option<char> {
        let next: u8 = self.peek()?;

        if is_ascii(next) && (next as char).is_ascii_whitespace() {
            self.stdin.as_mut().consume(1);
            Some(next as char)
        } else {
            None
        }
    }

    /// Reads next Unicode character from `stdin` and returns it as `char` even if that is a whitespace.
    /// `None` is returned if EOF.
    //ref: |https://stackoverflow.com/questions/5012803/test-if-char-string-contains-multibyte-characters|
    //ref: |https://stackoverflow.com/questions/75873135/how-to-convert-utf-8-hex-value-to-char-in-rust|
    pub fn read_char(&mut self) -> Option<char> {
        if self.is_eof {
            return None;
        }
        let next = self.stdin.as_mut().bytes().next();
        if next.is_none() {
            self.is_eof = true;
            return None;
        }

        let c = next.unwrap().unwrap();

        //if ASCII
        if is_ascii(c) {
            return Some(c as char);
        }

        //if Unicode
        let mut l = vec![c];
        let num_bytes = if (c >> 5) == 0b110 {
            2
        } else if (c >> 4) == 0b1110 {
            3
        } else {
            assert_eq!(0b11110, c >> 3);
            4
        };
        for _ in 0..(num_bytes - 1) {
            l.push(self.stdin.as_mut().bytes().next().unwrap().unwrap());
        }
        Some(String::from_utf8(l).unwrap().chars().next().unwrap())
    }

    /// Reads next word.
    ///
    /// "word" is a series of characters and each word is separated by one or more whitespaces.
    ///
    /// `read_word()` eats and ignores the surrounding whitespace:
    /// - any amount of leading whitespace is eaten
    /// - any amount of trailing whitespace is eaten but until the first newline (this newline is also consumed)
    ///
    /// `None` is returned if EOF.
    fn read_word(&mut self) -> Option<String> {
        if self.is_eof {
            return None;
        }

        //eats preceding whitespace (if any)
        while self.consume_next_whitespace().is_some() {}

        //reads a word
        let mut l = vec![];
        while let Some(c) = self.read_char() {
            if c.is_ascii_whitespace() {
                //eats trailing whitespace (if any) but until the first newline
                //For example, when stdin has ` -50 \n hello`, `read_word()` consumes ` -50 \n` and leaves ` hello`.
                if c != '\n' {
                    loop {
                        let c = self.consume_next_whitespace();
                        if c.is_none() || c.unwrap() == '\n' {
                            break;
                        }
                    }
                }
                break;
            }
            l.push(c);
        }

        if l.is_empty() {
            self.is_eof = true;
            return None;
        }

        Some(l.into_iter().join(""))
    }

    /// Reads next signed integer.
    /// `None` is returned if EOF or parse error because [the spec](https://www.dangermouse.net/esoteric/piet.html) says
    /// > If an integer read does not receive an integer value, this is an error and the command is ignored.
    pub fn read_integer(&mut self) -> Option<isize> {
        self.read_word()?.parse().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peek() {
        let mut stdin = Stdin::new_with_string("abc");

        assert_eq!(Some(b'a'), stdin.peek());
        assert_eq!(Some(b'a'), stdin.peek());
        assert_eq!(Some('a'), stdin.read_char());

        assert_eq!(Some(b'b'), stdin.peek());
        assert_eq!(Some(b'b'), stdin.peek());
        assert_eq!(Some('b'), stdin.read_char());

        assert_eq!(Some(b'c'), stdin.peek());
        assert_eq!(Some(b'c'), stdin.peek());
        assert_eq!(Some('c'), stdin.read_char());

        assert_eq!(false, stdin.is_eof);
        assert_eq!(None, stdin.peek());
        assert_eq!(true, stdin.is_eof);

        assert_eq!(None, stdin.peek());
    }

    #[test]
    fn test_consume_next_whitespace() {
        let mut stdin = Stdin::new_with_string(" \t\n 1 ");
        assert_eq!(Some(' '), stdin.consume_next_whitespace());
        assert_eq!(Some('\t'), stdin.consume_next_whitespace());
        assert_eq!(Some('\n'), stdin.consume_next_whitespace());
        assert_eq!(Some(' '), stdin.consume_next_whitespace());
        assert_eq!(None, stdin.consume_next_whitespace());
        assert_eq!(Some('1'), stdin.read_char());
        assert_eq!(Some(' '), stdin.consume_next_whitespace());
        assert_eq!(false, stdin.is_eof);
        assert_eq!(None, stdin.consume_next_whitespace());
        assert_eq!(true, stdin.is_eof);
    }

    #[test]
    fn test_read_char() {
        let mut stdin = Stdin::new_with_string("ab c\nd");
        assert_eq!(Some('a'), stdin.read_char());
        assert_eq!(Some('b'), stdin.read_char());
        assert_eq!(Some(' '), stdin.read_char());
        assert_eq!(Some('c'), stdin.read_char());
        assert_eq!(Some('\n'), stdin.read_char());
        assert_eq!(Some('d'), stdin.read_char());
        assert_eq!(None, stdin.read_char());
    }

    #[test]
    fn test_read_word() {
        let mut stdin = Stdin::new_with_string(" -50 \n hello");
        assert_eq!(Some("-50".to_owned()), stdin.read_word());
        assert_eq!(Some(b' '), stdin.peek());
        assert_eq!(Some(' '), stdin.read_char());
        assert_eq!(Some(b'h'), stdin.peek());

        let mut stdin = Stdin::new_with_string(" -50 \n hello");
        assert_eq!(Some("-50".to_owned()), stdin.read_word());
        assert_eq!(Some("hello".to_owned()), stdin.read_word());
        assert_eq!(None, stdin.read_word());

        let mut stdin = Stdin::new_with_string(" -50 \n hello\n\nfoo\n\n\n");
        assert_eq!(Some("-50".to_owned()), stdin.read_word());
        assert_eq!(Some("hello".to_owned()), stdin.read_word());
        assert_eq!(Some("foo".to_owned()), stdin.read_word());
        assert_eq!(Some(b'\n'), stdin.peek());
        assert_eq!(None, stdin.read_word());
    }

    #[test]
    fn test_read_integer() {
        let mut stdin = Stdin::new_with_string("1 2 -3\n4 5\n");
        assert_eq!(Some(1), stdin.read_integer());
        assert_eq!(Some(2), stdin.read_integer());
        assert_eq!(Some(-3), stdin.read_integer());
        assert_eq!(Some(4), stdin.read_integer());
        assert_eq!(Some(5), stdin.read_integer());
        assert_eq!(None, stdin.read_char());

        let mut stdin = Stdin::new_with_string("30 hello");
        assert_eq!(Some(30), stdin.read_integer());
        assert_eq!(Some(b'h'), stdin.peek());

        let mut stdin = Stdin::new_with_string("30hello abc");
        //`read_integer()` fails and returns `None` as it tries to parse `30hello` as an integer.
        //This is different from how `std::cin >> {int}` in C++ works for example.
        assert_eq!(None, stdin.read_integer());
        assert_eq!(Some(b'a'), stdin.peek());
    }

    #[test]
    fn test_mixed() {
        //char followed by char
        let mut stdin = Stdin::new_with_string(" h a\t");
        assert_eq!(Some(' '), stdin.read_char());
        assert_eq!(Some('h'), stdin.read_char());
        assert_eq!(Some(' '), stdin.read_char());
        assert_eq!(Some('a'), stdin.read_char());
        assert_eq!(Some('\t'), stdin.read_char());
        assert_eq!(None, stdin.read_char());

        //integer followed by integer
        let mut stdin = Stdin::new_with_string(" -1 5\n\n3 ");
        assert_eq!(Some(-1), stdin.read_integer());
        assert_eq!(Some(b'5'), stdin.peek());
        assert_eq!(Some(5), stdin.read_integer());
        assert_eq!(Some(b'\n'), stdin.peek());
        assert_eq!(Some(3), stdin.read_integer());
        assert_eq!(None, stdin.peek());

        //char followed by integer
        let mut stdin = Stdin::new_with_string(" a 100\nb200\n\nx");
        assert_eq!(Some(' '), stdin.read_char());
        assert_eq!(Some('a'), stdin.read_char());
        assert_eq!(Some(b' '), stdin.peek());
        assert_eq!(Some(100), stdin.read_integer());
        assert_eq!(Some('b'), stdin.read_char());
        assert_eq!(Some(200), stdin.read_integer());
        assert_eq!(Some('\n'), stdin.read_char());
        assert_eq!(Some('x'), stdin.read_char());

        //integer followed by char
        let mut stdin = Stdin::new_with_string("100    a\n\n  200\n\nb");
        assert_eq!(Some(100), stdin.read_integer());
        assert_eq!(Some('a'), stdin.read_char());
        assert_eq!(Some(200), stdin.read_integer());
        assert_eq!(Some('\n'), stdin.read_char());
        assert_eq!(Some('b'), stdin.read_char());
    }

    #[test]
    fn test_mixed_ascii() {
        let mut stdin = Stdin::new_with_string(" he llo abc abc -100 15 a20   ");
        assert_eq!(Some(' '), stdin.read_char());
        assert_eq!(Some('h'), stdin.read_char());
        assert_eq!(Some('e'), stdin.read_char());
        assert_eq!(Some(' '), stdin.read_char());
        assert_eq!(Some('l'), stdin.read_char());
        assert_eq!(Some('l'), stdin.read_char());
        assert_eq!(Some('o'), stdin.read_char());
        assert_eq!(Some("abc".to_string()), stdin.read_word());
        assert_eq!(None, stdin.read_integer());
        assert_eq!(Some(-100), stdin.read_integer());
        assert_eq!(Some(15), stdin.read_integer());
        assert_eq!(Some('a'), stdin.read_char());
        assert_eq!(Some(20), stdin.read_integer());
        assert_eq!(None, stdin.read_char());
        assert_eq!(None, stdin.read_word());
    }

    #[test]
    fn test_mixed_unicode() {
        let mut stdin = Stdin::new_with_string(" こん にちは 🌙🌱🌸   🌷🍄  -100 15 a20  あa aあ");
        assert_eq!(Some(' '), stdin.read_char());
        assert_eq!(Some('こ'), stdin.read_char());
        assert_eq!(Some('ん'), stdin.read_char());
        assert_eq!(Some(' '), stdin.read_char());
        assert_eq!(Some('に'), stdin.read_char());
        assert_eq!(Some('ち'), stdin.read_char());
        assert_eq!(Some('は'), stdin.read_char());
        assert_eq!(Some(' '), stdin.read_char());
        assert_eq!(Some("🌙🌱🌸".to_string()), stdin.read_word());
        assert_eq!(None, stdin.read_integer());
        assert_eq!(Some(-100), stdin.read_integer());
        assert_eq!(Some(15), stdin.read_integer());
        assert_eq!(Some('a'), stdin.read_char());
        assert_eq!(Some(20), stdin.read_integer());
        assert_eq!(Some('あ'), stdin.read_char());
        assert_eq!(Some('a'), stdin.read_char());
        assert_eq!(Some(' '), stdin.read_char());
        assert_eq!(Some('a'), stdin.read_char());
        assert_eq!(Some("あ".to_owned()), stdin.read_word());
        assert_eq!(None, stdin.read_char());
        assert_eq!(None, stdin.read_word());
    }
}
