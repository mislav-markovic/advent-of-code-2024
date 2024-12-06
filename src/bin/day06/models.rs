use std::{fmt::Debug, str::FromStr};

use rustc_hash::FxHashSet;
use tracing::debug;

use crate::error::Day06Error;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MapPosition {
    Empty,
    Obstacle,
}

#[derive(Clone)]
pub(crate) struct Map {
    content: Vec<Vec<MapPosition>>,
    guard_starting_position: (usize, usize),
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "map {}x{} guard @ {}x{}",
            self.content[0].len(),
            self.content.len(),
            self.guard_starting_position.0,
            self.guard_starting_position.1
        )
    }
}

impl FromStr for Map {
    type Err = Day06Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const EMPTY_CHAR: char = '.';
        const OBSTACLE_CHAR: char = '#';
        const GUARD_CHAR: char = '^';

        let mut guard_pos: Option<(usize, usize)> = None;
        let mut content: Vec<Vec<MapPosition>> = Vec::with_capacity(1 << 10);

        for (row, line) in s.lines().enumerate() {
            let mut row_content = Vec::with_capacity(line.len());

            for (col, sym) in line.chars().enumerate() {
                match sym {
                    EMPTY_CHAR => row_content.push(MapPosition::Empty),
                    OBSTACLE_CHAR => row_content.push(MapPosition::Obstacle),
                    GUARD_CHAR => {
                        if let Some(guard) = guard_pos {
                            return Err(Day06Error::GuardDoubleDefinedError {
                                first_post: guard,
                                second_pos: (col, row),
                            });
                        }

                        guard_pos = Some((col, row));

                        row_content.push(MapPosition::Empty);
                    }
                    _ => {
                        return Err(Day06Error::MapParseError {
                            input: sym.to_string(),
                            error_msg: "unkown symbol".to_string(),
                        })
                    }
                }
            }

            row_content.shrink_to_fit();
            content.push(row_content);
        }

        let Some(guard_pos) = guard_pos else {
            return Err(Day06Error::GuardMissingInitialPosition);
        };

        content.shrink_to_fit();
        Ok(Map::new(content, guard_pos))
    }
}

impl Map {
    fn new(content: Vec<Vec<MapPosition>>, guard_starting_position: (usize, usize)) -> Self {
        Self {
            content,
            guard_starting_position,
        }
    }

    pub(crate) fn at(&self, x: i32, y: i32) -> Option<MapPosition> {
        let (Ok(x), Ok(y)) = (usize::try_from(x), usize::try_from(y)) else {
            return None;
        };

        self.content.get(y).and_then(|row| row.get(x)).cloned()
    }

    pub(crate) fn set_at_position(&mut self, x: usize, y: usize, new_val: MapPosition) {
        self.content[y][x] = new_val;
    }

    pub(crate) fn dimension(&self) -> (usize, usize) {
        let width = self.content[0].len();
        let height = self.content.len();

        (width, height)
    }

    pub(crate) fn guard_starting_position(&self) -> (usize, usize) {
        self.guard_starting_position
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Guard {
    position: (usize, usize),
    facing: Orientation,
}

impl Guard {
    pub(crate) fn new(position: (usize, usize), facing: Orientation) -> Self {
        Self { position, facing }
    }

    fn next_step(&self) -> (i32, i32) {
        let x = self.position.0 as i32;
        let y = self.position.1 as i32;

        match self.facing {
            Orientation::Up => (x, y - 1),
            Orientation::Down => (x, y + 1),
            Orientation::Left => (x - 1, y),
            Orientation::Right => (x + 1, y),
        }
    }

    fn make_step(&mut self) -> Result<(), Day06Error> {
        let next = self.next_step();

        let (Ok(x), Ok(y)) = (usize::try_from(next.0), usize::try_from(next.1)) else {
            return Err(Day06Error::GuardStepError {
                current_position: self.position,
                next_position: next,
            });
        };

        self.position = (x, y);

        Ok(())
    }

    fn rotate(&mut self) {
        let next = match self.facing {
            Orientation::Up => Orientation::Right,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
            Orientation::Right => Orientation::Down,
        };

        self.facing = next;
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct LoopDetectionPosition((usize, usize), Orientation);

impl From<&Guard> for LoopDetectionPosition {
    fn from(value: &Guard) -> Self {
        Self(value.position, value.facing)
    }
}

pub(crate) fn simulate_guard_movement(
    mut guard: Guard,
    map: Map,
) -> Result<Vec<(usize, usize)>, Day06Error> {
    let mut step_seq = Vec::with_capacity(1 << 12);
    step_seq.push(guard.position);

    let mut loop_detector: FxHashSet<LoopDetectionPosition> = FxHashSet::default();
    loop_detector.insert((&guard).into());

    let mut rotation_count = 0;
    loop {
        if rotation_count >= 3 {
            return Err(Day06Error::MovementSimulationError {
                why: format!("rotated {rotation_count} without making step. spinning in circle"),
            });
        }

        let next_planned_step = guard.next_step();

        let Some(next_content) = map.at(next_planned_step.0, next_planned_step.1) else {
            debug!(
                "next step will take us out of map! guard = {:?}; step = {}x{}",
                guard, next_planned_step.0, next_planned_step.1
            );
            break;
        };

        debug!(
            "{next_content:?} @ {}x{}",
            next_planned_step.0, next_planned_step.1
        );

        match next_content {
            MapPosition::Empty => {
                guard.make_step()?;
                debug!("moved to {}x{}", guard.position.0, guard.position.1);
                step_seq.push(guard.position);

                if !loop_detector.insert((&guard).into()) {
                    return Err(Day06Error::SimulationLoopError);
                }

                rotation_count = 0;
            }
            MapPosition::Obstacle => {
                debug!(
                    "obstalce @ {}x{}; facing {:?}",
                    next_planned_step.0, next_planned_step.1, guard.facing
                );
                guard.rotate();
                rotation_count += 1;
            }
        };
    }

    Ok(step_seq)
}
