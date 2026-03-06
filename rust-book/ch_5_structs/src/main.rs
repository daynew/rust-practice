fn main() {
    println!("Hello, world!");
    let user1 = new_user(String::from("daynew"), String::from("daynew@example.com"));
    println!("username={0}", user1.username);
    let _user2 = User {
        username: String::from("doppleg"),
        ..user1
    };
    let _color = Color(1, 2, 3);
    rectangles();
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn new_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// Example of a tuple struct
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn rectangles() {
    let rect1 = Rectangle {
        width: 10,
        height: 20,
    };
    println!("rect1 {:?} area={}", rect1, rect1.area());
    dbg!(&rect1);
    let small_rect = Rectangle {
        width: 2,
        height: 2,
    };
    println!("Can rect1 hold small_rect? {}", rect1.can_hold(&small_rect));
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(width: u32) -> Self {
        Self {
            width: width,
            height: width,
        }
    }
}
