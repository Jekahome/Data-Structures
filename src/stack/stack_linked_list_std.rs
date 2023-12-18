#![allow(unused_imports)]

// https://doc.rust-lang.org/std/collections/struct.LinkedList.html
use std::collections::LinkedList;

/// $ cargo test stack_linked_list_std
#[cfg(test)]
mod test {
    use super::*; 

    #[test]
    fn basics() {
        let mut list: LinkedList<i32> = LinkedList::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn peek() {
        let mut list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.front(), None);
        assert_eq!(list.front_mut(), None);
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.front(), Some(&3));
        assert_eq!(list.front_mut(), Some(&mut 3));

        list.front_mut().map(|value| *value = 42);

        assert_eq!(list.front(), Some(&42));
        assert_eq!(list.pop_front(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}
