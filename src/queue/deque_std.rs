#![allow(unused_imports)]

// Двусторонняя очередь, реализованная с помощью расширяемого кольцевого буфера.
use std::collections::VecDeque;

/// $ cargo test deque_std
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forward_success() {
        let mut deque: VecDeque<isize> = VecDeque::new();
        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);
        deque.push_back(4);
        deque.push_back(5);

        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), Some(2));
        assert_eq!(deque.pop_front(), Some(3));
        assert_eq!(deque.pop_front(), Some(4));

        assert_eq!(deque.front(), Some(&5));
        assert_eq!(deque.pop_front(), Some(5));
        assert_eq!(deque.front(), None);
    }

    #[test]
    fn test_back_success() {
        let mut deque: VecDeque<isize> = VecDeque::new();
        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);
        deque.push_back(4);
        deque.push_back(5);

        assert_eq!(deque.pop_back(), Some(5));
        assert_eq!(deque.pop_back(), Some(4));
        assert_eq!(deque.pop_back(), Some(3));
        assert_eq!(deque.pop_back(), Some(2));

        assert_eq!(deque.back(), Some(&1));
        assert_eq!(deque.pop_back(), Some(1));
        assert_eq!(deque.back(), None);
    }

    #[test]
    fn queue_peek() {
        let mut deque: VecDeque<isize> = VecDeque::new();
        deque.push_back(1);
        deque.push_back(2);
        assert_eq!(deque.front(), Some(&1));
    }

    #[test]
    fn queue_peek_mut() {
        let mut deque: VecDeque<isize> = VecDeque::new();
        deque.push_back(1);
        deque.push_back(2);
        if let Some(v) = deque.front_mut() {
            *v = 99;
        }
        assert_eq!(deque.front(), Some(&99));
        assert_eq!(deque.pop_front(), Some(99));
    }

    #[test]
    fn queue_dequeue() {
        let mut deque: VecDeque<isize> = VecDeque::new();
        deque.push_back(1);
        deque.push_back(2);

        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), Some(2));
        assert_eq!(deque.is_empty(), true);
    }
}
