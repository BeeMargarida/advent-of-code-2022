use std::fs;
use std::error::Error;

pub fn challenge() -> Result<u32, Box<dyn Error>> {
    let contents = fs::read_to_string("./src/inputs/day1.txt")?;
    
    // converts file content into vectors containing the sum of calories
    // for each elf 
    let split_by_elf: Vec<Vec<&str>> = contents.split("\n\n").into_iter().map(|s| s.split("\n").collect()).collect();
    let mut sum_by_elf: Vec<u32> = split_by_elf.iter().map(|v| v.iter().map(|e| e.parse::<u32>().unwrap()).sum()).collect();
    
    // calculates the amount of calories of the 3 top elves
    sum_by_elf.sort();
    let max: u32 = sum_by_elf.iter().rev().take(3).sum();

    Ok(max)
}
