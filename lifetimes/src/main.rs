fn main() {
    

    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r); // 'a
}

fn example<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}




