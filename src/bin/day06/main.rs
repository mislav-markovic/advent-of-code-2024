mod error;
mod models;

use std::{collections::HashSet, time::Instant};

use advent_of_code_2024::{init, load_day_input};
use error::Day06Error;
use eyre::Context;
use models::{simulate_guard_movement, Guard, Map, MapPosition};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tracing::info;

fn main() -> eyre::Result<()> {
    init();

    info!("loading data...");
    let data = load_day_input("day06.txt")?;

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
    info!("parsing map...");
    let map = data.parse::<Map>().wrap_err("failed to parse map")?;

    let guard = Guard::new(map.guard_starting_position(), models::Orientation::Up);
    info!("found guard at {:?}", guard);

    info!("simulating guard movements...");
    let steps =
        simulate_guard_movement(guard, map).wrap_err("failed to simualte guard movements")?;

    let distinct = steps.into_iter().collect::<HashSet<_>>();

    Ok(distinct.len())
}

fn part2(data: &str) -> eyre::Result<usize> {
    info!("parsing map...");
    let map = data.parse::<Map>().wrap_err("failed to parse map")?;

    let guard = Guard::new(map.guard_starting_position(), models::Orientation::Up);
    info!("found guard at {:?}", guard);

    info!("prepare map candidates with new obstacle placed...");
    let maps = prepare_multi_map(&map);

    info!("simulating guard movements to find loops...");
    let loops = maps
        .into_par_iter()
        .filter_map(|m| is_looped(m, guard).then_some(()))
        .count();

    Ok(loops)
}

fn is_looped(map: Map, guard: Guard) -> bool {
    match simulate_guard_movement(guard, map) {
        Ok(_) => false,
        Err(Day06Error::SimulationLoopError) => true,
        _ => panic!("unexpected simulation result"),
    }
}

fn prepare_multi_map(original: &Map) -> Vec<Map> {
    let guard_start = original.guard_starting_position();
    let (width, height) = original.dimension();

    (0..width)
        .flat_map(|x| (0..height).map(move |y| (x, y)))
        .filter(|(x, y)| {
            (*x, *y) != guard_start
                && original
                    .at(*x as i32, *y as i32)
                    .expect("must be valid map position")
                    == MapPosition::Empty
        })
        .map(|(x, y)| {
            let mut new = original.clone();
            new.set_at_position(x, y, MapPosition::Obstacle);
            new
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part_1_sample_data() {
        let res = part1(SAMPLE).expect("part 1 not to error on sample data");

        assert_eq!(41, res);
    }
}
