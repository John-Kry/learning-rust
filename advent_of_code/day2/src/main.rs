use std::fs;

fn main() {
    let mut numbers = read_file();

    let mut curr_index = 0usize;
    numbers[1] = 12;
    numbers[2] = 2;
    loop {
        let op = numbers[curr_index];
        match op {
            1 => {
                let op1 = numbers[curr_index + 1];
                let op2 = numbers[curr_index + 2];
                let op3 = numbers[curr_index + 3];
                numbers[op3 as usize] = numbers[op1 as usize] + numbers[op2 as usize];
                curr_index += 4;
            }
            2 => {
                let op1 = numbers[curr_index + 1];
                let op2 = numbers[curr_index + 2];
                let op3 = numbers[curr_index + 3];
                numbers[op3 as usize] = numbers[op1 as usize] * numbers[op2 as usize];
                curr_index += 4;
            }
            99 => {
                break;
            }
            _ => unreachable!(),
        }
    }
    println!("{}", numbers[0])
}

fn read_file() -> Vec<u32> {
    let content = fs::read_to_string("./src/day2input").expect("Something went terribly wrong");
    let array_of_numbers = content.split(",");
    let vec: Vec<u32> = array_of_numbers
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    return vec;
}
