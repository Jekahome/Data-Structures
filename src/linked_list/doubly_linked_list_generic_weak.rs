//#![allow(dead_code)]
//#![allow(unused_variables)]
//#![allow(unused_imports)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::rc::Weak;

pub use ds_doubly_linked_list_generic_weak::{LinkedList, Node};
///! Двунаправленный связный список очередь
///!
///! Решает недостаток стандартной реализации linked list, а имеено скорость доступа по ключу не за `O(n)`, а за `O(1)` т.е. время доступа по ключу в HashMap
///! Имеет преимущество стандартной реализации linked list - возможность динамически менять размер, время вставки и удаления за `O(1)` в начало/конец
///
///! Отличие от стандартной реализации основанной на владении узлами родителями своими дочерними узлами и операциями клонирования для операций вставки в середину
///! то текущая реализация основана на слабых ссылках Weak и хранении узлов в HashMap в виде владения Rc структурой, что позволяет быстрее получать доступ
///! по ключу за `O(1)` и отсутствие операций клонирования при вставке в середину
///! TODO:RefCell для облегчения синтаксиса при взятии мутабельных ссылок Rc внутри мутабельных блоков HashMap
///!
mod ds_doubly_linked_list_generic_weak {
    use super::*;
    use std::cmp::PartialEq;
    use std::fmt::Debug;
    use std::fmt::Display;
    use std::hash::Hash;

    #[derive(Debug)]
    pub struct Node<T> {
        pub key: T,
        pub left: Option<Weak<RefCell<Node<T>>>>,
        pub right: Option<Weak<RefCell<Node<T>>>>,
    }
    impl<T> Node<T>
    where
        T: Clone + PartialEq + Display + Debug + PartialEq<T> + Hash + Eq + Default,
    {
        pub fn new(
            key: T,
            left: Option<Weak<RefCell<Node<T>>>>,
            right: Option<Weak<RefCell<Node<T>>>>,
        ) -> Self {
            Node { key, left, right }
        }
        pub fn set_left(&mut self, node: Weak<RefCell<Node<T>>>) {
            self.left = Some(node);
        }
        pub fn set_right(&mut self, node: Weak<RefCell<Node<T>>>) {
            self.right = Some(node);
        }
        pub fn get_right(&self, key: &T) -> Option<Weak<RefCell<Node<T>>>> {
            if let Some(ref right) = self.right {
                return unsafe { (&*right.as_ptr()).borrow().get_right(key) };
            }
            None
        }
        pub fn show(&self) {
            print!("{:?}", self.key);
            if let Some(ref right) = self.right {
                print!("->");
                unsafe { (&*right.as_ptr()).borrow().show() };
            }
        }
        pub fn get_key(&self) -> &T {
            &self.key
        }
        pub fn get_keys(&self, buff: &mut String) {
            buff.push_str(&format!("{}", self.key));
            if let Some(ref right) = self.right {
                buff.push_str("->");
                unsafe { (&*right.as_ptr()).borrow().get_keys(buff) };
            }
        }
    }

    #[derive(Debug)]
    pub struct LinkedList<T> {
        pub nodes: HashMap<T, Rc<RefCell<Node<T>>>>,
        pub key_head: T,
        pub key_tail: T,
        pub len: usize,
    }
    impl<T> LinkedList<T>
    where
        T: Clone + PartialEq + Display + Debug + PartialEq<T> + Hash + Eq + Default,
    {
        pub fn new(node: Node<T>) -> Self {
            let key_head = node.key.clone();
            let node_new: Rc<RefCell<Node<T>>> = Rc::new(RefCell::new(node));

            let mut nodes: HashMap<T, Rc<RefCell<Node<T>>>> = HashMap::new();
            nodes.insert(key_head.clone(), node_new);
            Self {
                nodes,
                key_head: key_head.clone(),
                key_tail: key_head,
                len: 1,
            }
        }
        pub fn show(&self) {
            if let Some(ref head) = self.nodes.get(&self.key_head) {
                head.borrow().show();
            }
        }
        pub fn get_keys(&self) -> Option<String> {
            if let Some(ref head) = self.nodes.get(&self.key_head) {
                let mut buff: String = String::from("");
                head.borrow().get_keys(&mut buff);
                return Some(buff);
            }
            None
        }

        /// Вставка после узла
        pub fn insert_after(&mut self, node: Node<T>, in_key: &T) -> bool {
            let mut key_node_right: T = T::default(); //String::from("");
            let key_node_new = node.key.clone();
            let node_new: Rc<RefCell<Node<T>>> = Rc::new(RefCell::new(node));
            if let Some(node_left) = self.nodes.get(in_key) {
                (*node_new.borrow_mut()).left = Some(Rc::downgrade(node_left));
                (*node_new.borrow_mut()).right = None;
                let mut node_left_mut = node_left.borrow_mut();
                if let Some(ref right) = (*node_left_mut).right {
                    // right:Weak<RefCell<Node>>
                    key_node_right = unsafe { (*(*right.as_ptr()).borrow().get_key()).clone() }
                }

                (*node_left_mut).right = Some(Rc::downgrade(&node_new));
                self.len += 1;
            } else {
                return false;
            }
            if key_node_right == T::default() {
                self.key_tail = key_node_new.clone();
                self.nodes.insert(key_node_new, node_new);
                return false;
            }
            if let Some(ref mut node_right) = self.nodes.get_mut(&key_node_right) {
                (*node_right.borrow_mut()).left = Some(Rc::downgrade(&node_new));
                (*node_new.borrow_mut()).right = Some(Rc::downgrade(&node_right));
            }
            self.nodes.insert(key_node_new, node_new);
            true
        }

        pub fn remove(&mut self, key: &T) -> Option<Node<T>> {
            let mut key_node_left: T = T::default();
            let mut key_node_right: T = T::default();
            if let Some(node_remove) = self.nodes.get(&key) {
                if let (Some(ref node_left), Some(ref node_right)) =
                    (&node_remove.borrow().left, &node_remove.borrow().right)
                {
                    //node_left:Weak<RefCell<Node>>
                    key_node_left = unsafe { (*(*node_left.as_ptr()).borrow().get_key()).clone() };
                    key_node_right =
                        unsafe { (*(*node_right.as_ptr()).borrow().get_key()).clone() };
                } else if let Some(ref node_left) = &node_remove.borrow().left {
                    key_node_left = unsafe { (*(*node_left.as_ptr()).borrow().get_key()).clone() };
                } else if let Some(ref node_right) = &node_remove.borrow().right {
                    key_node_right =
                        unsafe { (*(*node_right.as_ptr()).borrow().get_key()).clone() };
                }
            }

            if key_node_left != T::default() && key_node_right != T::default() {
                if let Some(ref node_left) = self.nodes.get(&key_node_left) {
                    if let Some(ref node_right) = self.nodes.get(&key_node_right) {
                        (*node_left.borrow_mut()).right = Some(Rc::downgrade(&node_right));
                        (*node_right.borrow_mut()).left = Some(Rc::downgrade(&node_left));
                        if let Some(node_weak) = self.nodes.remove(key) {
                            if let Ok(remove_node) = Rc::try_unwrap(node_weak) {
                                self.len -= 1;
                                return Some(remove_node.into_inner());
                            }
                        }
                    }
                }
            } else if key_node_left != T::default() {
                if let Some(ref node_left) = self.nodes.get(&key_node_left) {
                    (*node_left.borrow_mut()).right = None;
                    if let Some(node_weak) = self.nodes.remove(key) {
                        if let Ok(remove_node) = Rc::try_unwrap(node_weak) {
                            self.len -= 1;
                            if key == &self.key_tail {
                                self.key_tail = key_node_left.clone();
                            }
                            return Some(remove_node.into_inner());
                        }
                    }
                }
            } else if key_node_right != T::default() {
                if let Some(ref node_right) = self.nodes.get(&key_node_right) {
                    (*node_right.borrow_mut()).left = None;
                    if let Some(node_weak) = self.nodes.remove(key) {
                        if let Ok(remove_node) = Rc::try_unwrap(node_weak) {
                            self.len -= 1;
                            if key == &self.key_head {
                                self.key_head = key_node_right.clone();
                            }
                            return Some(remove_node.into_inner());
                        }
                    }
                }
            }
            None
        }

        pub fn pop_back(&mut self) -> Option<Node<T>> {
            let mut key_node_left: T = T::default();
            if let Some(node_tail) = self.nodes.get(&self.key_tail) {
                if let Some(ref node_left) = &node_tail.borrow().left {
                    key_node_left = unsafe { (*(*node_left.as_ptr()).borrow().get_key()).clone() };
                }
            }
            if key_node_left != T::default() {
                if let Some(node_left) = self.nodes.get(&key_node_left) {
                    (*node_left.borrow_mut()).right = None;
                    if let Some(tail_weak) = self.nodes.remove(&self.key_tail) {
                        if let Ok(tail_node) = Rc::try_unwrap(tail_weak) {
                            self.len -= 1;
                            self.key_tail = key_node_left;
                            return Some(tail_node.into_inner());
                        }
                    }
                }
            }
            None
        }

        pub fn pop_front(&mut self) -> Option<Node<T>> {
            let mut key_node_right: T = T::default();
            if let Some(node_head) = self.nodes.get(&self.key_head) {
                if let Some(ref node_right) = &node_head.borrow().right {
                    key_node_right =
                        unsafe { (*(*node_right.as_ptr()).borrow().get_key()).clone() };
                }
            }
            if key_node_right != T::default() {
                if let Some(node_right) = self.nodes.get(&key_node_right) {
                    (*node_right.borrow_mut()).left = None;
                    if let Some(head_weak) = self.nodes.remove(&self.key_head) {
                        if let Ok(head_node) = Rc::try_unwrap(head_weak) {
                            self.len -= 1;
                            self.key_head = key_node_right;
                            return Some(head_node.into_inner());
                        }
                    }
                }
            }
            None
        }

        pub fn push_back(&mut self, node_new: Node<T>) {
            let new_node_key: T = node_new.get_key().clone();
            let node_new: Rc<RefCell<Node<T>>> = Rc::new(RefCell::new(node_new));
            if let Some(node_tail) = self.nodes.get(&self.key_tail) {
                (*node_tail.borrow_mut()).right = Some(Rc::downgrade(&node_new));
                (*node_new.borrow_mut()).left = Some(Rc::downgrade(&node_tail));
                (*node_new.borrow_mut()).right = None;
            }
            self.len += 1;
            self.key_tail = new_node_key;
            self.nodes.insert(self.key_tail.clone(), node_new);
        }

        pub fn push_front(&mut self, node_new: Node<T>) {
            let new_node_key: T = node_new.get_key().clone();
            let node_new: Rc<RefCell<Node<T>>> = Rc::new(RefCell::new(node_new));
            if let Some(node_head) = self.nodes.get(&self.key_head) {
                (*node_new.borrow_mut()).right = Some(Rc::downgrade(&node_head));
                (*node_new.borrow_mut()).left = None;
                let mut node_head_mut = node_head.borrow_mut();
                (*node_head_mut).left = Some(Rc::downgrade(&node_new));
            }
            self.len += 1;
            self.key_head = new_node_key;
            self.nodes.insert(self.key_head.clone(), node_new);
        }
    }
}

/// $ cargo +nightly miri test doubly_linked_list_generic_weak
/// $ cargo test doubly_linked_list_generic_weak -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;

    #[test]
    fn test_use_cow() -> Result<(), String> {
        let node1: Node<Cow<'static, str>> = Node::new(Cow::Owned("1".to_owned()), None, None);
        let mut linked_list: LinkedList<Cow<'static, str>> = LinkedList::new(node1);

        for i in 2..=5 {
            linked_list.push_back(Node::new(Cow::Owned(format!("{}", i)), None, None));
        }

        assert_eq!(Some(String::from("1->2->3->4->5")), linked_list.get_keys());

        let node: Option<Node<Cow<'static, str>>> = linked_list.remove(&Cow::Borrowed("2"));
        assert_eq!(node.unwrap().get_key(), "2");
        assert_eq!(Some(String::from("1->3->4->5")), linked_list.get_keys());

        linked_list.show();
        let node: Option<Node<Cow<'static, str>>> = linked_list.remove(&Cow::Borrowed("1"));
        assert_eq!(node.unwrap().get_key(), "1");
        assert_eq!(Some(String::from("3->4->5")), linked_list.get_keys());

        let node: Option<Node<Cow<'static, str>>> = linked_list.remove(&Cow::Borrowed("5"));

        assert_eq!(node.unwrap().get_key(), "5");
        assert_eq!(Some(String::from("3->4")), linked_list.get_keys());
        Ok(())
    }

    #[test]
    fn test_use_i32() -> Result<(), String> {
        let node1: Node<usize> = Node::new(1usize, None, None);
        let mut linked_list: LinkedList<usize> = LinkedList::new(node1);

        for i in 2..=5 {
            linked_list.push_back(Node::new(i, None, None));
        }

        assert_eq!(Some(String::from("1->2->3->4->5")), linked_list.get_keys());

        let node: Option<Node<usize>> = linked_list.remove(&2usize);
        assert_eq!(node.unwrap().get_key(), &2usize);
        assert_eq!(Some(String::from("1->3->4->5")), linked_list.get_keys());

        linked_list.show();
        let node: Option<Node<usize>> = linked_list.remove(&1usize);
        assert_eq!(node.unwrap().get_key(), &1usize);
        assert_eq!(Some(String::from("3->4->5")), linked_list.get_keys());

        let node: Option<Node<usize>> = linked_list.remove(&5usize);

        assert_eq!(node.unwrap().get_key(), &5usize);
        assert_eq!(Some(String::from("3->4")), linked_list.get_keys());
        Ok(())
    }

    #[test]
    fn test_insert() -> Result<(), String> {
        let node1: Node<String> = Node::new("node1".to_owned(), None, None);
        let mut linked_list: LinkedList<String> = LinkedList::new(node1);

        let node2: Node<String> = Node::new("node2".to_owned(), None, None);
        linked_list.insert_after(node2, &"node1".to_owned());

        let node3: Node<String> = Node::new("node3".to_owned(), None, None);
        linked_list.insert_after(node3, &"node1".to_owned());

        //linked_list.show();
        assert_eq!(
            Some(String::from("node1->node3->node2")),
            linked_list.get_keys()
        );
        Ok(())
    }

    #[test]
    fn test_push_front() -> Result<(), String> {
        let node1: Node<String> = Node::new("node1".to_owned(), None, None);
        let mut linked_list: LinkedList<String> = LinkedList::new(node1);

        let node2: Node<String> = Node::new("node2".to_owned(), None, None);
        linked_list.insert_after(node2, &"node1".to_owned());

        let node3: Node<String> = Node::new("node3".to_owned(), None, None);
        linked_list.insert_after(node3, &"node1".to_owned());

        let node4: Node<String> = Node::new("node4".to_owned(), None, None);
        linked_list.push_front(node4);

        //linked_list.show();
        assert_eq!(
            Some(String::from("node4->node1->node3->node2")),
            linked_list.get_keys()
        );
        Ok(())
    }

    #[test]
    fn test_push_back() -> Result<(), String> {
        let node1: Node<String> = Node::new("node1".to_owned(), None, None);
        let mut linked_list: LinkedList<String> = LinkedList::new(node1);

        let node2: Node<String> = Node::new("node2".to_owned(), None, None);
        linked_list.insert_after(node2, &"node1".to_owned());

        let node3: Node<String> = Node::new("node3".to_owned(), None, None);
        linked_list.insert_after(node3, &"node1".to_owned());

        let node4: Node<String> = Node::new("node4".to_owned(), None, None);
        linked_list.push_back(node4);

        //linked_list.show();
        assert_eq!(
            Some(String::from("node1->node3->node2->node4")),
            linked_list.get_keys()
        );
        Ok(())
    }

    #[test]
    fn test_pop_front() -> Result<(), String> {
        let node1: Node<String> = Node::new("node1".to_owned(), None, None);
        let mut linked_list: LinkedList<String> = LinkedList::new(node1);

        let node2: Node<String> = Node::new("node2".to_owned(), None, None);
        linked_list.insert_after(node2, &"node1".to_owned());

        let node3: Node<String> = Node::new("node3".to_owned(), None, None);
        linked_list.insert_after(node3, &"node1".to_owned());

        let node4: Node<String> = Node::new("node4".to_owned(), None, None);
        linked_list.push_back(node4);
        assert_eq!(
            Some(String::from("node1->node3->node2->node4")),
            linked_list.get_keys()
        );

        let node: Option<Node<String>> = linked_list.pop_front();
        assert_eq!(node.unwrap().get_key(), "node1");
        assert_eq!(
            Some(String::from("node3->node2->node4")),
            linked_list.get_keys()
        );
        Ok(())
    }

    #[test]
    fn test_pop_back() -> Result<(), String> {
        let node1: Node<String> = Node::new("node1".to_owned(), None, None);
        let mut linked_list: LinkedList<String> = LinkedList::new(node1);

        let node2: Node<String> = Node::new("node2".to_owned(), None, None);
        linked_list.insert_after(node2, &"node1".to_owned());

        let node3: Node<String> = Node::new("node3".to_owned(), None, None);
        linked_list.insert_after(node3, &"node1".to_owned());

        let node4: Node<String> = Node::new("node4".to_owned(), None, None);
        linked_list.push_back(node4);

        let node: Option<Node<String>> = linked_list.pop_back();
        assert_eq!(node.unwrap().get_key(), "node4");

        //linked_list.show();
        assert_eq!(
            Some(String::from("node1->node3->node2")),
            linked_list.get_keys()
        );
        Ok(())
    }

    #[test]
    fn test_remove() -> Result<(), String> {
        let node1: Node<String> = Node::new("node1".to_owned(), None, None);
        let mut linked_list: LinkedList<String> = LinkedList::new(node1);

        let node2: Node<String> = Node::new("node2".to_owned(), None, None);
        linked_list.push_back(node2);

        let node3: Node<String> = Node::new("node3".to_owned(), None, None);
        linked_list.push_back(node3);

        let node4: Node<String> = Node::new("node4".to_owned(), None, None);
        linked_list.push_back(node4);

        let node5: Node<String> = Node::new("node5".to_owned(), None, None);
        linked_list.push_back(node5);

        assert_eq!(
            Some(String::from("node1->node2->node3->node4->node5")),
            linked_list.get_keys()
        );

        let node: Option<Node<String>> = linked_list.remove(&"node2".to_owned());
        assert_eq!(node.unwrap().get_key(), "node2");
        assert_eq!(
            Some(String::from("node1->node3->node4->node5")),
            linked_list.get_keys()
        );

        let node: Option<Node<String>> = linked_list.remove(&"node1".to_owned());
        assert_eq!(node.unwrap().get_key(), "node1");
        assert_eq!(
            Some(String::from("node3->node4->node5")),
            linked_list.get_keys()
        );

        let node: Option<Node<String>> = linked_list.remove(&"node5".to_owned());
        //linked_list.show();
        assert_eq!(node.unwrap().get_key(), "node5");
        assert_eq!(Some(String::from("node3->node4")), linked_list.get_keys());
        Ok(())
    }
}
