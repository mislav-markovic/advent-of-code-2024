use std::str::FromStr;

use itertools::Itertools;
use tracing::{info_span, warn};

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
