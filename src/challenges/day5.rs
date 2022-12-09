use regex::Regex;
use std::collections::LinkedList;
use std::error::Error;
use std::fs;

struct Instruction {
    number: u32,
    from: u32,
    to: u32,
}

fn initialize(mut initial: Vec<&str>) -> Vec<Vec<&str>> {
    let length: u32 = initial[initial.len() - 1]
        .trim()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap();

    // initializes the containers state with the number of rows
    let mut state: Vec<Vec<&str>> = vec![];
    for _ in 0..length {
        state.push(Vec::new());
    }

    // removes last line from the initial state string and reverse it
    initial.pop().unwrap();
    initial.reverse();

    for line in initial {
        let regex = Regex::new(r"(\[.\])|(\s{4})").unwrap();
        for (i, caps) in regex.captures_iter(line.trim()).enumerate() {
            let letter = caps.get(0).unwrap().as_str();
            if !letter.contains("[") {
                continue;
            }

            let clean_letter = letter
                .trim()
                .strip_prefix("[")
                .unwrap_or_else(|| letter)
                .strip_suffix("]")
                .unwrap_or_else(|| letter);
            state[i].push(clean_letter);
        }
    }

    state
}

fn instructions(initial: Vec<&str>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];
    let regex = Regex::new(r"\d+").unwrap();

    for line in initial {
        let mut caps = regex.captures_iter(line.trim());
        let instruction = Instruction {
            number: caps
                .next()
                .unwrap()
                .get(0)
                .unwrap()
                .as_str()
                .parse()
                .unwrap(),
            from: caps
                .next()
                .unwrap()
                .get(0)
                .unwrap()
                .as_str()
                .parse()
                .unwrap(),
            to: caps
                .next()
                .unwrap()
                .get(0)
                .unwrap()
                .as_str()
                .parse()
                .unwrap(),
        };
        instructions.push(instruction);
    }

    instructions
}

pub fn challenge() -> Result<u32, Box<dyn Error>> {
    let input = fs::read_to_string("./src/inputs/day5.txt")?;
    let parts: Vec<&str> = input.split("\n\n").collect();

    let containers_string: Vec<&str> = parts[0].split("\n").collect();
    let instructions_string: Vec<&str> = parts[1].split("\n").collect();

    let mut containers: Vec<Vec<&str>> = initialize(containers_string);
    let mut instructions = instructions(instructions_string);

    for inst in instructions {
        let column_from_index: usize = (inst.from - 1).try_into().unwrap();
        let column_from_length = containers[column_from_index].len();
        let column_to_index: usize = (inst.to - 1).try_into().unwrap();

        // retrieves the elements to be moved to another column
        let slice = column_from_length - (inst.number as usize);
        let mut moved: Vec<&str> = containers[column_from_index][slice..column_from_length]
            .try_into()
            .unwrap();
        moved.reverse();

        // moves the reversed elements to the another column and removes it
        // from the previous one
        containers[column_to_index].append(&mut moved);
        containers[column_from_index].drain(slice..column_from_length);
    }

    let top: Vec<&&str> = containers.iter().map(|c| c.last().unwrap()).collect();
    println!("{:?}", top);

    Ok(0)
}
