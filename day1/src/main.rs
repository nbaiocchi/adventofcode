use std::fs;
use std::collections::HashMap;

fn main() {
    let file_content = fs::read_to_string("day1.txt").unwrap();
    let lines: Vec<String> = file_content.split("\n")
        .map(|s: &str| s.to_string())
        .collect();
    let mut tab: HashMap<i32, i32> = HashMap::new();
    let mut count = 0;
    let mut tmp = 0;
    let mut res: i32 = 0;
    for line in lines {
        tab.insert(count, line.parse().unwrap());
        count += 1;
    }
    //part 1
    while tmp != count {
        if tab[&tmp] == tab[&0] {
            tmp += 1;
            continue
        } else {
            if tab[&tmp] > tab[&(tmp - 1)] {
                res += 1;
            }
        }
        tmp += 1;
    }
    println!("part one: {}", res);
    tmp = 2;
    let mut tab_of_3: HashMap<i32, i32> = HashMap::new();
    let mut cmp = 0;
    //count -= 2;
    // part 2
    while tmp != count {
        let add = tab[&tmp] + tab[&(tmp - 1)] + tab[&(tmp - 2)];
        tab_of_3.insert(cmp, add);
        tmp += 1;
        cmp += 1;
    }
    tmp = 0;
    res = 0;
    while tmp != cmp {
        if tab_of_3[&tmp] == tab_of_3[&0] {
            tmp += 1;
            continue
        } else {
            if tab_of_3[&tmp] > tab_of_3[&(tmp - 1)] {
                res += 1;
            }
        }
        tmp += 1;
    }
    println!("part two: {}", res);
}
