#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

// https://github.com/PacktPublishing/Hands-On-Data-Structures-and-Algorithms-with-Rust/blob/master/Chapter05/src/red_black_tree.rs
use std::cell::{Ref, RefCell};
use std::cmp;
use std::mem;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct IoTDevice {
    pub numerical_id: u64,
    pub path: String,
    pub address: String,
}

impl IoTDevice {
    pub fn new(id: u64, address: impl Into<String>, path: impl Into<String>) -> IoTDevice {
        IoTDevice {
            address: address.into(),
            numerical_id: id,
            path: path.into(),
        }
    }
}

impl PartialEq for IoTDevice {
    fn eq(&self, other: &IoTDevice) -> bool {
        self.numerical_id == other.numerical_id && self.address == other.address
    }
}

type BareTree = Rc<RefCell<Node>>;
type Tree = Option<BareTree>;

#[derive(Clone, Debug, PartialEq)]
enum Color {
    Red,
    Black,
}

#[derive(PartialEq)]
enum RBOperation {
    LeftNode,
    RightNode,
}

#[derive(PartialEq)]
enum Rotation {
    Left,
    Right,
}

struct Node {
    pub color: Color,
    pub dev: IoTDevice,
    pub parent: Tree,
    left: Tree,
    right: Tree,
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.dev == other.dev
    }
}

impl Node {
    pub fn new(dev: IoTDevice) -> Tree {
        Some(Rc::new(RefCell::new(Node {
            color: Color::Red,
            dev: dev,
            parent: None,
            left: None,
            right: None,
        })))
    }
}

pub struct BetterDeviceRegistry {
    root: Tree,
    pub length: u64,
}

fn display_node(node: BareTree) -> String {
    unsafe {
        let mut s: String = "".into();
        let color = if node.borrow().color == Color::Red {
            "[color=\"red\"]"
        } else {
            "[color=\"black\"]"
        };
        let n1 = node.borrow().dev.numerical_id;

        if let Some(left) = node.borrow().left.clone() {
            s.push_str(&format!(
                "\t{n1}->{n2} {color1}; {n1} {color1}; {n2} {color2};\n",
                n1 = n1,
                n2 = left.borrow().dev.numerical_id,
                color1 = color,
                color2 = if left.borrow().color == Color::Red {
                    "[color=\"red\"]"
                } else {
                    "[color=\"black\"]"
                },
            ));
            s.push_str(&display_node(left));
        } else if (*node.as_ptr()).right.is_some() {
            s.push_str(&format!(
                "\t{n1}->node_null_{n1} [color=\"grey\"]; {n1} {color1};\n",
                n1 = n1,
                color1 = color
            ));
            s.push_str(&format!("\tnode_null_{n1}[label=\"null\"]\n", n1 = n1));
        }

        if let Some(right) = (*node.as_ptr()).right.clone() {
            s.push_str(&format!(
                "\t{n1}->{n2} {color1}; {n1} {color1}; {n2} {color2};\n",
                n1 = n1,
                n2 = right.borrow().dev.numerical_id,
                color2 = if right.borrow().color == Color::Red {
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
                n1 = n1,
                color1 = color
            ));
            s.push_str(&format!("\tnode_null_{}[label=\"null\"]\n", n1));
        }
        s
    }
}

impl BetterDeviceRegistry {
    pub fn new_empty() -> BetterDeviceRegistry {
        BetterDeviceRegistry {
            root: None,
            length: 0,
        }
    }
    pub fn display(&self) -> String {
        if let Some(root) = self.root.clone() {
            return format!("\n\ndigraph Tree {{\n\tratio = fill;\n\tnode [style=filled fontcolor=\"white\"];\n{}}}",display_node(root));
        }
        "\nTree is empty".into()
    }
    pub fn add(&mut self, device: IoTDevice) {
        self.length += 1;
        let root = mem::replace(&mut self.root, None);
        let new_tree = self.add_r(root, device);
        self.root = self.fix_tree(new_tree.1);
    }

    fn check(&self, a: &IoTDevice, b: &IoTDevice) -> RBOperation {
        /*if a.numerical_id <= b.numerical_id {println!("LeftNode");
            RBOperation::LeftNode
        } else {println!("RightNode");
            RBOperation::RightNode
        }*/
        if a.numerical_id <= b.numerical_id {
            println!("LeftNode");
            RBOperation::RightNode
        } else {
            println!("RightNode");
            RBOperation::LeftNode
        }
    }

    fn add_r(&mut self, mut node: Tree, device: IoTDevice) -> (Tree, BareTree) {
        if let Some(n) = node.take() {
            let new: BareTree;
            let current_device = n.borrow().dev.clone();

            match self.check(&current_device, &device) {
                RBOperation::LeftNode => {
                    let left = n.borrow().left.clone();
                    let new_tree = self.add_r(left, device);
                    new = new_tree.1;
                    let new_tree = new_tree.0.unwrap();
                    new_tree.borrow_mut().parent = Some(n.clone());
                    n.borrow_mut().left = Some(new_tree);
                }

                RBOperation::RightNode => {
                    let right = n.borrow().right.clone();
                    let new_tree = self.add_r(right, device);
                    new = new_tree.1;
                    let new_tree = new_tree.0.unwrap();

                    new_tree.borrow_mut().parent = Some(n.clone());
                    n.borrow_mut().right = Some(new_tree);
                }
            }
            (Some(n), new)
        } else {
            let new = Node::new(device);
            (new.clone(), new.unwrap())
        }
    }

    pub fn is_a_valid_red_black_tree(&self) -> bool {
        let result = self.validate(&self.root, Color::Red, 0);
        let red_red = result.0;
        let black_height_min = result.1;
        let black_height_max = result.2;
        red_red == 0 && black_height_min == black_height_max
    }

    // red-red violations, min black-height, max-black-height
    fn validate(
        &self,
        node: &Tree,
        parent_color: Color,
        black_height: usize,
    ) -> (usize, usize, usize) {
        if let Some(n) = node {
            let n = n.borrow();
            let red_red = if parent_color == Color::Red && n.color == Color::Red {
                1
            } else {
                0
            };
            let black_height = black_height
                + match n.color {
                    Color::Black => 1,
                    _ => 0,
                };
            let l = self.validate(&n.left, n.color.clone(), black_height);
            let r = self.validate(&n.right, n.color.clone(), black_height);
            (red_red + l.0 + r.0, cmp::min(l.1, r.1), cmp::max(l.2, r.2))
        } else {
            (0, black_height, black_height)
        }
    }

    fn parent_color(&self, n: &BareTree) -> Color {
        n.borrow().parent.as_ref().unwrap().borrow().color.clone()
    }

    fn fix_tree(&mut self, inserted: BareTree) -> Tree {
        let mut not_root = inserted.borrow().parent.is_some();

        let root = if not_root {
            let mut parent_is_red = self.parent_color(&inserted) == Color::Red;
            let mut n = inserted.clone();
            while parent_is_red && not_root {
                if let Some(uncle) = self.uncle(n.clone()) {
                    let which = uncle.1;
                    let uncle = uncle.0;

                    match which {
                        RBOperation::LeftNode => {
                            // uncle is on the left
                            let mut parent = n.borrow().parent.as_ref().unwrap().clone();
                            if uncle.is_some()
                                && uncle.as_ref().unwrap().borrow().color == Color::Red
                            {
                                let uncle = uncle.unwrap();
                                parent.borrow_mut().color = Color::Black;
                                uncle.borrow_mut().color = Color::Black;
                                parent.borrow().parent.as_ref().unwrap().borrow_mut().color =
                                    Color::Red;

                                n = parent.borrow().parent.as_ref().unwrap().clone();
                            } else {
                                if self.check(&parent.borrow().dev, &n.borrow().dev)
                                    == RBOperation::LeftNode
                                {
                                    // do only if it's a right child
                                    let tmp = n.borrow().parent.as_ref().unwrap().clone();
                                    n = tmp;
                                    self.rotate(n.clone(), Rotation::Right);
                                    parent = n.borrow().parent.as_ref().unwrap().clone();
                                }
                                // until here. then for all black uncles
                                parent.borrow_mut().color = Color::Black;
                                parent.borrow().parent.as_ref().unwrap().borrow_mut().color =
                                    Color::Red;
                                let grandparent = n
                                    .borrow()
                                    .parent
                                    .as_ref()
                                    .unwrap()
                                    .borrow()
                                    .parent
                                    .as_ref()
                                    .unwrap()
                                    .clone();
                                self.rotate(grandparent, Rotation::Left);
                            }
                        }

                        RBOperation::RightNode => {
                            // uncle is on the right
                            let mut parent = n.borrow().parent.as_ref().unwrap().clone();

                            if uncle.is_some()
                                && uncle.as_ref().unwrap().borrow().color == Color::Red
                            {
                                let uncle = uncle.unwrap();

                                parent.borrow_mut().color = Color::Black;
                                uncle.borrow_mut().color = Color::Black;
                                parent.borrow().parent.as_ref().unwrap().borrow_mut().color =
                                    Color::Red;

                                n = parent.borrow().parent.as_ref().unwrap().clone();
                            } else {
                                if self.check(&parent.borrow().dev, &n.borrow().dev)
                                    == RBOperation::RightNode
                                {
                                    // do only if it's a right child
                                    let tmp = n.borrow().parent.as_ref().unwrap().clone();
                                    n = tmp;
                                    self.rotate(n.clone(), Rotation::Left);
                                    parent = n.borrow().parent.as_ref().unwrap().clone();
                                }
                                // until here. then for all black uncles
                                parent.borrow_mut().color = Color::Black;
                                parent.borrow().parent.as_ref().unwrap().borrow_mut().color =
                                    Color::Red;
                                let grandparent = n
                                    .borrow()
                                    .parent
                                    .as_ref()
                                    .unwrap()
                                    .borrow()
                                    .parent
                                    .as_ref()
                                    .unwrap()
                                    .clone();
                                self.rotate(grandparent, Rotation::Right);
                            }
                        }
                    }
                } else {
                    break;
                }

                not_root = n.borrow().parent.is_some();
                if not_root {
                    parent_is_red = self.parent_color(&n) == Color::Red;
                }
            }
            while n.borrow().parent.is_some() {
                let t = n.borrow().parent.as_ref().unwrap().clone();
                n = t;
            }
            Some(n)
        } else {
            Some(inserted)
        };
        root.map(|r| {
            r.borrow_mut().color = Color::Black;
            r
        })
    }

    fn rotate(&self, node: BareTree, direction: Rotation) {
        match direction {
            Rotation::Right => {
                let x = node;
                let y = x.borrow().left.clone();
                x.borrow_mut().left = match y {
                    Some(ref y) => y.borrow().right.clone(),
                    _ => None,
                };

                if y.is_some() {
                    y.as_ref().unwrap().borrow_mut().parent = x.borrow().parent.clone();
                    if y.as_ref().unwrap().borrow().right.is_some() {
                        let r = y.as_ref().unwrap().borrow().right.clone();
                        r.unwrap().borrow_mut().parent = Some(x.clone());
                    }
                }

                if let Some(ref parent) = x.borrow().parent {
                    let insert_direction = self.check(&parent.borrow().dev, &x.borrow().dev);
                    match insert_direction {
                        RBOperation::RightNode => parent.borrow_mut().right = y.clone(),
                        RBOperation::LeftNode => parent.borrow_mut().left = y.clone(),
                    }
                } else {
                    y.as_ref().unwrap().borrow_mut().parent = None;
                }
                y.as_ref().unwrap().borrow_mut().right = Some(x.clone());
                x.borrow_mut().parent = y.clone();
            }
            Rotation::Left => {
                let x = node;
                let y = x.borrow().right.clone();
                x.borrow_mut().right = match y {
                    Some(ref y) => y.borrow().left.clone(),
                    _ => None,
                };

                if y.is_some() {
                    y.as_ref().unwrap().borrow_mut().parent = x.borrow().parent.clone();

                    if y.as_ref().unwrap().borrow().left.is_some() {
                        let l = y.as_ref().unwrap().borrow().left.clone();
                        l.unwrap().borrow_mut().parent = Some(x.clone());
                    }
                }

                if let Some(ref parent) = x.borrow().parent {
                    let insert_direction = self.check(&parent.borrow().dev, &x.borrow().dev);

                    match insert_direction {
                        RBOperation::LeftNode => parent.borrow_mut().left = y.clone(),
                        RBOperation::RightNode => parent.borrow_mut().right = y.clone(),
                    }
                } else {
                    y.as_ref().unwrap().borrow_mut().parent = None;
                }
                y.as_ref().unwrap().borrow_mut().left = Some(x.clone());
                x.borrow_mut().parent = y.clone();
            }
        }
    }

    fn uncle(&self, tree: BareTree) -> Option<(Tree, RBOperation)> {
        let current = tree.borrow();

        if let Some(ref parent) = current.parent {
            let parent = parent.borrow();

            if let Some(ref grandparent) = parent.parent {
                let grandparent = grandparent.borrow();

                match self.check(&grandparent.dev, &parent.dev) {
                    RBOperation::LeftNode => {
                        Some((grandparent.right.clone(), RBOperation::RightNode))
                    }
                    RBOperation::RightNode => {
                        Some((grandparent.left.clone(), RBOperation::LeftNode))
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn find(&self, numerical_id: u64) -> Option<IoTDevice> {
        self.find_r(
            &self.root,
            &IoTDevice::new(numerical_id, "".to_owned(), "".to_owned()),
        )
    }

    fn find_r(&self, node: &Tree, dev: &IoTDevice) -> Option<IoTDevice> {
        match node {
            Some(n) => {
                let n = n.borrow();
                if n.dev.numerical_id == dev.numerical_id {
                    Some(n.dev.clone())
                } else {
                    match self.check(&n.dev, &dev) {
                        RBOperation::LeftNode => self.find_r(&n.left, dev),
                        RBOperation::RightNode => self.find_r(&n.right, dev),
                    }
                }
            }
            _ => None,
        }
    }

    pub fn walk(&self, callback: impl Fn(&IoTDevice) -> ()) {
        self.walk_in_order(&self.root, &callback);
    }

    fn walk_in_order(&self, node: &Tree, callback: &impl Fn(&IoTDevice) -> ()) {
        if let Some(n) = node {
            let n = n.borrow();

            self.walk_in_order(&n.left, callback);
            callback(&n.dev);
            self.walk_in_order(&n.right, callback);
        }
    }
}

/// $ cargo test red_black_tree_rc -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut tree = BetterDeviceRegistry::new_empty();
        let nodes = vec![24, 5, 1, 26, 15, 3, 8 /* ,13,16*/];
        let nodes = 1..=9;
        for i in nodes {
            let s = format!("{}", i);
            tree.add(IoTDevice::new(i, s.as_str(), s.as_str()));
        }

        println!("{}", tree.display());
        assert!(true);
    }
}
