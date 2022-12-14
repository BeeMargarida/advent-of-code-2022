use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::str::FromStr;

fn filesystem(lines: Vec<&str>) -> HashMap<String, u64> {
    let mut dirs: HashMap<String, u64> = HashMap::new();
    dirs.insert("/".to_string(), 0);

    let regex_cd = Regex::new(r"\$ cd (\W|\w)+").unwrap();
    let regex_ls = Regex::new(r"\$ ls").unwrap();
    let regex_dir = Regex::new(r"dir \w+").unwrap();
    let regex_file = Regex::new(r"\d+ \w+.\w+").unwrap();

    let mut current = vec!["."];

    for line in lines {
        match line {
            line if regex_cd.is_match(line) => {
                let parts: Vec<&str> = line.split("$ cd ").collect();
                let path = parts[1];

                match path {
                    "/" => {
                        current = vec!["."];
                    }
                    ".." => {
                        current.pop().unwrap();
                    }
                    name => {
                        current.push(name);
                    }
                };
            }
            line if regex_ls.is_match(line) => {}
            line if regex_dir.is_match(line) => {
                let parts: Vec<&str> = line.split(" ").collect();
                dirs.insert(format!("{}/{}", current.join("/"), parts[1].to_string()), 0);
            }
            line if regex_file.is_match(line) => {
                let parts: Vec<&str> = line.split(" ").collect();
                let size = parts[0];

                let size_u64: u64 = FromStr::from_str(size).unwrap();
                dirs.insert(
                    ".".to_string(),
                    dirs.get(".").unwrap_or_else(|| &0u64) + size_u64,
                );

                for (i, _) in current.iter().enumerate() {
                    let key = current[1..i + 1].join("/");
                    if dirs.contains_key(&key) {
                        let value = dirs.get(&key).unwrap();
                        let total = value + size_u64;
                        dirs.insert(key, total);
                    } else {
                        dirs.insert(key, size_u64);
                    }
                }
            }
            _ => {}
        }
    }

    dirs
}

pub fn challenge() -> Result<u64, Box<dyn Error>> {
    let input = fs::read_to_string("./src/inputs/day7.txt")?;
    let lines: Vec<&str> = input.split("\n").collect();

    let filesystem = filesystem(lines);

    println!("{:?}", filesystem);

    let mut sum = 0;
    for (_, val) in filesystem.iter() {
        if *val <= 100000 {
            sum += *val;
        }
    }

    let available_space = 70000000 - filesystem.get(".").unwrap();
    let required_space = 30000000 - available_space;
    let mut smallest_dir = ("", u64::MAX);
    for (path, val) in filesystem.iter() {
        if *val >= required_space && *val < smallest_dir.1 {
            smallest_dir = (path, *val);
        }
    }

    println!("Part2: {:?}", smallest_dir);

    Ok(sum)
}
