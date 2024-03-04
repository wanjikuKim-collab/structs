#[derive(Debug)]
struct Rectangle{
    width:u32,
    length:u32
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: i32,
}

fn main(){
    let rect1 = Rectangle{
        width: 30,
        length: 50
    };

    println!("The area of my rectangle is {}", area(&rect1));
    println!("My rect1 instance is: {:#?}", &rect1);

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

 fn area (rectangle: &Rectangle) ->u32{
    rectangle.length * rectangle.width
 }
