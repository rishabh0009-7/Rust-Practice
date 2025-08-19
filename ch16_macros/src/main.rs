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




// Procedural macors - These are more complex macros that allow you to define custom behavior for code generation through Rust code itself. Procedural macros operate on Rust's Abstract Syntax Tree (AST) and are commonly used for things like deriving traits automatically or creating custom attributes.
// Procedural macros are more powerful:

// Work on the AST (Abstract Syntax Tree).

// Three types:

// Custom Derive Macros

// Attribute Macros

// Function-like Macros

// Normal macros (macro_rules!) = pattern matching → replace text.

// Procedural macros = functions that take Rust code (as tokens) → transform → return new Rust code.

// Think of them as code generators

// setup create 
// [lib]
// proc-macro = true   # <-- very important!

// [dependencies]
// syn = "2.0"   # parses Rust code
// quote = "1.0" # lets us generate Rust code


// proc-macro = true → tells Cargo this is a macro crate.

// syn → converts raw tokens into a Rust AST (Abstract Syntax Tree).

// quote! → converts Rust code back into tokens we can return.

// Token stream --> Raw rust tokens like text  but structured 

// 1- custom derive  macros 


#[proc_macro_derive(HelloMacro)] //define new drive macro 


fn hello_macro(input :TokenStream)->Token{


    // parse rust code --> raw token into ast 
    let ast:syn::DeriveInput = syn::parse(Input).unwrap();
    let name = &ast.ident;

// generate rust code 
    let gen = quote! {
        impl Hello_macro(){
            println!("hello  my name is {}", stringify!(#name))
        }
    }

gen.into()

}




#[derive(HelloMacro )]   //custom derived macro 


struct  Dog; 

trait HelloMacro{
    fn Hello_macro();
}



fn main(){
    Dog::Hello_macro();
}





