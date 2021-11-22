mod tray;

#[cfg(test)]
mod tests {
    use std::ops::Add;

    #[test]
    fn adds_two_numbers_using_the_add_method() {
        // We can use the `add()` method instead of the `+`, but we need to import the trait
        assert_eq!(10_i32.add(5), 15);
    }
}
