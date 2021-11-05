use std::ops::Range;

use crate::{Change, Spellchecker};

struct DeleteSpacesSpellchecker;

impl Spellchecker for DeleteSpacesSpellchecker {
    fn check(&self, input: &str) -> Vec<Change> {
        input
            .match_indices(" ")
            .map(|(index, space)| delete_string(index, space)
            ).collect()
    }
}

fn delete_string(index: usize, space: &str) -> Change {
    Change::Delete(string_range(index, space))
}

fn string_range(index: usize, word: &str) -> Range<usize> {
    index..index + word.len()
}

#[cfg(test)]
mod tests {
    use crate::{spellcheck_dynamic, spellcheck_static};

    use super::*;

    #[test]
    fn remove_spaces() {
        let text = "This is an example of traits and traits objects";
        let result = spellcheck_static(text, DeleteSpacesSpellchecker);
        /* This is not working as expected as we never implemented the application of changes */
        assert_eq!(text, result);

        let result = spellcheck_dynamic(text, &DeleteSpacesSpellchecker);
        /* This is not working as expected as we never implemented the application of changes */
        assert_eq!(text, result);
    }
}
