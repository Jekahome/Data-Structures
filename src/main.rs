#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_unsafe)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::cmp::Ordering;
use std::fmt::{self, Debug, Display};
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct Tree<T: Ord + PartialEq + PartialOrd + Display + Clone> {
    pub root: Link<T>,
    count: usize,
    _boo: PhantomData<T>,
}

type Link<T> = Option<NonNull<Node<T>>>;

#[derive(Debug)]
pub struct Node<T: Display> {
    pub left: Link<T>,
    pub right: Link<T>,
    pub parent: Link<T>,
    pub is_red: bool,
    pub value: T,
}

impl<T: Ord + PartialEq + PartialOrd + Display + Clone> Tree<T> {
    pub fn new() -> Self {
        Self {
            root: None,
            count: 0,
            _boo: PhantomData,
        }
    }
    pub fn display(&self) -> String {
        if let Some(root) = self.root {
            return format!("\n\ndigraph Tree {{\n\tratio = fill;\n\tnode [style=filled fontcolor=\"white\"];\n{}}}",display_node(root));
        }
        "\nTree is empty".into()
    }
    unsafe fn rotate_right_item(
        &mut self,
        p_node_a: &mut NonNull<Node<T>>,
        parent: &mut NonNull<Node<T>>,
        is_left: bool,
    ) {
        let mut node_b = (*p_node_a.as_ptr()).left.unwrap();

        (*node_b.as_mut()).parent = Some(*parent);
        (*node_b.as_mut()).is_red = (*p_node_a.as_mut()).is_red;
        (*p_node_a.as_mut()).is_red = true;

        if (*node_b.as_ref()).right.is_some() {
            let mut node_d = (*node_b.as_mut()).right;
            if let Some(ref mut d) = &mut node_d {
                (*d.as_mut()).parent = Some(*p_node_a);
            }
            (*p_node_a.as_mut()).left = node_d;
        } else {
            (*p_node_a.as_mut()).left = None;
        }

        (*node_b.as_mut()).right = Some(*p_node_a);

        if is_left {
            (*parent.as_mut()).left = Some(node_b);
        } else {
            (*parent.as_mut()).right = Some(node_b);
        }
        if let Some(ref mut n_a) = (*node_b.as_mut()).right {
            (*n_a.as_mut()).parent = Some(node_b);
        }
    }

    unsafe fn rotate_right(&mut self, node_a: &mut NonNull<Node<T>>) {
        println!("rotate_right node_a={}", (*node_a.as_ptr()).value);
        if let Some(ref mut parent) = (*node_a.as_mut()).parent {
            if let Some(ref mut p_node_a) = (*parent.as_mut()).right {
                if std::ptr::eq(p_node_a.as_ptr(), node_a.as_ptr()) {
                    self.rotate_right_item(p_node_a, parent, false);
                    return ();
                }
            }
            if let Some(ref mut p_node_a) = (*parent.as_mut()).left {
                if std::ptr::eq(p_node_a.as_ptr(), node_a.as_ptr()) {
                    self.rotate_right_item(p_node_a, parent, true);
                    return ();
                }
            }
        } else {
            println!("ggggggggggggggg");
            println!("node_a={}", (*node_a.as_ptr()).value);

            let mut node_b = (*node_a.as_ptr()).left.unwrap();
            (*node_a.as_mut()).parent = Some(node_b);
            //(*node_a.as_mut()).left = None;
            (*node_a.as_mut()).is_red = false;

            if (*node_b.as_ref()).right.is_some() {
                let mut node_d = (*node_b.as_mut()).right.unwrap();
                (*node_d.as_mut()).parent = Some(*node_a);
                (*node_a.as_mut()).left = Some(node_d);
            } else {
                (*node_a.as_mut()).left = None;
            }

            (*node_b.as_mut()).right = Some(*node_a);
            (*node_b.as_mut()).parent = None;
            (*node_b.as_mut()).is_red = false;

            if let Some(ref mut left) = (*node_b.as_mut()).left {
                (*left.as_mut()).is_red = false;
            }
            self.root = Some(node_b);
        }
    }

    unsafe fn left_rotate_item(
        &mut self,
        p_node_a: &mut NonNull<Node<T>>,
        parent: &mut NonNull<Node<T>>,
        is_left: bool,
    ) {
        let mut node_c = (*p_node_a.as_ptr()).right.unwrap();

        (*p_node_a.as_mut()).is_red = true;
        (*node_c.as_mut()).is_red = false;

        (*node_c.as_mut()).parent = Some(*parent);

        if (*node_c.as_ref()).left.is_some() {
            let mut node_e = (*node_c.as_mut()).left;
            if let Some(ref mut e) = &mut node_e {
                (*e.as_mut()).parent = Some(*p_node_a);
            }
            (*p_node_a.as_mut()).right = node_e;
        } else {
            (*p_node_a.as_mut()).right = None;
        }

        (*node_c.as_mut()).left = Some(*p_node_a);

        if is_left {
            (*parent.as_mut()).left = Some(node_c);
        } else {
            (*parent.as_mut()).right = Some(node_c);
        }
        if let Some(ref mut n_a) = (*node_c.as_mut()).left {
            (*n_a.as_mut()).parent = Some(node_c);
        }
        //(*p_node_a.as_mut()).parent = Some(node_c);
    }

    unsafe fn left_rotate(&mut self, node_a: &mut NonNull<Node<T>>) {
        println!("left_rotate node={}", (*node_a.as_ptr()).value);

        if let Some(ref mut parent) = (*node_a.as_mut()).parent {
            if let Some(ref mut p_node_a) = (*parent.as_mut()).right {
                if std::ptr::eq(p_node_a.as_ptr(), node_a.as_ptr()) {
                    println!("right right right");
                    self.left_rotate_item(p_node_a, parent, false);
                }
            }
            if let Some(ref mut p_node_a) = (*parent.as_mut()).left {
                if std::ptr::eq(p_node_a.as_ptr(), node_a.as_ptr()) {
                    self.left_rotate_item(p_node_a, parent, true);
                }
            }
        } else {
            let mut new_node_a = Node::new_black((*node_a.as_ptr()).value.clone());
            let mut node_c = (*node_a.as_ptr()).right.unwrap();
            if let Some(ref mut n_a) = new_node_a {
                (*n_a.as_mut()).left = (*node_a.as_ptr()).left;
                let _ = std::mem::replace(&mut self.root, Some(node_c));

                (*n_a.as_mut()).parent = self.root;

                (*n_a.as_mut()).is_red = true;

                if let Some(ref mut root) = self.root {
                    (*root.as_mut()).is_red = false;
                    (*root.as_mut()).parent = None;

                    if (*root.as_ref()).left.is_some() {
                        if let Some(ref mut node_e) = &mut (*root.as_mut()).left {
                            (*node_e.as_mut()).parent = Some(*n_a);
                        }
                        println!("+++3");
                        (*n_a.as_mut()).right = (*root.as_ptr()).left;
                        println!("++++4");
                    } else {
                        (*n_a.as_mut()).right = None;
                    }
                    (*root.as_mut()).left = Some(*n_a);
                }
                (*n_a.as_mut()).parent = self.root;
            }
        }
    }
}
fn display_node<T: Display>(node: NonNull<Node<T>>) -> String {
    unsafe {
        let mut s: String = "".into();
        let color = if (*node.as_ptr()).is_red {
            "[color=\"red\"]"
        } else {
            "[color=\"black\"]"
        };

        if let Some(left) = (*node.as_ptr()).left {
            s.push_str(&format!(
                "\t{n1}->{n2} {color1}; {n1} {color1}; {n2} {color2};\n",
                n1 = (*node.as_ptr()).value,
                n2 = (*left.as_ptr()).value,
                color1 = color,
                color2 = if (*left.as_ptr()).is_red {
                    "[color=\"red\"]"
                } else {
                    "[color=\"black\"]"
                },
            ));
            s.push_str(&display_node(left));
        } else if (*node.as_ptr()).right.is_some() {
            s.push_str(&format!(
                "\t{n1}->node_null_{n1} [color=\"grey\"]; {n1} {color1};\n",
                n1 = (*node.as_ptr()).value,
                color1 = color
            ));
            s.push_str(&format!(
                "\tnode_null_{n1}[label=\"null\"]\n",
                n1 = (*node.as_ptr()).value
            ));
        }

        if let Some(right) = (*node.as_ptr()).right {
            s.push_str(&format!(
                "\t{n1}->{n2} {color1}; {n1} {color1}; {n2} {color2};\n",
                n1 = (*node.as_ptr()).value,
                n2 = (*right.as_ptr()).value,
                color2 = if (*right.as_ptr()).is_red {
                    "[color=\"red\"]"
                } else {
                    "[color=\"black\"]"
                },
                color1 = color
            ));
            s.push_str(&display_node(right));
        } else {
            s.push_str(&format!(
                "\t{n1}->node_null_{n1} [color=\"grey\"]; {n1} {color1};\n",
                n1 = (*node.as_ptr()).value,
                color1 = color
            ));
            s.push_str(&format!(
                "\tnode_null_{}[label=\"null\"]\n",
                (*node.as_ptr()).value
            ));
        }
        s
    }
}

impl<T: Display> Node<T> {
    pub fn new_black(value: T) -> Link<T> {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Self {
                left: None,
                right: None,
                parent: None,
                is_red: false,
                value,
            })));
            Some(new)
        }
    }

    pub fn new_red(value: T, parent: NonNull<Node<T>>) -> Link<T> {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Self {
                left: None,
                right: None,
                parent: Some(parent),
                is_red: true,
                value,
            })));
            Some(new)
        }
    }
}

impl<T: Display> Drop for Node<T> {
    fn drop(&mut self) {
        println!("Drop Node={}", self.value);
    }
}

fn test_left_rotation_with_parent() {
    /*
        -- node C is RED ---

                P           =>          P
                |                       |
               A(6)                    C(8)
              /   \                    /   \
           B(5)    C(8)              A(6)  D(9)
                  /    \             /   \
                 E(7)   D(9)      B(5)  E(7)
    */
    unsafe {
        let mut tree: Tree<i32> = Tree::new();
        tree.root = Node::new_black(2);
        if let Some(ref mut root) = &mut tree.root {
            (*root.as_mut()).left = Node::new_red(1, *root);
            (*root.as_mut()).right = Node::new_red(6, *root); // A

            if let Some(ref mut node_a) = &mut (*root.as_mut()).right {
                (*node_a.as_mut()).left = Node::new_red(5, *node_a); // B
                (*node_a.as_mut()).right = Node::new_red(8, *node_a); // C

                if let Some(ref mut node_c) = &mut (*node_a.as_mut()).right {
                    (*node_c.as_mut()).left = Node::new_red(7, *node_c); // E
                    (*node_c.as_mut()).right = Node::new_red(9, *node_c); // D
                }
            }
        }
        if let Some(ref mut root) = &mut tree.root {
            if let Some(ref mut node_a) = &mut (*root.as_mut()).right {
                tree.left_rotate(node_a);
            }
        }

        println!("{}", tree.display());

        if let Some(ref mut root) = &mut tree.root {
            {
                let n1 = (*root.as_ptr()).left.unwrap();
                assert_eq!((*n1.as_ptr()).value, 1, ">>>1");
                let n2 = (*n1.as_ptr()).parent.unwrap();
                assert_eq!((*n2.as_ptr()).value, 2, ">>>2");
            }
            {
                // C
                let node_c = (*root.as_ptr()).right.unwrap();
                assert_eq!((*node_c.as_ptr()).value, 8, ">>>3");
                let node_c_parent = (*node_c.as_ptr()).parent.unwrap();
                assert_eq!((*node_c_parent.as_ptr()).value, 2, ">>>4");

                let node_a = (*node_c.as_ptr()).left.unwrap();
                assert_eq!((*node_a.as_ptr()).value, 6, ">>>5");
                let node_a_parent = (*node_a.as_ptr()).parent.unwrap();
                assert_eq!((*node_a_parent.as_ptr()).value, 8, ">>>6");

                let node_d = (*node_c.as_ptr()).right.unwrap();
                assert_eq!((*node_d.as_ptr()).value, 9, ">>>7");
                let node_d_parent = (*node_d.as_ptr()).parent.unwrap();
                assert_eq!((*node_d_parent.as_ptr()).value, 8, ">>>8");
            }
            {
                // A
                let node_c = (*root.as_ptr()).right.unwrap();
                let node_a = (*node_c.as_ptr()).left.unwrap();
                let node_b = (*node_a.as_ptr()).left.unwrap();
                let node_e = (*node_a.as_ptr()).right.unwrap();
                assert_eq!((*node_b.as_ptr()).value, 5, ">>>9");
                assert_eq!((*node_e.as_ptr()).value, 7, ">>>10");
                let node_b_parent = (*node_b.as_ptr()).parent.unwrap();
                assert_eq!((*node_b_parent.as_ptr()).value, 6, ">>>11");
                let node_e_parent = (*node_e.as_ptr()).parent.unwrap();
                assert_eq!((*node_e_parent.as_ptr()).value, 6, ">>>12");
            }
        }
    }
}

fn test_left_rotation_without_parent() {
    /*
        -- node A is ROOT ---
        -- node C is RED ---

                            =>

               A(6)                    C(8)
              /   \                    /   \
           B(5)    C(8)              A(6)  D(9)
                  /    \             /   \
                 E(7)   D(9)      B(5)  E(7)
    */
    unsafe {
        let mut tree: Tree<i32> = Tree::new();
        tree.root = Node::new_black(6);
        if let Some(ref mut root) = &mut tree.root {
            (*root.as_mut()).left = Node::new_red(5, *root); // B
            (*root.as_mut()).right = Node::new_red(8, *root); // C

            if let Some(ref mut node_c) = &mut (*root.as_mut()).right {
                (*node_c.as_mut()).left = Node::new_red(7, *node_c); // E
                (*node_c.as_mut()).right = Node::new_red(9, *node_c); // D
            }
        }

        tree.left_rotate(&mut tree.root.unwrap());

        println!("{}", tree.display());

        if let Some(ref mut root) = &mut tree.root {
            {
                assert_eq!((*root.as_ptr()).parent, None, ">>>1");
                assert_eq!((*root.as_ptr()).value, 8, ">>>1");
                let node_d = (*root.as_ptr()).right.unwrap();
                assert_eq!((*node_d.as_ptr()).value, 9, ">>>");
                let node_d_parent = (*node_d.as_ptr()).parent.unwrap();
                assert_eq!((*node_d_parent.as_ptr()).value, 8, ">>>");

                let node_a = (*root.as_ptr()).left.unwrap();
                assert_eq!((*node_a.as_ptr()).value, 6, ">>>");
                let node_a_parent = (*node_a.as_ptr()).parent.unwrap();
                assert_eq!((*node_a_parent.as_ptr()).value, 8, ">>>");
            }
            {
                // A
                let node_a = (*root.as_ptr()).left.unwrap();
                let node_b = (*node_a.as_ptr()).left.unwrap();
                let node_e = (*node_a.as_ptr()).right.unwrap();
                assert_eq!((*node_b.as_ptr()).value, 5, ">>>");
                assert_eq!((*node_e.as_ptr()).value, 7, ">>>1");
                let node_b_parent = (*node_b.as_ptr()).parent.unwrap();
                assert_eq!((*node_b_parent.as_ptr()).value, 6, ">>>");
                let node_e_parent = (*node_e.as_ptr()).parent.unwrap();
                assert_eq!((*node_e_parent.as_ptr()).value, 6, ">>>");
            }
        }
    }
}

fn test_right_rotation_with_parent() {
    /*
       -- node E and B is RED ---

        P(5)         =>         P(5)
         |                       |
        / \                     / \
       1   A(26)               1   B(24)
          /    \                  /    \
       B(24)   C(30)           E(15)   A(26)
       /   \                          /    \
    E(15)  D(25)                    D(25)  C(30)

      */
    unsafe {
        let mut tree: Tree<i32> = Tree::new();
        tree.root = Node::new_black(5);
        if let Some(ref mut root) = &mut tree.root {
            (*root.as_mut()).left = Node::new_red(1, *root);
            (*root.as_mut()).right = Node::new_red(26, *root); // A

            if let Some(ref mut node_a) = &mut (*root.as_mut()).right {
                (*node_a.as_mut()).left = Node::new_red(24, *node_a); // B
                (*node_a.as_mut()).right = Node::new_red(30, *node_a); // C

                if let Some(ref mut node_e) = &mut (*node_a.as_mut()).left {
                    (*node_e.as_mut()).left = Node::new_red(15, *node_e); // E
                    (*node_e.as_mut()).right = Node::new_red(25, *node_e); // D
                }
            }
        }
        if let Some(ref mut root) = &mut tree.root {
            if let Some(ref mut node_a) = &mut (*root.as_mut()).right {
                tree.rotate_right(node_a);
            }
        }

        println!("{}", tree.display());

        // Проверить все узлы

        if let Some(ref mut root) = &mut tree.root {
            {
                let n1 = (*root.as_ptr()).left.unwrap();
                assert_eq!((*n1.as_ptr()).value, 1);
                let n2 = (*n1.as_ptr()).parent.unwrap();
                assert_eq!((*n2.as_ptr()).value, 5);
            }

            {
                // B A ----------------------------------------------------------

                let node_b = (*root.as_ptr()).right.unwrap();
                assert_eq!((*node_b.as_ptr()).value, 24, ">>>1");
                let parent = (*node_b.as_ptr()).parent.unwrap();
                assert_eq!((*parent.as_ptr()).value, 5, ">>>2");
                let node_e = (*node_b.as_ptr()).left.unwrap();
                assert_eq!((*node_e.as_ptr()).value, 15, ">>>3");
                let node_e_parent = (*node_e.as_ptr()).parent.unwrap();
                assert_eq!((*node_e_parent.as_ptr()).value, 24, ">>>4");
                let node_a = (*node_b.as_ptr()).right.unwrap();
                assert_eq!((*node_a.as_ptr()).value, 26, ">>>5");
                let node_a_parent = (*node_a.as_ptr()).parent.unwrap();
                assert_eq!((*node_a_parent.as_ptr()).value, 24, ">>>6");
            }

            {
                //D C ----------------------------------------------------------
                let node_b = (*root.as_ptr()).right.unwrap();
                let node_a = (*node_b.as_ptr()).right.unwrap();
                let node_c = (*node_a.as_ptr()).right.unwrap();
                let node_d = (*node_a.as_ptr()).left.unwrap();
                assert_eq!((*node_c.as_ptr()).value, 30, ">>>");
                assert_eq!((*node_d.as_ptr()).value, 25, ">>>");
                let node_c_parent = (*node_c.as_ptr()).parent.unwrap();
                assert_eq!((*node_c_parent.as_ptr()).value, 26, ">>>");
                let node_d_parent = (*node_d.as_ptr()).parent.unwrap();
                assert_eq!((*node_d_parent.as_ptr()).value, 26, ">>>");
            }
        }
    }
}

fn test_right_rotation_without_parent() {
    /*
       -- node A is ROOT ---
       -- node E and B is RED ---
                      =>
           A(26)                   B(24)
          /    \                  /    \
       B(24)   C(30)           E(15)   A(26)
       /   \                          /    \
    E(15)  D(25)                    D(25)  C(30)

      */
    unsafe {
        let mut tree: Tree<i32> = Tree::new();
        tree.root = Node::new_black(26);
        if let Some(ref mut root) = &mut tree.root {
            (*root.as_mut()).left = Node::new_red(24, *root);
            (*root.as_mut()).right = Node::new_red(30, *root); // A

            if let Some(ref mut node_b) = &mut (*root.as_mut()).left {
                (*node_b.as_mut()).left = Node::new_red(15, *node_b); // E
                (*node_b.as_mut()).right = Node::new_red(25, *node_b); // D
            }
        }

        tree.rotate_right(&mut tree.root.unwrap());

        // println!("{}",tree.display());

        if let Some(ref mut root) = &mut tree.root {
            {
                assert_eq!((*root.as_ptr()).parent, None);
                assert_eq!((*root.as_ptr()).value, 24);
                let node_e = (*root.as_ptr()).left.unwrap();
                assert_eq!((*node_e.as_ptr()).value, 15);

                let node_e_parent = (*node_e.as_ptr()).parent.unwrap();
                assert_eq!((*node_e_parent.as_ptr()).value, 24);

                let node_a = (*root.as_ptr()).right.unwrap();
                assert_eq!((*node_a.as_ptr()).value, 26);
                let node_a_parent = (*node_a.as_ptr()).parent.unwrap();
                assert_eq!((*node_a_parent.as_ptr()).value, 24);
            }

            {
                // A ----------------------------------------------------------
                let node_a = (*root.as_ptr()).right.unwrap();
                let node_c = (*node_a.as_ptr()).right.unwrap();
                let node_d = (*node_a.as_ptr()).left.unwrap();
                assert_eq!((*node_c.as_ptr()).value, 30);
                assert_eq!((*node_d.as_ptr()).value, 25);
                let node_c_parent = (*node_c.as_ptr()).parent.unwrap();
                assert_eq!((*node_c_parent.as_ptr()).value, 26);
                let node_d_parent = (*node_d.as_ptr()).parent.unwrap();
                assert_eq!((*node_d_parent.as_ptr()).value, 26);
            }
        }
    }
}

fn main() {
    // test_left_rotation_with_parent();
    test_right_rotation_with_parent();
    // test_left_rotation_without_parent();
    // test_right_rotation_without_parent();
}

/*

Имитировать левый/правый поворот с корнем и без, проверить все связи узла

*/

//----------------------------------------------------------------------
/*#![allow(unused_imports)]

use ds::{stack_array, stack_linked_list, stack_vec};

fn main() {
    println!("Hello, world!");
}
*/
