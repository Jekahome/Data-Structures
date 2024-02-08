#![allow(unused_imports)]

use ds::{
    binary_search_tree_good_nonnull, red_black_tree_nonnull, red_black_tree_vec, stack_array,
    stack_linked_list, stack_vec,
};

use std::time::Instant;

fn main() {
    cmp_llrb_vs_vec();

    cmp_llrb_ptr_vs_vec();
}

/*

N = 16_777_216, Tree H=24


LLRB Find ----------- :4310 nanos
Vec Find with sort -- :230 millis
Vec Find without sort :142 millis

*/
fn cmp_llrb_vs_vec() {
    let size = 16_777_216;
    let mut src = Vec::with_capacity(size);
    let mut for_find = vec![];
    for _ in 0..size {
        src.push(rand::random::<u64>());
    }

    loop {
        let value = rand::random::<u64>();
        if !src.contains(&value) {
            for_find.push(value);
        }
        if for_find.len() == 50 {
            break;
        }
    }
    let mut src_2 = src.clone();

    // LLRB

    let src: Vec<u64> = red_black_tree_nonnull::helper_prepare_batch_put(&mut src);
    let mut tree = red_black_tree_nonnull::Tree::new();
    let now = Instant::now();
    for i in src.iter() {
        tree.put(*i);
    }
    println!("LLRB Insert :{} millis", now.elapsed().as_millis());
    assert!(tree.helper_is_a_valid_red_black_tree());

    let mut result_find = vec![];
    for value in for_find.iter() {
        let now = Instant::now();
        tree.put(*value);
        assert!(tree.contains(value));
        result_find.push(now.elapsed().as_nanos());
    }

    println!(
        "LLRB average search time   :{} nanos",
        result_find.iter().sum::<u128>() / result_find.len() as u128
    );

    // Vec
    let mut result_find = vec![];
    for value in for_find.iter() {
        let now = Instant::now();
        src_2.push(*value);
        //src_2.sort();
        //assert!(src_2.binary_search(&value).is_ok());
        assert!(src_2.contains(&value));
        result_find.push(now.elapsed().as_millis());
    }
    println!(
        "Vec average search time    :{} millis",
        result_find.iter().sum::<u128>() / result_find.len() as u128
    );
}

/*
LLRB ptr Insert :21378 millis

LLRB Vec Insert :24780 millis
*/
fn cmp_llrb_ptr_vs_vec() {
    let size = 16_777_216;
    let mut src = Vec::with_capacity(size);
    let mut for_find = vec![];
    for _ in 0..size {
        src.push(rand::random::<u64>());
    }

    loop {
        let value = rand::random::<u64>();
        if !src.contains(&value) {
            for_find.push(value);
        }
        if for_find.len() == 50 {
            break;
        }
    }

    let now = Instant::now();
    let mut tree = red_black_tree_nonnull::Tree::new();
    for i in src.iter() {
        tree.put(*i);
    }
    println!("LLRB ptr Insert :{} millis", now.elapsed().as_millis());
    assert!(tree.helper_is_a_valid_red_black_tree());

    let now = Instant::now();
    let mut tree = red_black_tree_vec::Tree::new(src.len());
    for i in src.iter() {
        tree.put(*i);
    }
    println!("LLRB Vec Insert :{} millis", now.elapsed().as_millis());
    assert!(tree.helper_is_a_valid_red_black_tree());
}
