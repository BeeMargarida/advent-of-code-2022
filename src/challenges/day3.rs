use std::error::Error;
use std::fs;

const ABC: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn priority(letter: &char) -> u32 {
    let is_lowercase = letter.is_lowercase();
    let position = ABC
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
    for r in rucksacks.chunks(3) {
        let first = r[0];
        let badge = first
            .chars()
            .find(|e| r[1].contains(*e) && r[2].contains(*e))
            .unwrap();
        sum += priority(&badge);
    }

    Ok(sum)
}
