#![allow(unused_imports)]
#![allow(unused_attributes)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(unused_comparisons)]
#![allow(dead_code)]
#![feature(type_ascription)]

extern crate prusti_contracts;
use prusti_contracts::*;
use std::option::Option;

#[extern_spec]
impl<T> std::option::Option<T> {
    #[pure]
    #[ensures(matches!(*self, Some(_)) == result)]
    pub fn is_some(&self) -> bool;

    #[pure]
    #[ensures(self.is_some() == !result)]
    pub fn is_none(&self) -> bool;
}

#[extern_spec]
impl<T> Vec<T> {
    #[ensures(result.len() == 0)]
    fn new() -> Vec::<T>;

    #[pure]
    fn len(&self) -> usize;

    #[ensures(self.len() == old(self.len()) + 1)]
    fn push(&mut self, value: T);

    #[ensures(self.len() == 0)]
    fn clear(&mut self);

    #[ensures(old(self.len()) == 0 ==> result.is_none())]
    #[ensures(old(self.len()) > 0 ==> result.is_some())]
    #[ensures(old(self.len()) > 0 ==> self.len() == old(self.len()) - 1)]
    pub fn pop(&mut self) -> Option<T>;

    #[ensures(self.len() == old(self.len()) + other.len())]
    pub fn append(&mut self, other: &mut Vec<T>);
}

#[trusted]
#[pure]
#[requires(index < list.len())]
pub fn lookup(list: &Vec::<i32>, index: usize) -> i32 {
    list[index]
}

#[ensures(result.is_none() ==> (forall (|k: usize| (k >= 0 && k < arr.len()) ==> lookup(arr, k) != elem)))]
#[ensures((match result {
    Option::Some(i) => i < arr.len() && lookup(arr, i) == elem,
    Option::None => true}))]
fn linear_search_recursive(arr: &Vec<i32>, elem: i32) -> Option<usize> {
    linear_search_helper(arr, elem, 0)
}

#[ensures(result.is_none() ==> (forall (|k: usize| (k >= index && k < arr.len()) ==> lookup(arr, k) != elem)))]
#[ensures((match result {
    Option::Some(i) => i < arr.len() && lookup(arr, i) == elem,
    Option::None => true}))]
fn linear_search_helper(arr: &Vec<i32>, elem: i32, index: usize) -> Option<usize> {
    if index >= arr.len() {
        None
    } else if lookup(arr, index) == elem {
        Some(index)
    } else {
        linear_search_helper(arr, elem, index + 1)
    }
}

fn main() {}
