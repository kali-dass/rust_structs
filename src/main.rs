struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

//struct of tuple type
struct Color(i32,i32,i32);

//unit struct
struct AlwaysEqual;

// struct rectangle
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

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

    // unit like struct
    let subject = AlwaysEqual;

    let rect1 = Rectangle{
        width: 30,
        height: 80
    };

    // pass ref
    println!("The area of the rectangle is {} square pixels", area(&rect1));

    //implement display
    // pritty print as well
    println!("The rect is {:?} or {:#?}", rect1, rect1);

    // dbg macro
    dbg!(&rect1);

    // method
    println!("The area of the rectangle is {} square pixels", rect1.myarea());

}

fn build_user(email: String, username: String) -> User {
    User { 
        active: true, 
        username, 
        email, 
        sign_in_count: 1 }
}

// pass strcut and calculate area
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// method
impl Rectangle{
    fn myarea(&self) -> u32 {
        self.width * self.height
    }
}