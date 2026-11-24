use anyhow::Result;
use reindeer::{Input, read_file};
use std::io::Read;
use std::path::Path;
use std::str::FromStr;

fn main() -> Result<()> {
    let input = read_input(Input::Small)?;
    let sum = calculate_sum_of_equations(&input);
    println!("Sum of equations is {sum}");

    Ok(())
}

struct Equations {
    rows: Vec<Values>,
    operations: Vec<Operation>,
}

struct Values(Vec<u64>);

enum Operation {
    Addition,
    Multiply,
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Operation::Multiply),
            "+" => Ok(Operation::Addition),
            _ => anyhow::bail!("Unknown equation: {}", s),
        }
    }
}

fn read_input(input: Input) -> Result<Equations> {
    let mut reader = read_file(Path::new(env!("CARGO_MANIFEST_DIR")), input)?;
    let mut file_content = String::new();
    reader.read_to_string(&mut file_content)?;
    let lines = file_content.lines().collect::<Vec<_>>();

    let rows =
        lines.iter()
            .take(lines.len() - 1)
            .map(|line| {
                Values(line.split_ascii_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<_>>())
            })
            .collect::<Vec<_>>();

    let operations =
        lines.last()
            .ok_or(anyhow::anyhow!("Missing last line"))?
            .split_ascii_whitespace()
            .map(|s| s.parse::<Operation>())
            .collect::<Result<Vec<_>, _>>()?;

    Ok(Equations { rows, operations })
}

fn calculate_sum_of_equations(equations: &Equations) -> u64 {
        equations.operations
            .iter()
            .enumerate()
            .map(|(index, operation)| {
                let mut result = match operation {
                    Operation::Addition => 0,
                    Operation::Multiply => 1,
                };
                for row in &equations.rows {
                    let value = row.0.get(index).unwrap();
                    match operation {
                        Operation::Addition => {
                            result += value;
                        }
                        Operation::Multiply => {
                            result *= value;
                        }
                    }
                }
                result
            })
            .sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn small_sum_equations() {
        let input = read_input(Input::Small).unwrap();
        let sum = calculate_sum_of_equations(&input);
        assert_eq!(sum, 4277556);
    }


    #[test]
    fn large_sum_equations() {
        let input = read_input(Input::Large).unwrap();
        let sum = calculate_sum_of_equations(&input);
        assert_eq!(sum, 6100348226985);
    }
}