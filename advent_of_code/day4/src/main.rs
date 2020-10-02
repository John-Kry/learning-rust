fn main() {
    const MINIMUM: i32 = 172851;
    const MAXIMUM: i32 = 675869;
    let mut count = 0;
    for number in MINIMUM..MAXIMUM {
        let number_as_string = number.to_string();

        if is_valid_number(number_as_string) == true {
            count = count + 1;
        }
    }
    println!("Count: {}", count)
}
fn is_valid_number(number: String) -> bool {
    let characters: Vec<char> = number.chars().collect();
    let mut previous_number = 0;
    let mut adjacent_characters = false;
    for curr_char in characters {
        if curr_char.to_digit(10).unwrap() == previous_number {
            adjacent_characters = true;
        }
        if curr_char.to_digit(10).unwrap() < previous_number {
            return false;
        }
        previous_number = curr_char.to_digit(10).unwrap();
    }
    if adjacent_characters == false {
        return false;
    }
    return true;
}
