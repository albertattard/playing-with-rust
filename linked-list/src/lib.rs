// use std::mem::replace;

pub struct LinkedList<T> {
    head: Link<T>,
}

impl<T> LinkedList<T> {
    pub fn empty() -> Self {
        // LinkedList { head: None }
        Self { head: None }
    }

    pub fn push(&mut self, element: T) {
        // let current_head = replace(&mut self.head, None);
        let current_head = self.head.take();

        let next = Node {
            element,
            next: current_head,
        };
        self.head = Some(Box::new(next));
    }

    pub fn pop(&mut self) -> Option<T> {
        // let current_head = replace(&mut self.head, None);
        let current_head = self.head.take();

        // match current_head {
        //     None => None,
        //     Some(n) => {
        //         self.head = n.next;
        //         Some(n.element)
        //     }
        // }
        current_head.map(|n| {
            self.head = n.next;
            n.element
        })
    }

    pub fn peek(&self) -> Option<&T> {
        // match &self.head {
        //     None => None,
        //     Some(n) => Some(&n.element)
        // }
        self.head.as_ref().map(|n| &n.element)
    }
}

struct Node<T> {
    element: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peek_points_to_the_last_element_added_to_the_list() {
        let mut list: LinkedList<u32> = LinkedList::empty();
        assert_eq!(None, list.peek());

        list.push(42);
        assert_eq!(Some(&42), list.peek());

        list.push(24);
        assert_eq!(Some(&24), list.peek());

        /* Next peek should still point to the last element added to the list */
        assert_eq!(Some(&24), list.peek());
    }

    #[test]
    fn elements_are_removed_in_the_reverse_order_that_were_added() {
        let mut list: LinkedList<u32> = LinkedList::empty();
        list.push(24);
        list.push(42);

        assert_eq!(Some(42), list.pop());
        assert_eq!(Some(24), list.pop());
        assert_eq!(None, list.pop());
    }

    #[test]
    fn add_remove_and_view_elements() {
        let mut list: LinkedList<u32> = LinkedList::empty();

        list.push(24);
        assert_eq!(Some(&24), list.peek());
        assert_eq!(Some(24), list.pop());
        assert_eq!(None, list.peek());
        assert_eq!(None, list.pop());
    }
}
