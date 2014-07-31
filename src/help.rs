use std::str::CharOffsets;
use std::io::IoResult;

use super::action::{IFlagAction, Help, ParseResult};

pub struct HelpAction;

impl IFlagAction for HelpAction {
    fn parse_flag(&self) -> ParseResult {
        return Help;
    }
}


struct WordsIter<'a> {
    data: &'a str,
    iter: CharOffsets<'a>,
}

impl<'a> WordsIter<'a> {
    fn new(data: &'a str) -> WordsIter<'a> {
        return WordsIter {
            data: data,
            iter: data.char_indices(),
            };
    }
}

impl<'a> Iterator<&'a str> for WordsIter<'a> {
    fn next(&mut self) -> Option<&'a str> {
        let mut word_start;
        loop {
            let (idx, ch) = match self.iter.next() {
                None => return None,
                Some((idx, ch)) => ((idx, ch)),
            };
            match ch {
                ' ' | '\t' | '\r' | '\n' => continue,
                _ => {
                    word_start = idx;
                    break;
                }
            }
        }
       for (idx, ch) in self.iter {
            match ch {
                ' ' | '\t' | '\r' | '\n' => {
                    return Some(self.data.slice(word_start, idx));
                }
                _ => continue,
            }
        }
        return Some(self.data.slice(word_start, self.data.len()));
    }
}

pub fn wrap_text(buf: &mut Writer, data: &str, width: uint, indent: uint)
    -> IoResult<()>
{
    let mut witer = WordsIter::new(data);
    let mut off = indent;
    match witer.next() {
        None => {
            return Ok(());
        }
        Some(word) => {
            try!(buf.write_str(word));
            off += word.len();
        }
    }
    for word in witer {
        if off + word.len() + 1 > width {
            try!(buf.write_char('\n'));
            for _ in range(0, indent) {
                try!(buf.write_char(' '));
            }
            off = indent;
        } else {
            try!(buf.write_char(' '));
            off += 1;
        }
        try!(buf.write_str(word));
        off += word.len();
    }
    return Ok(());
}


