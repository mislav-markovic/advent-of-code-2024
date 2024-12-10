use std::str::FromStr;

use rustc_hash::FxHashMap;

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
struct Coordinate {
    x: usize,
    y: usize,
}

type CoordinateNeighbours = [Option<Coordinate>; 4];
impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn neighbours(&self, within: &Dim) -> CoordinateNeighbours {
        let left = (self.x > 0).then(|| Self::new(self.x - 1, self.y));
        let right = (self.x < within.width - 1).then(|| Self::new(self.x + 1, self.y));
        let up = (self.y > 0).then(|| Self::new(self.x, self.y - 1));
        let down = (self.y < within.height - 1).then(|| Self::new(self.x, self.y + 1));

        [left, right, up, down]
    }
}

impl From<(usize, usize)> for Coordinate {
    fn from(value: (usize, usize)) -> Self {
        Self::new(value.0, value.1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Height {
    value: usize,
}

struct Map {
    positions: FxHashMap<Coordinate, Height>,
    dim: Dim
}

impl Map {
    fn new(positions: FxHashMap<Coordinate, Height>, dim: Dim) -> Self {
        Self { positions, dim }
    }
}

impl FromStr for Map {
    type Err = Day10Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
