use std::error::Error;

pub mod challenges;

fn main() -> Result<(), Box<dyn Error>> {
    let result = challenges::day1::challenge();
    println!("Result {:?}", result);

    Ok(())
}
