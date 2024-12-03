use std::{str::FromStr, sync::OnceLock};

use regex::Regex;
use tracing::info;

use crate::error::Day03Error;

pub(crate) struct MulInstr {
    lhs: u64,
    rhs: u64,
}

impl MulInstr {
    fn new(lhs: u64, rhs: u64) -> Self {
        Self { lhs, rhs }
    }

    fn exec(&self) -> u64 {
        self.lhs * self.rhs
    }
}

impl FromStr for MulInstr {
    type Err = Day03Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const MUL_INSTR_PREFIX: &str = "mul(";

        let Some(s) = s.strip_prefix(MUL_INSTR_PREFIX) else {
            return Err(Day03Error::MulInstrParseError {
                input: s.to_owned(),
                error_msg: "Not correct instr prefix".to_owned(),
            });
        };

        let Some((lhs, rest)) = s.split_once(',') else {
            return Err(Day03Error::MulInstrParseError {
                input: s.to_owned(),
                error_msg: "operands not correcly split".to_owned(),
            });
        };

        let Ok(lhs) = lhs.parse::<u64>() else {
            return Err(Day03Error::MulInstrParseError {
                input: lhs.to_owned(),
                error_msg: "could not parse lhs".to_owned(),
            });
        };

        let Some(rhs) = rest.strip_suffix(')') else {
            return Err(Day03Error::MulInstrParseError {
                input: rest.to_owned(),
                error_msg: "could not strip rhs suffix".to_owned(),
            });
        };

        let Ok(rhs) = rhs.parse::<u64>() else {
            return Err(Day03Error::MulInstrParseError {
                input: rhs.to_owned(),
                error_msg: "could not parse rhs".to_owned(),
            });
        };

        Ok(MulInstr::new(lhs, rhs))
    }
}

pub(crate) struct Memory {
    instr: Vec<MulInstr>,
}

impl Memory {
    fn new(instr: Vec<MulInstr>) -> Self {
        Self { instr }
    }

    pub(crate) fn run(&self) -> u64 {
        self.instr.iter().map(|mul| mul.exec()).sum()
    }
}

impl FromStr for Memory {
    type Err = Day03Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static RE_CELL: OnceLock<Regex> = OnceLock::new();
        let re = RE_CELL.get_or_init(|| Regex::new(r"mul\(\d+,\d+\)").unwrap());

        let instrs = re
            .captures_iter(s)
            .map(|mul_str| mul_str.get(0).unwrap().as_str().parse::<MulInstr>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Memory::new(instrs))
    }
}

pub(crate) struct ConditionalMemory {
    enabled_regions: Vec<Memory>,
}

impl ConditionalMemory {
    fn new(memory_regions: Vec<Memory>) -> Self {
        Self {
            enabled_regions: memory_regions,
        }
    }

    pub(crate) fn run(&self) -> u64 {
        self.enabled_regions.iter().map(|mem| mem.run()).sum()
    }
}

impl FromStr for ConditionalMemory {
    type Err = Day03Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const ENABLE_INSTR: &str = "do()";
        const DISABLE_INSTR: &str = "don't()";

        let do_idxs = s
            .match_indices(ENABLE_INSTR)
            .map(|(start, slice)| start + slice.len())
            .collect::<Vec<_>>();

        let dont_idxs = s
            .match_indices(DISABLE_INSTR)
            .map(|(start, _)| start)
            .collect::<Vec<_>>();

        let mut enabled_ranges: Vec<(usize, usize)> = Vec::new();

        let mut enabled_region_start = 0;

        for disabled_start in dont_idxs.iter() {
            if *disabled_start < enabled_region_start {
                continue;
            }

            enabled_ranges.push((enabled_region_start, *disabled_start));

            info!(
                "enabled region: {} - {}",
                enabled_region_start, disabled_start
            );

            let Some(next_enable) = do_idxs.iter().find(|do_idx| *do_idx > disabled_start) else {
                break;
            };

            enabled_region_start = *next_enable;
        }

        if enabled_region_start > *dont_idxs.last().unwrap() {
            enabled_ranges.push((enabled_region_start, s.len()));
        }

        let memory_regions = enabled_ranges
            .into_iter()
            .map(|(start, end)| &s[start..end])
            .map(|reg| reg.parse::<Memory>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(ConditionalMemory::new(memory_regions))
    }
}
