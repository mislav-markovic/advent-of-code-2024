use core::panic;
use std::{collections::VecDeque, str::FromStr};

use crate::error::Day11Error;

#[derive(Debug)]
struct Stone {
    value: u128,
    digit_count: u8,
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
                right_value = right_value + add;
            } else {
                let add = digit as u128 * 10u128.pow(digit_counter - digit_mid);
                left_value = left_value + add;
            }

            digit_counter += 1;
        }

        (Self::new(left_value), Self::new(right_value))
    }
}

pub(crate) struct StoneLine {
    line: VecDeque<Stone>,
}

impl StoneLine {
    fn new(line: VecDeque<Stone>) -> Self {
        Self { line }
    }

    pub(crate) fn blink(&mut self) {
        let current_len = self.len();

        for _ in 0..current_len {
            let Some(stone) = self.line.pop_front() else {
                panic!("expected to find {current_len} items in line, but we ran out")
            };

            // rules in order of priority
            if stone.value == 0 {
                self.line.push_back(stone.replace_with(1));
            } else if stone.digit_count % 2 == 0 {
                let (left, right) = stone.split();
                self.line.push_back(left);
                self.line.push_back(right);
            } else {
                let Some(new) = stone.value.checked_mul(2024) else {
                    panic!(
                        "overflow detected! can not multiply {} with 2024",
                        stone.value
                    );
                };

                self.line.push_back(stone.replace_with(new));
            }
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.line.len()
    }
}

impl FromStr for StoneLine {
    type Err = Day11Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stones = s
            .split_ascii_whitespace()
            .map(|val| val.parse::<u128>().map(|v| Stone::new(v)))
            .collect::<Result<VecDeque<_>, _>>()
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
}
