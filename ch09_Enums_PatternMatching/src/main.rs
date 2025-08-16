// enum Direction{
//     North,
//     South,
//     East ,
//     West
// }

// fn main(){
//     lt my_direction = Direction:North;
//     let new_direction = my_direction; // no error becoz direction is  a copy 

// }


// enums and patern matching



enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}

// Function to calculate area based on shape
fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(side_length) => side_length * side_length,
        Shape::Rectangle(width, height) => width * height,
    }
}

fn main() {
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(3.0);
    let rectangle = Shape::Rectangle(4.0, 5.0);

    // Calculate and print the areas
    println!("Area of circle: {}", calculate_area(circle));
    println!("Area of square: {}", calculate_area(square));
    println!("Area of rectangle: {}", calculate_area(rectangle));
}
