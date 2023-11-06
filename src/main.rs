fn main(){
    //refactoring with tuples
    let rect1 =(10,20);
    println!("The area of the rectangle is :{}", area(rect1));
 }
 
 fn area(dimensions: (u32, u32))->u32{
    dimensions.0 * dimensions.1
 }
