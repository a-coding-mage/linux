/* SPDX-License-Identifier: GPL-2.0-only */

// C header guard: _LOONGARCH_CRASH_RESERVE_H

pub const CRASH_ALIGN: usize = SZ_2M;

pub const CRASH_ADDR_LOW_MAX: usize = SZ_4G;

// C macro: CRASH_ADDR_HIGH_MAX memblock_end_of_DRAM()
pub unsafe fn crash_addr_high_max() -> phys_addr_t {
    memblock_end_of_DRAM()
}

unsafe extern "C" {
    pub fn memblock_end_of_DRAM() -> phys_addr_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
