#![allow(unused_imports)]

// Tree на основе NonNull
pub use ds_binary_tree::Tree;
mod ds_binary_tree {
    use std::cmp::Ordering;
    use std::fmt::{self, Debug, Display};
    use std::hash::{Hash, Hasher};
    use std::iter::FromIterator;
    use std::marker::PhantomData;
    use std::ptr::NonNull;

    pub struct Tree<T: Ord + PartialEq + PartialOrd + Display + Clone> {
        root: Link<T>,
        count: usize,
        _boo: PhantomData<T>,
    }

    type Link<T> = Option<NonNull<Node<T>>>;

    #[derive(Debug)]
    pub struct Node<T: Display> {
        left: Link<T>,
        right: Link<T>,
        parent: Link<T>,
        elem: T,
    }

    impl<T: Ord + PartialEq + PartialOrd + Display + Clone> Tree<T> {
        pub fn new() -> Self {
            Self {
                root: None,
                count: 0,
                _boo: PhantomData,
            }
        }

        /// Возвращает количество узлов дерева.
        pub fn node_count(&self) -> usize {
            assert!(self.count != 0 || self.root.is_none());
            self.count
        }

        /// Вставляем новый элемент в дерево;
        /// возвращает true, если вставка произошла,
        /// и значение false, если данные данные уже присутствовали в дереве.
        pub fn insert(&mut self, elem: T) -> bool {
            if let Some(root) = self.root {
                if !insert_node(root, elem) {
                    return false;
                }
            } else {
                self.root = Node::new(elem);
            }
            self.count += 1;
            true
        }

        /// Найти элемент в дереве.
        pub fn find(&self, elem: T) -> bool {
            !find_node(self.root, elem).is_none()
        }

        /// Возвращает строковое представление дерева для отладки.
        /// TODO: open http://www.webgraphviz.com/?tab=map
        pub fn display(&self) -> String {
            if let Some(root) = self.root {
                return format!("digraph Tree {{\n{}}}", display_node(root));
            }
            "".into()
        }

        /// Возвращает все данные дерева начиная с узла `start_node`
        fn depth_first_get_values(&self, start_node: Link<T>) -> Vec<T> {
            let mut v = vec![];
            let node = {
                if start_node.is_some() {
                    start_node
                } else {
                    self.root
                }
            };

            fn get_nodes<T: PartialEq + PartialOrd + Display + Clone>(
                node: Link<T>,
                buf: &mut Vec<T>,
            ) {
                if let Some(node) = node {
                    unsafe {
                        buf.push((*node.as_ref()).elem.clone());
                        get_nodes((*node.as_ref()).left, buf);
                        get_nodes((*node.as_ref()).right, buf);
                    }
                }
            }

            if let Some(node) = node {
                unsafe {
                    v.push((*node.as_ref()).elem.clone());
                    get_nodes((*node.as_ref()).left, &mut v);
                    get_nodes((*node.as_ref()).right, &mut v);
                }
            }
            v
        }

        /// Возвращает все данные дерева
        /// Симметричный поиск в глубину (In-order).
        #[cfg(feature = "in-order")]
        pub fn depth_first_in_order_recursive(&self, start_node: Link<T>) -> Vec<&T> {
            let mut v = vec![];
            let node = {
                if start_node.is_some() {
                    start_node
                } else {
                    self.root
                }
            };

            if let Some(node) = node {
                unsafe {
                    in_order_recursive((*node.as_ref()).left, &mut v);
                    v.push(&(*node.as_ref()).elem);
                    in_order_recursive((*node.as_ref()).right, &mut v);
                }
            }
            v
        }

        //pub fn depth_first_in_order(&self) -> Vec<&T> {
        //    let mut v = vec![];
        //    if let Some(root) = self.root{
        //        let mut node: Link<T> = leftmost_child_in_order(Some(root));
        //        loop {
        //            if let Some(n) = node{
        //                unsafe {
        //                    v.push(&(*n.as_ref()).elem);
        //                }
        //                node = successor_of_node_in_order(n);
        //
        //            }else{
        //                break;
        //            }
        //        }
        //    }
        //    v
        //}

        /// Возвращает все данные дерева
        /// Обратный поиск в глубину (Post order).
        //#[cfg(feature = "post-order")]
        //pub fn depth_first_post_order_recursive(&self) -> Vec<&T> {
        //    let mut v = vec![];
        //    if let Some(root) = self.root{
        //        unsafe {
        //            post_order_recursive((*root.as_ref()).left, &mut v);
        //            post_order_recursive((*root.as_ref()).right, &mut v);
        //            v.push(&(*root.as_ref()).elem);
        //        }
        //    }
        //    v
        //}

        #[cfg(feature = "post-order")]
        pub fn depth_first_post_order_recursive(&self) -> Vec<&T> {
            let mut v = vec![];
            if let Some(root) = self.root {
                let mut node: Link<T> = Some(leaf_post_order(root));
                loop {
                    if let Some(n) = node {
                        unsafe {
                            v.push(&(*n.as_ref()).elem);
                            if (*n.as_ref()).parent.is_none() {
                                break;
                            }
                        }
                        node = successor_of_node_post_order(n);
                    } else {
                        break;
                    }
                }
            }
            v
        }

        /// Возвращает все данные дерева
        /// Прямой поиск в глубину (Pre order).
        #[cfg(feature = "pre-order")]
        pub fn depth_first_pre_order_recursive(&self) -> Vec<&T> {
            let mut v = vec![];
            if let Some(node) = self.root {
                unsafe {
                    v.push(&(*node.as_ref()).elem);
                    pre_order_recursive((*node.as_ref()).left, &mut v);
                    pre_order_recursive((*node.as_ref()).right, &mut v);
                }
            }
            v
        }

        /// Возвращает все данные дерева
        /// Поиск в ширину (BFS).
        pub fn breadth_first_search(&self) -> Vec<&T> {
            let mut ret: Vec<&T> = vec![];
            if let Some(node) = self.root {
                // breadth_first_search(node, &mut ret);
                // or
                breadth_first_search_with_deque(node, &mut ret);
            }
            ret
        }

        /// Удаляем данный элемент из дерева; возвращает true, если такой узел был
        /// найдено и удалено, в противном случае — false.
        pub fn remove(&mut self, elem: T) -> bool {
            if let Some(node) = find_node(self.root, elem) {
                self.remove_node(node);
                self.count -= 1;
                true
            } else {
                false
            }
        }

        /// Найти следующий элемент данного элемента в дереве.  
        /// Симметричный поиск в глубину (In-order).
        #[cfg(feature = "in-order")]
        pub fn successor_dfs_in_order(&self, elem: T) -> Option<&T> {
            unsafe {
                let node = find_node(self.root, elem);
                if let Some(n) = node {
                    if let Some(nodesucc) = successor_of_node_in_order(n) {
                        return Some(&(*nodesucc.as_ref()).elem);
                    }
                }
                None
            }
        }

        /// Найти следующий элемент данного элемента в дереве.  
        /// Обратный поиск в глубину (Post order).
        #[cfg(feature = "post-order")]
        pub fn successor_dfs_post_order(&self, elem: T) -> Option<&T> {
            unsafe {
                if let Some(n) = find_node(self.root, elem) {
                    if let Some(nodesucc) = successor_of_node_post_order(n) {
                        return Some(&(*nodesucc.as_ref()).elem);
                    }
                }
                None
            }
        }

        /// Найти следующий элемент данного элемента в дереве.
        /// Прямой поиск в глубину (Pre order).
        #[cfg(feature = "pre-order")]
        pub fn successor_dfs_pre_order(&self, elem: T) -> Option<&T> {
            unsafe {
                let node = find_node(self.root, elem);
                if let Some(n) = node {
                    if let Some(nodesucc) = successor_of_node_pre_order(n) {
                        return Some(&(*nodesucc.as_ref()).elem);
                    }
                }
                None
            }
        }

        fn remove_tree(&mut self, node: Link<T>) {
            unsafe {
                if let Some(node) = node {
                    self.remove_tree((*node.as_ptr()).left);
                    self.remove_tree((*node.as_ptr()).right);
                    if self.remove_leaf(node) {
                        assert!(self.count > 0);
                        self.count -= 1;
                    }
                }
            }
        }

        fn remove_node(&mut self, node: NonNull<Node<T>>) {
            unsafe {
                let left = (*node.as_ref()).left;
                let right = (*node.as_ref()).right;
                if left.is_none() && right.is_none() {
                    // У узла нет дочерних элементов, поэтому его можно безопасно удалить.
                    self.remove_leaf(node);
                } else if left.is_some() && right.is_none() {
                    self.replace_node(node, left);
                } else if left.is_none() && right.is_some() {
                    self.replace_node(node, right);
                } else if left.is_some() && right.is_some() {
                    let replace_node = self.find_replace_node(left.unwrap());
                    (*node.as_ptr()).elem = (*replace_node.as_ptr()).elem.clone();
                    if (*replace_node.as_ptr()).left.is_some() {
                        self.replace_node(replace_node, (*replace_node.as_ptr()).left);
                    } else {
                        self.remove_leaf(replace_node);
                    }
                } else {
                    unreachable!()
                }
            }
        }
        unsafe fn find_replace_node(&self, node: NonNull<Node<T>>) -> NonNull<Node<T>> {
            if let Some(right) = (*node.as_ptr()).right {
                return self.find_replace_node(right);
            } else {
                return node;
            }
        }
        fn remove_leaf(&mut self, node: NonNull<Node<T>>) -> bool {
            unsafe {
                if (*node.as_ref()).left.is_some() || (*node.as_ref()).right.is_some() {
                    panic!("node is not leaf");
                } else {
                    if let Some(mut parent) = (*node.as_ref()).parent {
                        if let Some(ref mut left) = (*parent.as_mut()).left {
                            if std::ptr::eq(left.as_ptr(), node.as_ptr()) {
                                (*parent.as_mut()).left = None;
                            }
                        }
                        if let Some(ref mut right) = (*parent.as_mut()).right {
                            if std::ptr::eq(right.as_ptr(), node.as_ptr()) {
                                (*parent.as_mut()).right = None;
                            }
                        }
                    } else {
                        self.root = None;
                    }
                    let _ = Box::from_raw(node.as_ptr());
                    true
                }
            }
        }

        fn replace_node(&mut self, node: NonNull<Node<T>>, mut replace: Link<T>) {
            unsafe {
                if let Some(mut parent) = (*node.as_ref()).parent {
                    if let Some(ref mut replace) = replace {
                        // поменять ссылку на родителя
                        (*replace.as_mut()).parent = Some(parent);

                        // поменять у родителя ссылки
                        if let Some(ref mut left) = (*parent.as_mut()).left {
                            if std::ptr::eq(left.as_ptr(), node.as_ptr()) {
                                *left = *replace;
                            }
                        }
                        if let Some(ref mut right) = (*parent.as_mut()).right {
                            if std::ptr::eq(right.as_ptr(), node.as_ptr()) {
                                *right = *replace;
                            }
                        }
                    }
                } else {
                    // Removing the root node.
                    self.root = replace;
                    if let Some(mut n) = replace {
                        (*n.as_mut()).parent = None;
                    }
                }
                // узел сейчас не используется, поэтому мы можем освободить его, который будет автоматически удален.
                let _ = Box::from_raw(node.as_ptr());
            }
        }

        #[cfg(feature = "in-order")]
        pub fn iter_dfs_in_order(&self) -> IterInOrder<T> {
            IterInOrder::new(leftmost_child_in_order(self.root), self.count)
        }

        #[cfg(feature = "pre-order")]
        pub fn iter_dfs_pre_order(&self) -> IterPreOrder<T> {
            IterPreOrder::new(self.root, self.count)
        }

        #[cfg(feature = "post-order")]
        pub fn iter_dfs_post_order(&self) -> IterPostOrder<T> {
            IterPostOrder::new(self.root, self.count)
        }
    }

    impl<T: Display> Node<T> {
        fn new(elem: T) -> Link<T> {
            unsafe {
                let new = NonNull::new_unchecked(Box::into_raw(Box::new(Self {
                    left: None,
                    right: None,
                    parent: None,
                    elem,
                })));
                Some(new)
            }
        }

        fn new_with_parent(elem: T, parent: NonNull<Node<T>>) -> Link<T> {
            unsafe {
                let new = NonNull::new_unchecked(Box::into_raw(Box::new(Self {
                    left: None,
                    right: None,
                    parent: Some(parent),
                    elem,
                })));
                Some(new)
            }
        }
    }

    impl<T: Ord + PartialEq + PartialOrd + Display + Clone> Drop for Tree<T> {
        fn drop(&mut self) {
            self.remove_tree(self.root);
        }
    }

    impl<T: Display> Drop for Node<T> {
        fn drop(&mut self) {
            // println!("Drop Node={}", self.elem);
        }
    }

    #[cfg(feature = "in-order")]
    impl<'a, T: Ord + PartialEq + PartialOrd + Display + Clone> std::iter::IntoIterator
        for &'a Tree<T>
    {
        type IntoIter = IterInOrder<'a, T>;
        type Item = &'a T;

        fn into_iter(self) -> Self::IntoIter {
            IterInOrder::new(leftmost_child_in_order(self.root), self.count)
        }
    }

    #[cfg(feature = "pre-order")]
    impl<'a, T: Ord + PartialEq + PartialOrd + Display + Clone> std::iter::IntoIterator
        for &'a Tree<T>
    {
        type IntoIter = IterPreOrder<'a, T>;
        type Item = &'a T;

        fn into_iter(self) -> Self::IntoIter {
            IterPreOrder::new(self.root, self.count)
        }
    }

    #[cfg(feature = "post-order")]
    impl<'a, T: Ord + PartialEq + PartialOrd + Display + Clone> std::iter::IntoIterator
        for &'a Tree<T>
    {
        type IntoIter = IterPostOrder<'a, T>;
        type Item = &'a T;

        fn into_iter(self) -> Self::IntoIter {
            IterPostOrder::new(self.root, self.count)
        }
    }

    /// Итерация методом прохода по дереву - поиск в глубину симметричным способом (In-order)
    #[cfg(feature = "in-order")]
    use iter_dfs_in_order::IterInOrder;
    #[cfg(feature = "in-order")]
    mod iter_dfs_in_order {
        use super::dfs_in_order::{leftmost_child_in_order, successor_of_node_in_order};
        use super::{Link, PhantomData};
        use std::fmt::Display;
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
                            self.elem = Some(&(*node.as_ref()).elem);
                        }
                        self.current_node = successor_of_node_in_order(node);
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
    use iter_dfs_pre_order::IterPreOrder;
    #[cfg(feature = "pre-order")]
    mod iter_dfs_pre_order {
        use super::dfs_pre_order::successor_of_node_pre_order;
        use super::{Link, PhantomData};
        use std::fmt::Display;

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
                            self.elem = Some(&(*node.as_ref()).elem);
                        }
                        self.current_node = successor_of_node_pre_order(node);
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
    use iter_dfs_post_order::IterPostOrder;
    #[cfg(feature = "post-order")]
    mod iter_dfs_post_order {
        use super::dfs_post_order::successor_of_node_post_order;
        use super::{Link, PhantomData};
        use std::fmt::Display;

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
                                self.elem = Some(&(*node.as_ref()).elem);
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

    // опционально ------------------------------------------------------
    impl<T: Ord + PartialEq + PartialOrd + Display + Clone> Default for Tree<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T: Ord + PartialEq + PartialOrd + Display + Clone> Clone for Tree<T> {
        fn clone(&self) -> Self {
            let mut new_list = Self::new();
            for item in self {
                new_list.insert(item.clone());
            }
            new_list
        }
    }

    impl<T: Ord + PartialEq + PartialOrd + Display + Clone> Extend<T> for Tree<T> {
        fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
            for item in iter {
                self.insert(item);
            }
        }
    }

    impl<T: Ord + PartialEq + PartialOrd + Display + Clone> FromIterator<T> for Tree<T> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            let mut list = Self::new();
            list.extend(iter);
            list
        }
    }

    impl<T: Ord + Debug + PartialEq + PartialOrd + Display + Clone> Debug for Tree<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self).finish()
        }
    }

    #[cfg(feature = "in-order")]
    impl<T: Ord + PartialEq + PartialOrd + Display + Clone> PartialEq for Tree<T> {
        fn eq(&self, other: &Self) -> bool {
            self.node_count() == other.node_count() && self.iter_dfs_in_order().eq(other)
        }
    }

    #[cfg(feature = "in-order")]
    impl<T: Ord + Eq + PartialEq + PartialOrd + Display + Clone> Eq for Tree<T> {}

    #[cfg(feature = "in-order")]
    impl<T: Ord + PartialEq + PartialOrd + Display + Clone> PartialOrd for Tree<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.iter_dfs_in_order().partial_cmp(other)
        }
    }

    #[cfg(feature = "in-order")]
    impl<T: Ord + PartialEq + PartialOrd + Display + Clone> Ord for Tree<T> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.iter_dfs_in_order().cmp(other)
        }
    }

    impl<T: Ord + Hash + PartialEq + PartialOrd + Display + Clone> Hash for Tree<T> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.node_count().hash(state);
            for item in self {
                item.hash(state);
            }
        }
    }
    // ------------------------------------------------------------------

    // Вставляет `elem` в новый узел поддерева `node`.
    fn insert_node<T: Ord + PartialEq + PartialOrd + Display>(
        node: NonNull<Node<T>>,
        elem: T,
    ) -> bool {
        unsafe {
            match elem.cmp(&(*node.as_ptr()).elem) {
                Ordering::Equal => false,
                Ordering::Less => {
                    if let Some(left) = (*node.as_ptr()).left {
                        insert_node(left, elem)
                    } else {
                        (*node.as_ptr()).left = Node::new_with_parent(elem, node);
                        true
                    }
                }
                Ordering::Greater => {
                    if let Some(right) = (*node.as_ptr()).right {
                        insert_node(right, elem)
                    } else {
                        (*node.as_ptr()).right = Node::new_with_parent(elem, node);
                        true
                    }
                }
            }
        }
    }

    // Возвращает строковое представление поддерева `node`.
    fn display_node<T: Display>(node: NonNull<Node<T>>) -> String {
        unsafe {
            let mut s: String = "".into();
            if let Some(left) = (*node.as_ptr()).left {
                s.push_str(&format!(
                    "{}->{}\n",
                    (*node.as_ptr()).elem,
                    (*left.as_ptr()).elem
                ));
                s.push_str(&display_node(left));
            } else if (*node.as_ptr()).right.is_some() {
                s.push_str(&format!(
                    "{}->node_null_{}\n",
                    (*node.as_ptr()).elem,
                    (*node.as_ptr()).elem
                ));
                s.push_str(&format!(
                    "node_null_{}[label=\"NULL\"]\n",
                    (*node.as_ptr()).elem
                ));
            }
            if let Some(right) = (*node.as_ptr()).right {
                s.push_str(&format!(
                    "{}->{}\n",
                    (*node.as_ptr()).elem,
                    (*right.as_ptr()).elem
                ));
                s.push_str(&display_node(right));
            } else {
                s.push_str(&format!(
                    "{}->node_null_{}\n",
                    (*node.as_ptr()).elem,
                    (*node.as_ptr()).elem
                ));
                s.push_str(&format!(
                    "node_null_{}[label=\"NULL\"]\n",
                    (*node.as_ptr()).elem
                ));
            }
            s
        }
    }

    // Находит данные в поддереве `fromnode`.
    fn find_node<T: Ord + PartialEq + PartialOrd + Display>(fromnode: Link<T>, elem: T) -> Link<T> {
        unsafe {
            if let Some(fromnode) = fromnode {
                match elem.cmp(&(*fromnode.as_ptr()).elem) {
                    Ordering::Equal => Some(fromnode),
                    Ordering::Less => find_node((*fromnode.as_ptr()).left, elem),
                    Ordering::Greater => find_node((*fromnode.as_ptr()).right, elem),
                }
            } else {
                fromnode
            }
        }
    }

    #[cfg(feature = "post-order")]
    use dfs_post_order::{leaf_post_order, post_order_recursive, successor_of_node_post_order};
    #[cfg(feature = "post-order")]
    mod dfs_post_order {
        use super::{Link, Node, NonNull};
        use std::fmt::Display;

        #[allow(dead_code)]
        pub fn post_order_recursive<T: Display>(node: Link<T>, buf: &mut Vec<&T>) {
            if let Some(node) = node {
                unsafe {
                    post_order_recursive((*node.as_ref()).left, buf);
                    post_order_recursive((*node.as_ref()).right, buf);
                    buf.push(&(*node.as_ref()).elem);
                }
            }
        }

        // Найдите преемника узла в дереве.
        pub fn successor_of_node_post_order<T: Display>(current_node: NonNull<Node<T>>) -> Link<T> {
            unsafe {
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
        }

        // Найдите лист.
        pub fn leaf_post_order<T: Display>(node: NonNull<Node<T>>) -> NonNull<Node<T>> {
            unsafe {
                if let Some(left) = (*node.as_ref()).left {
                    leaf_post_order(left)
                } else if let Some(right) = (*node.as_ref()).right {
                    leaf_post_order(right)
                } else {
                    node
                }
            }
        }
    }

    #[cfg(feature = "pre-order")]
    use dfs_pre_order::{pre_order_recursive, successor_of_node_pre_order};
    #[cfg(feature = "pre-order")]
    mod dfs_pre_order {
        use super::{Link, Node, NonNull};
        use std::fmt::Display;

        pub fn pre_order_recursive<T: Display>(node: Link<T>, buf: &mut Vec<&T>) {
            if let Some(node) = node {
                unsafe {
                    buf.push(&(*node.as_ref()).elem);
                    pre_order_recursive((*node.as_ref()).left, buf);
                    pre_order_recursive((*node.as_ref()).right, buf);
                }
            }
        }

        // Найдите преемника узла в дереве.
        pub fn successor_of_node_pre_order<T: Display>(node: NonNull<Node<T>>) -> Link<T> {
            unsafe {
                if let Some(node) = (*node.as_ref()).left {
                    Some(node)
                } else if let Some(node) = (*node.as_ref()).right {
                    Some(node)
                } else {
                    right_with_parent(node)
                }
            }
        }

        fn right_with_parent<T: Display>(node: NonNull<Node<T>>) -> Link<T> {
            unsafe {
                if let Some(parent) = (*node.as_ref()).parent {
                    next_right(Some(parent), node)
                } else {
                    None
                }
            }
        }

        fn next_right<T: Display>(node: Link<T>, child: NonNull<Node<T>>) -> Link<T> {
            unsafe {
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
        }
    }

    #[cfg(feature = "in-order")]
    use dfs_in_order::{in_order_recursive, leftmost_child_in_order, successor_of_node_in_order};
    #[cfg(feature = "in-order")]
    mod dfs_in_order {
        use super::{Link, Node, NonNull};
        use std::fmt::Display;
        pub fn in_order_recursive<T: Display>(node: Link<T>, buf: &mut Vec<&T>) {
            if let Some(node) = node {
                unsafe {
                    in_order_recursive((*node.as_ref()).left, buf);
                    buf.push(&(*node.as_ref()).elem);
                    in_order_recursive((*node.as_ref()).right, buf);
                }
            }
        }

        // Найдите преемника узла в дереве.
        pub fn successor_of_node_in_order<T: Display>(node: NonNull<Node<T>>) -> Link<T> {
            unsafe {
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
        }

        // Находим самого левого дочернего элемента `node` или самого `node`, если у него нет
        // левого дочернего элемента. `node` не может быть нулевым.
        pub fn leftmost_child_in_order<T: Display>(node: Link<T>) -> Link<T> {
            unsafe {
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
        }

        // Находим родителя в цепочке предков `node`, до которого можно добраться через его левую часть
        // ребенок.
        fn parent_with_left<T: Display>(node: NonNull<Node<T>>) -> Link<T> {
            unsafe {
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
        }
    }

    pub use bfs::{breadth_first_search, breadth_first_search_with_deque};
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
                    ret.push(&(*node.as_ref()).elem);
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
                queue.push((&(*root.as_ref()).elem, 1));
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
                    queue.push((&(*node.as_ref()).elem, level + 1));
                    breadth_first_search_recursive((*node.as_ref()).left, queue, level + 1);
                    breadth_first_search_recursive((*node.as_ref()).right, queue, level + 1);
                }
            }
        }
    }
}

/// $ cargo +nightly miri test binary_search_tree_good_nonnull
/// $ cargo test binary_search_tree_good_nonnull --features in-order -- --nocapture
/// $ cargo test binary_search_tree_good_nonnull --no-default-features --features pre-order -- --nocapture
/// $ cargo test binary_search_tree_good_nonnull --no-default-features --features post-order -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_success() {
        let mut tree: Tree<i32> = Tree::new();
        tree.insert(4);
        tree.insert(3);
        tree.insert(2);
        tree.insert(1);

        tree.insert(5);
        tree.insert(8);
        tree.insert(7);
        tree.insert(9);
        tree.insert(10);
        tree.insert(11);

        let elements = tree.breadth_first_search();
        assert_eq!(elements, vec![&4, &3, &5, &2, &8, &1, &7, &9, &10, &11]);
    }

    #[cfg(feature = "in-order")]
    #[test]
    fn test_dfs_in_order_display() {
        let mut tree: Tree<i32> = Tree::new();
        tree.insert(4);
        tree.insert(3);
        tree.insert(9);
        tree.insert(1);
        tree.insert(2);
        tree.insert(10);
        tree.insert(7);
        tree.insert(8);
        tree.insert(6);
        tree.insert(11);

        let fmt = tree.display();
        println!("{}", fmt);
    }

    // $ cargo +nightly miri test test_dfs_in_order_success
    #[cfg(feature = "in-order")]
    #[test]
    fn test_dfs_in_order_success() {
        let mut tree: Tree<i32> = Tree::new();
        tree.insert(4);
        tree.insert(3);
        tree.insert(9);
        tree.insert(1);
        tree.insert(2);
        tree.insert(10);
        tree.insert(7);
        tree.insert(8);
        tree.insert(6);
        tree.insert(11);

        assert!(tree.find(9), "find true");
        tree.remove(9);
        /*        assert!(!tree.find(9), "find false");

        let nodes = tree.depth_first_in_order_recursive(None);
        assert_eq!(nodes, vec![&1, &2, &3, &4, &6, &7, &8, &10, &11]);
        assert_eq!(nodes.len(), tree.node_count());

        let mut buf: Vec<&i32> = vec![];
        for item in tree.iter_dfs_in_order() {
            buf.push(item);
        }
        assert_eq!(buf, vec![&1, &2, &3, &4, &6, &7, &8, &10, &11]);

        assert!(tree.find(6), "find true");
        tree.remove(6);
        assert!(!tree.find(6), "find false");
        let nodes = tree.depth_first_in_order_recursive(None);
        assert_eq!(nodes, vec![&1, &2, &3, &4, &7, &8, &10, &11]);
        assert_eq!(nodes.len(), tree.node_count());

        let mut buf: Vec<&i32> = vec![];
        for item in tree.iter_dfs_in_order() {
            buf.push(item);
        }
        assert_eq!(buf, vec![&1, &2, &3, &4, &7, &8, &10, &11]);*/
    }

    #[cfg(feature = "in-order")]
    #[test]
    fn test_dfs_iter_in_order() {
        let mut tree: Tree<i32> = Tree::new();
        tree.insert(4);
        tree.insert(3);
        tree.insert(9);
        tree.insert(1);
        tree.insert(2);
        tree.insert(10);
        tree.insert(7);
        tree.insert(8);
        tree.insert(6);
        //println!("display:\n{}",tree.display());

        let mut buf: Vec<&i32> = vec![];
        for item in tree.iter_dfs_in_order() {
            buf.push(item);
        }
        assert_eq!(buf, vec![&1, &2, &3, &4, &6, &7, &8, &9, &10]);

        let elements = tree.depth_first_in_order_recursive(None);
        assert_eq!(elements, vec![&1, &2, &3, &4, &6, &7, &8, &9, &10]);

        assert_eq!(Some(&6), tree.successor_dfs_in_order(4), "4->6");
        assert_eq!(Some(&2), tree.successor_dfs_in_order(1), "1->2");
        assert_eq!(Some(&3), tree.successor_dfs_in_order(2), "2->3");
        assert_eq!(Some(&4), tree.successor_dfs_in_order(3), "3->4");
        assert_eq!(Some(&6), tree.successor_dfs_in_order(4), "4->6");
        assert_eq!(Some(&7), tree.successor_dfs_in_order(6), "6->7");
        assert_eq!(Some(&8), tree.successor_dfs_in_order(7), "7->8");
        assert_eq!(Some(&9), tree.successor_dfs_in_order(8), "8->9");
        assert_eq!(Some(&10), tree.successor_dfs_in_order(9), "9->10");
    }

    #[cfg(feature = "pre-order")]
    #[test]
    fn test_dfs_iter_pre_order() {
        let mut tree: Tree<i32> = Tree::new();
        tree.insert(4);
        tree.insert(3);
        tree.insert(9);
        tree.insert(1);
        tree.insert(2);
        tree.insert(10);
        tree.insert(7);
        tree.insert(8);
        tree.insert(6);
        let elements = tree.depth_first_pre_order_recursive();
        assert_eq!(elements, vec![&4, &3, &1, &2, &9, &7, &6, &8, &10]);

        assert_eq!(Some(&3), tree.successor_dfs_pre_order(4), "4->3");
        assert_eq!(Some(&9), tree.successor_dfs_pre_order(2), "2->9");
        assert_eq!(Some(&8), tree.successor_dfs_pre_order(6), "6->8");
        assert_eq!(Some(&10), tree.successor_dfs_pre_order(8), "8->10");

        let mut buf: Vec<&i32> = vec![];
        for item in tree.iter_dfs_pre_order() {
            buf.push(item);
        }
        assert_eq!(buf, vec![&4, &3, &1, &2, &9, &7, &6, &8, &10]);

        assert!(tree.find(8), "msg err: find false");
        assert!(tree.remove(8), "msg err: not removed");
        assert!(!tree.find(8), "msg err: find true");

        let nodes = tree.depth_first_pre_order_recursive();
        assert_eq!(nodes.len(), tree.node_count());
        assert_eq!(nodes, vec![&4, &3, &1, &2, &9, &7, &6, &10]);

        assert!(tree.find(9), "msg err: find false");
        assert!(tree.remove(9), "msg err: not removed");
        assert!(!tree.find(9), "msg err: find true");
        let nodes = tree.depth_first_pre_order_recursive();
        assert_eq!(nodes.len(), tree.node_count());
        assert_eq!(nodes, vec![&4, &3, &1, &2, &7, &6, &10]);

        let mut buf: Vec<&i32> = vec![];
        for item in tree.iter_dfs_pre_order() {
            buf.push(item);
        }
        assert_eq!(buf, vec![&4, &3, &1, &2, &7, &6, &10]);
    }

    #[cfg(feature = "pre-order")]
    #[test]
    fn test_dfs_pre_order_success() {
        let mut tree: Tree<i32> = Tree::new();
        tree.insert(4);
        tree.insert(3);
        tree.insert(9);
        tree.insert(1);
        tree.insert(2);
        tree.insert(10);
        tree.insert(7);
        tree.insert(8);
        tree.insert(6);

        assert!(tree.find(8), "find true");
        tree.remove(8);
        assert!(!tree.find(8), "find false");

        let mut buf: Vec<&i32> = vec![];
        for item in tree.iter_dfs_pre_order() {
            buf.push(item);
        }
        assert_eq!(buf, vec![&4, &3, &1, &2, &9, &7, &6, &10]);

        let nodes = tree.depth_first_pre_order_recursive();
        assert_eq!(nodes, vec![&4, &3, &1, &2, &9, &7, &6, &10]);
        assert_eq!(nodes.len(), tree.node_count());
    }

    #[cfg(feature = "post-order")]
    #[test]
    fn test_dfs_iter_post_order() {
        let mut tree: Tree<i32> = Tree::new();
        tree.insert(4);
        tree.insert(3);
        tree.insert(9);
        tree.insert(1);
        tree.insert(2);
        tree.insert(10);
        tree.insert(7);
        tree.insert(8);
        tree.insert(6);

        let elements = tree.depth_first_post_order_recursive();
        assert_eq!(elements, vec![&2, &1, &3, &6, &8, &7, &10, &9, &4]);

        assert_eq!(Some(&2), tree.successor_dfs_post_order(4), "4->2");
        assert_eq!(Some(&1), tree.successor_dfs_post_order(2), "2->1");
        assert_eq!(Some(&3), tree.successor_dfs_post_order(1), "1->3");
        assert_eq!(Some(&6), tree.successor_dfs_post_order(3), "3->6");
        assert_eq!(Some(&8), tree.successor_dfs_post_order(6), "6->8");
        assert_eq!(Some(&7), tree.successor_dfs_post_order(8), "8->7");
        assert_eq!(Some(&10), tree.successor_dfs_post_order(7), "7->10");
        assert_eq!(Some(&9), tree.successor_dfs_post_order(10), "10->9");
        assert_eq!(Some(&4), tree.successor_dfs_post_order(9), "9->4");

        let mut buf: Vec<&i32> = vec![];
        for item in tree.iter_dfs_post_order() {
            //Тут и in-order
            buf.push(item);
        }
        assert_eq!(buf, vec![&2, &1, &3, &6, &8, &7, &10, &9, &4]);

        assert!(tree.find(8), "find true");
        tree.remove(8);
        assert!(!tree.find(8), "find false");

        let mut buf: Vec<&i32> = vec![];
        for item in tree.iter_dfs_post_order() {
            //Тут и in-order
            buf.push(item);
        }
        let nodes = tree.depth_first_post_order_recursive();
        assert_eq!(nodes.len(), tree.node_count());
        assert_eq!(nodes, buf);
        assert_eq!(nodes, vec![&2, &1, &3, &6, &7, &10, &9, &4]);

        assert!(tree.find(9), "find true");
        tree.remove(9);
        assert!(!tree.find(9), "find false");

        let mut buf: Vec<&i32> = vec![];
        for item in tree.iter_dfs_post_order() {
            //Тут и in-order
            buf.push(item);
        }
        let nodes = tree.depth_first_post_order_recursive();
        assert_eq!(nodes.len(), tree.node_count());
        assert_eq!(nodes, buf);
        assert_eq!(nodes, vec![&2, &1, &3, &6, &10, &7, &4]);
    }

    #[cfg(feature = "post-order")]
    #[test]
    fn test_dfs_post_order_success() {
        let mut tree: Tree<i32> = Tree::new();
        tree.insert(4);
        tree.insert(3);
        tree.insert(9);
        tree.insert(1);
        tree.insert(2);
        tree.insert(7);
        tree.insert(8);
        tree.insert(6);
        tree.insert(10);

        assert!(tree.find(9), "find true");
        tree.remove(9);
        assert!(!tree.find(9), "find false");

        let mut buf: Vec<&i32> = vec![];
        for item in tree.iter_dfs_post_order() {
            buf.push(item);
        }

        let nodes = tree.depth_first_post_order_recursive();
        assert_eq!(nodes, vec![&2, &1, &3, &6, &10, &8, &7, &4]);
        assert_eq!(nodes, buf);
        assert_eq!(nodes.len(), tree.node_count());
    }

    /*#[test]
    fn test_std_btree(){
        use std::collections::BTreeMap;
        let mut tree = BTreeMap::new();
        tree.insert(4,       4);
        tree.insert(3,3);
        tree.insert(9,9);
        tree.insert(1,1);
        tree.insert(2,2);
        tree.insert(10,10);
        tree.insert(7,7);
        tree.insert(8,8);
        tree.insert(6,6);
        println!("BTreeMap:{:?}",tree);
    }*/
}
