pub use llrb::{Node, Tree, helper_prepare_batch_put};
mod llrb {
    use std::cmp::Ordering;
    use std::fmt::{Debug, Display};
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

        pub fn contains(&self, value: &T) -> bool {
            !find_node(self.get_root(), value).is_none()
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
 
        pub fn remove(&mut self, value: T) -> bool {
            unsafe {
                let fixed_head = self.fixed_head.unwrap();
                if let Some(node) = find_node((*fixed_head.as_ptr()).left, &value) {
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

        pub fn find(&self, value: T) -> Option<&T> {
            unsafe {
                if let Some(fixed_head) = self.fixed_head{
                    if let Some(node) = find_node((*fixed_head.as_ptr()).left, &value){
                        return Some(&(*node.as_ptr()).value);
                    }
                }
                None
            }
        }

        /// Найти следующий элемент данного элемента в дереве.  
        /// Симметричный поиск в глубину (In-order).
        #[cfg(feature = "in-order")]
        pub fn successor_dfs_in_order(&self, value: T) -> Option<&T> {
            unsafe {
                let node = find_node(self.get_root(), &value);
                if let Some(n) = node {
                    if let Some(nodesucc) = dfs_in_order::successor_of_node_in_order(n) {
                        return Some(&(*nodesucc.as_ref()).value);
                    }
                }
                None
            }
        }

        /// Найти следующий элемент данного элемента в дереве.  
        /// Обратный поиск в глубину (Post order).
        #[cfg(feature = "post-order")]
        pub fn successor_dfs_post_order(&self, value: T) -> Option<&T> {
            unsafe {
                if let Some(n) = find_node(self.get_root(), value) {
                    if let Some(nodesucc) = dfs_post_order::successor_of_node_post_order(n) {
                        return Some(&(*nodesucc.as_ref()).value);
                    }
                }
                None
            }
        }

        /// Найти следующий элемент данного элемента в дереве.
        /// Прямой поиск в глубину (Pre order).
        #[cfg(feature = "pre-order")]
        pub fn successor_dfs_pre_order(&self, value: T) -> Option<&T> {
            unsafe {
                let node = find_node(self.get_root(), value);
                if let Some(n) = node {
                    if let Some(nodesucc) = dfs_pre_order::successor_of_node_pre_order(n) {
                        return Some(&(*nodesucc.as_ref()).value);
                    }
                }
                None
            }
        }

        fn get_root(&self) -> Link<T> {
            if let Some(fixed_head) = self.fixed_head {
                unsafe { (*fixed_head.as_ptr()).left }
            } else {
                None
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
                                        return OperationRemoveBlackLeaf::LeftRedABlackBRedCleaf;
                                        // 2.1.1.1
                                    }
                                } else {
                                    return OperationRemoveBlackLeaf::LeftRedABlackBleaf;
                                    // 2.1.2.1
                                }
                            }
                        }
                    }
                    if let Some(node_b) = (*node_a.as_ref()).right {
                        if !std::ptr::eq(node_b.as_ptr(), node_x.as_ptr()) {
                            if !(*node_b.as_ref()).is_red {
                                if let Some(node_c) = (*node_b.as_ref()).left {
                                    if (*node_c.as_ref()).is_red {
                                        return OperationRemoveBlackLeaf::RightRedABlackBRedCleaf;
                                        // 2.1.1.2
                                    }
                                } else {
                                    return OperationRemoveBlackLeaf::RightRedABlackBleaf;
                                    // 2.1.2.2
                                }
                            }
                        }
                    }
                } else {
                    if let Some(node_b) = (*node_a.as_ref()).left {
                        if !std::ptr::eq(node_b.as_ptr(), node_x.as_ptr()) {
                            if (*node_b.as_ref()).is_red && (*node_b.as_ref()).right.is_some() {
                                let node_c = (*node_b.as_ref()).right.unwrap();
                                if !(*node_c.as_ref()).is_red {
                                    if (*node_c.as_ref()).left.is_some() {
                                        if let Some(node_d) = (*node_c.as_ref()).left {
                                            if (*node_d.as_ref()).is_red {
                                                return OperationRemoveBlackLeaf::BlackARedBWithBlackChildrenRightHaveRedLeaf;
                                                // 2.2.1
                                            }
                                        }
                                    }
                                    return OperationRemoveBlackLeaf::BlackARedBWithBlackChildrenLeaf;
                                    // 2.2.2
                                }
                            }
                            if !(*node_b.as_ref()).is_red && (*node_b.as_ref()).left.is_some() {
                                if let Some(node_d) = (*node_b.as_ref()).left {
                                    if (*node_d.as_ref()).is_red {
                                        return OperationRemoveBlackLeaf::BlackALeftBlackBRedDleaf;
                                        // 2.3.1.1
                                    }
                                }
                            }
                            if !(*node_b.as_ref()).is_red {
                                return OperationRemoveBlackLeaf::BlackALeftBlackBleaf;
                                // 2.3.2.1
                            }
                        }
                    }
                    if let Some(node_b) = (*node_a.as_ref()).right {
                        if !std::ptr::eq(node_b.as_ptr(), node_x.as_ptr()) {
                            if !(*node_b.as_ref()).is_red && (*node_b.as_ref()).left.is_some() {
                                if let Some(node_d) = (*node_b.as_ref()).left {
                                    if (*node_d.as_ref()).is_red {
                                        return OperationRemoveBlackLeaf::BlackARightBlackBRedDleaf;
                                        // 2.3.1.2
                                    }
                                }
                            }
                            if !(*node_b.as_ref()).is_red {
                                return OperationRemoveBlackLeaf::BlackARightBlackBleaf;
                                // 2.3.2.2
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
            }
        }

        unsafe fn remove_node_attempt_avoid_black_leaf(
            &mut self,
            node_x: NonNull<Node<T>>,
        ) -> bool {
            match self.operation_remove_black_leaf(&node_x) {
                OperationRemoveBlackLeaf::LeftRedABlackBRedCleaf => {
                    self.remove_black_leaf_2_1_1_1_balancing(&node_x);
                    return self.remove_leaf(node_x);
                }
                OperationRemoveBlackLeaf::RightRedABlackBRedCleaf => {
                    self.remove_black_leaf_2_1_1_2_balancing(&node_x);
                    return self.remove_leaf(node_x);
                }
                OperationRemoveBlackLeaf::LeftRedABlackBleaf => {
                    self.remove_black_leaf_2_1_2_1_balancing(&node_x);
                    return self.remove_leaf(node_x);
                }
                OperationRemoveBlackLeaf::RightRedABlackBleaf => {
                    self.remove_black_leaf_2_1_2_2_balancing(&node_x);
                    return self.remove_leaf(node_x);
                }
                OperationRemoveBlackLeaf::BlackARedBWithBlackChildrenRightHaveRedLeaf => {
                    self.remove_black_leaf_2_2_1_balancing(&node_x);
                    return self.remove_leaf(node_x);
                }
                OperationRemoveBlackLeaf::BlackARedBWithBlackChildrenLeaf => {
                    self.remove_black_leaf_2_2_2_balancing(&node_x);
                    return self.remove_leaf(node_x);
                }
                OperationRemoveBlackLeaf::BlackALeftBlackBRedDleaf => {
                    self.remove_black_leaf_2_3_1_1_balancing(&node_x);
                    return self.remove_leaf(node_x);
                }
                OperationRemoveBlackLeaf::BlackARightBlackBRedDleaf => {
                    self.remove_black_leaf_2_3_1_2_balancing(&node_x);
                    return self.remove_leaf(node_x);
                }
                _ => {
                    return false;
                }
            }
        }

        unsafe fn find_max(&self, node: NonNull<Node<T>>) -> NonNull<Node<T>> {
            if (*node.as_ref()).right.is_some() {
                self.find_max((*node.as_ref()).right.unwrap())
            } else {
                node
            }
        }

        unsafe fn find_min(&self, node: NonNull<Node<T>>) -> NonNull<Node<T>> {
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

        /*
           Rotate left and Rotate right and Flip color
        
               |              ||
               P              C
             //  \          /   \
            A     F        A     P
           / \\      =>   / \   / \
          B   C          B   E D   F
             / \
            E   D
        */
        unsafe fn rotate_left_right_flip_color(&mut self, node_a: NonNull<Node<T>>) -> Link<T> {
            let node_p = (*node_a.as_ptr()).parent.unwrap();
            if let Some(parent) = (*node_p.as_ptr()).parent {
                let mut node_p_from_left = false;
                if let Some(p_node_p) = (*parent.as_ptr()).left {
                    if std::ptr::eq(p_node_p.as_ptr(), node_p.as_ptr()) {
                        node_p_from_left = true;
                    }
                }
                let node_p = if node_p_from_left {
                    (*parent.as_ptr()).left.unwrap()
                } else {
                    (*parent.as_ptr()).right.unwrap()
                };  
                //let node_a = (*node_p.as_ptr()).left.unwrap();
                (*node_a.as_ptr()).is_red = false;
                let node_c = (*node_a.as_ptr()).right.unwrap();
                (*node_c.as_ptr()).is_red = false;
                let node_e = (*node_c.as_ptr()).left;
                let node_d = (*node_c.as_ptr()).right;

                (*node_c.as_ptr()).is_red = true;

                (*node_c.as_ptr()).parent = Some(parent);
                (*node_c.as_ptr()).left = Some(node_a);
                (*node_a.as_ptr()).parent = Some(node_c);
                (*node_c.as_ptr()).right = Some(node_p);
                (*node_p.as_ptr()).parent = Some(node_c);
                (*node_p.as_ptr()).left = node_d;
                if let Some(n_d) = node_d{
                    (*n_d.as_ptr()).parent = Some(node_p);
                }
                (*node_a.as_ptr()).right = node_e;
                if let Some(n_e) = node_e{
                    (*n_e.as_ptr()).parent = Some(node_a);
                }
                if node_p_from_left {
                    (*parent.as_ptr()).left = Some(node_c);
                } else {
                    (*parent.as_ptr()).right = Some(node_c);
                } 
                return Some(parent);
            }else {
                let fixed_head = self.fixed_head.unwrap();
                let node_p = (*fixed_head.as_ptr()).left.unwrap();

                let node_a = (*node_p.as_ptr()).left.unwrap();
                (*node_a.as_ptr()).is_red = false;
                let node_c = (*node_a.as_ptr()).right.unwrap();
                (*node_c.as_ptr()).is_red = false;
                let node_e = (*node_c.as_ptr()).left;
                let node_d = (*node_c.as_ptr()).right;

                (*node_c.as_ptr()).is_red = false;

                (*node_c.as_ptr()).parent = None;
                (*node_c.as_ptr()).left = Some(node_a);
                (*node_a.as_ptr()).parent = Some(node_c);
                (*node_c.as_ptr()).right = Some(node_p);
                (*node_p.as_ptr()).parent = Some(node_c);
                (*node_p.as_ptr()).left = node_d;
                if let Some(n_d) = node_d{
                    (*n_d.as_ptr()).parent = Some(node_p);
                }
                (*node_a.as_ptr()).right = node_e;
                if let Some(n_e) = node_e{
                    (*n_e.as_ptr()).parent = Some(node_a);
                }
                (*fixed_head.as_ptr()).left = Some(node_c);
                  
                return (*fixed_head.as_ptr()).left;
            }
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
            if let Some(parent) = (*node_a.as_ptr()).parent {
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
                (*node_c.as_ptr()).is_red = (*node_a.as_ptr()).is_red; 
                (*node_a.as_ptr()).is_red = true;  
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
            } else {
                let fixed_head = self.fixed_head.unwrap();
                let node_a = (*fixed_head.as_ptr()).left.unwrap();
                let node_c = (*node_a.as_ptr()).right.unwrap();
                (*node_c.as_ptr()).is_red = (*node_a.as_ptr()).is_red;  
                (*node_a.as_ptr()).is_red = true;  
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
            if let Some(parent) = (*node_a.as_ptr()).parent {
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
                (*node_b.as_ptr()).is_red = (*node_a.as_ptr()).is_red;  
                (*node_a.as_ptr()).is_red = true;  
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
            } else {
                let fixed_head = self.fixed_head.unwrap();
                let node_a = (*fixed_head.as_ptr()).left.unwrap();
                let node_b = (*node_a.as_ptr()).left.unwrap();
                (*node_b.as_ptr()).is_red = (*node_a.as_ptr()).is_red;  
                (*node_a.as_ptr()).is_red = true;  
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

        /*
             |          ||
             A    =>    A
           // \\       / \
          B    C      B   C

        */
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

        unsafe fn put_balancing(&mut self, next: NonNull<Node<T>>) {
            let mut next = next;
            loop { 
                match self.check_put_balancing(next) { 
                    OperationPut::Left => { 
                        if (*next.as_ptr()).is_red {
                            if let Some(n) = self.rotate_left_right_flip_color(next) {
                                next = n;
                            }  
                        }else{
                            if let Some(n) = self.rotate_left(next) {
                                next = n;
                            }                            
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
                        break;
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

        pub fn helper_checking_connections(&self) {
            unsafe {
                let fixed_head = self.fixed_head.unwrap();
                let node = (*fixed_head.as_ptr()).left;
                return checking_connections(node);
            }

            unsafe fn checking_connections<
                T: Ord + PartialEq + PartialOrd + Default + Display + Clone + Debug,
            >(
                node: Link<T>,
            ) {
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
                            "left connection with {:?} parent is broken",
                            (*n.as_ptr()).value
                        );
                        let p = (*right.as_ptr()).parent.unwrap();
                        assert_eq!(
                            (*n.as_ptr()).value,
                            (*p.as_ptr()).value,
                            "right connection with {:?} parent is broken",
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
                                "connection with {:?} parent is broken",
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
                                "connection with {:?} parent is broken",
                                (*n.as_ptr()).value
                            );
                        }
                    } else {
                        println!("[{}]", (*n.as_ptr()).value);
                    }
                    checking_connections((*n.as_ptr()).left);
                    checking_connections((*n.as_ptr()).right);
                }
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

        /// TODO: open http://www.webgraphviz.com/?tab=map
        /// or https://dreampuf.github.io/GraphvizOnline/
        pub fn display(&self) -> String {
            unsafe {
                let fixed_head = self.fixed_head.unwrap();
                if let Some(root) = (*fixed_head.as_ptr()).left {
                    return format!("\n\ndigraph Tree {{\n\tratio = fill;\n\tnode [style=filled fontcolor=\"white\"];\n{}}}",helper_display_tree(root));
                }
                "\nTree is empty".into()
            }
        }

        #[cfg(feature = "in-order")]
        pub fn iter_dfs_in_order(&self) -> IterInOrder<T> {
            unsafe {
                IterInOrder::new(
                    dfs_in_order::leftmost_child_in_order(self.get_root()),
                    self.count,
                )
            }
        }

        #[cfg(feature = "pre-order")]
        pub fn iter_dfs_pre_order(&self) -> IterPreOrder<T> {
            IterPreOrder::new(self.get_root(), self.count)
        }

        #[cfg(feature = "post-order")]
        pub fn iter_dfs_post_order(&self) -> IterPostOrder<T> {
            IterPostOrder::new(self.get_root(), self.count)
        }

        #[cfg(feature = "bfs")]
        pub fn breadth_first_search_with_deque(&self) -> Vec<&T> {
            let mut ret: Vec<&T> = vec![];
            bfs::breadth_first_search_with_deque(self.get_root().unwrap(), &mut ret);
            ret
        }
        #[cfg(feature = "bfs")]
        pub fn breadth_first_search(&self) -> Vec<&T> {
            let mut ret: Vec<&T> = vec![];
            bfs::breadth_first_search(self.get_root().unwrap(), &mut ret);
            ret
        }
    }

    #[cfg(feature = "in-order")]
    impl<'a, T: Ord + PartialEq + PartialOrd + Default + Display + Clone + Debug>
        std::iter::IntoIterator for &'a Tree<T>
    {
        type IntoIter = IterInOrder<'a, T>;
        type Item = &'a T;

        fn into_iter(self) -> Self::IntoIter {
            unsafe {
                IterInOrder::new(
                    dfs_in_order::leftmost_child_in_order(self.get_root()),
                    self.count,
                )
            }
        }
    }

    #[cfg(feature = "pre-order")]
    #[cfg(not(feature = "post-order"))]
    #[cfg(not(feature = "in-order"))]
    impl<'a, T: Ord + PartialEq + PartialOrd + Default + Display + Clone + Debug>
        std::iter::IntoIterator for &'a Tree<T>
    {
        type IntoIter = IterPreOrder<'a, T>;
        type Item = &'a T;

        fn into_iter(self) -> Self::IntoIter {
            IterPreOrder::new(self.get_root(), self.count)
        }
    }

    #[cfg(feature = "post-order")]
    #[cfg(not(feature = "in-order"))]
    #[cfg(not(feature = "pre-order"))]
    impl<'a, T: Ord + PartialEq + PartialOrd + Default + Display + Clone + Debug>
        std::iter::IntoIterator for &'a Tree<T>
    {
        type IntoIter = IterPostOrder<'a, T>;
        type Item = &'a T;

        fn into_iter(self) -> Self::IntoIter {
            IterPostOrder::new(self.get_root(), self.count)
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
                (*fixed_head.as_ptr()).left = Node::new_black(value); //TODO: without reference back to parent
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
            //println!("Drop Node={}", self.value);
        }
    }

    pub fn find_node<T: Ord + PartialEq + PartialOrd + Display>(
        fromnode: Link<T>,
        value: &T,
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

    /// Data preparation to reduce balancing operations
    pub fn helper_prepare_batch_put<T: Ord + Copy>(src_l: &mut Vec<T>) -> Vec<T>{
        let mut ret: Vec<T> = Vec::with_capacity(src_l.len());
        src_l.sort();   
        let middle = src_l[src_l.len()/2]; 
          
        if let Ok(index) = src_l.binary_search(&middle){
            let src_r = src_l.split_off(index);
                 
            let mut index_max = 0;
            let mut index_min = src_l.len()-1;
            loop { 
                if index_max < src_r.len(){
                    ret.push(src_r[index_max]);  
                    index_max+=1;
                }
                if index_min+1 != 0{
                    ret.push(src_l[index_min]);  
                    if index_min > 0{index_min-=1;}
                }
                
                if index_max >= src_r.len() && index_min <= 0{
                    break;
                }
            }
        }
        ret
    }

    fn helper_display_tree<T: Display>(node: NonNull<Node<T>>) -> String {
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
                s.push_str(&helper_display_tree(left));
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
                s.push_str(&helper_display_tree(right));
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

    /*

             4
           /   \
          2     9
         / \   //\
        1   3  7   10
              / \
             6   8

        Depth-First Search Прямой/Pre-order:      4, 2, 1, 3, 9, 7, 6, 8, 10
        Depth-First Search Симметричный/In-order: 1, 2, 3, 4, 6, 7, 8, 9, 10
        Depth-First Search Обратный/Post-order:   1, 3, 2, 6, 8, 7, 10, 9, 4
        Breadth-First Search:                     4, 2, 9, 1, 3, 7, 10, 6, 8

    */
    #[cfg(feature = "in-order")]
    use dfs_in_order::IterInOrder;
    #[cfg(feature = "in-order")]
    mod dfs_in_order {
        use super::{Link, Node, NonNull};
        use std::fmt::Display;
        use std::marker::PhantomData;

        // Найдите преемника узла в дереве.
        pub unsafe fn successor_of_node_in_order<T: Display>(node: NonNull<Node<T>>) -> Link<T> {
            if (*node.as_ref()).right.is_some() {
                // Случай 1: узел имеет правого дочернего элемента;
                // тогда преемником является самый левый дочерний элемент этого правого дочернего элемента
                // (или самого правого дочернего элемента, если у него нет левых потомков).
                leftmost_child_in_order((*node.as_ref()).right)
            } else {
                // Случай 2: нет правого дочернего элемента;
                // затем пройдите по родительским ссылкам, чтобы найти узел, левым дочерним элементом которого мы являемся.
                // Не удалось найти такого родителя до достижения корня означает, что преемника нет.
                parent_with_left(node)
            }
        }

        // Находим самого левого дочернего элемента `node` или самого `node`, если у него нет
        // левого дочернего элемента. `node` не может быть нулевым.
        pub unsafe fn leftmost_child_in_order<T: Display>(node: Link<T>) -> Link<T> {
            if let Some(node) = node {
                if (*node.as_ref()).left.is_none() {
                    Some(node)
                } else {
                    leftmost_child_in_order((*node.as_ref()).left)
                }
            } else {
                node
            }
        }

        // Находим родителя в цепочке предков `node`, до которого можно добраться через его левую часть
        // ребенок.
        unsafe fn parent_with_left<T: Display>(node: NonNull<Node<T>>) -> Link<T> {
            let parent = (*node.as_ref()).parent;
            if let Some(parent) = parent {
                if let Some(left) = (*parent.as_ref()).left {
                    if std::ptr::eq(left.as_ptr(), node.as_ptr()) {
                        return Some(parent);
                    }
                }
                return parent_with_left(parent);
            }
            // У этого узла нет родителя, поэтому мы достигли корня
            None
        }

        #[cfg(feature = "in-order")]
        pub struct IterInOrder<'a, T: PartialEq + PartialOrd + Display + Clone> {
            current_node: Link<T>,
            count: usize,
            elem: Option<&'a T>,
            _boo: PhantomData<&'a T>,
        }

        impl<'a, T: PartialEq + PartialOrd + Display + Clone> IterInOrder<'a, T> {
            pub fn new(node: Link<T>, count: usize) -> Self {
                Self {
                    current_node: node,
                    count,
                    elem: None,
                    _boo: PhantomData,
                }
            }
        }

        impl<'a, T: PartialEq + PartialOrd + Display + Clone> Iterator for IterInOrder<'a, T> {
            type Item = &'a T;

            fn next(&mut self) -> Option<Self::Item> {
                if self.count > 0 {
                    self.count -= 1;
                    if let Some(node) = self.current_node {
                        unsafe {
                            self.elem = Some(&(*node.as_ref()).value);
                            self.current_node = successor_of_node_in_order(node);
                        }
                    } else {
                        self.elem = None;
                    }
                } else {
                    self.elem = None;
                }
                self.elem
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                (self.count, Some(self.count))
            }
        }
    }

    #[cfg(feature = "pre-order")]
    use dfs_pre_order::IterPreOrder;
    #[cfg(feature = "pre-order")]
    mod dfs_pre_order {
        use super::{Link, Node, NonNull};
        use std::fmt::Display;
        use std::marker::PhantomData;

        // Найдите преемника узла в дереве.
        pub unsafe fn successor_of_node_pre_order<T: Display>(node: NonNull<Node<T>>) -> Link<T> {
            if let Some(node) = (*node.as_ref()).left {
                Some(node)
            } else if let Some(node) = (*node.as_ref()).right {
                Some(node)
            } else {
                right_with_parent(node)
            }
        }

        unsafe fn right_with_parent<T: Display>(node: NonNull<Node<T>>) -> Link<T> {
            if let Some(parent) = (*node.as_ref()).parent {
                next_right(Some(parent), node)
            } else {
                None
            }
        }

        unsafe fn next_right<T: Display>(node: Link<T>, child: NonNull<Node<T>>) -> Link<T> {
            if let Some(n) = node {
                if let Some(right) = (*n.as_ref()).right {
                    if std::ptr::eq(right.as_ptr(), child.as_ptr()) {
                        next_right((*n.as_ref()).parent, n)
                    } else {
                        Some(right)
                    }
                } else {
                    next_right((*n.as_ref()).parent, n)
                }
            } else {
                None
            }
        }

        pub struct IterPreOrder<'a, T: PartialEq + PartialOrd + Display> {
            current_node: Link<T>,
            count: usize,
            elem: Option<&'a T>,
            _boo: PhantomData<&'a T>,
        }

        impl<'a, T: PartialEq + PartialOrd + Display> IterPreOrder<'a, T> {
            pub fn new(root: Link<T>, count: usize) -> Self {
                Self {
                    current_node: root,
                    count,
                    elem: None,
                    _boo: PhantomData,
                }
            }
        }

        impl<'a, T: PartialEq + PartialOrd + Display> Iterator for IterPreOrder<'a, T> {
            type Item = &'a T;

            fn next(&mut self) -> Option<Self::Item> {
                if self.count > 0 {
                    self.count -= 1;
                    if let Some(node) = self.current_node {
                        unsafe {
                            self.elem = Some(&(*node.as_ref()).value);
                            self.current_node = successor_of_node_pre_order(node);
                        }
                    } else {
                        self.elem = None;
                    }
                } else {
                    self.elem = None;
                }
                self.elem
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                (self.count, Some(self.count))
            }
        }
    }

    #[cfg(feature = "post-order")]
    use dfs_post_order::IterPostOrder;
    #[cfg(feature = "post-order")]
    mod dfs_post_order {
        use super::{Link, Node, NonNull};
        use std::fmt::Display;
        use std::marker::PhantomData;

        // Найдите преемника узла в дереве.
        pub unsafe fn successor_of_node_post_order<T: Display>(
            current_node: NonNull<Node<T>>,
        ) -> Link<T> {
            if let Some(parent) = (*current_node.as_ref()).parent {
                if let Some(right) = (*parent.as_ref()).right {
                    if std::ptr::eq(current_node.as_ptr(), right.as_ptr()) {
                        Some(parent)
                    } else {
                        Some(leaf_post_order(right))
                    }
                } else {
                    Some(parent)
                }
            } else {
                Some(leaf_post_order(current_node))
            }
        }

        // Найдите лист.
        unsafe fn leaf_post_order<T: Display>(node: NonNull<Node<T>>) -> NonNull<Node<T>> {
            if let Some(left) = (*node.as_ref()).left {
                leaf_post_order(left)
            } else if let Some(right) = (*node.as_ref()).right {
                leaf_post_order(right)
            } else {
                node
            }
        }

        pub struct IterPostOrder<'a, T: PartialEq + PartialOrd + Display + Clone> {
            current_node: Link<T>,
            count: usize,
            elem: Option<&'a T>,
            _boo: PhantomData<&'a T>,
        }

        impl<'a, T: PartialEq + PartialOrd + Display + Clone> IterPostOrder<'a, T> {
            pub fn new(root: Link<T>, count: usize) -> Self {
                Self {
                    current_node: root,
                    count,
                    elem: None,
                    _boo: PhantomData,
                }
            }
        }

        impl<'a, T: PartialEq + PartialOrd + Display + Clone> Iterator for IterPostOrder<'a, T> {
            type Item = &'a T;

            fn next(&mut self) -> Option<Self::Item> {
                if self.count > 0 {
                    self.count -= 1;
                    if let Some(node) = self.current_node {
                        unsafe {
                            self.current_node = successor_of_node_post_order(node);
                            if let Some(node) = self.current_node {
                                self.elem = Some(&(*node.as_ref()).value);
                                if (*node.as_ref()).parent.is_none() {
                                    self.current_node = None;
                                }
                            }
                        }
                    } else {
                        self.elem = None;
                    }
                } else {
                    self.elem = None;
                }
                self.elem
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                (self.count, Some(self.count))
            }
        }
    }

    #[cfg(feature = "bfs")]
    mod bfs {
        use super::{Link, Node, NonNull};
        use std::collections::VecDeque;
        use std::fmt::Display;

        pub fn breadth_first_search_with_deque<T: Display>(
            root: NonNull<Node<T>>,
            ret: &mut Vec<&T>,
        ) {
            let mut deque: VecDeque<NonNull<Node<T>>> = VecDeque::new();
            unsafe {
                deque.push_back(root);
                let mut node: NonNull<Node<T>>;
                while !deque.is_empty() {
                    node = deque.pop_front().unwrap();
                    ret.push(&(*node.as_ref()).value);
                    if let Some(left) = (*node.as_ref()).left {
                        deque.push_back(left);
                    }
                    if let Some(right) = (*node.as_ref()).right {
                        deque.push_back(right);
                    }
                }
            }
        }

        /// Возвращает все данные дерева
        /// Поиск в ширину (BFS).
        #[allow(dead_code)]
        pub fn breadth_first_search<T: Display>(root: NonNull<Node<T>>, ret: &mut Vec<&T>) {
            let mut queue: Vec<(&T, usize)> = vec![];
            unsafe {
                queue.push((&(*root.as_ref()).value, 1));
                breadth_first_search_recursive((*root.as_ref()).left, &mut queue, 1);
                breadth_first_search_recursive((*root.as_ref()).right, &mut queue, 1);
            }
            let mut level = 1;

            #[allow(unused_assignments)]
            let mut come_in = false;
            loop {
                come_in = false;
                for &(el, l) in queue.iter() {
                    if l == level {
                        ret.push(el);
                        come_in = true;
                    }
                }
                if !come_in {
                    break;
                }
                level += 1;
            }
        }

        #[allow(dead_code)]
        fn breadth_first_search_recursive<T: Display>(
            node: Link<T>,
            queue: &mut Vec<(&T, usize)>,
            level: usize,
        ) {
            unsafe {
                if let Some(node) = node {
                    queue.push((&(*node.as_ref()).value, level + 1));
                    breadth_first_search_recursive((*node.as_ref()).left, queue, level + 1);
                    breadth_first_search_recursive((*node.as_ref()).right, queue, level + 1);
                }
            }
        }
    }

    impl<T: Ord + PartialEq + PartialOrd + Default + Display + Clone + Debug> Default for Tree<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(feature = "in-order")]
    impl<T: Ord + PartialEq + PartialOrd + Default + Display + Clone + Debug> Clone for Tree<T> {
        fn clone(&self) -> Self {
            let mut new_list = Self::new();
            for value in self {
                new_list.put(value.clone());
            }
            new_list
        }
    }

    #[cfg(feature = "in-order")]
    impl<T: Ord + PartialEq + PartialOrd + Default + Display + Clone + Debug> Extend<T> for Tree<T> {
        fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
            for value in iter {
                self.put(value);
            }
        }
    }

    #[cfg(feature = "in-order")]
    impl<T: Ord + PartialEq + PartialOrd + Default + Display + Clone + Debug> FromIterator<T>
        for Tree<T>
    {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            let mut list = Self::new();
            list.extend(iter);
            list
        }
    }

    #[cfg(feature = "in-order")]
    impl<T: Ord + PartialEq + PartialOrd + Default + Display + Clone + Debug> Debug for Tree<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list().entries(self).finish()
        }
    }

    #[cfg(feature = "in-order")]
    impl<T: Ord + PartialEq + PartialOrd + Default + Display + Clone + Debug> PartialEq for Tree<T> {
        fn eq(&self, other: &Self) -> bool {
            self.node_count() == other.node_count() && self.iter_dfs_in_order().eq(other)
        }
    }

    #[cfg(feature = "in-order")]
    impl<T: Ord + PartialEq + PartialOrd + Default + Display + Clone + Debug> Eq for Tree<T> {}

    #[cfg(feature = "in-order")]
    impl<T: Ord + PartialEq + PartialOrd + Default + Display + Clone + Debug> PartialOrd for Tree<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.iter_dfs_in_order().partial_cmp(other)
        }
    }

    #[cfg(feature = "in-order")]
    impl<T: Ord + PartialEq + PartialOrd + Default + Display + Clone + Debug> Ord for Tree<T> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.iter_dfs_in_order().cmp(other)
        }
    }

    #[cfg(feature = "in-order")]
    impl<T: std::hash::Hash + Ord + PartialEq + PartialOrd + Default + Display + Clone + Debug>
        std::hash::Hash for Tree<T>
    {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.node_count().hash(state);
            for item in self {
                item.hash(state);
            }
        }
    }
}

/// $ cargo test red_black_tree_nonnull -- --test-threads=1
/// $ cargo test red_black_tree_nonnull --no-default-features --features pre-order -- --nocapture
/// $ cargo test red_black_tree_nonnull --no-default-features --features post-order -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_success -- --nocapture
    #[test]
    fn test_success() {
        let mut tree: Tree<i32> = Tree::new();
        let mut nodes = vec![
            480, 978, 379, 784, 999, 695, 23, 97, 309, 312, 449, 958, 992, 220, 95, 257, 869, 959,
            450, 258, 315, 783, 731, 914, 880 , 984, 734, 570, 801, 908, 181, 466, 238, 916, 77,
            801, 867, 382, 943, 603, 65, 545, 200, 759, 158, 987, 821, 630, 537, 704, 149, 617,
            498, 261, 160, 192, 760, 417, 939 , 757, 858, 376, 885, 336, 764, 443, 155, 983, 586,
            957, 375, 893, 707, 255, 811, 86, 370, 384, 177, 834, 177, 834, 313, 209, 623, 176,
            875, 748, 949, 529, 932, 369, 385, 419, 222, 719, 342, 68, 156, 314, 343, 262 , 467,
            499, 604, 732, 758, 765, 812, 859, 876, 
        ];
        let nodes: Vec<i32> = helper_prepare_batch_put(&mut nodes);
        for i in nodes {
            tree.put(i);
        }
        
        println!("{}", tree.display());
        tree.helper_checking_connections();
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
        tree.helper_checking_connections();
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
        assert!(tree.contains(&4));
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

        tree.remove(314); //2.1.1.1

        tree.helper_checking_connections();
        assert!(tree.helper_is_a_valid_red_black_tree());
        tree.remove(147); // 2.1.2.1
        assert!(tree.helper_is_a_valid_red_black_tree());
        tree.helper_checking_connections();
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
        assert!(!tree.contains(&479));
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
        tree.helper_checking_connections();

        tree.remove(6);
        println!("{}", tree.display());
        assert!(tree.helper_is_a_valid_red_black_tree());
        tree.helper_checking_connections();
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
        tree.helper_checking_connections();
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

    /*

             4
           /   \
          2     9
         / \   //\
        1   3  7   10
              / \
             6   8

        Depth-First Search Прямой/Pre-order:      4, 2, 1, 3, 9, 7, 6, 8, 10
        Depth-First Search Симметричный/In-order: 1, 2, 3, 4, 6, 7, 8, 9, 10
        Depth-First Search Обратный/Post-order:   1, 3, 2, 6, 8, 7, 10, 9, 4
        Breadth-First Search:                     4, 2, 9, 1, 3, 7, 10, 6, 8
    */
    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_dfs_iter_in_order -- --nocapture
    #[cfg(feature = "in-order")]
    #[test]
    fn test_dfs_iter_in_order() {
        let mut tree: Tree<i32> = Tree::new();
        let nodes: Vec<i32> = vec![4, 3, 9, 1, 2, 10, 7, 8, 6];
        for i in nodes {
            tree.put(i);
        }

        let mut buf: Vec<&i32> = vec![];
        for item in tree.iter_dfs_in_order() {
            buf.push(item);
        }

        assert_eq!(buf, vec![&1, &2, &3, &4, &6, &7, &8, &9, &10]);

        assert_eq!(Some(&6), tree.successor_dfs_in_order(4), "4->6");
        assert_eq!(Some(&2), tree.successor_dfs_in_order(1), "1->2");
        assert_eq!(Some(&3), tree.successor_dfs_in_order(2), "2->3");
        assert_eq!(Some(&4), tree.successor_dfs_in_order(3), "3->4");
        assert_eq!(Some(&6), tree.successor_dfs_in_order(4), "4->6");
        assert_eq!(Some(&7), tree.successor_dfs_in_order(6), "6->7");
        assert_eq!(Some(&8), tree.successor_dfs_in_order(7), "7->8");
        assert_eq!(Some(&9), tree.successor_dfs_in_order(8), "8->9");
        assert_eq!(Some(&10), tree.successor_dfs_in_order(9), "9->10");

        let mut buf: Vec<&i32> = vec![];
        for item in tree.into_iter() {
            buf.push(item);
        }
        assert_eq!(buf, vec![&1, &2, &3, &4, &6, &7, &8, &9, &10]);
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_dfs_iter_pre_order --no-default-features --features pre-order -- --nocapture
    #[cfg(feature = "pre-order")]
    #[test]
    fn test_dfs_iter_pre_order() {
        let mut tree: Tree<i32> = Tree::new();
        let nodes: Vec<i32> = vec![4, 3, 9, 1, 2, 10, 7, 8, 6];
        for i in nodes {
            tree.put(i);
        }

        let mut buf: Vec<&i32> = vec![];
        for item in tree.iter_dfs_pre_order() {
            buf.push(item);
        }
        assert_eq!(buf, vec![&4, &2, &1, &3, &9, &7, &6, &8, &10]);

        assert_eq!(Some(&2), tree.successor_dfs_pre_order(4), "4->2");
        assert_eq!(Some(&1), tree.successor_dfs_pre_order(2), "2->1");
        assert_eq!(Some(&3), tree.successor_dfs_pre_order(1), "1->3");
        assert_eq!(Some(&9), tree.successor_dfs_pre_order(3), "3->9");
        assert_eq!(Some(&7), tree.successor_dfs_pre_order(9), "9->7");
        assert_eq!(Some(&6), tree.successor_dfs_pre_order(7), "7->6");
        assert_eq!(Some(&8), tree.successor_dfs_pre_order(6), "6->8");
        assert_eq!(Some(&10), tree.successor_dfs_pre_order(8), "8->10");

        let mut buf: Vec<&i32> = vec![];
        for item in tree.into_iter() {
            buf.push(item);
        }
        assert_eq!(buf, vec![&4, &2, &1, &3, &9, &7, &6, &8, &10]);
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_dfs_iter_post_order --no-default-features --features post-order -- --nocapture
    #[cfg(feature = "post-order")]
    #[test]
    fn test_dfs_iter_post_order() {
        let mut tree: Tree<i32> = Tree::new();
        let nodes: Vec<i32> = vec![4, 3, 9, 1, 2, 10, 7, 8, 6];
        for i in nodes {
            tree.put(i);
        }

        let mut buf: Vec<&i32> = vec![];
        for item in tree.iter_dfs_post_order() {
            buf.push(item);
        }
        assert_eq!(buf, vec![&1, &3, &2, &6, &8, &7, &10, &9, &4]);

        assert_eq!(Some(&3), tree.successor_dfs_post_order(1), "1->3");
        assert_eq!(Some(&2), tree.successor_dfs_post_order(3), "3->2");
        assert_eq!(Some(&6), tree.successor_dfs_post_order(2), "2->6");
        assert_eq!(Some(&8), tree.successor_dfs_post_order(6), "6->8");
        assert_eq!(Some(&7), tree.successor_dfs_post_order(8), "8->7");
        assert_eq!(Some(&10), tree.successor_dfs_post_order(7), "7->10");
        assert_eq!(Some(&9), tree.successor_dfs_post_order(10), "10->9");
        assert_eq!(Some(&4), tree.successor_dfs_post_order(9), "9->4");

        let mut buf: Vec<&i32> = vec![];
        for item in tree.into_iter() {
            buf.push(item);
        }
        assert_eq!(buf, vec![&1, &3, &2, &6, &8, &7, &10, &9, &4]);
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_bfs -- --nocapture
    #[cfg(feature = "bfs")]
    #[test]
    fn test_bfs() {
        let mut tree: Tree<i32> = Tree::new();
        let nodes: Vec<i32> = vec![4, 3, 9, 1, 2, 10, 7, 8, 6];
        for i in nodes {
            tree.put(i);
        }

        let buf: Vec<&i32> = tree.breadth_first_search_with_deque();
        assert_eq!(buf, vec![&4, &2, &9, &1, &3, &7, &10, &6, &8]);

        let buf: Vec<&i32> = tree.breadth_first_search();
        assert_eq!(buf, vec![&4, &2, &9, &1, &3, &7, &10, &6, &8]);
    }
}
