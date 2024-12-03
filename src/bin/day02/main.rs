mod error;
mod models;

use advent_of_code_2024::{init, load_day_input};
use eyre::Context;
use models::{make_diff_rule, make_sorted_rule, SafetyRules, UnusualData};
use tracing::info;

fn main() -> eyre::Result<()> {
    init();

    info!("loading data...");
    let data = load_day_input("day02.txt")?;

    info!("solving part 1...");
    let part_1_res = part1(&data)?;
    info!("Part 1 solution: {part_1_res}");

    info!("solving part 2...");
    let part_2_res = part2(&data)?;
    info!("Part 2 solution: {part_2_res}");

    Ok(())
}

fn part1(data: &str) -> eyre::Result<usize> {
    let data = data
        .parse::<UnusualData>()
        .wrap_err("failed to parse data")?;
    let maker = || SafetyRules::new(vec![make_sorted_rule(), make_diff_rule()]);

    let safe_count = data.count_safe_with_rules(maker);

    Ok(safe_count)
}

fn part2(data: &str) -> eyre::Result<usize> {
    Ok(0)
}
