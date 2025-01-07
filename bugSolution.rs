fn main() {
    let mut x = 5;
    { // Creating a new scope for the first mutable borrow.
        let y = &mut x;
        *y += 1;
    }
    { // Creating a new scope for the second mutable borrow
        let z = &mut x;
        *z += 1;
    }
    println!("x = {}", x);
}
//Alternative using a temporary variable
fn main() {
    let mut x = 5;
    let mut temp = x;
    temp += 1;
    x = temp;
    temp = x;
    temp += 1;
    x = temp;
    println!("x = {}", x);
}