use anyhow::Result;
use reindeer::{Input, read_file};
use std::io::Read;
use std::path::Path;
use std::str::FromStr;

fn main() -> Result<()> {
    let input = read_input(Input::Small)?;
    let sum = calculate_sum_of_equations(&input);
    println!("Sum of equations is {sum}");

    let input = read_input_ext(Input::Small)?;
    let sum = calculate_sum_of_cephalopods_equations(&input);
    println!("Sum of equations is {sum}");

    Ok(())
}

struct Equations {
    rows: Vec<Values>,
    operations: Vec<Operation>,
}

struct Values(Vec<u64>);

#[derive(Debug, Copy, Clone)]
enum Operation {
    Addition,
    Multiply,
}

struct CephalopodsEquations {
    rows: Vec<ValueIndices>,
    operations: OperationIndices,
    length: usize,
}

struct ValueIndices(Vec<ValueIndex>);

struct OperationIndices(Vec<OperationIndex>);

struct ValueIndex {
    value: u8,
    index: usize,
}
struct OperationIndex {
    operation: Operation,
    index: usize,
}

impl FromStr for ValueIndices {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ValueIndices(
            s.char_indices()
                .filter_map(|(index, char)| {
                    char.to_digit(10).map(|value| ValueIndex {
                        value: value as u8,
                        index,
                    })
                })
                .collect::<Vec<_>>(),
        ))
    }
}

impl FromStr for OperationIndices {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(OperationIndices(
            s.char_indices()
                .filter_map(|(index, char)| match char {
                    '*' => Some(OperationIndex {
                        operation: Operation::Multiply,
                        index,
                    }),
                    '+' => Some(OperationIndex {
                        operation: Operation::Addition,
                        index,
                    }),
                    _ => None,
                })
                .collect::<Vec<_>>(),
        ))
    }
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

    let rows = lines
        .iter()
        .take(lines.len() - 1)
        .map(|line| {
            Values(
                line.split_ascii_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let operations = lines
        .last()
        .ok_or(anyhow::anyhow!("Missing last line"))?
        .split_ascii_whitespace()
        .map(|s| s.parse::<Operation>())
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Equations { rows, operations })
}

fn read_input_ext(input: Input) -> Result<CephalopodsEquations> {
    let mut reader = read_file(Path::new(env!("CARGO_MANIFEST_DIR")), input)?;
    let mut file_content = String::new();
    reader.read_to_string(&mut file_content)?;
    let lines = file_content.lines().collect::<Vec<_>>();

    let rows = lines
        .iter()
        .take(lines.len() - 1)
        .map(|line| line.parse::<ValueIndices>())
        .collect::<Result<Vec<_>>>()?;

    let last_line = lines.last().ok_or(anyhow::anyhow!("Missing last line"))?;
    let operations = last_line.parse::<OperationIndices>()?;
    let length = last_line.len() + 1;

    Ok(CephalopodsEquations { rows, operations, length })
}

fn calculate_sum_of_equations(equations: &Equations) -> u64 {
    equations
        .operations
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

fn calculate_sum_of_cephalopods_equations(equations: &CephalopodsEquations) -> u64 {
    let mut sum: u64 = 0;

    struct OperationIndices {
        operation: Operation,
        start: usize,
        end: usize,
    }

    let mut operation_indices = Vec::<OperationIndices>::new();
    for operation_index in &equations.operations.0 {
        let previous = operation_indices.last_mut();
        if let Some(previous) = previous {
            previous.end = operation_index.index;
        }
        operation_indices.push(OperationIndices {
            operation: operation_index.operation,
            start: operation_index.index,
            end: equations.length,
        })
    }

    for operation_index in operation_indices {
        let mut result = match operation_index.operation {
            Operation::Addition => 0,
            Operation::Multiply => 1,
        };

        for index in operation_index.start..operation_index.end {
            let mut current: Option<u64> = None;

            for values in &equations.rows {
                let value = values.0.iter()
                    .find(|value_index| { value_index.index == index });
                if let Some(ValueIndex { value, index: _ }) = value {
                    let value = *value as u64;
                    current = if let Some(current) = current {
                        Some(current * 10 + value)
                    } else {
                        Some(value)
                    }
                }
            }
            if let Some(current) = current {
                match operation_index.operation {
                    Operation::Addition => {
                        result += current;
                    }
                    Operation::Multiply => {
                        result *= current;
                    }
                }
            }
        }

        sum += result;
    }

    sum
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

    #[test]
    fn small_sum_cephalopods_equations() {
        let input = read_input_ext(Input::Small).unwrap();
        let sum = calculate_sum_of_cephalopods_equations(&input);
        assert_eq!(sum, 3263827);
    }

    #[test]
    fn large_sum_cephalopods_equations() {
        let input = read_input_ext(Input::Large).unwrap();
        let sum = calculate_sum_of_cephalopods_equations(&input);
        assert_eq!(12377473011151_u64, sum);
    }
}
