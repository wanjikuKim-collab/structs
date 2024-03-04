  // refactoring with structs
  #[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
} 

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }    
}

struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: i32,
}
fn main(){
    let rect1= Rectangle{
        width: 20,
        height: 20,
    };

    let mut kimmy = User{
        username: String::from("Kimmy"),
        email: String::from("kimmy@gmail.com"),
        active: true,
        sign_in_count: 5,
    };

    kimmy.active = false;
 
    println!("The area of the rectangle is :{}", rect1.area());
 }

 fn build_user(username: String, email: String)-> User{
    User{
        active: true,
        username,
        email,
        sign_in_count: 5,
    }
}
