use std::fs;

const FILE_PATH: &str = "day3.txt";

fn first_part(vec: &[Vec<u32>]) {
    let length = vec.first().unwrap().len();
    let mut test: Vec<(u32, u32)> = Vec::new();
    //let test = (i..length).iter().filter_map(||)

    for n in 0..length {
        let tmp = vec.iter()
            .filter_map(|bits| bits.get(n))
            .fold((0, 0), |(acc_0, acc_1), bit| match bit {
                0 => (acc_0 + 1, acc_1),
                1 => (acc_0, acc_1 + 1),
                _ => (acc_0, acc_1),
            });
        test.push(tmp);
    }
    let mut first = String::new();
    let mut second = String::new();
    for bit in test {
        if bit.0 > bit.1 {
            first.push('0');
            second.push('1');
        } else {
            first.push('1');
            second.push('0');
        }
    }
    let gamma = u32::from_str_radix(&first, 2).unwrap();
    let epsilon = u32::from_str_radix(&second, 2).unwrap();
    println!("gamma: {} = {} --- epsilon: {} = {} --- power consuption: {}", first, gamma, second, epsilon, gamma * epsilon);
}

fn main() {
    let bits: Vec<Vec<u32>> = fs::read_to_string(FILE_PATH)
        .unwrap()
        .split('\n')
        .map(|str| {
            str.chars()
                .filter_map(|char| char.to_string().parse().ok())
                .collect()
        })
        .collect();

    let _ = first_part(&bits);
}
