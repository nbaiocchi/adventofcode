use std::fs;

const FILE_PATH: &str = "test.txt";

fn first_part(pos: &[i32]) {
    pos.to_vec().sort();
    let align_pos = pos[pos.len() / 2];
    let mut res = 0;

    for p in pos {
        if p > &align_pos  {
            res += p - align_pos;
        } else {
            res += align_pos - p;
        }
    }

    println!("{}", res)
}

fn main() {
    let pos: Vec<i32> = fs::read_to_string(FILE_PATH)
        .unwrap()
        .split(',')
        .filter_map(|str| str.parse().ok())
        .collect();

    first_part(&pos);
}
