struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}
fn main() {
    let user1 = User{
        active: true,
        username: String::from("km"),
        email: String::from("km@example.com"),
        sign_in_count: 1
    };

    // can not update as not mutable
    //user1.email = String::from("km1@example.com");

    let mut user2 = User{
        active: true,
        username: String::from("km"),
        email: String::from("km@example.com"),
        sign_in_count: 1
    };

    // either full mutable or nothing mutable
    user2.email = String::from("km1@example.com");
}

fn build_user(email: String, username: String) -> User {
    User { 
        active: true, 
        username, 
        email, 
        sign_in_count: 1 }
}