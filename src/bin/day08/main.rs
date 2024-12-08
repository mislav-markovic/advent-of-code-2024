mod error;
mod models;

use std::time::Instant;

use advent_of_code_2024::{init, load_day_input};
use eyre::Context;
use models::{CityMap, Pos};
use rustc_hash::FxHashSet;
use tracing::info;

fn main() -> eyre::Result<()> {
    init();

    info!("loading data...");
    let data = load_day_input("day08.txt")?;

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
    let map = data
        .parse::<CityMap>()
        .wrap_err("failed to parse data into city map")?;

    let antinodes = map.antinodes();

    let antinodes = antinodes
        .into_iter()
        .filter(|an| map.is_in_bounds(an.pos()))
        .map(|an| an.pos().clone())
        .collect::<FxHashSet<Pos>>();

    Ok(antinodes.len())
}

fn part2(data: &str) -> eyre::Result<usize> {
    let map = data
        .parse::<CityMap>()
        .wrap_err("failed to parse data into city map")?;

    let antinodes = map.resonant_antinodes();

    let antinodes = antinodes
        .into_iter()
        .filter(|an| map.is_in_bounds(an.pos()))
        .map(|an| an.pos().clone())
        .collect::<FxHashSet<Pos>>();

    Ok(antinodes.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const SAMPLE: &str = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn part_1_sample_data() {
        let res = part1(SAMPLE).expect("part 1 not to error on sample data");

        assert_eq!(14, res);
    }

    #[test]
    fn part_2_sample_data() {
        let res = part2(SAMPLE).expect("part 2 not to error on sample data");

        // TODO when we get example for part 2
        assert_eq!(34, res);
    }
}
