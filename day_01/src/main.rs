use anyhow::Result;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

const DIAL_START_POS: u8 = 50;

fn main() -> Result<()> {
    for input in [Input::Small, Input::Large] {
        let rotations = read_input(input)?;
        let result = calculate_zeros(&rotations)?;
        println!(
            "There are {} zero positions, {} end zero positions.",
            result.total_zeros, result.end_zeros
        );
    }
    Ok(())
}

fn calculate_zeros(input: &Rotations) -> Result<DialResult> {
    let mut end_zeros = 0;
    let mut total_zeros = 0;
    let mut dial = DIAL_START_POS;
    for rotation in &input.0 {
        let dialed = rotate_dial(dial, rotation)?;
        if dialed.end_position == 0 {
            end_zeros += 1;
        }
        dial = dialed.end_position;
        total_zeros += dialed.zero_clicks;
    }
    Ok(DialResult {
        end_zeros,
        total_zeros,
    })
}

#[derive(Debug)]
struct DialResult {
    end_zeros: u32,
    total_zeros: u32,
}

#[derive(Debug)]
struct Rotations(Vec<Rotation>);

#[derive(Debug)]
struct Rotation {
    direction: Direction,
    value: u32,
}

impl FromStr for Rotation {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, value) = s.split_at(1);
        let direction = direction.parse::<Direction>()?;
        let value = value.parse::<u32>()?;
        Ok(Rotation { direction, value })
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Direction::Right),
            "L" => Ok(Direction::Left),
            _ => Err(anyhow::anyhow!("Unknown direction: {s}")),
        }
    }
}

#[derive(Debug)]
struct DialRotation {
    end_position: u8,
    zero_clicks: u32,
}

#[derive(Debug, Copy, Clone)]
enum Input {
    Small,
    Large,
}

fn read_input(input: Input) -> Result<Rotations> {
    let current_dir = env::current_dir()?;
    let base_path = if cfg!(test) {
        current_dir.as_path()
    } else {
        Path::new(env!("CARGO_PKG_NAME"))
    };
    let file_name = match input {
        Input::Small => "input_small.txt",
        Input::Large => "input.txt",
    };
    let file = File::open(base_path.join(file_name))?;
    let reader = BufReader::new(file);

    let rotations = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.parse::<Rotation>())
        .filter_map(|rotation| rotation.ok())
        .collect::<Vec<Rotation>>();
    Ok(Rotations(rotations))
}

fn rotate_dial(dial: u8, rotation: &Rotation) -> Result<DialRotation> {
    let mut dial = i32::from(dial);
    let mut zero_clicks: i32 = 0;
    match rotation.direction {
        Direction::Left => {
            if dial == 0 {
                zero_clicks = -1;
            }
            dial -= rotation.value.cast_signed();
            while dial < 0 {
                dial += 100;
                zero_clicks += 1;
            }
            if dial == 0 {
                zero_clicks += 1;
            }
        }
        Direction::Right => {
            dial += rotation.value.cast_signed();
            while dial >= 100 {
                dial -= 100;
                zero_clicks += 1;
            }
        }
    }

    Ok(DialRotation {
        end_position: u8::try_from(dial)?,
        zero_clicks: zero_clicks.cast_unsigned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_count_zero_dial_end_positions() {
        let input = read_input(Input::Small).unwrap();
        let result = calculate_zeros(&input).unwrap();
        assert_eq!(result.end_zeros, 3);
    }

    #[test]
    fn small_count_zero_dial_positions() {
        let input = read_input(Input::Small).unwrap();
        let result = calculate_zeros(&input).unwrap();
        assert_eq!(result.total_zeros, 6);
    }

    #[test]
    fn normal_count_zero_dial_end_positions() {
        let input = read_input(Input::Large).unwrap();
        let result = calculate_zeros(&input).unwrap();
        assert_eq!(result.end_zeros, 1011);
    }

    #[test]
    fn normal_count_zero_dial_positions() {
        let input = read_input(Input::Large).unwrap();
        let result = calculate_zeros(&input).unwrap();
        assert_eq!(result.total_zeros, 5937);
    }
}
