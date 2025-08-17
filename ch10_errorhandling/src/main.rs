// in js --> for error handling we have try catch 

//  enum Result <T,E>{
//     Ok(T)
// ,
// Err(E),
//  }

use std::fs;
fn main(){
    let greeting_result:Result<String, std::io::Error> = fs::read_to_string("hello.txt");


    match greeting_result{
        Ok(file_content )=>{
            println!("file created sucessfully {:?}" , file_content)
        },
        Err(error)=>{
            println!("error {:?}", error)
        }
    }
}

