mod error;
mod models;

use std::time::Instant;

use advent_of_code_2024::{init, load_day_input};
use eyre::Context;
use models::{FencedGarden, Garden};
use tracing::info;

fn main() -> eyre::Result<()> {
    init();

    info!("loading data...");
    let data = load_day_input("day12.txt")?;

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
    let garden = data
        .parse::<Garden>()
        .wrap_err("failed to parse data into garden")?;

    let fenced_garden: FencedGarden = garden.into();
    let total_cost = fenced_garden.total_cost();

    Ok(total_cost)
}

fn part2(data: &str) -> eyre::Result<usize> {
    let garden = data
        .parse::<Garden>()
        .wrap_err("failed to parse data into garden")?;

    let fenced_garden: FencedGarden = garden.into();
    let total_cost = fenced_garden.discount_cost();

    Ok(total_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn part_1_sample_data() {
        let res = part1(SAMPLE).expect("part 1 not to error on sample data");

        assert_eq!(1930, res);
    }

    #[test]
    fn part_2_sample_data() {
        let res = part2(SAMPLE).expect("part 2 not to error on sample data");

        assert_eq!(1206, res);
    }
}
