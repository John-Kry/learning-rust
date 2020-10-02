use std::collections::HashMap;
use std::fs;

#[derive(Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
    steps: i32,
}
impl Coordinate {
    fn coord_to_string(self) -> String {
        let string: String = "x".to_owned() + &self.x.to_string() + "y" + &self.y.to_string();
        return string;
    }
}
fn main() {
    let directions = read_file();
    let mut curr_index = 0usize;
    let line_one = directions.0;
    let line_two = directions.1;
    let mut line_one_coords: Vec<Coordinate> = Vec::new();
    let mut current_coordinate = Coordinate {
        x: 0,
        y: 0,
        steps: 0,
    };
    let line_one_steps = 0;
    while curr_index < line_one.len() {
        // println!("{}", line_one[curr_index]);
        let mut segment_points: Vec<Coordinate> =
            calculate_change(line_one[curr_index].clone(), current_coordinate);
        println!("{}", line_one_steps);
        current_coordinate = *segment_points.last().unwrap();
        line_one_coords.append(&mut segment_points);
        curr_index = curr_index + 1;
    }

    let mut steps = i32::MAX;
    let mut curr_index = 0usize;
    let mut current_coordinate = Coordinate {
        x: 0,
        y: 0,
        steps: 0,
    };
    let mut line_two_coords: Vec<Coordinate> = Vec::new();
    while curr_index < line_two.len() {
        println!("{}", line_two[curr_index]);
        let mut segment_points: Vec<Coordinate> =
            calculate_change(line_two[curr_index].clone(), current_coordinate);
        current_coordinate = *segment_points.last().unwrap();
        line_two_coords.append(&mut segment_points);
        curr_index = curr_index + 1;
        // println!("curr x={}, curr y ={}", currx, curry);
    }
    let mut line_two_x_to_y: HashMap<String, i32> = HashMap::new();
    for line_two_coord in &line_two_coords {
        line_two_x_to_y.insert(line_two_coord.coord_to_string(), line_two_coord.steps);
    }
    println!("time to loop");
    for line_one_coord in &line_one_coords {
        match line_two_x_to_y.get(&line_one_coord.coord_to_string()) {
            Some(coordinate_steps) => {
                println!("MATCH: {},{}", line_one_coord.x, line_one_coord.y);
                let current_total_steps = *coordinate_steps + line_one_coord.steps;
                if current_total_steps < steps {
                    steps = current_total_steps;
                }
            }
            None => {}
        };
    }
    println!("lowest steps:{}", steps);
}

fn read_file() -> (Vec<String>, Vec<String>) {
    let content = fs::read_to_string("./src/day3input").expect("Something went terribly wrong");
    let two_lines: Vec<&str> = content.lines().collect();
    let line_one = two_lines[0].split(",");
    let line_two = two_lines[1].split(",");

    let line_one_vec: Vec<String> = line_one.map(|n| n.parse::<String>().unwrap()).collect();
    let line_two_vec: Vec<String> = line_two.map(|n| n.parse::<String>().unwrap()).collect();
    return (line_one_vec, line_two_vec);
}
fn calculate_change(input: String, mut coordinate: Coordinate) -> Vec<Coordinate> {
    let step_count: String = input[1..].to_string();
    let step_count = step_count.parse::<i32>().unwrap();
    let mut segment_points: Vec<Coordinate> = Vec::new();
    if input.starts_with("R") {
        for _i in 0..step_count {
            coordinate.x = coordinate.x + 1;
            coordinate.steps = coordinate.steps + 1;
            segment_points.push(coordinate);
        }
        return segment_points;
    } else if input.starts_with("L") {
        for _i in 0..step_count {
            coordinate.x = coordinate.x - 1;
            coordinate.steps = coordinate.steps + 1;
            segment_points.push(coordinate);
        }
        return segment_points;
    } else if input.starts_with("U") {
        for _i in 0..step_count {
            coordinate.y = coordinate.y + 1;
            coordinate.steps = coordinate.steps + 1;
            segment_points.push(coordinate);
        }
        return segment_points;
    } else {
        for _i in 0..step_count {
            coordinate.y = coordinate.y - 1;
            coordinate.steps = coordinate.steps + 1;
            segment_points.push(coordinate);
        }
        return segment_points;
    }
}
