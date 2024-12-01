mod error;
mod models;

use advent_of_code_2024::{init, load_day_input};
use eyre::Context;
use models::SideBySide;
use tracing::info;

fn main() -> eyre::Result<()> {
    init();

    info!("loading data...");
    let data = load_day_input("day01.txt")?;

    info!("solving part 1...");
    let part_1_res = part1(&data)?;
    info!("Part 1 solution: {part_1_res}");

    info!("solving part 2...");
    let part_2_res = part2(&data)?;
    info!("Part 2 solution: {part_2_res}");

    Ok(())
}

fn part1(input: &str) -> eyre::Result<u64> {
    let side_by_side = input
        .parse::<SideBySide>()
        .wrap_err("failed to parse side by side lists from input")?;

    let total_distance = side_by_side.compute_total_distance();

    Ok(total_distance)
}

fn part2(input: &str) -> eyre::Result<u64> {
    let side_by_side = input
        .parse::<SideBySide>()
        .wrap_err("failed to parse side by side lists from input")?;

    let similarity_score = side_by_side.similarity_score();

    Ok(similarity_score)
}
