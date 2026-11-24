use anyhow::Result;
use reindeer::{Input, read_file};
use std::io::BufRead;
use std::path::Path;

fn main() -> Result<()> {
    let input = read_input(Input::Small)?;
    let sum = sum_available_fresh_ingredients(&input.fresh, &input.available);
    println!("Part 1 result: {sum}");

    Ok(())
}

struct FileInput {
    fresh: FreshIngredients,
    available: Ingredients
}

struct FreshIngredients {
    data: Vec<IngredientRange>
}

struct IngredientRange {
    first: usize,
    last: usize
}

impl FreshIngredients {
    fn new() -> Self {
        Self {
            data: Vec::new()
        }
    }

    fn add(&mut self, first: usize, last: usize) {
        self.data.push(IngredientRange { first, last });
    }

    fn contains(&self, ingredient: usize) -> bool {
        self.data.iter()
            .any(|i|
                i.first <= ingredient && ingredient <= i.last
            )
    }
}

struct Ingredients(Vec<usize>);

fn read_input(input: Input) -> Result<FileInput> {

    let reader = read_file(Path::new(env!("CARGO_MANIFEST_DIR")), input)?;

    let mut fresh = FreshIngredients::new();
    let mut available = Ingredients(Vec::new());

    let mut found_empty_line = false;
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            found_empty_line = true;
        }
        else {
            if found_empty_line {
                available.0.push(line.parse::<usize>()?);
            } else {
                let (first, last) = line.split_once('-')
                    .ok_or(anyhow::anyhow!("Invalid line '{}'", line))?;
                fresh.add(first.parse()?, last.parse()?);
            }
        }
    }

    Ok(FileInput {
        fresh,
        available
    })
}

fn sum_available_fresh_ingredients(fresh: &FreshIngredients, available: &Ingredients) -> usize {
    available.0
        .iter()
        .filter(|&&i| fresh.contains(i))
        .count()
}

#[cfg(test)]
mod tests {
    use crate::{read_input, sum_available_fresh_ingredients};
    use reindeer::Input;

    #[test]
    fn small_sum_fresh_ingredients() {
        let input = read_input(Input::Small).unwrap();
        let sum = sum_available_fresh_ingredients(&input.fresh, &input.available);
        assert_eq!(3, sum);
    }

    #[test]
    fn large_sum_fresh_ingredients() {
        let input = read_input(Input::Large).unwrap();
        let sum = sum_available_fresh_ingredients(&input.fresh, &input.available);
        assert_eq!(701, sum);
    }
}