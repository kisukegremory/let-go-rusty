// Modify the solution to the Section 4 assignment by creating a Trait that has the set_mpg, set_color, and set_top_speed methods. Then create a Motorcycle struct with the same fields as the Car struct: mpg, color, and top_speed. Now implement your Trait on both the Car and Motorcycle struct. Print out the results to confirm a working solution.

use std::fmt::Debug;

trait Engine {
    fn set_color(&mut self, color: String);
    fn set_mpg(&mut self, mpg: i32);
    fn set_top_speed(&mut self, top_speed: i32);
}

#[derive(Debug)]
struct Car {
    mpg: i32,
    color: String,
    top_speed: i32,
}

#[derive(Debug)]
struct Motorcycle {
    mpg: i32,
    color: String,
    top_speed: i32,
}

impl Engine for Car {
    fn set_mpg(&mut self, mpg: i32) {
        self.mpg = mpg;
    }
    fn set_color(&mut self, color: String) {
        self.color = color;
    }
    fn set_top_speed(&mut self, top_speed: i32) {
        self.top_speed = top_speed;
    }
}

impl Engine for Motorcycle {
    fn set_mpg(&mut self, mpg: i32) {
        self.mpg = mpg;
    }
    fn set_color(&mut self, color: String) {
        self.color = color;
    }
    fn set_top_speed(&mut self, top_speed: i32) {
        self.top_speed = top_speed;
    }
}

// Create a simple print function that uses Generic T. This Generic T will need to implement std::fmt::Debug depending on the values you pass in. Our function takes one parameter of type T. Our function will then print out the value that is passed in.

fn new_printer<T: Debug>(item: T) {
    println!("value - {:?}", item)
}

fn main() {
    let mut car = Car {
        mpg: 1,
        color: String::from("Vermelho"),
        top_speed: 10,
    };
    car.set_color("blue".to_string());
    car.set_mpg(10);
    car.set_top_speed(250);

    new_printer(car);

    let mut moto = Motorcycle {
        mpg: 2,
        color: String::from("Red"),
        top_speed: 100,
    };
    moto.set_color("blue".to_string());
    moto.set_mpg(4);
    moto.set_top_speed(250);

    new_printer(moto);
}
