use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub fn challenge() -> Result<u32, Box<dyn Error>> {
    let mut rules = HashMap::new();
    rules.insert("X", 0);
    rules.insert("Y", 3);
    rules.insert("Z", 6);

    let mut moves = HashMap::new();
    moves.insert("A X", 3); // plays scissors
    moves.insert("A Y", 1); // plays rock
    moves.insert("A Z", 2); // plays paper
    moves.insert("B X", 1);
    moves.insert("B Y", 2);
    moves.insert("B Z", 3);
    moves.insert("C X", 2);
    moves.insert("C Y", 3);
    moves.insert("C Z", 1);

    let input = fs::read_to_string("./src/inputs/day2.txt")?;
    let plays: Vec<&str> = input.split("\n").collect();

    let mut my_score = 0;
    for p in plays {
        let shapes: Vec<&str> = p.split(" ").collect();
        my_score += moves[p] + rules[shapes[1]];
    }

    Ok(my_score)
}
