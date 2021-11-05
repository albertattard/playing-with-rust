use std::ops::Range;

mod noop_spellchecker;
mod delete_spaces_spellchecker;

pub fn spellcheck_static<C: Spellchecker>(input: &str, spellchecker: C) -> String {
    let mut result = input.to_owned();
    for change in spellchecker.check(input) {
        apply_change(&mut result, change);
    }
    result
}

pub fn spellcheck_dynamic(input: &str, spellchecker: &dyn Spellchecker) -> String {
    let mut result = input.to_owned();
    for change in spellchecker.check(input) {
        apply_change(&mut result, change);
    }
    result
}

pub trait Spellchecker {
    fn check(&self, input: &str) -> Vec<Change>;
}

fn apply_change(_string: &mut String, _change: Change) {
    // TODO: apply change
}

pub enum Change {
    Delete(Range<usize>),
    Replace(Range<usize>, String),
}
