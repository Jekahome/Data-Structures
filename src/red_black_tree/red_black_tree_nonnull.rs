
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use llrb::{Tree,Node};
mod llrb {

    use std::cmp::Ordering;
    use std::fmt::{self, Debug, Display};
    use std::hash::{Hash, Hasher};
    use std::iter::FromIterator;
    use std::marker::PhantomData;
    use std::ptr::NonNull;

    pub struct Tree<T: Ord + PartialEq + PartialOrd + Display + Clone + Debug> {
        pub root: Link<T>,
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

    enum OperationPut{
        Left,
        Right,
        FlipColors, 
        Skip
    }

    enum OperationRemoveFourOptions{
        RedLeaf,
        BlackLeaf,
        NodeWithChildren,
        BlackNodeWithRedLeaf,
        
        Unimplemented
    }
    enum OperationRemoveBlackLeaf{
        LeftRedABlackBRedCleaf,// 2.1.1.1
        RightRedABlackBRedCleaf,// 2.1.1.2
        LeftRedABlackBleaf,// 2.1.2.1
        RightRedABlackBleaf,// 2.1.2.2
        BlackARedBWithBlackChildrenLeaf,//2.2.4
        BlackARedBWithBlackChildrenRightHaveRedLeaf,//2.2.1
        
        BlackALeftBlackBRedDleaf,//2.3.1.1
        BlackARightBlackBRedDleaf,//2.3.1.2
        BlackALeftBlackBleaf,//2.3.2.1
        BlackARightBlackBleaf,//2.3.2.2
        Root,
        Unimplemented
    }

    impl<T: Ord + PartialEq + PartialOrd + Display + Clone + Debug> Tree<T> {
        pub fn new() -> Self {
            Self {
                root: None,
                count: 0,
                _boo: PhantomData,
            }
        }
 
        pub fn node_count(&self) -> usize {
            assert!(self.count != 0 || self.root.is_none());
            self.count
        }
 
        pub fn put(&mut self, value: T) -> bool {
            let elem = value.clone();
            unsafe{
                if self.root.is_some(){
                    if !Tree::put_node(self.root.unwrap(), value){
                        return false;
                    }
                }else{
                    self.root = Node::new_black(value);   
                }
               
                self.count += 1;
                true   
           }
        }
 
        /// Найти элемент в дереве.
        pub fn contains(&self, value: T) -> bool {
            !find_node(self.root, value).is_none()
        }

        // https://youtu.be/a9EwBVLQ364?list=PL79T_6pcZAxwN-q69gAnSle0J_IliNawo&t=613
        // https://youtu.be/OAkmHIR9YkY?t=1096
        // https://www.youtube.com/watch?v=T70nn4EyTrs
        pub fn remove(&mut self, value: T) -> bool {
            unsafe{
                if let Some(node) = find_node(self.root, value) {
                    if !self.remove_node(node){
                        return false;
                    }
                }else{
                    return false;
                } 
                self.count -= 1;
                true               
            }
        }

        unsafe fn operation_remove(&self, node_x: &NonNull<Node<T>>) -> OperationRemoveFourOptions{
            if (*node_x.as_ref()).is_red && (*node_x.as_ref()).left.is_none() && (*node_x.as_ref()).right.is_none(){
                return OperationRemoveFourOptions::RedLeaf;
            }
            if (*node_x.as_ref()).is_red{
                if (*node_x.as_ref()).left.is_some() && (*node_x.as_ref()).right.is_some(){
                    return OperationRemoveFourOptions::NodeWithChildren;
                } 
            }
            if !(*node_x.as_ref()).is_red{
                if (*node_x.as_ref()).left.is_some() && (*node_x.as_ref()).right.is_some(){
                    return OperationRemoveFourOptions::NodeWithChildren;
                } else if (*node_x.as_ref()).left.is_some() && (*node_x.as_ref()).right.is_none(){
                    if let Some(left) = (*node_x.as_ref()).left{
                        if (*left.as_ref()).is_red && (*left.as_ref()).left.is_none() && (*left.as_ref()).right.is_none(){
                            return OperationRemoveFourOptions::BlackNodeWithRedLeaf;
                        }
                    }
                }else if (*node_x.as_ref()).left.is_none() && (*node_x.as_ref()).right.is_none(){
                    return OperationRemoveFourOptions::BlackLeaf;
                }
            }

            panic!("такого варианта быть не может");
            OperationRemoveFourOptions::Unimplemented
        }

        unsafe fn operation_remove_black_leaf(&self, node_x: &NonNull<Node<T>>) -> OperationRemoveBlackLeaf{
            // TODO: не учитывается рекурсивный подьем после измененной высоты
            if let Some(node_a) = (*node_x.as_ref()).parent{
                if (*node_a.as_ref()).is_red{
                    if let Some(node_b) = (*node_a.as_ref()).left{
                        if !std::ptr::eq(node_b.as_ptr(), node_x.as_ptr()) {
                            if !(*node_b.as_ref()).is_red{
                                if let Some(node_c) = (*node_b.as_ref()).left{
                                    if (*node_c.as_ref()).is_red && 
                                        (*node_c.as_ref()).left.is_none() && 
                                        (*node_c.as_ref()).right.is_none() && (*node_b.as_ref()).right.is_none(){
                                        return OperationRemoveBlackLeaf::LeftRedABlackBRedCleaf; // 2.1.1.1 
                                    }
                                }else if (*node_b.as_ref()).right.is_none(){
                                   
                                    return OperationRemoveBlackLeaf::LeftRedABlackBleaf;// 2.1.2.1
                                }
                            }
                        }
                    }
                    if let Some(node_b) = (*node_a.as_ref()).right{
                        if !std::ptr::eq(node_b.as_ptr(), node_x.as_ptr()) {
                            if !(*node_b.as_ref()).is_red{
                                if let Some(node_c) = (*node_b.as_ref()).left{
                                    if (*node_c.as_ref()).is_red && 
                                        (*node_c.as_ref()).left.is_none() && 
                                        (*node_c.as_ref()).right.is_none() && (*node_b.as_ref()).right.is_none(){
                                        return OperationRemoveBlackLeaf::RightRedABlackBRedCleaf; // 2.1.1.2
                                    }
                                }else if (*node_b.as_ref()).right.is_none(){
                                    return OperationRemoveBlackLeaf::RightRedABlackBleaf;// 2.1.2.2
                                }
                            }
                        }
 
                    }
                }else{ 
                    // node A is black
                    if let Some(node_b) = (*node_a.as_ref()).left{ 
                        if !std::ptr::eq(node_b.as_ptr(), node_x.as_ptr()) { 
                            if (*node_b.as_ref()).is_red && (*node_b.as_ref()).right.is_some(){
                                let node_c = (*node_b.as_ref()).right.unwrap();
                                if !(*node_c.as_ref()).is_red {
                                    if (*node_c.as_ref()).left.is_none() && (*node_c.as_ref()).right.is_none() {
                                        return OperationRemoveBlackLeaf::BlackARedBWithBlackChildrenLeaf;// 2.2.2
                                    }
                                    if (*node_c.as_ref()).left.is_some(){
                                        if let Some(node_d) = (*node_c.as_ref()).left{
                                            if (*node_d.as_ref()).is_red{
                                                return OperationRemoveBlackLeaf::BlackARedBWithBlackChildrenRightHaveRedLeaf;// 2.2.1
                                            }
                                        }
                                    }                                
                                }
                            }else if !(*node_b.as_ref()).is_red && (*node_b.as_ref()).left.is_some() && (*node_b.as_ref()).right.is_none(){
                                if let Some(node_d) = (*node_b.as_ref()).left{
                                    if (*node_d.as_ref()).is_red{
                                        return OperationRemoveBlackLeaf::BlackALeftBlackBRedDleaf;// 2.3.1.1
                                    }
                                }
                            }else if !(*node_b.as_ref()).is_red && (*node_b.as_ref()).left.is_none() && (*node_b.as_ref()).right.is_none(){
                                return OperationRemoveBlackLeaf::BlackALeftBlackBleaf;// 2.3.2.1
                            }
                        }
                    }
                    if let Some(node_b) = (*node_a.as_ref()).right{ 
                        if !std::ptr::eq(node_b.as_ptr(), node_x.as_ptr()) { 
                            if !(*node_b.as_ref()).is_red && (*node_b.as_ref()).left.is_some() && (*node_b.as_ref()).right.is_none(){
                                if let Some(node_d) = (*node_b.as_ref()).left{
                                    if (*node_d.as_ref()).is_red{
                                        return OperationRemoveBlackLeaf::BlackARightBlackBRedDleaf;// 2.3.1.2
                                    }
                                }
                            }else if !(*node_b.as_ref()).is_red && (*node_b.as_ref()).left.is_none() && (*node_b.as_ref()).right.is_none(){
                                return OperationRemoveBlackLeaf::BlackARightBlackBleaf;// 2.3.2.2

                            }
                        }
                    }
                }
            }else{
                return OperationRemoveBlackLeaf::Root;
            }
            OperationRemoveBlackLeaf::Unimplemented
        }

        /*
            2.1.1.1 remove black X 

                 P            P
                //           // 
               A            B
              / \          / \
             B   X   =>   C   A
            //
           C
        */
        unsafe fn remove_black_leaf_2_1_1_1(&mut self, node_x: NonNull<Node<T>>) -> bool{
  
            let mut node_a = (*node_x.as_ptr()).parent.unwrap();

            let mut node_b = (*node_a.as_ptr()).left.unwrap();
            (*node_b.as_mut()).parent = (*node_a.as_ptr()).parent;
            if let Some(ref mut parent) = (*node_b.as_mut()).parent{
                (*parent.as_mut()).left = Some(node_b);
            }
            (*node_a.as_mut()).left = None;
            (*node_a.as_mut()).right = None;
            (*node_a.as_mut()).is_red = false;

            (*node_b.as_mut()).right = Some(node_a);
            (*node_a.as_mut()).parent = Some(node_b);
            (*node_b.as_mut()).is_red = true;
            if let Some(ref mut node_c) = (*node_b.as_mut()).left{
                (*node_c.as_mut()).is_red = false;
            }
            self.remove_leaf(node_x)
        }

        /*
          2.1.1.2 remove black X 

                P              P
               //             //
              A              C
             / \            / \
            X   B   =>     A   B 
               //
               C

        */
        unsafe fn remove_black_leaf_2_1_1_2(&mut self, node_x: NonNull<Node<T>>) -> bool{
            let mut node_a = (*node_x.as_ptr()).parent.unwrap();
            (*node_a.as_ptr()).is_red = false;
            let mut parent: NonNull<Node<T>> = (*node_a.as_ptr()).parent.unwrap();
            let mut node_b = (*node_a.as_ptr()).right.unwrap();
            let mut node_c = (*node_b.as_ptr()).left.unwrap();
 
            (*parent.as_ptr()).left = Some(node_c);
            (*node_c.as_ptr()).parent = Some(parent);

            (*node_b.as_ptr()).left = None;
            (*node_c.as_ptr()).right = Some(node_b);
            (*node_b.as_ptr()).parent = Some(node_c);
            (*node_c.as_ptr()).left = Some(node_a);
            (*node_a.as_ptr()).parent = Some(node_c);
            
            (*node_a.as_ptr()).right = None;
            self.remove_leaf(node_x)
        }

        /*
            2.1.2.1 remove black X 

                 P            P
                //           / 
               A            A
              / \          //  
             B   X   =>   B    
             
        */
        unsafe fn remove_black_leaf_2_1_2_1(&mut self, node_x: NonNull<Node<T>>) -> bool{ 
            // A не может быть root
            // A только слева от P 
            if let Some(ref mut node_a) = (*node_x.as_ptr()).parent{
                (*node_a.as_mut()).is_red = false;
                if let Some(ref mut node_b) = (*node_a.as_ptr()).left{
                    (*node_b.as_mut()).is_red = true;
                }
            }
            self.remove_leaf(node_x)
        }

        /*
           2.1.2.2 remove black X 
 
                P            P
               //           /
              A     =>     B
             / \          //
            X   B        A

        */
        unsafe fn remove_black_leaf_2_1_2_2(&mut self, node_x: NonNull<Node<T>>) -> bool{ 
            println!("******************");
            let node_a = (*node_x.as_ptr()).parent.unwrap();
            let node_b = (*node_a.as_ptr()).right.unwrap();
             
            let parent = (*node_a.as_ptr()).parent.unwrap();

             
            if let Some(left) = (*parent.as_ptr()).left{
                if std::ptr::eq(node_a.as_ptr(), left.as_ptr()) { 
                    (*parent.as_ptr()).left = Some(node_b);
                }else{
                    (*parent.as_ptr()).right = Some(node_b);
                }
            }else{
                (*parent.as_ptr()).right = Some(node_b);
            }

            (*node_b.as_ptr()).parent = Some(parent);
            (*node_b.as_ptr()).left = Some(node_a);
            (*node_a.as_ptr()).parent = Some(node_b);
            (*node_a.as_ptr()).right = None;

            self.remove_leaf(node_x)
        }

        /*
            2.2.1 remove black X 

                P            P
                |            | 
                A            B
              // \            \
             B    X   =>       A
              \               //
               C             C
              //
             D 
        */        
        unsafe fn remove_black_leaf_2_2_1(&mut self, node_x: NonNull<Node<T>>) -> bool{ 
            let n_a = (*node_x.as_ref()).parent.unwrap();
            if (*n_a.as_ref()).parent.is_some(){
                let mut parent = (*n_a.as_ptr()).parent.unwrap(); 
                let mut node_a_from_left = false;
                if let Some(ref mut node_a) = (*parent.as_ptr()).left{
                    if std::ptr::eq(node_a.as_ptr(), n_a.as_ptr()) { 
                        node_a_from_left = true;
                    } 
                } 
                let mut node_a = if node_a_from_left{
                    (*parent.as_ptr()).left.unwrap()
                }else{
                    (*parent.as_ptr()).right.unwrap()
                };
    
                let mut node_b = (*node_a.as_ptr()).left.unwrap();
                let mut node_c = (*node_b.as_ptr()).right.unwrap();
                let mut node_d = (*node_c.as_ptr()).left.unwrap();
     
                (*node_b.as_mut()).right = Some(node_d);
                (*node_b.as_mut()).parent = Some(node_c);
                (*node_d.as_mut()).parent = Some(node_b);
    
                (*node_a.as_mut()).parent = Some(node_c);
                (*node_a.as_mut()).left = None;
                (*node_a.as_mut()).right = None;
    
                (*node_d.as_mut()).is_red = false;
                (*node_c.as_mut()).left = Some(node_b);
                (*node_c.as_mut()).right = Some(node_a);
                (*node_c.as_mut()).parent = Some(parent);
     
                if node_a_from_left{
                    (*parent.as_mut()).left = Some(node_c);
                }else{
                    (*parent.as_mut()).right = Some(node_c);
                }
                 
            }else{
                // node_a is root
                let mut node_b = (*n_a.as_ptr()).left.unwrap();
                 
                let mut node_c = (*node_b.as_ptr()).right.unwrap();
                let mut node_d = (*node_c.as_ptr()).left.unwrap();
                (*node_d.as_ptr()).is_red = false;
              
                let _ = std::mem::replace(&mut self.root,  Some(node_c));
                if let Some(node_c) = self.root{
                    (*node_c.as_ptr()).parent = None;
                    (*node_c.as_ptr()).right = Some(n_a);
                    (*node_c.as_ptr()).left = Some(node_b);
                    (*node_b.as_ptr()).parent = Some(node_c);
                    (*n_a.as_ptr()).parent = Some(node_c);
                    (*n_a.as_ptr()).left = None;
                    (*node_b.as_ptr()).right = Some(node_d);
                    (*node_d.as_ptr()).parent = Some(node_b);
                }
            }
            return self.remove_leaf(node_x);
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
        unsafe fn remove_black_leaf_2_2_2(&mut self, node_x: NonNull<Node<T>>) -> bool{ 
            let n_a = (*node_x.as_ref()).parent.unwrap();
            if (*n_a.as_ref()).parent.is_some(){
                let mut parent = (*n_a.as_ptr()).parent.unwrap(); 
                let mut node_a_from_left = false;
                if let Some(ref mut node_a) = (*parent.as_ptr()).left{
                    if std::ptr::eq(node_a.as_ptr(), n_a.as_ptr()) { 
                        node_a_from_left = true;
                    } 
                } 
            
                let mut node_a = if node_a_from_left{
                    (*parent.as_ptr()).left.unwrap()
                }else{
                    (*parent.as_ptr()).right.unwrap()
                };
    
                let mut node_b = (*node_a.as_ptr()).left.unwrap();
                (*node_b.as_mut()).is_red = false;
     
                (*node_b.as_mut()).parent = Some(parent);
    
                let mut node_c = (*node_b.as_ptr()).right.unwrap();
                (*node_c.as_mut()).is_red = true;
                (*node_b.as_mut()).right = Some(node_a);
                
                (*node_a.as_mut()).left = Some(node_c);
                (*node_c.as_mut()).parent = Some(node_a);
                (*node_a.as_mut()).parent = Some(node_b);
     
                if node_a_from_left{
                    (*parent.as_mut()).left = Some(node_b);
                }else{
                    (*parent.as_mut()).right = Some(node_b);
                }

            }else{
                // node_a is root
                let mut node_b = (*n_a.as_ptr()).left.unwrap();
                (*node_b.as_mut()).is_red = false;
                let mut node_c = (*node_b.as_ptr()).right.unwrap();
                (*node_c.as_mut()).is_red = true;
                (*node_c.as_ptr()).parent = Some(n_a);
                (*n_a.as_ptr()).left = Some(node_c);
                let _ = std::mem::replace(&mut self.root,  Some(node_b));
                if let Some(node_b) = self.root{
                    (*node_b.as_ptr()).parent = None;
                    (*node_b.as_ptr()).right = Some(n_a);
                    (*n_a.as_ptr()).parent = Some(node_b);
                }
            }
            return self.remove_leaf(node_x);
        }
 
        /*
          2.3.1.1 remove black X 

              A           B
             / \         / \
            B   X  =>   D   A
           //
          D
        
        */
        unsafe fn remove_black_leaf_2_3_1_1(&mut self, node_x: NonNull<Node<T>>) -> bool{ 
            let node_a = (*node_x.as_ptr()).parent.unwrap();
            let node_b = (*node_a.as_ptr()).left.unwrap();
            let node_d = (*node_b.as_ptr()).left.unwrap();
            (*node_d.as_ptr()).is_red = false;
 
            if let Some(parent) = (*node_a.as_ptr()).parent{
                let mut node_a_from_left = false;
                let mut node_a = {
                    if let Some(p_node_a) = (*parent.as_ptr()).left{
                        if std::ptr::eq(p_node_a.as_ptr(), node_a.as_ptr()) {
                            node_a_from_left = true;
                            p_node_a
                        } else{
                            (*parent.as_ptr()).right.unwrap()   
                        }               
                    }else{
                        (*parent.as_ptr()).right.unwrap()
                    }
                };
                
                let mut node_b = (*node_a.as_ptr()).left.unwrap();
                (*node_b.as_ptr()).parent = Some(parent);
                (*node_b.as_ptr()).right = Some(node_a);
                (*node_a.as_ptr()).parent = Some(node_b);
                (*node_a.as_ptr()).left = None;
                if node_a_from_left{
                    (*parent.as_ptr()).left = Some(node_b);
                }else{
                    (*parent.as_ptr()).right = Some(node_b);
                }   
            }else{
                // node_a is root
                //self.root = Some(node_b);
                let _ = std::mem::replace(&mut self.root,  Some(node_b));

                if let Some(node_b) = self.root{
                    (*node_b.as_ptr()).parent = None;
                    (*node_b.as_ptr()).right = Some(node_a);
                    (*node_a.as_ptr()).parent = Some(node_b);
                    (*node_a.as_ptr()).left = None;
                }
            }
            return self.remove_leaf(node_x);
        }

        /*
            2.3.1.2 remove black X 

              A           D
             / \         / \
            X   B  =>   A   B
               //
               D
        
        */
        unsafe fn remove_black_leaf_2_3_1_2(&mut self, node_x: NonNull<Node<T>>) -> bool{ 
            let node_a = (*node_x.as_ptr()).parent.unwrap();
            let node_b = (*node_a.as_ptr()).right.unwrap();
            let node_d = (*node_b.as_ptr()).left.unwrap();
            (*node_d.as_ptr()).is_red = false;

            if let Some(parent) = (*node_a.as_ptr()).parent{
                let mut node_a_from_left = false;
                let node_a = {
                    if let Some(p_node_a) = (*parent.as_ptr()).left{
                        if std::ptr::eq(p_node_a.as_ptr(), node_a.as_ptr()) {
                            node_a_from_left = true;
                            p_node_a
                        } else{
                            (*parent.as_ptr()).right.unwrap()   
                        }               
                    }else{
                        (*parent.as_ptr()).right.unwrap()
                    }
                };

                (*node_d.as_ptr()).parent = Some(parent);
                (*node_d.as_ptr()).left = Some(node_a);
                (*node_d.as_ptr()).right = Some(node_b);
                (*node_a.as_ptr()).parent = Some(node_d);
                (*node_b.as_ptr()).parent = Some(node_d);
                (*node_a.as_ptr()).right = None;
                (*node_b.as_ptr()).left = None;

                if node_a_from_left{
                    (*parent.as_ptr()).left = Some(node_d);
                }else{
                    (*parent.as_ptr()).right = Some(node_d);
                }
            }else{
                // node_a is root
                let _ = std::mem::replace(&mut self.root,  Some(node_d));
                if let Some(node_d) = self.root{
                    (*node_d.as_ptr()).parent = None;
                    (*node_d.as_ptr()).left = Some(node_a);
                    (*node_d.as_ptr()).right = Some(node_b);
                    (*node_a.as_ptr()).parent = Some(node_d);
                    (*node_b.as_ptr()).parent = Some(node_d);
                    (*node_a.as_ptr()).right = None;
                    (*node_b.as_ptr()).left = None;
                }
            }
            return self.remove_leaf(node_x);
        }

        /*
            https://youtu.be/gCPtoK07yRA?t=3266
            https://youtu.be/gCPtoK07yRA?t=4599

            2.3.2.1 remove black X 

            4 Возможных случая после удаления !!!!

            1) P is red

              P           P
              ||          |
              A           A
             / \   =>    //   
            B   X       B

            2) P is black

             P           P
             |           |
             A           A
            / \   =>    //   => next check node P
           B   X       B


        */
        unsafe fn remove_black_leaf_2_3_2_1(&mut self, node_x: NonNull<Node<T>>) -> bool{
            // найти кандидата для замены цвета
            // По анимации видно что ищут красный узел и спускают красный цвет по ветвям вниз пока не дойдут до B
           
           // Двойная чернота (т.е. долг по высоте) если она передвигается на красный узел то он становится черным
            // если на черный узел то он станет дважды черным

            // 1. Красим B в красный
            // 2. Если P был красным то теперь он станет черным и конец
            //    Если P был черным мы передвигает дважды черноту выше
            unimplemented!();
            false
        }

        /*
            2.3.2.2 remove black X 

              P           P
              |           |
              A           B
             / \   =>    //  =>  ...
            X   B       A

        */
        unsafe fn remove_black_leaf_2_3_2_2(&mut self, node_x: NonNull<Node<T>>) -> bool{
            // найти кандидата для замены цвета
            let node_a = (*node_x.as_ptr()).parent.unwrap();
            (*node_a.as_ptr()).is_red = true;
            let node_b = (*node_a.as_ptr()).right.unwrap();
            if let Some(parent) = (*node_a.as_ptr()).parent{
                let mut node_a_from_left = false;
                let node_a = {
                    if let Some(p_node_a) = (*parent.as_ptr()).left{
                        if std::ptr::eq(p_node_a.as_ptr(), node_a.as_ptr()) {
                            node_a_from_left = true;
                            p_node_a
                        } else{
                            (*parent.as_ptr()).right.unwrap()   
                        }               
                    }else{
                        (*parent.as_ptr()).right.unwrap()
                    }
                };

                (*node_b.as_ptr()).parent = Some(parent);
                (*node_a.as_ptr()).right = (*node_b.as_ptr()).left;
                (*node_b.as_ptr()).left = Some(node_a);
                (*node_a.as_ptr()).parent = Some(node_b);
                 
                if node_a_from_left{
                    (*parent.as_ptr()).left = Some(node_b);
                }else{
                    (*parent.as_ptr()).right = Some(node_b);
                }

                // NEXT
                self.remove_black_leaf_with_black_deficiency(node_b);
            }else{
                 // node_a is root
                let _ = std::mem::replace(&mut self.root,  Some(node_b));
                if let Some(node_b) = self.root{
                    (*node_b.as_ptr()).parent = None;
                    (*node_a.as_ptr()).right = (*node_b.as_ptr()).left;
                    (*node_b.as_ptr()).left = Some(node_a);
                    (*node_a.as_ptr()).parent = Some(node_b);
                }

            }
 
            return self.remove_leaf(node_x);
        }


        unsafe fn remove_black_leaf_with_black_deficiency(&mut self, node: NonNull<Node<T>>){

        }

        // https://www.youtube.com/watch?v=T70nn4EyTrs
        unsafe fn remove_node(&mut self, node: NonNull<Node<T>>) -> bool{ 
            match self.operation_remove(&node){
                OperationRemoveFourOptions::RedLeaf => {
                    /*
                       1.0.0

                        P
                        ||
                        X

                     */
                    println!("RedLeaf");
                    return self.remove_leaf(node); 
                },
                OperationRemoveFourOptions::BlackLeaf => {
                    println!("BlackLeaf::");
                    // 2
                    //unimplemented!("BlackLeaf")
                    match self.operation_remove_black_leaf(&node){
                        OperationRemoveBlackLeaf::LeftRedABlackBRedCleaf =>{
                            println!("2.1.1.1");
                            self.remove_black_leaf_2_1_1_1(node);
                        },
                        OperationRemoveBlackLeaf::RightRedABlackBRedCleaf => {
                            println!("2.1.1.2");
                            self.remove_black_leaf_2_1_1_2(node);
                        },

                        OperationRemoveBlackLeaf::LeftRedABlackBleaf => {
                            println!("2.1.2.1");
                            self.remove_black_leaf_2_1_2_1(node);
                        },
                        OperationRemoveBlackLeaf::RightRedABlackBleaf => {
                            println!("2.1.2.2");
                            self.remove_black_leaf_2_1_2_2(node);
                        },

                        OperationRemoveBlackLeaf::BlackARedBWithBlackChildrenRightHaveRedLeaf => {
                            println!("2.2.1");
                            self.remove_black_leaf_2_2_1(node);
                        },
                        OperationRemoveBlackLeaf::BlackARedBWithBlackChildrenLeaf => {
                            println!("2.2.2");
                            self.remove_black_leaf_2_2_2(node);
                        },
                        OperationRemoveBlackLeaf::BlackALeftBlackBRedDleaf=>{
                            println!("2.3.1.1");
                            self.remove_black_leaf_2_3_1_1(node);
                        },

                        OperationRemoveBlackLeaf::BlackARightBlackBRedDleaf=>{
                            println!("2.3.1.2");
                            self.remove_black_leaf_2_3_1_2(node);
                        },
                       OperationRemoveBlackLeaf::BlackALeftBlackBleaf=>{
                            println!("2.3.2.1");
                            self.remove_black_leaf_2_3_2_1(node);
                        },
                        OperationRemoveBlackLeaf::BlackARightBlackBleaf=>{
                            println!("2.3.2.2");
                            self.remove_black_leaf_2_3_2_2(node);
                        }, 
                        _ => {
                            unimplemented!("BlackLeaf")
                        }
                    }
                },
                OperationRemoveFourOptions::NodeWithChildren => {
                    /*
                      4.0.0

                           X
                         /   \
                        L     R
                         \    /
                          C  L
                            /
                           C
                        ... ...

                    */
                    println!("NodeWithChildren::");
                    // из левого мы можем искать максимальное, а из правого - минимальное
                     
                    // выбрать лучшую стратегию удаления из доступных
                    /*
                        - Ориентироваться на цвет (красные удалять мы уже умеем, и, как мы узнаем далее, они проще)
                        - Ориентироваться на "дальность" -- на какой глубине относительно удаляемой вершины находится кандидат.
                        - Ориентироваться на модуль, то есть брать ближайшую по значению вершину.
                    */
                    // Стратегия избежать черного листа
                    let (min_l_n,min_level) = self.find_min((*node.as_ref()).right.unwrap(),0); 
                    let (max_r_n,max_level) = self.find_max((*node.as_ref()).left.unwrap(),0);
                    if (*min_l_n.as_ref()).is_red {
                        println!("::1");
                        // 1
                        std::mem::swap(&mut (*min_l_n.as_ptr()).value, &mut (*node.as_ptr()).value);
                        return self.remove_leaf(min_l_n);
                    }else{
                        if let Some(red_left) = (*max_r_n.as_ptr()).left{
                            if (*red_left.as_ref()).is_red {
                                println!("::3");
                               // 3
                               std::mem::swap(&mut (*max_r_n.as_ptr()).value, &mut (*node.as_ptr()).value);
                               std::mem::swap(&mut (*max_r_n.as_ptr()).value, &mut (*red_left.as_ptr()).value);
                               return self.remove_leaf(red_left);
                            }
                        }
                    }
                    // 2 Иначе черный лист
                    // Стратегия избежать изменнения высоты (в ветке min_l_n может быть крысный родитель)
                    // сперва проверить красного родителя он может быть только у min_l_n
                    match self.operation_remove_black_leaf(&min_l_n){
                        OperationRemoveBlackLeaf::LeftRedABlackBRedCleaf =>{
                            println!("RedABlackBRedCleaf");
                            self.remove_black_leaf_2_1_1_1(min_l_n);
                        },
                        OperationRemoveBlackLeaf::LeftRedABlackBleaf => {
                            self.remove_black_leaf_2_1_2_1(min_l_n);
                        },
                        _ => {
                            unimplemented!("BlackLeaf")
                        }
                    }

                    unimplemented!("BlackLeaf")
                      
                },
                OperationRemoveFourOptions::BlackNodeWithRedLeaf => {
                    /*
                     3.0.0
                       
                        X       
                       //   
                       A

                    */
                    println!("BlackNodeWithRedLeaf");
                    let red_left = (*node.as_ptr()).left.unwrap();
                    std::mem::swap(&mut (*node.as_ptr()).value, &mut (*red_left.as_ptr()).value);
                    return self.remove_leaf(red_left);
                },
                OperationRemoveFourOptions::Unimplemented =>{
                    panic!();
                }
            }
            
            false
        }

        unsafe fn find_max(&self, node: NonNull<Node<T>>,level: usize) -> (NonNull<Node<T>>,usize){
            if (*node.as_ref()).right.is_some(){
                self.find_max((*node.as_ref()).right.unwrap(),level+1)
            }else{
                (node,level)
            }
        }
      
        unsafe fn find_min(&self, node: NonNull<Node<T>>,level: usize) -> (NonNull<Node<T>>,usize){
            if (*node.as_ref()).left.is_some(){
                self.find_min((*node.as_ref()).left.unwrap(),level+1)
            }else{
                (node,level)
            }
        }

        unsafe fn remove_leaf(&mut self, node: NonNull<Node<T>>) -> bool{
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

        unsafe fn remove_tree(&mut self, node: NonNull<Node<T>>) {
            let left = (*node.as_ref()).left;
            let right = (*node.as_ref()).right;
            if left.is_none() && right.is_none() {
                if self.remove_leaf(node){
                    assert!(self.count > 0);
                    self.count -= 1;
                }
            } else if left.is_some() && right.is_none() {
                self.remove_tree(left.unwrap());
            } else if left.is_none() && right.is_some() {
                self.remove_tree(right.unwrap());
            } else {
                self.remove_tree(left.unwrap());
                self.remove_tree(right.unwrap());
            }
        }

        /// TODO: open http://www.webgraphviz.com/?tab=map 
        /// or https://dreampuf.github.io/GraphvizOnline/
        pub fn display(&self) -> String {
            if let Some(root) = self.root {
                return format!("\n\ndigraph Tree {{\n\tratio = fill;\n\tnode [style=filled fontcolor=\"white\"];\n{}}}",display_node(root));
            }
            "\nTree is empty".into()
        }

        // red-red violations, min black-height, max-black-height
        unsafe fn validate(
            &self,
            node: &Link<T>,
            is_red: bool,
            black_height: usize,
        ) -> (usize, usize, usize) {
            if let Some(n) = node {
                
                let red_red = if is_red && (*n.as_ref()).is_red {
                    1
                } else {
                    0
                };
                let black_height = black_height + match (*n.as_ref()).is_red {
                    false => 1,
                    _ => 0,
                };
                let l = self.validate(&(*n.as_ref()).left, (*n.as_ref()).is_red, black_height);
                let r = self.validate(&(*n.as_ref()).right, (*n.as_ref()).is_red, black_height);
                (red_red + l.0 + r.0, std::cmp::min(l.1, r.1), std::cmp::max(l.2, r.2))
            } else {
                (0, black_height, black_height)
            }
        }

        pub fn is_a_valid_red_black_tree(&self) -> bool {
            unsafe{
                let result = self.validate(&self.root, true, 0);
                let red_red = result.0;
                let black_height_min = result.1;
                let black_height_max = result.2;
                println!("Validation black height = {}",black_height_min);
                red_red == 0 && black_height_min == black_height_max                
            }
        }

        unsafe fn rotate_left_item(parent: NonNull<Node<T>>, node_a_from_left: bool) -> Link<T>{
            //println!("{}",self.display());
            let node_a = if node_a_from_left{
                (*parent.as_ptr()).left.unwrap()
            }else{
                (*parent.as_ptr()).right.unwrap()
            };
            println!("rotate left node_a={}",(*node_a.as_ptr()).value);
            let node_c = (*node_a.as_ptr()).right.unwrap();
                    
            (*node_c.as_ptr()).is_red = (*node_a.as_ptr()).is_red;// childNode принимает цвет своего parentNode
            (*node_a.as_ptr()).is_red = true;// цвет parentNode всегда определяется как красный

            (*node_c.as_ptr()).parent = Some(parent); 
            
            if (*node_c.as_ref()).left.is_some(){
                let mut node_e = (*node_c.as_ptr()).left;
                if let Some(ref mut e) = &mut node_e{
                    (*e.as_ptr()).parent = Some(node_a);
                }
                (*node_a.as_ptr()).right = node_e;
            }else{
                (*node_a.as_ptr()).right = None;
            } 
        
            (*node_c.as_ptr()).left = Some(node_a); 
             
            (*node_a.as_ptr()).parent = Some(node_c);
            /*if let Some(ref mut n_a) = (*node_c.as_mut()).left{
                (*n_a.as_mut()).parent = Some(node_c);
            }*/
            
            if node_a_from_left{
                (*parent.as_ptr()).left = Some(node_c);
            }else{
                (*parent.as_ptr()).right = Some(node_c);
                println!("rotate left parent={} node_c={}",(*parent.as_ptr()).value, (*node_c.as_ptr()).value);
            }

            //Tree::checking_connections(self.root);
            if node_a_from_left{
                return (*parent.as_ptr()).left;
            }else{
                return (*parent.as_ptr()).right;
            }
        }

        /*
            Rotate left

               A                  C
              / \\               //\
             B    C     =>      A   D
                 / \           / \
                E   D         B   E

        */
        unsafe fn rotate_left(mut node_a: NonNull<Node<T>>) -> Link<T>{
            if (*node_a.as_ref()).parent.is_some(){
                println!("rotate_left parent is_some");
                let parent  = (*node_a.as_ptr()).parent.unwrap();
                let mut node_a_from_left = false;
                if let Some(p_node_a) = (*parent.as_ptr()).left{
                    if std::ptr::eq(p_node_a.as_ptr(), node_a.as_ptr()) {
                        node_a_from_left = true;
                    } 
                } 
                return Tree::rotate_left_item( parent, node_a_from_left);
                 
            }else{
                println!("rotate_left parent is_none");
               
                let node_c = (*node_a.as_ptr()).right.unwrap();
                let mut new_node_c = Node::new_black((*node_c.as_ptr()).value.clone()).unwrap();
                let node_e = (*node_c.as_ptr()).left;
                (*new_node_c.as_ptr()).right = (*node_c.as_ptr()).right;
                if let Some(d) = (*new_node_c.as_ptr()).right{
                    (*d.as_ptr()).parent = Some(new_node_c);
                }

                let new_node_a = Node::new_black((*node_a.as_ptr()).value.clone()).unwrap();
                (*new_node_a.as_ptr()).is_red = (*node_a.as_ptr()).is_red;
                (*new_node_a.as_ptr()).left = (*node_a.as_ptr()).left;
                (*new_node_a.as_ptr()).right = (*node_c.as_ptr()).left;
                (*new_node_c.as_ptr()).is_red = (*new_node_a.as_ptr()).is_red;// childNode принимает цвет своего parentNode
                (*new_node_a.as_ptr()).is_red = true;// цвет parentNode всегда определяется как красный
                if let Some(b) = (*new_node_a.as_ptr()).left{
                    (*b.as_ptr()).parent = Some(new_node_a);
                }
                if let Some(e) = (*new_node_a.as_ptr()).right{
                    (*e.as_ptr()).parent = Some(new_node_a);
                }

                // ************************* root
                let _ = std::mem::replace(&mut node_a,   new_node_c);
                (*new_node_a.as_ptr()).parent = Some(node_a);
                (*node_a.as_ptr()).left = Some(new_node_a);
                (*node_a.as_ptr()).parent = None;
                
                

                #[cfg(not(feature = "test-rbt-snapshot"))]
                (*node_a.as_ptr()).drop();
                #[cfg(not(feature = "test-rbt-snapshot"))]
                (*node_c.as_ptr()).drop();

                return Some(node_a);
            }
        }

        pub unsafe fn checking_connections(node: Link<T>)->bool{
            if let Some(n) = node{
                if (*n.as_ptr()).parent.is_none(){
                    println!("\nROOT:[{}]", (*n.as_ptr()).value);
                }
                if (*n.as_ptr()).left.is_some() && (*n.as_ptr()).right.is_some(){
                    let left = (*n.as_ptr()).left.unwrap();
                    let right = (*n.as_ptr()).right.unwrap();
                    println!("[{}] <- [{}] -> [{}]",(*left.as_ptr()).value, (*n.as_ptr()).value,(*right.as_ptr()).value);

                    let p = (*left.as_ptr()).parent.unwrap();
                    assert_eq!((*n.as_ptr()).value, (*p.as_ptr()).value,"Нарушена связь с {:?} родителем слева",(*n.as_ptr()).value);
                    let p = (*right.as_ptr()).parent.unwrap();
                    assert_eq!((*n.as_ptr()).value, (*p.as_ptr()).value,"Нарушена связь с {:?} родителем справа",(*n.as_ptr()).value);
                }else if (*n.as_ptr()).left.is_some() && (*n.as_ptr()).right.is_none() {
                    if let Some(left) = (*n.as_ptr()).left{
                        println!("[{}] <- [{}] -> [NULL]",(*left.as_ptr()).value,(*n.as_ptr()).value);
                        let p = (*left.as_ptr()).parent.unwrap();
                        assert_eq!((*n.as_ptr()).value, (*p.as_ptr()).value,"Нарушена связь с {:?} родителем",(*n.as_ptr()).value);
                    }
                }else if (*n.as_ptr()).left.is_none() && (*n.as_ptr()).right.is_some() {
                    if let Some(right) = (*n.as_ptr()).right{
                        println!("[NULL] <- [{}] -> [{}]",(*n.as_ptr()).value,(*right.as_ptr()).value);
                        let p = (*right.as_ptr()).parent.unwrap();
                        assert_eq!((*n.as_ptr()).value, (*p.as_ptr()).value,"Нарушена связь с {:?} родителем",(*n.as_ptr()).value);
                    }
                }else {
                    println!("[{}]",(*n.as_ptr()).value); 
                }
                Tree::checking_connections((*n.as_ptr()).left);
                Tree::checking_connections((*n.as_ptr()).right);
            }
            true
        }

        unsafe fn rotate_right_item(parent: NonNull<Node<T>>, node_a_from_left: bool) -> Link<T>{
            let node_a = if node_a_from_left{
                (*parent.as_ptr()).left.unwrap()
            }else{
                (*parent.as_ptr()).right.unwrap()
            };
 
            let node_b = (*node_a.as_ptr()).left.unwrap();
             
            (*node_b.as_ptr()).parent = Some(parent); 
             
            (*node_b.as_ptr()).is_red = (*node_a.as_ptr()).is_red;// childNode принимает цвет своего parentNode
            (*node_a.as_ptr()).is_red = true;// цвет parentNode всегда определяется как красный
            
            if (*node_b.as_ref()).right.is_some(){
                let mut node_d = (*node_b.as_ptr()).right;
                if let Some(ref mut d) = &mut node_d{
                    (*d.as_ptr()).parent = Some(node_a);
                }
                (*node_a.as_ptr()).left = node_d;
                
            }else{
                (*node_a.as_ptr()).left = None;
            } 
            (*node_b.as_ptr()).right = Some(node_a); 
                 
             
            (*node_a.as_ptr()).parent = Some(node_b);
            /*if let Some(ref mut n_a) = (*node_b.as_mut()).right{
                (*n_a.as_ptr()).parent = Some(node_b);
            }*/
           
            if node_a_from_left{
                (*parent.as_ptr()).left = Some(node_b);
                return (*parent.as_ptr()).left;
            }else{
                (*parent.as_ptr()).right = Some(node_b);
                return (*parent.as_ptr()).right;
            }
        }

        /*
            Rotate right

                 A               B     
               // \             / \\   
               B   C     =>    E    A
             // \                  / \
            E    D                D   C

        */
        unsafe fn rotate_right(mut node_a: NonNull<Node<T>>) -> Link<T> {
            if (*node_a.as_ptr()).parent.is_some(){
                let parent = (*node_a.as_ptr()).parent.unwrap();
                //println!("rotate_right parent is_some node_a={}",(*node_a.as_ref()).value);
                let mut node_a_from_left = false;
                if let Some(p_node_a) = (*parent.as_ptr()).left{
                    if std::ptr::eq(p_node_a.as_ptr(), node_a.as_ptr()) {
                        node_a_from_left = true;
                    } 
                } 
                return Tree::rotate_right_item( parent, node_a_from_left);    
            }else{
                //println!("rotate_right parent is_none node={}",(*node_a.as_ref()).value);
                let mut new_node_a = Node::new_black((*node_a.as_ptr()).value.clone());
                let mut node_b = (*node_a.as_ptr()).left.unwrap();

                if let Some(ref mut n_a) = new_node_a{
                    
                    (*n_a.as_mut()).right = (*node_a.as_ptr()).right;
                    (*n_a.as_mut()).is_red = true;
 
                    if (*node_b.as_ref()).right.is_some(){
                        let mut node_d = (*node_b.as_mut()).right.unwrap();
                        (*node_d.as_mut()).parent = Some(*n_a);
                        (*n_a.as_mut()).left = Some(node_d);
                    }else{
                        (*n_a.as_mut()).left = None;
                    }

                    let _ = std::mem::replace(&mut node_a,   node_b);

                    //***************************** root
                        (*n_a.as_mut()).parent = Some(node_a);
                        (*node_a.as_mut()).right = Some(*n_a);
                        (*node_a.as_mut()).parent = None;
                        (*node_a.as_mut()).is_red = false;

                        return Some(node_a);
                    
                }
                None
            }  
             
        }
 
        unsafe fn flip_colors(mut node: NonNull<Node<T>>){
            //println!("FLIP COLORS node={}",(*node.as_ref()).value);
            if (*node.as_ref()).left.is_some() && (*node.as_ref()).right.is_some(){
                if let Some(ref mut left) = (*node.as_mut()).left{
                    (*left.as_mut()).is_red = false;
                }
                if let Some(ref mut right) = (*node.as_mut()).right{
                    (*right.as_mut()).is_red = false;
                }  
                (*node.as_mut()).is_red = true;  
                 
                if (*node.as_ptr()).parent.is_none(){ 
                    (*node.as_mut()).is_red = false;  
                }
            }

            if (*node.as_ptr()).parent.is_some(){
                Tree::check((*node.as_ptr()).parent.unwrap());  
            }
        }

        unsafe fn operation_put(node: NonNull<Node<T>>) -> OperationPut{
            if (*node.as_ref()).right.is_some(){
                let r = (*node.as_ref()).right.unwrap();
                if (*node.as_ref()).left.is_some(){
                    if !(*node.as_ref()).is_red {
                        let l = (*node.as_ref()).left.unwrap();
                        if (*l.as_ref()).is_red && (*r.as_ref()).is_red {
                            return OperationPut::FlipColors;
                        }
                    }
                } 
                if(*r.as_ref()).is_red {
                    return OperationPut::Left;
                }
            }

            if (*node.as_ref()).is_red && (*node.as_ref()).left.is_some(){
                let l = (*node.as_ref()).left.unwrap();
                if (*l.as_ref()).is_red{
                    return OperationPut::Right;
                }
            }
            return OperationPut::Skip;
        }

        unsafe fn put_node(parent: NonNull<Node<T>>, elem: T) -> bool {
              
            match elem.cmp(&(*parent.as_ref()).value) {
                Ordering::Equal =>{
                    return false;
                },
                Ordering::Less =>{
                    if (*parent.as_ptr()).left.is_some() {
                        return Tree::put_node((*parent.as_ptr()).left.unwrap(), elem);
                    } else {
                        println!("NEW Less={} < {}",elem, (*parent.as_ref()).value);
                        (*parent.as_ptr()).left = Node::new_red(elem, parent);
                        if (*parent.as_ref()).is_red && (*parent.as_ptr()).parent.is_some(){  
                            let node_a = (*parent.as_ptr()).parent.unwrap();
                            println!("ROTATE RIGHT");
                            if let Some(next) = Tree::rotate_right( node_a){
                                Tree::check(next);
                            } 
                        }
                        return true;  
                    }
                },
                Ordering::Greater => {
                    if (*parent.as_ptr()).right.is_some() {
                        return Tree::put_node((*parent.as_ptr()).right.unwrap(), elem);
                    } else {
                        println!("NEW Greater={} > {}",elem, (*parent.as_ref()).value);
                        (*parent.as_ptr()).right = Node::new_red(elem, parent);
 
                        if (*parent.as_ref()).left.is_some(){
                            let left = (*parent.as_ref()).left.unwrap();
                            if (*left.as_ref()).is_red{
                                Tree::flip_colors(parent);
                            }
                        }else{ 
                            println!("ROTATE LEFT");
                            if let Some(next) = Tree::rotate_left( parent){ 
                                Tree::check(next);
                            }                          
                        } 
                        return true; 
                    }
                }
            }
        }

        unsafe fn check(node: NonNull<Node<T>>){
            println!("check node={}",(*node.as_ptr()).value);  
            match Tree::operation_put(node){
                OperationPut::Left => {
                    println!("check rotate_left");
                    if let Some(next) = Tree::rotate_left(node){
                        Tree::check(next);
                    }     
                },
                OperationPut::Right => {  
                    println!("check rotate_right");
                    if (*node.as_ref()).parent.is_some(){
                        let node_a = (*node.as_ptr()).parent.unwrap();
                        if let Some(next) = Tree::rotate_right(node_a){
                            Tree::check(next);
                        } 
                    }
                },
                OperationPut::FlipColors => {
                    println!("check flip_colors");
                    Tree::flip_colors(node);

                    if let Some(parent) = (*node.as_ptr()).parent{
                        Tree::check(parent);
                    }                 
                },
                OperationPut::Skip => {}
            }
        }

        #[cfg(feature = "test-rbt-snapshot")]
        pub unsafe fn test_left_rotation_with_parent(&mut self){
            /*
                Test Left Rotation With Parent
                -- node C is RED ---

                    P(2)                     P(2)
                      \                        \
                     A(6)                     C(8)
                    /   \\                   //  \
                B(5)    C(8)      =>       A(6)   D(9)
                       /   \               /   \
                      E(7)  D(9)        B(5)   E(7)
            */
             
            // Initialization
            let mut tree: Tree<i32> = Tree::new();
            tree.root = Node::new_black(2);
            if let Some(ref mut root) = &mut tree.root{
              
                (*root.as_mut()).right = Node::new_black_with_parent(6, *root);// A
                if let Some(ref mut node_a) = &mut (*root.as_mut()).right{
                    (*node_a.as_mut()).left = Node::new_black_with_parent(5, *node_a);// B

                    (*node_a.as_mut()).right = Node::new_red(8, *node_a);// C

                    if let Some(ref mut node_c) = &mut (*node_a.as_mut()).right{
                        (*node_c.as_mut()).left = Node::new_black_with_parent(7, *node_c);// E
                        (*node_c.as_mut()).right = Node::new_black_with_parent(9, *node_c);// D
                    }
                }  
            }

            // Operation
            if let Some(root) = tree.root{
                if let Some(node_a) =  (*root.as_ptr()).right{
                    Tree::check(node_a);
                }
            }
            
            // Validation
            if let Some(ref mut root) = &mut tree.root{
                {// C
                    let node_c = (*root.as_ptr()).right.unwrap();
                    assert_eq!((*node_c.as_ptr()).value, 8);
                    assert_eq!((*node_c.as_ptr()).is_red, false);
                    let node_c_parent = (*node_c.as_ptr()).parent.unwrap();
                    assert_eq!((*node_c_parent.as_ptr()).value, 2);

                    let node_a = (*node_c.as_ptr()).left.unwrap();
                    assert_eq!((*node_a.as_ptr()).is_red, true);
                    assert_eq!((*node_a.as_ptr()).value, 6);
                    let node_a_parent = (*node_a.as_ptr()).parent.unwrap();
                    assert_eq!((*node_a_parent.as_ptr()).value, 8);

                    let node_d = (*node_c.as_ptr()).right.unwrap();
                    assert_eq!((*node_d.as_ptr()).value, 9);
                    let node_d_parent = (*node_d.as_ptr()).parent.unwrap();
                    assert_eq!((*node_d_parent.as_ptr()).value, 8);
                }
                {// A
                    let node_c = (*root.as_ptr()).right.unwrap();
                    let node_a = (*node_c.as_ptr()).left.unwrap();
                    let node_b = (*node_a.as_ptr()).left.unwrap();
                    let node_e = (*node_a.as_ptr()).right.unwrap();
                    assert_eq!((*node_b.as_ptr()).value, 5,">>>7");
                    assert_eq!((*node_e.as_ptr()).value, 7,">>>8");
                    let node_b_parent = (*node_b.as_ptr()).parent.unwrap();
                    assert_eq!((*node_b_parent.as_ptr()).value, 6,">>>9");
                    let node_e_parent = (*node_e.as_ptr()).parent.unwrap();
                    assert_eq!((*node_e_parent.as_ptr()).value, 6,">>>10");
                }
            }

            tree.root = None;
        }
   
        #[cfg(feature = "test-rbt-snapshot")]
        pub unsafe fn test_left_rotation_without_parent(&mut self) {
            /*
                Test Left Rotation Without Parent
                -- node A is ROOT ---
                -- node C is RED ---                
                                               
                       A(6)                     C(8)
                      /   \\                   //   \
                   B(5)    C(8)      =>       A(6)  D(9)
                          /    \             /   \
                        E(7)   D(9)        B(5)   E(7)
            */

            // Initialization
            let mut tree: Tree<i32> = Tree::new();
            tree.root = Node::new_black(6);
            if let Some(ref mut root) = &mut tree.root{
                (*root.as_mut()).left = Node::new_black_with_parent(5, *root);// B
                (*root.as_mut()).right = Node::new_red(8, *root);// C
                    
                if let Some(ref mut node_c) = &mut (*root.as_mut()).right{
                    (*node_c.as_mut()).left = Node::new_black_with_parent(7, *node_c);// E
                    (*node_c.as_mut()).right = Node::new_black_with_parent(9, *node_c);// D
                }  
            }

            // Operation
            Tree::check( tree.root.unwrap());  
            
            // Validation
            if let Some(ref mut root) = &mut tree.root{
                {
                    assert_eq!((*root.as_ptr()).is_red, false);
                    assert_eq!((*root.as_ptr()).parent, None);
                    assert_eq!((*root.as_ptr()).value, 8);
                    let node_d = (*root.as_ptr()).right.unwrap();
                    assert_eq!((*node_d.as_ptr()).value, 9);
                    let node_d_parent = (*node_d.as_ptr()).parent.unwrap();
                    assert_eq!((*node_d_parent.as_ptr()).value, 8);
    
                    let node_a = (*root.as_ptr()).left.unwrap();
                    assert_eq!((*node_a.as_ptr()).is_red, true);  
                    assert_eq!((*node_a.as_ptr()).value, 6);  
                    let node_a_parent = (*node_a.as_ptr()).parent.unwrap();
                    assert_eq!((*node_a_parent.as_ptr()).value, 8);              
                }
                {// A
                    let node_a = (*root.as_ptr()).left.unwrap();
                    let node_b = (*node_a.as_ptr()).left.unwrap();
                    let node_e = (*node_a.as_ptr()).right.unwrap();
                    assert_eq!((*node_b.as_ptr()).value, 5);
                    assert_eq!((*node_e.as_ptr()).value, 7);
                    let node_b_parent = (*node_b.as_ptr()).parent.unwrap();
                    assert_eq!((*node_b_parent.as_ptr()).value, 6);  
                    let node_e_parent = (*node_e.as_ptr()).parent.unwrap();
                    assert_eq!((*node_e_parent.as_ptr()).value, 6);
                }
            }
            tree.root = None;
        }
        
                
        #[cfg(feature = "test-rbt-snapshot")]
        pub unsafe fn test_right_rotation_with_parent_2(&mut self){
            /*
                Test Right Rotation With Parent
                -- node E and B is RED ---
             
                         575                         396 
                        //   \                    /       \
                      396    792              B(73)        575                      
                     /    \                   /  \        /   \
                  A(139)  546               E(7) A(139)  546  792
                   //   \                        
                B(73)   C(30)    =>           
                //                                 
             E(7)                                 
            
            */
             
            // Initialization
            let mut tree = Tree::new();
            tree.root = Node::new_black(575);// P
            if let Some(ref mut root) = &mut tree.root{
                (*root.as_mut()).left = Node::new_red(396, *root); 
                //(*root.as_mut()).right = Node::new_black_with_parent(792, *root); 
                
                if let Some(ref mut node) = &mut (*root.as_mut()).left{
                    //(*node.as_mut()).left = Node::new_black_with_parent(139, *node);// A
                    //(*node.as_mut()).right = Node::new_black_with_parent(546, *node);// C
    
                    if let Some(ref mut node_a) = &mut (*node.as_mut()).left{
                        //(*node_a.as_mut()).left = Node::new_red(73, *node_a);// B 
                        if let Some(ref mut node_b) = &mut (*node_a.as_mut()).left{
                             //(*node_b.as_mut()).left = Node::new_red(7, *node_b);// E
                        }
                    }
                }  
            }
            
            // Operation
            /*if let Some(ref mut root) = &mut tree.root{
                if let Some(ref mut node) = &mut (*root.as_mut()).left{
                    if let Some(ref mut node_a) = &mut (*node.as_mut()).left{
                        
                        if let Some(ref mut node) = tree.rotate_right(*node_a){
                            tree.check(node);
                        }
                    } 
                }
            }*/

            tree.put(575);
            tree.put(396);
            tree.put(139);
            tree.put(792);
            tree.put(546);
            tree.put(73);
            //println!(">>>>{}",tree.display());
            tree.put(7);
            unsafe{ Tree::checking_connections(tree.root);} 

            println!("{}",tree.display());
            // Validation
            /*if let Some(ref mut root) = &mut tree.root{
              
            }*/
            tree.root = None;
        }
           

        #[cfg(feature = "test-rbt-snapshot")]
        pub unsafe fn test_right_rotation_with_parent(&mut self){
            /*
                Test Right Rotation With Parent
                -- node E and B is RED ---
            
                 P(5)                      P(5)                           
                     \                       \\
                    A(26)                    B(24)
                   //   \                   /    \
                B(24)   C(30)    =>      E(15)   A(26)
                //   \                          /   \
            E(15)   D(25)                    D(25)  C(30)
            
            */
             
            // Initialization
            let mut tree = Tree::new();
            tree.root = Node::new_black(5);// P
            if let Some(ref mut root) = &mut tree.root{
                (*root.as_mut()).right = Node::new_black_with_parent(26, *root);// A
                
                if let Some(ref mut node_a) = &mut (*root.as_mut()).right{
                    (*node_a.as_mut()).left = Node::new_red(24, *node_a);// B
                    (*node_a.as_mut()).right = Node::new_black_with_parent(30, *node_a);// C
    
                    if let Some(ref mut node_b) = &mut (*node_a.as_mut()).left{
                        (*node_b.as_mut()).left = Node::new_red(15, *node_b);// E 
                        (*node_b.as_mut()).right = Node::new_black_with_parent(25, *node_b);// D
                    }
                }  
            }
            
            // Operation
            if let Some(ref mut root) = &mut tree.root{
                if let Some(ref mut node_a) = &mut (*root.as_mut()).right{
                    Tree::rotate_right( *node_a);
                }
            }
             
            // Validation
            if let Some(ref mut root) = &mut tree.root{
                { // B A ----------------------------------------------------------
                    let node_b = (*root.as_ptr()).right.unwrap();
                    //assert_eq!((*node_b.as_ptr()).is_red, true);
                    assert_eq!((*node_b.as_ptr()).value, 24);
                    let parent = (*node_b.as_ptr()).parent.unwrap(); 
                    assert_eq!((*parent.as_ptr()).value, 5);  
                    let node_e = (*node_b.as_ptr()).left.unwrap(); 
                    assert_eq!((*node_e.as_ptr()).value, 15); 
                    
                    let node_e_parent = (*node_e.as_ptr()).parent.unwrap(); 
                    assert_eq!((*node_e_parent.as_ptr()).value, 24); 
                    let node_a = (*node_b.as_ptr()).right.unwrap();
                    //assert_eq!((*node_a.as_ptr()).is_red, false); 
                    assert_eq!((*node_a.as_ptr()).value, 26); 
                    let node_a_parent = (*node_a.as_ptr()).parent.unwrap(); 
                    assert_eq!((*node_a_parent.as_ptr()).value, 24); 
                }
                {//D C ----------------------------------------------------------
                    let node_b = (*root.as_ptr()).right.unwrap();
                    let node_a = (*node_b.as_ptr()).right.unwrap();
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
            tree.root = None;
        }
           
        #[cfg(feature = "test-rbt-snapshot")]
        pub unsafe fn test_right_rotation_without_parent(&mut self){
            /*
                Test Right Rotation Without Parent
                -- node A is ROOT ---
                -- node E and B is RED ---

                                                                           
                    A(26)                   B(24)
                   //    \                  /    \
                 B(24)   C(30)   =>     E(15)   A(26)
                //   \                         /    \
            E(15)   D(25)                   D(25)  C(30)
            
            */
            
            // Initialization
            let mut tree  = Tree::new();
            tree.root = Node::new_black(26);// A
            if let Some(ref mut root) = &mut tree.root{
                (*root.as_mut()).left = Node::new_red(24, *root);// B
                (*root.as_mut()).right = Node::new_black_with_parent(30, *root);// C 
                
                if let Some(ref mut node_b) = &mut (*root.as_mut()).left{
                    (*node_b.as_mut()).left = Node::new_red(15, *node_b);// E
                    (*node_b.as_mut()).right = Node::new_black_with_parent(25, *node_b);// D
                }  
            }
             
            // Operation
            if let Some(root) = tree.root{
                if let Some(parent) = (*root.as_ptr()).left{
                    if let Some(node_a) = (*parent.as_ptr()).parent{
                        if let Some(n) = Tree::rotate_right(node_a){
                            Tree::flip_colors(n);
                        }
                    }
                }                
            }
            // self.rotate_right(&mut tree.root.unwrap());
             
            // Validation
            if let Some(ref mut root_b) = &mut tree.root{
                {
                    assert_eq!((*root_b.as_ptr()).parent, None);
                    assert_eq!((*root_b.as_ptr()).is_red, false);
                    assert_eq!((*root_b.as_ptr()).value, 24);
                    let node_e = (*root_b.as_ptr()).left.unwrap();
                    assert_eq!((*node_e.as_ptr()).value, 15);
                    assert_eq!((*node_e.as_ptr()).is_red, false);
                    let node_e_parent = (*node_e.as_ptr()).parent.unwrap();
                    assert_eq!((*node_e_parent.as_ptr()).value, 24);  
    
                    let node_a = (*root_b.as_ptr()).right.unwrap();
                    assert_eq!((*node_a.as_ptr()).is_red, false);
                    assert_eq!((*node_a.as_ptr()).value, 26);
                    let node_a_parent = (*node_a.as_ptr()).parent.unwrap();
                    assert_eq!((*node_a_parent.as_ptr()).value, 24);
                } 
                { // A ----------------------------------------------------------
                    let node_a = (*root_b.as_ptr()).right.unwrap();
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
            tree.root = None;
        }
          
        #[cfg(feature = "test-rbt-snapshot")]
        pub unsafe fn test_remove_black_2_2_2(&mut self){
            /*
              2.2.2 remove black X 

                  P            P
                  |            | 
                  A            B
                // \          / \
               B    X   =>   E   A
              / \               //
             E   C             C

            */           

            // Initialization
            let mut tree  = Tree::new();
            tree.root = Node::new_black(449);
            if let Some(ref mut root) = &mut tree.root{
                (*root.as_mut()).left = Node::new_black_with_parent(331, *root);// A
                (*root.as_mut()).right = Node::new_black_with_parent(755, *root);  
                
                if let Some(ref mut node_a) = &mut (*root.as_mut()).left{
                    (*node_a.as_mut()).left = Node::new_red(119, *node_a);// B 
                    (*node_a.as_mut()).right = Node::new_black_with_parent(382, *node_a);// X 

                    if let Some(ref mut node_b) = &mut (*node_a.as_mut()).left{
                        (*node_b.as_mut()).left = Node::new_black_with_parent(118, *node_b);// E 
                        (*node_b.as_mut()).right = Node::new_black_with_parent(328, *node_b);// C
                    } 
                } 
                if let Some(ref mut node) = &mut (*root.as_mut()).right{
                    (*node.as_mut()).left = Node::new_black_with_parent(495, *node); 
                    (*node.as_mut()).right = Node::new_black_with_parent(850, *node); 
                }
            }
             
            // Operation
            tree.remove(382);

            
            // println!("{}",tree.display());
            
            // Validation
            if let Some(ref mut root) = &mut tree.root{
                assert_eq!((*root.as_ptr()).parent, None);
                assert_eq!((*root.as_ptr()).is_red, false);
                assert_eq!((*root.as_ptr()).value, 449);

                // B
                let node_b = (*root.as_ptr()).left.unwrap();
                assert_eq!((*node_b.as_ptr()).value, 119);
                assert_eq!((*node_b.as_ptr()).is_red, false);

                let node_b_parent = (*node_b.as_ptr()).parent.unwrap();
                assert_eq!((*node_b_parent.as_ptr()).value, 449); 
                // E
                let node_e = (*node_b.as_ptr()).left.unwrap();
                assert_eq!((*node_e.as_ptr()).is_red, false);
                assert_eq!((*node_e.as_ptr()).value, 118);
                let node_e_parent = (*node_e.as_ptr()).parent.unwrap();
                assert_eq!((*node_e_parent.as_ptr()).value, 119); 
                // A
                let node_a = (*node_b.as_ptr()).right.unwrap();
                assert_eq!((*node_a.as_ptr()).is_red, false);
                assert_eq!((*node_a.as_ptr()).value, 331);
                let node_a_parent = (*node_a.as_ptr()).parent.unwrap();
                assert_eq!((*node_a_parent.as_ptr()).value, 119);
                // C
                let node_c = (*node_a.as_ptr()).left.unwrap();
                assert_eq!((*node_c.as_ptr()).value, 328);
                assert_eq!((*node_c.as_ptr()).is_red, true);
                let node_c_parent = (*node_c.as_ptr()).parent.unwrap();
                assert_eq!((*node_c_parent.as_ptr()).value, 331);
            }

            assert!(tree.is_a_valid_red_black_tree());
            tree.root = None;
        }

        #[cfg(feature = "test-rbt-snapshot")]
        pub unsafe fn test_remove_black_2_2_1(&mut self){
            /*
              2.2.1 remove black X 

                 P            P
                 |            | 
                 A            B
               // \            \
              B    X   =>       A
               \               //
                C             C
               //
              D 
            */ 

            // Initialization
            let mut tree  = Tree::new();
            tree.root = Node::new_black(486);
            if let Some(ref mut root) = &mut tree.root{
                (*root.as_mut()).left = Node::new_black_with_parent(324, *root);// A
                (*root.as_mut()).right = Node::new_black_with_parent(612, *root);  
                
                if let Some(ref mut node_a) = &mut (*root.as_mut()).left{
                    (*node_a.as_mut()).left = Node::new_red(226, *node_a);// B 
                    (*node_a.as_mut()).right = Node::new_black_with_parent(479, *node_a);// X 

                    if let Some(ref mut node_b) = &mut (*node_a.as_mut()).left{
                         
                        (*node_b.as_mut()).right = Node::new_black_with_parent(290, *node_b);// C
                        if let Some(ref mut node_c) = &mut (*node_b.as_mut()).right{
                            (*node_c.as_mut()).left = Node::new_red(280, *node_a);// D 
                        }
                    } 
                }  
            }
             
            // Operation
            tree.remove(479);

            // Validation
            if let Some(ref mut root) = &mut tree.root{
                assert_eq!((*root.as_ptr()).parent, None);
                assert_eq!((*root.as_ptr()).is_red, false);
                assert_eq!((*root.as_ptr()).value, 486);
                // C
                let node_c = (*root.as_ptr()).left.unwrap();
                assert_eq!((*node_c.as_ptr()).is_red, false);
                assert_eq!((*node_c.as_ptr()).value, 290);
                let node_c_parent = (*node_c.as_ptr()).parent.unwrap();
                assert_eq!((*node_c_parent.as_ptr()).value, 486); 

                // A
                let node_a = (*node_c.as_ptr()).right.unwrap();
                assert_eq!((*node_a.as_ptr()).is_red, false);
                assert_eq!((*node_a.as_ptr()).value, 324);
                let node_a_parent = (*node_a.as_ptr()).parent.unwrap();
                assert_eq!((*node_a_parent.as_ptr()).value, 290); 
                assert_eq!((*node_a.as_ptr()).left, None);
                assert_eq!((*node_a.as_ptr()).right, None);
                // B
                let node_b = (*node_c.as_ptr()).left.unwrap();
                assert_eq!((*node_b.as_ptr()).is_red, true);
                assert_eq!((*node_b.as_ptr()).value, 226);
                let node_b_parent = (*node_b.as_ptr()).parent.unwrap();
                assert_eq!((*node_b_parent.as_ptr()).value, 290); 

                // D
                let node_d = (*node_b.as_ptr()).right.unwrap();
                assert_eq!((*node_d.as_ptr()).is_red, false);
                assert_eq!((*node_d.as_ptr()).value, 280);
                assert_eq!((*node_d.as_ptr()).left, None);
                let node_d_parent = (*node_d.as_ptr()).parent.unwrap();
                assert_eq!((*node_d_parent.as_ptr()).value, 226); 
            }

            tree.root = None;
 
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
        // Изначально узел RED.Объясняется это тем, что мы всегда сначала добавляем значение к уже существующей ноде 
        // и только после этого занимаемся балансировкой
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

        #[cfg(feature = "test-rbt-snapshot")]
        pub fn new_black_with_parent(value: T, parent: NonNull<Node<T>>) -> Link<T> {
            unsafe {
                let new = NonNull::new_unchecked(Box::into_raw(Box::new(Self {
                    left: None,
                    right: None,
                    parent: Some(parent),
                    is_red: false,
                    value,
                })));
                Some(new)
            }
        }
    }

    #[cfg(not(feature = "test-rbt-snapshot"))]
    impl<T: Ord + PartialEq + PartialOrd + Display + Clone> Drop for Tree<T> {
        fn drop(&mut self) {
            unsafe {
                self.remove_tree(self.root.unwrap());
            }
        }
    }

    #[cfg(not(feature = "test-rbt-snapshot"))]
    impl<T: Display> Drop for Node<T> {
        fn drop(&mut self) {
            println!("Drop Node={}", self.value);
        }
    }

    // Находит данные в поддереве `fromnode`.
    fn find_node<T: Ord + PartialEq + PartialOrd + Display>(
        fromnode: Link<T>,
        value: T,
    ) -> Link<T> {
        unsafe {
            if let Some(fromnode) = fromnode {
                match value.cmp(&(*fromnode.as_ptr()).value) {
                    Ordering::Equal =>{
                        Some(fromnode)
                    },
                    Ordering::Less =>{
                        find_node((*fromnode.as_ptr()).left, value)
                    },
                    Ordering::Greater => {
                        find_node((*fromnode.as_ptr()).right, value)
                    }
                } 
            } else {
                fromnode
            }
        }
    }
  
    fn display_node<T: Display>(node: NonNull<Node<T>>) -> String {
        unsafe {
            let mut s: String = "".into();
            let color = if (*node.as_ptr()).is_red{"[color=\"red\"]"}else{"[color=\"black\"]"};

            if let Some(left) = (*node.as_ptr()).left {
                s.push_str(&format!("\t{n1}->{n2} {color1}; {n1} {color1}; {n2} {color2};\n", 
                n1=(*node.as_ptr()).value,
                n2=(*left.as_ptr()).value,
                color1=color,
                color2=if (*left.as_ptr()).is_red{"[color=\"red\"]"}else{"[color=\"black\"]"},
                ));
                s.push_str(&display_node(left));
            } else if (*node.as_ptr()).right.is_some(){
                s.push_str(&format!("\t{n1}->node_null_{n1} [color=\"grey\"]; {n1} {color1};\n",n1=(*node.as_ptr()).value,color1=color));
                s.push_str(&format!("\tnode_null_{n1}[label=\"null\"]\n",n1=(*node.as_ptr()).value));
            } 

            if let Some(right) = (*node.as_ptr()).right {
                s.push_str(&format!("\t{n1}->{n2} {color1}; {n1} {color1}; {n2} {color2};\n", 
                n1=(*node.as_ptr()).value,
                n2=(*right.as_ptr()).value,
                color2=if (*right.as_ptr()).is_red{"[color=\"red\"]"}else{"[color=\"black\"]"}, 
                color1=color));
                s.push_str(&display_node(right));
            }else{
                s.push_str(&format!("\t{n1}->node_null_{n1} [color=\"grey\"]; {n1} {color1};\n",n1=(*node.as_ptr()).value,color1=color));
                s.push_str(&format!("\tnode_null_{}[label=\"null\"]\n",(*node.as_ptr()).value));
            }
            s
        }
    }

}


/// $ cargo test red_black_tree_nonnull -- --nocapture
/// $ cargo test red_black_tree_nonnull --features test-rbt-snapshot -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_success -- --nocapture
    #[test]
    fn test_success() {
     
        let mut tree: Tree<i32> = Tree::new();
        let nodes = vec![ 24,5 ,1  ,15   ,3 ,8     ,13,16];// [24, 5, 6] и [24, 5, 1]
        let nodes = vec![480,978,379,784,398,71,695,23,97,309,312,449,958,992,220,95,257,869,959,450,258,315,783,731,914,
        880,984,734,570,801,908,181,466,238,916,77,801,867,382,943,603,65,545,200,759,158,987,821,630,537,
        704,149,617,498,261,160,192,760,417,939,757,858,376,885,336,764,443,155,983,586,957,375,
        893,707,255,811,86,370,384,177,834,177,834,313,209,623,176,875,748,949,529,932,369,385,
        419,222,719,342,68,156,314,343,262,467,499,604,732,758,765,812,859,876];
        //let nodes = 0..=44;
        for i in nodes{ tree.put(i);}
         
        println!("After Test:\n{}",tree.display());
        unsafe{ Tree::checking_connections(tree.root);} 
        assert!(true);
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_rotate_right_success -- --nocapture
    #[test]
    fn test_rotate_right_success() {
       let mut tree: Tree<i32> = Tree::new();
       let nodes = vec![575,396,139,792,546,73,7 ];
       for i in nodes{ tree.put(i);}
        
       println!("After Test:\n{}",tree.display());
       unsafe{ Tree::checking_connections(tree.root);} 
       assert!(tree.is_a_valid_red_black_tree());
    }

    #[test]
    fn test_find_success() {
        let mut tree = Tree::new();
        let nodes = 0..=28;
        for i in nodes{tree.put(i);}
        assert!(tree.contains(4));
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_snapshot_success --features test-rbt-snapshot -- --nocapture
    #[cfg(feature = "test-rbt-snapshot")]
    #[test]
    fn test_snapshot_success() {
        unsafe{
            let mut tree: Tree<i32> = Tree::new();
            tree.test_left_rotation_with_parent();

            tree.test_left_rotation_without_parent();

            tree.test_right_rotation_with_parent();

            tree.test_right_rotation_without_parent();

            tree.test_remove_black_2_2_2();

            tree.test_remove_black_2_2_1();

            tree.test_right_rotation_with_parent_2();
        }
    }
    
    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_validate_success -- --nocapture
    #[test]
    fn test_validate_success() {
        let mut tree = Tree::new();
        let nodes = 0..=28;
        for i in nodes{tree.put(i);}
        assert!(tree.is_a_valid_red_black_tree());
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_tree_success -- --nocapture
    #[test]
    fn test_remove_tree_success() {
        let mut tree = Tree::new();
        let nodes = 0..=1;
        for i in nodes{tree.put(i);}
        // Check impl Drop
        assert!(true);  
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_1_0_0_success -- --nocapture
    #[test]
    fn test_remove_1_0_0_success() {
        let mut tree = Tree::new();
        let nodes = 0..=27;
        for i in nodes{tree.put(i);}
        assert!(tree.is_a_valid_red_black_tree());
        tree.remove(26);
        assert!(tree.is_a_valid_red_black_tree());
    }
 
    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_3_0_0_success -- --nocapture
    #[test]
    fn test_remove_3_0_0_success() {
        let mut tree = Tree::new();
        let nodes = 0..=3;
        for i in nodes{tree.put(i);}
        assert!(tree.is_a_valid_red_black_tree());
        tree.remove(3);
        assert!(tree.is_a_valid_red_black_tree());
    }
  
    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_node_4_0_0_to_3_0_0_success -- --nocapture
    #[test]
    fn test_remove_node_4_0_0_to_3_0_0_success() {
        let mut tree = Tree::new();
        let nodes = 0..=9;
        for i in nodes{tree.put(i);}
        assert!(tree.is_a_valid_red_black_tree());
        tree.remove(7);
        assert!(tree.is_a_valid_red_black_tree());
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_node_4_0_0_to_1_0_0_success -- --nocapture
    #[test]
    fn test_remove_node_4_0_0_to_1_0_0_success() {
        let mut tree = Tree::new();
        let nodes = vec![315,897,267,995,843,520];
        for i in nodes{tree.put(i);}
        assert!(tree.is_a_valid_red_black_tree());
        tree.remove(897);
        assert!(tree.is_a_valid_red_black_tree());
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_black_2_1_1_1_success  -- --nocapture
    #[test]
    fn test_remove_black_2_1_1_1_success() {//2.1.1.1
        let mut tree = Tree::new();
        let nodes = vec![314,147,119,331,755,449,118];
        for i in nodes{tree.put(i);}
        assert!(tree.is_a_valid_red_black_tree());
        tree.remove(314);
        assert!(tree.is_a_valid_red_black_tree());
        tree.remove(147);
        assert!(tree.is_a_valid_red_black_tree());
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_black_2_1_1_2_success  -- --nocapture
    #[test]
    fn test_remove_black_2_1_1_2_success() {//2.1.1.2
        let mut tree = Tree::new();
        let nodes = vec![231,511,914,699,532,531];
        for i in nodes{tree.put(i);}
        assert!(tree.is_a_valid_red_black_tree());
        println!("{}",tree.display());
        tree.remove(231);
        println!("{}",tree.display());
        assert!(tree.is_a_valid_red_black_tree());
    }
 
    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_black_2_1_2_1_success  -- --nocapture
    #[test]
    fn test_remove_black_2_1_2_1_success() {//2.1.2.1
        let mut tree = Tree::new();
        let nodes = vec![438,440,260,530,34,355];
        for i in nodes{tree.put(i);}
        assert!(tree.is_a_valid_red_black_tree());
        tree.remove(355);
        assert!(tree.is_a_valid_red_black_tree());
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_black_2_1_2_2_success  -- --nocapture
    #[test]
    fn test_remove_black_2_1_2_2_success() {//2.1.2.2
        let mut tree = Tree::new();
        let nodes = vec![231,511,914,699,532];
        for i in nodes{tree.put(i);}
        assert!(tree.is_a_valid_red_black_tree());
        println!("{}",tree.display());
        tree.remove(231);
        println!("{}",tree.display());
        assert!(tree.is_a_valid_red_black_tree());

    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_black_2_2_1_node_a_root_success -- --nocapture
    #[test]
    fn test_remove_black_2_2_1_node_a_root_success() { 
      // https://youtu.be/T70nn4EyTrs?t=4320
      let mut tree = Tree::new();
      let nodes = vec![315,897,267,995,843,520];
      for i in nodes{tree.put(i);}
      assert!(tree.is_a_valid_red_black_tree());
      tree.remove(995);
      assert!(tree.is_a_valid_red_black_tree());
    } 

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_black_2_2_1_success -- --nocapture
    #[test]
    fn test_remove_black_2_2_1_success() { 
        // https://youtu.be/T70nn4EyTrs?t=4320
        let mut tree = Tree::new();
        let nodes = vec![486,226,612,121,479,69,559,990,290,324,280];
        for i in nodes{tree.put(i);}
        assert!(tree.is_a_valid_red_black_tree());
        tree.remove(479);
        assert!(!tree.contains(479));
        assert!(tree.is_a_valid_red_black_tree());  
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_black_2_2_2_success -- --nocapture
    #[test]
    fn test_remove_black_2_2_2_success() { 
        let mut tree = Tree::new();
        let nodes: Vec<i32> = vec![119,331,755,449,118,850,495,382,328];
        for i in nodes{tree.put(i);}
        assert!(tree.is_a_valid_red_black_tree());
        tree.remove(382);
        assert!(tree.is_a_valid_red_black_tree()); 
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_black_2_2_2_root_success -- --nocapture
    #[test]
    fn test_remove_black_2_2_2_root_success() { 
        let mut tree = Tree::new();
        let nodes: Vec<i32> = vec![106,734,951,753,205,730];
        for i in nodes{tree.put(i);}
        tree.remove(753);
        assert!(tree.is_a_valid_red_black_tree());
        tree.remove(951);
        assert!(tree.is_a_valid_red_black_tree());
    }
 
    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_black_2_3_1_1_success -- --nocapture
    #[test]
    fn test_remove_black_2_3_1_1_success() { 
        let mut tree = Tree::new();
        let nodes: Vec<i32> = vec![575,396,139,792,546,73,7,6];
        for i in nodes{tree.put(i);}
        assert!(tree.is_a_valid_red_black_tree());
        tree.remove(139);
        assert!(tree.is_a_valid_red_black_tree()); 
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_black_2_3_1_1_root_success -- --nocapture
    #[test]
    fn test_remove_black_2_3_1_1_root_success() { 
        let mut tree = Tree::new();
        let nodes: Vec<i32> = vec![106,107,108,105];
        for i in nodes{tree.put(i);}
        assert!(tree.is_a_valid_red_black_tree()); 
        tree.remove(108);
        assert!(tree.is_a_valid_red_black_tree()); 
    }

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_black_2_3_1_2_success -- --nocapture
    #[test]
    fn test_remove_black_2_3_1_2_success() { 
        let mut tree = Tree::new();
        let nodes: Vec<i32> = vec![575,396,139,792,546,73,7,138];
        for i in nodes{tree.put(i);}
        assert!(tree.is_a_valid_red_black_tree()); 

        tree.remove(7);

        assert!(tree.is_a_valid_red_black_tree()); 
    }
 
    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_black_2_3_1_2_root_success -- --nocapture
    #[test]
    fn test_remove_black_2_3_1_2_root_success() { 
        let mut tree = Tree::new();
        let nodes: Vec<i32> = vec![106,107,109,108 ];
        for i in nodes{tree.put(i);}
        assert!(tree.is_a_valid_red_black_tree()); 
        tree.remove(106);
        assert!(tree.is_a_valid_red_black_tree()); 
    }
 
    // 2.3.2.1 
    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_black_2_3_2_1_success -- --nocapture
    #[test]
    fn test_remove_black_2_3_2_1_success() { 
        let mut tree = Tree::new();
        let nodes: Vec<i32> = vec![246,562,950,237,417,418,416];
        for i in nodes{tree.put(i);}
        assert!(tree.is_a_valid_red_black_tree()); 
        tree.remove(416);
        assert!(tree.is_a_valid_red_black_tree()); 
        //println!("{}",tree.display());
    }

    // 2.3.2.1 + root
    #[test]
    fn test_remove_black_2_3_2_1_root_success() { 
        let mut tree = Tree::new();
        let nodes: Vec<i32> = vec![5,4,6];
        for i in nodes{tree.put(i);}
        assert!(tree.is_a_valid_red_black_tree()); 
        tree.remove(6);
        assert!(tree.is_a_valid_red_black_tree()); 
        //println!("{}",tree.display());
    }

    
    // https://rflinux.blogspot.com/2011/10/red-black-trees.html
    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_black_2_3_2_2_up_to_down_success -- --nocapture
    #[test]
    fn test_remove_black_2_3_2_2_up_to_down_success() {
        
        let mut tree: Tree<i32> = Tree::new();
      
        let nodes = vec![352,873,462,836,316,381,595,288,600,263,310,
        74,544,621,402,618,61,576,654,579,985,949/*,856,796,894,6,991,880,652,349,525,9,515,371,53*/];
        for i in nodes{ tree.put(i);}
            
        //println!("{}",tree.display());
        unsafe{Tree::checking_connections(tree.root);} 

        //tree.remove(6);
        //println!("{}",tree.display());
        // unsafe{Tree::checking_connections(tree.root);} 
    }
    


/* 

    // $ cargo test red_black_tree::red_black_tree_nonnull::tests::test_remove_black_2_3_2_2_start_v5_success -- --nocapture
    #[test]
    fn test_remove_black_2_3_2_2_start_v5_success() {
        
        let mut tree: Tree<i32> = Tree::new();
      
        let nodes = vec![480,978,379,784,398,71,695,23,97,309,312,449,958,992,220,95,257,869,959,450,258,315,783,731,914,
        880,984,734,570,801,908,181,466,238,916,77,801,867,382,943,603,65,545,200,759,158,987,821,630,537,
        704,149,617,498,261,160,192,760,417,939,757,858,376,885,336,764,443,155,983,586,957,375,
        893,707,255,811,86,370,384,177,834,177,834,313,209,623,176,875,748,949,529,932,369,385,
        419,222,719,342,68,156,314,343,262,467,499,604,732,758,765,812,859,876];
        for i in nodes{ tree.put(i);}
            
        // println!("{}",tree.display());
        unsafe{Tree::checking_connections(tree.root);} 

        tree.remove(23);
        println!("{}",tree.display());
        // unsafe{Tree::checking_connections(tree.root);} 
    }

    // 2.3.2.2 
    #[test]
    fn test_remove_black_2_3_2_2_success() { 
        let mut tree = Tree::new();
        let nodes: Vec<i32> = vec![246,562,950,237,417,418,416];
        for i in nodes{tree.put(i);}
        assert!(tree.is_a_valid_red_black_tree()); 
        tree.remove(418);
        assert!(tree.is_a_valid_red_black_tree()); 
        println!("{}",tree.display());
    }

    // 2.3.2.2 + root
    #[test]
    fn test_remove_black_2_3_2_2_root_success() { 
        let mut tree = Tree::new();
        let nodes: Vec<i32> = vec![5,4,6];
        for i in nodes{tree.put(i);}
        assert!(tree.is_a_valid_red_black_tree()); 
        tree.remove(4);
        assert!(tree.is_a_valid_red_black_tree()); 
        println!("{}",tree.display());
    }
 */
}