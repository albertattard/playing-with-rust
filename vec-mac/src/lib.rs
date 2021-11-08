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
        let mut v = Vec::new();
        $(v.push($element);)*
        v
    }};

    ($element:expr; $count:expr) => {{
        let mut v = Vec::new();
        let value = $element;
        for _ in 0..$count {
            v.push(value.clone());
        }
        v
    }};
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
    fn fill_vector_with_many_elements() {
        let v: Vec<u32> = my_vec![1; 42];
        assert!(!v.is_empty());
        assert_eq!(v.len(), 42);
        for i in v {
            assert_eq!(i, 1);
        }
    }
}
