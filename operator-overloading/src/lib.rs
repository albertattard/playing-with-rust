use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Tray {
    quantity: usize,
}

impl Add for Tray {
    type Output = Tray;

    fn add(self, rhs: Self) -> Self::Output {
        Tray {
            quantity: self.quantity + rhs.quantity,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Tray;

    #[test]
    fn adds_two_trays() {
        let a = Tray { quantity: 2 };
        let b = Tray { quantity: 5 };

        let result = a + b;
        assert_eq!(result, Tray { quantity: 7 });
    }
}
