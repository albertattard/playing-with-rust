#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

#[derive(Debug)]
pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimiter: D,
}

impl<'haystack, D> StrSplit<'haystack, D> {
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where
    D: Delimiter,
{
    type Item = &'haystack str;

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some((delimiter_start, delimiter_end)) = self.delimiter.find_next(remainder) {
            let head = &remainder[..delimiter_start];
            *remainder = &remainder[delimiter_end..];
            Some(head)
        } else {
            self.remainder.take()
        }
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + self.len_utf8()))
    }
}

fn sub_string_before_str<'haystack>(s: &'haystack str, delimiter: &str) -> &'haystack str {
    StrSplit::new(s, delimiter)
        .next()
        .expect("StrSplit always return at least one result")
}

fn sub_string_before_char(s: &str, delimiter: char) -> &str {
    StrSplit::new(s, delimiter)
        .next()
        .expect("StrSplit always return at least one result")
}

#[cfg(test)]
mod tests {
    use crate::str_split_2::{sub_string_before_char, sub_string_before_str, StrSplit};

    #[test]
    fn split_empty_string() {
        let haystack = "";
        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec![""]);
    }

    #[test]
    fn split_string() {
        let haystack = "a b c";
        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c"]);
    }

    #[test]
    fn split_utf8_string_using_utf8_str() {
        let haystack = "Köln";
        let letters: Vec<_> = StrSplit::new(haystack, "ö").collect();
        assert_eq!(letters, vec!["K", "ln"]);
    }

    #[test]
    fn split_string_with_a_non_matching_delimiter() {
        let haystack = "a b c";
        let letters: Vec<_> = StrSplit::new(haystack, "x").collect();
        assert_eq!(letters, vec!["a b c"]);
    }

    #[test]
    fn return_an_empty_string_when_haystack_ends_with_delimiter() {
        let haystack = "a b c ";
        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", ""]);
    }

    #[test]
    fn return_the_substring_before_the_given_str() {
        assert_eq!(sub_string_before_str("hello world", "ld"), "hello wor");
    }

    #[test]
    fn return_the_substring_before_the_given_char() {
        assert_eq!(sub_string_before_char("hello world", 'o'), "hell");
    }

    #[test]
    fn return_the_substring_before_the_given_utf8_char() {
        assert_eq!(sub_string_before_char("Köln", 'ö'), "K");
    }
}
