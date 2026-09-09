/* SPDX-License-Identifier: GPL-2.0-only */

// Header guard: _ARM64_CRASH_RESERVE_H

/* Current arm64 boot protocol requires 2MB alignment */
pub const CRASH_ALIGN: usize = SZ_2M;

pub const CRASH_ADDR_LOW_MAX: u64 = arm64_dma_phys_limit;
pub const CRASH_ADDR_HIGH_MAX: u64 = PHYS_MASK.wrapping_add(1);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
