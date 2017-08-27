fn main() {

    let user1 = User{
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1
    };

    println!("{}", user1.username);

    let user2 = User {
        email: String::from("someone-else@example.com"),
        username: String::from("someotherusername123"),
        ..user1
    };

    println!("{}", user2.username);

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}
