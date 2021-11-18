pub mod cell;

#[cfg(test)]
mod test {
    #[test]
    fn cell_example() {
        // -----------------------------------------------------------------------------------------
        // Example 1 - without Cell (it does not work for the reasons explained below)
        // -----------------------------------------------------------------------------------------
        // // Consider a tray with case that contains the number of cakes on the tray
        // struct CakeTray {
        //     quantity: usize,
        // }
        //
        // // The guests have access to the tray and they can tak one as long as there are cakes left
        // struct Guest<'a> {
        //     tray: &'a mut CakeTray,
        // }
        // impl Guest<'_> {
        //     fn take(&mut self) -> bool {
        //         if self.tray.quantity > 0 {
        //             self.tray.quantity -= 1;
        //             return true;
        //         }
        //
        //         false
        //     }
        // }
        //
        // // Create a tray with 2 cakes ane 1 guest
        // let mut tray = CakeTray { quantity: 2 };
        // let mut a = Guest { tray: &mut tray };
        //
        // // As it is we cannot create the second guest and share the tray with them too.  This will
        // // fail to compile with the following error:
        // // 'cannot borrow `tray` as mutable more than once at a time'
        // let mut b = Guest { tray: &mut tray };
        //
        // // The guest should be able to take the cake and the quantity updated.
        // assert!(a.take());
        // assert!(b.take());
        // assert!(!a.take());
        // assert_eq!(tray.quantity, 0);
        // -----------------------------------------------------------------------------------------

        // -----------------------------------------------------------------------------------------
        // Example 2 - an alternative using Cell (it works)
        // -----------------------------------------------------------------------------------------
        // Consider a tray with case that contains the number of cakes on the tray
        #[derive(Copy, Clone)]
        struct CakeTray {
            quantity: usize,
        }

        // The guests have access to the tray and they can tak one as long as there are cakes left
        struct Guest<'a> {
            tray: &'a std::cell::Cell<CakeTray>,
        }
        impl Guest<'_> {
            fn take(&self) -> bool {
                let quantity = self.tray.get().quantity;
                if quantity > 0 {
                    self.tray.set(CakeTray {
                        quantity: quantity - 1,
                    });
                    return true;
                }

                false
            }
        }

        // Create a tray with 2 cakes ane 1 guest
        let tray = &std::cell::Cell::new(CakeTray { quantity: 2 });
        let a = Guest { tray };
        let b = Guest { tray };

        // The guest should be able to take the cake and the quantity updated.
        assert!(a.take());
        assert!(b.take());
        assert!(!a.take());
        assert_eq!(tray.get().quantity, 0);
        // -----------------------------------------------------------------------------------------
    }
}
