#[derive(Debug)]
#[allow(dead_code)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1: User = User {
        username: String::from("RyanLilleyman"),
        email: String::from("lilleymanryan2020@gmail.com"),
        active: true,
        sign_in_count: 1,
    };
    println!("{:?}", user1);
    let name: &String = &user1.username;
    let user2: User = build_user(
        "MarthaStewart@gmail.com".to_string(),
        "MarthaStewart".to_string(),
    );
    println!("{}", name);
    println!("{:?}", user2);

    // creating new instances of a struct fron an existing instance
    let user3: User = User {
        email: String::from("newemail@gmail.com"),
        username: String::from("hello"),
        ..user2
    };

    println!("{:?}", user3);

    //tuple struct
    //notice neither have named fields just expected types
    //the entire struct name bundles these types
    //together and expects relevant and valid data
    //struct Color(i32, i32, i32);
    //struct Point(i32, i32, i32);
}
