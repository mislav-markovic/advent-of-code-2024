use std::str::FromStr;

use itertools::Itertools;
use tracing::{info, info_span, warn};

use crate::error::Day04Error;

pub(crate) struct Puzzle {
    letters: Vec<Vec<u8>>,
}

impl Puzzle {
    fn new(letters: Vec<Vec<u8>>) -> Self {
        Self { letters }
    }

    pub(crate) fn count_pattern(&self, pattern: &[u8]) -> usize {
        let rows = self.letters.len();
        let cols = self.letters[0].len();

        let mut counter = 0;
        let mut temp_buf = Vec::<u8>::with_capacity(pattern.len());
        for row_idx in 0..rows {
            for col_idx in 0..cols {
                let span = info_span!("pattern match", row = row_idx, col = col_idx);
                let _span_guard = span.enter();

                let letter = self.letters[row_idx][col_idx];

                if letter == pattern[0] {
                    // check line forwards
                    if col_idx + pattern.len() <= cols
                        && *pattern == self.letters[row_idx][col_idx..(col_idx + pattern.len())]
                    {
                        counter += 1;
                    }

                    // check line backwards
                    if col_idx + 1 >= pattern.len() {
                        let start = col_idx + 1 - pattern.len();
                        let end = start + pattern.len();
                        let slice = &self.letters[row_idx][start..end];
                        if cmp_rev(pattern, slice) {
                            counter += 1;
                        }
                    }

                    // check col down
                    if row_idx + pattern.len() <= rows {
                        temp_buf.clear();

                        for i in 0..pattern.len() {
                            let letter = self.letters[row_idx + i][col_idx];
                            temp_buf.push(letter);
                        }

                        if pattern == &temp_buf[..] {
                            counter += 1;
                        }
                    }

                    // check col up
                    if row_idx + 1 >= pattern.len() {
                        temp_buf.clear();

                        for i in 0..pattern.len() {
                            let letter = self.letters[row_idx - i][col_idx];
                            temp_buf.push(letter);
                        }

                        if pattern == &temp_buf[..] {
                            counter += 1;
                        }
                    }

                    // check diag right down
                    if row_idx + pattern.len() <= rows && col_idx + pattern.len() <= cols {
                        temp_buf.clear();

                        for i in 0..pattern.len() {
                            let letter = self.letters[row_idx + i][col_idx + i];
                            temp_buf.push(letter);
                        }

                        if pattern == &temp_buf[..] {
                            counter += 1;
                        }
                    }

                    // check diag right up
                    if row_idx + 1 >= pattern.len() && col_idx + pattern.len() <= cols {
                        temp_buf.clear();

                        for i in 0..pattern.len() {
                            let letter = self.letters[row_idx - i][col_idx + i];
                            temp_buf.push(letter);
                        }

                        if pattern == &temp_buf[..] {
                            counter += 1;
                        }
                    }

                    // check diag left down
                    if row_idx + pattern.len() <= rows && col_idx + 1 >= pattern.len() {
                        temp_buf.clear();

                        for i in 0..pattern.len() {
                            let letter = self.letters[row_idx + i][col_idx - i];
                            temp_buf.push(letter);
                        }

                        if pattern == &temp_buf[..] {
                            counter += 1;
                        }
                    }

                    // check diag left up
                    if row_idx + 1 >= pattern.len() && col_idx + 1 >= pattern.len() {
                        temp_buf.clear();

                        for i in 0..pattern.len() {
                            let letter = self.letters[row_idx - i][col_idx - i];
                            temp_buf.push(letter);
                        }

                        if pattern == &temp_buf[..] {
                            counter += 1;
                        }
                    }
                }
            }
        }

        counter
    }

    pub(crate) fn count_x_pattern(&self, pattern: &[u8]) -> usize {
        let rows = self.letters.len();
        let cols = self.letters[0].len();
        let block_size = pattern.len();
        let block_size_inc = block_size - 1;
        let pattern_start = pattern.first().cloned().unwrap();
        let pattern_end = pattern.last().cloned().unwrap();
        let quick_check_elems = [pattern_start, pattern_end];

        info!("haystack: {rows}x{cols}");
        info!("needle: {block_size} (+{block_size_inc})");
        info!(
            "quick check: {:?}",
            quick_check_elems
                .iter()
                .cloned()
                .map(char::from)
                .collect::<Vec<_>>()
        );

        let mut counter = 0;
        let mut left_diag_buf = Vec::<u8>::with_capacity(block_size);
        let mut right_diag_buf = Vec::<u8>::with_capacity(block_size);
        for block_row_start in 0..(rows - block_size_inc) {
            for block_col_start in 0..(cols - block_size_inc) {
                let block_span = info_span!("", row = block_row_start, col = block_col_start);
                let _block_span_guard = block_span.enter();

                info!("enter block");

                // quick check corners for fast failure
                let left_up = self.letters[block_row_start][block_col_start];
                let right_up = self.letters[block_row_start][block_col_start + block_size_inc];
                let left_down = self.letters[block_row_start + block_size_inc][block_col_start];
                let right_down = self.letters[block_row_start + block_size_inc]
                    [block_col_start + block_size_inc];

                if !quick_check_elems.contains(&left_up)
                    || !quick_check_elems.contains(&right_up)
                    || !quick_check_elems.contains(&left_down)
                    || !quick_check_elems.contains(&right_down)
                {
                    warn!("qucik check discard");
                    continue;
                }

                // candidate block, do full check
                left_diag_buf.clear();
                right_diag_buf.clear();
                for i in 0..block_size {
                    let left_letter = self.letters[block_row_start + i][block_col_start + i];
                    let right_letter =
                        self.letters[block_row_start + i][block_col_start + block_size_inc - i];
                    left_diag_buf.push(left_letter);
                    right_diag_buf.push(right_letter);
                }

                if is_match(pattern, &left_diag_buf) && is_match(pattern, &right_diag_buf) {
                    counter += 1;
                    info!("is match");
                } else {
                    warn!("full check discard");
                }
            }
        }

        counter
    }
}

fn is_match(pattern: &[u8], search: &[u8]) -> bool {
    pattern == search || cmp_rev(pattern, search)
}

fn cmp_rev(pattern: &[u8], rev_search: &[u8]) -> bool {
    rev_search
        .iter()
        .rev()
        .zip_eq(pattern.iter())
        .all(|(lhs, rhs)| lhs == rhs)
}

impl FromStr for Puzzle {
    type Err = Day04Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .trim()
            .lines()
            .map(|l| l.trim())
            .map(|l| l.chars().map(u8::try_from).collect::<Result<Vec<_>, _>>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| Day04Error::PuzzleParseError {
                input: "".to_owned(),
                error_msg: e.to_string(),
            })?;

        Ok(Puzzle::new(data))
    }
}
