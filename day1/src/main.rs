use std::fs;
use std::collections::HashMap;

pub fn find_occurence(map: HashMap<i32, i32>, size: i32) -> i32 {
    let mut tmp = 0;
    let mut res = 0;
    while tmp != size {
        if map[&tmp] == map[&0] {
            tmp += 1;
            continue
        } else {
            if map[&tmp] > map[&(tmp - 1)] {
                res += 1;
            }
        }
        tmp += 1;
    }
    res
}

fn main() {
    let file_content = fs::read_to_string("day1.txt").unwrap();
    let lines: Vec<String> = file_content.split("\n")
        .map(|s: &str| s.to_string())
        .collect();
    let mut tab: HashMap<i32, i32> = HashMap::new();
    let mut count = 0;
    for line in lines {
        tab.insert(count, line.parse().unwrap());
        count += 1;
    }
    //part 1
    let res: i32 = find_occurence(tab.clone(), count);
    println!("part one: {}", res);

    // part 2
    let mut tmp = 2;
    let mut tab_of_3: HashMap<i32, i32> = HashMap::new();
    let mut cmp = 0;

    while tmp != count {
        let add = tab[&tmp] + tab[&(tmp - 1)] + tab[&(tmp - 2)];
        tab_of_3.insert(cmp, add);
        tmp += 1;
        cmp += 1;
    }
    let res: i32 = find_occurence(tab_of_3.clone(), cmp);
    println!("part two: {}", res);
}
