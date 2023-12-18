///! Очередь следует правилу (FIFO) «первым пришел - первым ушел» 
///!
///! Основные операции с очередью:
///! - добавить элемент в конец очереди;
///! - удалить элемент из начала очереди;
///! - проверка на пустоту;
///! - проверка на заполненость;
///! - получить значение передней части очереди, не удаляя ее;
pub use ds_queue::Queue;
mod ds_queue {

    #[derive(Debug)]
    pub struct Queue<T> {
        queue: Vec<T>,
    }
    impl<T:std::cmp::PartialEq> Queue<T>
    where
        T: Copy,
    {
        pub fn new() -> Self {
            Queue { queue: Vec::new() }
        }
        pub fn enqueue(&mut self, item: T) {
            self.queue.push(item);
        }
        pub fn dequeue(&mut self) -> Option<T> {
            if !self.is_empty() {
                return Some(self.queue.remove(0));
            }
            None
        }
        pub fn peek(&self) -> Option<&T> {
            self.queue.first()
        }
        pub fn peek_mut(&mut self) -> Option<&mut T> {
            self.queue.first_mut()
        }
        pub fn length(&self) -> usize {
            self.queue.len()
        }
        pub fn is_empty(&self) -> bool {
            self.queue.is_empty()
        }
        pub fn search(&mut self, item: T) -> Option<&mut T>{
            if !self.is_empty() {
                for el in self.queue.iter_mut(){
                    if el == &item{
                        return Some(el);
                    }
                }
            }
            None
        }
    }
}

/// $ cargo test queue_vec
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut queue: Queue<isize> = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        queue.enqueue(4);
        queue.enqueue(5);

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), Some(4));

        assert_eq!(queue.peek(), Some(&5));
        assert_eq!(queue.dequeue(), Some(5));
        assert_eq!(queue.peek(), None);
    }

    #[test]
    fn test_search() {
        let mut queue: Queue<isize> = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        queue.enqueue(4);
        queue.enqueue(5);

        let item = queue.search(4).unwrap();
        *item=99;

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), Some(99));
        assert_eq!(queue.dequeue(), Some(5));
    }

    #[test]
    fn queue_peek() {
        let mut queue: Queue<isize> = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        assert_eq!(queue.peek(), Some(&1));
    }

    #[test]
    fn queue_peek_mut() {
        let mut queue: Queue<isize> = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        if let Some(v) = queue.peek_mut(){
            *v=99;
        }
        assert_eq!(queue.peek(), Some(&99));
        assert_eq!(queue.dequeue(), Some(99));
    }

    #[test]
    fn queue_dequeue() {
        let mut queue: Queue<isize> = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.is_empty(), true);
    }
}
