pub use ds_stack::Stack;

/// Время поиска `O(n)`, время удаления pop и добавления push и просмотра вершины peek `O(1)`
///
/// Основные операции стека:
///
/// Push: добавить элемент в верхнюю часть стека.
/// Pop: удалить элемент из вершины стека
/// IsEmpty: проверьте, пуст ли стек
/// Peek: получить значение верхнего элемента, не удаляя его.
mod ds_stack {

    #[derive(Debug)]
    pub struct Stack<T> {
        data: Vec<T>,
    }

    impl<T> Stack<T> {
        pub fn new() -> Self {
            Self {
                data: Vec::<T>::new(),
            }
        }

        pub fn push(&mut self, item: T) {
            self.data.push(item);
        }

        pub fn pop(&mut self) -> Option<T> {
            self.data.pop()
        }

        pub fn peek(&self) -> Option<&T> {
            if !self.data.is_empty() {
                return Some(self.data.last().unwrap());
            }
            return None;
        }

        pub fn peek_mut(&mut self) -> Option<&mut T> {
            if !self.data.is_empty() {
                return Some(self.data.last_mut().unwrap());
            }
            return None;
        }

        pub fn is_empty(&self) -> bool {
            self.data.is_empty()
        }
    }
}

/// $ cargo test --lib stack_vec
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut stack = Stack::<i32>::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(4);
        stack.push(5);

        let first = stack.peek();
        assert_eq!(Some(&5), first);

        assert_eq!(Some(5), stack.pop());
        assert_eq!(Some(4), stack.pop());
        assert_eq!(Some(3), stack.pop());
        assert_eq!(Some(2), stack.pop());
        assert_eq!(Some(1), stack.pop());
        assert_eq!(None, stack.pop());

        stack.push(1);
        assert_eq!(Some(&1), stack.peek());
        assert_eq!(Some(1), stack.pop());
        assert!(stack.is_empty());
    }

    #[test]
    fn test_add() {
        let mut stack = Stack::<i32>::new();
        stack.push(8);
        let first = stack.pop();
        assert_eq!(Some(8), first);
    }

    #[test]
    fn test_empty() {
        let mut stack = Stack::<i32>::new();
        stack.push(8);
        let _ = stack.pop();
        let first = stack.pop();
        assert!(first.is_none());
        assert!(stack.is_empty());
    }

    #[test]
    fn test_peek() {
        let mut stack = Stack::<i32>::new();
        stack.push(8);
        let first = stack.peek();

        assert_eq!(first, Some(&8));
    }

    #[test]
    fn test_peek_mut() {
        let mut stack = Stack::<i32>::new();
        stack.push(8);
        let first = stack.peek_mut();
        if let Some(v) = first {
            *v = 99;
        }
        assert_eq!(Some(99), stack.pop());
    }
}
