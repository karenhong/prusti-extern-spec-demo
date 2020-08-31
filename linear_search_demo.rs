use prusti_contracts::*;

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
