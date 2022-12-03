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
}

fn calculate_score(opp: char,me: char, score: &mut i32) {
    match me {
        'Y' => match_to_Y(me, score),
        'X' => match_to_X(me, score),
        'Z' => match_to_Z(me, score),
        _ => println!("Unexpected character!")
    }
}

fn match_to_Y(me: char, score: &mut i32) {

}

fn match_to_X(me: char, score: &mut i32) {
    
}

fn match_to_Z(me: char, score: &mut i32) {
    
}