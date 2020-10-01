use std::fs;

fn main() {
    let numbers = read_file();
    let mut result = 0;
    for number in numbers {
        let number_as_i32 = number.parse::<i32>().unwrap();
        result = result + calculate_fuel(number_as_i32, 0);
    }
    println!("{}", result);
}

fn calculate_fuel(mass: i32, fuel_used_so_far: i32) -> i32 {
    let mass_divided: f32 = (mass / 3) as f32;
    let fuel_cost = (mass_divided.floor()) as i32 - 2;
    if fuel_cost <= 0 {
        return fuel_used_so_far;
    } else {
        return calculate_fuel(fuel_cost, fuel_cost + fuel_used_so_far);
    }
}
fn read_file() -> Vec<String> {
    let content = fs::read_to_string("./src/day1input").expect("Something went terribly wrong");
    let array_of_numbers = content.lines();
    let vec: Vec<String> = array_of_numbers.map(String::from).collect();
    return vec;
}
