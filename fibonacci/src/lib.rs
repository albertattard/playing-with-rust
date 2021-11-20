use std::ops::Add;

trait Number {
    const ZERO: Self;
    const ONE: Self;
}

impl Number for i32 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}

fn fib<T: Number + Add<Output = T>>(n: usize) -> T {
    match n {
        0 => T::ZERO,
        1 => T::ONE,
        n => fib::<T>(n - 1) + fib::<T>(n - 2),
    }
}

#[cfg(test)]
mod tests {
    use crate::fib;

    #[test]
    fn first_fibonacci_number_is_0() {
        let result:i32 = fib(0);
        assert_eq!(result, 0);
    }

    #[test]
    fn second_fibonacci_number_is_1() {
        let result:i32 = fib(1);
        assert_eq!(result, 1);
    }

    #[test]
    fn tenth_fibonacci_number_is_55() {
        let result:i32 = fib(10);
        assert_eq!(result, 55);
    }
}
