#![allow(dead_code)]
#![allow(unused_variables)]

pub use ds_stack_array::Stack;

/// Стек на массивах, фиксированного размера и значения не затираются, возвращает ссылки на значения
/// Визуализация https://www.cs.usfca.edu/~galles/visualization/StackArray.html
///
/// Время поиска `O(n)`, время удаления pop и добавления push и просмотра вершины peek `O(1)`
///
/// Основные операции стека:
///
/// Push: добавить элемент в верхнюю часть стека.
/// Pop: удалить элемент из вершины стека
/// IsEmpty: проверьте, пуст ли стек
/// IsFull: проверьте, заполнен ли стек
/// Peek: получить значение верхнего элемента, не удаляя его.
mod ds_stack_array {
    use core::fmt::Debug;

    #[derive(Debug)]
    pub struct Stack<T: Debug> {
        data: [Option<T>; N],
        index: usize,
    }
    const N: usize = 5;

    impl<T: Debug> Stack<T> {
        pub fn new() -> Self {
            Stack::<T> {
                data: [None, None, None, None, None],
                index: 0,
            }
        }

        pub fn push(&mut self, item: T) -> bool {
            if self.is_full() {
                return false;
            }

            self.data[self.index] = Some(item);
            self.index += 1;
            return true;
        }

        pub fn pop(&mut self) -> Option<&T> {
            if self.is_empty() {
                return None;
            }
            self.index -= 1;
            self.data[self.index].as_ref()
        }

        pub fn peek(&self) -> Option<&T> {
            if !self.data.is_empty() && self.index > 0 {
                let ret = self.data[self.index - 1].as_ref().unwrap();
                return Some::<&T>(ret);
            }
            return None;
        }

        pub fn peek_mut(&mut self) -> Option<&mut T> {
            if !self.data.is_empty() && self.index > 0 {
                let ret = self.data[self.index - 1].as_mut().unwrap();
                return Some::<&mut T>(ret);
            }
            return None;
        }

        pub fn is_full(&self) -> bool {
            self.index >= N
        }

        pub fn is_empty(&self) -> bool {
            self.index == 0
        }
    }
}

/// $ cargo test --lib stack_arr
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
        assert_eq!(true, stack.push(5));
        assert_eq!(false, stack.push(6));

        assert_eq!(Some(&5), stack.peek());
        assert_eq!(Some(&5), stack.pop());
        assert_eq!(Some(&4), stack.pop());
        assert_eq!(Some(&3), stack.pop());
        assert_eq!(Some(&2), stack.pop());
        assert_eq!(Some(&1), stack.pop());
        assert_eq!(None, stack.pop());

        stack.push(1);
        assert_eq!(Some(&1), stack.peek());
        assert_eq!(Some(&1), stack.pop());
        assert!(stack.is_empty());
    }

    #[test]
    fn test_add() {
        let mut stack = Stack::<i32>::new();
        stack.push(8);
        let first = stack.pop();
        assert_eq!(Some(&8), first);
    }

    #[test]
    fn test_empty() {
        let mut stack = Stack::<i32>::new();
        stack.push(8);
        let first = stack.pop();
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
        assert_eq!(Some(&99), stack.pop());
    }
}
