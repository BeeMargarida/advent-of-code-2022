use std::error::Error;

pub mod challenges;

fn main() -> Result<(), Box<dyn Error>> {
    // TEST: change here the day
    let result = challenges::day7::challenge();
    println!("Result {:?}", result);

    Ok(())
}
