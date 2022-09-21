fn task_1() {
    // Create a function that takes one argument called val that is of type Vec with the values: 1,3,5,7. Inside of this function check the first value of the vector and see if it is equal to one. If the value is equal to one, then return true, otherwise return false. Next add the value 15 to the vector. Print out the vector to confirm your results.
    let mut val = vec![1, 3, 5, 7];
    func(&val);
    val.push(15);
    println!("{:?}", val)
}

fn func(val: &Vec<i32>) -> bool {
    if val[0] == 1 {
        true
    } else {
        false
    }
}

fn add_two(mut _val: i8) {
    _val = _val + 2;
}

fn task_2() {
    // Create a function called "add_two". This function is going to take one parameter that is i8 and add two to it. For the function, do you have to pass the value by reference in order for you to maintain ownership of it inside of main?
    let val: i8 = 4;
    add_two(val);
    println!("new value {}", val)
}

fn main() {
    println!("Hello, world!");
    task_1();
    task_2();
}
