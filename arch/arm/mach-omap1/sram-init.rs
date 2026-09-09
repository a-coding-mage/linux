// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP SRAM detection and management
 *
 * Copyright (C) 2005 Nokia Corporation
 * Written by Tony Lindgren <tony@atomide.com>
 */

// C header dependencies: linux/module.h, linux/kernel.h, linux/init.h,
// linux/io.h, linux/set_memory.h, asm/fncpy.h, asm/tlb.h,
// asm/cacheflush.h, asm/mach/map.h, soc.h, and sram.h.

const OMAP1_SRAM_PA: usize = 0x20000000;
const SRAM_BOOTLOADER_SZ: usize = 0x80;
const FNCPY_ALIGN: usize = 8;

static mut omap_sram_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut omap_sram_start: usize = 0;
static mut omap_sram_skip: usize = 0;
static mut omap_sram_size: usize = 0;
static mut omap_sram_ceil: *mut core::ffi::c_void = core::ptr::null_mut();

extern "C" {
    fn cpu_is_omap15xx() -> bool;
    fn cpu_is_omap1610() -> bool;
    fn cpu_is_omap1611() -> bool;
    fn cpu_is_omap1621() -> bool;
    fn cpu_is_omap1710() -> bool;
    fn __arm_ioremap_exec(phys_addr: usize, size: usize, memtype: i32) -> *mut core::ffi::c_void;
    fn memset_io(addr: *mut core::ffi::c_void, value: i32, count: usize);
    fn set_memory_rw(addr: usize, pages: i32) -> i32;
    fn set_memory_rox(addr: usize, pages: i32) -> i32;
    fn fncpy(dst: *mut core::ffi::c_void, src: *mut core::ffi::c_void, size: usize) -> *mut core::ffi::c_void;
    fn pr_err(fmt: *const u8, ...);
    fn BUG_ON(condition: bool);
    static mut omap1_sram_reprogram_clock: *mut core::ffi::c_void;
    static omap1_sram_reprogram_clock_sz: usize;
}

/*
 * Memory allocator for SRAM: calculates the new ceiling address
 * for pushing a function using the fncpy API.
 *
 * Note that fncpy requires the returned address to be aligned
 * to an 8-byte boundary.
 */
unsafe fn omap_sram_push_address(size: usize) -> *mut core::ffi::c_void {
    let available: usize = (omap_sram_ceil as usize)
        - (omap_sram_base as usize + omap_sram_skip);
    let mut new_ceil: usize = omap_sram_ceil as usize;

    if size > available {
        pr_err(b"Not enough space in SRAM\0".as_ptr());
        return core::ptr::null_mut();
    }

    new_ceil = new_ceil.wrapping_sub(size);
    new_ceil &= !(FNCPY_ALIGN - 1);
    omap_sram_ceil = new_ceil as *mut core::ffi::c_void;

    omap_sram_ceil
}

pub unsafe fn omap_sram_push(funcp: *mut core::ffi::c_void, size: usize) -> *mut core::ffi::c_void {
    let sram = omap_sram_push_address(size);
    let mut dst: *mut core::ffi::c_void = core::ptr::null_mut();
    if sram.is_null() {
        return core::ptr::null_mut();
    }

    let base = (sram as usize) & !(4096usize - 1);
    let pages = (size + 4096 - 1) / 4096;

    set_memory_rw(base, pages as i32);
    dst = fncpy(sram, funcp, size);
    set_memory_rox(base, pages as i32);

    dst
}

/*
 * The amount of SRAM depends on the core type.
 * Note that we cannot try to test for SRAM here because writes
 * to secure SRAM will hang the system. Also the SRAM is not
 * yet mapped at this point.
 * Note that we cannot use ioremap for SRAM, as clock init needs SRAM early.
 */
unsafe fn omap_detect_and_map_sram() {
    let mut base: usize;
    let pages: i32;

    omap_sram_skip = SRAM_BOOTLOADER_SZ;
    omap_sram_start = OMAP1_SRAM_PA;

    if cpu_is_omap15xx() {
        omap_sram_size = 0x30000; // 192K
    } else if cpu_is_omap1610() || cpu_is_omap1611() || cpu_is_omap1621() || cpu_is_omap1710() {
        omap_sram_size = 0x4000; // 16K
    } else {
        pr_err(b"Could not detect SRAM size\0".as_ptr());
        omap_sram_size = 0x4000;
    }

    omap_sram_start &= !(4096usize - 1);
    omap_sram_base = __arm_ioremap_exec(omap_sram_start, omap_sram_size, 1);
    if omap_sram_base.is_null() {
        pr_err(b"SRAM: Could not map\0".as_ptr());
        return;
    }

    omap_sram_ceil = (omap_sram_base as usize + omap_sram_size) as *mut core::ffi::c_void;

    /*
     * Looks like we need to preserve some bootloader code at the
     * beginning of SRAM for jumping to flash for reboot to work...
     */
    memset_io(
        (omap_sram_base as usize + omap_sram_skip) as *mut core::ffi::c_void,
        0,
        omap_sram_size - omap_sram_skip,
    );

    base = omap_sram_base as usize;
    pages = (omap_sram_size + 4096 - 1) as i32 / 4096;
    set_memory_rox(base, pages);
}

static mut _omap_sram_reprogram_clock: Option<unsafe extern "C" fn(u32, u32)> = None;

pub unsafe fn omap_sram_reprogram_clock(dpllctl: u32, ckctl: u32) {
    BUG_ON(_omap_sram_reprogram_clock.is_none());
    if let Some(func) = _omap_sram_reprogram_clock {
        func(dpllctl, ckctl);
    }
}

pub unsafe fn omap1_sram_init() -> i32 {
    omap_detect_and_map_sram();
    _omap_sram_reprogram_clock = Some(core::mem::transmute(omap_sram_push(
        omap1_sram_reprogram_clock,
        omap1_sram_reprogram_clock_sz,
    )));
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
