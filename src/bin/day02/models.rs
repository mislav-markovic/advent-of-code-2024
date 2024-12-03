use std::str::FromStr;

use derive_more::derive::Display;
use itertools::Itertools;

use crate::error::Day02Error;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Display)]
pub(crate) struct Level(u64);

impl Level {
    fn new(value: u64) -> Self {
        Self(value)
    }
}

impl FromStr for Level {
    type Err = Day02Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed: u64 = s
            .trim()
            .parse::<u64>()
            .map_err(|e| Day02Error::LevelParseError {
                input: s.trim().to_string(),
                error_msg: e.to_string(),
            })?;

        Ok(Level::new(parsed))
    }
}

pub(crate) struct Report {
    levels: Vec<Level>,
}

impl Report {
    fn new(levels: Vec<Level>) -> Self {
        Self { levels }
    }

    fn is_safe(&self, mut rules: SafetyRules, skip_level_idx: Option<usize>) -> bool {
        self.levels
            .iter()
            .enumerate()
            // if `skip_level_idx` given, that means we ignore nth element completely when checking
            // the rules
            .filter(|(idx, _)| skip_level_idx.map_or(true, |skip| skip != *idx))
            .tuple_windows()
            .all(|((_, lhs), (_, rhs))| rules.rules.iter_mut().all(|rule| rule(*lhs, *rhs)))
    }
}

impl FromStr for Report {
    type Err = Day02Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let levels = s
            .split_whitespace()
            .map(|e| e.trim())
            .map(|val| val.parse::<Level>())
            .collect::<Result<Vec<_>, _>>()?;

        if levels.is_empty() {
            return Err(Day02Error::ReportParseError {
                input: s.to_string(),
                error_msg: "empty report is invalid, no levels found".to_owned(),
            });
        }

        Ok(Report::new(levels))
    }
}

pub(crate) struct UnusualData {
    reports: Vec<Report>,
}

impl UnusualData {
    fn new(reports: Vec<Report>) -> Self {
        Self { reports }
    }

    pub(crate) fn count_safe_with_rules(
        &self,
        rules_maker: impl Fn() -> SafetyRules,
        problem_dampener: bool,
    ) -> usize {
        let mut safe_count = 0;
        for report in self.reports.iter() {
            if !report.is_safe(rules_maker(), None) {
                if problem_dampener {
                    for i in 0..report.levels.len() {
                        if report.is_safe(rules_maker(), Some(i)) {
                            safe_count += 1;
                            break;
                        }
                    }
                }
            } else {
                safe_count += 1;
            }
        }

        safe_count
    }
}

impl FromStr for UnusualData {
    type Err = Day02Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let reports = s
            .trim()
            .lines()
            .map(|l| l.parse::<Report>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(UnusualData::new(reports))
    }
}

type Rule = Box<dyn FnMut(Level, Level) -> bool>;
pub(crate) struct SafetyRules {
    rules: Vec<Rule>,
}

impl SafetyRules {
    pub(crate) fn new(rules: Vec<Rule>) -> Self {
        Self { rules }
    }
}

pub(crate) fn make_sorted_rule() -> Rule {
    #[derive(PartialEq, Eq, Clone, Copy, Debug)]
    enum Order {
        Asc,
        Desc,
    }

    let mut detected_order = None;
    let rule = move |lhs, rhs| {
        let value_order = if lhs < rhs { Order::Asc } else { Order::Desc };

        if let Some(order) = detected_order {
            order == value_order
        } else if lhs != rhs {
            detected_order = Some(value_order);
            true
        } else {
            false
        }
    };

    Box::new(rule)
}

pub(crate) fn make_diff_rule() -> Rule {
    let rule = move |lhs: Level, rhs: Level| {
        let diff = lhs.0.abs_diff(rhs.0);
        (1..=3).contains(&diff)
    };

    Box::new(rule)
}
