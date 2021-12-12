use std::fs;

const FILE_PATH: &str = "day3.txt";

fn first_part(vec: &[Vec<u32>]) {
    let length = vec.first().unwrap().len();
    let mut test: Vec<(u32, u32)> = Vec::new();
    // (0..length).fold((), |acc, len| 
    //     acc.push(vec.iter()
    //         .filter_map(|bits| bits.get(len))
    //         .fold((0, 0), |(acc_0, acc_1), bit| match bit {
    //             0 => (acc_0 + 1, acc_1),
    //             1 => (acc_0, acc_1 + 1),
    //             _ => (acc_0, acc_1),
    //         }));
    // );

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
    println!("gamma: {} = {} --- epsilon: {} = {} --- power: {}", first, gamma, second, epsilon, gamma * epsilon);
}

fn is_one(vec: &[Vec<u32>], len: usize) -> bool {
    let tmp = vec.iter()
    .filter_map(|bits| bits.get(len))
    .fold((0, 0), |(acc_0, acc_1), bit| match bit {
        0 => (acc_0 + 1, acc_1),
        1 => (acc_0, acc_1 + 1),
        _ => (acc_0, acc_1),
    });
    if tmp.0 > tmp.1 {
        false
    } else {
        true
    }
}

fn depush(vec: &[Vec<u32>], digit: u32, len: usize) -> Vec<Vec<u32>> {
    let mut res: Vec<Vec<u32>> = Vec::new();

    for bits in vec {
        if bits[len] == digit {
            res.push(bits.to_vec());
        }
    }
    res
}

fn last(vec: &[Vec<u32>]) -> bool {
    let mut count = 0;
    for _ in vec {
         count += 1;
    }
    match count {
        1 => return true,
        _ => return false,
    }
}

fn push(vec: &[Vec<u32>]) -> Vec<u32> {
    let mut res: Vec<u32> = Vec::new();
    for n in vec {
        res = n.to_vec();
    }
    res
}

fn vec_to_string(vec: Vec<u32>) -> String {
    let mut res = String::new();
    for bit in vec {
        if bit == 0 {
            res.push('0');
        } else {
            res.push('1');
        }
    }
    res
}

fn second_part(mut vec: Vec<Vec<u32>>) {
    let length = vec.first().unwrap().len();
    let mut oxygen: Vec<u32> = Vec::new();
    let mut co2: Vec<u32> = Vec::new();
    let mut copy = vec.clone();

    for n in 0..length {
        if last(&vec) == true {
            oxygen = push(&vec);
            break
        }
        if is_one(&vec, n) == true {
            oxygen.push(1);
            vec = depush(&vec, 1, n);
        } else {
            oxygen.push(0);
            vec = depush(&vec, 0, n);
        }
    }
    for n in 0..length {
        if last(&copy) == true {
            co2 = push(&copy);
            break
        }
        if is_one(&copy, n) == true {
            co2.push(0);
            copy = depush(&copy, 0, n);
        } else {
            co2.push(1);
            copy = depush(&copy, 1, n);
        }
    }

    let first = vec_to_string(oxygen);
    let second = vec_to_string(co2);
    let oxygen = u32::from_str_radix(&first, 2).unwrap();
    let co2 = u32::from_str_radix(&second, 2).unwrap();
    println!("oxygen: {} -- co2: {} -- life suport: {}", oxygen, co2, oxygen * co2);
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

    first_part(&bits);
    second_part(bits);
}
