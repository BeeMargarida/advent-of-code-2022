use std::error::Error;
use std::fs;

const _PACKET: usize = 4;
const MESSAGE: usize = 14;

pub fn challenge() -> Result<u32, Box<dyn Error>> {
    let input = fs::read_to_string("./src/inputs/day6.txt")?;

    let buffer = input.trim();

    for (i, _) in buffer.chars().enumerate() {
        let slice = &buffer[i..i + MESSAGE];

        // checks if the slice contains repeated letters
        let mut slice_chars: Vec<char> = slice.chars().collect();
        slice_chars.sort();
        slice_chars.dedup();
        let has_repeated = slice_chars.len() < slice.len();

        if !has_repeated {
            return Ok((i + MESSAGE) as u32);
        }
    }

    Ok(0)
}
