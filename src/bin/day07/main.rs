mod error;
mod models;

use std::time::Instant;

use advent_of_code_2024::{init, load_day_input};
use error::Day07Error;
use eyre::Context;
use models::{Equation, Operator};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tracing::info;

fn main() -> eyre::Result<()> {
    init();

    info!("loading data...");
    let data = load_day_input("day07.txt")?;

    info!("solving part 1...");
    let start = Instant::now();
    let part_1_res = part1(&data)?;
    let end = Instant::now();
    let elapsed = end - start;
    info!(
        "Part 1 solved in {time}ms: {part_1_res}",
        time = elapsed.as_millis()
    );

    info!("solving part 2...");
    let start = Instant::now();
    let part_2_res = part2(&data)?;
    let end = Instant::now();
    let elapsed = end - start;
    info!(
        "Part 2 solved in {time}ms: {part_2_res}",
        time = elapsed.as_millis()
    );

    Ok(())
}

fn part1(data: &str) -> eyre::Result<u64> {
    let equations = load_equations(data).wrap_err("failed to parse equations")?;

    let rv = equations
        .into_par_iter()
        .filter_map(|eq| {
            eq.is_solveable_with(&[Operator::Add, Operator::Mul])
                .then_some(eq.test_value())
        })
        .sum();

    Ok(rv)
}

fn part2(data: &str) -> eyre::Result<u64> {
    let equations = load_equations(data).wrap_err("failed to parse equations")?;

    let rv = equations
        .into_par_iter()
        .filter_map(|eq| {
            eq.is_solveable_with(&[Operator::Add, Operator::Mul, Operator::Concatenation])
                .then_some(eq.test_value())
        })
        .sum();

    Ok(rv)
}

fn load_equations(data: &str) -> Result<Vec<Equation>, Day07Error> {
    data.lines().map(|l| l.parse::<Equation>()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const SAMPLE: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn part_1_sample_data() {
        let res = part1(SAMPLE).expect("part 1 not to error on sample data");

        assert_eq!(3749, res);
    }

    #[test]
    fn part_2_sample_data() {
        let res = part2(SAMPLE).expect("part 2 not to error on sample data");

        assert_eq!(11387, res);
    }
}
