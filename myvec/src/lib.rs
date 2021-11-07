use std::alloc::{alloc, dealloc, realloc, Layout};
use std::ptr::NonNull;

#[derive(Debug)]
pub struct MyVec<T> {
    allocation: NonNull<T>,
    len: usize,
    capacity: usize,
}

impl<T> MyVec<T> {
    pub fn empty() -> Self {
        Self {
            allocation: NonNull::dangling(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }

        let value = unsafe { &*self.allocation.as_ptr().add(index) };
        Some(value)
    }

    pub fn push(&mut self, item: T) {
        if self.capacity == 0 {
            let capacity = 4;
            let layout = Layout::array::<T>(capacity).expect("Could not allocate memory");

            // SAFETY: We are hardcoding layout to a non-zero value, never 0, thus the following
            // will never produce undefined behaviour
            let allocation = unsafe { alloc(layout) } as *mut T;

            // SAFETY: We have allocated enough space for this item to fit in memory, when we
            // set the capacity to 4.  We want to use the write() function as we don't want to
            // read from the pointer.
            unsafe { allocation.write(item) }

            self.allocation = NonNull::new(allocation).expect("Could not allocate memory");
            self.len = 1;
            self.capacity = capacity;
        } else if self.len < self.capacity {
            // Make sure that we are not growing more that we are allowed.  We may have memory,
            // there is a limit of isize::MAX and we need to stay below that.
            assert!(
                self.len
                    .checked_mul(std::mem::size_of::<T>())
                    .expect("Cannot reach memory location")
                    < isize::MAX as usize
            );

            // SAFETY: We have allocated enough space for this item to fit in memory, thus this
            // should not be a problem.
            unsafe {
                self.allocation.as_ptr().add(self.len).write(item);
            }
            self.len += 1;
        } else if self.len == self.capacity {
            let new_capacity = self
                .capacity
                .checked_mul(2)
                .expect("Cannot allocate new memory");
            let align = std::mem::align_of::<T>();
            let size = std::mem::size_of::<T>() * self.capacity;
            size.checked_add(size % align)
                .expect("Cannot allocate new memory");
            let new_size = std::mem::size_of::<T>()
                .checked_mul(new_capacity)
                .expect("Cannot allocate new memory");
            let allocation = unsafe {
                let layout = Layout::from_size_align_unchecked(size, align);
                let allocation = realloc(self.allocation.as_ptr() as *mut u8, layout, new_size);
                let allocation =
                    NonNull::new(allocation as *mut T).expect("Cannot allocate new memory");

                // SAFETY: We have allocated enough space for this item to fit in memory, thus this
                // should not be a problem.
                allocation.as_ptr().add(self.len).write(item);
                allocation
            };

            self.allocation = allocation;
            self.len += 1;
            self.capacity = new_capacity;
        } else {
            panic!("Invalid state, as length is greater than capacity")
        }
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 {
            unsafe {
                let data = std::slice::from_raw_parts_mut(self.allocation.as_ptr(), self.len);
                std::ptr::drop_in_place(data);

                let layout = Layout::from_size_align_unchecked(
                    std::mem::size_of::<T>() * self.capacity,
                    std::mem::align_of::<T>(),
                );
                // Let the compiler figure it out
                // dealloc(self.allocation.as_ptr() as _, layout);
                dealloc(self.allocation.as_ptr() as *mut u8, layout);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::MyVec;

    #[test]
    fn create_empty_vector() {
        // let vec: MyVec<u32> = MyVec::empty();
        let vec = MyVec::<u32>::empty();
        assert_eq!(vec.capacity(), 0);
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.get(0), None);
    }

    #[test]
    fn add_item_to_empty_vector() {
        // let vec: MyVec<u32> = MyVec::empty();
        let mut vec = MyVec::<u32>::empty();
        vec.push(42);
        assert_eq!(vec.capacity(), 4);
        assert_eq!(vec.len(), 1);
        assert_eq!(vec.get(0), Some(&42));
    }

    #[test]
    fn read_the_same_item_twice() {
        // let vec: MyVec<u32> = MyVec::empty();
        let mut vec = MyVec::<u32>::empty();
        vec.push(42);
        assert_eq!(vec.get(0), Some(&42));
        assert_eq!(vec.get(0), Some(&42));
    }

    #[test]
    fn force_vector_to_resize() {
        // let vec: MyVec<u32> = MyVec::empty();
        let mut vec = MyVec::<u32>::empty();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.push(4);
        vec.push(5);

        assert_eq!(vec.capacity(), 8);
        assert_eq!(vec.len(), 5);
        assert_eq!(vec.get(4), Some(&5));
    }
}
