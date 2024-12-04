mod error;
mod models;

use advent_of_code_2024::{init, load_day_input};
use eyre::Context;
use models::Puzzle;
use tracing::info;

fn main() -> eyre::Result<()> {
    init();

    info!("loading data...");
    let data = load_day_input("day04.txt")?;

    info!("solving part 1...");
    let part_1_res = part1(&data)?;
    info!("Part 1 solution: {part_1_res}");

    info!("solving part 2...");
    let part_2_res = part2(&data)?;
    info!("Part 2 solution: {part_2_res}");

    Ok(())
}

fn part1(data: &str) -> eyre::Result<usize> {
    const XMAS: &[u8] = b"XMAS";

    let res = data
        .parse::<Puzzle>()
        .wrap_err("Failed to parse word puzzle")?
        .count_pattern(XMAS);

    Ok(res)
}

fn part2(data: &str) -> eyre::Result<usize> {
    const MAS: &[u8] = b"MAS";

    let res = data
        .parse::<Puzzle>()
        .wrap_err("Failed to parse word puzzle")?
        .count_x_pattern(MAS);

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part_1_sample_data() {
        let part1_res = part1(SAMPLE).unwrap();

        assert_eq!(18, part1_res);
    }

    #[test]
    fn part_2_sample_data() {
        let part2_res = part2(SAMPLE).unwrap();

        assert_eq!(9, part2_res);
    }
}
