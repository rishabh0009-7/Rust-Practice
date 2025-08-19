use ch17_macros_procedural::HelloMacro;  // import the derive macro

// The trait definition must exist in scope
trait HelloMacro {
    fn hello_macro();
}

#[derive(HelloMacro)]
struct Dog;

fn main() {
    Dog::hello_macro();
}
