// Traits are similar to interfaces in Java/JS. They let you define the shape / interface of what you’re building


trait Shape {
    fn area (&self)->f32;
}


struct Rect {
    width:f32,
    height:f32,
}


impl Shape for Rect{
    fn area(&self)->f32{
        return self.width * self.height
    }
} 



fn main() {
    let r = Rect{
        width:30.0,
        height:50.0,
    };
    println!("{}", r.area());


}
