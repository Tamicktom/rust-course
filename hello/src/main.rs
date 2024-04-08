fn main() {
    basic();
    tuples();
    arrays();
    vectors();
    strings();
    input();
}

fn basic() {
    let x: i32 = 5;

    println!("The value of x is: {}", x);

    let y: u8 = 10;

    let decimal: f64 = 65.4321;
    let hex: i32 = 0x1234abcd;
    let octal: i32 = 0o54321;
    let binary: i32 = 0b1010101010101010;

    println!("y: {}", y);

    println!("decimal: {}", decimal);
    println!("hex: {}", hex);
    println!("octal: {}", octal);
    println!("binary: {}", binary);

    let remainder = 43 % 5;

    println!("The remainder of 43 divided by 5 is: {}", remainder);
}

fn tuples() {
    let tup: (i32, &str, f64, bool) = (500, "h1", 1.5, false);

    let (a, b, c, d) = tup;

    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);
}

fn arrays() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];

    println!("array[0]: {}", array[0]);

    let mut array2: [i32; 5] = [0; 5];

    array2[0] = 10;
    array2[1] = 20;
    array2[2] = 30;
    array2[3] = 40;
    array2[4] = 50;

    println!("array2[0]: {}", array2[0]);
}

fn vectors() {
    let mut vector: Vec<i32> = vec![1, 2, 3, 4, 5];

    vector.push(6);

    println!("vector: {:?}", vector);

    let mut vec: Vec<&str> = Vec::with_capacity(2);

    vec.push("hello");
    vec.push("world");
    vec.push("banana");

    vec.reverse();

    println!("vec: {:?}", vec);

    let v: Vec<i32> = (1..5).collect();

    let sv: &[i32] = &v;

    println!("sv: {:?}", sv);
}

fn strings(){
    let s: &str = "hello";

    let mut s2: String = String::from("world");

    s2.push_str(" banana");

    println!("s: {}", s);
    println!("s2: {}", s2);
}

fn input(){
    use std::io;

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    println!("You typed: {}", input);
}