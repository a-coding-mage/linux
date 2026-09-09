// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP SRAM detection and management
 *
 * Copyright (C) 2005 Nokia Corporation
 * Written by Tony Lindgren <tony@atomide.com>
 *
 * Copyright (C) 2009-2012 Texas Instruments
 * Added OMAP4/5 support - Santosh Shilimkar <santosh.shilimkar@ti.com>
 */

// C dependencies: linux/module.h, kernel.h, init.h, io.h, set_memory.h,
// asm/fncpy.h, tlb.h, cacheflush.h, mach/map.h, and local OMAP headers.

const OMAP2_SRAM_PUB_PA: usize = OMAP2_SRAM_PA + 0xf800;
const OMAP3_SRAM_PUB_PA: usize = OMAP3_SRAM_PA + 0x8000;
const SRAM_BOOTLOADER_SZ: usize = 0x00;
const OMAP24XX_VA_REQINFOPERM0: usize = OMAP2_L3_IO_ADDRESS(0x68005048);
const OMAP24XX_VA_READPERM0: usize = OMAP2_L3_IO_ADDRESS(0x68005050);
const OMAP24XX_VA_WRITEPERM0: usize = OMAP2_L3_IO_ADDRESS(0x68005058);
const OMAP34XX_VA_REQINFOPERM0: usize = OMAP2_L3_IO_ADDRESS(0x68012848);
const OMAP34XX_VA_READPERM0: usize = OMAP2_L3_IO_ADDRESS(0x68012850);
const OMAP34XX_VA_WRITEPERM0: usize = OMAP2_L3_IO_ADDRESS(0x68012858);
const OMAP34XX_VA_ADDR_MATCH2: usize = OMAP2_L3_IO_ADDRESS(0x68012880);
const OMAP34XX_VA_SMS_RG_ATT0: usize = OMAP2_L3_IO_ADDRESS(0x6C000048);
const GP_DEVICE: usize = 0x300;

static mut omap_sram_start: usize = 0;
static mut omap_sram_size: usize = 0;
static mut omap_sram_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut omap_sram_skip: usize = 0;
static mut omap_sram_ceil: *mut core::ffi::c_void = core::ptr::null_mut();

static mut _omap2_sram_ddr_init: Option<unsafe extern "C" fn(*mut u32, u32, u32, u32)> = None;
static mut _omap2_sram_reprogram_sdrc: Option<unsafe extern "C" fn(u32, u32, u32)> = None;
static mut _omap2_set_prcm: Option<unsafe extern "C" fn(u32, u32, i32) -> u32> = None;

unsafe fn omap_sram_push_address(size: usize) -> *mut core::ffi::c_void {
    let mut new_ceil = omap_sram_ceil as usize;
    let available = (omap_sram_ceil as usize) - (omap_sram_base as usize + omap_sram_skip);
    if size > available { pr_err!("Not enough space in SRAM\n"); return core::ptr::null_mut(); }
    new_ceil = (new_ceil - size) & !(FNCPY_ALIGN - 1);
    omap_sram_ceil = new_ceil as *mut core::ffi::c_void;
    omap_sram_ceil
}

pub unsafe fn omap_sram_push(funcp: *mut core::ffi::c_void, size: usize) -> *mut core::ffi::c_void {
    let sram = omap_sram_push_address(size);
    if sram.is_null() { return core::ptr::null_mut(); }
    let base = (sram as usize) & PAGE_MASK;
    let pages = PAGE_ALIGN(size) / PAGE_SIZE;
    set_memory_rw(base, pages);
    let dst = fncpy(sram, funcp, size);
    set_memory_rox(base, pages);
    dst
}

unsafe fn omap_sram_reset() { omap_sram_ceil = (omap_sram_base as usize + omap_sram_size) as *mut _; }

unsafe fn is_sram_locked() -> i32 {
    if omap_type() == OMAP2_DEVICE_TYPE_GP {
        if cpu_is_omap242x() {
            writel_relaxed(0xFF, OMAP24XX_VA_REQINFOPERM0); writel_relaxed(0xCFDE, OMAP24XX_VA_READPERM0); writel_relaxed(0xCFDE, OMAP24XX_VA_WRITEPERM0);
        }
        if cpu_is_omap34xx() {
            writel_relaxed(0xFFFF, OMAP34XX_VA_REQINFOPERM0); writel_relaxed(0xFFFF, OMAP34XX_VA_READPERM0); writel_relaxed(0xFFFF, OMAP34XX_VA_WRITEPERM0);
            writel_relaxed(0x0, OMAP34XX_VA_ADDR_MATCH2); writel_relaxed(0xFFFFFFFF, OMAP34XX_VA_SMS_RG_ATT0);
        }
        0
    } else { 1 }
}

unsafe fn omap_detect_sram() {
    omap_sram_skip = SRAM_BOOTLOADER_SZ;
    if is_sram_locked() != 0 {
        if cpu_is_omap34xx() {
            omap_sram_start = OMAP3_SRAM_PUB_PA;
            if omap_type() == OMAP2_DEVICE_TYPE_EMU || omap_type() == OMAP2_DEVICE_TYPE_SEC { omap_sram_size = 0x7000; omap_sram_skip += SZ_16K; } else { omap_sram_size = 0x8000; }
        } else { omap_sram_start = OMAP2_SRAM_PUB_PA; omap_sram_size = 0x800; }
    } else if cpu_is_omap34xx() { omap_sram_start = OMAP3_SRAM_PA; omap_sram_size = 0x10000; }
    else { omap_sram_start = OMAP2_SRAM_PA; if cpu_is_omap242x() { omap_sram_size = 0xa0000; } else if cpu_is_omap243x() { omap_sram_size = 0x10000; } }
}

unsafe fn omap2_map_sram() {
    let cached = if cpu_is_omap34xx() { 0 } else { 1 };
    if omap_sram_size == 0 { return; }
    omap_sram_start &= !(PAGE_SIZE - 1);
    omap_sram_base = __arm_ioremap_exec(omap_sram_start, omap_sram_size, cached);
    if omap_sram_base.is_null() { pr_err!("SRAM: Could not map\n"); return; }
    omap_sram_reset();
    memset_io((omap_sram_base as usize + omap_sram_skip) as *mut _, 0, omap_sram_size - omap_sram_skip);
    set_memory_rox(omap_sram_base as usize, PAGE_ALIGN(omap_sram_size) / PAGE_SIZE);
}

pub unsafe fn omap2_sram_ddr_init(a: *mut u32, b: u32, c: u32, d: u32) { BUG_ON!(_omap2_sram_ddr_init.is_none()); (_omap2_sram_ddr_init.unwrap())(a,b,c,d); }
pub unsafe fn omap2_sram_reprogram_sdrc(a: u32, b: u32, c: u32) { BUG_ON!(_omap2_sram_reprogram_sdrc.is_none()); (_omap2_sram_reprogram_sdrc.unwrap())(a,b,c); }
pub unsafe fn omap2_set_prcm(a: u32, b: u32, c: i32) -> u32 { BUG_ON!(_omap2_set_prcm.is_none()); (_omap2_set_prcm.unwrap())(a,b,c) }

// CONFIG_SOC_OMAP2420 / CONFIG_SOC_OMAP2430 and CONFIG_ARCH_OMAP3 are
// build-time conditions; declarations are retained with their intent here.
unsafe fn omap242x_sram_init() -> i32 { 0 }
unsafe fn omap243x_sram_init() -> i32 { 0 }
unsafe fn omap34xx_sram_init() -> i32 { omap3_sram_restore_context(); 0 }

pub unsafe fn omap3_sram_restore_context() { omap_sram_reset(); omap_push_sram_idle(); }

pub unsafe fn omap_sram_init() -> i32 {
    omap_detect_sram(); omap2_map_sram();
    if cpu_is_omap242x() { omap242x_sram_init(); } else if cpu_is_omap2430() { omap243x_sram_init(); } else if cpu_is_omap34xx() { omap34xx_sram_init(); }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
