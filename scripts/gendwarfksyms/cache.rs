// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2024 Google LLC
//! Last-write-wins integer cache for visited DWARF addresses.

use std::collections::HashMap;

#[derive(Default)]
pub(crate) struct Cache {
    values: HashMap<usize, i32>,
}

impl Cache {
    pub(crate) fn set(&mut self, key: usize, value: i32) {
        self.values.insert(key, value);
    }

    pub(crate) fn get(&self, key: usize) -> i32 {
        self.values.get(&key).copied().unwrap_or(-1)
    }

    pub(crate) fn clear(&mut self) {
        self.values.clear();
    }

    pub(crate) fn mark_expanded(&mut self, address: usize) {
        self.set(address, 1);
    }

    pub(crate) fn was_expanded(&self, address: usize) -> bool {
        self.get(address) == 1
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
