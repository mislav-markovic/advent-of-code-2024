use core::panic;
use std::{cmp::Reverse, collections::BinaryHeap, str::FromStr};

use crate::error::Day09Error;

enum BlockType {
    File,
    Free,
}

impl BlockType {
    fn flip(self) -> Self {
        match self {
            Self::File => Self::Free,
            Self::Free => Self::File,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct MemoryRegion {
    from: usize,
    to: usize,
}

impl MemoryRegion {
    fn from_to(from: usize, to: usize) -> Self {
        match from.cmp(&to) {
            std::cmp::Ordering::Less => Self { from, to },
            std::cmp::Ordering::Equal => panic!("memory region can not be empty! {from} == {to}"),
            std::cmp::Ordering::Greater => {
                panic!("memory region reversed! to < from: {to} < {from}")
            }
        }
    }

    fn from_with_count(from: usize, count: usize) -> Self {
        let to = from + count;
        Self::from_to(from, to)
    }

    fn size(&self) -> usize {
        self.to - self.from
    }

    fn allocate(
        self,
        how_much: usize,
        value: usize,
    ) -> Result<(AllocatedRegion, Option<Self>), Day09Error> {
        let size = self.size();

        if size < how_much {
            return Err(Day09Error::AllocationError {
                how_much,
                value,
                from: self.from,
                to: self.to,
            });
        }

        let alloc_region = Self::from_with_count(self.from, how_much);
        let leftover = (size > how_much).then_some(Self::from_to(self.from + how_much, self.to));
        let alloc = AllocatedRegion::new(alloc_region, value);

        Ok((alloc, leftover))
    }

    fn split_off(self, how_much: usize) -> (Self, Self) {
        if how_much >= self.size() {
            panic!(
                "memory region split_at_index greater than region size! {how_much} >= {}",
                self.size()
            );
        }

        let remaining_size = self.size() - how_much;
        (
            Self::from_with_count(self.from, remaining_size),
            Self::from_to(self.from + how_much, self.to),
        )
    }
}

impl PartialOrd for MemoryRegion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MemoryRegion {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.from.cmp(&other.from)
    }
}

enum ReallocMemResult {
    DataRemainig(AllocatedRegion, MemoryRegion),
    FreeSpaceRemaining(MemoryRegion, MemoryRegion),
    ExactMove(MemoryRegion),
}

#[derive(Debug, Eq)]
struct AllocatedRegion {
    region: MemoryRegion,
    value: usize,
}

impl AllocatedRegion {
    fn new(region: MemoryRegion, value: usize) -> Self {
        Self { region, value }
    }

    fn checksum(&self) -> u64 {
        (self.region.from..self.region.to)
            .map(|pos| (pos * self.value) as u64)
            .sum()
    }

    fn reallocate(self, into: MemoryRegion) -> (Self, ReallocMemResult) {
        match self.region.size().cmp(&into.size()) {
            std::cmp::Ordering::Less => {
                // we were given more space than needed
                let (moved_data, lefotover_space) = into
                    .allocate(self.region.size(), self.value)
                    .expect("alloc with enough space to work");
                let original_space = self.region;
                let lefotover_space =
                    lefotover_space.expect("we know there must be some space left");
                (
                    moved_data,
                    ReallocMemResult::FreeSpaceRemaining(original_space, lefotover_space),
                )
            }
            std::cmp::Ordering::Equal => {
                // we were given exact amount of space
                let moved_data = Self::new(into, self.value);
                (moved_data, ReallocMemResult::ExactMove(self.region))
            }
            std::cmp::Ordering::Greater => {
                // we were given too little space
                let (still_occupied, freed_space) = self.region.split_off(into.size());
                let moved_data = Self::new(into, self.value);
                let still_occupied = Self::new(still_occupied, self.value);

                (
                    moved_data,
                    ReallocMemResult::DataRemainig(still_occupied, freed_space),
                )
            }
        }
    }
}

impl PartialEq for AllocatedRegion {
    fn eq(&self, other: &Self) -> bool {
        self.region.eq(&other.region)
    }
}

impl PartialOrd for AllocatedRegion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AllocatedRegion {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.region.cmp(&other.region)
    }
}

pub(crate) struct DiskMap {
    // min-heap -> we need to lookup earliest free mem regions
    free_space: BinaryHeap<Reverse<MemoryRegion>>,
    // max-heap -> we want to lookup allocs at the end first to move them into earlier free space
    allocs: BinaryHeap<AllocatedRegion>,
}

impl DiskMap {
    fn new(
        free_space: impl Iterator<Item = MemoryRegion>,
        allocs: impl Iterator<Item = AllocatedRegion>,
    ) -> Self {
        let allocs = allocs.collect();
        let free_space = free_space.map(Reverse).collect();
        Self { free_space, allocs }
    }

    pub(crate) fn checksum(&self) -> u64 {
        self.allocs.iter().map(|a| a.checksum()).sum()
    }

    pub(crate) fn compact_memory(&mut self) {
        if self.free_space.is_empty() || self.allocs.is_empty() {
            return;
        }

        while self.free_space.peek().map_or(false, |earlies_free_region| {
            earlies_free_region.0
                < self
                    .allocs
                    .peek()
                    .expect("must have some allocs at this point")
                    .region
        }) {
            let first_region = self
                .free_space
                .pop()
                .expect("we check in loop condition that we have free mem regions")
                .0;
            let last_alloc = self.allocs.pop().expect("must have some alloc");

            let (moved_alloc, res) = last_alloc.reallocate(first_region);
            self.allocs.push(moved_alloc);

            match res {
                ReallocMemResult::DataRemainig(allocated_region, memory_region) => {
                    self.allocs.push(allocated_region);
                    self.free_space.push(Reverse(memory_region));
                }
                ReallocMemResult::FreeSpaceRemaining(free_mem_1, free_mem_2) => {
                    self.free_space.push(Reverse(free_mem_1));
                    self.free_space.push(Reverse(free_mem_2));
                }
                ReallocMemResult::ExactMove(memory_region) => {
                    self.free_space.push(Reverse(memory_region));
                }
            };
        }
    }

    pub(crate) fn defragment(&mut self) {
        if self.free_space.is_empty() || self.allocs.is_empty() {
            return;
        }

        let mut seen_allocs = Vec::new();
        let mut free_mem_holder = Vec::new();
        while let Some(alloc) = self.allocs.pop() {
            if !free_mem_holder.is_empty() {
                self.free_space
                    .extend(free_mem_holder.drain(..).map(Reverse));
            }

            // either we no longer have any free space, or first free space is after last
            // allocation
            if self
                .free_space
                .peek()
                .map_or(true, |mem| mem.0 > alloc.region)
            {
                seen_allocs.push(alloc);
                break;
            }

            let mut target_region: Option<MemoryRegion> = None;
            // while we have free memory regions and they are before allocation we are trying to
            // move
            while self
                .free_space
                .peek()
                .map_or(false, |mem| mem.0 < alloc.region)
            {
                // pop it from collection, it is either what we are looking for, or it is not
                // needed for moving this allocation
                let candidate_mem = self.free_space.pop().unwrap().0;

                if candidate_mem.size() >= alloc.region.size() {
                    target_region = Some(candidate_mem);
                    break;
                }

                free_mem_holder.push(candidate_mem);
            }

            // we didn't find large enoung memory region to move this allocation
            let Some(target_region) = target_region else {
                seen_allocs.push(alloc);
                continue;
            };

            let (moved_alloc, res) = alloc.reallocate(target_region);
            seen_allocs.push(moved_alloc);

            match res {
                ReallocMemResult::DataRemainig(_, _) => {
                    panic!(
                        "we partially moved allocation during defragment, this should never happen"
                    )
                }
                ReallocMemResult::FreeSpaceRemaining(free_mem_1, free_mem_2) => {
                    self.free_space.push(Reverse(free_mem_1));
                    self.free_space.push(Reverse(free_mem_2));
                }
                ReallocMemResult::ExactMove(memory_region) => {
                    self.free_space.push(Reverse(memory_region));
                }
            };
        }

        // return resources to their collections
        self.free_space
            .extend(free_mem_holder.drain(..).map(Reverse));
        self.allocs.extend(seen_allocs);
    }

    #[cfg(test)]
    pub(crate) fn show(&self) -> String {
        let last_idx = self.allocs.peek().unwrap().region.to;

        let allocated_iter = self.allocs.iter().map(|alloc| {
            (
                alloc.region.from,
                alloc.region.size(),
                alloc.value.to_string(),
            )
        });
        let free_iter = self
            .free_space
            .iter()
            .filter(|free| free.0.from < last_idx)
            .map(|reg| (reg.0.from, reg.0.size(), ".".to_owned()));

        let mut vec = allocated_iter.collect::<Vec<_>>();
        vec.extend(free_iter);

        vec.sort_by_key(|e| e.0);

        vec.into_iter()
            .map(|(_, count, s)| s.repeat(count))
            .collect::<String>()
    }
}

impl FromStr for DiskMap {
    type Err = Day09Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut block_type = BlockType::Free;
        let mut file_id = 0;
        let mut memory_cursor = 0;

        let mut allocs = Vec::new();
        let mut free_space = Vec::new();

        for num in s.trim().chars() {
            block_type = block_type.flip();
            let length = num
                .to_digit(10)
                .ok_or(Day09Error::DiskMapParseError {
                    input: num.to_string(),
                    error_msg: "could not parse digit".to_owned(),
                })
                .and_then(|digit| {
                    usize::try_from(digit).map_err(|_cast_err| Day09Error::DiskMapParseError {
                        input: num.to_string(),
                        error_msg: "could not cast digit to usize".to_owned(),
                    })
                })?;

            // handle length zero, either skip or error
            if length == 0 {
                if let BlockType::Free = block_type {
                    continue;
                } else {
                    return Err(Day09Error::DiskMapParseError {
                        input: num.to_string(),
                        error_msg: "zero-sized file is error".to_owned(),
                    });
                }
            }

            let region = MemoryRegion::from_with_count(memory_cursor, length);
            memory_cursor += length;
            match block_type {
                BlockType::File => {
                    let alloc = AllocatedRegion::new(region, file_id);
                    allocs.push(alloc);
                    file_id += 1;
                }
                BlockType::Free => free_space.push(region),
            }
        }

        Ok(Self::new(free_space.into_iter(), allocs.into_iter()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mem_region_() {}
}
