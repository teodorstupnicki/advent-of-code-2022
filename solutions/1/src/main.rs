use std::fs;

fn main() {
    let content = fs::read_to_string("data.txt").unwrap();
    let (mut max0 , mut max1, mut max2) = (0, 0, 0);
    let mut sum = 0;
    for line in content.lines() {
        sum = if line.is_empty() {
            if sum > max0 {
                max0 = sum;
            } else if sum > max1 {
                max1 = sum;
            } else if sum > max2 {
                max2 = sum;
            }
            0
        } else {
            sum + line.parse::<i32>().unwrap()
        };
    }
    println!("Sum of calories: {0}", max0 + max1 + max2);
}
