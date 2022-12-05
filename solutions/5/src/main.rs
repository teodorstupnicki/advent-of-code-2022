use std::fs;

fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();
    let mut n=0;
    for line in contents.lines() {
        if line.contains("1") {
            let it = line.split("   ");
            n = it.last().unwrap().trim().parse::<i32>().unwrap();
            break;
        }
    }
    println!("n = {0}", n);
    
}
