mod error;
mod models;

use advent_of_code_2024::{init, load_day_input};
use eyre::Context;
use models::{ConditionalMemory, Memory};
use tracing::info;

fn main() -> eyre::Result<()> {
    init();

    info!("loading data...");
    let data = load_day_input("day03.txt")?;

    info!("solving part 1...");
    let part_1_res = part1(&data)?;
    info!("Part 1 solution: {part_1_res}");

    info!("solving part 2...");
    let part_2_res = part2(&data)?;
    info!("Part 2 solution: {part_2_res}");

    Ok(())
}

fn part1(data: &str) -> eyre::Result<u64> {
    let memory = data
        .parse::<Memory>()
        .wrap_err("failed to parse instructions from corrupted memory")?;

    Ok(memory.run())
}

fn part2(data: &str) -> eyre::Result<u64> {
    let memory = data
        .parse::<ConditionalMemory>()
        .wrap_err("failed to parse instructions from corrupted memory")?;

    Ok(memory.run())
}
