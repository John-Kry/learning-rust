fn is_divide_by(number: i32, a: i32, b: i32) -> bool {
    return (number % a).eq(&0) && (number % b).eq(&0);
}
fn main() {
    let correct = is_divide_by(45, 1, 5);
    if (correct == true) {
        println!("poggy");
    } else {
        println!("falsey");
    }
}
