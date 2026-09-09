/* SPDX-License-Identifier: GPL-2.0-only */

// The following names are provided by other translation units.
// `PMD_SIZE`, `dma32_phys_limit`, and `phys_addr_t` are intentionally not
// defined here, matching the dependencies of the original header.

pub const CRASH_ALIGN: usize = PMD_SIZE;

pub const CRASH_ADDR_LOW_MAX: phys_addr_t = dma32_phys_limit;

#[inline]
pub unsafe fn CRASH_ADDR_HIGH_MAX() -> phys_addr_t {
    memblock_end_of_DRAM()
}

unsafe extern "C" {
    pub fn memblock_end_of_DRAM() -> phys_addr_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
