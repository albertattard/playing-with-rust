pub fn flatten<I>(iter: I) -> Flatten<I>
where
    I: Iterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter)
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    inner: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            inner: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    // We can cast `O` to `Iterator`, as shown next, but the compiler infers that automatically
    // <O as Iterator>::Item: IntoIterator,
    O::Item: IntoIterator,
{
    // Here we need to cast `O::Item` as `IntoIterator` as this can be ambiguous.  I suspect that
    // this can be seen as a path to some namespace and its internal types, rather than the Rust
    // compiler not able to infer this.
    type Item = <O::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iter) = self.inner {
                if let Some(item) = inner_iter.next() {
                    return Some(item);
                }
                self.inner = None;
            }

            // Move to the next inner iterator
            self.inner = Some(self.outer.next()?.into_iter());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::flatten;

    #[test]
    fn flatten_an_empty_iterator() {
        let count = flatten(std::iter::empty::<Vec<()>>()).count();
        assert_eq!(count, 0);
    }

    #[test]
    fn flatten_an_iterator_with_one_vector() {
        let count = flatten(std::iter::once(vec!["an item"])).count();
        assert_eq!(count, 1);
    }

    #[test]
    fn flatten_an_iterator_with_one_vector_containing_multiple_items() {
        let count = flatten(std::iter::once(vec!["an item", "a second item"])).count();
        assert_eq!(count, 2);
    }

    #[test]
    fn flatten_an_iterator_with_multiple_vectors_containing_multiple_items() {
        let count =
            flatten(vec![vec!["an item", "a second item"], vec![], vec!["an item"]].into_iter())
                .count();
        assert_eq!(count, 3);
    }
}
