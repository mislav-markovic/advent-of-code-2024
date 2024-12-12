mod error;
mod models;

use std::time::Instant;

use advent_of_code_2024::{init, load_day_input};
use eyre::Context;
use models::StoneLine;
use tracing::info;

fn main() -> eyre::Result<()> {
    init();

    info!("loading data...");
    let data = load_day_input("day11.txt")?;

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

fn part1(data: &str) -> eyre::Result<usize> {
    let mut stone_line = data
        .parse::<StoneLine>()
        .wrap_err("could not parse input into stone line")?;
    let iterations = 25;

    for _ in 0..iterations {
        stone_line.blink();
    }

    let len = stone_line.len();

    Ok(len)
}

fn part2(data: &str) -> eyre::Result<usize> {
    let mut stone_line = data
        .parse::<StoneLine>()
        .wrap_err("could not parse input into stone line")?;
    let iterations = 75;

    for _ in 0..iterations {
        stone_line.blink();
    }

    let len = stone_line.len();

    Ok(len)
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const SAMPLE: &str = "125 17";

    #[test]
    fn part_1_sample_data() {
        let res = part1(SAMPLE).expect("part 1 not to error on sample data");

        assert_eq!(55312, res);
    }

    #[test]
    fn blink_3_times() {
        let mut stone_line = SAMPLE.parse::<StoneLine>().expect("to parse sample");

        let iterations = 3;

        assert_eq!(2, stone_line.len());
        for _ in 0..iterations {
            stone_line.blink();
        }

        assert_eq!(5, stone_line.len());
    }

    #[test]
    fn blink_5_times() {
        let mut stone_line = SAMPLE.parse::<StoneLine>().expect("to parse sample");

        let iterations = 5;

        assert_eq!(2, stone_line.len());
        for _ in 0..iterations {
            stone_line.blink();
        }

        assert_eq!(13, stone_line.len());
    }

    #[test]
    fn blink_6_times() {
        let mut stone_line = SAMPLE.parse::<StoneLine>().expect("to parse sample");

        let iterations = 6;

        assert_eq!(2, stone_line.len());
        for _ in 0..iterations {
            stone_line.blink();
        }

        assert_eq!(22, stone_line.len());
    }

    #[test]
    fn part_2_sample_data() {
        let res = part2(SAMPLE).expect("part 2 not to error on sample data");

        assert_eq!(0, res);
    }
}
