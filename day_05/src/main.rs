use anyhow::Result;
use reindeer::{Input, read_file};
use std::io::BufRead;
use std::path::Path;

fn main() -> Result<()> {
    let input = read_input(Input::Small)?;
    let sum = sum_available_fresh_ingredients(&input.fresh, &input.available);
    println!("Part 1 result: {sum}");

    let input = read_input(Input::Small)?;
    let sum = sum_fresh_ingredients(&input.fresh);
    println!("Part 2 result: {sum}");

    Ok(())
}

struct FileInput {
    fresh: FreshIngredients,
    available: Ingredients
}

struct FreshIngredients {
    data: Vec<IngredientRange>
}

#[derive(Debug, Copy, Clone)]
struct IngredientRange {
    first: usize,
    last: usize
}

impl FreshIngredients {
    const fn new() -> Self {
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

    fn refresh(&self) -> Self {
        let mut sorted_data = self.data.clone();
        sorted_data.sort_by_key(|range| range.first);

        let mut data = vec![sorted_data[0]];
        for current in sorted_data.iter().skip(1) {
            let previous = data.last_mut().unwrap();

            // no intersection between previous and current...
            if previous.last < current.first {
                // ...so push current at the end
                data.push(*current);
            // both ranges intersect with each other...
            } else if previous.last < current.last {
                // ...modify previous range with current value
                previous.last = current.last;
            }
        }

        Self { data }
    }

    fn count(&self) -> usize {
        self.data.iter()
            .map(|i| i.last - i.first + 1)
            .sum()
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
        } else if found_empty_line {
            available.0.push(line.parse::<usize>()?);
        } else {
            let (first, last) = line.split_once('-')
                .ok_or(anyhow::anyhow!("Invalid line '{line}'"))?;
            fresh.add(first.parse()?, last.parse()?);
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

fn sum_fresh_ingredients(fresh: &FreshIngredients) -> usize {
    fresh.refresh().count()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn small_sum_fresh_available_ingredients() {
        let input = read_input(Input::Small).unwrap();
        let sum = sum_available_fresh_ingredients(&input.fresh, &input.available);
        assert_eq!(3, sum);
    }

    #[test]
    fn large_sum_fresh_available_ingredients() {
        let input = read_input(Input::Large).unwrap();
        let sum = sum_available_fresh_ingredients(&input.fresh, &input.available);
        assert_eq!(701, sum);
    }

    #[test]
    fn small_sum_fresh_ingredients() {
        let input = read_input(Input::Small).unwrap();
        let sum = sum_fresh_ingredients(&input.fresh);
        assert_eq!(14, sum);
    }

    #[test]
    fn large_sum_fresh_ingredients() {
        let input = read_input(Input::Large).unwrap();
        let sum = sum_fresh_ingredients(&input.fresh);
        assert_eq!(352340558684863, sum);
    }
}