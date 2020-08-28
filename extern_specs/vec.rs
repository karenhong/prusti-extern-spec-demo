use prusti_contracts::*;

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
