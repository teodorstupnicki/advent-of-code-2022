use std::fs;

fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();
    let mut score = 0;
    for line in contents.lines() {
        let chars: Vec<char> = line.chars().collect();
        let opp = chars[0];
        let me = chars[2];
        calculate_score(opp, me, &mut score);
    }
    println!("Score: {0}", score);
}

fn calculate_score(opp: char,me: char, score: &mut i32) {
    match me {
        'Y' => match_to_draw(opp, score),
        'X' => match_to_loss(opp, score),
        'Z' => match_to_win(opp, score),
        _ => println!("Unexpected character!"),
    }
}

fn match_to_y(opp: char, score: &mut i32) {
    match opp {
        'A' => *score += 8,
        'B' => *score += 5,
        'C' => *score += 2,
        _ => println!("Unexpected character!"),
    }
}

fn match_to_x(opp: char, score: &mut i32) {
    match opp {
        'A' => *score += 4,
        'B' => *score += 1,
        'C' => *score += 7,
        _ => println!("Unexpected character!"),
    }
}

fn match_to_z(opp: char, score: &mut i32) {
    match opp {
        'A' => *score += 3,
        'B' => *score += 9,
        'C' => *score += 6,
        _ => println!("Unexpected character!"),
    }
}

fn match_to_loss(opp: char, score: &mut i32) {
    match opp {
        'A' => *score += 3,
        'B' => *score += 1,
        'C' => *score += 2,
        _ => println!("Unexpected character!"),
    }
}

fn match_to_draw(opp: char, score: &mut i32) {
    match opp {
        'A' => *score += 4,
        'B' => *score += 5,
        'C' => *score += 6,
        _ => println!("Unexpected character!"),
    }
}

fn match_to_win(opp: char, score: &mut i32) {
    match opp {
        'A' => *score += 8,
        'B' => *score += 9,
        'C' => *score += 7,
        _ => println!("Unexpected character!"),
    }
}