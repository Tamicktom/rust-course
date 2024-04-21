fn main() {
    // let var = 1; // created on the stack
    //     let mut s = "Hello".to_string(); // created on the heap
    //    s.push_str(", world!");

    //  println!("var: {}", var);
    //    println!("s: {}", s);

    //  let x = vec!["tyler".to_string()];
    //let y = x.clone();
    //    let z = y.clone();

    //  println!("{:?}", x); // error: value used here after move
    //    println!("{:?}", y); // error: value used here after move
    //    println!("{:?}", z); // error: value used here after move
    //
    let s = String::from("takes"); // create a variable with a string takes
    takes_ownership(s); // give ownership of the variable to the function

    let val = 1;
    makes_copy(val);

    //let str1: String = give_ownership();
    //    println!("{}", str1);

    //  let str3 = take_and_give(str1);
    //    println!("{}", str2);

    //if (true) {
    //  let str4 = str3;
    //    } else {
    //      let str5 = str3;
    //    }

    //  println!("{}", str4);

    let mut str1 = String::from("hello");
    let mut str2: String;

    loop {
        str2 = str1;
        println!("{}", str2);
        break;
    }
}

fn takes_ownership(s: String) {
    let strin = s;

    println!("{}", strin);
}

fn makes_copy(one: i32) {
    let val2 = one;
    println!("{}", val2);
}

fn give_ownership() -> String {
    "given".to_string()
}

fn take_and_give(str2: String) -> String {
    str2
}
