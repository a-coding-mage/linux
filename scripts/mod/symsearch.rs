// SPDX-License-Identifier: GPL-2.0

//! Deterministic nearest-symbol lookup for section-reference diagnostics.

/// An immutable index ordered by section, address, and original symbol order.
pub(crate) struct SymbolSearch {
    entries: Vec<(u32, u64, usize)>,
}

impl SymbolSearch {
    pub(crate) fn new(entries: impl IntoIterator<Item = (u32, u64, usize)>) -> Self {
        let mut entries: Vec<_> = entries.into_iter().collect();
        entries.sort_unstable();
        // Duplicate addresses always name the first symbol in the ELF table.
        entries.dedup_by(|later, earlier| later.0 == earlier.0 && later.1 == earlier.1);
        Self { entries }
    }

    pub(crate) fn nearest(
        &self,
        address: u64,
        section: u32,
        allow_negative: bool,
        mut distance: u64,
    ) -> Option<usize> {
        let position = self
            .entries
            .partition_point(|&(sec, addr, _)| (sec, addr) <= (section, address));
        let mut result = None;
        if allow_negative {
            if let Some(&(sec, addr, index)) = self.entries.get(position) {
                if sec == section && addr - address <= distance {
                    result = Some(index);
                    distance = addr - address;
                }
            }
        }
        if let Some(&(sec, addr, index)) = position.checked_sub(1).and_then(|p| self.entries.get(p))
        {
            if sec == section && address - addr <= distance {
                result = Some(index);
            }
        }
        result
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
