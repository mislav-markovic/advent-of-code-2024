mod error;
mod models;

use std::time::Instant;

use advent_of_code_2024::{init, load_day_input};
use eyre::Context;
use models::DiskMap;
use tracing::info;

fn main() -> eyre::Result<()> {
    init();

    info!("loading data...");
    let data = load_day_input("day09.txt")?;

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
    let mut disk_map = data.parse::<DiskMap>().wrap_err("failed to parse input")?;

    info!("defragment disk...");
    disk_map.defragment();

    info!("get disk checksum");
    let checksum = disk_map.checksum();

    Ok(checksum)
}

fn part2(data: &str) -> eyre::Result<u64> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const SAMPLE: &str = "2333133121414131402";

    #[test]
    fn short_example() {
        const INPUT: &str = "12345";
        const EXPECTED: &str = "022111222";

        let mut disk = INPUT.parse::<DiskMap>().expect("input parse to work");
        disk.defragment();
        let result = disk.show();

        assert_eq!(EXPECTED, result)
    }

    #[test]
    fn long_example_correctly_parsed() {
        const EXPECTED: &str = "00...111...2...333.44.5555.6666.777.888899";

        let disk = SAMPLE.parse::<DiskMap>().expect("input parse to work");
        let original = disk.show();

        assert_eq!(EXPECTED, original)
    }

    #[test]
    fn long_example_correctly_defragged() {
        const EXPECTED: &str = "0099811188827773336446555566";

        let mut disk = SAMPLE.parse::<DiskMap>().expect("input parse to work");
        disk.defragment();
        let original = disk.show();

        assert_eq!(EXPECTED, original)
    }

    #[test]
    fn part_1_sample_data() {
        let res = part1(SAMPLE).expect("part 1 not to error on sample data");

        assert_eq!(1928, res);
    }

    #[test]
    fn part_2_sample_data() {
        let res = part2(SAMPLE).expect("part 2 not to error on sample data");

        // TODO when we get example for part 2
        assert_eq!(0, res);
    }
}
