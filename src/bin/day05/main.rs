mod error;
mod models;

use advent_of_code_2024::{init, load_day_input};
use eyre::Context;
use models::{PageOrderList, PageRule, PageRuleList};
use tracing::info;

fn main() -> eyre::Result<()> {
    init();

    info!("loading data...");
    let data = load_day_input("day05.txt")?;

    info!("solving part 1...");
    let part_1_res = part1(&data)?;
    info!("Part 1 solution: {part_1_res}");

    info!("solving part 2...");
    let part_2_res = part2(&data)?;
    info!("Part 2 solution: {part_2_res}");

    Ok(())
}

fn part1(data: &str) -> eyre::Result<usize> {
    info!("Parsing data...");

    let (pages, rules) = load_from_data(data)?;

    let rv = pages
        .into_iter()
        .filter(|p| p.is_valid(rules.as_slice()))
        .map(|valid| valid.middle_page())
        .sum();
    Ok(rv)
}

fn part2(data: &str) -> eyre::Result<usize> {
    info!("Parsing data...");

    let (pages, rules) = load_from_data(data)?;

    let rv = pages
        .into_iter()
        .filter(|p| !p.is_valid(rules.as_slice()))
        .map(|unordered| unordered.fix(rules.as_slice()))
        .map(|fixed| fixed.middle_page())
        .sum();

    Ok(rv)
}

fn load_from_data(data: &str) -> eyre::Result<(Vec<PageOrderList>, PageRuleList)> {
    let rules = data
        .lines()
        .take_while(|l| !l.trim().is_empty())
        .map(|l| l.parse::<PageRule>())
        .collect::<Result<Vec<_>, _>>()
        .wrap_err("failed to parse list of page rules")?;

    let pages = data
        .lines()
        .skip_while(|l| !l.trim().is_empty())
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.parse::<PageOrderList>())
        .collect::<Result<Vec<_>, _>>()
        .wrap_err("failed to parse list of page orderings")?;

    Ok((pages, rules))
}
