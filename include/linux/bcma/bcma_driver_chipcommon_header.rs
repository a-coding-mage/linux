/* SPDX-License-Identifier: GPL-2.0 */
//! Rust translation of `bcma_driver_chipcommon.h`.
//!
//! Kernel-provided types (`bcma_device`, `platform_device`, `brcmnand_platform_data`,
//! `spinlock_t`, and `gpio_chip`) are intentionally referenced but not defined here.

#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

pub const BCMA_CC_ID: u32 = 0x0000;
pub const BCMA_CC_ID_ID: u32 = 0x0000FFFF;
pub const BCMA_CC_ID_ID_SHIFT: u32 = 0;
pub const BCMA_CC_ID_REV: u32 = 0x000F0000;
pub const BCMA_CC_ID_REV_SHIFT: u32 = 16;
pub const BCMA_CC_ID_PKG: u32 = 0x00F00000;
pub const BCMA_CC_ID_PKG_SHIFT: u32 = 20;
pub const BCMA_CC_ID_NRCORES: u32 = 0x0F000000;
pub const BCMA_CC_ID_NRCORES_SHIFT: u32 = 24;
pub const BCMA_CC_ID_TYPE: u32 = 0xF0000000;
pub const BCMA_CC_ID_TYPE_SHIFT: u32 = 28;
pub const BCMA_CC_CAP: u32 = 0x0004;
pub const BCMA_CC_CAP_NRUART: u32 = 0x3;
pub const BCMA_CC_CAP_MIPSEB: u32 = 0x4;
pub const BCMA_CC_CAP_UARTCLK: u32 = 0x18;
pub const BCMA_CC_CAP_UARTCLK_INT: u32 = 0x8;
pub const BCMA_CC_CAP_UARTGPIO: u32 = 0x20;
pub const BCMA_CC_CAP_EXTBUS: u32 = 0xC0;
pub const BCMA_CC_CAP_FLASHT: u32 = 0x700;
pub const BCMA_CC_FLASHT_NONE: u32 = 0;
pub const BCMA_CC_FLASHT_STSER: u32 = 0x100;
pub const BCMA_CC_FLASHT_ATSER: u32 = 0x200;
pub const BCMA_CC_FLASHT_NAND: u32 = 0x300;
pub const BCMA_CC_FLASHT_PARA: u32 = 0x700;
pub const BCMA_CC_CAP_PLLT: u32 = 0x38000;
pub const BCMA_PLLTYPE_NONE: u32 = 0;
pub const BCMA_PLLTYPE_1: u32 = 0x10000;
pub const BCMA_PLLTYPE_2: u32 = 0x20000;
pub const BCMA_PLLTYPE_3: u32 = 0x30000;
pub const BCMA_PLLTYPE_4: u32 = 0x8000;
pub const BCMA_PLLTYPE_5: u32 = 0x18000;
pub const BCMA_PLLTYPE_6: u32 = 0x28000;
pub const BCMA_PLLTYPE_7: u32 = 0x38000;
pub const BCMA_CC_CAP_PCTL: u32 = 0x40000;
pub const BCMA_CC_CAP_OTPS: u32 = 0x380000;
pub const BCMA_CC_CAP_OTPS_SHIFT: u32 = 19;
pub const BCMA_CC_CAP_OTPS_BASE: u32 = 5;
pub const BCMA_CC_CAP_JTAGM: u32 = 0x400000;
pub const BCMA_CC_CAP_BROM: u32 = 0x800000;
pub const BCMA_CC_CAP_64BIT: u32 = 0x08000000;
pub const BCMA_CC_CAP_PMU: u32 = 0x10000000;
pub const BCMA_CC_CAP_ECI: u32 = 0x20000000;
pub const BCMA_CC_CAP_SPROM: u32 = 0x40000000;
pub const BCMA_CC_CAP_NFLASH: u32 = 0x80000000;

/* Register offsets and masks retain the source names and integer widths. */
pub const BCMA_CC_CORECTL: u32 = 0x0008;
pub const BCMA_CC_BIST: u32 = 0x000C;
pub const BCMA_CC_OTPS: u32 = 0x0010;
pub const BCMA_CC_OTPC: u32 = 0x0014;
pub const BCMA_CC_OTPP: u32 = 0x0018;
pub const BCMA_CC_OTPL: u32 = 0x001C;
pub const BCMA_CC_IRQSTAT: u32 = 0x0020;
pub const BCMA_CC_IRQMASK: u32 = 0x0024;
pub const BCMA_CC_CHIPCTL: u32 = 0x0028;
pub const BCMA_CC_CHIPSTAT: u32 = 0x002C;
pub const BCMA_CC_JCMD: u32 = 0x0030;
pub const BCMA_CC_JIR: u32 = 0x0034;
pub const BCMA_CC_JDR: u32 = 0x0038;
pub const BCMA_CC_JCTL: u32 = 0x003C;
pub const BCMA_CC_FLASHCTL: u32 = 0x0040;
pub const BCMA_CC_FLASHADDR: u32 = 0x0044;
pub const BCMA_CC_FLASHDATA: u32 = 0x0048;
pub const BCMA_CC_BCAST_ADDR: u32 = 0x0050;
pub const BCMA_CC_BCAST_DATA: u32 = 0x0054;
pub const BCMA_CC_GPIOIN: u32 = 0x0060;
pub const BCMA_CC_GPIOOUT: u32 = 0x0064;
pub const BCMA_CC_GPIOOUTEN: u32 = 0x0068;
pub const BCMA_CC_GPIOCTL: u32 = 0x006C;
pub const BCMA_CC_GPIOPOL: u32 = 0x0070;
pub const BCMA_CC_GPIOIRQ: u32 = 0x0074;
pub const BCMA_CC_WATCHDOG: u32 = 0x0080;
pub const BCMA_CC_PMU_CTL: u32 = 0x0600;
pub const BCMA_CC_PMU_CAP: u32 = 0x0604;
pub const BCMA_CC_PMU_STAT: u32 = 0x0608;
pub const BCMA_CC_SPROM: u32 = 0x0800;
pub const BCMA_CC_PMU_ALP_CLOCK: u32 = 20_000_000;
pub const BCMA_CC_PMU_HT_CLOCK: u32 = 80_000_000;

#[repr(C)]
pub struct bcma_chipcommon_pmu {
    pub core: *mut bcma_device,
    pub rev: u8,
    pub crystalfreq: u32,
}

#[repr(C)]
pub struct bcma_pflash { pub present: bool }
#[repr(C)]
pub struct bcma_sflash { pub present: bool, pub blocksize: u32, pub numblocks: u16, pub size: u32 }
#[repr(C)]
pub struct bcma_nflash { pub brcmnand_info: brcmnand_platform_data, pub present: bool, pub boot: bool }
#[repr(C)]
pub struct bcma_serial_port { pub regs: *mut core::ffi::c_void, pub clockspeed: usize, pub irq: u32, pub baud_base: u32, pub reg_shift: u32 }

#[repr(C)]
pub struct bcma_drv_cc {
    pub core: *mut bcma_device,
    pub status: u32,
    pub capabilities: u32,
    pub capabilities_ext: u32,
    pub setup_done: u8,
    pub early_setup_done: u8,
    pub fast_pwrup_delay: u16,
    pub pmu: bcma_chipcommon_pmu,
    pub ticks_per_ms: u32,
    pub watchdog: *mut platform_device,
    pub gpio_lock: spinlock_t,
    pub gpio: gpio_chip,
}
#[repr(C)]
pub struct bcma_drv_cc_b { pub core: *mut bcma_device, pub setup_done: u8, pub mii: *mut core::ffi::c_void }

pub type u8_ = u8;
pub type u16_ = u16;
pub type u32_ = u32;
pub enum bcma_device {}
pub enum platform_device {}
pub enum brcmnand_platform_data {}
pub enum spinlock_t {}
pub enum gpio_chip {}

extern "C" {
    pub fn bcma_chipco_watchdog_timer_set(cc: *mut bcma_drv_cc, ticks: u32) -> u32;
    pub fn bcma_chipco_get_alp_clock(cc: *mut bcma_drv_cc) -> u32;
    pub fn bcma_chipco_irq_mask(cc: *mut bcma_drv_cc, mask: u32, value: u32);
    pub fn bcma_chipco_irq_status(cc: *mut bcma_drv_cc, mask: u32) -> u32;
    pub fn bcma_chipco_gpio_in(cc: *mut bcma_drv_cc, mask: u32) -> u32;
    pub fn bcma_chipco_gpio_out(cc: *mut bcma_drv_cc, mask: u32, value: u32) -> u32;
    pub fn bcma_chipco_gpio_outen(cc: *mut bcma_drv_cc, mask: u32, value: u32) -> u32;
    pub fn bcma_chipco_gpio_control(cc: *mut bcma_drv_cc, mask: u32, value: u32) -> u32;
    pub fn bcma_chipco_gpio_intmask(cc: *mut bcma_drv_cc, mask: u32, value: u32) -> u32;
    pub fn bcma_chipco_gpio_polarity(cc: *mut bcma_drv_cc, mask: u32, value: u32) -> u32;
    pub fn bcma_chipco_gpio_pullup(cc: *mut bcma_drv_cc, mask: u32, value: u32) -> u32;
    pub fn bcma_chipco_gpio_pulldown(cc: *mut bcma_drv_cc, mask: u32, value: u32) -> u32;
    pub fn bcma_chipco_pll_write(cc: *mut bcma_drv_cc, offset: u32, value: u32);
    pub fn bcma_chipco_pll_maskset(cc: *mut bcma_drv_cc, offset: u32, mask: u32, set: u32);
    pub fn bcma_chipco_chipctl_maskset(cc: *mut bcma_drv_cc, offset: u32, mask: u32, set: u32);
    pub fn bcma_chipco_regctl_maskset(cc: *mut bcma_drv_cc, offset: u32, mask: u32, set: u32);
    pub fn bcma_pmu_spuravoid_pllupdate(cc: *mut bcma_drv_cc, spuravoid: i32);
    pub fn bcma_pmu_get_bus_clock(cc: *mut bcma_drv_cc) -> u32;
    pub fn bcma_chipco_b_mii_write(ccb: *mut bcma_drv_cc_b, offset: u32, value: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
