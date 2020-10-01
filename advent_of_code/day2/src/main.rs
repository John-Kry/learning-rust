use std::fs;

fn main() {
    let initial_numbers = read_file();

    loop {
        for noun in 0..100 {
            for verb in 0..100 {
                let mut curr_index = 0usize;
                let mut numbers = initial_numbers.clone();
                numbers[1] = noun;
                numbers[2] = verb;
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
                if numbers[0] == 19690720 {
                    println!("DONE! Correct number FOUND: {}", numbers[0]);
                    println!("DONE! Correct number FOUND: {}", 100 * noun + verb);
                    return;
                }
            }
        }
    }
}

fn read_file() -> Vec<u32> {
    let content = fs::read_to_string("./src/day2input").expect("Something went terribly wrong");
    let array_of_numbers = content.split(",");
    let vec: Vec<u32> = array_of_numbers
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    return vec;
}
