// Macros in Rust let you:

// Generate repetitive code (avoid boilerplate).

// Write code that writes code.

// Enable domain-specific languages (DSLs).

// Power many famous crates (serde, tokio, clap).

// There are two kinds:

// Declarative macros (macro_rules!) → pattern matching.

// Procedural macros → functions that operate on Rust code (AST).





// Declarative macros 

// 1-
// macro_rules! say_hello{
//     ()=>{
//         println!("hello everyone ")
//     };
// }


// fn main (){
//     say_hello!();  //expands to println!("hello everyone ")
// }




// 2-macros with  arguments 

// macro_rules! square{
//     ($x:expr)=>{
//         $x*$x

//     };
// }



// fn main (){
//     println!("{}", square!(5))
// }



// 3-Definiing a creation function macros 

macro_rules! create_function{
    ($func_name:ident)=>{
        fn $func_name(){
            println!("Hello from {}" ,stringify!($func_name));

        }

    };
}


create_function!(hello);


fn main (){
    hello ();
}



