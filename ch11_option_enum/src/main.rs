//  in rust there is no concept of returning null 
// The Option enum was introduced in Rust to handle the concept of nullability in a safe and expressive way.
// rust doesnt have null
// enum Option<T>{
//     None, 
//     Some(T),
// }


// example 


fn find_first_a (s:String)->Option<i32>{

    for(index, character ) in s.chars().enumerate(){
        if character  == 'a'{
            return Some(index as i32 );
            
        }
        
    }
     None 
    
}
fn main (){
    let my_string = String::from("rizzcode");
    match find_first_a(my_string) {
        Some(index) => println!("The letter 'a' is found at index: {}", index),
        None => println!("The letter 'a' is not found in the string."),
    }


}