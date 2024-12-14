use std::{fmt::Display, str::FromStr};

use rustc_hash::{FxHashMap, FxHashSet};
use tracing::{info, warn};

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

            info!("Region {region_plant_type:?}; starting: {starting_coord:?}");

            let mut region_plots: Vec<Plot> = Vec::new();
            let mut next_steps = vec![starting_coord];
            let mut already_visited_in_same_region: FxHashSet<Coord> = FxHashSet::default();

            while let Some(current_coord) = next_steps.pop() {
                info!("Processing plot @ {current_coord:?}");

                if !already_visited_in_same_region.insert(current_coord.clone()) {
                    warn!("Plot @ {current_coord:?} already processed");
                    continue;
                }

                let neighbours = current_coord.cardinal_direction_neighbours(&garden.dimension);

                let mut border_count = 0;

                for neighbour in neighbours {
                    let Some(neighbour) = neighbour else {
                        border_count += 1;
                        info!("neighbour {neighbour:?} not in map");
                        continue;
                    };

                    debug_assert!(neighbour != current_coord, "can not have self as neighbour");

                    if let Some(plant_type) = garden.plots.get(&neighbour) {
                        if *plant_type == region_plant_type {
                            info!("neighbour {neighbour:?} our plant type!");
                            if !already_visited_in_same_region.contains(&neighbour) {
                                next_steps.push(neighbour);
                            }
                        } else {
                            info!("neighbour {neighbour:?} different plant type!");
                            border_count += 1;
                        }
                    } else {
                        info!("neighbour {neighbour:?} already processed!");
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
