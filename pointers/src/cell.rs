use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// Cell is not thread-safe and we need to tell the compiler not to allow this to be shared between
// threads
// Negative traits are not yet supported but these will be in the near future.
// Furthermore, we don't need to do this in our case as `UnsafeCell` does not support `Sync`, which
// implies that our `Cell` is not too, and that's why it is commented out.
// impl<T> !Sync for Cell<T> {}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        // SAFETY: we know that no-one is concurrently mutating self.value (because `!Sync`)
        // SAFETY: we know that we are not invalidating any references, because we never give any
        //         out
        unsafe { *self.value.get() = value }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFETY: we know no-one else is value, since this cannot be shared between threads
        //         (because `!Sync`).
        unsafe { *self.value.get() }
    }
}
