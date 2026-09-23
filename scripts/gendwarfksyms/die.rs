// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2024 Google LLC
//! Owned DIE fragments with the original cache lookup and iteration ordering.

use crate::gendwarfksyms_header::{hash_32, Diagnostics};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub(crate) enum DieState {
    Incomplete,
    Fqn,
    Unexpanded,
    Complete,
    Symbol,
}

impl DieState {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Incomplete => "DIE_INCOMPLETE",
            Self::Fqn => "DIE_FQN",
            Self::Unexpanded => "DIE_UNEXPANDED",
            Self::Complete => "DIE_COMPLETE",
            Self::Symbol => "DIE_SYMBOL",
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Fragment {
    String(Vec<u8>),
    Linebreak(i32),
    Die(usize),
}

#[derive(Debug)]
pub(crate) struct Die {
    pub(crate) state: DieState,
    pub(crate) mapped: bool,
    pub(crate) fqn: Option<Vec<u8>>,
    pub(crate) tag: i32,
    pub(crate) addr: usize,
    pub(crate) fragments: Vec<Fragment>,
}

#[derive(Default)]
pub(crate) struct DieMap {
    pub(crate) entries: Vec<Die>,
    buckets: BTreeMap<u16, Vec<usize>>,
    hits: u32,
    misses: u32,
}

impl DieMap {
    fn bucket(addr: usize, state: DieState) -> u16 {
        hash_32(hash_32(addr as u32) ^ state as u32) as u16
    }

    pub(crate) fn find(&self, addr: usize, state: DieState) -> Option<usize> {
        self.buckets
            .get(&Self::bucket(addr, state))?
            .iter()
            .rev()
            .copied()
            .find(|&index| self.entries[index].addr == addr && self.entries[index].state == state)
    }

    pub(crate) fn get_or_insert(&mut self, addr: usize, want: DieState) -> usize {
        if let Some(index) = self.find(addr, want) {
            self.hits = self.hits.wrapping_add(1);
            return index;
        }
        self.misses = self.misses.wrapping_add(1);
        let index = self.entries.len();
        self.entries.push(Die {
            state: DieState::Incomplete,
            mapped: false,
            fqn: None,
            tag: -1,
            addr,
            fragments: Vec::new(),
        });
        // The lookup state selects the bucket, but an expansion is incomplete
        // until the caller finishes it. State changes do not rehash the entry.
        self.buckets
            .entry(Self::bucket(addr, want))
            .or_default()
            .push(index);
        index
    }

    pub(crate) fn ordered_indices(&self) -> Vec<usize> {
        self.buckets
            .values()
            .flat_map(|bucket| bucket.iter().rev().copied())
            .collect()
    }

    pub(crate) fn clear(&mut self, diagnostics: &mut Diagnostics) {
        let mut counts = [0u32; 5];
        for entry in &self.entries {
            counts[entry.state as usize] = counts[entry.state as usize].wrapping_add(1);
        }
        self.entries.clear();
        self.buckets.clear();
        // C keeps these counters across compilation units.
        let total = self.hits.wrapping_add(self.misses);
        if total != 0 {
            let rate = (100.0f32 * self.hits as f32) / total as f32;
            diagnostics.debug(
                "die_map_free",
                &[format!(
                    "hits {}, misses {} (hit rate {:.02}%)",
                    self.hits, self.misses, rate
                )
                .as_bytes()],
            );
        }
        for state in [
            DieState::Incomplete,
            DieState::Fqn,
            DieState::Unexpanded,
            DieState::Complete,
            DieState::Symbol,
        ] {
            diagnostics.debug(
                "die_map_free",
                &[format!("{}: {} entries", state.name(), counts[state as usize]).as_bytes()],
            );
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
