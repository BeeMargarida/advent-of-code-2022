use std::error::Error;
use std::fs;

const ABCs: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn priority(letter: &char) -> u32 {
    let is_lowercase = letter.is_lowercase();
    let position = ABCs
        .iter()
        .position(|&x| x == letter.to_ascii_lowercase())
        .unwrap();
    let priority = if is_lowercase {
        position + 1
    } else {
        27 + position
    };
    priority.try_into().unwrap()
}

pub fn challenge() -> Result<u32, Box<dyn Error>> {
    let input = fs::read_to_string("./src/inputs/day3.txt")?;
    let rucksacks: Vec<&str> = input.split("\n").collect();

    let mut sum = 0;
    for r in rucksacks {
        let (first, second) = r.split_at(r.len() / 2);
        let common = first.chars().find(|e| second.contains(*e)).unwrap();
        sum += priority(&common)
    }

    Ok(sum)
}
