#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // constructor
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

pub fn main() {
    let user1 = User {
        active: true,
        username: String::from("John"),
        email: String::from("john@john.com"),
        sign_in_count: 10,
    };

    dbg!(&user1);

    let email = user1.email;

    println!("User1 email: {email}");

    // struct update syntax
    let user2 = User {
        email: String::from("johnpork@bacon.com"),
        ..user1
    };

    let email = user2.email;
    println!("User2 email: {email}");


    println!("");
    
    let rect1 = Rectangle {
        width: 10,
        height: 12,
    };

    println!("Rectangle: {rect1:#?}");
    
    let area = rect1.area();

    println!("Area of rectangle = {area}");



}