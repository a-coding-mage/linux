/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020 ARM Ltd.
 */

// C dependencies: asm/compiler.h, asm/cputype.h, asm/mte-def.h, linux/types.h.
// The original header guard and __ASSEMBLER__ exclusion are represented by
// this Rust source file containing declarations only.

#[cfg(feature = "CONFIG_KASAN_HW_TAGS")]
extern "C" {
    // DECLARE_STATIC_KEY_FALSE(mte_async_or_asymm_mode)
    static mte_async_or_asymm_mode: bool;
}

#[inline]
pub fn system_uses_mte_async_or_asymm_mode() -> bool {
    #[cfg(feature = "CONFIG_KASAN_HW_TAGS")]
    unsafe {
        // Equivalent to static_branch_unlikely(&mte_async_or_asymm_mode).
        core::ptr::read_volatile(&mte_async_or_asymm_mode)
    }
    #[cfg(not(feature = "CONFIG_KASAN_HW_TAGS"))]
    {
        false
    }
}

#[cfg(feature = "CONFIG_ARM64_MTE")]
#[inline]
pub unsafe fn mte_disable_tco() {
    // ALTERNATIVE("nop", SET_PSTATE_TCO(0), ARM64_MTE, CONFIG_KASAN_HW_TAGS)
    core::arch::asm!("msr tco, #0", options(nostack, preserves_flags));
}

#[cfg(not(feature = "CONFIG_ARM64_MTE"))]
#[inline]
pub unsafe fn mte_disable_tco() {}

#[cfg(feature = "CONFIG_ARM64_MTE")]
#[inline]
pub unsafe fn mte_enable_tco() {
    // ALTERNATIVE("nop", SET_PSTATE_TCO(1), ARM64_MTE, CONFIG_KASAN_HW_TAGS)
    core::arch::asm!("msr tco, #1", options(nostack, preserves_flags));
}

#[cfg(not(feature = "CONFIG_ARM64_MTE"))]
#[inline]
pub unsafe fn mte_enable_tco() {}

#[cfg(feature = "CONFIG_ARM64_MTE")]
#[inline]
pub unsafe fn __mte_disable_tco_async() {
    if system_uses_mte_async_or_asymm_mode() {
        mte_disable_tco();
    }
}

#[cfg(not(feature = "CONFIG_ARM64_MTE"))]
#[inline]
pub unsafe fn __mte_disable_tco_async() {}

#[cfg(feature = "CONFIG_ARM64_MTE")]
#[inline]
pub unsafe fn __mte_enable_tco_async() {
    if system_uses_mte_async_or_asymm_mode() {
        mte_enable_tco();
    }
}

#[cfg(not(feature = "CONFIG_ARM64_MTE"))]
#[inline]
pub unsafe fn __mte_enable_tco_async() {}

#[cfg(feature = "CONFIG_ARM64_MTE")]
#[inline]
pub fn mte_get_ptr_tag(ptr: *mut core::ffi::c_void) -> u8 {
    // Note: The format of KASAN tags is 0xF<x>.
    const MTE_TAG_SHIFT: u32 = 56; // supplied by asm/mte-def.h
    0xF0 | (((ptr as u64) >> MTE_TAG_SHIFT) as u8)
}

#[cfg(not(feature = "CONFIG_ARM64_MTE"))]
#[inline]
pub fn mte_get_ptr_tag(_ptr: *mut core::ffi::c_void) -> u8 { 0xFF }

#[cfg(feature = "CONFIG_ARM64_MTE")]
#[inline]
pub unsafe fn mte_get_mem_tag(addr: *mut core::ffi::c_void) -> u8 {
    let mut addr = addr;
    core::arch::asm!("ldg {0}, [{0}]", inout(reg) addr, options(nostack));
    mte_get_ptr_tag(addr)
}

#[cfg(not(feature = "CONFIG_ARM64_MTE"))]
#[inline]
pub unsafe fn mte_get_mem_tag(_addr: *mut core::ffi::c_void) -> u8 { 0xFF }

#[cfg(feature = "CONFIG_ARM64_MTE")]
#[inline]
pub unsafe fn mte_get_random_tag() -> u8 {
    let mut addr: *mut core::ffi::c_void;
    core::arch::asm!("irg {0}, {0}", out(reg) addr, options(nostack));
    mte_get_ptr_tag(addr)
}

#[cfg(not(feature = "CONFIG_ARM64_MTE"))]
#[inline]
pub unsafe fn mte_get_random_tag() -> u8 { 0xFF }

#[cfg(feature = "CONFIG_ARM64_MTE")]
#[inline]
pub unsafe fn __stg_post(mut p: u64) -> u64 {
    core::arch::asm!("stg {0}, [{0}], #16", inout(reg) p, options(nostack));
    p
}

#[cfg(feature = "CONFIG_ARM64_MTE")]
#[inline]
pub unsafe fn __stzg_post(mut p: u64) -> u64 {
    core::arch::asm!("stzg {0}, [{0}], #16", inout(reg) p, options(nostack));
    p
}

#[cfg(feature = "CONFIG_ARM64_MTE")]
#[inline]
pub unsafe fn __dc_gva(p: u64) {
    core::arch::asm!("dc gva, {0}", in(reg) p, options(nostack));
}

#[cfg(feature = "CONFIG_ARM64_MTE")]
#[inline]
pub unsafe fn __dc_gzva(p: u64) {
    core::arch::asm!("dc gzva, {0}", in(reg) p, options(nostack));
}

#[cfg(feature = "CONFIG_ARM64_MTE")]
#[inline]
pub unsafe fn mte_set_mem_tag_range(
    addr: *mut core::ffi::c_void,
    size: usize,
    tag: u8,
    init: bool,
) {
    // Assign allocation tags for a region of memory based on the pointer tag.
    // The address is required to be non-NULL and MTE_GRANULE_SIZE aligned;
    // size must also be MTE_GRANULE_SIZE aligned.
    const MTE_GRANULE_SIZE: u64 = 16;
    let dczid = read_cpuid_dczid_el0();
    let dczid_bs = 4u64 << (dczid & 0xf);
    let dczid_dzp = (dczid >> 4) & 1;
    let mut curr = tag_set(addr, tag);
    let mask = dczid_bs - 1;
    let end1 = curr | mask;
    let end3 = curr.wrapping_add(size as u64);
    let end2 = end3 & !mask;
    if dczid_dzp == 0 && size as u64 >= 2 * dczid_bs {
        loop {
            curr = if init { __stzg_post(curr) } else { __stg_post(curr) };
            if curr >= end1 { break; }
        }
        while curr < end2 {
            if init { __dc_gzva(curr); } else { __dc_gva(curr); }
            curr = curr.wrapping_add(dczid_bs);
        }
    }
    while curr < end3 {
        curr = if init { __stzg_post(curr) } else { __stg_post(curr) };
    }
    let _ = MTE_GRANULE_SIZE;
}

#[cfg(not(feature = "CONFIG_ARM64_MTE"))]
#[inline]
pub unsafe fn mte_set_mem_tag_range(_addr: *mut core::ffi::c_void, _size: usize, _tag: u8, _init: bool) {}

#[cfg(feature = "CONFIG_ARM64_MTE")]
extern "C" {
    pub fn mte_enable_kernel_sync();
    pub fn mte_enable_kernel_async();
    pub fn mte_enable_kernel_asymm();
    pub fn mte_enable_kernel_store_only() -> core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_ARM64_MTE"))]
#[inline] pub fn mte_enable_kernel_sync() {}
#[cfg(not(feature = "CONFIG_ARM64_MTE"))]
#[inline] pub fn mte_enable_kernel_async() {}
#[cfg(not(feature = "CONFIG_ARM64_MTE"))]
#[inline] pub fn mte_enable_kernel_asymm() {}
#[cfg(not(feature = "CONFIG_ARM64_MTE"))]
#[inline] pub fn mte_enable_kernel_store_only() -> core::ffi::c_int { -22 }

// External dependencies supplied by the translated architecture support.
extern "C" {
    fn read_cpuid_dczid_el0() -> u64;
    fn tag_set(addr: *mut core::ffi::c_void, tag: u8) -> u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
