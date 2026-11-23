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

fn calculate_total_output_joltage_ext(banks: &BatteryBanks) -> usize {
    banks
        .iter()
        .map(|bank| calculate_joltage_ext(bank, 12))
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

fn calculate_joltage_ext(bank: &BatteryBank, digits: u8) -> usize {
    let mut max_values = Vec::<u8>::with_capacity(digits as usize);
    let bank_len = bank.len();

    let mut bank_index: i32 = -1;
    for index in (0..digits).rev() {
        let max = bank
            .iter()
            .enumerate()
            .take(bank_len - index as usize)
            .skip((bank_index + 1) as usize)
            .rev()
            .max_by(|x, y| x.1.cmp(&y.1))
            .unwrap();
        bank_index = max.0 as i32;
        max_values.push(*max.1);
    }
    println!("{:?}", max_values);

    let max_joltage = max_values
        .iter()
        .rev()
        .enumerate()
        .fold(0_usize, |max, (index, value)|
            max + 10_usize.pow(index as u32) * *value as usize,
        );
    println!("{:?}", max_joltage);

    max_joltage
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_sum_joltage() {
        let banks = read_input(Input::Small).unwrap();
        let joltage = calculate_total_output_joltage(&banks);
        assert_eq!(joltage, 357);
    }

    #[test]
    fn large_sum_joltage() {
        let banks = read_input(Input::Large).unwrap();
        let joltage = calculate_total_output_joltage(&banks);
        assert_eq!(joltage, 17031);
    }

    #[test]
    fn small_sum_joltage_ext() {
        let banks = read_input(Input::Small).unwrap();
        let joltage = calculate_total_output_joltage_ext(&banks);
        assert_eq!(joltage, 3121910778619);
    }
}