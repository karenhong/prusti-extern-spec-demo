/// This version of linear search uses the following unsupported features 
///     1) The slice type is currently not supported with the external specs feature 
///        therefore indexing into the array is not supported in specifications
///     2) Loop invariants are temporarily broken due to the compiler upgrades

use prusti_contracts::*;

#[ensures(result.is_some() == arr.contains(&elem))]
#[ensures(result.is_some() ==> arr[result.unwrap()] == elem)]
fn linear_search(arr: &Vec<i32>, elem: i32) -> Option<usize> {
    let mut i = 0;
    let mut found = false;

    #[invariant(i < arr.len())]
    #[invariant(forall (|k: usize| (k < i) ==> arr[k] != elem))]
    #[invariant(found ==> (i < arr.len() && arr[i] == elem))]
    while i < arr.len() && !found {
        if arr[i] == elem {
            found = true;
        } else {
            i += 1;
        }
    }

    if found {
        Some(i)
    } else {
        None
    }
}
