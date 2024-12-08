use std::{cmp, str::FromStr};

use itertools::Itertools;
use rustc_hash::FxHashMap;

use crate::error::Day08Error;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub(crate) struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Dimension {
    width: usize,
    height: usize,
}

impl Dimension {
    fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    fn contains(&self, pos: &Pos) -> bool {
        if let (Ok(x), Ok(y)) = (usize::try_from(pos.x), usize::try_from(pos.y)) {
            x <= self.width && y <= self.height
        } else {
            false
        }
    }
}

struct Vector {
    x: isize,
    y: isize,
}

impl Vector {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn reverse(&self) -> Self {
        Self::new(-self.x, -self.y)
    }

    fn apply(&self, from: &Pos) -> Pos {
        let x = from.x + self.x;
        let y = from.y + self.y;

        Pos::new(x, y)
    }

    fn from_to(from: &Pos, to: &Pos) -> Self {
        let x = to.x - from.x;
        let y = to.y - from.y;

        Self::new(x, y)
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct AntennaId(u8);

impl From<u8> for AntennaId {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl TryFrom<char> for AntennaId {
    type Error = Day08Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        u8::try_from(value)
            .map(AntennaId::from)
            .map_err(|cast_err| Day08Error::AntennaIdParseError {
                input: value.to_string(),
                error_msg: format!("could not convert char to antenna id: {cast_err}"),
            })
    }
}

#[derive(Clone, Debug)]
struct Antenna {
    _id: AntennaId,
    pos: Pos,
}

impl Antenna {
    fn new(id: AntennaId, pos: Pos) -> Self {
        Self { _id: id, pos }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) struct AntiNode {
    pos: Pos,
}

impl AntiNode {
    fn new(pos: Pos) -> Self {
        Self { pos }
    }

    fn for_frequency_pair(a: &Pos, b: &Pos) -> (Self, Self) {
        let vec = Vector::from_to(a, b);
        let short_b = vec.apply(b);

        let vec = vec.reverse();
        let short_a = vec.apply(a);

        (Self::new(short_a), Self::new(short_b))
    }

    pub(crate) fn pos(&self) -> &Pos {
        &self.pos
    }
}

pub(crate) struct CityMap {
    dim: Dimension,
    antennas: FxHashMap<AntennaId, Vec<Antenna>>,
}

impl CityMap {
    fn new(dim: Dimension, antennas: FxHashMap<AntennaId, Vec<Antenna>>) -> Self {
        Self { dim, antennas }
    }

    pub(crate) fn is_in_bounds(&self, pos: &Pos) -> bool {
        self.dim.contains(pos)
    }

    pub(crate) fn antinodes(&self) -> Vec<AntiNode> {
        let mut rv = Vec::new();

        for antenna_frequency in self.antennas.values() {
            let antinodes = antenna_frequency
                .iter()
                .cartesian_product(antenna_frequency.iter())
                .filter(|(l, r)| l.pos != r.pos)
                .flat_map(|(a, b)| {
                    let (a, b) = AntiNode::for_frequency_pair(&a.pos, &b.pos);
                    [a, b]
                });

            rv.extend(antinodes);
        }

        rv.shrink_to_fit();
        rv
    }

    pub(crate) fn resonant_antinodes(&self) -> Vec<AntiNode> {
        let mut rv = Vec::new();

        for antenna_frequency in self.antennas.values() {
            let antinodes = antenna_frequency
                .iter()
                .cartesian_product(antenna_frequency.iter())
                .filter(|(l, r)| l.pos != r.pos)
                .flat_map(|(a, b)| node_resonants(&a.pos, &b.pos, &self.dim))
                .map(AntiNode::new);

            rv.extend(antinodes);
        }

        rv.shrink_to_fit();
        rv
    }
}

fn node_resonants(a: &Pos, b: &Pos, within: &Dimension) -> Vec<Pos> {
    let mut rv = Vec::new();

    // from a to b
    let v = Vector::from_to(a, b);
    let mut next = v.apply(a);

    while within.contains(&next) {
        rv.push(next.clone());
        next = v.apply(&next);
    }

    // from b to a
    let v = Vector::from_to(b, a);
    let mut next = v.apply(b);

    while within.contains(&next) {
        rv.push(next.clone());
        next = v.apply(&next);
    }

    rv
}

impl FromStr for CityMap {
    type Err = Day08Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = 0usize;
        let mut height = 0usize;

        let mut map: FxHashMap<AntennaId, Vec<Antenna>> = FxHashMap::default();
        for (row, line) in s.lines().enumerate() {
            height = cmp::max(height, row);

            for (col, c) in line.chars().enumerate() {
                width = cmp::max(width, col);

                // empty spot on map
                if c == '.' {
                    continue;
                }

                let id: AntennaId = c.try_into()?;
                let (Ok(x), Ok(y)) = (col.try_into(), row.try_into()) else {
                    return Err(Day08Error::CityMapParseError {
                        input: line.to_owned(),
                        error_msg: format!("can not determine position of antenna @ {col}x{row}"),
                    });
                };
                let pos = Pos::new(x, y);
                let antenna = Antenna::new(id.clone(), pos);

                map.entry(id).or_default().push(antenna);
            }
        }

        let dim = Dimension::new(width, height);
        let res = CityMap::new(dim, map);
        Ok(res)
    }
}
