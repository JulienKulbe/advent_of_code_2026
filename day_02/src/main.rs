use anyhow::{Result, anyhow};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::str::{FromStr, from_utf8};

fn main() -> Result<()> {
    let ranges = read_input(Input::Small)?;
    let sum = sum_invalid_ids(&ranges, InvalidIds::TwoParts);
    println!("Sum: {}", sum);

    let ranges = read_input(Input::Small)?;
    let sum = sum_invalid_ids(&ranges, InvalidIds::Multiple);
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

#[derive(Debug, Copy, Clone)]
enum InvalidIds {
    TwoParts,
    Multiple,
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

fn sum_invalid_ids(ranges: &Ranges, id_type: InvalidIds) -> u64 {
    let mut sum = 0;
    for range in ranges {
        for value in range.min..=range.max {
            if is_invalid_id(value, id_type) {
                //println!("Invalid value: {}", value);
                sum += value;
            }
        }
    }
    sum
}

fn is_invalid_id(value: u64, id_type: InvalidIds) -> bool {
    match id_type {
        InvalidIds::TwoParts => {
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

        InvalidIds::Multiple => {
            let value_str = value.to_string();
            let digits = value_str.len();
            let half_digits = digits / 2;

            for parts in 1..=half_digits {
                if digits % parts != 0 {
                    continue;
                }
                let parts: Vec<&str> = value_str
                    .as_bytes()
                    .chunks(parts)
                    .map(|c| from_utf8(c).unwrap())
                    .collect();
                let first = parts[0];
                if parts.iter().all(|&part| part == first) {
                    return true;
                }
            }
            false
        }
    }
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
        let sum = sum_invalid_ids(&ranges, InvalidIds::TwoParts);
        assert_eq!(sum, 1227775554);
    }

    #[test]
    fn large_sum_invalid_ids() {
        let ranges = read_input(Input::Large).unwrap();
        let sum = sum_invalid_ids(&ranges, InvalidIds::TwoParts);
        assert_eq!(sum, 19605500130);
    }

    #[test]
    fn small_sum_invalid_ids_multiple_times() {
        let ranges = read_input(Input::Small).unwrap();
        let sum = sum_invalid_ids(&ranges, InvalidIds::Multiple);
        assert_eq!(sum, 4174379265);
    }

    #[test]
    fn large_sum_invalid_ids_multiple_times() {
        let ranges = read_input(Input::Large).unwrap();
        let sum = sum_invalid_ids(&ranges, InvalidIds::Multiple);
        assert_eq!(sum, 36862281418);
    }
}