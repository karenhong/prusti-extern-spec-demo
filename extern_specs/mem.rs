use prusti_contracts::*;

#[extern_spec]
mod std {
    mod mem {
        use prusti_contracts::*;

        #[ensures(*a == old(*b) && *b == old(*a))]
        pub fn swap(a: &mut i32, b: &mut i32);
    }
}
