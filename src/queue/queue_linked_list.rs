pub use ds_queue_rc::{Node, Queue};
mod ds_queue_rc {
    use std::fmt::Debug;
    use std::rc::Rc;

    #[derive(Debug)]
    pub struct Node<T> {
        data: T,
        next: Option<Rc<Node<T>>>,
    }

    impl<T> Node<T>
    where
        T: Copy + Debug,
    {
        pub fn new(data: T) -> Self {
            Node { data, next: None }
        }
        pub fn set_next(&mut self, next: Option<Rc<Node<T>>>) -> bool {
            if self.next.is_none() {
                self.next = next;
                return true;
            } else {
                if let Some(e) = &mut self.next {
                    (*Rc::<Node<T>>::get_mut(e).unwrap()).set_next(next);
                    return true;
                }
            }
            return false;
        }
        pub fn peek_all<'b>(&'b self, buf: &mut Vec<&'b T>) {
            buf.push(&self.data);
            if self.next.is_some() {
                self.next.as_ref().unwrap().peek_all(buf);
            }
        }
    }

    #[derive(Debug)]
    pub struct Queue<T> {
        head: Option<Rc<Node<T>>>,
        count: usize,
    }
    impl<T> Queue<T>
    where
        T: Copy + Debug,
    {
        pub fn new(head: Option<Rc<Node<T>>>) -> Self {
            Self { head, count: 1 }
        }
        pub fn enqueue(&mut self, next: Option<Rc<Node<T>>>) {
            if self.head.is_some() {
                if let Some(e) = Rc::<Node<T>>::get_mut(&mut self.head.as_mut().unwrap()) {
                    e.set_next(next);
                    self.count += 1;
                }
            }
        }
        pub fn dequeue(&mut self) -> Option<T> {
            if self.head.is_some() {
                let head = self.head.as_mut().unwrap();
                let data = head.data.clone();
                if let Some(e) = &head.next {
                    self.head = Some(Rc::clone(&e));
                    self.count -= 1;
                } else {
                    self.head = None;
                    self.count = 0;
                }
                return Some(data);
            }
            return None;
        }
        pub fn peek(&self) -> Option<&T> {
            if self.head.is_some() {
                return Some(&self.head.as_ref().unwrap().data);
            }
            return None;
        }
        pub fn peek_all<'b>(&'b self, buf: &mut Vec<&'b T>) {
            if self.head.is_some() {
                self.head.as_ref().unwrap().peek_all(buf);
            }
        }
        pub fn length(&self) -> usize {
            self.count
        }
        pub fn is_empty(&self) -> bool {
            self.count == 0
        }
    }

    impl<T> Iterator for Queue<T>
    where
        T: Copy + Debug,
    {
        type Item = T;

        fn next(&mut self) -> Option<T> {
            self.dequeue()
        }
    }
}

/// $ cargo test queue_rc
#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn test_success() {
        let node1 = Node::<i32>::new(1);
        let node2 = Node::<i32>::new(2);
        let node3 = Node::<i32>::new(3);
        let node4 = Node::<i32>::new(4);
        let node5 = Node::<i32>::new(5);
        let mut queue = Queue::new(Some(Rc::new(node1)));
        queue.enqueue(Some(Rc::new(node2)));
        queue.enqueue(Some(Rc::new(node3)));
        queue.enqueue(Some(Rc::new(node4)));
        queue.enqueue(Some(Rc::new(node5)));

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), Some(4));

        assert_eq!(queue.peek(), Some(&5));
        assert_eq!(queue.dequeue(), Some(5));
        assert_eq!(queue.peek(), None);
    }

    #[test]
    fn test_enqueue() {
        let node1 = Node::<i32>::new(1);
        let mut queue = Queue::new(Some(Rc::new(node1)));

        let node2 = Node::<i32>::new(2);
        queue.enqueue(Some(Rc::new(node2)));

        assert_eq!(queue.peek(), Some(&1));
        assert_eq!(queue.length(), 2);
        assert!(!queue.is_empty());
    }

    #[test]
    fn test_dequeue() {
        let node1 = Node::<i32>::new(1);
        let mut queue = Queue::new(Some(Rc::new(node1)));

        let node2 = Node::<i32>::new(2);
        queue.enqueue(Some(Rc::new(node2)));

        let item = queue.dequeue();
        assert_eq!(item, Some(1));
        assert_eq!(queue.peek(), Some(&2));
        assert_eq!(queue.length(), 1);
        assert!(!queue.is_empty());

        let item = queue.dequeue();
        assert_eq!(item, Some(2));
        assert_eq!(queue.peek(), None);
        assert_eq!(queue.length(), 0);
        assert!(queue.is_empty());
    }

    #[test]
    fn test_peek_all() {
        let node1 = Node::<i32>::new(1);
        let node2 = Node::<i32>::new(2);
        let node3 = Node::<i32>::new(3);

        let mut q = Queue::new(Some(Rc::new(node1)));
        q.enqueue(Some(Rc::new(node2)));
        q.enqueue(Some(Rc::new(node3)));

        let mut buf: Vec<&i32> = vec![];
        let _ = &q.peek_all(&mut buf);
        assert_eq!(buf, vec![&1, &2, &3]);
    }
}
