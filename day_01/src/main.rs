use anyhow::{anyhow, Result};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

const DIAL_START_POS: i32 = 50;
const DIAL_POSITIONS: i32 = 100;

fn main() -> Result<()> {
    for input in [Input::Small, Input::Large] {
        let rotations = read_input(input)?;
        let result = calculate_zeros(&rotations);
        println!(
            "There are {} zero positions, {} end zero positions.",
            result.total_zeros, result.end_zeros
        );
    }
    Ok(())
}

fn calculate_zeros(input: &Rotations) -> DialResult {
    let mut end_zeros = 0;
    let mut total_zeros = 0;
    let mut dial = DIAL_START_POS;
    for rotation in input {
        let dialed = rotate_dial(dial, rotation);
        if dialed.end_position == 0 {
            end_zeros += 1;
        }
        dial = dialed.end_position;
        total_zeros += dialed.zero_clicks;
    }
    DialResult {
        end_zeros,
        total_zeros,
    }
}

#[derive(Debug)]
struct DialResult {
    end_zeros: u32,
    total_zeros: u32,
}

type Rotations = Vec<Rotation>;

#[derive(Debug)]
struct Rotation(i32);

impl FromStr for Rotation {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let slitted = s.split_at_checked(1);
        let (direction, value) = slitted.ok_or_else(|| anyhow!("invalid line"))?;
        let value = value.parse::<i32>()?;
        match direction {
            "L" => Ok(Rotation(-value)),
            "R" => Ok(Rotation(value)),
            _ => Err(anyhow!("invalid rotation direction")),
        }
    }
}

#[derive(Debug)]
struct DialRotation {
    end_position: i32,
    zero_clicks: u32,
}

#[derive(Debug, Copy, Clone)]
enum Input {
    Small,
    Large,
}

fn read_input(input: Input) -> Result<Rotations> {
    let rotations = read_file(input)?
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.parse::<Rotation>())
        .collect::<Result<Vec<Rotation>>>()?;
    Ok(rotations)
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

fn rotate_dial(dial: i32, rotation: &Rotation) -> DialRotation {
    let mut end_position = dial + rotation.0;
    let mut zero_clicks = (end_position / DIAL_POSITIONS).unsigned_abs();
    if (dial.is_positive() && end_position.is_negative())
        || (dial.is_negative() && end_position.is_positive())
        || end_position == 0
    {
        zero_clicks += 1;
    }
    end_position = end_position.rem_euclid(DIAL_POSITIONS);

    DialRotation {
        end_position,
        zero_clicks,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_count_zero_dial_end_positions() {
        let input = read_input(Input::Small).unwrap();
        let result = calculate_zeros(&input);
        assert_eq!(result.end_zeros, 3);
    }

    #[test]
    fn small_count_zero_dial_positions() {
        let input = read_input(Input::Small).unwrap();
        let result = calculate_zeros(&input);
        assert_eq!(result.total_zeros, 6);
    }

    #[test]
    fn normal_count_zero_dial_end_positions() {
        let input = read_input(Input::Large).unwrap();
        let result = calculate_zeros(&input);
        assert_eq!(result.end_zeros, 1011);
    }

    #[test]
    fn normal_count_zero_dial_positions() {
        let input = read_input(Input::Large).unwrap();
        let result = calculate_zeros(&input);
        assert_eq!(result.total_zeros, 5937);
    }
}
