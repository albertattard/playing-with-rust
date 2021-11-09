#[macro_export]
macro_rules! my_vec {
    // The following is not required as we can have it as part of the following match
    // () => {
    //     Vec::new()
    // };

    // Note that given that we have multiple lines, we need to create a block (the two sets of `{}`)
    // and then return from the block.  Note that in the following macro we have two statements,
    // followed by an expression.  The block, evaluates to this expression.
    // Given that we can take an arbitrary number of elements, we need to use the pattern `$(...)`.
    // ($($element:expr),+ $(,)?) => {{
    // We used the `*` instead of the `+`, to support 0 or more elements covering the empty vec.
    // I prefer the other option as you can optimise the code better
    ($($element:expr),* $(,)?) => {{
        // This is only needed is we allow empty vec
        #[allow(unused_mut)]
        // We can make use of a few tricks to create the Vector with the right capacity
        // let mut v = Vec::new();
        let mut v = Vec::with_capacity($crate::count![@COUNT; $($element),*]);
        $(v.push($element);)*
        v
    }};

    ($element:expr; $count:expr) => {{
        // Do not evaluate the expression more than once
        let count = $count;
        // We can create the Vector with the right capacity and saving the efforts resize and move
        // the data
        // let mut v = Vec::new();
        let mut v = Vec::with_capacity(count);

        // There are several ways to put elements in a vector
        // 1. Using a loop
        // let value = $element;
        // for _ in 0..$count {
        //     v.push(value.clone());
        // }
        // 2. Use the Vector's `extend()` and `repeat()` methods
        // v.extend(std::iter::repeat($element).take($count));
        // 3. Alternatively, we can use the Vector's `resize()` method, which is more efficient as
        //    it does not need to do bounds checking.
        v.resize(count, $element);
        v
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! count {
    // Counts the var args provided, without consuming the elements.  The elements are converted
    // into units (zero size) and captured into an array.  Then we get the length of the array.
    // There seem to be more ways to do this, but this is quite a nice trick.
    // The `@COUNT;` is a label
    (@COUNT; $($element:expr),*) => {
        <[()]>::len(&[$($crate::count!(@SUBST; $element)),*])
    };

    // Substitutes the elements with unit, that has zero size, so that these are not consumed.
    // Using unit here served two purposes, 1. we don't use the expression and 2. no memory is used.
    // The `@SUBST;` is a label
    (@SUBST; $($_element:expr),*) => { () }
}

#[cfg(test)]
mod tests {
    #[test]
    fn create_empty_vector() {
        let v: Vec<u32> = my_vec![];
        assert!(v.is_empty());
    }

    #[test]
    fn create_vector_with_one_element() {
        let v: Vec<u32> = my_vec![42];
        assert!(!v.is_empty());
        assert_eq!(v.len(), 1);
        assert_eq!(v[0], 42);
    }

    #[test]
    fn create_vector_with_two_elements() {
        let v: Vec<u32> = my_vec![42, 24];
        assert!(!v.is_empty());
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], 42);
        assert_eq!(v[1], 24);
    }

    #[test]
    fn create_vector_with_many_elements() {
        let v: Vec<u32> = my_vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert!(!v.is_empty());
        assert_eq!(v.len(), 9);
        for i in 0..9 {
            assert_eq!(v[i], (i + 1) as u32);
        }
    }

    #[test]
    fn create_vector_with_a_trialing_comma() {
        let v: Vec<u32> = my_vec![1, 2, 3,];
        assert!(!v.is_empty());
        assert_eq!(v.len(), 3);
        for i in 0..3 {
            assert_eq!(v[i], (i + 1) as u32);
        }
    }

    #[test]
    fn fill_vector_with_many_elements_of_the_same_value() {
        let v: Vec<u32> = my_vec![7; 42];
        assert!(!v.is_empty());
        assert_eq!(v.len(), 42);
        for i in v {
            assert_eq!(i, 7);
        }
    }
}
