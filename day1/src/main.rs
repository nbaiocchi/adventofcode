use std::fs;

const FILE_PATH: &str = "day1.txt";

pub fn find_occurence(map: Vec<i32>, lenth: usize) -> i32 {
    let mut res = 0;
    for n in 0..lenth {
        if map[n] != map[0] && map[n] > map[n-1] {
            res += 1;
        }
    }
    res
}

fn main() {
    let file_content: Vec<i32> = fs::read_to_string(FILE_PATH)
        .unwrap()
        .split('\n')
        .filter_map(|str| str.parse().ok())
        .collect();

    //part 1
    let res: i32 = find_occurence(file_content.clone(), file_content.len());
    println!("part one: {}", res);

    // part 2
    let mut vec: Vec<i32> = Vec::new();
    for tmp in 2..file_content.len() {
        let add = file_content[tmp] + file_content[tmp - 1] + file_content[tmp - 2];
        vec.push(add);
    }
    let res: i32 = find_occurence(vec.clone(), vec.len());
    println!("part two: {}", res);
}
