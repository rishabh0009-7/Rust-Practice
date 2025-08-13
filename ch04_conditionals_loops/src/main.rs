// conditionals
// fn main() {
//     let x: i32 = 99;
//     let is_even: bool = is_even(x);

//     if is_even {
//         println!("{} is even", x);
//     } else {
//         println!("{} is odd", x);
//     }
// }

// fn is_even(x: i32) -> bool {
//     x % 2 == 0
// }


// loops


fn main() {
    let str = String::from("hello everyone");
    println!("first name: {}", get_first_name(str));
}

fn get_first_name(str: String) -> String {
    let mut first_name = String::new();

    for c in str.chars() {
        if c == ' ' {
            break;
        }
        first_name.push(c);
    }

    first_name
}
