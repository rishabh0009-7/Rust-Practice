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
//  fn lonegest<'a>(s1:&'a str , s2:& 'a str) -> &'a str{
//     if s1.len() > s2.len() {
//         s1
//     } else{
//         s2
//     }

//  }


// fn main(){
//     let s1 = String::from("hello ");
//     let s2  = "world";

//     let result = lonegest(s1.as_str() , s2);
//     println!("{}" , result);
// }


// Lifetime Elision (when you don’t need 'a)
// if there is only input reference there is no need to write  'a


// fn first_word (s:&str) -> &str{
//     &s[..1]


// }

// fn main (){
//     let s = String::from ("hello")
//     let result = first_word(&s);
//     println!("{}" , result)
// }


// Structs with Lifetimes

// If a struct holds references, it must carry lifetime annotations.

// struct Book <'a>{
//     title:&'a str,
//     author: &'a str,



// }

// fn main(){
//     let title = String::from("Rust in action ");
//     let author = String::from ("Tim McNamara");

//     let book = Book{
//         title : &title ,
//         author : &author,
//     };

//     println!("{} by {}" , book.title , book.author);


    
// }




// Multiple Lifetimes

// You can have multiple lifetime parameters if references are unrelated.


// fn mix <'a,'b>( x:&'a str , y :&'b str) -> (&'a str , &'b str) {
//     (x,y)


// }
// fn main (){
//     let s1 = String::from ("hello ");
//     let s2 = String::from ("world");

//     let (a, b )= mix(&s1 , &s2) ;
//     println!( "a = {} , b= {}" , a,b);


// }



// Lifetimes in Methods

// When implementing methods, annotate lifetimes consistently.


// struct Sentence <'a>{
//     text:&'a str,

// }

// impl <'a> Sentence <'a>{
//     fn get_text(&self) -> &'a str {
//         self.text

//     }

// }


// fn main(){
//     let text = String::from("hello");
//     let s = Sentence{text:&text} ;
//     println!("{}" , s.get_text());

// }



// Static Lifetime

// 'static means a reference lives for the entire duration of the program.

fn static_str () -> &'static str {
    "i live forever "

}

fn main(){
    let s = static_str();
    println!("{}" , s)
}






