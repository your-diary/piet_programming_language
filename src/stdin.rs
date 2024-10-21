use std::{
    collections::VecDeque,
    io::{self, Read},
};

use itertools::Itertools;

/// Stdin reader which can read a single Unicode character.
pub struct Stdin {
    is_eof: bool,
    stdin: Box<dyn Read>, //`Box` is for dependency injection.
}

impl Stdin {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            is_eof: false,
            stdin: Box::new(io::stdin()),
        }
    }

    //for dependency injection
    pub fn new_with_string(s: &str) -> Self {
        Self {
            is_eof: false,
            stdin: Box::new(VecDeque::from(s.to_string().into_bytes())),
        }
    }

    /// Reads next Unicode character from `stdin` and returns it as `char` even if that is a whitespace.
    /// `None` is returned if EOF.
    //ref: |https://stackoverflow.com/questions/5012803/test-if-char-string-contains-multibyte-characters|
    //ref: |https://stackoverflow.com/questions/75873135/how-to-convert-utf-8-hex-value-to-char-in-rust|
    fn next(&mut self) -> Option<char> {
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
        if (c >> 7) == 0b0 {
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

    /// Reads next non-whitespace character.
    /// `None` is returned if EOF.
    pub fn read_char(&mut self) -> Option<char> {
        loop {
            let next = self.next()?;
            if !next.is_ascii_whitespace() {
                return Some(next);
            }
        }
    }

    /// Reads next word.
    /// "word" is a series of characters and each word is separated by one or more whitespaces.
    /// `None` is returned if EOF.
    fn read_word(&mut self) -> Option<String> {
        let mut l = vec![];

        //eats the preceding whitespace (if any) and reads the first character of a word
        loop {
            let next = self.next()?;
            if !next.is_ascii_whitespace() {
                l.push(next);
                break;
            }
        }

        //reads the remaining characters of a word
        loop {
            let next = self.next();
            if next.is_none() {
                break;
            }
            let next = next.unwrap();
            if next.is_ascii_whitespace() {
                break;
            }
            l.push(next);
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
    fn test_ascii() {
        let mut stdin = Stdin::new_with_string(" he llo abc abc -100 15 a20   ");
        assert_eq!(Some('h'), stdin.read_char());
        assert_eq!(Some('e'), stdin.read_char());
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
    fn test_unicode() {
        let mut stdin = Stdin::new_with_string(" こん にちは 🌙🌱🌸   🌷🍄  -100 15 a20  あa aあ");
        assert_eq!(Some('こ'), stdin.read_char());
        assert_eq!(Some('ん'), stdin.read_char());
        assert_eq!(Some('に'), stdin.read_char());
        assert_eq!(Some('ち'), stdin.read_char());
        assert_eq!(Some('は'), stdin.read_char());
        assert_eq!(Some("🌙🌱🌸".to_string()), stdin.read_word());
        assert_eq!(None, stdin.read_integer());
        assert_eq!(Some(-100), stdin.read_integer());
        assert_eq!(Some(15), stdin.read_integer());
        assert_eq!(Some('a'), stdin.read_char());
        assert_eq!(Some(20), stdin.read_integer());
        assert_eq!(Some('あ'), stdin.read_char());
        assert_eq!(Some('a'), stdin.read_char());
        assert_eq!(Some('a'), stdin.read_char());
        assert_eq!(Some("あ".to_owned()), stdin.read_word());
        assert_eq!(None, stdin.read_char());
        assert_eq!(None, stdin.read_word());
    }
}
