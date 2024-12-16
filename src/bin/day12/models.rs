use std::{fmt::Display, str::FromStr};

use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::error::Day12Error;

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

type CoordNeighbours = [Option<Coord>; 4];
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn cardinal_direction_neighbours(&self, within: &Dim) -> CoordNeighbours {
        let left = (self.x > 0).then(|| Self::new(self.x - 1, self.y));
        let right = (self.x + 1 < within.width).then(|| Self::new(self.x + 1, self.y));
        let up = (self.y > 0).then(|| Self::new(self.x, self.y - 1));
        let down = (self.y + 1 < within.height).then(|| Self::new(self.x, self.y + 1));

        [left, right, up, down]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct PlantType(u8);

impl Display for PlantType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = char::from(self.0);

        write!(f, "{c}")
    }
}

impl TryFrom<char> for PlantType {
    type Error = Day12Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        u8::try_from(value)
            .map_err(|_| Day12Error::PlantTypeParseError { input: value })
            .map(PlantType)
    }
}

struct Plot {
    pos: Coord,
    fenced_borders: usize,
}

impl Plot {
    fn new(pos: Coord, borders: usize) -> Self {
        Self {
            pos,
            fenced_borders: borders,
        }
    }
}

pub(crate) struct Region {
    plots: &'static [Plot],
    of: PlantType,
}

impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let plot_type = char::from(self.of.0);

        let area = self.area();
        let perimeter = self.perimeter();

        write!(f, "[{plot_type} A:{area} P:{perimeter}]")
    }
}

impl Region {
    fn new(plots: &'static [Plot], of: PlantType) -> Self {
        Self { plots, of }
    }

    fn area(&self) -> usize {
        self.plots.len()
    }

    fn perimeter(&self) -> usize {
        self.plots.iter().map(|p| p.fenced_borders).sum()
    }

    fn sides(&self) -> usize {
        let mut border_plots_by_x: FxHashMap<usize, Vec<usize>> = FxHashMap::default();
        let mut border_plots_by_y: FxHashMap<usize, Vec<usize>> = FxHashMap::default();

        let mut all_coords: FxHashSet<Coord> = FxHashSet::default();

        for plot in self.plots.iter() {
            let Coord { x, y } = plot.pos;

            all_coords.insert(plot.pos.clone());

            if plot.fenced_borders > 0 {
                border_plots_by_x
                    .entry(x)
                    .and_modify(|ys| ys.push(y))
                    .or_insert_with(|| vec![y]);

                border_plots_by_y
                    .entry(y)
                    .and_modify(|xs| xs.push(x))
                    .or_insert_with(|| vec![x]);
            }
        }

        for ys in border_plots_by_x.values_mut() {
            ys.sort_unstable();
        }

        for xs in border_plots_by_y.values_mut() {
            xs.sort_unstable();
        }

        fn has_left_fence(pos: &Coord, all_coords: &FxHashSet<Coord>) -> bool {
            pos.x == 0 || !all_coords.contains(&Coord::new(pos.x - 1, pos.y))
        }

        fn has_right_fence(pos: &Coord, all_coords: &FxHashSet<Coord>) -> bool {
            !all_coords.contains(&Coord::new(pos.x + 1, pos.y))
        }

        fn has_top_fence(pos: &Coord, all_coords: &FxHashSet<Coord>) -> bool {
            pos.y == 0 || !all_coords.contains(&Coord::new(pos.x, pos.y - 1))
        }

        fn has_bottom_fence(pos: &Coord, all_coords: &FxHashSet<Coord>) -> bool {
            !all_coords.contains(&Coord::new(pos.x, pos.y + 1))
        }

        fn count_sides<'a>(points: impl Iterator<Item = &'a usize>) -> usize {
            points.tuple_windows().fold(
                1,
                |counter, (a, b)| if a + 1 == *b { counter } else { counter + 1 },
            )
        }

        let mut total_sides = 0;

        // left | right fences
        for (x, ys) in border_plots_by_x.iter() {
            let left_sides = ys
                .iter()
                .filter(|y| has_left_fence(&Coord::new(*x, **y), &all_coords))
                .collect_vec();

            let right_sides = ys
                .iter()
                .filter(|y| has_right_fence(&Coord::new(*x, **y), &all_coords))
                .collect_vec();

            let left_count = if left_sides.is_empty() {
                0
            } else {
                count_sides(left_sides.into_iter())
            };

            let right_count = if right_sides.is_empty() {
                0
            } else {
                count_sides(right_sides.into_iter())
            };

            total_sides += left_count + right_count;
        }

        // top | bottom fences
        for (y, xs) in border_plots_by_y.iter() {
            let top_sides = xs
                .iter()
                .filter(|x| has_top_fence(&Coord::new(**x, *y), &all_coords))
                .collect_vec();

            let bottom_sides = xs
                .iter()
                .filter(|x| has_bottom_fence(&Coord::new(**x, *y), &all_coords))
                .collect_vec();

            let top_count = if top_sides.is_empty() {
                0
            } else {
                count_sides(top_sides.into_iter())
            };

            let bottom_count = if bottom_sides.is_empty() {
                0
            } else {
                count_sides(bottom_sides.into_iter())
            };

            total_sides += top_count + bottom_count;
        }

        total_sides
    }
}

pub(crate) struct FencedGarden {
    pub(crate) regions: &'static [Region],
}

impl FencedGarden {
    fn new(plots: &'static [Region]) -> Self {
        Self { regions: plots }
    }

    pub(crate) fn total_cost(&self) -> usize {
        self.regions.iter().map(|p| p.area() * p.perimeter()).sum()
    }

    pub(crate) fn discount_cost(&self) -> usize {
        self.regions.iter().map(|r| r.area() * r.sides()).sum()
    }
}

impl From<Garden> for FencedGarden {
    fn from(mut garden: Garden) -> Self {
        let mut regions = Vec::new();

        while let Some(key) = garden.plots.keys().next() {
            let starting_coord = key.clone();
            let region_plant_type = garden
                .plots
                .get(&starting_coord)
                .cloned()
                .expect("we just queried map for this key, it must exist");

            let mut region_plots: Vec<Plot> = Vec::new();
            let mut next_steps = vec![starting_coord];
            let mut already_visited_in_same_region: FxHashSet<Coord> = FxHashSet::default();

            while let Some(current_coord) = next_steps.pop() {
                if !already_visited_in_same_region.insert(current_coord.clone()) {
                    continue;
                }

                let neighbours = current_coord.cardinal_direction_neighbours(&garden.dimension);

                let mut border_count = 0;

                for neighbour in neighbours {
                    let Some(neighbour) = neighbour else {
                        border_count += 1;
                        continue;
                    };

                    debug_assert!(neighbour != current_coord, "can not have self as neighbour");

                    if let Some(plant_type) = garden.plots.get(&neighbour) {
                        if *plant_type == region_plant_type {
                            if !already_visited_in_same_region.contains(&neighbour) {
                                next_steps.push(neighbour);
                            }
                        } else {
                            border_count += 1;
                        }
                    } else {
                        border_count += 1;
                    }
                }

                debug_assert!(border_count <= 4, "border count max is 4");

                // make plot and add it to region
                let plot = Plot::new(current_coord, border_count);
                region_plots.push(plot);
            }

            for visited_plot in already_visited_in_same_region {
                garden.plots.remove(&visited_plot);
            }

            region_plots.shrink_to_fit();
            regions.push(Region::new(region_plots.leak(), region_plant_type));
        }

        regions.shrink_to_fit();
        Self::new(regions.leak())
    }
}

pub(crate) struct Garden {
    plots: FxHashMap<Coord, PlantType>,
    dimension: Dim,
}

impl Garden {
    fn new(plots: FxHashMap<Coord, PlantType>, dimension: Dim) -> Self {
        Self { plots, dimension }
    }
}

impl FromStr for Garden {
    type Err = Day12Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(col, c)| ((col, row), c))
            })
            .map(|((x, y), c)| PlantType::try_from(c).map(|p| (Coord::new(x, y), p)))
            .collect::<Result<FxHashMap<_, _>, _>>()?;

        let height = s.lines().count();
        let width = s.lines().next().unwrap().chars().count();
        let dim = Dim::new(width, height);

        Ok(Self::new(map, dim))
    }
}
