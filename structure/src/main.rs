struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self,other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

impl Rectangle {
    fn square(size: u32) ->Rectangle {
        Rectangle { 
            width: size, 
            height: size, 
        }
    }
}

fn main() {
    let mut user = User {
        email: String::from("sdaulibin@126.com"),
        username: String::from("binginx"),
        sign_in_count: 1,
        active: true
    };
    let username = user.username;
    //cannot assign to `user.email`, as `user` is not declared as mutable
    user.email = String::from("sdaulibin@gmail.com");
    println!("the info of user : {},{},{},{}",username,user.email,user.sign_in_count,user.active);

    let user2 = bulid_user(String::from("binginx@126.com"), String::from("sdaulibin"));
    println!("the info of user2 : {},{},{},{}",user2.username,user2.email,user2.sign_in_count,user2.active);

    let user3 = User {
        email: String::from("binginx@gmail.com"),
        username: String::from("andy"),
        ..user2
    };
    println!("the info of user3 : {},{},{},{}",user3.username,user3.email,user3.sign_in_count,user3.active);

    let rect = Rectangle {
        height: 40,
        width: 30,
    };
    println!("the value of rect is : {:#?}",rect);
    println!("the value of area is : {}",area(&rect));

    println!("the value of area is : {}",rect.area());

    let other_rect = Rectangle {
        height: 30,
        width: 20,
    };
    println!("the value of can_hold is {}",rect.can_hold(&other_rect));
    println!("the value of can_hold is {}",other_rect.can_hold(&rect));

    let square = Rectangle::square(50);
    println!("the value of can_hold is {}",square.can_hold(&rect));
}

fn bulid_user(email: String,username: String) ->User {
    User { 
        username, 
        email, 
        sign_in_count: 2, 
        active: false 
    }
}

fn area(rectangle: &Rectangle) -> u32{
    rectangle.height * rectangle.width
}