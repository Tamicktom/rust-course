fn main() {
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
