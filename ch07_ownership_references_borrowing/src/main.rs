// following code will not work 
// fn main(){
//     let str:String = String::from("hello world")
//     let len:String = get_length(str)
//     println!("{}", len)
// }

// fn get_length(str:String)-> String{
//     return str.len()
//     // str is no longer owner of hello world



// }


// Fix - references/borrowing

// fn main(){
//     let my_str:String = String::from("hello world");
//     let len = get_length(&my_str);
//     // len borrow the variable 
//     println!("{}", len);
// }

// fn get_length(s:&String) -> usize {
//       s.len()
    



// }



// Assignment 

// Goal: Write a function calculate_length that takes an immutable reference to a String and returns its length. Then call this function from main and print both the original String and its length.
// fn main(){
//     let my_str = String::from ("hello world");
//     let len = calculate_length(&my_str);
//     println!("{}", my_str);
//     println!("{}", len);
// }
// fn calculate_length(s:&String) ->usize{
//     s.len()
   
// }


// Goal: Write a function append_text that takes a mutable reference to a String and appends some text to it. For example, if the string is "Hello", the function could append ", World!".

fn main(){
    let  mut my_text = String::from("hello");
     append_text(&mut my_text);
    println!("{}", my_text);
}



fn append_text(s:&mut String){
    s.push_str("world");


}