use std::{fs, collections::HashMap};

fn main() {
    let content = fs::read_to_string("data.txt").unwrap();
    let mut sum = 0;
    let mut common: char = 'x';
    let mut key_map = HashMap::new();
    for line in content.lines() {
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
        println!("Left: {0}", left);
        println!("Right: {0}", right);
        println!("Char: {0}", common);

        println!("Digit: {0}", common as u32 - 'a' as u32 );
    }
}
