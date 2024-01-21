#![allow(unused_imports)]

use ds::{stack_array, stack_linked_list, stack_vec, red_black_tree_nonnull};


use red_black_tree_nonnull::Tree;
use std::time::Instant;
 
fn main() {
    
    let mut value = 0;
    let mut src = vec![];
    for i in 0..10000000 {
        src.push( rand::random::<i32>());
        if i == 500000{
           value = rand::random::<i32>();
           src.push( value);
        }
    }
    // *******************************************************
    let mut tree: Tree<i32> = Tree::new();
    for i in src.iter() {
        tree.put(*i);
    }
    let now = Instant::now();
    assert!(tree.contains(value));
    println!("LLRB Find {} time", now.elapsed().as_nanos());
    assert!(tree.helper_is_a_valid_red_black_tree());

    // *******************************************************
    let mut array = vec![];
    for i in src.iter()  {
        array.push(*i);
    }
    array.sort();
    let now = Instant::now();    
    assert!(array.binary_search(&value).is_ok());
    //assert!(array.contains(&value));
    println!("Vec Find {} time", now.elapsed().as_nanos());

}

  