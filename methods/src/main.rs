struct Square {
    width: u32,
    height: u32,
}

impl Square {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn whats_my_width(&self) -> u32 {
        return self.width;
    }

    fn cange_width(&mut self, new_width: u32) {
        self.width = new_width;
    }
}

fn main() {
    let mut square = Square {
        width: 5,
        height: 5,
    };

    let area = square.area();
    println!("{}", area);

    println!("Whats my width {}", square.whats_my_width());
    
    square.cange_width(10);
    println!("Whats my width {}", square.whats_my_width());
}