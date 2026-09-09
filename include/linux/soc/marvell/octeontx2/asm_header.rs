/* SPDX-License-Identifier: GPL-2.0-only
 * Copyright (C) 2020 Marvell.
 */

// Translated from the Linux kernel OTX2 assembly header.

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub unsafe fn otx2_lmt_flush(ioaddr: *const u8) -> u64 {
    let result: u64;
    core::arch::asm!(
        ".cpu generic+lse",
        "ldeor xzr, {result}, [{ioaddr}]",
        result = lateout(reg) result,
        ioaddr = in(reg) ioaddr,
        options(nostack)
    );
    result
}

/*
 * STEORL store to memory with release semantics.
 * This will avoid using DMB barrier after each LMTST
 * operation.
 */
#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub unsafe fn cn10k_lmt_flush(val: u64, addr: *mut u64) {
    let mut val = val;
    core::arch::asm!(
        ".cpu generic+lse",
        "steorl {val}, [{addr}]",
        val = inout(reg) val,
        addr = in(reg) addr,
        options(nostack)
    );
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub unsafe fn otx2_atomic64_fetch_add(incr: u64, ptr: *mut u64) -> u64 {
    let result: u64;
    core::arch::asm!(
        ".cpu generic+lse",
        "ldadda {incr}, {result}, [{ptr}]",
        result = lateout(reg) result,
        incr = in(reg) incr,
        ptr = in(reg) ptr,
        options(nostack)
    );
    result
}

// Non-ARM64 builds use the fallback macro definitions from the C header.
#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub unsafe fn otx2_lmt_flush(_ioaddr: *const u8) -> u64 {
    0
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub unsafe fn cn10k_lmt_flush(val: u64, addr: *mut u64) {
    *addr = val;
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub unsafe fn otx2_atomic64_fetch_add(incr: u64, _ptr: *mut u64) -> u64 {
    incr
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
