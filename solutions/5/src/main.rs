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
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..n {
        stacks.push(Vec::new());
    }
    for line in contents.lines() {
        if line.contains("1") {
            break;
        }
        let chars: Vec<char> = line.chars().collect();
        for i in 0..n {
            let temp = &mut stacks[i as usize];
            let ch = chars[(1+i*4) as usize];
            if ch != ' ' {
                temp.insert(0, ch);
            }
        }
    }
    for line in contents.lines() {
        if !line.contains("move") {
            continue;
        }
        let arr: Vec<&str> = line.split(" ").collect();
        let n = arr[1].parse::<i32>().unwrap();
        let src = arr[3].parse::<usize>().unwrap() - 1;
        let dest = arr[5].parse::<usize>().unwrap() - 1;
        let mut v:Vec<char> = Vec::new();
        for _ in 0..n {
            let tmp = stacks[src].pop().unwrap();
            v.insert(0, tmp);
        }
        stacks[dest].append(&mut v);
    }
    for stack in stacks {
        print!("{0}", stack.last().unwrap());
    }
}
