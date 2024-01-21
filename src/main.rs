#![allow(unused_imports)]

use ds::{stack_array, stack_linked_list, stack_vec, red_black_tree_nonnull};

 
use red_black_tree_nonnull::Tree;
use std::time::Instant;
 
fn main() {
    
    let mut value = 0;
    let size = 16_777_216;// 67_108_864 2^26 // 16_777_216 2^24 
    let mut src = Vec::with_capacity(size);
    for i in 0..size {
        src.push(rand::random::<u64>());
        if i == 500_000{
           value = rand::random::<u64>();
           println!("Find {}",value);
           src.push(value);
        }
    }
    // *******************************************************
    let now = Instant::now();
    let mut tree: Tree<u64> = Tree::new();
    for i in src.iter() {
        tree.put(*i);
    }
    println!("LLRB Insert :{} time", now.elapsed().as_millis());
    let now = Instant::now();
    assert!(tree.contains(value));
    println!("LLRB Find   :{} time", now.elapsed().as_nanos());
    assert!(tree.helper_is_a_valid_red_black_tree());

    // *******************************************************
    let now = Instant::now();
    src.sort();
    println!("Vec sort    :{} time", now.elapsed().as_millis());
    let now = Instant::now();    
    assert!(src.binary_search(&value).is_ok());
    //assert!(src.contains(&value));
    println!("Vec Find    :{} time", now.elapsed().as_nanos());

}

/*

N = 16_777_216 H=24 

LLRB Insert :46987 time
LLRB Find   :3772 time
Validation black height = 20
Vec sort    :4757 time
Vec Find    :3212 time

*/