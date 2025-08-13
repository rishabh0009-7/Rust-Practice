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

fn main(){
    let my_str:String = String::from("hello world");
    let len = get_length(&my_str);
    // len borrow the variable 
    println!("{}", len);
}

fn get_length(s:&String) -> usize {
      s.len()
    



}

