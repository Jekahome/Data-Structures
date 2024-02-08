
/*
  Удаление: создать очередь свободных индексов вместо nodes[None]
  Проблема - долгая вставка, как решить? Особенно в ставнении с std::collections::BinaryHeap
*/
pub use llrb::Tree;
mod llrb{
    use std::cmp::Ordering;
    use std::fmt::{Debug, Display};

    pub struct Tree<T> {
        root: Option<IndexNode>,
        nodes: Vec<Option<Node<T>>>,// 0 => IndexNode
        count: usize
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct IndexNode(usize);

    impl IndexNode {
        pub fn new(value: usize) -> Self {
            Self(value)
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct Node<T> {
        left: Option<IndexNode>,
        right: Option<IndexNode>,
        parent: Option<IndexNode>,
        is_red: bool,
        value: T,
    }

    enum OperationPut {
        Left,
        Right,
        FlipColors,
        Nothing,
    }

    impl<T: Ord + PartialEq + PartialOrd + Default + Display + Clone + Debug> Tree<T>{
        pub fn new(size: usize) -> Self {
            Self {
                root: None,
                nodes: Vec::with_capacity(size),
                count: 0,
            }
        }

        pub fn put(&mut self, value: T) -> bool {
            if let Some(index_parent) = self.find_put_parent_candidate(self.root, &value) {
                if self.attach_node(index_parent, value) {
                   self.put_balancing(index_parent);
                }
            }else{
                if self.root.is_none(){
                    self.nodes.push(Node::new_black(value));
                    self.root = Some(IndexNode::new(0));
                }else{
                   return false; 
                }
            }
            self.count += 1;
            true
        }
        
        fn get_mut_node(&mut self, index: &IndexNode) -> Option<&mut Node<T>>{
            unsafe { self.nodes.get_unchecked_mut(index.0).as_mut() }
        }
       
        fn get_node(&self, index: &IndexNode) -> Option<&Node<T>>{
            unsafe { self.nodes.get_unchecked(index.0).as_ref() }
        }
       
        fn find_put_parent_candidate(&self, parent: Option<IndexNode>, elem: &T) -> Option<IndexNode> {
            if let Some(index_parent) = parent {
                if let Some(parent_node) = self.get_node(&index_parent){
                    match elem.cmp(&parent_node.value) {
                        Ordering::Equal => {
                            return None;
                        }
                        Ordering::Less => {
                            if parent_node.left.is_some() {
                                return self.find_put_parent_candidate(parent_node.left, elem);
                            } else {
                                return Some(index_parent);
                            }
                        }
                        Ordering::Greater => {
                            if parent_node.right.is_some() {
                                return self.find_put_parent_candidate(parent_node.right, elem);
                            } else {
                                return Some(index_parent);
                            }
                        }
                    }
                }
            } 
            return None;
        }
       
        fn attach_node(&mut self, index_parent: IndexNode, elem: T) -> bool {
            self.nodes.push(Node::new_red(elem.clone(), index_parent));
            let index = self.nodes.len()-1;
            let parent_node = self.get_mut_node(&index_parent).unwrap();
            match elem.cmp(&parent_node.value) {
                Ordering::Equal => {
                    return false;
                }
                Ordering::Less => {
                    parent_node.left = Some(IndexNode::new(index));
                    return true;
                }
                Ordering::Greater => {
                    parent_node.right = Some(IndexNode::new(index));
                    return true;
                }
            }
        }

        fn check_put_balancing(&self, index: &IndexNode) -> OperationPut {
            let node = self.get_node(index).unwrap();
            if node.right.is_some() {
                let r = self.get_node(&node.right.unwrap()).unwrap();
                if node.left.is_some() {
                    if !node.is_red {
                        if let Some(l) = self.get_node(&node.left.unwrap()){
                            if l.is_red && r.is_red {
                                return OperationPut::FlipColors;
                            }
                        } 
                    }
                }
                if r.is_red {
                    return OperationPut::Left;
                }
            }
            if node.is_red && node.left.is_some() && node.parent.is_some()  {
                if let Some(l) = self.get_node(&node.left.unwrap()){
                    if l.is_red {
                        return OperationPut::Right;
                    }
                }
            }                
            return OperationPut::Nothing;
        }

        fn put_balancing(&mut self, next: IndexNode) {
            let mut next = next;
            loop {
                //let next_node = self.get_node(next).unwrap();
                match self.check_put_balancing(&next) {
                    OperationPut::Left => {
                        //if next_node.is_red {
                        //    if let Some(n) = self.rotate_left_right_flip_color(next) {
                        //        next = n;
                        //    }
                        //} else {
                            if let Some(n) = self.rotate_left(next) {
                                next = n;
                            }
                        //}
                    }
                    OperationPut::Right => {
                        let next_node = self.get_node(&next).unwrap();
                        let node_a = next_node.parent.unwrap();
                        if let Some(n) = self.rotate_right(node_a) {
                            next = n;
                        } 
                    }
                    OperationPut::FlipColors => {
                        self.flip_colors(next);
                        if let Some(next_node) = self.get_node(&next){
                            if next_node.parent.is_some() {
                                next = next_node.parent.unwrap();
                            } else {
                                break;
                            }
                        }
                    }
                    OperationPut::Nothing => {
                        break;
                    }
                }
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
        fn rotate_left(&mut self, index_node_a: IndexNode) -> Option<IndexNode> {
            
            if let Some(index_parent) = self.get_node(&index_node_a).unwrap().parent {
                let mut node_a_from_left = false;
                {
                    let parent = self.get_node(&index_parent).unwrap();
                    
                    if let Some(left) = parent.left{
                        if left.eq(&index_node_a) {
                            node_a_from_left = true;
                        }
                    }
                }

                let (color_node_a,index_node_c) = {
                    let node_a = self.get_node(&index_node_a).unwrap();
                    (node_a.is_red,node_a.right.unwrap())
                    
                };
                let index_node_e = {
                   let node_c = self.get_mut_node(&index_node_c).unwrap(); 
                   (*node_c).is_red = color_node_a; 
                   (*node_c).parent = Some(index_parent);
                   node_c.left
                };
                
                if let Some(index_node_e) = index_node_e {
                    if let Some(e) = self.get_mut_node(&index_node_e){
                        (*e).parent = Some(index_node_a);
                    }
                    let node_a = self.get_mut_node(&index_node_a).unwrap();
                    (*node_a).is_red = true;
                    (*node_a).right = Some(index_node_e);
                } else {
                    let node_a = self.get_mut_node(&index_node_a).unwrap();
                    (*node_a).is_red = true;
                    (*node_a).right = None;
                }                    
                 
                {
                    let node_c = self.get_mut_node(&index_node_c).unwrap(); 
                    (*node_c).left = Some(index_node_a);
                }
                
                {
                    let node_a = self.get_mut_node(&index_node_a).unwrap();
                    (*node_a).parent = Some(index_node_c);
                }
                
                let parent = self.get_mut_node(&index_parent).unwrap();
                if node_a_from_left {
                    (*parent).left = Some(index_node_c);
                } else {
                    (*parent).right = Some(index_node_c);
                } 
                return Some(index_node_c);
            } else {
            
                let node_a = self.get_node(&index_node_a).unwrap();
                let color_node_a = node_a.is_red;
                let index_node_c = node_a.right.unwrap();

                let index_node_e = {
                    let node_c = self.get_mut_node(&index_node_c).unwrap();
                    (*node_c).is_red = color_node_a;
                    (*node_c).left
                };

                {
                    let node_a = self.get_mut_node(&index_node_a).unwrap();
                    (*node_a).is_red = true;
                    (*node_a).right = index_node_e;
                }
            
                if let Some(index_node_e) = index_node_e {
                    let node_e = self.get_mut_node(&index_node_e).unwrap();
                    (*node_e).parent = Some(index_node_a);
                }  

                {
                    let node_c = self.get_mut_node(&index_node_c).unwrap();
                    (*node_c).left = Some(index_node_a);
                }
               
                {
                    let node_a = self.get_mut_node(&index_node_a).unwrap();
                    (*node_a).parent = Some(index_node_c);
                }
                {
                    let node_c = self.get_mut_node(&index_node_c).unwrap();
                    (*node_c).parent = None;
                }
                self.root = Some(index_node_c);
                
                return Some(index_node_c);  
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
        fn rotate_right(&mut self, index_node_a: IndexNode) -> Option<IndexNode> {
            if let Some(index_parent) = self.get_node(&index_node_a).unwrap().parent {
                let mut node_a_from_left = false;
                {
                    let parent = self.get_node(&index_parent).unwrap();
                    if let Some(left) = parent.left{
                        if left.eq(&index_node_a) {
                            node_a_from_left = true;
                        }
                    }
                }
                let (color_node_a,index_node_b) = {
                    let node_a = self.get_mut_node(&index_node_a).unwrap();
                    let color = node_a.is_red;
                    (*node_a).is_red = true;
                    (color,node_a.left.unwrap())
                    
                };
 
                let index_node_d = {
                    let node_b = self.get_mut_node(&index_node_b).unwrap(); 
                    (*node_b).is_red = color_node_a; 
                    (*node_b).parent = Some(index_parent);
                    node_b.right
                };

                if let Some(index_node_d) = index_node_d {
                    let node_d = self.get_mut_node(&index_node_d).unwrap();
                    (*node_d).parent = Some(index_node_a);
                    let node_a = self.get_mut_node(&index_node_a).unwrap();
                    (*node_a).left = Some(index_node_d);
                    (*node_a).parent = Some(index_node_b);
                } else {
                    let node_a = self.get_mut_node(&index_node_a).unwrap();
                    (*node_a).left = None;
                    (*node_a).parent = Some(index_node_b);
                }

                {
                    let node_b = self.get_mut_node(&index_node_b).unwrap(); 
                    (*node_b).right = Some(index_node_a);
                }
                 
                let parent = self.get_mut_node(&index_parent).unwrap();
                if node_a_from_left {
                    (*parent).left = Some(index_node_b);
                } else {
                    (*parent).right = Some(index_node_b);
                }
                return Some(index_node_b);
            } else {
                let node_a = self.get_node(&index_node_a).unwrap();
                let color_node_a = node_a.is_red;
                let index_node_b = node_a.left.unwrap();
                let index_node_d = {
                    let node_b = self.get_mut_node(&index_node_b).unwrap();
                    (*node_b).is_red = color_node_a;
                    (*node_b).right
                };

                {
                    let node_a = self.get_mut_node(&index_node_a).unwrap();
                    (*node_a).is_red = true;
                    (*node_a).parent = Some(index_node_b);

                }
                
                if let Some(index_node_d) = index_node_d {
                    let node_d = self.get_mut_node(&index_node_d).unwrap();
                    (*node_d).parent = Some(index_node_a);
                    
                    let node_a = self.get_mut_node(&index_node_a).unwrap();
                    (*node_a).left = Some(index_node_d);
                } else {
                    let node_a = self.get_mut_node(&index_node_a).unwrap();
                    (*node_a).left = None;
                }
                {
                    let node_b = self.get_mut_node(&index_node_b).unwrap();
                    (*node_b).right = Some(index_node_a);
                    (*node_b).parent = None;
                }
                self.root = Some(index_node_b);
                return Some(index_node_b);   
            }
        }

        /*
          Flip color

             |          ||
             A    =>    A
           // \\       / \
          B    C      B   C

        */
        fn flip_colors(&mut self, index: IndexNode) {
           if let Some(node) = self.get_mut_node(&index){
                if node.left.is_some() && node.right.is_some() {
                    (*node).is_red = true;
                    if node.parent.is_none() {
                        (*node).is_red = false;
                    }
                }else{
                    return ();
                }
            }else{
                return ();
            }
            
            let i = self.get_mut_node(&index).unwrap().left.unwrap();
            let left = self.get_mut_node(&i).unwrap();
            (*left).is_red = false;
            let i = self.get_mut_node(&index).unwrap().right.unwrap();
            let right = self.get_mut_node(&i).unwrap();
            right.is_red = false;                
        }

        /// DOT specification.
        /// TODO: open http://www.webgraphviz.com/?tab=map
        /// or https://dreampuf.github.io/GraphvizOnline/
        pub fn display(&self) -> String {
            if let Some(index_root) = self.root{
                return format!("\n\ndigraph Tree {{\n\tratio = fill;\n\tnode [style=filled fontcolor=\"white\"];\n{}}}",self.helper_display_tree(index_root)); 
            }
            "\nTree is empty".into() 
        }

        fn helper_display_tree(&self, index_node: IndexNode) -> String {
            let mut s: String = "".into();
            let node = self.get_node(&index_node).unwrap();
            let color = if node.is_red {
                "[color=\"red\"]"
            } else {
                "[color=\"black\"]"
            };

            if let Some(index_left) = node.left {
                if let Some(left) = self.get_node(&index_left){
                    s.push_str(&format!(
                        "\t{n1}->{n2} {color1}; {n1} {color1}; {n2} {color2};\n",
                        n1 = node.value,
                        n2 = left.value,
                        color1 = color,
                        color2 = if left.is_red {
                            "[color=\"red\"]"
                        } else {
                            "[color=\"black\"]"
                        },
                    ));
                }
            } else if node.right.is_some() {
                s.push_str(&format!(
                    "\t{n1}->node_null_{n1} [color=\"grey\"]; {n1} {color1};\n",
                    n1 = (*node).value,
                    color1 = color
                ));
                s.push_str(&format!(
                    "\tnode_null_{n1}[label=\"null\"]\n",
                    n1 = (*node).value
                ));
            }

            if let Some(index_right) = node.right {
                if let Some(right) = self.get_node(&index_right){
                    s.push_str(&format!(
                        "\t{n1}->{n2} {color1}; {n1} {color1}; {n2} {color2};\n",
                        n1 = node.value,
                        n2 = right.value,
                        color2 = if right.is_red {
                            "[color=\"red\"]"
                        } else {
                            "[color=\"black\"]"
                        },
                        color1 = color
                    ));
                }
            } else {
                s.push_str(&format!(
                    "\t{n1}->node_null_{n1} [color=\"grey\"]; {n1} {color1};\n",
                    n1 = node.value,
                    color1 = color
                ));
                s.push_str(&format!(
                    "\tnode_null_{}[label=\"null\"]\n",
                    node.value
                ));
            }

            if let Some(index_left) = node.left {
                s.push_str(&self.helper_display_tree(index_left));  
            }
            if let Some(index_right) = node.right {
                s.push_str(&self.helper_display_tree(index_right));  
            }

            s
         
        }

        pub fn helper_is_a_valid_red_black_tree(&self) -> bool {
            if self.count > 0 {
                let result = self.validate(self.root.as_ref(), true, 0);
                let red_red = result.0;
                let black_height_min = result.1;
                let black_height_max = result.2;
                println!("Validation black height = {}", black_height_min);
                return red_red == 0 && black_height_min == black_height_max;
            }
            false
        }
        // red-red violations, min black-height, max-black-height
        fn validate(
            &self,
            index_node: Option<&IndexNode>,
            is_red: bool,
            black_height: usize,
        ) -> (usize, usize, usize) {
            if let Some(index_node) = index_node{
                if let Some(n) = self.get_node(index_node) {
                    let red_red = if is_red && n.is_red { 1 } else { 0 };
                    let black_height = black_height
                        + match n.is_red {
                            false => 1,
                            _ => 0,
                        };
                    let l = self.validate(n.left.as_ref(), n.is_red, black_height);
                    let r = self.validate(n.right.as_ref(), n.is_red, black_height);
                    return (
                        red_red + l.0 + r.0,
                        std::cmp::min(l.1, r.1),
                        std::cmp::max(l.2, r.2),
                    );
                }
            } else {
               return (0, black_height, black_height);
            } 
            return (0, black_height, black_height);
        }
    }

    impl<T: Default + Display> Node<T> {
        pub fn new_black(value: T) -> Option<Node<T>> {    
            Some(Node{
                left: None,
                right: None,
                parent: None,
                is_red: false,
                value, 
            })  
        }

        pub fn new_red(value: T, index_parent: IndexNode) -> Option<Node<T>> {
            Some(Node{
                left: None,
                right: None,
                parent: Some(index_parent),
                is_red: true,
                value, 
            })  
        }
    }
 
}

/// $ cargo +nightly miri test red_black_tree_vec
/// $ cargo test red_black_tree_vec -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;
 
    #[test]
    fn test_success() {
        let nodes = vec![
            480, 978, 379, 784, 999, 695, 23, 97, 309, 312, 449, 958, 992, 220, 95, 257, 869, 959,
            450, 258, 315, 783, 731, 914, 880, 984, 734, 570, 801, 908, 181, 466, 238, 916, 77,
            801, 867, 382, 943, 603, 65, 545, 200, 759, 158, 987, 821, 630, 537, 704, 149, 617,
            498, 261, 160, 192, 760, 417, 939, 757, 858, 376, 885, 336, 764, 443, 155, 983, 586,
            957, 375, 893, 707, 255, 811, 86, 370, 384, 177, 834, 177, 834, 313, 209, 623, 176,
            875, 748, 949, 529, 932, 369, 385, 419, 222, 719, 342, 68, 156, 314, 343, 262, 467,
            499, 604, 732, 758, 765, 812, 859, 876,
        ];
        let mut tree: Tree<i32> = Tree::new(nodes.len());
        for i in nodes {
            tree.put(i);
        }

        println!("{}", tree.display());

        assert!(tree.helper_is_a_valid_red_black_tree());
    }

}