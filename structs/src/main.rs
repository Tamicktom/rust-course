// Hashmap like struct
struct User {
    active: bool,
    username: String,
    sign_in_count: u32,
}

// Tuple struct
struct Coordenates(i32, i32, i32);

// Unit struct
struct UnitStruct;

fn main() {
    let user = User {
        active: true,
        username: String::from("Tyler"),
        sign_in_count: 1,
    };
    println!("{:?}", user.username);

    let user2 = build_user(String::from("Tyler2"));
    println!("{:?}", user2.username);

    let cords = Coordenates(1, 2, 3);
    println!("{:?}", cords.0);

    let unit = UnitStruct;
}

fn build_user(username: String) -> User {
    return User {
        username,
        active: true,
        sign_in_count: 0,
    };
}
