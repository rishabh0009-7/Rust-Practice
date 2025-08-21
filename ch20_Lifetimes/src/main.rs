// Lifetime --> Rust doesn’t have a garbage collector. Instead, it uses ownership and borrowing to ensure memory safety.
// A lifetime is the scope for which a reference is valid.

// (no explicit lifetime needed yet)
// fn main(){
//     let x = 6;
//     let y = &x;  // y borrows x 
//     println!("{}", y )
// }



// dangling references 

// fn main (){
//     let r;
//     {
//         let x = 5;
//         r = &x ;   // `r` borrows `x`
//     } // x goes out fo scope so r becomes invalidf 
//     println!("{}", r); // invalid 
// }

// Basic Lifetime Annotations

// When functions take references, Rust sometimes cannot infer how long references should live.
// We annotate with 'a (a lifetime parameter).
 fn lonegest(s1:&'a str , s2:& 'a str) {
    if s1.len() > s2.len() {
        s1
    } else{
        s2
    }

 }


fn main(){
    let s1 = String::from("hello ");
    let s2  = "world"

    let result = lonegest(s1.as_str() , s2)
    println!("{}" , result)
}