// Rust doesn’t have “decorators” like Python/JS, but it has attributes and procedural macros, which together cover annotations + decorators.

#[derive(Debug)] // annotation
struct User {
    name: String,
    age: u32,
}

fn main() {
    let u = User { name: "Abhinav".to_string(), age: 21 };
    println!("{:?}", u);
}
