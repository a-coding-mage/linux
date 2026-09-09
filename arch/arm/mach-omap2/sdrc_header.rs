/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * OMAP2/3 SDRC/SMS macros and prototypes
 *
 * Translated from the C header.  C preprocessor configuration and assembler
 * address macros are preserved below as comments where they cannot be
 * represented by a file-local Rust item.
 */

use core::ffi::c_void;

extern "C" {
    pub static mut omap2_sdrc_base: *mut c_void;
    pub static mut omap2_sms_base: *mut c_void;

    pub fn writel_relaxed(val: u32, addr: *mut c_void);
    pub fn readl_relaxed(addr: *const c_void) -> u32;

    pub fn omap2_set_globals_sdrc(sdrc: *mut c_void, sms: *mut c_void);
    pub fn omap2_sdrc_init(sdrc_cs0: *mut omap_sdrc_params, sdrc_cs1: *mut omap_sdrc_params);
    pub fn omap2_sms_restore_context();
    pub fn omap2xxx_sdrc_init_params(force_lock_to_unlock_mode: u32);
    pub fn omap2xxx_sdrc_dll_is_unlocked() -> u32;
    pub fn omap2xxx_sdrc_reprogram(level: u32, force: u32) -> u32;
}

#[inline]
pub unsafe fn OMAP_SDRC_REGADDR(reg: u16) -> *mut c_void {
    (omap2_sdrc_base as *mut u8).add(reg as usize) as *mut c_void
}

#[inline]
pub unsafe fn OMAP_SMS_REGADDR(reg: u16) -> *mut c_void {
    (omap2_sms_base as *mut u8).add(reg as usize) as *mut c_void
}

#[inline]
pub unsafe fn sdrc_write_reg(val: u32, reg: u16) {
    writel_relaxed(val, OMAP_SDRC_REGADDR(reg));
}

#[inline]
pub unsafe fn sdrc_read_reg(reg: u16) -> u32 {
    readl_relaxed(OMAP_SDRC_REGADDR(reg) as *const c_void)
}

#[inline]
pub unsafe fn sms_write_reg(val: u32, reg: u16) {
    writel_relaxed(val, OMAP_SMS_REGADDR(reg));
}

#[inline]
pub unsafe fn sms_read_reg(reg: u16) -> u32 {
    readl_relaxed(OMAP_SMS_REGADDR(reg) as *const c_void)
}

#[repr(C)]
pub struct omap_sdrc_params {
    pub rate: core::ffi::c_ulong,
    pub actim_ctrla: u32,
    pub actim_ctrlb: u32,
    pub rfr_ctrl: u32,
    pub mr: u32,
}

/* When CONFIG_SOC_HAS_OMAP2_SDRC is disabled, C supplies an empty inline. */

#[repr(C)]
pub struct memory_timings {
    pub m_type: u32,       /* ddr = 1, sdr = 0 */
    pub dll_mode: u32,    /* use lock mode = 1, unlock mode = 0 */
    pub slow_dll_ctrl: u32, /* unlock mode, dll value for slow speed */
    pub fast_dll_ctrl: u32, /* unlock mode, dll value for fast speed */
    pub base_cs: u32,     /* base chip select to use for calculations */
}

pub const MIN_SDRC_DLL_LOCK_FREQ: u32 = 83000000;
pub const SDRC_MPURATE_SCALE: u32 = 8;
pub const SDRC_MPURATE_BASE_SHIFT: u32 = 9;
pub const SDRC_MPURATE_LOOPS: u32 = 96;

pub const SDRC_SYSCONFIG: u32 = 0x010;
pub const SDRC_CS_CFG: u32 = 0x040;
pub const SDRC_SHARING: u32 = 0x044;
pub const SDRC_ERR_TYPE: u32 = 0x04C;
pub const SDRC_DLLA_CTRL: u32 = 0x060;
pub const SDRC_DLLA_STATUS: u32 = 0x064;
pub const SDRC_DLLB_CTRL: u32 = 0x068;
pub const SDRC_DLLB_STATUS: u32 = 0x06C;
pub const SDRC_POWER: u32 = 0x070;
pub const SDRC_MCFG_0: u32 = 0x080;
pub const SDRC_MR_0: u32 = 0x084;
pub const SDRC_EMR2_0: u32 = 0x08c;
pub const SDRC_ACTIM_CTRL_A_0: u32 = 0x09c;
pub const SDRC_ACTIM_CTRL_B_0: u32 = 0x0a0;
pub const SDRC_RFR_CTRL_0: u32 = 0x0a4;
pub const SDRC_MANUAL_0: u32 = 0x0a8;
pub const SDRC_MCFG_1: u32 = 0x0B0;
pub const SDRC_MR_1: u32 = 0x0B4;
pub const SDRC_EMR2_1: u32 = 0x0BC;
pub const SDRC_ACTIM_CTRL_A_1: u32 = 0x0C4;
pub const SDRC_ACTIM_CTRL_B_1: u32 = 0x0C8;
pub const SDRC_RFR_CTRL_1: u32 = 0x0D4;
pub const SDRC_MANUAL_1: u32 = 0x0D8;

pub const SDRC_POWER_AUTOCOUNT_SHIFT: u32 = 8;
pub const SDRC_POWER_AUTOCOUNT_MASK: u32 = 0xffff << SDRC_POWER_AUTOCOUNT_SHIFT;
pub const SDRC_POWER_CLKCTRL_SHIFT: u32 = 4;
pub const SDRC_POWER_CLKCTRL_MASK: u32 = 0x3 << SDRC_POWER_CLKCTRL_SHIFT;
pub const SDRC_SELF_REFRESH_ON_AUTOCOUNT: u32 = 0x2 << SDRC_POWER_CLKCTRL_SHIFT;

pub const SDRC_RFR_CTRL_165MHz: u32 = 0x00044c00 | 1;
pub const SDRC_RFR_CTRL_133MHz: u32 = 0x0003de00 | 1;
pub const SDRC_RFR_CTRL_100MHz: u32 = 0x0002da01 | 1;
pub const SDRC_RFR_CTRL_110MHz: u32 = 0x0002da01 | 1; /* Need to calc */
pub const SDRC_RFR_CTRL_BYPASS: u32 = 0x00005000 | 1; /* Need to calc */

pub const SMS_SYSCONFIG: u32 = 0x010;
/* REVISIT: fill in other SMS registers here */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
