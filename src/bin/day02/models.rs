use std::str::FromStr;

use itertools::Itertools;

use crate::error::Day02Error;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub(crate) struct Level(u64);

impl Level {
    fn new(value: u64) -> Self {
        Self { 0: value }
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

    fn is_safe(&self, mut rules: SafetyRules) -> bool {
        for (lhs, rhs) in self.levels.iter().tuple_windows() {
            for rule in rules.rules.iter_mut() {
                if !rule(*lhs, *rhs) {
                    return false;
                }
            }
        }

        true
    }
}

impl FromStr for Report {
    type Err = Day02Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let levels = s
            .trim()
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

    pub(crate) fn count_safe_with_rules(&self, rules_maker: impl Fn() -> SafetyRules) -> usize {
        self.reports
            .iter()
            .filter(|r| r.is_safe(rules_maker()))
            .count()
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
        } else {
            detected_order = Some(value_order);
            true
        }
    };

    Box::new(rule)
}

pub(crate) fn make_diff_rule() -> Rule {
    let rule = move |lhs: Level, rhs: Level| {
        let diff = lhs.0.abs_diff(rhs.0);

        diff >= 1 && diff <= 3
    };

    Box::new(rule)
}
