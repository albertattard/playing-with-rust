use crate::{Change, Spellchecker};

struct NoopSpellchecker;

impl Spellchecker for NoopSpellchecker {
    fn check(&self, _input: &str) -> Vec<Change> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::{spellcheck_dynamic, spellcheck_static};

    use super::*;

    #[test]
    fn no_changes() {
        let text = "This is an example of traits and traits objects";
        let result = spellcheck_static(text, NoopSpellchecker);
        assert_eq!(text, result);

        let result = spellcheck_dynamic(text, &NoopSpellchecker);
        assert_eq!(text, result);
    }
}
