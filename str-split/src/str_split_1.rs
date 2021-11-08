#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

#[derive(Debug)]
pub struct StrSplit<'haystack, 'delimiter> {
    remainder: Option<&'haystack str>,
    delimiter: &'delimiter str,
}

impl<'haystack, 'delimiter> StrSplit<'haystack, 'delimiter> {
    pub fn new(haystack: &'haystack str, delimiter: &'delimiter str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

// Only the `'haystack` is required, and the `'delimiter` lifetime can be left for the compiler to
// guess.  We kept it as would like to show another property, one lifetime should live longer than
// another.
// impl<'haystack> Iterator for StrSplit<'haystack, '_>
impl<'haystack, 'delimiter> Iterator for StrSplit<'haystack, 'delimiter>
// The following is not required, and only kept for reference.  We can tell the compiler that one
// lifetime should be greater than the other
where
    'haystack: 'delimiter,
{
    type Item = &'haystack str;

    fn next(&mut self) -> Option<Self::Item> {
        // We need to be able to modify the remainder and thus we need a `ref mut` to it.
        // We cannot use `Some(&mut remainder)` as that will only match `Option<&mut str>`, while we
        // have `Option<&str>`.  We want a reference to the matched value so that we can modify it.
        // if let Some(ref mut remainder) = self.remainder {
        // Al alternative approach would be writing it this way, without using the `ref mut`
        if let Some(remainder) = &mut self.remainder {
            if let Some(next_delimiter) = remainder.find(self.delimiter) {
                let head = &remainder[..next_delimiter];
                // We want to get the address of what's left and update `remainder` to point to it.
                // Assign to where the pointer is referencing and not change the pointer value
                // itself.  That why we have `*remainder`.  Get the address of the remaining string,
                // and that why we have `&remainder[next_delimiter + self.delimiter.len()..]`
                *remainder = &remainder[next_delimiter + self.delimiter.len()..];
                Some(head)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }

        // We can replace the outer if condition using the following, as the `?` works with both
        // `Result` and `Option`.  Note that we need to get a mutable reference, using `as_mut()`.
        // let remainder = self.remainder.as_mut()?;
        // if let Some(next_delimiter) = remainder.find(self.delimiter) {
        //     let head = &remainder[..next_delimiter];
        //     *remainder = &remainder[next_delimiter + self.delimiter.len()..];
        //     Some(head)
        // } else {
        //     self.remainder.take()
        // }
    }
}

// The lifetime of the returned `&str` is the same as the given `s: &str` and there is no need to
// indicate that, as the compiler will figure it out.
fn sub_string_before(s: &str, c: char) -> &str {
    // The delimiter here as a very short lifetime, until the end of the function.  That's
    let delimiter = format!("{}", c);
    StrSplit::new(s, &delimiter)
        .next()
        .expect("StrSplit always return at least one result")
}

#[cfg(test)]
mod tests {
    use crate::str_split_1::{sub_string_before, StrSplit};

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
    fn return_the_substring_before_the_given_char() {
        assert_eq!(sub_string_before("hello world", 'o'), "hell");
    }
}
