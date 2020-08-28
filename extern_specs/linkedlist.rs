use prusti_contracts::*;

use std::collections::LinkedList;

/// Ghost method for LinkedList used to access an index in the LinkedList
#[trusted]
#[pure]
#[requires(index < list.len())]
pub fn get(list: &LinkedList<i32>, index: usize) -> i32 {
    for (i, elem) in list.iter().enumerate() {
        if i == index {
            return *elem;
        }
    }
    unreachable!()
}

/// Using i32 instead of a generic type because Prusti does not currently support it for this example.
/// However, it is not possible to define specifications for different variants of the same type and if
/// attempted, will result in an error indicating duplicate specifications.
#[extern_spec]
impl LinkedList<i32> {
    #[ensures(result.is_empty())]
    pub fn new() -> LinkedList<i32>;

    #[pure]
    #[ensures(result ==> self.len() == 0)]
    #[ensures(!result ==> self.len() > 0)]
    pub fn is_empty(&self) -> bool;

    #[pure]
    pub fn len(&self) -> usize;

    #[ensures(self.len() == 0)]
    pub fn clear(&mut self);

    #[ensures(self.len() == old(self.len()) + 1)]
    #[ensures(get(self, 0) == elt)]
    #[ensures(forall (|i: usize| (i < old(self.len())) ==>
        get(self, i + 1) == old(get(self, i))))]
    pub fn push_front(&mut self, elt: i32);

    #[ensures(old(self.len()) == 0 ==> (self.len() == old(self.len())) && result.is_none())]
    #[ensures(old(self.len()) > 0 ==> self.len() == old(self.len()) - 1 && result.is_some())]
    #[ensures(old(self.len()) > 0 ==> forall (|i: usize| (i < self.len()) ==>
        get(self, i) == old(get(self, i + 1))))]
    pub fn pop_front(&mut self) -> Option<i32>;

    #[ensures(self.len() == old(self.len()) + 1)]
    #[ensures(get(self, self.len() - 1) == elt)]
    #[ensures(forall (|i: usize| (i < old(self.len())) ==>
    get(self, i) == old(get(self, i))))]
    pub fn push_back(&mut self, elt: i32);

    #[ensures(old(self.len()) == 0 ==> (self.len() == old(self.len())) && result.is_none())]
    #[ensures(old(self.len()) > 0 ==> self.len() == old(self.len()) - 1 && result.is_some())]
    #[ensures(old(self.len()) > 0 ==> forall (|i: usize| (i < self.len()) ==>
        get(self, i) == old(get(self, i))))]
    pub fn pop_back(&mut self) -> Option<i32>;

    #[ensures(self.len() == old(self.len() + other.len()))]
    #[ensures(forall (|i: usize| (i < old(self.len())) ==>
        get(self, i) == old(get(self, i))))]
    #[ensures(forall (|j: usize| (old(self.len()) <= j && j < self.len()) ==>
        get(self, j) == old(get(other, j - self.len()))))]
    pub fn append(&mut self, other: &mut LinkedList<i32>);

    #[requires(at <= self.len())]
    #[ensures(result.len() == old(self.len()) - at)]
    #[ensures(self.len() == at)]
    #[ensures(forall (|i: usize| (i < self.len()) ==>
        get(self, i) == old(get(self, i))))]
    #[ensures(forall (|j: usize| (j < result.len()) ==>
        get(&result, j) == old(get(self, j + at))))]
    pub fn split_off(&mut self, at: usize) -> LinkedList<i32>;
}
