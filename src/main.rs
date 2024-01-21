#![allow(unused_imports)]

use ds::{stack_array, stack_linked_list, stack_vec, red_black_tree_nonnull};

/*
Есть ли смысл паралелить поиск?
Время вставки/поиска
*/
use red_black_tree_nonnull::Tree;
use std::time::{Duration, Instant};
use std::thread::sleep; 

fn main() {
    
    let mut tree: Tree<i32> = Tree::new();
    
    for i in 0..100 {
        tree.put(i);
    }
    let value = 55;
    let now = Instant::now();
    tree.find(value);
    println!("Find {} micros", now.elapsed().as_micros());
    assert!(tree.helper_is_a_valid_red_black_tree());
}

  