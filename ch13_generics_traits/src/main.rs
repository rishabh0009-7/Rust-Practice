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

fn biggest_element<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        return a;
    }
    return b;
}

fn main() {
    println!("{}", biggest_element(10, 20));         // works with i32
    println!("{}", biggest_element(3.5, 2.1));       // works with f64
    println!("{}", biggest_element('a', 'z'));       // works with char
    println!("{}", biggest_element("Rust", "Go"));   // works with &str
}
