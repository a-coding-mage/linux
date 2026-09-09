/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by asm/cpu-features.h and asm/mipsregs.h are expected
// to be available to the translation unit using this header.

macro_rules! _UNIQUE_ENTRYHI {
    ($base:expr, $idx:expr) => {
        (($base) + (($idx) << (PAGE_SHIFT + 1))
            | if cpu_has_tlbinv { MIPS_ENTRYHI_EHINV } else { 0 })
    };
}

macro_rules! UNIQUE_ENTRYHI {
    ($idx:expr) => {
        _UNIQUE_ENTRYHI!(CKSEG0, $idx)
    };
}

macro_rules! UNIQUE_GUEST_ENTRYHI {
    ($idx:expr) => {
        _UNIQUE_ENTRYHI!(CKSEG1, $idx)
    };
}

#[inline]
pub unsafe fn num_wired_entries() -> core::ffi::c_uint {
    let mut wired: core::ffi::c_uint = read_c0_wired();

    if cpu_has_mips_r6 {
        wired &= MIPSR6_WIRED_WIRED;
    }

    wired
}

// #include <asm-generic/tlb.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
