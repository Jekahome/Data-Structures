pub use ds_stack_2_linked_list::List;
///! [Реализация неизменяемого stack на основе linked list](https://rust-unofficial.github.io/too-many-lists/third-final.html)
///!
///! Плюсы связного списка:
///!    - динамический размер
///! Минусы связного списка:
///!    - поиск елемента за `O(n)` (в худшем случае), для двунаправленного `O(n/2)`
///!
///! Основные операции стека:
///!
///! push_front: добавить элемент в верхнюю часть стека.
///! pop_front: удалить элемент из вершины стека
///! peek: получить значение верхнего элемента, не удаляя его.
///!
///! Дополнительно:
///!
///! peek_mut: получить mut ссылку значения верхнего элемента.  
///! insert: вставить элемент в позицию
///! remove: удалить элемент
///! push_back: добавить элемент в конец
///! pop_back: удалить элемент c хвоста
///! search: поиск елемента
mod ds_stack_2_linked_list {
    use std::cmp::PartialEq;
    use std::fmt::Display;

    pub struct List<T> {
        head: Option<Box<Node<T>>>,
    }

    struct Node<T> {
        elem: T,
        next: Option<Box<Node<T>>>,
    }

    impl<T: Display + PartialEq> Node<T> {
        pub fn contains<V: PartialEq<T>>(&self, elem: V) -> bool {
            if elem == self.elem {
                return true;
            } else if let Some(ref next) = self.next {
                return next.contains(elem);
            }
            return false;
        }

        pub fn insert<V: PartialEq<T>>(&mut self, left_elem: V, new_elem: T) -> bool {
            if left_elem == self.elem {
                self.next.take().map(|node| {
                    self.next = Some(Box::new(Node {
                        elem: new_elem,
                        next: Some(node),
                    }))
                });
                return true;
            } else if let Some(ref mut next) = self.next {
                return next.insert(left_elem, new_elem);
            }
            false
        }

        pub fn remove<V: PartialEq<T>>(&mut self, elem: V) -> Option<T> {
            if let Some(ref mut next) = self.next {
                if elem == next.elem {
                    return self.next.take().map(|node| {
                        self.next = node.next;
                        node.elem
                    });
                }
                return next.remove(elem);
            }
            None
        }

        pub fn push_back(&mut self, elem: T) {
            if self.next.is_none() {
                self.next = Some(Box::new(Node {
                    elem: elem,
                    next: None,
                }));
            } else {
                if let Some(ref mut next) = self.next {
                    next.push_back(elem);
                }
            }
        }

        pub fn pop_back(&mut self) -> Option<T> {
            if self.next.is_some() {
                if let Some(ref mut next) = &mut self.next {
                    if next.next.is_some() {
                        return next.pop_back();
                    }
                }
                return self.next.take().map(|node| node.elem);
            }
            return None;
        }

        pub fn search<V: PartialEq<T> + ?Sized>(&self, elem: &V) -> Option<&T> {
            if *elem == self.elem {
                return Some(&self.elem);
            } else if let Some(ref next) = self.next {
                return next.search(elem);
            }
            return None;
        }
    }

    impl<T: Display + PartialEq> List<T> {
        pub fn new() -> Self {
            List { head: None }
        }

        pub fn contains<V: PartialEq<T>>(&self, elem: V) -> bool {
            if self.head.is_none() {
                return false;
            }
            if let Some(ref node) = self.head {
                return node.contains(elem);
            }
            return false;
        }

        pub fn insert<V: PartialEq<T>>(&mut self, left_elem: V, new_elem: T) -> bool {
            if self.head.is_none() {
                self.push_front(new_elem);
                return true;
            }
            if let Some(ref mut head) = self.head {
                return head.insert(left_elem, new_elem);
            }
            false
        }

        pub fn remove<V: PartialEq<T>>(&mut self, elem: V) -> Option<T> {
            if let Some(ref mut head) = &mut self.head {
                if elem == head.elem {
                    return self.head.take().map(|node| {
                        self.head = node.next;
                        node.elem
                    });
                } else if let Some(ref mut next) = &mut head.next {
                    if elem == next.elem {
                        return head.next.take().map(|node| {
                            head.next = node.next;
                            node.elem
                        });
                    }
                    return head.remove(elem);
                }
            }
            None
        }

        pub fn push_back(&mut self, elem: T) {
            if self.head.is_none() {
                self.head = Some(Box::new(Node {
                    elem: elem,
                    next: None,
                }));
            } else {
                if let Some(ref mut next) = self.head {
                    next.push_back(elem);
                }
            }
        }

        pub fn pop_back(&mut self) -> Option<T> {
            if self.head.is_some() {
                if let Some(ref mut head) = self.head {
                    if head.next.is_some() {
                        return head.pop_back();
                    }
                }
                return self.head.take().map(|node| node.elem);
            }
            return None;
        }

        pub fn get_keys(&self) -> String {
            let mut buff: String = String::from("");
            for node in self.iter() {
                if buff.len() > 0 {
                    buff.push_str("->");
                }
                buff.push_str(&format!("{}", node));
            }
            return buff;
        }

        pub fn search<V: PartialEq<T> + ?Sized>(&self, elem: &V) -> Option<&T> {
            if self.head.is_none() {
                return None;
            }
            if let Some(ref node) = &self.head {
                return node.search(elem);
            }
            return None;
        }

        pub fn push_front(&mut self, elem: T) {
            let new_node = Box::new(Node {
                elem: elem,
                next: self.head.take(),
            });

            self.head = Some(new_node);
        }

        pub fn pop_front(&mut self) -> Option<T> {
            self.head.take().map(|node| {
                self.head = node.next;
                node.elem
            })
        }

        pub fn peek(&self) -> Option<&T> {
            self.head.as_ref().map(|node| &node.elem)
        }

        pub fn peek_mut(&mut self) -> Option<&mut T> {
            self.head.as_mut().map(|node| &mut node.elem)
        }

        pub fn into_iter(self) -> IntoIter<T> {
            IntoIter(self)
        }

        pub fn iter(&self) -> Iter<'_, T> {
            Iter {
                next: self.head.as_deref(),
            }
        }

        pub fn iter_mut(&mut self) -> IterMut<'_, T> {
            IterMut {
                next: self.head.as_deref_mut(),
            }
        }
    }

    impl<T> Drop for List<T> {
        fn drop(&mut self) {
            let mut cur_link = self.head.take();
            while let Some(mut boxed_node) = cur_link {
                cur_link = boxed_node.next.take();
            }
        }
    }

    pub struct IntoIter<T>(List<T>);

    impl<T: Display + PartialEq<T>> Iterator for IntoIter<T> {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            // access fields of a tuple struct numerically
            self.0.pop_front()
        }
    }

    pub struct Iter<'a, T> {
        next: Option<&'a Node<T>>,
    }

    impl<'a, T> Iterator for Iter<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            self.next.map(|node| {
                self.next = node.next.as_deref();
                &node.elem
            })
        }
    }

    pub struct IterMut<'a, T> {
        next: Option<&'a mut Node<T>>,
    }

    impl<'a, T> Iterator for IterMut<'a, T> {
        type Item = &'a mut T;

        fn next(&mut self) -> Option<Self::Item> {
            self.next.take().map(|node| {
                self.next = node.next.as_deref_mut();
                &mut node.elem
            })
        }
    }
}

/// $ cargo test stack_linked_list_2
#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list: List<i32> = List::new();

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
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(|value| *value = 42);

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop_front(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
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
        let mut list = List::new();
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
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }

    #[test]
    fn pop_back() {
        let mut list = List::new();
        assert_eq!(list.pop_back(), None);
        list.push_front("1".to_owned());
        list.push_front("2".to_owned());
        list.push_front("3".to_owned());
        assert_eq!(list.pop_back(), Some("1".to_owned()));
        assert_eq!(list.pop_back(), Some("2".to_owned()));
        assert_eq!(list.pop_back(), Some("3".to_owned()));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_remove() {
        let mut list = List::new();
        assert_eq!(list.remove("3"), None);
        list.push_front("1".to_owned());
        list.push_front("2".to_owned());
        list.push_front("3".to_owned());
        assert_eq!(list.remove("2"), Some("2".to_owned()));
        assert_eq!(list.remove("1"), Some("1".to_owned()));
        assert_eq!(list.remove("3"), Some("3".to_owned()));
        assert_eq!(list.remove("3"), None);
    }

    #[test]
    fn test_insert() {
        let mut list = List::new();
        assert_eq!(list.remove("3"), None);
        list.push_front("1".to_owned());
        assert_eq!(String::from("1"), list.get_keys());
        list.push_front("2".to_owned());
        assert_eq!(String::from("2->1"), list.get_keys());
        list.push_front("3".to_owned());
        assert_eq!(String::from("3->2->1"), list.get_keys());
        list.insert("2", "55".to_owned());
        assert_eq!(String::from("3->2->55->1"), list.get_keys());
        assert_eq!(list.pop_front(), Some("3".to_owned()));
        assert_eq!(String::from("2->55->1"), list.get_keys());
        assert_eq!(list.pop_front(), Some("2".to_owned()));
        assert_eq!(String::from("55->1"), list.get_keys());
        assert_eq!(list.pop_front(), Some("55".to_owned()));
        assert_eq!(String::from("1"), list.get_keys());
        assert_eq!(list.pop_front(), Some("1".to_owned()));
    }
    #[test]
    fn test_search() {
        let mut list = List::new();
        list.push_front("1".to_owned());
        list.push_front("2".to_owned());
        list.push_front("3".to_owned());
        assert_eq!(String::from("3->2->1"), list.get_keys());
        assert_eq!(Some(&"2".to_owned()), list.search("2"));
    }
    #[test]
    fn test_contains() {
        let mut list = List::new();
        list.push_front("1".to_owned());
        list.push_front("2".to_owned());
        list.push_front("3".to_owned());
        assert_eq!(String::from("3->2->1"), list.get_keys());
        assert_eq!(true, list.contains("2"));
        assert_eq!(false, list.contains("4"));
        assert_eq!(list.remove("2"), Some("2".to_owned()));
        assert_eq!(false, list.contains("2"));
    }
}
