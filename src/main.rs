fn main(){
   let height1 = 10;
   let width1 = 20;

   println!("The area of the rectangle is :{}", area(width1,height1));
}

fn area(width: u32, height: u32)->u32{
   width*height
}
