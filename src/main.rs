#[derive(Debug)]
struct Rectangle{
    width:u32,
    length:u32
}

// the method can either take ownership, a reference(mut or unimmutable)
impl Rectangle{
    fn area(&self)-> u32{
        self.width * self.length
    }

    fn can_hold (&self, other: &Rectangle) ->bool {
        self.area() >= other.area()
    }
}

//Associated functions
impl Rectangle {
    fn Square(size: u32) -> Rectangle {
        Rectangle{
            width: size,
            length: size,
        }
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
    //Rectangle instances
    let rect1 = Rectangle{
        width: 30,
        length: 50
    };

    let rect2 = Rectangle{
        width: 40,
        ..rect1
    };

    let rect3 = Rectangle{
        width: 20,
        length: 200
    };

    // creating instance for an associated fn 
    let square = Rectangle::Square(25);

    println!("The area of my rect1 is {}",rect1.area());
    println!("The area of my rect2 is {}",rect2.area());
    println!("The area of my rect3 is {}",rect3.area());
    println!("The area of my square is {}",square.area());
    println!("My rect1 instance is: {:#?}", &rect1);
    println!("rect1 can hold rect2 ? {}", rect1.can_hold(&rect2));
    println!("rect2 can hold rect1 ? {}", rect2.can_hold(&rect1));
    println!("rect2 can hold rect3 ? {}", rect2.can_hold(&rect3));
    println!("rect3 can hold rect2 ? {}", rect3.can_hold(&rect2));




    //User instances
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

//  fn area (rectangle: &Rectangle) ->u32{
//     rectangle.length * rectangle.width
//  }
