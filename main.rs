
fn tabuada() {
    let multiplyer = 5;
    let choosed_number = 10;

    for i in 1..=choosed_number {
        println!("{} x {} = {}", i, multiplyer, i*multiplyer)
    }
}

fn match_statment() {
    let lang = "Java";
    let use_case = match lang {
        "Python" => "Data Science",
        "Scala" => "Data Enginnering",
        _ => "General case"
    };
    println!("the {} is for {}", lang, use_case);
}

fn main() {

    let idade: u8 = 17;
    let allow_enter = if idade > 18 {"allowed"} else { "not allowed "};
    println!("you are {}", allow_enter);
    tabuada();
    match_statment();
}