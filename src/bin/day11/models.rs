use std::{hash::Hash, str::FromStr};

use rustc_hash::FxHashMap;

use crate::error::Day11Error;

#[derive(Debug, Clone, Eq)]
struct Stone {
    value: u128,
    digit_count: u8,
}

impl Hash for Stone {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl PartialEq for Stone {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl PartialOrd for Stone {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Stone {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl Stone {
    fn new(value: u128) -> Self {
        Self {
            value,
            digit_count: count_digits(value),
        }
    }

    fn replace_with(self, value: u128) -> Self {
        Self::new(value)
    }

    fn split(mut self) -> (Self, Self) {
        let mut left_value = 0u128;
        let mut right_value = 0u128;
        let mut digit_counter = 0u32;

        let digit_mid = self.digit_count / 2;
        let digit_mid = digit_mid as u32;

        while self.value > 0 {
            let digit = (self.value % 10) as u8;
            self.value /= 10;

            if digit_counter < digit_mid {
                let add = digit as u128 * 10u128.pow(digit_counter);
                right_value += add;
            } else {
                let add = digit as u128 * 10u128.pow(digit_counter - digit_mid);
                left_value += add;
            }

            digit_counter += 1;
        }

        (Self::new(left_value), Self::new(right_value))
    }
}

#[derive(Debug, Clone)]
enum StoneProcessResult {
    Split(Stone, Stone),
    Inc(Stone),
}

#[derive(Debug, Clone)]
struct StoneCacheItem {
    res: StoneProcessResult,
    count: usize,
}

impl StoneCacheItem {
    fn new(res: StoneProcessResult, initial_count: usize) -> Self {
        let count = initial_count;
        Self { res, count }
    }

    fn inc(&mut self, count: usize) {
        self.count += count
    }
    fn dec(&mut self, count: usize) {
        self.count -= count
    }
}

fn process_stone(stone: Stone) -> StoneProcessResult {
    if stone.value == 0 {
        StoneProcessResult::Inc(stone.replace_with(1))
    } else if stone.digit_count % 2 == 0 {
        let (left, right) = stone.split();
        StoneProcessResult::Split(left, right)
    } else {
        let Some(new) = stone.value.checked_mul(2024) else {
            panic!(
                "overflow detected! can not multiply {} with 2024",
                stone.value
            );
        };

        StoneProcessResult::Inc(stone.replace_with(new))
    }
}

pub(crate) struct StoneLine {
    stones: FxHashMap<Stone, StoneCacheItem>,
    blink_buf: Vec<(Stone, StoneProcessResult, usize)>,
    blink_count: usize,
}

impl StoneLine {
    fn new(line: Vec<Stone>) -> Self {
        let mut stones: FxHashMap<Stone, StoneCacheItem> = FxHashMap::default();

        for stone in line {
            stones
                .entry(stone.clone())
                .and_modify(|item| item.inc(1))
                .or_insert_with(|| StoneCacheItem::new(process_stone(stone), 1));
        }

        let blink_buf = Vec::new();
        let blink_count = 0;
        Self {
            stones,
            blink_buf,
            blink_count,
        }
    }

    pub(crate) fn blink(&mut self) {
        self.blink_count += 1;

        for (stone, res) in self.stones.iter().filter(|(_, item)| item.count > 0) {
            self.blink_buf
                .push((stone.clone(), res.res.clone(), res.count));
        }

        for (stone, res, count) in self.blink_buf.drain(..) {
            self.stones
                .get_mut(&stone)
                .expect("must exist at this point in cache")
                .dec(count);

            match res {
                StoneProcessResult::Split(left, right) => {
                    self.stones
                        .entry(left.clone())
                        .and_modify(|item| item.inc(count))
                        .or_insert_with(|| StoneCacheItem::new(process_stone(left), count));
                    self.stones
                        .entry(right.clone())
                        .and_modify(|item| item.inc(count))
                        .or_insert_with(|| StoneCacheItem::new(process_stone(right), count));
                }
                StoneProcessResult::Inc(new_stone) => {
                    self.stones
                        .entry(new_stone.clone())
                        .and_modify(|item| item.inc(count))
                        .or_insert_with(|| StoneCacheItem::new(process_stone(new_stone), count));
                }
            }
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.stones
            .values()
            .filter_map(|item| (item.count > 0).then_some(item.count))
            .sum()
    }
}

impl FromStr for StoneLine {
    type Err = Day11Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stones = s
            .split_ascii_whitespace()
            .map(|val| val.parse::<u128>().map(Stone::new))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|parse_err| Day11Error::StoneLineParseError {
                input: s.to_owned(),
                error_msg: format!("can not parse to u128! {parse_err}"),
            })?;

        Ok(Self::new(stones))
    }
}

fn count_digits(mut val: u128) -> u8 {
    if val == 0 {
        return 1;
    }

    let mut count = 0;
    while val > 0 {
        val /= 10;
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stone_100100_split_works() {
        let stone_100100 = Stone::new(100100);
        let (stone_left_100, stone_right_100) = stone_100100.split();
        assert_eq!(100, stone_left_100.value);
        assert_eq!(100, stone_right_100.value);
    }

    #[test]
    fn stone_1000_split_works() {
        let stone_1000 = Stone::new(1000);
        let (stone_10, stone_00) = stone_1000.split();
        assert_eq!(0, stone_00.value);
        assert_eq!(10, stone_10.value);
    }

    #[test]
    fn stone_99_split_works() {
        let stone_99 = Stone::new(99);
        let (stone_left_9, stone_right_9) = stone_99.split();
        assert_eq!(9, stone_left_9.value);
        assert_eq!(9, stone_right_9.value);
    }

    #[test]
    fn stone_correct_digit_count() {
        let stone = Stone::new(0);
        assert_eq!(1, stone.digit_count);

        let stone = stone.replace_with(10);
        assert_eq!(2, stone.digit_count);

        let stone = stone.replace_with(9999999);
        assert_eq!(7, stone.digit_count);

        let stone = stone.replace_with(1234);
        let (left, right) = stone.split();
        assert_eq!(2, left.digit_count);
        assert_eq!(2, right.digit_count);
    }

    #[test]
    fn stone_line_state_correct_after_1_blink() {
        let starting_stones = vec![Stone::new(125), Stone::new(17)];
        let mut stone_line = StoneLine::new(starting_stones.clone());

        assert_eq!(2, stone_line.len());

        for stone in starting_stones.iter() {
            let s = stone_line
                .stones
                .get(&stone)
                .expect("starting stone must exist");
            assert_eq!(1, s.count);
        }

        stone_line.blink();

        assert_eq!(3, stone_line.len());

        for stone in starting_stones.iter() {
            let s = stone_line
                .stones
                .get(&stone)
                .expect("starting stone must exist");
            assert_eq!(0, s.count);
        }

        let blink_1_expected_stones = vec![Stone::new(253000), Stone::new(1), Stone::new(7)];
        for stone in blink_1_expected_stones.iter() {
            let s = stone_line
                .stones
                .get(&stone)
                .expect("starting stone must exist");
            assert_eq!(1, s.count);
        }
    }

    #[test]
    fn split_example_works() {
        let num = 28676032;
        let stone = Stone::new(num);

        let (left, right) = stone.split();

        assert_eq!(2867, left.value);
        assert_eq!(6032, right.value);
    }

    #[test]
    fn blink_3_times_test() {
        let starting_stones = vec![Stone::new(125), Stone::new(17)];
        let mut stone_line = StoneLine::new(starting_stones.clone());

        assert_eq!(2, stone_line.len());

        stone_line.blink();
        assert_eq!(3, stone_line.len());

        stone_line.blink();
        assert_eq!(4, stone_line.len());

        stone_line.blink();
        assert_eq!(5, stone_line.len());

        let blink_1_expected_stones = vec![
            Stone::new(512072),
            Stone::new(1),
            Stone::new(20),
            Stone::new(24),
            Stone::new(28676032),
        ];
        for stone in blink_1_expected_stones.iter() {
            let s = stone_line
                .stones
                .get(&stone)
                .expect("starting stone must exist");
            assert_eq!(1, s.count);
        }
    }

    #[test]
    fn blink_splits_into_same_stone_works() {
        let starting_stones = vec![Stone::new(2020), Stone::new(20)];
        let mut stone_line = StoneLine::new(starting_stones.clone());

        assert_eq!(
            1,
            stone_line
                .stones
                .get(&Stone::new(2020))
                .expect("must exist")
                .count
        );

        assert_eq!(
            1,
            stone_line
                .stones
                .get(&Stone::new(20))
                .expect("must exist")
                .count
        );

        assert_eq!(2, stone_line.len());

        stone_line.blink();

        assert_eq!(4, stone_line.len());

        assert_eq!(
            0,
            stone_line
                .stones
                .get(&Stone::new(2020))
                .expect("must exist")
                .count
        );

        assert_eq!(
            2,
            stone_line
                .stones
                .get(&Stone::new(20))
                .expect("must exist")
                .count
        );
        assert_eq!(
            1,
            stone_line
                .stones
                .get(&Stone::new(2))
                .expect("must exist")
                .count
        );
        assert_eq!(
            1,
            stone_line
                .stones
                .get(&Stone::new(0))
                .expect("must exist")
                .count
        );
    }
}
