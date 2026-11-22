use anyhow::Result;
use reindeer::{Input, read_file};
use std::io::BufRead;
use std::path::Path;

fn main() -> Result<()> {
    let banks = read_input(Input::Small)?;
    let joltage = calculate_total_output_joltage(&banks);
    println!("Total output is {}", joltage);
    Ok(())
}

type BatteryBank = Vec<u8>;
type BatteryBanks = Vec<BatteryBank>;

fn read_input(input: Input) -> Result<BatteryBanks> {
    Ok(
        read_file(Path::new(env!("CARGO_MANIFEST_DIR")), input)?
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c as u8 - '0' as u8)
                .collect::<BatteryBank>()
        })
        .collect::<BatteryBanks>())
}

fn calculate_total_output_joltage(banks: &BatteryBanks) -> usize {
    banks
        .iter()
        .map(|bank| calculate_joltage(bank))
        .sum()
}

fn calculate_joltage(bank: &BatteryBank) -> usize {
    let first = bank
        .iter()
        .take(bank.len() - 1)
        .enumerate()
        .rev()
        .max_by(|x, y| x.1.cmp(&y.1))
        .unwrap();
    let second = bank
        .iter()
        .skip(first.0+1)
        .max()
        .unwrap();

    (first.1 * 10 + second) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_sum_invalid_ids() {
        let banks = read_input(Input::Small).unwrap();
        let joltage = calculate_total_output_joltage(&banks);
        assert_eq!(joltage, 357);
    }

    #[test]
    fn large_sum_invalid_ids() {
        let banks = read_input(Input::Large).unwrap();
        let joltage = calculate_total_output_joltage(&banks);
        assert_eq!(joltage, 17031);
    }
}