pub struct MyIterator<'iter, T> {
    slice: &'iter [T],
}

impl<'iter, T> MyIterator<'iter, T> {
    pub fn wrap(collection: &'iter [T]) -> MyIterator<'iter, T> {
        MyIterator {
            slice: &collection[..],
        }
    }
}

impl<'iter, T> Iterator for MyIterator<'iter, T> {
    type Item = &'iter T;

    fn next(&mut self) -> Option<Self::Item> {
        // if self.slice.is_empty() {
        //     return None;
        // }
        // let head = self.slice.get(0);
        // let tail = &self.slice[1..];
        let (head, tail) = self.slice.split_first()?;

        self.slice = tail;
        Some(head)
    }
}

pub struct MyMutIterator<'iter, T> {
    slice: &'iter mut [T],
}

impl<'iter, T> MyMutIterator<'iter, T> {
    pub fn wrap(collection: &'iter mut [T]) -> MyMutIterator<'iter, T> {
        MyMutIterator {
            slice: &mut collection[..],
        }
    }
}

impl<'iter, T> Iterator for MyMutIterator<'iter, T> {
    type Item = &'iter mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let slice = &mut self.slice;
        let slice = std::mem::replace(slice, &mut []);
        let (head, tail) = slice.split_first_mut()?;

        self.slice = tail;
        Some(head)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn iterator_over_empty_readonly_collection() {
        let collection = Vec::<i32>::new();
        let mut wrapper = MyIterator::wrap(&collection);

        assert_eq!(wrapper.next(), None);
    }

    #[test]
    fn iterator_over_non_empty_readonly_collection() {
        let collection = vec![1, 2, 3, 4];
        let mut wrapper = MyIterator::wrap(&collection);

        assert_eq!(wrapper.next(), Some(&collection[0]));
        assert_eq!(wrapper.next(), Some(&collection[1]));
        assert_eq!(wrapper.next(), Some(&collection[2]));
        assert_eq!(wrapper.next(), Some(&collection[3]));
        assert_eq!(wrapper.next(), None);
    }

    #[test]
    fn iterator_over_empty_mut_collection() {
        let mut collection = Vec::<i32>::new();
        let mut wrapper = MyMutIterator::wrap(&mut collection);

        assert_eq!(wrapper.next(), None);
    }

    #[test]
    fn update_an_item_returned_by_the_iterator() {
        let mut collection = vec![1];
        let mut wrapper = MyMutIterator::wrap(&mut collection);

        let element = wrapper.next().expect("The collection has one element");
        *element = *element + 1;

        assert_eq!(collection[0], 2);
    }
}
