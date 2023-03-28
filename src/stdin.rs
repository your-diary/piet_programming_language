use std::{
    collections::VecDeque,
    io::{self, Read},
};

use itertools::Itertools;

pub struct Stdin {
    is_eof: bool,
    stdin: Box<dyn Read>,
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

    //reads next byte from `stdin` and returns it as `char` even if that is a whitespace
    fn next(&mut self) -> Option<char> {
        if (self.is_eof) {
            return None;
        }
        let next = self.stdin.as_mut().bytes().next();
        if (next.is_none()) {
            self.is_eof = true;
            return None;
        }
        Some(next.unwrap().unwrap() as char)
    }

    //reads next non-whitespace character
    pub fn read_char(&mut self) -> Option<char> {
        loop {
            let next = self.next()?;
            if (!next.is_ascii_whitespace()) {
                return Some(next);
            }
        }
    }

    //reads next word
    //"word" is a series of characters and each word is separated by one or more whitespaces.
    fn read_word(&mut self) -> Option<String> {
        let mut l = vec![];

        //eats the preceding whitespace
        loop {
            let next = self.next()?;
            if (!next.is_ascii_whitespace()) {
                l.push(next);
                break;
            }
        }

        //reads the contents of a word
        loop {
            let next = self.next();
            if (next.is_none()) {
                break;
            }
            let next = next.unwrap();
            if (next.is_ascii_whitespace()) {
                break;
            }
            l.push(next);
        }

        Some(l.into_iter().join(""))
    }

    //reads next signed integer
    pub fn read_integer(&mut self) -> Option<isize> {
        self.read_word()?.parse().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
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
}
