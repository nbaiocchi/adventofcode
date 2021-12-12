use std::fs;

const FILE_PATH: &str = "day6.txt";

fn the_daily_life_of_the_immortal_lanternfish(mut vec: Vec<u64>, day: i32) -> u64{
    for _ in 1..=day {
        vec.rotate_left(1);
        vec[6] += vec[8];
    }
    let res: u64 = vec.iter().sum();
    res
}

fn main() {
    let file_content: Vec<i32> = fs::read_to_string(FILE_PATH)
    .unwrap()
    .split(',')
    .filter_map(|str| str.parse().ok())
    .collect();

    let mut vec: Vec<u64> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
    for fish in file_content {
        match fish  {
            1 => vec[1] += 1,
            2 => vec[2] += 1,
            3 => vec[3] += 1,
            4 => vec[4] += 1,
            5 => vec[5] += 1,
            6 => vec[6] += 1,
            _ => continue,
        }
    }
    let one = the_daily_life_of_the_immortal_lanternfish(vec.clone(), 80);
    let two = the_daily_life_of_the_immortal_lanternfish(vec, 256);
    println!("part one => {}\npart two => {}", one, two)
}
