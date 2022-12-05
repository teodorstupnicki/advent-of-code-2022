use std::{fs, collections::HashMap};

fn main() {
    let content = fs::read_to_string("data.txt").unwrap();
    let mut sum = 0;
    let mut common: char = 'x';
    let mut key_map = HashMap::new();
    for (i, line) in content.lines().enumerate() {
        find_commons(&mut key_map, line, &mut sum)
    }
    println!("Sum: {0}", sum);
}

fn find_commons(key_map: &mut HashMap<char, i32>, line: &str, sum: &mut i32) {
    let mut common: char = 'x';
    let length = line.len();
        let left = &line[..length/2];
        let right = &line[length/2..length];
        for c in left.chars() {
            key_map.insert(c, 2);
        }
        for r in right.chars() {
            let value = key_map.get(&r);
            match value {
                Some(v) => if *v == 2 {
                    common = r;
                    break;
                },
                None => {
                    key_map.insert(r, 1);
                },
            }
        }
        key_map.clear();
        println!("Char: {0}", common);
        // println!("Index: {0}", i);
        if common.is_uppercase() {
            *sum += (common as u32 - 'A' as u32) as i32 + 27;
            println!("Digit: {0}", common as u32 - 'A' as u32 + 27);
        } else {
            *sum += (common as u32 - 'a' as u32) as i32 + 1;
            println!("Digit: {0}", common as u32 - 'a' as u32 + 1);
        }
}