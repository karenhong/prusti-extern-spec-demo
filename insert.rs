use std::collections::LinkedList;

use prusti_contracts::*;
use extern_specs::linkedlist::get;

#[requires(index <= list.len())]
#[ensures(list.len() == old(list.len()) + 1)]
#[ensures(forall (|i: usize| (0 < i && i < index && i < old(list.len())) ==>
    get(list, i) == old(get(list, i))))]
#[ensures(get(list, index) == val)]
#[ensures(forall (|j: usize| (index < j && j < list.len()) ==>
    get(list, j) == old(get(list, j - 1))))]
fn insert(list: &mut LinkedList<i32>, index: usize, val:i32) {
    if index == 0 {
        list.push_front(val);
    } else {
        let mut tail = list.split_off(index);
        list.push_back(val);
        list.append(&mut tail);
    }
}
