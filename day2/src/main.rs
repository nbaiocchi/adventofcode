use std::fs;

pub fn first_part(lines: &Vec<String>) -> i32 {
    let (mut horizontal, mut depth): (i32, i32) = (0, 0);

    for line in lines {
        let values: Vec<&str> = line.split(' ').collect();
        let digit: i32 = values[1].parse().unwrap();
        match values[0] {
            "forward" => horizontal += digit,
            "down" => depth += digit,
            "up" => depth -= digit,
            _ => continue,
        }
    }
    horizontal * depth
}

pub fn second_part(lines: &Vec<String>) -> i32 {
    let (mut horizontal, mut depth, mut aim): (i32, i32, i32) = (0, 0, 0);

    for line in lines {
        let values: Vec<&str> = line.split(' ').collect();
        let digit: i32 = values[1].parse().unwrap();
        match values[0] {
            "forward" => {
                horizontal += digit;
                depth += aim * digit;
            }
            "down" => aim += digit,
            "up" => aim -= digit,
            _ => continue,
        }
    }
    horizontal * depth
}

fn main() {
    let lines: Vec<String>  = fs::read_to_string("day2.txt")
        .unwrap()
        .split("\n")
        .map(|s: &str| s.to_string())
        .collect();

    let res = first_part(&lines);
    println!("res => {}", res);
    let res = second_part(&lines);
    println!("res => {}", res);
}