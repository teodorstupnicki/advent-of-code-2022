use std::fs;

fn main() {
    let content = fs::read_to_string("data.txt").unwrap();
    let mut sum = 0;
    let mut v = Vec::new();
    for line in content.lines() {
        sum = if line.is_empty() {
            v.push(sum);
            0
        } else {
            sum + line.parse::<i32>().unwrap()
        };
    }
    v.sort();
    let max0 = v.pop().unwrap();
    let max1 = v.pop().unwrap();
    let max2 = v.pop().unwrap();
    println!("Sum of calories: {0}", max0 + max1 + max2);
}
