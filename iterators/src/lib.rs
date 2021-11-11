pub trait MyIteratorExt: Iterator {
    fn custom_flatten(self) -> Flatten<Self>
    where
        Self: Sized,
        Self::Item: IntoIterator;
}

impl<T> MyIteratorExt for T
where
    T: Iterator,
{
    fn custom_flatten(self) -> Flatten<Self>
    where
        Self: Sized,
        Self::Item: IntoIterator,
    {
        flatten(self)
    }
}

pub fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    next_inner: Option<<O::Item as IntoIterator>::IntoIter>,
    back_inner: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            next_inner: None,
            back_inner: None,
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
            if let Some(ref mut inner_iter) = self.next_inner {
                if let Some(item) = inner_iter.next() {
                    return Some(item);
                }
                self.next_inner = None;
            }

            if let Some(next_inner) = self.outer.next() {
                // Move to the next inner iterator
                self.next_inner = Some(next_inner.into_iter());
            } else {
                // This get a bit complicated because of the double ended iterator.  The outer
                // iterator may be consuming items from both end.  Before we terminate, we need to
                // check if the back inner iterator is empty as this may have be yielded by the
                // outer but not yet exhausted.
                return self.back_inner.as_mut()?.next();
            }
        }
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    // DoubleEndedIterator implements Iterator
    // O: Iterator + DoubleEndedIterator,
    O: DoubleEndedIterator,
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut back_iter) = self.back_inner {
                if let Some(item) = back_iter.next_back() {
                    return Some(item);
                }
                self.back_inner = None;
            }

            // Move to the previous (`next_back()`) inner iterator
            if let Some(back_inner) = self.outer.next_back() {
                // Move to the previous back inner iterator
                self.back_inner = Some(back_inner.into_iter());
            } else {
                // This get a bit complicated because of the double ended iterator.  The outer
                // iterator may be consuming items from both end.  Before we terminate, we need to
                // check if the next inner iterator is empty as this may have be yielded by the
                // outer but not yet exhausted.
                return self.next_inner.as_mut()?.next_back();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{flatten, MyIteratorExt};

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

    #[test]
    fn flatten_and_reverse_iterator_with_multiple_vectors_containing_multiple_items() {
        let reverse = flatten(vec![vec!["a", "b"], vec![], vec!["c"]].into_iter())
            // The `rev()` method is available as we have implemented the `DoubleEndedIterator`
            // trait for `Flatten`
            .rev()
            .collect::<Vec<_>>();
        assert_eq!(reverse, vec!["c", "b", "a"]);
    }

    #[test]
    fn consume_items_from_both_ends_starting_from_the_front() {
        let mut iter = flatten(vec![vec!["a", "b"], vec![], vec!["c"]].into_iter());
        assert_eq!(iter.next(), Some("a"));
        assert_eq!(iter.next_back(), Some("c"));
        assert_eq!(iter.next(), Some("b"));

        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn consume_items_from_both_ends_starting_from_the_rear() {
        let mut iter = flatten(vec![vec!["a", "b"], vec![], vec!["c"]].into_iter());
        assert_eq!(iter.next_back(), Some("c"));
        assert_eq!(iter.next(), Some("a"));
        assert_eq!(iter.next_back(), Some("b"));

        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn consume_items_only_from_the_front() {
        let mut iter = flatten(vec![vec!["a", "b"], vec![], vec!["c"]].into_iter());
        assert_eq!(iter.next(), Some("a"));
        assert_eq!(iter.next(), Some("b"));
        assert_eq!(iter.next(), Some("c"));

        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn consume_items_only_from_the_rear() {
        let mut iter = flatten(vec![vec!["a", "b"], vec![], vec!["c"]].into_iter());
        assert_eq!(iter.next_back(), Some("c"));
        assert_eq!(iter.next_back(), Some("b"));
        assert_eq!(iter.next_back(), Some("a"));

        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn flatten_iterator_through_the_extension_function() {
        let result = vec![vec!["a", "b"], vec![], vec!["c"]]
            .into_iter()
            .custom_flatten()
            .collect::<Vec<_>>();
        assert_eq!(result, vec!["a", "b", "c"]);
    }
}
