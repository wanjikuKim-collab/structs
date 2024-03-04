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
 
    println!("The area of the rectangle is :{}", rect1.area());
 }
