#![allow(unused_imports)]

// Tree на основе NonNull
pub use ds_binary_tree::Tree;
mod ds_binary_tree{
    use std::cmp::Ordering;
    use std::fmt::{self, Debug, Display}; 
    use std::hash::{Hash, Hasher};
    use std::iter::FromIterator;
    use std::marker::PhantomData;
    use std::ptr::NonNull;

    //#[derive(Debug)]
    pub struct Tree<T: PartialEq + PartialOrd + Display> {
        root: Link<T>,
        count: usize,
        _boo: PhantomData<T>,
    }

    type Link<T> = Option<NonNull<Node<T>>>;

    #[derive(Debug)]
    pub struct Node<T> {
        left: Link<T>,
        right: Link<T>,
        parent: Link<T>,
        elem: T,
    }
 
    impl<T: PartialEq + PartialOrd + Display> Tree<T> {
        pub fn new() -> Self {
            Self {
                root: None,
                count: 0,
                _boo: PhantomData,
            }
        }

        /// Возвращает количество узлов дерева.
        pub fn node_count(&self) -> usize{
            assert!(self.count != 0 || self.root.is_none());
            self.count
        }

        /// Вставляем новый элемент в дерево; возвращает true, если вставка
        /// произошло, и значение false, если данные данные уже присутствовали в
        /// дерево.
        pub fn insert(&mut self, elem: T) -> bool{
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
        pub fn find(&self, elem: T) -> bool{
            !find_node(self.root, elem).is_none()
        }

        /// Возвращает строковое представление дерева для отладки.
        pub fn display(&self) -> String {
            if let Some(root) = self.root{
                return display_node(root, 0);
            }
            "".into()
        }

        /// Возвращает все данные дерева
        /// Метод прохода по дереву - поиск в глубину симметричным способом (In-order).
        pub fn depth_first_in_order(&self) -> Vec<&T> {
            let mut v = vec![];
            if let Some(root) = self.root{
                unsafe {
                    in_order((*root.as_ref()).left, &mut v);
                    v.push(&(*root.as_ref()).elem);
                    in_order((*root.as_ref()).right, &mut v);
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
        /// Метод прохода по дереву - поиск в глубину обратным способом (Post order).
        pub fn depth_first_post_order(&self) -> Vec<&T> {
            let mut v = vec![];
            if let Some(root) = self.root{
                unsafe {
                    post_order((*root.as_ref()).left, &mut v);
                    post_order((*root.as_ref()).right, &mut v);
                    v.push(&(*root.as_ref()).elem);
                }
            }
            v
        }

        /// Возвращает все данные дерева
        /// Метод прохода по дереву - поиск в глубину прямым способом (Pre order).
        pub fn depth_first_pre_order(&self) -> Vec<&T> {
            let mut v = vec![];
            if let Some(root) = self.root{
                unsafe {
                    v.push(&(*root.as_ref()).elem);
                    pre_order((*root.as_ref()).left, &mut v);
                    pre_order((*root.as_ref()).right, &mut v);
                }
            }
            v
        }

        /// Удаляем данный элемент из дерева; возвращает true, если такой узел был
        /// найдено и удалено, в противном случае — false.
        pub fn remove(&mut self, elem: T) -> bool{
            if let Some(node) = find_node(self.root, elem){
                self.remove_node(node);
                self.count -= 1;
                true           
            }else{
                false
            }
        }

        /// Найти следующий элемент данного элемента в дереве.  
        /// Метод прохода по дереву - поиск в глубину симметричным способом (In-order).
        pub fn successor_in_order(&self, elem: T) -> Option<&T>{
            unsafe {
                let node = find_node(self.root, elem);
                if let Some(n) = node {
                    if let Some(nodesucc) = successor_of_node_in_order(n){
                        return Some(&(*nodesucc.as_ref()).elem);
                    }
                }
                None
            }
        }

        /// Найти следующий элемент данного элемента в дереве.  
        /// Метод прохода по дереву - поиск в глубину обратным способом (Post order).
        pub fn successor_post_order(&self, _elem: T) -> Option<&T>{
            unimplemented!()
        }

        /// Найти следующий элемент данного элемента в дереве. 
        /// Метод прохода по дереву - поиск в глубину прямым способом (Pre order). 
        pub fn successor_pre_order(&self, _elem: T) -> Option<&T>{
            unimplemented!()
        }

        // Удаляем данный узел из дерева.
        fn remove_node(&mut self, mut node: NonNull<Node<T>>){
            unsafe {
                let lchild = (*node.as_ref()).left;
                let rchild = (*node.as_ref()).right;
                if lchild.is_none() && rchild.is_none() {
                    // У узла нет дочерних элементов, поэтому его можно безопасно удалить.
                    self.replace_node(node, None);
                } else if !lchild.is_none() && !rchild.is_none() {
                    // У узла есть оба дочерних узла.
                    // Находим преемника этого узла, заменяем данные нашего узла
                    // его данные, а затем рекурсивно удаляем преемника.
                    let mut succ = successor_of_node_in_order(node);
                    assert!(!succ.is_none());
                    if let Some(ref mut n) = succ{
                        //(*node.as_mut()).elem = (*n.as_ref()).elem; 
                        std::mem::swap(&mut (*node.as_mut()).elem, &mut (*n.as_mut()).elem);
                    }
                    self.remove_node(succ.unwrap());
                     
                } else if !lchild.is_none() {
                    // У узла остался только left дочерний элемент, поэтому замените его единственным дочерним элементом.
                    self.replace_node(node, lchild);
                } else if !rchild.is_none() {
                    // У узла остался только right дочерний элемент, поэтому замените его единственным дочерним элементом.
                    self.replace_node(node, rchild);
                } else {
                    panic!("unreachable");
                }
            }
        }

        // Заменяет `node` на `r` в дереве, устанавливая родительский элемент `node`
        // левая/правая ссылка на `node` со ссылкой на `r` и установкой родителя `r`
        // ссылка на родителя узла. `узел` не может быть нулевым.
        fn replace_node(&mut self, node: NonNull<Node<T>>, r: Link<T>){
            unsafe {
                let parent = (*node.as_ref()).parent;
                if parent.is_none() {
                    // Removing the root node.
                    self.root = r;
                    if let Some(mut n) = r{
                        (*n.as_mut()).parent = None;
                    }
                } else {
                    if let Some(mut n) = r{
                        (*n.as_mut()).parent = parent;
                    }
                    let mut parent = parent.unwrap();

                    if (*parent.as_ref()).left == Some(node) {
                        (*parent.as_mut()).left = r;
                    } else if (*parent.as_ref()).right == Some(node) {
                        (*parent.as_mut()).right = r;
                    }
                }
                // узел сейчас не используется, поэтому мы можем освободить его, который будет автоматически удален.
                let _ = Box::from_raw(node.as_ptr());
            }
        }

        pub fn iter_in_order(&self) -> IterInOrder<T> {
            IterInOrder::new(self.root, self.count)
        }
      
    }

    impl<T: PartialEq + PartialOrd + Display> Drop for Tree<T> {
        fn drop(&mut self) {
            while !self.root.is_none() {
                if let Some(root) = self.root{
                    self.remove_node(root);
                }
            }
        }
    }
    
    impl<'a,T: PartialEq + PartialOrd + Display> std::iter::IntoIterator for &'a Tree<T> {
        type IntoIter = IterInOrder<'a, T>;
        type Item = &'a T;

        fn into_iter(self) -> Self::IntoIter{
            IterInOrder::new(self.root, self.count)
        }
    }

    /// Итерация методом прохода по дереву - поиск в глубину симметричным способом (In-order)
    use iter_depth_first_in_order::IterInOrder;
    mod iter_depth_first_in_order{
        use std::fmt::Display; 
        use super::{Link,PhantomData};
        use super::depth_first_in_order::{leftmost_child_in_order, successor_of_node_in_order};

        pub struct IterInOrder<'a, T: PartialEq + PartialOrd + Display> {
            current_node: Link<T>,
            count: usize,
            elem: Option<&'a T>,
            is_start: bool,
            _boo: PhantomData<&'a T>,
        }

        impl<'a, T: PartialEq + PartialOrd + Display> IterInOrder<'a, T> {
            pub fn new(root: Link<T>, count: usize) -> Self{
                    Self{
                        current_node: root,
                        count,
                        elem: None,
                        is_start: false,
                        _boo: PhantomData
                    }
            }
        }

        impl<'a, T: PartialEq + PartialOrd + Display> Iterator for IterInOrder<'a, T> {
            type Item = &'a T;

            fn next(&mut self) -> Option<Self::Item> {
                if !self.is_start{
                    self.is_start = true;
                    let node: Link<T> = leftmost_child_in_order(self.current_node);
                    self.current_node = node;
                }
                if self.count > 0{
                    self.count-=1;
                    if let Some(node) = self.current_node{
                        unsafe {
                            self.elem = Some(&(*node.as_ref()).elem);
                        }
                        self.current_node = successor_of_node_in_order(node);                      
                    }else{
                        self.elem = None;
                    }              
                }else{
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
    impl<T: PartialEq + PartialOrd + Display> Default for Tree<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T: PartialEq + PartialOrd + Display + Clone> Clone for Tree<T> {
        fn clone(&self) -> Self {
            let mut new_list = Self::new();
            for item in self {
                new_list.insert(item.clone());
            }
            new_list
        }
    }

    impl<T: PartialEq + PartialOrd + Display> Extend<T> for Tree<T> {
        fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
            for item in iter {
                self.insert(item);
            }
        }
    }

    impl<T: PartialEq + PartialOrd + Display> FromIterator<T> for Tree<T> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            let mut list = Self::new();
            list.extend(iter);
            list
        }
    }

    impl<T: Debug + PartialEq + PartialOrd + Display> Debug for Tree<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self).finish()
        }
    }

    impl<T: PartialEq + PartialOrd + Display> PartialEq for Tree<T> {
        fn eq(&self, other: &Self) -> bool {
            self.node_count() == other.node_count() && self.iter_in_order().eq(other)
        }
    }

    impl<T: Eq + PartialEq + PartialOrd + Display> Eq for Tree<T> {}

    impl<T: PartialEq + PartialOrd + Display> PartialOrd for Tree<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.iter_in_order().partial_cmp(other)
        }
    }

    impl<T: Ord + PartialEq + PartialOrd + Display> Ord for Tree<T> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.iter_in_order().cmp(other)
        }
    }

    impl<T: Hash + PartialEq + PartialOrd + Display> Hash for Tree<T> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.node_count().hash(state);
            for item in self {
                item.hash(state);
            }
        }
    }
    // ------------------------------------------------------------------
 
    impl<T> Node<T> {
        fn new(elem: T) -> Link<T>  {
            unsafe {
                let new = NonNull::new_unchecked(Box::into_raw(Box::new(Self {
                    left: None,
                    right: None,// std::ptr::null_mut()
                    parent: None,
                    elem,
                })));
                Some(new)
            }
        }
        fn new_with_parent(elem: T, parent: NonNull<Node<T>>) -> Link<T>  {
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

    // Вставляет `elem` в новый узел поддерева `node`.
    fn insert_node<T:PartialEq+PartialOrd>(node: NonNull<Node<T>>, elem: T) -> bool {
        unsafe {
            if (*node.as_ptr()).elem == elem {
                false
            }else if elem < (*node.as_ptr()).elem  {
                if let Some(left) = (*node.as_ptr()).left{
                    insert_node(left, elem)
                }else{
                    (*node.as_ptr()).left = Node::new_with_parent(elem, node);
                    true
                }
            }else{// elem > (*node.as_ptr()).elem
                if let Some(right) = (*node.as_ptr()).right{
                    insert_node(right, elem)
                }else{
                    (*node.as_ptr()).right = Node::new_with_parent(elem, node);
                    true
                }
            }
        }
    }

    // Возвращает строковое представление поддерева `node`.
    fn display_node<T: Display>(node: NonNull<Node<T>>, indent: usize) -> String {
        let indent_str = " ".repeat(indent);
        unsafe {
            let mut s = format!("{}{}\n", indent_str, (*node.as_ptr()).elem);
            if let Some(right) = (*node.as_ptr()).right{
                s.push_str(&display_node(right, indent + 2));
            } else{
                s.push_str(".\n");
            }
            if let Some(left) = (*node.as_ptr()).left{
                s.push_str(&display_node(left, indent + 2));
            } else{
                s.push_str(".\n");
            }
            s
        } 
    }

    // Находит данные в поддереве `fromnode`. 
    fn find_node<T: PartialEq + PartialOrd>(fromnode: Option<NonNull<Node<T>>>, elem: T) -> Option<NonNull<Node<T>>>{
        unsafe {
            if let Some(fromnode) = fromnode{
                if (*fromnode.as_ptr()).elem == elem {
                    Some(fromnode)
                } else if elem < (*fromnode.as_ptr()).elem {
                    find_node((*fromnode.as_ptr()).left, elem)
                } else {
                    find_node((*fromnode.as_ptr()).right, elem)
                }
            }else{
                fromnode
            }
        }
    } 

    use depth_first_post_order::post_order;
    mod depth_first_post_order{
        use super::Link;
        pub fn post_order<T>(node: Link<T>, buf: &mut Vec<&T>){
            if let Some(node) = node{
                unsafe { 
                    post_order((*node.as_ref()).left, buf);
                    post_order((*node.as_ref()).right, buf);
                    buf.push(&(*node.as_ref()).elem);
                }
            }
        } 
    }

    use depth_first_pre_order::pre_order;
    mod depth_first_pre_order{
        use super::Link;
        pub fn pre_order<T>(node: Link<T>, buf: &mut Vec<&T>){
            if let Some(node) = node{
                unsafe { 
                    buf.push(&(*node.as_ref()).elem);
                    pre_order((*node.as_ref()).left, buf);
                    pre_order((*node.as_ref()).right, buf);
                }
            }
        } 
    }

    use depth_first_in_order::{leftmost_child_in_order, successor_of_node_in_order, in_order};
    mod depth_first_in_order{
        use super::{Link, NonNull, Node};
        pub fn in_order<T>(node: Link<T>, buf: &mut Vec<&T>){
            if let Some(node) = node{
                unsafe { 
                    in_order((*node.as_ref()).left, buf);
                    buf.push(&(*node.as_ref()).elem);
                    in_order((*node.as_ref()).right, buf);
                }
            }
        } 
        
        // Находим самого левого дочернего элемента `node` или самого `node`, если у него нет
        // левого дочернего элемента. `node` не может быть нулевым.
        pub fn leftmost_child_in_order<T>(node: Link<T>) -> Link<T>{
            unsafe {
                if let Some(node) = node{
                    if (*node.as_ref()).left.is_none() {
                        Some(node)
                    } else {
                        leftmost_child_in_order((*node.as_ref()).left)
                    }
                }else{
                    node
                }
            }
        }

        // Найдите преемника узла в дереве.
        pub fn successor_of_node_in_order<T>(node: NonNull<Node<T>>) -> Link<T> {
            unsafe {
                if !(*node.as_ref()).right.is_none() {
                    // Случай 1: узел имеет правого дочернего элемента; тогда преемником является
                    // самый левый дочерний элемент этого правого дочернего элемента (или самого правого дочернего элемента, если
                    // у него нет левых потомков).
                    leftmost_child_in_order((*node.as_ref()).right)
                } else {
                    // Случай 2: нет правого дочернего элемента; затем пройдите по родительским ссылкам, чтобы найти
                    // узел, левым дочерним элементом которого мы являемся. Не удалось найти такого родителя
                    // до достижения корня означает, что преемника нет.
                    parent_with_left(node)
                }
            }
        }

        // Находим родителя в цепочке предков `node`, до которого можно добраться через его левую часть
        // ребенок.
        fn parent_with_left<T>(node: NonNull<Node<T>>) -> Link<T> {
            unsafe {
                // Если у этого узла есть родительский элемент, и у этого родителя есть левый дочерний элемент, и
                // `node` — это левый дочерний элемент, мы его нашли!
                let parent = (*node.as_ref()).parent;
                if let Some(parent) = parent{
                    if let Some(left) = (*parent.as_ref()).left{
                        if std::ptr::eq(left.as_ptr(), node.as_ptr()){
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

}


/// $ cargo test binary_search_tree_good_nonnull -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut tree: Tree<i32> = Tree::new();
        tree.insert(5);
        tree.insert(4);
        tree.insert(8);
        tree.insert(6);
        tree.insert(9);

        assert!(tree.find(8));
        tree.remove(8);
        assert!(!tree.find(8));
 
        let nodes = tree.depth_first_in_order();
        assert_eq!(nodes.len(),tree.node_count());
    }

    #[test]
    fn test_iter_in_order() {
        // Depth First Search Symmetrical method
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
        println!("display:\n{}",tree.display());

        for item in tree.iter_in_order(){
            println!("iter:{}",item);
        }
        let elements = tree.depth_first_in_order();
        println!("in_order:{:?}",elements);
        assert_eq!(elements,vec![&1, &2, &3, &4, &6, &7, &8, &9, &10]);

        assert_eq!(Some(&4), tree.successor_in_order(3));
    }

    #[test]
    fn test_iter_pre_order() {
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
        let elements = tree.depth_first_pre_order();
        println!("pre_order:{:?}",elements);
        assert_eq!(elements,vec![&4, &3, &1, &2, &9, &7, &6, &8, &10]);
    }
    
    #[test]
    fn test_iter_post_order() {
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
        let elements = tree.depth_first_post_order();
        println!("post_order:{:?}",elements);
        assert_eq!(elements,vec![&2, &1, &3, &6, &8, &7, &10, &9, &4]);
    }
}