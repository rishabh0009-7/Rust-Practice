// the following code will not compile
// immutable  

// fn main(){
//     let x:i32 = 5;
//     println!("the value of x is :{}", x);
//     x:i32 = 6 ;
//     println!("the value of x is :{}", x);
// }


// mutable 

fn main(){

let mut x:i32 = 8;
println!("the value of x is :{}", x);
let x = 9;
println!("the value of x is :{}", x);


}