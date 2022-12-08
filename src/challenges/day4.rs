use std::error::Error;
use std::fs;

pub fn challenge() -> Result<u32, Box<dyn Error>> {
    let input = fs::read_to_string("./src/inputs/day4.txt")?;
    let pairs: Vec<&str> = input.split("\n").collect();

    let mut sum = 0;
    for p in pairs {
        let parts: Vec<Vec<&str>> = p.split(",").map(|p| p.split("-").collect()).collect();
        let values1: Vec<u32> = parts[0].iter().map(|v| v.parse().unwrap()).collect();
        let values2: Vec<u32> = parts[1].iter().map(|v| v.parse().unwrap()).collect();

        if !(values1[1] < values2[0] && values1[1] < values2[1]
            || values2[1] < values1[0] && values2[1] < values1[1])
        {
            sum += 1;
        }
    }

    Ok(sum)
}
