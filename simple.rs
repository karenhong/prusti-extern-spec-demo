use prusti_contracts::*;

fn simple_test() {
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
