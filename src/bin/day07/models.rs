use std::fmt::Display;
use std::str::FromStr;

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use tracing::debug;

use crate::error::Day07Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Operator {
    Add,
    Mul,
    Concatenation,
}

impl Operator {
    fn apply(&self, lhs: u64, rhs: u64) -> u64 {
        fn num_concat(mut lhs: u64, mut rhs: u64) -> u64 {
            let mut rhs_digits = Vec::new();

            while rhs > 0 {
                let digit = rhs % 10;
                rhs_digits.push(digit);
                rhs /= 10;
            }

            for digit in rhs_digits.into_iter().rev() {
                lhs = lhs * 10 + digit;
            }
            lhs
        }

        match self {
            Operator::Add => lhs + rhs,
            Operator::Mul => lhs * rhs,
            Operator::Concatenation => num_concat(lhs, rhs),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            Operator::Add => "+",
            Operator::Mul => "*",
            Operator::Concatenation => "||",
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

    pub(crate) fn is_solveable_with(&self, ops: &[Operator]) -> bool {
        if self.operands.is_empty() {
            return false;
        } else if self.operands.len() == 1 {
            return self.operands[0] == self.test_value;
        }

        let operator_slots = self.operands.len() - 1;
        let combinations = ops.len().pow(operator_slots as u32);

        debug!("testing solveability for {testing_value} with {operand_count} operands and {operator_slots} operators with {combinations} combos", testing_value = self.test_value, operand_count = self.operands.len());
        let mut operators_buf = vec![Operator::Add; operator_slots].into_boxed_slice();

        for combination in 0..combinations {
            make_combination_into(combination, ops, &mut operators_buf);

            if test_equation(&self.operands, &operators_buf, self.test_value) {
                return true;
            }
        }

        false
    }

    pub(crate) fn test_value(&self) -> u64 {
        self.test_value
    }
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

fn make_combination_into(mut combination_id: usize, op_choice: &[Operator], buf: &mut [Operator]) {
    let max_id_allowed = (op_choice.len() as u32).pow(buf.len() as u32) as usize;
    debug_assert!(combination_id < max_id_allowed, "combination id too large");

    for buf_idx in (0..buf.len()).rev() {
        let choose_op = combination_id % op_choice.len();
        combination_id /= op_choice.len();

        buf[buf_idx] = op_choice[choose_op];
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_combinations_made() {
        use Operator as Op;

        let op_choice = [Op::Add, Op::Mul, Op::Concatenation];
        let mut buf = vec![Op::Add; 2];

        let results = [
            [Op::Add, Op::Add],
            [Op::Add, Op::Mul],
            [Op::Add, Op::Concatenation],
            [Op::Mul, Op::Add],
            [Op::Mul, Op::Mul],
            [Op::Mul, Op::Concatenation],
            [Op::Concatenation, Op::Add],
            [Op::Concatenation, Op::Mul],
            [Op::Concatenation, Op::Concatenation],
        ];

        for id in 0..9 {
            make_combination_into(id, &op_choice, &mut buf);
            assert_eq!(results[id][..], buf[..]);
        }
    }
}
