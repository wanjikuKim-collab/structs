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

#[derive(Debug)]
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
    let claire = build_user(String::from("Claire"), String::from("claire@gmail.com"));
    let emy = User{
        username: String::from("Emy Kimani"),
        email: String::from("emy@gmail.com"),
        ..kimmy
    };
 
    println!("The area of the rectangle is :{}", rect1.area());
    println!("My new users are {:#?} and {:#?}", claire, emy);
    println!("My name is {:?} and I've signed in {:?} times", kimmy.username, kimmy.sign_in_count);
 }  

 fn build_user(username: String, email: String)-> User{
    User{
        active: true,
        username,
        email,
        sign_in_count: 5,
    }
}
