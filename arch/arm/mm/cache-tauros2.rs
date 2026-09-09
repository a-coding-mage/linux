// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mm/cache-tauros2.c - Tauros2 L2 cache controller support
 *
 * Copyright (C) 2008 Marvell Semiconductor
 *
 * References:
 * - PJ1 CPU Core Datasheet,
 *   Document ID MV-S104837-01, Rev 0.7, January 24 2008.
 * - PJ4 CPU Core Datasheet,
 *   Document ID MV-S105190-00, Rev 0.7, March 14 2008.
 */

use core::ffi::{c_char, c_int};

const CCR_L2C_PREFETCH_DISABLE: u32 = 1u32 << 24;
const CCR_L2C_ECC_ENABLE: u32 = 1u32 << 23;
const CCR_L2C_WAY7_4_DISABLE: u32 = 1u32 << 21;
const CCR_L2C_BURST8_ENABLE: u32 = 1u32 << 20;

// When Tauros2 is used on a CPU supporting v7 hierarchical cache operations,
// the v7 cache handling code takes care of everything, including DMA coherency.
// Outer cache operations are therefore registered only for pre-v7 CPUs.

extern "C" {
    static mut processor_id: u32;
    static mut outer_cache: OuterCache;
    fn dsb();
    fn pr_info(fmt: *const c_char, ...);
    fn pr_crit(fmt: *const c_char, ...);
    fn of_find_matching_node(from: *mut DeviceNode, matches: *const OfDeviceId) -> *mut DeviceNode;
    fn of_property_read_u32(node: *mut DeviceNode, propname: *const c_char, out_value: *mut u32) -> c_int;
}

#[repr(C)]
pub struct OuterCache {
    pub inv_range: Option<unsafe extern "C" fn(usize, usize)>,
    pub clean_range: Option<unsafe extern "C" fn(usize, usize)>,
    pub flush_range: Option<unsafe extern "C" fn(usize, usize)>,
    pub disable: Option<unsafe extern "C" fn()>,
    pub resume: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct OfDeviceId {
    pub compatible: *const c_char,
}

const CACHE_TAUROS2_PREFETCH_ON: u32 = 1 << 0;
const CACHE_TAUROS2_LINEFILL_BURST8: u32 = 1 << 1;

#[cfg(CONFIG_CPU_32v5)]
unsafe extern "C" fn tauros2_clean_pa(addr: usize) {
    core::arch::asm!("mcr p15, 1, {0}, c7, c11, 3", in(reg) addr);
}

#[cfg(CONFIG_CPU_32v5)]
unsafe extern "C" fn tauros2_clean_inv_pa(addr: usize) {
    core::arch::asm!("mcr p15, 1, {0}, c7, c15, 3", in(reg) addr);
}

#[cfg(CONFIG_CPU_32v5)]
unsafe extern "C" fn tauros2_inv_pa(addr: usize) {
    core::arch::asm!("mcr p15, 1, {0}, c7, c7, 3", in(reg) addr);
}

#[cfg(CONFIG_CPU_32v5)]
const CACHE_LINE_SIZE: usize = 32;

#[cfg(CONFIG_CPU_32v5)]
unsafe extern "C" fn tauros2_inv_range(mut start: usize, mut end: usize) {
    // Clean and invalidate partial first cache line.
    if start & (CACHE_LINE_SIZE - 1) != 0 {
        tauros2_clean_inv_pa(start & !(CACHE_LINE_SIZE - 1));
        start = (start | (CACHE_LINE_SIZE - 1)).wrapping_add(1);
    }
    if end & (CACHE_LINE_SIZE - 1) != 0 {
        // Clean and invalidate partial last cache line.
        tauros2_clean_inv_pa(end & !(CACHE_LINE_SIZE - 1));
        end &= !(CACHE_LINE_SIZE - 1);
    }
    while start < end {
        // Invalidate all full cache lines between start and end.
        tauros2_inv_pa(start);
        start = start.wrapping_add(CACHE_LINE_SIZE);
    }
    dsb();
}

#[cfg(CONFIG_CPU_32v5)]
unsafe extern "C" fn tauros2_clean_range(mut start: usize, end: usize) {
    start &= !(CACHE_LINE_SIZE - 1);
    while start < end {
        tauros2_clean_pa(start);
        start = start.wrapping_add(CACHE_LINE_SIZE);
    }
    dsb();
}

#[cfg(CONFIG_CPU_32v5)]
unsafe extern "C" fn tauros2_flush_range(mut start: usize, end: usize) {
    start &= !(CACHE_LINE_SIZE - 1);
    while start < end {
        tauros2_clean_inv_pa(start);
        start = start.wrapping_add(CACHE_LINE_SIZE);
    }
    dsb();
}

#[cfg(CONFIG_CPU_32v5)]
unsafe extern "C" fn tauros2_disable() {
    let mut value: usize = 0;
    core::arch::asm!(
        "mcr p15, 1, {0}, c7, c11, 0",
        "mrc p15, 0, {0}, c1, c0, 0",
        "bic {0}, {0}, #(1 << 26)",
        "mcr p15, 0, {0}, c1, c0, 0",
        inout(reg) value,
    );
}

#[cfg(CONFIG_CPU_32v5)]
unsafe extern "C" fn tauros2_resume() {
    let mut value: usize = 0;
    core::arch::asm!(
        "mcr p15, 1, {0}, c7, c7, 0",
        "mrc p15, 0, {0}, c1, c0, 0",
        "orr {0}, {0}, #(1 << 26)",
        "mcr p15, 0, {0}, c1, c0, 0",
        inout(reg) value,
    );
}

unsafe fn read_extra_features() -> u32 {
    let mut u: u32;
    core::arch::asm!("mrc p15, 1, {0}, c15, c1, 0", out(reg) u);
    u
}

unsafe fn write_extra_features(u: u32) {
    core::arch::asm!("mcr p15, 1, {0}, c15, c1, 0", in(reg) u);
}

unsafe fn cpuid_scheme() -> bool {
    (processor_id & 0x000f0000) == 0x000f0000
}

unsafe fn read_mmfr3() -> u32 {
    let mut mmfr3: u32;
    core::arch::asm!("mrc p15, 0, {0}, c0, c1, 7", out(reg) mmfr3);
    mmfr3
}

unsafe fn read_actlr() -> u32 {
    let mut actlr: u32;
    core::arch::asm!("mrc p15, 0, {0}, c1, c0, 1", out(reg) actlr);
    actlr
}

unsafe fn write_actlr(actlr: u32) {
    core::arch::asm!("mcr p15, 0, {0}, c1, c0, 1", in(reg) actlr);
}

unsafe fn enable_extra_feature(features: u32) {
    let mut u = read_extra_features();
    if features & CACHE_TAUROS2_PREFETCH_ON != 0 { u &= !CCR_L2C_PREFETCH_DISABLE; }
    else { u |= CCR_L2C_PREFETCH_DISABLE; }
    pr_info(b"Tauros2: %s L2 prefetch.\0".as_ptr() as *const c_char,
        if features & CACHE_TAUROS2_PREFETCH_ON != 0 { b"Enabling\0".as_ptr() } else { b"Disabling\0".as_ptr() });
    if features & CACHE_TAUROS2_LINEFILL_BURST8 != 0 { u |= CCR_L2C_BURST8_ENABLE; }
    else { u &= !CCR_L2C_BURST8_ENABLE; }
    pr_info(b"Tauros2: %s burst8 line fill.\n\0".as_ptr() as *const c_char,
        if features & CACHE_TAUROS2_LINEFILL_BURST8 != 0 { b"Enabling\0".as_ptr() } else { b"Disabling\0".as_ptr() });
    write_extra_features(u);
}

unsafe fn tauros2_internal_init(features: u32) {
    enable_extra_feature(features);
    let mut mode: *const c_char = core::ptr::null();

    #[cfg(CONFIG_CPU_32v5)]
    if (processor_id & 0xff0f0000) == 0x56050000 {
        let feat = read_extra_features();
        if feat & 0x00400000 == 0 {
            pr_info(b"Tauros2: Enabling L2 cache.\n\0".as_ptr() as *const c_char);
            write_extra_features(feat | 0x00400000);
        }
        mode = b"ARMv5\0".as_ptr() as *const c_char;
        outer_cache.inv_range = Some(tauros2_inv_range);
        outer_cache.clean_range = Some(tauros2_clean_range);
        outer_cache.flush_range = Some(tauros2_flush_range);
        outer_cache.disable = Some(tauros2_disable);
        outer_cache.resume = Some(tauros2_resume);
    }

    #[cfg(CONFIG_CPU_32v7)]
    if cpuid_scheme() && (read_mmfr3() & 0xf) == 1 {
        let actlr = read_actlr();
        if actlr & 0x00000002 == 0 {
            pr_info(b"Tauros2: Enabling L2 cache.\n\0".as_ptr() as *const c_char);
            write_actlr(actlr | 0x00000002);
        }
        mode = b"ARMv7\0".as_ptr() as *const c_char;
    }
    if mode.is_null() {
        pr_crit(b"Tauros2: Unable to detect CPU mode.\n\0".as_ptr() as *const c_char);
        return;
    }
    pr_info(b"Tauros2: L2 cache support initialised in %s mode.\n\0".as_ptr() as *const c_char, mode);
}

#[cfg(CONFIG_OF)]
static TAUROS2_IDS: [OfDeviceId; 2] = [
    OfDeviceId { compatible: b"marvell,tauros2-cache\0".as_ptr() as *const c_char },
    OfDeviceId { compatible: core::ptr::null() },
];

pub unsafe fn tauros2_init(mut features: u32) {
    #[cfg(CONFIG_OF)]
    {
        let node = of_find_matching_node(core::ptr::null_mut(), TAUROS2_IDS.as_ptr());
        if node.is_null() {
            pr_info(b"Not found marvell,tauros2-cache, disable it\n\0".as_ptr() as *const c_char);
        } else {
            let mut f = 0u32;
            if of_property_read_u32(node, b"marvell,tauros2-cache-features\0".as_ptr() as *const c_char, &mut f) != 0 {
                pr_info(b"Not found marvell,tauros-cache-features property, disable extra features\n\0".as_ptr() as *const c_char);
                features = 0;
            } else { features = f; }
        }
    }
    tauros2_internal_init(features);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
