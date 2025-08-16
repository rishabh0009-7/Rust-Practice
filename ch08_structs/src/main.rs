// struct User {
//     email:String,
//     username:String

// }

// fn main(){
//     let user1 = User{
//         email:String::from("rizz@gmail.com"),
//         username:String::from("rizz")
//     };

//     println!("{:?}", user1.username);

// }

// only stack types in struct  

// struct User {
//     active: bool,
//     sign_in_count: u64,
// }

// fn main() {
//     let mut user1 = User {
//         active: true,
//         sign_in_count: 1,
//     };

//     name(user1);
//     print!("User 1 username: {}", user1.active); // Error - can not use borrowed value
// }

// fn name(user1: User) {
//     print!("User 1 username: {}", user1.active);
// }



// Add the copy trait
// #[derive(Copy,Clone)]
// struct User {
//     active: bool,
//     sign_in_count: u64,
// }

// fn main() {
//     let mut user1 = User {
//         active: true,
//         sign_in_count: 1,
//     };

//     name(user1);
//     print!("User 1 username: {}", user1.active); // Error goes away because user1 is copied
// }

// fn name(user1: User) {
//     print!("User 1 username: {}", user1.active);
// }




// Add strings

// struct User{
//     active:bool, 
//     sign_in_count:u64,
//     username : String
// }



// fn main(){
//     let mut user1 = User {
//         active:true ,
//         sign_in_count :1,
//         username:String::from("raa")
//     }

//     name(user1);
//     print!("User 1 username: {}", user1.username); //Error - can not use borrowed value

    
// }


// fn name(user1:User){
//     print!("User 1 username: {}", user1.active);

// }


// add the trait clone  so now  it is clone and it will work 
// in strng you cant use copy trait so use clone 
// #[derive(Clone)]
// struct User{
//     active:bool, 
//     sign_in_count:u64,
//     username : String
// }



// fn main(){
//     let mut user1 = User {
//         active:true ,
//         sign_in_count :1,
//         username:String::from("raa")
//     };

//     name(user1.clone());
//     print!("User 1 username: {}", user1.username); //Error - can not use borrowed value

    
// }


// fn name(user1:User){
//     print!("User 1 username: {}", user1.active);

// }



// implementing struct 
// Passing in &self as the first argument to a function


struct Rect{
    width:u32,
    height:u32,
}

impl Rect {
    fn area (&self)->u32{
        self.width * self.height
    }
}


fn main(){
    let Rect1 = Rect {
        width : 30,
        height : 50
    };


    println!(" The area of rectangle is {}" , Rect1.area());

}