fn main() {
    let list: Vec<i8> = (0..5).rev().collect();
    for element in list {
        println!("element {}", element)
    }
}
