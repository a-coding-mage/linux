/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 * Copyright (C) 2001 PPC64 Team, IBM Corp
 */

/* Dependency: asm/asm-const.h */

/*
 * We always define HW_PAGE_SHIFT to 12 as use of 64K pages remains Linux
 * specific, every notion of page number shared with the firmware, TCEs,
 * iommu, etc... still uses a page size of 4K.
 */
pub const HW_PAGE_SHIFT: usize = 12;
pub const HW_PAGE_SIZE: usize = 1usize << HW_PAGE_SHIFT;
pub const HW_PAGE_MASK: usize = !(HW_PAGE_SIZE - 1);

/*
 * PAGE_FACTOR is the number of bits factor between PAGE_SHIFT and
 * HW_PAGE_SHIFT, that is 4K pages.
 */
pub const PAGE_FACTOR: usize = PAGE_SHIFT - HW_PAGE_SHIFT;

/* Segment size; normal 256M segments */
pub const SID_SHIFT: usize = 28;
pub const SID_MASK: u64 = 0xfffffffff;
pub const ESID_MASK: u64 = 0xfffffffff0000000;

#[inline]
pub const fn GET_ESID(x: u64) -> u64 {
    (x >> SID_SHIFT) & SID_MASK
}

/* 1T segments */
pub const SID_SHIFT_1T: usize = 40;
pub const SID_MASK_1T: u64 = 0xffffff;
pub const ESID_MASK_1T: u64 = 0xffffff0000000000;

#[inline]
pub const fn GET_ESID_1T(x: u64) -> u64 {
    (x >> SID_SHIFT_1T) & SID_MASK_1T
}

/* Dependency: asm/cache.h */
pub type pte_basic_t = ::core::ffi::c_ulong;

#[inline]
pub unsafe fn clear_page(mut addr: *mut ::core::ffi::c_void) {
    let iterations: ::core::ffi::c_ulong = ppc64_caches.l1d.blocks_per_page / 8;
    let onex: ::core::ffi::c_ulong = ppc64_caches.l1d.block_size;
    let twox = onex << 1;
    let fourx = onex << 2;
    let eightx = onex << 3;

    /*
     * Some verisions of gcc use multiply instructions to
     * calculate the offsets so lets give it a hand to
     * do better.
     */
    core::arch::asm!(
        "mtctr {iterations} # clear_page",
        ".balign 16",
        "1: dcbz 0, {addr}",
        "dcbz {onex}, {addr}",
        "dcbz {twox}, {addr}",
        "dcbz {twox_onex}, {addr}",
        "dcbz {fourx}, {addr}",
        "dcbz {fourx_onex}, {addr}",
        "dcbz {twox_fourx}, {addr}",
        "dcbz {eightx_onex}, {addr}",
        "add {addr}, {addr}, {eightx}",
        "bdnz+ 1b",
        addr = inout(reg) addr,
        iterations = in(reg) iterations,
        onex = in(reg) onex,
        twox = in(reg) twox,
        twox_onex = in(reg) twox + onex,
        fourx = in(reg) fourx,
        fourx_onex = in(reg) fourx + onex,
        twox_fourx = in(reg) twox + fourx,
        eightx_onex = in(reg) eightx - onex,
        eightx = in(reg) eightx,
        lateout("ctr") _,
        options(preserves_flags)
    );
}

unsafe extern "C" {
    pub fn copy_page(to: *mut ::core::ffi::c_void, from: *mut ::core::ffi::c_void);

    /* Log 2 of page table size */
    pub static mut ppc64_pft_size: u64;
}

macro_rules! VMA_DATA_DEFAULT_FLAGS {
    () => { if is_32bit_task() { VMA_DATA_DEFAULT_FLAGS32 } else { VMA_DATA_DEFAULT_FLAGS64 } };
}

/*
 * This is the default if a program doesn't have a PT_GNU_STACK
 * program header entry. The PPC64 ELF ABI has a non executable stack
 * stack by default, so in the absence of a PT_GNU_STACK program header
 * we turn execute permission off.
 */
macro_rules! VMA_STACK_DEFAULT_FLAGS32 { () => { VMA_DATA_FLAGS_EXEC }; }
macro_rules! VMA_STACK_DEFAULT_FLAGS64 { () => { VMA_DATA_FLAGS_NON_EXEC }; }

macro_rules! VMA_STACK_DEFAULT_FLAGS {
    () => { if is_32bit_task() { VMA_STACK_DEFAULT_FLAGS32!() } else { VMA_STACK_DEFAULT_FLAGS64!() } };
}

/* Dependency: asm-generic/getorder.h */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
