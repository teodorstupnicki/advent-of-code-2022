use std::fs;

fn main() {
    let content = fs::read_to_string("data.txt").unwrap();
    let chars: Vec<char> = content.chars().collect();
    for i in 13..chars.len() {
        let mut flag = true;
        for j in 0..14{
            for k in 0..14 {
                if j != k && chars[i-j] == chars[i-k] {
                    flag = false;
                }

            }
        }
        if flag {
            println!("{0}", i+1);
            break;
        }
    }
    
}
