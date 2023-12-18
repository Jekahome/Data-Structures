#![allow(dead_code)]

///! Очередь с приоритетом - это особый тип очереди, в которой каждый элемент связан
///! со значением приоритета.
///! И элементы обслуживаются на основе их приоритета.
///! То есть первыми обслуживаются элементы с более высоким приоритетом.
pub use ds_queue_priority::{Priority, QueuePriority};
mod ds_queue_priority {

    #[derive(PartialEq, PartialOrd, Debug)]
    pub enum Priority {
        LOW,
        MIDDLE,
        HIGH,
    }

    #[derive(Debug)]
    pub struct QueuePriority<T> {
        queue_high: Vec<T>,
        queue_middle: Vec<T>,
        queue_low: Vec<T>,
    }

    impl<T: std::cmp::PartialEq> QueuePriority<T> {
        pub fn new() -> Self {
            Self {
                queue_low: Vec::new(),
                queue_middle: Vec::new(),
                queue_high: Vec::new(),
            }
        }
        pub fn enqueue(&mut self, item: T, priority: Priority) {
            match priority {
                Priority::HIGH => {
                    self.queue_high.push(item);
                }
                Priority::MIDDLE => {
                    self.queue_middle.push(item);
                }
                Priority::LOW => {
                    self.queue_low.push(item);
                }
            }
        }

        pub fn dequeue(&mut self) -> Option<T> {
            if !self.is_empty() {
                if !self.queue_high.is_empty() {
                    return Some(self.queue_high.remove(0));
                }
                if !self.queue_middle.is_empty() {
                    return Some(self.queue_middle.remove(0));
                }
                if !self.queue_low.is_empty() {
                    return Some(self.queue_low.remove(0));
                }
            }
            None
        }
        pub fn peek(&self) -> Option<&T> {
            if !self.is_empty() {
                if !self.queue_high.is_empty() {
                    return self.queue_high.first();
                }
                if !self.queue_middle.is_empty() {
                    return self.queue_middle.first();
                }
                if !self.queue_low.is_empty() {
                    return self.queue_low.first();
                }
            }
            None
        }

        pub fn search(&mut self, item: T) -> Option<&mut T> {
            if !self.is_empty() {
                if !self.queue_high.is_empty() {
                    for el in self.queue_high.iter_mut() {
                        if el == &item {
                            return Some(el);
                        }
                    }
                }
                if !self.queue_middle.is_empty() {
                    for el in self.queue_middle.iter_mut() {
                        if el == &item {
                            return Some(el);
                        }
                    }
                }
                if !self.queue_low.is_empty() {
                    for el in self.queue_low.iter_mut() {
                        if el == &item {
                            return Some(el);
                        }
                    }
                }
            }
            None
        }

        pub fn change_priority(
            &mut self,
            find_item: T,
            find_priority: Priority,
            new_priority: Priority,
        ) -> bool {
            if !self.is_empty() {
                let el = match find_priority {
                    Priority::HIGH => {
                        let mut el = None;
                        if !self.queue_high.is_empty() {
                            let mut find_index = None;
                            for (index, el) in self.queue_high.iter_mut().enumerate() {
                                if el == &find_item {
                                    find_index = Some(index);
                                }
                            }
                            if let Some(index) = find_index {
                                el = Some(self.queue_high.remove(index));
                            }
                        }
                        el
                    }
                    Priority::MIDDLE => {
                        let mut el = None;
                        if !self.queue_middle.is_empty() {
                            let mut find_index = None;
                            for (index, el) in self.queue_middle.iter_mut().enumerate() {
                                if el == &find_item {
                                    find_index = Some(index);
                                }
                            }
                            if let Some(index) = find_index {
                                el = Some(self.queue_middle.remove(index));
                            }
                        }
                        el
                    }
                    Priority::LOW => {
                        let mut el = None;
                        if !self.queue_low.is_empty() {
                            let mut find_index = None;
                            for (index, el) in self.queue_low.iter_mut().enumerate() {
                                if el == &find_item {
                                    find_index = Some(index);
                                }
                            }
                            if let Some(index) = find_index {
                                el = Some(self.queue_low.remove(index));
                            }
                        }
                        el
                    }
                };
                if let Some(el) = el {
                    match new_priority {
                        Priority::HIGH => {
                            self.queue_high.push(el);
                        }
                        Priority::MIDDLE => {
                            self.queue_middle.push(el);
                        }
                        Priority::LOW => {
                            self.queue_low.push(el);
                        }
                    }
                    return true;
                }
            }
            false
        }

        pub fn peek_mut(&mut self) -> Option<&mut T> {
            if !self.is_empty() {
                if !self.queue_high.is_empty() {
                    return self.queue_high.first_mut();
                }
                if !self.queue_middle.is_empty() {
                    return self.queue_middle.first_mut();
                }
                if !self.queue_low.is_empty() {
                    return self.queue_low.first_mut();
                }
            }
            None
        }

        pub fn is_empty(&self) -> bool {
            self.queue_high.is_empty() && self.queue_middle.is_empty() && self.queue_low.is_empty()
        }

        pub fn iter<'a>(&'a self) -> IterQueuePriority<'a, T> {
            IterQueuePriority::new(self)
        }
    }

    pub struct IterQueuePriority<'a, T>((usize, usize, usize), &'a QueuePriority<T>);
    impl<'a, T> IterQueuePriority<'a, T> {
        fn new(q: &'a QueuePriority<T>) -> Self {
            Self((0, 0, 0), q)
        }
    }
    impl<'a, T> Iterator for IterQueuePriority<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            if self.0 .0 < self.1.queue_high.len() {
                self.0 .0 += 1;
                return self.1.queue_high.get(self.0 .0 - 1);
            }
            if self.0 .1 < self.1.queue_middle.len() {
                self.0 .1 += 1;
                return self.1.queue_middle.get(self.0 .1 - 1);
            }
            if self.0 .2 < self.1.queue_low.len() {
                self.0 .2 += 1;
                return self.1.queue_low.get(self.0 .2 - 1);
            }
            None
        }
    }
}

/// $ cargo test queue_priority  
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut queue: QueuePriority<i32> = QueuePriority::new();
        queue.enqueue(18, Priority::LOW);
        queue.enqueue(7, Priority::HIGH);
        queue.enqueue(6, Priority::MIDDLE);

        assert_eq!(Some(7), queue.dequeue());
        assert_eq!(Some(6), queue.dequeue());
        assert_eq!(Some(18), queue.dequeue());
    }

    #[test]
    fn test_search() {
        let mut queue: QueuePriority<i32> = QueuePriority::new();
        queue.enqueue(18, Priority::LOW);
        queue.enqueue(7, Priority::HIGH);
        queue.enqueue(6, Priority::MIDDLE);

        let item = queue.search(7).unwrap();
        *item = 99;

        assert_eq!(queue.dequeue(), Some(99));
        assert_eq!(queue.dequeue(), Some(6));
        assert_eq!(queue.dequeue(), Some(18));
    }

    #[test]
    fn test_change_priority() {
        let mut queue: QueuePriority<i32> = QueuePriority::new();
        queue.enqueue(18, Priority::LOW);
        queue.enqueue(7, Priority::HIGH);
        queue.enqueue(6, Priority::MIDDLE);

        queue.change_priority(18, Priority::LOW, Priority::HIGH);

        assert_eq!(Some(7), queue.dequeue());
        assert_eq!(Some(18), queue.dequeue());
        assert_eq!(Some(6), queue.dequeue());
    }

    // $ cargo test queue::queue_priority::tests::test_iter -- --nocapture
    #[test]
    fn test_iter() {
        let mut queue: QueuePriority<i32> = QueuePriority::new();
        queue.enqueue(18, Priority::LOW);
        queue.enqueue(7, Priority::HIGH);
        queue.enqueue(6, Priority::MIDDLE);

        for item in queue.iter() {
            println!("{:?}", item);
        }
        assert!(true);
    }

    #[test]
    fn queue_peek() {
        let mut queue: QueuePriority<i32> = QueuePriority::new();
        queue.enqueue(1, Priority::LOW);
        queue.enqueue(3, Priority::LOW);
        queue.enqueue(2, Priority::MIDDLE);
        assert_eq!(queue.peek(), Some(&2));
        assert_eq!(Some(2), queue.dequeue());
        assert_eq!(queue.peek(), Some(&1));
    }

    #[test]
    fn queue_peek_mut() {
        let mut queue: QueuePriority<i32> = QueuePriority::new();
        queue.enqueue(1, Priority::LOW);
        queue.enqueue(2, Priority::LOW);
        if let Some(v) = queue.peek_mut() {
            *v = 99;
        }
        assert_eq!(queue.peek(), Some(&99));
        assert_eq!(queue.dequeue(), Some(99));
    }
}
