mod error;
mod models;

use std::time::Instant;

use advent_of_code_2024::{init, load_day_input};
use eyre::Context;
use models::Map;
use tracing::info;

fn main() -> eyre::Result<()> {
    init();

    info!("loading data...");
    let data = load_day_input("day10.txt")?;

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
    let map = data.parse::<Map>().wrap_err("failed to parse map")?;

    let trailheads = map.trailheads();

    let total_score = trailheads.iter().map(|th| th.score()).sum();

    Ok(total_score)
}

fn part2(data: &str) -> eyre::Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const SAMPLE: &str = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn map_parse_print_roundtrip() {
        let map = SAMPLE.parse::<Map>().expect("failed to parse map");
        let map_print = map.print();

        assert_eq!(SAMPLE, map_print);
    }

    #[test]
    fn correct_number_of_trailheads() {
        let map = SAMPLE.parse::<Map>().expect("failed to parse map");
        let trailheads = map.trailheads();
        assert_eq!(9, trailheads.len());
    }

    #[test]
    fn part_1_sample_data() {
        let res = part1(SAMPLE).expect("part 1 not to error on sample data");

        assert_eq!(36, res);
    }

    #[test]
    fn part_2_sample_data() {
        let res = part2(SAMPLE).expect("part 2 not to error on sample data");

        assert_eq!(0, res);
    }
}
