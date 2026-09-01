// SPDX-License-Identifier: GPL-2.0

pub const L1_CACHE_SHIFT: u32 = 5;
pub const L1_CACHE_BYTES: u32 = 1u32 << L1_CACHE_SHIFT;

pub const SMP_CACHE_BYTES: u32 = L1_CACHE_BYTES;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
