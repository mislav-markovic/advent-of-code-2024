use std::{cmp, collections::VecDeque, str::FromStr};

use rustc_hash::{FxHashMap, FxHashSet};

use crate::error::Day10Error;

#[derive(Debug, Clone)]
struct Dim {
    width: usize,
    height: usize,
}

impl Dim {
    fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct Coordinate {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

type CoordinateNeighbours = [Option<Coordinate>; 4];
impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn cardinal_direction_neighbours(&self, within: &Dim) -> CoordinateNeighbours {
        let left = (self.x > 0).then(|| Self::new(self.x - 1, self.y));
        let right = (self.x + 1 < within.width).then(|| Self::new(self.x + 1, self.y));
        let up = (self.y > 0).then(|| Self::new(self.x, self.y - 1));
        let down = (self.y + 1 < within.height).then(|| Self::new(self.x, self.y + 1));

        [left, right, up, down]
    }
}

impl From<(usize, usize)> for Coordinate {
    fn from(value: (usize, usize)) -> Self {
        Self::new(value.0, value.1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Height {
    value: usize,
}

impl Height {
    const MAX: Height = Self::new(9);
    const MIN: Height = Self::new(0);

    const fn new(value: usize) -> Self {
        Self { value }
    }
}

pub(crate) struct Trailhead {
    start: Coordinate,
    score: usize,
}

impl Trailhead {
    fn new(start: Coordinate, score: usize) -> Self {
        Self { start, score }
    }

    pub(crate) fn start(&self) -> &Coordinate {
        &self.start
    }

    pub(crate) fn score(&self) -> usize {
        self.score
    }
}

struct TrailPos {
    start: Coordinate,
    current_height: Height,
    current_pos: Coordinate,
}

impl TrailPos {
    fn new(start: Coordinate, current_height: Height, current_pos: Coordinate) -> Self {
        Self {
            start,
            current_height,
            current_pos,
        }
    }

    fn from_start(start: Coordinate) -> Self {
        let current_height = Height::MIN;
        let current_pos = start.clone();
        Self {
            start,
            current_height,
            current_pos,
        }
    }

    fn next_pos(&self, pos: Coordinate) -> Self {
        Self::new(
            self.start.clone(),
            Height::new(self.current_height.value + 1),
            pos,
        )
    }

    fn is_complete(&self) -> bool {
        self.current_height == Height::MAX
    }

    fn can_step_to_height(&self, next_height: &Height) -> bool {
        (self.current_height.value + 1) == next_height.value
    }
}

pub(crate) struct Map {
    positions: FxHashMap<Coordinate, Height>,
    dim: Dim,
}

impl Map {
    fn new(positions: FxHashMap<Coordinate, Height>, dim: Dim) -> Self {
        Self { positions, dim }
    }

    #[cfg(test)]
    pub(crate) fn print(&self) -> String {
        let mut rv = String::new();
        for y in 0..self.dim.height {
            for x in 0..self.dim.width {
                let h = self
                    .positions
                    .get(&(x, y).into())
                    .expect("must have all coordinates within given dimensions");
                let h = h.value.to_string();
                assert!(h.len() == 1);

                rv.push_str(&h);
            }
            rv.push('\n');
        }
        rv.pop();
        rv
    }

    pub(crate) fn trailheads(&self) -> Vec<Trailhead> {
        let mut active_trails = self
            .positions
            .iter()
            .filter(|(_, h)| h == &&Height::MIN)
            .map(|(pos, _)| TrailPos::from_start(pos.clone()))
            .collect::<VecDeque<_>>();

        let mut completed_trails: Vec<TrailPos> = Vec::new();

        while let Some(trail) = active_trails.pop_front() {
            let neighbours = trail.current_pos.cardinal_direction_neighbours(&self.dim);

            for trail in neighbours
                .into_iter()
                // removes coords not in map
                .flatten()
                // renoves coors with wrong height
                .filter(|pos| {
                    trail.can_step_to_height(
                        self.positions.get(pos).expect("coord must have height"),
                    )
                })
                // advances trail for each valid direction
                .map(|next| trail.next_pos(next))
            {
                if trail.is_complete() {
                    completed_trails.push(trail);
                } else {
                    active_trails.push_back(trail);
                }
            }
        }

        let mut trailheads_map: FxHashMap<Coordinate, FxHashSet<Coordinate>> = FxHashMap::default();
        for trail in completed_trails {
            trailheads_map
                .entry(trail.start)
                .or_default()
                .insert(trail.current_pos);
        }

        trailheads_map
            .into_iter()
            .map(|(k, v)| Trailhead::new(k, v.len()))
            .collect()
    }
}

impl FromStr for Map {
    type Err = Day10Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width_tracker = 0;
        let mut height_tracker = 0;
        let mut map = FxHashMap::default();

        for (row, line) in s.lines().enumerate() {
            height_tracker = cmp::max(height_tracker, row);

            for (col, pos) in line.chars().enumerate() {
                width_tracker = cmp::max(width_tracker, col);

                let height = pos
                    .to_digit(10)
                    .ok_or(Day10Error::MapParseError {
                        input: pos.to_string(),
                        error_msg: "character not decimal digit".to_owned(),
                    })
                    .and_then(|digit| {
                        usize::try_from(digit).map_err(|_| Day10Error::MapParseError {
                            input: pos.to_string(),
                            error_msg: "can not convert to usize".to_owned(),
                        })
                    })?;

                let coord = Coordinate::new(col, row);
                let height = Height::new(height);
                if let Some(old) = map.insert(coord.clone(), height) {
                    return Err(Day10Error::MapParseError {
                        input: line.to_owned(),
                        error_msg: format!(
                            "overwritten value at ({}, {});{}",
                            coord.x, coord.y, old.value
                        ),
                    });
                };
            }
        }

        let dim = Dim::new(width_tracker + 1, height_tracker + 1);
        Ok(Self::new(map, dim))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coordinate_neighbours_correct() {
        let dim = Dim::new(8, 8);
        let coord = Coordinate::new(2, 3);

        let expected = [
            Some((1, 3).into()),
            Some((3, 3).into()),
            Some((2, 2).into()),
            Some((2, 4).into()),
        ];
        let neighbours = coord.cardinal_direction_neighbours(&dim);

        assert_eq!(expected, neighbours)
    }

    #[test]
    fn coordinate_neighbours_correct_when_on_left_edge() {
        let dim = Dim::new(8, 8);
        let coord = Coordinate::new(0, 3);

        let expected = [
            None,
            Some((1, 3).into()),
            Some((0, 2).into()),
            Some((0, 4).into()),
        ];
        let neighbours = coord.cardinal_direction_neighbours(&dim);

        assert_eq!(expected, neighbours)
    }

    #[test]
    fn coordinate_neighbours_correct_when_on_right_edge() {
        let dim = Dim::new(8, 8);
        let coord = Coordinate::new(7, 3);

        let expected = [
            Some((6, 3).into()),
            None,
            Some((7, 2).into()),
            Some((7, 4).into()),
        ];
        let neighbours = coord.cardinal_direction_neighbours(&dim);

        assert_eq!(expected, neighbours)
    }
}
