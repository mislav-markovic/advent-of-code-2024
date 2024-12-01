use std::{cmp::Reverse, collections::BinaryHeap, str::FromStr};

use crate::error::Day01Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct LocationId {
    id: u64,
}

impl LocationId {
    pub(crate) fn new(id: u64) -> Self {
        Self { id }
    }

    pub(crate) fn distance(&self, other: &LocationId) -> u64 {
        self.id.abs_diff(other.id)
    }
}

impl FromStr for LocationId {
    type Err = Day01Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = s
            .trim()
            .parse::<u64>()
            .map_err(|e| Day01Error::LocationIdParseError {
                input: s.to_owned(),
                error_msg: e.to_string(),
            })?;

        Ok(LocationId::new(parsed))
    }
}

pub(crate) struct LocationList {
    ids: BinaryHeap<Reverse<LocationId>>,
}

impl LocationList {
    pub(crate) fn from_ids(ids: impl Iterator<Item = LocationId>) -> Self {
        let ids = ids.map(Reverse).collect::<BinaryHeap<_>>();

        Self { ids }
    }

    pub(crate) fn into_iter(self) -> impl Iterator<Item = LocationId> {
        LocationListIterator::new(self.ids)
    }
}

pub(crate) struct LocationListIterator {
    ids: BinaryHeap<Reverse<LocationId>>,
}

impl LocationListIterator {
    fn new(ids: BinaryHeap<Reverse<LocationId>>) -> Self {
        Self { ids }
    }
}

impl Iterator for LocationListIterator {
    type Item = LocationId;

    fn next(&mut self) -> Option<Self::Item> {
        self.ids.pop().map(|rev_id| rev_id.0)
    }
}

pub(crate) struct SideBySide {
    lhs: LocationList,
    rhs: LocationList,
}

impl SideBySide {
    pub(crate) fn new(lhs: LocationList, rhs: LocationList) -> Self {
        Self { lhs, rhs }
    }

    pub(crate) fn compute_total_distance(self) -> u64 {
        self.lhs
            .into_iter()
            .zip(self.rhs.into_iter())
            .map(|(lhs, rhs)| lhs.distance(&rhs))
            .sum()
    }
}

impl FromStr for SideBySide {
    type Err = Day01Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut first_list = Vec::new();
        let mut second_list = Vec::new();

        for line in s.lines() {
            if let Some((first, second)) = line.split_once(' ') {
                let first = first.trim().parse()?;
                let second = second.trim().parse()?;

                first_list.push(first);
                second_list.push(second)
            }
        }

        let first_list = LocationList::from_ids(first_list.into_iter());
        let second_list = LocationList::from_ids(second_list.into_iter());
        Ok(SideBySide::new(first_list, second_list))
    }
}
