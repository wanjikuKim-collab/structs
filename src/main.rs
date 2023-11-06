  // refactoring with structs
  struct Rectangle{
    width: u32,
    height: u32,
} 
fn main(){

    let rect1= Rectangle{
        width: 10,
        height: 20,
    };
 
    println!("The area of the rectangle is :{}", area(&rect1));
 }
 
 fn area(rectangle: &Rectangle)->u32{
    rectangle.width * rectangle.height
 }
