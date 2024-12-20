use std::str::FromStr;

use rustc_hash::{FxHashMap, FxHashSet};
use tracing::warn;

use crate::error::Day05Error;

pub(crate) struct PageRule {
    before: usize,
    after: usize,
}

impl PageRule {
    fn new(before: usize, after: usize) -> Self {
        Self { before, after }
    }
}

impl FromStr for PageRule {
    type Err = Day05Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((before, after)) = s.split_once('|') else {
            return Err(Day05Error::PageOrderingRuleError {
                input: s.to_owned(),
                error_msg: "page rule not properly delimited".to_owned(),
            });
        };

        let before = before
            .parse::<usize>()
            .map_err(|e| Day05Error::PageOrderingRuleError {
                input: s.to_owned(),
                error_msg: format!("could not parse before of rule: {e}").to_owned(),
            })?;
        let after = after
            .parse::<usize>()
            .map_err(|e| Day05Error::PageOrderingRuleError {
                input: s.to_owned(),
                error_msg: format!("could not parse after of rule: {e}").to_owned(),
            })?;

        Ok(PageRule::new(before, after))
    }
}

pub(crate) type PageRuleList = Vec<PageRule>;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
struct PageIndex(usize);
impl PageIndex {
    fn new(idx: usize) -> Self {
        Self(idx)
    }
}

pub(crate) struct PageOrderList {
    pages: Vec<usize>,
    page_index: FxHashMap<usize, PageIndex>,
}

impl PageOrderList {
    fn new(pages: Vec<usize>) -> Self {
        let page_index = pages
            .iter()
            .enumerate()
            .map(|(idx, page)| (*page, PageIndex::new(idx)))
            .collect::<FxHashMap<usize, PageIndex>>();

        Self { pages, page_index }
    }

    fn index_of(&self, page: usize) -> Option<&PageIndex> {
        self.page_index.get(&page)
    }

    pub(crate) fn is_valid(&self, rules: &[PageRule]) -> bool {
        for rule in rules {
            let before_idx = self.index_of(rule.before);
            let after_idx = self.index_of(rule.after);

            let (Some(before_idx), Some(after_idx)) = (before_idx, after_idx) else {
                continue;
            };

            if before_idx >= after_idx {
                return false;
            }
        }

        true
    }

    pub(crate) fn middle_page(&self) -> usize {
        let middle = (self.pages.len() - 1) / 2;
        self.pages[middle]
    }

    pub(crate) fn fix(mut self, rules: &[PageRule]) -> Self {
        if self.is_valid(rules) {
            warn!("was valid page, nothing to fix");
            return self;
        }

        // after -> [before]
        let mut deps: FxHashMap<usize, FxHashSet<usize>> = FxHashMap::default();
        let original_set: FxHashSet<usize> = FxHashSet::from_iter(self.pages.iter().cloned());
        for rule in rules {
            if !original_set.contains(&rule.before) || !original_set.contains(&rule.after) {
                continue;
            }

            deps.entry(rule.after)
                .and_modify(|deps| {
                    deps.insert(rule.before);
                })
                .or_insert_with(|| {
                    let mut set: FxHashSet<usize> = FxHashSet::default();
                    set.insert(rule.before);
                    set
                });
        }

        let mut reordered: Vec<usize> = Vec::with_capacity(self.pages.len());
        while !self.pages.is_empty() {
            let starting_size = self.pages.len();
            self.pages.retain(|page| match deps.get(page) {
                None => {
                    reordered.push(*page);
                    remove_val(&mut deps, page);

                    false
                }
                _ => true,
            });

            let remaining_size = self.pages.len();

            assert!(
                remaining_size < starting_size,
                "nothing removed in iteration, we will get stuck in infinite loop"
            );
        }

        Self::new(reordered)
    }
}

fn remove_val(map: &mut FxHashMap<usize, FxHashSet<usize>>, val: &usize) {
    for vals in map.values_mut() {
        vals.remove(val);
    }

    map.retain(|_, vals| !vals.is_empty())
}

impl FromStr for PageOrderList {
    type Err = Day05Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pages = s
            .split(',')
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| Day05Error::PageListError {
                input: s.to_owned(),
                error_msg: e.to_string(),
            })?;

        Ok(PageOrderList::new(pages))
    }
}
