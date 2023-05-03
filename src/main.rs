struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

//struct of tuple type
struct Color(i32,i32,i32);

fn main() {
    // create struct instance
    let user1 = User{
        active: true,
        username: String::from("km"),
        email: String::from("km@example.com"),
        sign_in_count: 1
    };

    // can not update as not mutable
    //user1.email = String::from("km1@example.com");

    // create mutable instance
    let mut user2 = User{
        active: true,
        username: String::from("km"),
        email: String::from("km@example.com"),
        sign_in_count: 1
    };

    // either full mutable or nothing mutable
    user2.email = String::from("km1@example.com");

    // create instance from another instance values
    let user3 = User{
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count
    };

    // shorthand copy
    let user4 = User{
        email: String::from("123@example.com"),
        ..user2
    };

    // struct tuple type
    let black = Color(0,0,0);

}

fn build_user(email: String, username: String) -> User {
    User { 
        active: true, 
        username, 
        email, 
        sign_in_count: 1 }
}