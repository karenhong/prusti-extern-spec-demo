use prusti_contracts::*;

#[extern_spec]
mod std {
    mod mem {
        use prusti_contracts::*;

        pub fn swap<T: std::cmp::PartialEq + Copy>(a: &mut T, b: &mut T);
    }
}
