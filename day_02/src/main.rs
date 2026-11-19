use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::str::FromStr;
use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let ranges = read_input(Input::Small)?;
    let sum = sum_invalid_ids(&ranges);
    println!("Sum: {}", sum);
    Ok(())
}

#[derive(Debug, Copy, Clone)]
enum Input {
    Small,
    Large,
}

type Ranges = Vec<Range>;

#[derive(Debug)]
struct Range {
    min: u64,
    max: u64,
}

impl FromStr for Range {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (min, max) = s.split_once('-')
            .ok_or(anyhow!("Invalid range format"))?;
        let min = min.parse()?;
        let max = max.parse()?;
        Ok(Self { min, max })
    }
}

fn sum_invalid_ids(ranges: &Ranges) -> u64 {
    let mut sum = 0;
    for range in ranges {
        for value in range.min..=range.max {
            if is_invalid_id(value) {
                println!("Invalid value: {}", value);
                sum += value;
            }
        }
    }
    sum
}

fn is_invalid_id(value: u64) -> bool {
    let digits = value.ilog10() + 1;
    // all values with odd number of digits are valid
    if digits % 2 == 1 {
        return false;
    }
    let half_digits = digits / 2;
    let divisor = 10_u64.pow(half_digits);
    let first = value % divisor;
    let second = value / divisor;
    first == second
}

fn read_input(input: Input) -> Result<Ranges> {
    let mut reader = read_file(input)?;
    let mut file_input: String = String::new();
    reader.read_to_string(&mut file_input)?;

    let ranges = file_input
        .split(',')
        .filter_map(|line| line.parse::<Range>().ok())
        .collect::<Vec<Range>>();
    Ok(ranges)
}

fn read_file(input: Input) -> Result<BufReader<File>> {
    let base_path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let file_name = match input {
        Input::Small => "input_small.txt",
        Input::Large => "input.txt",
    };
    let file = File::open(base_path.join(file_name))?;
    Ok(BufReader::new(file))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_sum_invalid_ids() {
        let ranges = read_input(Input::Small).unwrap();
        let sum = sum_invalid_ids(&ranges);
        assert_eq!(sum, 1227775554);
    }

    #[test]
    fn large_sum_invalid_ids() {
        let ranges = read_input(Input::Large).unwrap();
        let sum = sum_invalid_ids(&ranges);
        assert_eq!(sum, 19605500130);
    }
}