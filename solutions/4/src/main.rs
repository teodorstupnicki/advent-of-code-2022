use std::fs;

fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();
    let mut sum = 0;
    for line in contents.lines() {
        read_line(&mut sum, line);
    }
    println!("{0}", sum);
}

fn read_line(sum: &mut i32, line: &str) {
    let sections: Vec<&str> = line.split(",").collect();
    let mut s1 = sections[0].split("-");
    let b1 = s1.next().unwrap().parse::<i32>().unwrap();
    let e1 = s1.next().unwrap().parse::<i32>().unwrap();
    let mut s2 = sections[1].split("-");
    let b2 = s2.next().unwrap().parse::<i32>().unwrap();
    let e2 = s2.next().unwrap().parse::<i32>().unwrap();
    if b1 > e2 || b2 > e1 {

    } else {
        *sum+=1;
    }
}
