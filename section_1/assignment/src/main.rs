use std::vec;

fn task_1() {
    // Create three variables with the names: val1, val2, and ans. We want to perform a simple operation of generating the modulo of val1 and val2. Set val1 to 5 and val2 to 2. Assign the answer to the ans variable. Before executing your code, what do you think the answer will be?
    println!("Task 1:");
    let val1 = 5;
    let val2 = 2;
    let ans = val1 % val2;
    println!("{}", ans)
}

fn task_2() {
    // Create a vector and put in the values "2, 4, 6, 8, 10". Once you have created the vector perform the following: print out the current values, remove the value 10, add the value 12, and then print the vector back out to confirm your results.
    println!("task 2:");
    let mut vector = vec![2, 4, 6, 7, 10];
    println!("vector {:#?}", vector);
    vector[4] = 12;
    println!("vector {:?}", vector);
}

fn concat_string(mut word: String) -> String {
    word += " World";
    word
}

fn task_3() {
    // Create a function called "concat_string". Create a string variable and assign the value "Hello" to it. The function is going to take one argument that is of type string and is going to return a String. Inside this function, concatenate the string " World". Print out the results in main() to confirm your results.

    println!("Task 3: {}", concat_string("Hello".to_string()))
}

fn control_flow(flow_num: i64) {
    if flow_num == 1 {
        println!("this value is one")
    } else if flow_num > 50 {
        println!("this value is grater than 50")
    } else if flow_num < 25 {
        println!("this value is less than 25")
    } else {
        println!("The value is greater than 25 but less than 50")
    }
}

fn task_4() {
    // Create a function called control_flow. This is going to take one argument that is an integer. Based on this integer, print out the following: "The value is one", "The value is greater than 50", "The value is less than 25", or "The value is greater than 25 but less than 50".
    println!("Task 4:");
    control_flow(14);
}

fn main() {
    task_1();
    task_2();
    task_3();
    task_4();
}
