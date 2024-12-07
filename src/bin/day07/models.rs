use std::fmt::Display;
use std::str::FromStr;

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use tracing::{debug, info, warn};

use crate::error::Day07Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operator {
    Add,
    Mul,
}

impl Operator {
    fn apply(&self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Operator::Add => lhs + rhs,
            Operator::Mul => lhs * rhs,
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            Operator::Add => "+",
            Operator::Mul => "*",
        };

        write!(f, "{v}")
    }
}

pub(crate) struct Equation {
    test_value: u64,
    operands: Box<[u64]>,
}

impl Equation {
    fn new(test_value: u64, operands: Vec<u64>) -> Self {
        let operands = operands.into_boxed_slice();
        Self {
            test_value,
            operands,
        }
    }

    pub(crate) fn is_solveable(&self) -> bool {
        if self.operands.is_empty() {
            return false;
        } else if self.operands.len() == 1 {
            return self.operands[0] == self.test_value;
        }

        let operator_slots = self.operands.len() - 1;
        let combinations = 1 << operator_slots;
        debug!("testing solveability for {testing_value} with {operand_count} operands and {operator_slots} operators with {combinations} combos", testing_value = self.test_value, operand_count = self.operands.len());
        let mut operators_buf = vec![Operator::Add; operator_slots].into_boxed_slice();

        for combination in 0..combinations {
            make_combination_into(combination, &mut operators_buf);

            if test_equation(&self.operands, &operators_buf, self.test_value) {
                info!(
                    "solved! {}",
                    format_solved_equation(&self.operands, &operators_buf, self.test_value)
                );
                return true;
            }
        }

        false
    }

    pub(crate) fn test_value(&self) -> u64 {
        self.test_value
    }
}

fn format_unsolved_equation(operands: &[u64], test_value: u64) -> String {
    let mut buf = String::new();

    buf.push_str(&format!("{test_value}:"));

    operands.iter().fold(&mut buf, |acc, val| {
        acc.push_str(&format!(" {val}"));
        acc
    });

    buf
}

fn format_solved_equation(operands: &[u64], operators: &[Operator], test_value: u64) -> String {
    let mut buf = String::new();

    buf.push_str(&format!("{test_value}: {}", operands[0]));

    for (val, op) in operands[1..].iter().zip_eq(operators.iter()) {
        buf.push_str(&format!(" {op} {val}"));
    }

    buf
}

impl FromStr for Equation {
    type Err = Day07Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((test_val, operands)) = s.split_once(':') else {
            return Err(Day07Error::EquationParseError {
                input: s.to_owned(),
                error_msg: "could not split equation into test value and operand list".to_owned(),
            });
        };

        let test_val =
            test_val
                .parse::<u64>()
                .map_err(|parse_err| Day07Error::EquationParseError {
                    input: test_val.to_string(),
                    error_msg: format!("could not parse test val into number: {parse_err}"),
                })?;

        let operands = operands
            .trim()
            .split(' ')
            .map(|op| op.trim())
            .map(|op| op.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|op_parse_err| Day07Error::EquationParseError {
                input: operands.to_owned(),
                error_msg: format!("failed to parse operand list: {op_parse_err}"),
            })?;

        Ok(Self::new(test_val, operands))
    }
}

fn make_combination_into(bitflags: usize, buf: &mut [Operator]) {
    let max_idx = buf.len() - 1;

    for idx in 0..buf.len() {
        let op = if (bitflags & (1 << idx)) != 0 {
            Operator::Mul
        } else {
            Operator::Add
        };

        buf[max_idx - idx] = op;
    }
}

fn test_equation(operands: &[u64], operators: &[Operator], test_value: u64) -> bool {
    debug_assert!(
        operands.len() - 1 == operators.len(),
        "operands = {}, operators: {}, test_val = {}",
        operands.len(),
        operators.len(),
        test_value
    );

    let val = operators
        .iter()
        .zip_eq(operands.iter().skip(1))
        .fold_while(operands[0], |acc, (op, val)| {
            let acc = op.apply(acc, *val);

            if acc > test_value {
                Done(acc)
            } else {
                Continue(acc)
            }
        })
        .into_inner();

    val == test_value
}
