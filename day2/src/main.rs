use std::fs;

fn main() {
    let file_content = fs::read_to_string("day2.txt").unwrap();
    let lines: Vec<String> = file_content.split('\n')
        .map(|s: &str| s.to_string())
        .collect();
    //let mut map: HashMap<String, i32> = HashMap::new();
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;
    for line in lines {
        let values: Vec<&str> = line.split(' ').collect();
        let digit: i32 = values[1].parse().unwrap();
        match values[0] {
            "forward" => {
                horizontal += digit;
                depth += aim * digit;
            }
            "down" => {
                aim += digit;
            }
            "up" => {
                aim -= digit;
            }
            _ => continue,
        }
    }
    println!("horizontal => {}\ndepth => {}", horizontal, depth);
    println!("res => {}", horizontal*depth);
}
