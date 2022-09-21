// Create a struct called Car with the fields: mpg, color, and top_speed. Once the struct is created, implement the following methods: set_mpg, set_color, and set_top_speed. Once you have created these methods, create a car, provide it default values, and then set the mpg, color, and top speed and then print them out.

struct Car {
    mpg: i32,
    color: String,
    top_speed: i32,
}

impl Car {
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

fn main() {
    let mut car = Car {
        mpg: 1,
        color: String::from("Vermelho"),
        top_speed: 10,
    };
    car.set_color("blue".to_string());
    car.set_mpg(10);
    car.set_top_speed(250);

    println!(
        "mpg: {}, color: {}, top_speed: {}",
        car.mpg, car.color, car.top_speed
    )
}
