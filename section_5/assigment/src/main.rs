// Create an enum called Shape and provide the values of "triangle square, pentagon, octagon".  Then create a method for this enum that returns the number of corners each shape has based on the type of shape.

// Example:

// triangle.corners() will return 3

// square.corners() will return 4

enum Shape {
    Triangle,
    Square,
    Pentagon,
    Octagon,
}

impl Shape {
    fn corners(&self) -> i8 {
        let to_return = match &self {
            Shape::Triangle => 3,
            Shape::Square => 4,
            Shape::Pentagon => 5,
            Shape::Octagon => 8,
        };
        return to_return;
    }
}

fn main() {
    println!("Hello, world!");
    println!("Shape {}", Shape::Triangle.corners());
    println!("Shape {}", Shape::Square.corners());
    println!("Shape {}", Shape::Pentagon.corners());
    println!("Shape {}", Shape::Octagon.corners());
}
