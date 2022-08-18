fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail.example.com");

    //Creating instances from another instances with struct update syntax
    let user2 = User {
        email: String::from("anoter@example.com"),
        username: user1.username,
        active: user1.active,
        sign_in_count: 1,
    };

    let user3 = User {
        email: String::from("another3@example.com"),
        ..user2 //rest of the values from user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//unit-like structs
struct AlwaysEqual;

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
