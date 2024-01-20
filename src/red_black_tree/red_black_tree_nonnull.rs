#![allow(dead_code)]
#![allow(unused_imports)]

pub use llrb::{Node, Tree};
mod llrb {
    use std::cmp::Ordering;
    use std::fmt::{self, Debug, Display};
    use std::hash::{Hash, Hasher};
    use std::iter::FromIterator;
    use std::marker::PhantomData;
    use std::ptr::NonNull;

    pub struct Tree<T: Ord + PartialEq + PartialOrd + Default + Display + Clone + Debug> {
        fixed_head: Link<T>,
        count: usize,
        _boo: PhantomData<T>,
    }

    type Link<T> = Option<NonNull<Node<T>>>;

    #[derive(Debug)]
    pub struct Node<T: Display> {
        left: Link<T>,
        right: Link<T>,
        parent: Link<T>,
        is_red: bool,
        value: T,
    }

    enum OperationPut {
        Left,
        Right,
        FlipColors,
        Nothing,
    }

    enum OperationRemove {
        RedLeaf,
        BlackLeaf,
        NodeWithChildren,
        BlackNodeWithRedLeaf,
        Unimplemented,
    }

    enum OperationRemoveBlackLeaf {
        LeftRedABlackBRedCleaf,                      // 2.1.1.1
        RightRedABlackBRedCleaf,                     // 2.1.1.2
        LeftRedABlackBleaf,                          // 2.1.2.1
        RightRedABlackBleaf,                         // 2.1.2.2
        BlackARedBWithBlackChildrenLeaf,             // 2.2.4
        BlackARedBWithBlackChildrenRightHaveRedLeaf, // 2.2.1
        BlackALeftBlackBRedDleaf,                    // 2.3.1.1
        BlackARightBlackBRedDleaf,                   // 2.3.1.2
        BlackALeftBlackBleaf,                        // 2.3.2.1
        BlackARightBlackBleaf,                       // 2.3.2.2
        Root,
        Unimplemented,
    }

    impl<T: Ord + PartialEq + PartialOrd + Default + Display + Clone + Debug> Tree<T> {
        pub fn new() -> Self {
            Self {
                fixed_head: None,
                count: 0,
                _boo: PhantomData,
            }
        }

        pub fn node_count(&self) -> usize {
            assert!(self.count != 0 || self.fixed_head.is_none());
            self.count
        }

        pub fn contains(&self, value: T) -> bool {
            !find_node(self.get_root(), value).is_none()
        }

        pub fn get_root(&self) -> Link<T> {
            if let Some(fixed_head) = self.fixed_head {
                unsafe { (*fixed_head.as_ptr()).left }
            } else {
                None
            }
        }

        pub fn remove(&mut self, value: T) -> bool {
            unsafe {
                let fixed_head = self.fixed_head.unwrap();
                if let Some(node) = find_node((*fixed_head.as_ptr()).left, value) {
                    if !self.remove_node(node) {
                        return false;
                    }
                } else {
                    return false;
                }
                self.count -= 1;
                true
            }
        }

        unsafe fn operation_remove(&self, node_x: &NonNull<Node<T>>) -> OperationRemove {
            if (*node_x.as_ref()).is_red
                && (*node_x.as_ref()).left.is_none()
                && (*node_x.as_ref()).right.is_none()
            {
                return OperationRemove::RedLeaf;
            }
            if (*node_x.as_ref()).is_red {
                if (*node_x.as_ref()).left.is_some() && (*node_x.as_ref()).right.is_some() {
                    return OperationRemove::NodeWithChildren;
                }
            }
            if !(*node_x.as_ref()).is_red {
                if (*node_x.as_ref()).left.is_some() && (*node_x.as_ref()).right.is_some() {
                    return OperationRemove::NodeWithChildren;
                } else if (*node_x.as_ref()).left.is_some() && (*node_x.as_ref()).right.is_none() {
                    if let Some(left) = (*node_x.as_ref()).left {
                        if (*left.as_ref()).is_red
                            && (*left.as_ref()).left.is_none()
                            && (*left.as_ref()).right.is_none()
                        {
                            return OperationRemove::BlackNodeWithRedLeaf;
                        }
                    }
                } else if (*node_x.as_ref()).left.is_none() && (*node_x.as_ref()).right.is_none() {
                    return OperationRemove::BlackLeaf;
                }
            }
            panic!();
        }

        unsafe fn operation_remove_black_leaf(
            &self,
            node_x: &NonNull<Node<T>>,
        ) -> OperationRemoveBlackLeaf {
            if let Some(node_a) = (*node_x.as_ref()).parent {
                if (*node_a.as_ref()).is_red {
                    if let Some(node_b) = (*node_a.as_ref()).left {
                        if !std::ptr::eq(node_b.as_ptr(), node_x.as_ptr()) {
                            if !(*node_b.as_ref()).is_red {
                                if let Some(node_c) = (*node_b.as_ref()).left {
                                    if (*node_c.as_ref()).is_red {
                                        return OperationRemoveBlackLeaf::LeftRedABlackBRedCleaf;// 2.1.1.1
                                    }
                                } else {
                                    return OperationRemoveBlackLeaf::LeftRedABlackBleaf;// 2.1.2.1
                                }
                            }
                        }
                    }
                    if let Some(node_b) = (*node_a.as_ref()).right {
                        if !std::ptr::eq(node_b.as_ptr(), node_x.as_ptr()) {
                            if !(*node_b.as_ref()).is_red {
                                if let Some(node_c) = (*node_b.as_ref()).left {
                                    if (*node_c.as_ref()).is_red {
                                        return OperationRemoveBlackLeaf::RightRedABlackBRedCleaf;// 2.1.1.2 
                                    }
                                } else {
                                    return OperationRemoveBlackLeaf::RightRedABlackBleaf;// 2.1.2.2
                                }
                            }
                        }
                    }
                } else {
                    // node A is black
                    if let Some(node_b) = (*node_a.as_ref()).left {
                        if !std::ptr::eq(node_b.as_ptr(), node_x.as_ptr()) {
                            if (*node_b.as_ref()).is_red && (*node_b.as_ref()).right.is_some() {
                                let node_c = (*node_b.as_ref()).right.unwrap();
                                if !(*node_c.as_ref()).is_red {
                                    if (*node_c.as_ref()).left.is_some() {
                                        if let Some(node_d) = (*node_c.as_ref()).left {
                                            if (*node_d.as_ref()).is_red {
                                                return OperationRemoveBlackLeaf::BlackARedBWithBlackChildrenRightHaveRedLeaf;// 2.2.1
                                            }
                                        }
                                    }
                                    return OperationRemoveBlackLeaf::BlackARedBWithBlackChildrenLeaf;// 2.2.2 
                                }
                            }
                            if !(*node_b.as_ref()).is_red && (*node_b.as_ref()).left.is_some() {
                                if let Some(node_d) = (*node_b.as_ref()).left {
                                    if (*node_d.as_ref()).is_red {
                                        return OperationRemoveBlackLeaf::BlackALeftBlackBRedDleaf;// 2.3.1.1
                                    }
                                }
                            }
                            if !(*node_b.as_ref()).is_red {
                                return OperationRemoveBlackLeaf::BlackALeftBlackBleaf;// 2.3.2.1
                            }
                        }
                    }
                    if let Some(node_b) = (*node_a.as_ref()).right {
                        if !std::ptr::eq(node_b.as_ptr(), node_x.as_ptr()) {
                            if !(*node_b.as_ref()).is_red && (*node_b.as_ref()).left.is_some() {
                                if let Some(node_d) = (*node_b.as_ref()).left {
                                    if (*node_d.as_ref()).is_red {
                                        return OperationRemoveBlackLeaf::BlackARightBlackBRedDleaf;// 2.3.1.2
                                    }
                                }
                            }
                            if !(*node_b.as_ref()).is_red {
                                return OperationRemoveBlackLeaf::BlackARightBlackBleaf;// 2.3.2.2
                            }
                        }
                    }
                }
            } else {
                return OperationRemoveBlackLeaf::Root;
            }
            OperationRemoveBlackLeaf::Unimplemented
        }

        unsafe fn remove_black_leaf(&mut self, node_x: NonNull<Node<T>>) -> bool {
            let mut next = node_x;
            let mut removed = false;
            loop {
                match self.operation_remove_black_leaf(&next) {
                    OperationRemoveBlackLeaf::LeftRedABlackBRedCleaf => {
                        self.remove_black_leaf_2_1_1_1_balancing(&next);
                        if !removed {
                            removed = self.remove_leaf(node_x);
                        }
                        break;
                    }
                    OperationRemoveBlackLeaf::RightRedABlackBRedCleaf => {
                        self.remove_black_leaf_2_1_1_2_balancing(&next);
                        if !removed {
                            removed = self.remove_leaf(node_x);
                        }
                        break;
                    }
                    OperationRemoveBlackLeaf::LeftRedABlackBleaf => {
                        self.remove_black_leaf_2_1_2_1_balancing(&next);
                        if !removed {
                            removed = self.remove_leaf(node_x);
                        }
                        break;
                    }
                    OperationRemoveBlackLeaf::RightRedABlackBleaf => {
                        self.remove_black_leaf_2_1_2_2_balancing(&next);
                        if !removed {
                            removed = self.remove_leaf(node_x);
                        }
                        break;
                    }
                    OperationRemoveBlackLeaf::BlackARedBWithBlackChildrenRightHaveRedLeaf => {
                        self.remove_black_leaf_2_2_1_balancing(&next);
                        if !removed {
                            removed = self.remove_leaf(node_x);
                        }
                        break;
                    }
                    OperationRemoveBlackLeaf::BlackARedBWithBlackChildrenLeaf => {
                        self.remove_black_leaf_2_2_2_balancing(&next);
                        if !removed {
                            removed = self.remove_leaf(node_x);
                        }
                        break;
                    }
                    OperationRemoveBlackLeaf::BlackALeftBlackBRedDleaf => {
                        self.remove_black_leaf_2_3_1_1_balancing(&next);
                        if !removed {
                            removed = self.remove_leaf(node_x);
                        }
                        break;
                    }
                    OperationRemoveBlackLeaf::BlackARightBlackBRedDleaf => {
                        self.remove_black_leaf_2_3_1_2_balancing(&next);
                        if !removed {
                            removed = self.remove_leaf(node_x);
                        }
                        break;
                    }
                    OperationRemoveBlackLeaf::BlackALeftBlackBleaf => {
                        next = self.remove_black_leaf_2_3_2_1_balancing(&next);
                        if !removed {
                            removed = self.remove_leaf(node_x);
                        }
                    }
                    OperationRemoveBlackLeaf::BlackARightBlackBleaf => {
                        next = self.remove_black_leaf_2_3_2_2_balancing(&next);
                        if !removed {
                            removed = self.remove_leaf(node_x);
                        }
                    }
                    OperationRemoveBlackLeaf::Root => {
                        removed = true;
                        break;
                    }
                    OperationRemoveBlackLeaf::Unimplemented => {
                        panic!();
                    }
                }
            }
            return removed;
        }

        /*
            2.1.1.1 remove black X

                 P            P
                //           //
               A            B
              / \          / \
             B   X   =>   C   A
            //\              /
           C   D            D
        */
        unsafe fn remove_black_leaf_2_1_1_1_balancing(&mut self, node_x: &NonNull<Node<T>>) {
            let node_a = (*node_x.as_ptr()).parent.unwrap();
            let node_b = (*node_a.as_ptr()).left.unwrap();
            let node_d = (*node_b.as_ptr()).right;
            (*node_b.as_ptr()).parent = (*node_a.as_ptr()).parent;
            if let Some(ref mut parent) = (*node_b.as_ptr()).parent {
                (*parent.as_ptr()).left = Some(node_b);
            }
            (*node_a.as_ptr()).left = node_d;
            if let Some(n_d) = node_d {
                (*n_d.as_ptr()).parent = Some(node_a);
            }
            (*node_a.as_ptr()).is_red = false;
            (*node_b.as_ptr()).right = Some(node_a);
            (*node_a.as_ptr()).parent = Some(node_b);
            (*node_b.as_ptr()).is_red = true;
            if let Some(node_c) = (*node_b.as_ptr()).left {
                (*node_c.as_ptr()).is_red = false;
            }
        }

        /*
          2.1.1.2 remove black X

                P              P
               //             //
              A               C
             / \            /   \
            X   B   =>     A     B
               //           \   /
               C             E D
              / \
             E   D
        */
        unsafe fn remove_black_leaf_2_1_1_2_balancing(&mut self, node_x: &NonNull<Node<T>>) {
            let node_a = (*node_x.as_ptr()).parent.unwrap();
            (*node_a.as_ptr()).is_red = false;
            let parent: NonNull<Node<T>> = (*node_a.as_ptr()).parent.unwrap();
            let node_b = (*node_a.as_ptr()).right.unwrap();
            let node_c = (*node_b.as_ptr()).left.unwrap();
            let node_e = (*node_c.as_ptr()).left;
            let node_d = (*node_c.as_ptr()).right;
            (*parent.as_ptr()).left = Some(node_c);
            (*node_c.as_ptr()).parent = Some(parent);
            (*node_b.as_ptr()).left = node_d;
            if let Some(n_d) = node_d {
                (*n_d.as_ptr()).parent = Some(node_b);
            }
            (*node_c.as_ptr()).right = Some(node_b);
            (*node_b.as_ptr()).parent = Some(node_c);
            (*node_c.as_ptr()).left = Some(node_a);
            (*node_a.as_ptr()).parent = Some(node_c);
            (*node_a.as_ptr()).right = node_e;
            if let Some(n_e) = node_e {
                (*n_e.as_ptr()).parent = Some(node_a);
            }
        }

        /*
            2.1.2.1 remove black X

                 P            P
                //           /
               A            A
              / \          //
             B   X   =>   B

        */
        unsafe fn remove_black_leaf_2_1_2_1_balancing(&mut self, node_x: &NonNull<Node<T>>) {
            let node_a = (*node_x.as_ptr()).parent.unwrap();
            (*node_a.as_ptr()).is_red = false;
            if let Some(node_b) = (*node_a.as_ptr()).left {
                (*node_b.as_ptr()).is_red = true;
            }
        }

        /*
           2.1.2.2 remove black X

                P            P
               //           /
              A     =>     B
             / \          //\
            X   B        A   D
               / \        \
              C   D        C

        */
        unsafe fn remove_black_leaf_2_1_2_2_balancing(&mut self, node_x: &NonNull<Node<T>>) {
            let node_a = (*node_x.as_ptr()).parent.unwrap();
            let node_b = (*node_a.as_ptr()).right.unwrap();
            let node_c = (*node_b.as_ptr()).left;
            let node_d = (*node_b.as_ptr()).right;
            let parent = (*node_a.as_ptr()).parent.unwrap();
            if let Some(left) = (*parent.as_ptr()).left {
                if std::ptr::eq(node_a.as_ptr(), left.as_ptr()) {
                    (*parent.as_ptr()).left = Some(node_b);
                } else {
                    (*parent.as_ptr()).right = Some(node_b);
                }
            } else {
                (*parent.as_ptr()).right = Some(node_b);
            }
            (*node_b.as_ptr()).parent = Some(parent);
            (*node_b.as_ptr()).left = Some(node_a);
            (*node_b.as_ptr()).right = node_d;
            if let Some(n_d) = node_d {
                (*n_d.as_ptr()).parent = Some(node_b);
            }
            (*node_a.as_ptr()).parent = Some(node_b);
            (*node_a.as_ptr()).right = node_c;
            if let Some(n_c) = node_c {
                (*n_c.as_ptr()).parent = Some(node_a);
            }
        }

        /*
            2.2.1 remove black X

                P               P
                |               |
                A               C
              // \            //  \
             B    X    =>    B     A
              \               \   /
               C               D E
              //\
             D   E
        */
        unsafe fn remove_black_leaf_2_2_1_balancing(&mut self, node_x: &NonNull<Node<T>>) {
            let n_a = (*node_x.as_ref()).parent.unwrap();
            if (*n_a.as_ref()).parent.is_some() {
                let parent = (*n_a.as_ptr()).parent.unwrap();
                let mut node_a_from_left = false;
                if let Some(ref mut node_a) = (*parent.as_ptr()).left {
                    if std::ptr::eq(node_a.as_ptr(), n_a.as_ptr()) {
                        node_a_from_left = true;
                    }
                }
                let node_a = if node_a_from_left {
                    (*parent.as_ptr()).left.unwrap()
                } else {
                    (*parent.as_ptr()).right.unwrap()
                };
                let node_b = (*node_a.as_ptr()).left.unwrap();
                let node_c = (*node_b.as_ptr()).right.unwrap();
                let node_e = (*node_c.as_ptr()).right;
                let node_d = (*node_c.as_ptr()).left.unwrap();
                (*node_b.as_ptr()).right = Some(node_d);
                (*node_b.as_ptr()).parent = Some(node_c);
                (*node_d.as_ptr()).parent = Some(node_b);
                (*node_a.as_ptr()).parent = Some(node_c);
                (*node_a.as_ptr()).left = node_e;
                if let Some(n_e) = node_e {
                    (*n_e.as_ptr()).parent = Some(node_a);
                }
                (*node_d.as_ptr()).is_red = false;
                (*node_c.as_ptr()).left = Some(node_b);
                (*node_c.as_ptr()).right = Some(node_a);
                (*node_c.as_ptr()).parent = Some(parent);
                if node_a_from_left {
                    (*parent.as_ptr()).left = Some(node_c);
                } else {
                    (*parent.as_ptr()).right = Some(node_c);
                }
            } else {
                // node_a is root
                if let Some(fixed_head) = self.fixed_head {
                    let node_a = (*fixed_head.as_ptr()).left.unwrap();
                    let node_b = (*node_a.as_ptr()).left.unwrap();
                    let node_c = (*node_b.as_ptr()).right.unwrap();
                    let node_e = (*node_c.as_ptr()).right;
                    let node_d = (*node_c.as_ptr()).left.unwrap();
                    (*node_b.as_ptr()).right = Some(node_d);
                    (*node_b.as_ptr()).parent = Some(node_c);
                    (*node_d.as_ptr()).parent = Some(node_b);
                    (*node_a.as_ptr()).parent = Some(node_c);
                    (*node_a.as_ptr()).left = node_e;
                    if let Some(n_e) = node_e {
                        (*n_e.as_ptr()).parent = Some(node_a);
                    }
                    (*node_d.as_ptr()).is_red = false;
                    (*node_c.as_ptr()).left = Some(node_b);
                    (*node_c.as_ptr()).right = Some(node_a);
                    (*node_c.as_ptr()).parent = None;
                    (*fixed_head.as_ptr()).left = Some(node_c);
                }
            }
        }

        /*
            2.2.2 remove black X

                P            P
                |            |
                A            B
              // \            \
             B    X   =>       A
              \               //
               C             C
        */
        unsafe fn remove_black_leaf_2_2_2_balancing(&mut self, node_x: &NonNull<Node<T>>) {
            let n_a = (*node_x.as_ref()).parent.unwrap();
            if (*n_a.as_ref()).parent.is_some() {
                let parent = (*n_a.as_ptr()).parent.unwrap();
                let mut node_a_from_left = false;
                if let Some(ref mut node_a) = (*parent.as_ptr()).left {
                    if std::ptr::eq(node_a.as_ptr(), n_a.as_ptr()) {
                        node_a_from_left = true;
                    }
                }
                let node_a = if node_a_from_left {
                    (*parent.as_ptr()).left.unwrap()
                } else {
                    (*parent.as_ptr()).right.unwrap()
                };
                let node_b = (*node_a.as_ptr()).left.unwrap();
                (*node_b.as_ptr()).is_red = false;
                (*node_b.as_ptr()).parent = Some(parent);
                let node_c = (*node_b.as_ptr()).right.unwrap();
                (*node_c.as_ptr()).is_red = true;
                (*node_b.as_ptr()).right = Some(node_a);
                (*node_a.as_ptr()).left = Some(node_c);
                (*node_c.as_ptr()).parent = Some(node_a);
                (*node_a.as_ptr()).parent = Some(node_b);
                if node_a_from_left {
                    (*parent.as_ptr()).left = Some(node_b);
                } else {
                    (*parent.as_ptr()).right = Some(node_b);
                }
            } else {
                // node_a is root
                if let Some(fixed_head) = self.fixed_head {
                    let node_a = (*fixed_head.as_ptr()).left.unwrap();
                    let node_b = (*node_a.as_ptr()).left.unwrap();
                    (*node_b.as_ptr()).is_red = false;
                    (*node_b.as_ptr()).parent = None;
                    let node_c = (*node_b.as_ptr()).right.unwrap();
                    (*node_c.as_ptr()).is_red = true;
                    (*node_b.as_ptr()).right = Some(node_a);
                    (*node_a.as_ptr()).left = Some(node_c);
                    (*node_c.as_ptr()).parent = Some(node_a);
                    (*node_a.as_ptr()).parent = Some(node_b);
                    (*fixed_head.as_ptr()).left = Some(node_b);
                }
            }
        }

        /*
          2.3.1.1 remove black X

              A           B
             / \         / \
            B   X  =>   D   A
           // \            /
          D    C          C

        */
        unsafe fn remove_black_leaf_2_3_1_1_balancing(&mut self, node_x: &NonNull<Node<T>>) {
            let node_a = (*node_x.as_ptr()).parent.unwrap();
            if let Some(parent) = (*node_a.as_ptr()).parent {
                let node_b = (*node_a.as_ptr()).left.unwrap();
                let node_c = (*node_b.as_ptr()).right;
                let node_d = (*node_b.as_ptr()).left.unwrap();
                (*node_d.as_ptr()).is_red = false;
                let mut node_a_from_left = false;
                if let Some(p_node_a) = (*parent.as_ptr()).left {
                    if std::ptr::eq(p_node_a.as_ptr(), node_a.as_ptr()) {
                        node_a_from_left = true;
                    }
                }
                (*node_b.as_ptr()).parent = Some(parent);
                (*node_b.as_ptr()).right = Some(node_a);
                (*node_a.as_ptr()).parent = Some(node_b);
                (*node_a.as_ptr()).left = node_c;
                if let Some(n_c) = node_c {
                    (*n_c.as_ptr()).parent = Some(node_a);
                }
                if node_a_from_left {
                    (*parent.as_ptr()).left = Some(node_b);
                } else {
                    (*parent.as_ptr()).right = Some(node_b);
                }
            } else {
                // node_a is root
                if let Some(fixed_head) = self.fixed_head {
                    let node_a = (*fixed_head.as_ptr()).left.unwrap();
                    let node_b = (*node_a.as_ptr()).left.unwrap();
                    let node_c = (*node_b.as_ptr()).right;
                    let node_d = (*node_b.as_ptr()).left.unwrap();
                    (*node_d.as_ptr()).is_red = false;
                    let node_b = (*node_a.as_ptr()).left.unwrap();
                    (*node_b.as_ptr()).parent = None;
                    (*node_b.as_ptr()).right = Some(node_a);
                    (*node_a.as_ptr()).parent = Some(node_b);
                    (*node_a.as_ptr()).left = node_c;
                    if let Some(n_c) = node_c {
                        (*n_c.as_ptr()).parent = Some(node_a);
                    }
                    (*fixed_head.as_ptr()).left = Some(node_b);
                }
            }
        }

        /*
            2.3.1.2 remove black X

              P            P
              |            |
              A            D
             / \         /   \
            X   B  =>   A     B
               //        \   /
               D          C E
              / \
             C   E
        */
        unsafe fn remove_black_leaf_2_3_1_2_balancing(&mut self, node_x: &NonNull<Node<T>>) {
            let node_a = (*node_x.as_ptr()).parent.unwrap();
            if let Some(parent) = (*node_a.as_ptr()).parent {
                let node_b = (*node_a.as_ptr()).right.unwrap();
                let node_d = (*node_b.as_ptr()).left.unwrap();
                let node_c = (*node_d.as_ptr()).left;
                let node_e = (*node_d.as_ptr()).right;
                (*node_d.as_ptr()).is_red = false;
                let mut node_a_from_left = false;
                if let Some(p_node_a) = (*parent.as_ptr()).left {
                    if std::ptr::eq(p_node_a.as_ptr(), node_a.as_ptr()) {
                        node_a_from_left = true;
                    }
                }
                (*node_d.as_ptr()).parent = Some(parent);
                (*node_d.as_ptr()).left = Some(node_a);
                (*node_d.as_ptr()).right = Some(node_b);
                (*node_a.as_ptr()).parent = Some(node_d);
                (*node_b.as_ptr()).parent = Some(node_d);
                (*node_a.as_ptr()).right = node_c;
                if let Some(n_c) = node_c {
                    (*n_c.as_ptr()).parent = Some(node_a);
                }
                (*node_b.as_ptr()).left = node_e;
                if let Some(n_e) = node_e {
                    (*n_e.as_ptr()).parent = Some(node_b);
                }
                if node_a_from_left {
                    (*parent.as_ptr()).left = Some(node_d);
                } else {
                    (*parent.as_ptr()).right = Some(node_d);
                }
            } else {
                // node_a is root
                if let Some(fixed_head) = self.fixed_head {
                    let node_a = (*fixed_head.as_ptr()).left.unwrap();
                    let node_b = (*node_a.as_ptr()).right.unwrap();
                    let node_d = (*node_b.as_ptr()).left.unwrap();
                    let node_c = (*node_d.as_ptr()).left;
                    let node_e = (*node_d.as_ptr()).right;
                    (*node_d.as_ptr()).is_red = false;
                    (*node_d.as_ptr()).parent = None;
                    (*node_d.as_ptr()).left = Some(node_a);
                    (*node_d.as_ptr()).right = Some(node_b);
                    (*node_a.as_ptr()).parent = Some(node_d);
                    (*node_b.as_ptr()).parent = Some(node_d);
                    (*node_a.as_ptr()).right = node_c;
                    if let Some(n_c) = node_c {
                        (*n_c.as_ptr()).parent = Some(node_a);
                    }
                    (*node_b.as_ptr()).left = node_e;
                    if let Some(n_e) = node_e {
                        (*n_e.as_ptr()).parent = Some(node_b);
                    }
                    (*fixed_head.as_ptr()).left = Some(node_d);
                }
            }
        }

        /*
            2.3.2.1 remove black X

             P           P
             |           |
             A           A
            / \   =>    //   => next check node P
           B   X       B

        */
        unsafe fn remove_black_leaf_2_3_2_1_balancing(
            &mut self,
            node_x: &NonNull<Node<T>>,
        ) -> NonNull<Node<T>> {
            let node_a = (*node_x.as_ptr()).parent.unwrap();
            let node_b = (*node_a.as_ptr()).left.unwrap();
            (*node_b.as_ptr()).is_red = true;
            return node_a;
        }

        /*
            2.3.2.2 remove black X

              P            P
              |            |
              A            B
             / \   =>    // \    =>  next check node P
            X   B       A    D
               / \       \
              C   D       C
        */
        unsafe fn remove_black_leaf_2_3_2_2_balancing(
            &mut self,
            node_x: &NonNull<Node<T>>,
        ) -> NonNull<Node<T>> {
            let node_a = (*node_x.as_ptr()).parent.unwrap();
            (*node_a.as_ptr()).is_red = true;
            if let Some(parent) = (*node_a.as_ptr()).parent {
                let node_b = (*node_a.as_ptr()).right.unwrap();
                let node_c = (*node_b.as_ptr()).left;
                let node_d = (*node_b.as_ptr()).right;
                let mut node_a_from_left = false;
                if let Some(p_node_a) = (*parent.as_ptr()).left {
                    if std::ptr::eq(p_node_a.as_ptr(), node_a.as_ptr()) {
                        node_a_from_left = true;
                    }
                }
                (*node_b.as_ptr()).parent = Some(parent);
                (*node_a.as_ptr()).right = node_c;
                if let Some(n_c) = node_c {
                    (*n_c.as_ptr()).parent = Some(node_a);
                }
                (*node_b.as_ptr()).right = node_d;
                if let Some(n_d) = node_d {
                    (*n_d.as_ptr()).parent = Some(node_b);
                }
                (*node_b.as_ptr()).left = Some(node_a);
                (*node_a.as_ptr()).parent = Some(node_b);
                if node_a_from_left {
                    (*parent.as_ptr()).left = Some(node_b);
                } else {
                    (*parent.as_ptr()).right = Some(node_b);
                }
                return node_b;
            } else {
                // node_a is root
                if let Some(fixed_head) = self.fixed_head {
                    let node_a = (*fixed_head.as_ptr()).left.unwrap();
                    (*node_a.as_ptr()).is_red = true;
                    let node_b = (*node_a.as_ptr()).right.unwrap();
                    let node_c = (*node_b.as_ptr()).left;
                    let node_d = (*node_b.as_ptr()).right;
                    (*node_b.as_ptr()).parent = None;
                    (*node_a.as_ptr()).right = node_c;
                    if let Some(n_c) = node_c {
                        (*n_c.as_ptr()).parent = Some(node_a);
                    }
                    (*node_b.as_ptr()).right = node_d;
                    if let Some(n_d) = node_d {
                        (*n_d.as_ptr()).parent = Some(node_b);
                    }
                    (*node_b.as_ptr()).left = Some(node_a);
                    (*node_a.as_ptr()).parent = Some(node_b);
                    (*fixed_head.as_ptr()).left = Some(node_b);
                }
                return self.get_root().unwrap();
            }
        }

        unsafe fn remove_node(&mut self, node: NonNull<Node<T>>) -> bool {
            match self.operation_remove(&node) {
                OperationRemove::RedLeaf => {
                    /*
                      option 1.0.0

                       P
                       ||
                       X

                    */
                    return self.remove_leaf(node);
                }
                OperationRemove::BlackLeaf => {
                    // option 2 black leaf
                    return self.remove_black_leaf(node);
                }
                OperationRemove::NodeWithChildren => {
                    /*
                      option 4.0.0

                             X
                         /      \
                        L        R
                       / \      / \
                          C    L
                         ...  /
                             C
                             ...

                    */
                    // TODO: Strategy to avoid altitude change
                    let min_l_n = self.find_min((*node.as_ref()).right.unwrap());
                    let max_r_n = self.find_max((*node.as_ref()).left.unwrap());
                    if (*min_l_n.as_ref()).is_red {
                        // option 1.0.0
                        std::mem::swap(&mut (*min_l_n.as_ptr()).value, &mut (*node.as_ptr()).value);
                        return self.remove_leaf(min_l_n);
                    } else {
                        if let Some(red_left) = (*max_r_n.as_ptr()).left {
                            if (*red_left.as_ref()).is_red {
                                // option 3.0.0
                                std::mem::swap(
                                    &mut (*max_r_n.as_ptr()).value,
                                    &mut (*node.as_ptr()).value,
                                );
                                std::mem::swap(
                                    &mut (*max_r_n.as_ptr()).value,
                                    &mut (*red_left.as_ptr()).value,
                                );
                                return self.remove_leaf(red_left);
                            }
                        }
                    }
                    // option 2 black leaf
                    let mut res = self.remove_node_attempt_avoid_black_leaf(min_l_n);
                    if !res {
                        res = self.remove_node_attempt_avoid_black_leaf(max_r_n);
                        if !res {
                            res = self.remove_black_leaf(node);
                        }
                    }
                    return res;
                }
                OperationRemove::BlackNodeWithRedLeaf => {
                    /*
                     option 3.0.0

                        X
                       //
                       A

                    */
                    let red_left = (*node.as_ptr()).left.unwrap();
                    std::mem::swap(&mut (*node.as_ptr()).value, &mut (*red_left.as_ptr()).value);
                    return self.remove_leaf(red_left);
                }
                OperationRemove::Unimplemented => {
                    panic!();
                }
            }
        }

        unsafe fn remove_node_attempt_avoid_black_leaf(
            &mut self,
            node_x: NonNull<Node<T>>,
        ) -> bool {
            match self.operation_remove_black_leaf(&node_x) {
                OperationRemoveBlackLeaf::LeftRedABlackBRedCleaf => {
                    println!("2.1.1.1");
                    self.remove_black_leaf_2_1_1_1_balancing(&node_x);
                    return self.remove_leaf(node_x);
                }
                OperationRemoveBlackLeaf::RightRedABlackBRedCleaf => {
                    println!("2.1.1.2");
                    self.remove_black_leaf_2_1_1_2_balancing(&node_x);
                    return self.remove_leaf(node_x);
                }
                OperationRemoveBlackLeaf::LeftRedABlackBleaf => {
                    println!("2.1.2.1");
                    self.remove_black_leaf_2_1_2_1_balancing(&node_x);
                    return self.remove_leaf(node_x);
                }
                OperationRemoveBlackLeaf::RightRedABlackBleaf => {
                    println!("2.1.2.2");
                    self.remove_black_leaf_2_1_2_2_balancing(&node_x);
                    return self.remove_leaf(node_x);
                }
                OperationRemoveBlackLeaf::BlackARedBWithBlackChildrenRightHaveRedLeaf => {
                    println!("2.2.1");
                    self.remove_black_leaf_2_2_1_balancing(&node_x);
                    return self.remove_leaf(node_x);
                }
                OperationRemoveBlackLeaf::BlackARedBWithBlackChildrenLeaf => {
                    println!("2.2.2");
                    self.remove_black_leaf_2_2_2_balancing(&node_x);
                    return self.remove_leaf(node_x);
                }
                OperationRemoveBlackLeaf::BlackALeftBlackBRedDleaf => {
                    println!("2.3.1.1");
                    self.remove_black_leaf_2_3_1_1_balancing(&node_x);
                    return self.remove_leaf(node_x);
                }
                OperationRemoveBlackLeaf::BlackARightBlackBRedDleaf => {
                    println!("2.3.1.2");
                    self.remove_black_leaf_2_3_1_2_balancing(&node_x);
                    return self.remove_leaf(node_x);
                }
                _ => {
                    return false;
                }
            }
        }

        unsafe fn find_max(
            &self,
            node: NonNull<Node<T>>
        ) -> NonNull<Node<T>>{
            if (*node.as_ref()).right.is_some() {
                self.find_max((*node.as_ref()).right.unwrap())
            } else {
                node
            }
        }

        unsafe fn find_min(
            &self,
            node: NonNull<Node<T>>,
        ) -> NonNull<Node<T>>{
            if (*node.as_ref()).left.is_some() {
                self.find_min((*node.as_ref()).left.unwrap())
            } else {
                node
            }
        }

        unsafe fn remove_leaf(&mut self, node: NonNull<Node<T>>) -> bool {
            if let Some(mut parent) = (*node.as_ref()).parent {
                if let Some(ref mut left) = (*parent.as_mut()).left {
                    if std::ptr::eq(left.as_ptr(), node.as_ptr()) {
                        (*parent.as_mut()).left = None;
                    } else {
                        (*parent.as_mut()).right = None;
                    }
                } else {
                    (*parent.as_mut()).right = None;
                }
            }
            let _ = Box::from_raw(node.as_ptr());
            true
        }

        unsafe fn remove_tree(&mut self, node: Link<T>) {
            if let Some(node) = node {
                self.remove_tree((*node.as_ref()).left);
                self.remove_tree((*node.as_ref()).right);
                if self.remove_leaf(node) {
                    assert!(self.count > 0);
                    self.count -= 1;
                }
            }
        }

        /// TODO: open http://www.webgraphviz.com/?tab=map
        /// or https://dreampuf.github.io/GraphvizOnline/
        pub fn display(&self) -> String {
            unsafe {
                let fixed_head = self.fixed_head.unwrap();
                if let Some(root) = (*fixed_head.as_ptr()).left {
                    return format!("\n\ndigraph Tree {{\n\tratio = fill;\n\tnode [style=filled fontcolor=\"white\"];\n{}}}",display_node(root));
                }
                "\nTree is empty".into()
            }
        }

        // red-red violations, min black-height, max-black-height
        unsafe fn validate(
            node: &Link<T>,
            is_red: bool,
            black_height: usize,
        ) -> (usize, usize, usize) {
            if let Some(n) = node {
                let red_red = if is_red && (*n.as_ref()).is_red { 1 } else { 0 };
                let black_height = black_height
                    + match (*n.as_ref()).is_red {
                        false => 1,
                        _ => 0,
                    };
                let l = Tree::validate(&(*n.as_ref()).left, (*n.as_ref()).is_red, black_height);
                let r = Tree::validate(&(*n.as_ref()).right, (*n.as_ref()).is_red, black_height);
                (
                    red_red + l.0 + r.0,
                    std::cmp::min(l.1, r.1),
                    std::cmp::max(l.2, r.2),
                )
            } else {
                (0, black_height, black_height)
            }
        }

        pub fn helper_is_a_valid_red_black_tree(&self) -> bool {
            if self.node_count() > 0 {
                unsafe {
                    let fixed_head = self.fixed_head.unwrap();
                    let result = Tree::validate(&(*fixed_head.as_ptr()).left, true, 0);
                    let red_red = result.0;
                    let black_height_min = result.1;
                    let black_height_max = result.2;
                    println!("Validation black height = {}", black_height_min);
                    return red_red == 0 && black_height_min == black_height_max;
                }
            }
            false
        }

        /*
            Rotate left without parent
            A is root

               A                  C
              / \\               //\
             B    C     =>      A   D
                 / \           / \
                E   D         B   E

       
            Rotate left  with parent

               P                  P
               |                  |
               A                  C
              / \\               //\
             B    C     =>      A   D
                 / \           / \
                E   D         B   E

        */
        unsafe fn rotate_left(&mut self, node_a: NonNull<Node<T>>) -> Link<T> {
            if let Some(parent) = (*node_a.as_ptr()).parent{
                let mut node_a_from_left = false;
                if let Some(p_node_a) = (*parent.as_ptr()).left {
                    if std::ptr::eq(p_node_a.as_ptr(), node_a.as_ptr()) {
                        node_a_from_left = true;
                    }
                }
                let node_a = if node_a_from_left {
                    (*parent.as_ptr()).left.unwrap()
                } else {
                    (*parent.as_ptr()).right.unwrap()
                };
                let node_c = (*node_a.as_ptr()).right.unwrap();
                (*node_c.as_ptr()).is_red = (*node_a.as_ptr()).is_red; // childNode принимает цвет своего parentNode
                (*node_a.as_ptr()).is_red = true; // цвет parentNode всегда определяется как красный
                (*node_c.as_ptr()).parent = Some(parent);
                if (*node_c.as_ref()).left.is_some() {
                    let mut node_e = (*node_c.as_ptr()).left;
                    if let Some(ref mut e) = &mut node_e {
                        (*e.as_ptr()).parent = Some(node_a);
                    }
                    (*node_a.as_ptr()).right = node_e;
                } else {
                    (*node_a.as_ptr()).right = None;
                }
                (*node_c.as_ptr()).left = Some(node_a);
                (*node_a.as_ptr()).parent = Some(node_c);
                if node_a_from_left {
                    (*parent.as_ptr()).left = Some(node_c);
                    return (*parent.as_ptr()).left;
                } else {
                    (*parent.as_ptr()).right = Some(node_c);
                    return (*parent.as_ptr()).right;
                }
            }else{
                let fixed_head = self.fixed_head.unwrap();
                let node_a = (*fixed_head.as_ptr()).left.unwrap();
                let node_c = (*node_a.as_ptr()).right.unwrap();
                (*node_c.as_ptr()).is_red = (*node_a.as_ptr()).is_red; // childNode принимает цвет своего parentNode
                (*node_a.as_ptr()).is_red = true; // цвет parentNode всегда определяется как красный
                if (*node_c.as_ref()).left.is_some() {
                    let mut node_e = (*node_c.as_ptr()).left;
                    if let Some(ref mut e) = &mut node_e {
                        (*e.as_ptr()).parent = Some(node_a);
                    }
                    (*node_a.as_ptr()).right = node_e;
                } else {
                    (*node_a.as_ptr()).right = None;
                }
                (*node_c.as_ptr()).left = Some(node_a);
                (*node_a.as_ptr()).parent = Some(node_c);
                (*node_c.as_ptr()).parent = None;
                (*fixed_head.as_ptr()).left = Some(node_c);
                return (*fixed_head.as_ptr()).left;
            }
        }

        pub unsafe fn helper_checking_connections(node: Link<T>) -> bool {
            if let Some(n) = node {
                if (*n.as_ptr()).parent.is_none() {
                    println!("\nROOT:[{}]", (*n.as_ptr()).value);
                }
                if (*n.as_ptr()).left.is_some() && (*n.as_ptr()).right.is_some() {
                    let left = (*n.as_ptr()).left.unwrap();
                    let right = (*n.as_ptr()).right.unwrap();
                    println!(
                        "[{}] <- [{}] -> [{}]",
                        (*left.as_ptr()).value,
                        (*n.as_ptr()).value,
                        (*right.as_ptr()).value
                    );

                    let p = (*left.as_ptr()).parent.unwrap();
                    assert_eq!(
                        (*n.as_ptr()).value,
                        (*p.as_ptr()).value,
                        "Нарушена left связь с {:?} родителем",
                        (*n.as_ptr()).value
                    );
                    let p = (*right.as_ptr()).parent.unwrap();
                    assert_eq!(
                        (*n.as_ptr()).value,
                        (*p.as_ptr()).value,
                        "Нарушена right связь с {:?} родителем",
                        (*n.as_ptr()).value
                    );
                } else if (*n.as_ptr()).left.is_some() && (*n.as_ptr()).right.is_none() {
                    if let Some(left) = (*n.as_ptr()).left {
                        println!(
                            "[{}] <- [{}] -> [NULL]",
                            (*left.as_ptr()).value,
                            (*n.as_ptr()).value
                        );
                        let p = (*left.as_ptr()).parent.unwrap();
                        assert_eq!(
                            (*n.as_ptr()).value,
                            (*p.as_ptr()).value,
                            "Нарушена связь с {:?} родителем",
                            (*n.as_ptr()).value
                        );
                    }
                } else if (*n.as_ptr()).left.is_none() && (*n.as_ptr()).right.is_some() {
                    if let Some(right) = (*n.as_ptr()).right {
                        println!(
                            "[NULL] <- [{}] -> [{}]",
                            (*n.as_ptr()).value,
                            (*right.as_ptr()).value
                        );
                        let p = (*right.as_ptr()).parent.unwrap();
                        assert_eq!(
                            (*n.as_ptr()).value,
                            (*p.as_ptr()).value,
                            "Нарушена связь с {:?} родителем",
                            (*n.as_ptr()).value
                        );
                    }
                } else {
                    println!("[{}]", (*n.as_ptr()).value);
                }
                Tree::helper_checking_connections((*n.as_ptr()).left);
                Tree::helper_checking_connections((*n.as_ptr()).right);
            }
            true
        }

        /*
            Rotate right without parent
            A is root

                 A               B
               // \             / \\
               B   C     =>    E    A
             // \                  / \
            E    D                D   C

         
            Rotate right with parent

                 P               P
                 |               |
                 A               B
               // \             / \\
               B   C     =>    E    A
             // \                  / \
            E    D                D   C

        */
        unsafe fn rotate_right(&mut self, node_a: NonNull<Node<T>>) -> Link<T> {
            if let Some(parent) = (*node_a.as_ptr()).parent{
                let mut node_a_from_left = false;
                if let Some(p_node_a) = (*parent.as_ptr()).left {
                    if std::ptr::eq(p_node_a.as_ptr(), node_a.as_ptr()) {
                        node_a_from_left = true;
                    }
                }
                let node_a = if node_a_from_left {
                    (*parent.as_ptr()).left.unwrap()
                } else {
                    (*parent.as_ptr()).right.unwrap()
                };
                let node_b = (*node_a.as_ptr()).left.unwrap();
                (*node_b.as_ptr()).parent = Some(parent);
                (*node_b.as_ptr()).is_red = (*node_a.as_ptr()).is_red; // childNode принимает цвет своего parentNode
                (*node_a.as_ptr()).is_red = true; // цвет parentNode всегда определяется как красный
                if (*node_b.as_ref()).right.is_some() {
                    let mut node_d = (*node_b.as_ptr()).right;
                    if let Some(ref mut d) = &mut node_d {
                        (*d.as_ptr()).parent = Some(node_a);
                    }
                    (*node_a.as_ptr()).left = node_d;
                } else {
                    (*node_a.as_ptr()).left = None;
                }
                (*node_b.as_ptr()).right = Some(node_a);
                (*node_a.as_ptr()).parent = Some(node_b);
                if node_a_from_left {
                    (*parent.as_ptr()).left = Some(node_b);
                    return (*parent.as_ptr()).left;
                } else {
                    (*parent.as_ptr()).right = Some(node_b);
                    return (*parent.as_ptr()).right;
                }
            }else{
                let fixed_head = self.fixed_head.unwrap();
                let node_a = (*fixed_head.as_ptr()).left.unwrap();
                let node_b = (*node_a.as_ptr()).left.unwrap();
                (*node_b.as_ptr()).is_red = (*node_a.as_ptr()).is_red; // childNode принимает цвет своего parentNode
                (*node_a.as_ptr()).is_red = true; // цвет parentNode всегда определяется как красный
                if (*node_b.as_ref()).right.is_some() {
                    let mut node_d = (*node_b.as_ptr()).right;
                    if let Some(ref mut d) = &mut node_d {
                        (*d.as_ptr()).parent = Some(node_a);
                    }
                    (*node_a.as_ptr()).left = node_d;
                } else {
                    (*node_a.as_ptr()).left = None;
                }
                (*node_b.as_ptr()).right = Some(node_a);
                (*node_a.as_ptr()).parent = Some(node_b);
                (*node_b.as_ptr()).parent = None;
                (*fixed_head.as_ptr()).left = Some(node_b);
                return (*fixed_head.as_ptr()).left;
            }
        }

        unsafe fn flip_colors(&mut self, mut node: NonNull<Node<T>>) {
            if (*node.as_ref()).left.is_some() && (*node.as_ref()).right.is_some() {
                if let Some(ref mut left) = (*node.as_mut()).left {
                    (*left.as_mut()).is_red = false;
                }
                if let Some(ref mut right) = (*node.as_mut()).right {
                    (*right.as_mut()).is_red = false;
                }
                (*node.as_mut()).is_red = true;

                if (*node.as_ptr()).parent.is_none() {
                    (*node.as_mut()).is_red = false;
                }
            }
        }

        unsafe fn check_put_balancing(&mut self, node: NonNull<Node<T>>) -> OperationPut {
            if (*node.as_ref()).right.is_some() {
                let r = (*node.as_ref()).right.unwrap();
                if (*node.as_ref()).left.is_some() {
                    if !(*node.as_ref()).is_red {
                        let l = (*node.as_ref()).left.unwrap();
                        if (*l.as_ref()).is_red && (*r.as_ref()).is_red {
                            return OperationPut::FlipColors;
                        }
                    }
                }
                if (*r.as_ref()).is_red {
                    return OperationPut::Left;
                }
            }
            if (*node.as_ref()).is_red && (*node.as_ref()).left.is_some() {
                let l = (*node.as_ref()).left.unwrap();
                if (*l.as_ref()).is_red {
                    return OperationPut::Right;
                }
            }
            return OperationPut::Nothing;
        }

        pub fn put(&mut self, value: T) -> bool {
            unsafe {
                if self.fixed_head.is_some() {
                    let parent = self.find_put_parent_candidate(self.get_root(), &value);
                    if parent.is_some() {
                        let parent = parent.unwrap();
                        if self.attach_node(parent, value) {
                            self.put_balancing(parent);
                        }
                    } else {
                        return false;
                    }
                } else {
                    self.fixed_head = Node::new_fixed_head(value);
                }
                self.count += 1;
                true
            }
        }

        unsafe fn put_balancing(&mut self, next: NonNull<Node<T>>) {
            let mut next = next;
            loop {
                match self.check_put_balancing(next) {
                    OperationPut::Left => {
                        if let Some(n) = self.rotate_left(next) {
                            next = n;
                        }
                    }
                    OperationPut::Right => {
                        if (*next.as_ptr()).parent.is_some() {
                            let node_a = (*next.as_ptr()).parent.unwrap();
                            if let Some(n) = self.rotate_right(node_a) {
                                next = n;
                            }
                        }
                    }
                    OperationPut::FlipColors => {
                        self.flip_colors(next);
                        if (*next.as_ptr()).parent.is_some() {
                            next = (*next.as_ptr()).parent.unwrap();
                        } else {
                            break;
                        }
                    }
                    OperationPut::Nothing => {
                        if (*next.as_ptr()).parent.is_some() {
                            next = (*next.as_ptr()).parent.unwrap();
                        } else {
                            break;
                        }
                    }
                }
            } 
        }

        unsafe fn find_put_parent_candidate(&mut self, parent: Link<T>, elem: &T) -> Link<T> {
            if let Some(parent) = parent {
                match elem.cmp(&(*parent.as_ref()).value) {
                    Ordering::Equal => {
                        return None;
                    }
                    Ordering::Less => {
                        if (*parent.as_ptr()).left.is_some() {
                            return self.find_put_parent_candidate((*parent.as_ptr()).left, elem);
                        } else {
                            return Some(parent);
                        }
                    }
                    Ordering::Greater => {
                        if (*parent.as_ptr()).right.is_some() {
                            return self.find_put_parent_candidate((*parent.as_ptr()).right, elem);
                        } else {
                            return Some(parent);
                        }
                    }
                }
            } else {
                return parent;
            }
        }

        unsafe fn attach_node(&mut self, parent: NonNull<Node<T>>, elem: T) -> bool {
            match elem.cmp(&(*parent.as_ref()).value) {
                Ordering::Equal => {
                    return false;
                }
                Ordering::Less => {
                    (*parent.as_ptr()).left = Node::new_red(elem, parent);
                    return true;
                }
                Ordering::Greater => {
                    (*parent.as_ptr()).right = Node::new_red(elem, parent);
                    return true;
                }
            }
        }
    }

    impl<T: Default + Display> Node<T> {
        pub fn new_fixed_head(value: T) -> Link<T> {
            unsafe {
                let fixed_head = NonNull::new_unchecked(Box::into_raw(Box::new(Self {
                    left: None,
                    right: None,
                    parent: None,
                    is_red: false,
                    value: T::default(),
                })));
                (*fixed_head.as_ptr()).left = Node::new_black(value); //TODO: без обратной ссылки на parent  
                Some(fixed_head)
            }
        }
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

    impl<T: Ord + PartialEq + PartialOrd + Default + Display + Clone + Debug> Drop for Tree<T> {
        fn drop(&mut self) {
            unsafe {
                if let Some(fixed_head) = self.fixed_head {
                    self.remove_tree((*fixed_head.as_ptr()).left);
                    let _ = Box::from_raw(fixed_head.as_ptr());
                }
                self.fixed_head = None;
            }
        }
    }

    impl<T: Display> Drop for Node<T> {
        fn drop(&mut self) {
            println!("Drop Node={}", self.value);
        }
    }

    fn find_node<T: Ord + PartialEq + PartialOrd + Display>(
        fromnode: Link<T>,
        value: T,
    ) -> Link<T> {
        unsafe {
            if let Some(fromnode) = fromnode {
                match value.cmp(&(*fromnode.as_ptr()).value) {
                    Ordering::Equal => Some(fromnode),
                    Ordering::Less => find_node((*fromnode.as_ptr()).left, value),
                    Ordering::Greater => find_node((*fromnode.as_ptr()).right, value),
                }
            } else {
                fromnode
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
}

/// $ cargo test red_black_tree_nonnull -- --test-threads=1
#[cfg(test)]
mod tests {
    use super::*;

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_success -- --nocapture
    #[test]
    fn test_success() {
        let mut tree: Tree<i32> = Tree::new();
        let nodes = vec![
            480, 978, 379, 784, 999, 695, 23, 97, 309, 312, 449, 958, 992, 220, 95, 257, 869, 959,
            450, 258, 315, 783, 731, 914, 880, 984, 734, 570, 801, 908, 181, 466, 238, 916, 77,
            801, 867, 382, 943, 603, 65, 545, 200, 759, 158, 987, 821, 630, 537, 704, 149, 617,
            498, 261, 160, 192, 760, 417, 939, 757, 858, 376, 885, 336, 764, 443, 155, 983, 586,
            957, 375, 893, 707, 255, 811, 86, 370, 384, 177, 834, 177, 834, 313, 209, 623, 176,
            875, 748, 949, 529, 932, 369, 385, 419, 222, 719, 342, 68, 156, 314, 343, 262, 467,
            499, 604, 732, 758, 765, 812, 859, 876,
        ];
        for i in nodes {
            tree.put(i);
        }

        println!("{}", tree.display());
        unsafe {
            Tree::helper_checking_connections(tree.get_root());
        }
        assert!(tree.helper_is_a_valid_red_black_tree());
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_rotate_right_success -- --nocapture
    #[test]
    fn test_rotate_right_success() {
        let mut tree: Tree<i32> = Tree::new();
        let nodes = vec![575, 396, 139, 792, 546, 73, 7];
        for i in nodes {
            tree.put(i);
        }
        unsafe {
            Tree::helper_checking_connections(tree.get_root());
        }
        assert!(tree.helper_is_a_valid_red_black_tree());
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_find_success -- --nocapture
    #[test]
    fn test_find_success() {
        let mut tree = Tree::new();
        let nodes = 0..=28;
        for i in nodes {
            tree.put(i);
        }
        assert!(tree.contains(4));
    }
 
    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_validate_success -- --nocapture
    #[test]
    fn test_validate_success() {
        let mut tree = Tree::new();
        let nodes = 0..=28;
        for i in nodes {
            tree.put(i);
        }
        assert!(tree.helper_is_a_valid_red_black_tree());
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_tree_success -- --nocapture
    #[test]
    #[should_panic(expected = "tree empty")]
    fn test_remove_tree_success() {
        let mut tree = Tree::new();
        let nodes = 1..=6;
        for i in nodes {
            tree.put(i);
        }
        assert_eq!(6, tree.node_count());
        assert_eq!(0, tree.node_count(), "tree empty");
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_1_0_0_success -- --nocapture
    #[test]
    fn test_remove_1_0_0_success() {
        let mut tree = Tree::new();
        let nodes = 0..=27;
        for i in nodes {
            tree.put(i);
        }
        assert!(tree.helper_is_a_valid_red_black_tree());
        tree.remove(26);
        assert!(tree.helper_is_a_valid_red_black_tree());
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_3_0_0_success -- --nocapture
    #[test]
    fn test_remove_3_0_0_success() {
        let mut tree = Tree::new();
        let nodes = 0..=3;
        for i in nodes {
            tree.put(i);
        }
        assert!(tree.helper_is_a_valid_red_black_tree());
        tree.remove(3);
        assert!(tree.helper_is_a_valid_red_black_tree());
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_node_4_0_0_to_3_0_0_success -- --nocapture
    #[test]
    fn test_remove_node_4_0_0_to_3_0_0_success() {
        let mut tree = Tree::new();
        let nodes = 0..=9;
        for i in nodes {
            tree.put(i);
        }
        assert!(tree.helper_is_a_valid_red_black_tree());
        tree.remove(7);
        assert!(tree.helper_is_a_valid_red_black_tree());
    }
 
    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_node_4_0_0_to_1_0_0_success -- --nocapture
    #[test]
    fn test_remove_node_4_0_0_to_1_0_0_success() {
        let mut tree = Tree::new();
        let nodes = vec![315, 897, 267, 995, 843, 520];
        for i in nodes {
            tree.put(i);
        }
        assert!(tree.helper_is_a_valid_red_black_tree());
        tree.remove(897);
        assert!(tree.helper_is_a_valid_red_black_tree());
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_black_2_1_1_1_success  -- --nocapture
    #[test]
    fn test_remove_black_2_1_1_1_success() {
        let mut tree = Tree::new();
        let nodes = vec![314, 147, 119, 331, 755, 449, 118];
        for i in nodes {
            tree.put(i);
        }
        assert!(tree.helper_is_a_valid_red_black_tree());
        
        tree.remove(314);//2.1.1.1
 
        unsafe {
            Tree::helper_checking_connections(tree.get_root());
        }
        assert!(tree.helper_is_a_valid_red_black_tree());
        tree.remove(147);// 2.1.2.1
        assert!(tree.helper_is_a_valid_red_black_tree());
        unsafe {
            Tree::helper_checking_connections(tree.get_root());
        }
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_black_2_1_1_2_success  -- --nocapture
    #[test]
    fn test_remove_black_2_1_1_2_success() {
        let mut tree = Tree::new();
        let nodes = vec![231, 511, 914, 699, 532, 531];
        for i in nodes {
            tree.put(i);
        }
        assert!(tree.helper_is_a_valid_red_black_tree());
        tree.remove(231);
        assert!(tree.helper_is_a_valid_red_black_tree());
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_black_2_1_2_1_success  -- --nocapture
    #[test]
    fn test_remove_black_2_1_2_1_success() {
        let mut tree = Tree::new();
        let nodes = vec![438, 440, 260, 530, 34, 355];
        for i in nodes {
            tree.put(i);
        }
        assert!(tree.helper_is_a_valid_red_black_tree());
        tree.remove(355);
        assert!(tree.helper_is_a_valid_red_black_tree());
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_black_2_1_2_2_success  -- --nocapture
    #[test]
    fn test_remove_black_2_1_2_2_success() {
        let mut tree = Tree::new();
        let nodes = vec![231, 511, 914, 699, 532];
        for i in nodes {
            tree.put(i);
        }
        assert!(tree.helper_is_a_valid_red_black_tree());
        println!("{}", tree.display());
        tree.remove(231);
        assert!(tree.helper_is_a_valid_red_black_tree());
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_black_2_2_1_node_a_root_success -- --nocapture
    #[test]
    fn test_remove_black_2_2_1_node_a_root_success() {
        let mut tree = Tree::new();
        let nodes = vec![315, 897, 267, 995, 843, 520];
        for i in nodes {
            tree.put(i);
        }
        assert!(tree.helper_is_a_valid_red_black_tree());
        tree.remove(995);
        assert!(tree.helper_is_a_valid_red_black_tree());
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_black_2_2_1_success -- --nocapture
    #[test]
    fn test_remove_black_2_2_1_success() {
        let mut tree = Tree::new();
        let nodes = vec![486, 226, 612, 121, 479, 69, 559, 990, 290, 324, 280];
        for i in nodes {
            tree.put(i);
        }
        assert!(tree.helper_is_a_valid_red_black_tree());
        tree.remove(479);
        assert!(!tree.contains(479));
        assert!(tree.helper_is_a_valid_red_black_tree());
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_black_2_2_2_success -- --nocapture
    #[test]
    fn test_remove_black_2_2_2_success() {
        let mut tree = Tree::new();
        let nodes: Vec<i32> = vec![119, 331, 755, 449, 118, 850, 495, 382, 328];
        for i in nodes {
            tree.put(i);
        }
        assert!(tree.helper_is_a_valid_red_black_tree());
        tree.remove(382);
        assert!(tree.helper_is_a_valid_red_black_tree());
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_black_2_2_2_root_success -- --nocapture
    #[test]
    fn test_remove_black_2_2_2_root_success() {
        let mut tree = Tree::new();
        let nodes: Vec<i32> = vec![106, 734, 951, 753, 205, 730];
        for i in nodes {
            tree.put(i);
        }
        tree.remove(753);
        assert!(tree.helper_is_a_valid_red_black_tree());
        tree.remove(951);
        assert!(tree.helper_is_a_valid_red_black_tree());
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_black_2_3_1_1_success -- --nocapture
    #[test]
    fn test_remove_black_2_3_1_1_success() {
        let mut tree = Tree::new();
        let nodes: Vec<i32> = vec![575, 396, 139, 792, 546, 73, 7, 6];
        for i in nodes {
            tree.put(i);
        }
        assert!(tree.helper_is_a_valid_red_black_tree());
        tree.remove(139);
        assert!(tree.helper_is_a_valid_red_black_tree());
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_black_2_3_1_1_root_success -- --nocapture
    #[test]
    fn test_remove_black_2_3_1_1_root_success() {
        let mut tree = Tree::new();
        let nodes: Vec<i32> = vec![106, 107, 108, 105];
        for i in nodes {
            tree.put(i);
        }
        assert!(tree.helper_is_a_valid_red_black_tree());
        tree.remove(108);
        assert!(tree.helper_is_a_valid_red_black_tree());
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_black_2_3_1_2_success -- --nocapture
    #[test]
    fn test_remove_black_2_3_1_2_success() {
        let mut tree = Tree::new();
        let nodes: Vec<i32> = vec![575, 396, 139, 792, 546, 73, 7, 138];
        for i in nodes {
            tree.put(i);
        }
        assert!(tree.helper_is_a_valid_red_black_tree());
        tree.remove(7);
        assert!(tree.helper_is_a_valid_red_black_tree());
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_black_2_3_1_2_root_success -- --nocapture
    #[test]
    fn test_remove_black_2_3_1_2_root_success() {
        let mut tree = Tree::new();
        let nodes: Vec<i32> = vec![106, 107, 109, 108];
        for i in nodes {
            tree.put(i);
        }
        assert!(tree.helper_is_a_valid_red_black_tree());
        tree.remove(106);
        assert!(tree.helper_is_a_valid_red_black_tree());
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_black_2_3_2_1_success -- --nocapture
    #[test]
    fn test_remove_black_2_3_2_1_success() {
        let mut tree = Tree::new();
        let nodes: Vec<i32> = vec![15, 19, 20, 14, 17, 18, 16];
        for i in nodes {
            tree.put(i);
        }
        assert!(tree.helper_is_a_valid_red_black_tree());
        tree.remove(20);
        assert!(tree.helper_is_a_valid_red_black_tree());
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_black_2_3_2_1_root_success -- --nocapture
    #[test]
    fn test_remove_black_2_3_2_1_root_success() {
        let mut tree = Tree::new();
        let nodes: Vec<i32> = vec![5, 4, 6];
        for i in nodes {
            tree.put(i);
        }
        assert!(tree.helper_is_a_valid_red_black_tree());
        tree.remove(6);
        assert!(tree.helper_is_a_valid_red_black_tree());
        println!("{}", tree.display());
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_black_2_3_2_2_up_to_down_success -- --nocapture
    #[test]
    fn test_remove_black_2_3_2_2_up_to_down_success() {
        let mut tree: Tree<i32> = Tree::new();

        let nodes = vec![
            352, 873, 462, 836, 316, 381, 595, 288, 600, 263, 310, 74, 544, 621, 402, 618, 61, 576,
            654, 579, 985, 949, 856, 796, 894, 6, 991, 880, 652, 349, 525, 9, 515, 371, 53,
        ];
        for i in nodes {
            tree.put(i);
        }

        println!("{}", tree.display());
        unsafe {
            Tree::helper_checking_connections(tree.get_root());
        }

        tree.remove(6);
        println!("{}", tree.display());
        assert!(tree.helper_is_a_valid_red_black_tree());
        unsafe {
            Tree::helper_checking_connections(tree.get_root());
        }
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_black_2_3_2_2_start_v5_success -- --nocapture
    #[test]
    fn test_remove_black_2_3_2_2_start_v5_success() {
        let mut tree: Tree<i32> = Tree::new();

        let nodes = vec![
            480, 978, 379, 784, 398, 71, 695, 23, 97, 309, 312, 449, 958, 992, 220, 95, 257, 869,
            959, 450, 258, 315, 783, 731, 914, 880, 984, 734, 570, 801, 908, 181, 466, 238, 916,
            77, 801, 867, 382, 943, 603, 65, 545, 200, 759, 158, 987, 821, 630, 537, 704, 149, 617,
            498, 261, 160, 192, 760, 417, 939, 757, 858, 376, 885, 336, 764, 443, 155, 983, 586,
            957, 375, 893, 707, 255, 811, 86, 370, 384, 177, 834, 177, 834, 313, 209, 623, 176,
            875, 748, 949, 529, 932, 369, 385, 419, 222, 719, 342, 68, 156, 314, 343, 262, 467,
            499, 604, 732, 758, 765, 812, 859, 876,
        ];
        for i in nodes {
            tree.put(i);
        }

        tree.remove(23);

        assert!(tree.helper_is_a_valid_red_black_tree());
        unsafe {
            Tree::helper_checking_connections(tree.get_root());
        }
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_black_2_3_2_2_success -- --nocapture
    #[test]
    fn test_remove_black_2_3_2_2_success() {
        let mut tree = Tree::new();
        let nodes: Vec<i32> = vec![246, 562, 950, 237, 417, 418, 416];
        for i in nodes {
            tree.put(i);
        }
        assert!(tree.helper_is_a_valid_red_black_tree());
        tree.remove(418);
        assert!(tree.helper_is_a_valid_red_black_tree());
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_black_2_3_2_2_root_success -- --nocapture
    #[test]
    fn test_remove_black_2_3_2_2_root_success() {
        let mut tree = Tree::new();
        let nodes: Vec<i32> = vec![5, 4, 6];
        for i in nodes {
            tree.put(i);
        }
        assert!(tree.helper_is_a_valid_red_black_tree());
        tree.remove(4);
        assert!(tree.helper_is_a_valid_red_black_tree());
    }
}
