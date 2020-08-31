use prusti_contracts::*;

fn simple_vec_test() {
    let mut v = Vec::new();
    
    assert!(v.len() == 0);
    
    v.push(2);
    v.push(3);
    v.push(5);
    
    assert!(v.len() == 3);
    
    v.pop();
    
    assert!(v.len() == 2);
    
    v.pop();
    v.pop();
    
    assert!(v.pop().is_none());
}

fn swap_test() {
    let mut x = 5;
    let mut y = 42;

    std::mem::swap(&mut x, &mut y);

    assert!(42 == x);
    assert!(5 == y);
}
