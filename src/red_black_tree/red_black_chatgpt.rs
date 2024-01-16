use std::cmp::Ord;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Color {
    Red,
    Black,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node<K, V> {
    key: K,
    value: V,
    color: Color,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}

impl<K, V> Node<K, V> {
    fn new(key: K, value: V, color: Color) -> Self {
        Node {
            key,
            value,
            color,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedBlackTree<K, V> {
    root: Option<Box<Node<K, V>>>,
}

impl<K, V> RedBlackTree<K, V>
where
    K: Ord,
{
    pub fn new() -> Self {
        RedBlackTree { root: None }
    }

    // Вставка в красно-черное дерево
    fn insert_helper(
        root: Option<Box<Node<K, V>>>,
        key: K,
        value: V,
    ) -> Option<Box<Node<K, V>>> {
        let mut new_root = root;

        if let Some(mut node) = new_root.take() {
            if key < node.key {
                node.left = Self::insert_helper(node.left, key, value);
            } else if key > node.key {
                node.right = Self::insert_helper(node.right, key, value);
            } else {
                // Если ключ уже существует, обновляем значение
                node.value = value;
            }

            new_root = Some(Box::new(Self::balance(node)));
        } else {
            new_root = Some(Box::new(Node::new(key, value, Color::Black)));
        }

        new_root
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.root = Self::insert_helper(self.root.take(), key, value);
        if let Some(ref mut root) = self.root {
            root.color = Color::Black;
        }
    }

    // Балансировка красно-черного дерева
    fn balance(mut node: Box<Node<K, V>>) -> Node<K, V> {
        if node.color == Color::Red {
            return *node;
        }

        if let Some(ref left) = node.left {
            if left.color == Color::Red {
                if let Some(ref left_left) = left.left {
                    if left_left.color == Color::Red {
                        node = Self::rotate_right(node);
                    }
                }
            }
        }

        if let Some(ref right) = node.right {
            if right.color == Color::Red {
                node = Self::rotate_left(node);
            }
        }

        if let Some(ref left) = node.left {
            if let Some(ref right) = node.right {
                if left.color == Color::Red && right.color == Color::Red {
                    node.color = Color::Red;
                    node.left.as_mut().unwrap().color = Color::Black;
                    node.right.as_mut().unwrap().color = Color::Black;
                }
            }
        }

        *node
    }

    // Левый поворот
    fn rotate_left(mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut new_root = node.right.take().unwrap();
        std::mem::swap(&mut node, &mut new_root.left.as_mut().unwrap());
        node.color = Color::Red;
        new_root.color = Color::Black;
        new_root
    }

    // Правый поворот
    fn rotate_right(mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut new_root = node.left.take().unwrap();
        std::mem::swap(&mut node, &mut new_root.right.as_mut().unwrap());
        node.color = Color::Red;
        new_root.color = Color::Black;
        new_root
    }
}

fn main() {
    let mut tree = RedBlackTree::new();
    tree.insert(3, "Three");
    tree.insert(1, "One");
    tree.insert(4, "Four");

    println!("{:?}", tree);
}
