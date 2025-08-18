// generic means writing code that works on any type 



//generic without trait bound 
// fn first_element<T>(v1: &Vec<T>) -> Option<&T> {
    
//     v1.get(0)
// }

// fn main() {
//     let v1 = vec![1, 2, 3];
//     let v2 = vec![String::from("Rizz"), String::from("Raman")];
//     let v3 = vec![1.0, 2.0, 3.0];

//     println!("{}", first_element(&v1).unwrap());
//     println!("{}", first_element(&v2).unwrap());
//     println!("{}", first_element(&v3).unwrap());
// }



// trait bound --> A trait bound tells the compiler:

//"This generic type T must have certain abilities (traits) so I can use them." 

// fn biggest_element<T: PartialOrd>(a: T, b: T) -> T {
//     if a > b {
//         return a;
//     }
//     return b;
// }

// fn main() {
//     println!("{}", biggest_element(10, 20));         // works with i32
//     println!("{}", biggest_element(3.5, 2.1));       // works with f64
//     println!("{}", biggest_element('a', 'z'));       // works with char
//     println!("{}", biggest_element("Rust", "Go"));   // works with &str
// }



// generic over structs 

// without generic 


// struct Rect{
//     width :u32,
//     height:u32,

// }

// impl Rect{
//     fn area(&self)->u32{
//         self.width*self.height



//     }
// }

// fn main(){
//     let r = Rect{
//         width:20,
//         height:30,

//     };

//     println!("{}" , r.area());
// }


// with generic 

// use std::ops::Mul;

// #[derive(Debug)]
// struct Rect<T> {
//     width: T,
//     height: T,
// }

// impl<T: Mul<Output = T> + Copy> Rect<T> {
//     fn area(&self) -> T {
//         self.width * self.height
//     }
// }

// fn main() {
//     let r = Rect {
//         width: 20,
//         height: 30,
//     };

//     println!("Area = {}", r.area());
// }


// generic over enums 


// enum MyOption<T> {
//     Some(T),  // holds a value of type T
//     None,     // no value
// }

// fn main() {
//     let a: MyOption<i32> = MyOption::Some(42);
//     let b: MyOption<i32> = MyOption::None;

//     match a {
//         MyOption::Some(v) => println!("Got {}", v),
//         MyOption::None => println!("Got nothing"),
//     }

//     match b {
//         MyOption::Some(v) => println!("Got {}", v),
//         MyOption::None => println!("Got nothing"),
//     }
// }




enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let success: MyResult<i32, &str> = MyResult::Ok(200);
    let failure: MyResult<i32, &str> = MyResult::Err("Something went wrong");

    match success {
        MyResult::Ok(v) => println!("Success: {}", v),
        MyResult::Err(e) => println!("Error: {}", e),
    }

    match failure {
        MyResult::Ok(v) => println!("Success: {}", v),
        MyResult::Err(e) => println!("Error: {}", e),
    }
}

