// custom derived macros 

// struct User{
//     name :String ,
//     age :u32 , 
// }


// fn main(){
//     let u = User{
//         name :String::from("rzz"),
//         age:22
//     };
//     println!("{}", u)  // it will not get print coz println  use  formatting traits  to decide how to print things

// }



// #[derive(Debug)]
// struct User{
//     name :String ,
//     age :u32 , 
// }


// fn main(){
//     let u = User{
//         name :String::from("rzz"),
//         age:22
//     };
//     println!("{:?}", u)  

// }



#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn main(){
    let p1 = Point {
        x: 1,
         y: 2 
       };
   let p2 = p1; // p1 is copied, not moved
   println!("{}, {}", p1.x, p2.x);
}




