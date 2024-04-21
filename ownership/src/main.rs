fn main() {
    let var = 1; // created on the stack
    let mut s = "Hello".to_string(); // created on the heap
    s.push_str(", world!");

    println!("var: {}", var);
    println!("s: {}", s);

    let x = vec!["tyler".to_string()];
    let y = x;
    let z = y;
    println!("{:?}", z); // error: value used here after move
}

//var is droped, s is dropped
