/* SPDX-License-Identifier: GPL-2.0 */

// The declarations in this header are enabled when CONFIG_RS780_HPET is set.
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_MMAP_SIZE: usize = 1024;

#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_ID: usize = 0x000;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_PERIOD: usize = 0x004;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_CFG: usize = 0x010;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_STATUS: usize = 0x020;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_COUNTER: usize = 0x0f0;

#[cfg(feature = "CONFIG_RS780_HPET")]
#[inline]
pub const fn HPET_Tn_CFG(n: usize) -> usize { 0x100 + 0x20 * n }
#[cfg(feature = "CONFIG_RS780_HPET")]
#[inline]
pub const fn HPET_Tn_CMP(n: usize) -> usize { 0x108 + 0x20 * n }
#[cfg(feature = "CONFIG_RS780_HPET")]
#[inline]
pub const fn HPET_Tn_ROUTE(n: usize) -> usize { 0x110 + 0x20 * n }

#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_T0_IRS: u32 = 0x001;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_T1_IRS: u32 = 0x002;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_T3_IRS: u32 = 0x004;

#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_T0_CFG: usize = 0x100;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_T0_CMP: usize = 0x108;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_T0_ROUTE: usize = 0x110;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_T1_CFG: usize = 0x120;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_T1_CMP: usize = 0x128;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_T1_ROUTE: usize = 0x130;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_T2_CFG: usize = 0x140;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_T2_CMP: usize = 0x148;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_T2_ROUTE: usize = 0x150;

#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_ID_REV: u32 = 0x000000ff;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_ID_NUMBER: u32 = 0x00001f00;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_ID_64BIT: u32 = 0x00002000;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_ID_LEGSUP: u32 = 0x00008000;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_ID_VENDOR: u32 = 0xffff0000;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_ID_NUMBER_SHIFT: u32 = 8;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_ID_VENDOR_SHIFT: u32 = 16;

#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_CFG_ENABLE: u32 = 0x001;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_CFG_LEGACY: u32 = 0x002;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_LEGACY_8254: u32 = 2;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_LEGACY_RTC: u32 = 8;

#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_TN_LEVEL: u32 = 0x0002;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_TN_ENABLE: u32 = 0x0004;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_TN_PERIODIC: u32 = 0x0008;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_TN_PERIODIC_CAP: u32 = 0x0010;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_TN_64BIT_CAP: u32 = 0x0020;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_TN_SETVAL: u32 = 0x0040;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_TN_32BIT: u32 = 0x0100;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_TN_ROUTE: u32 = 0x3e00;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_TN_FSB: u32 = 0x4000;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_TN_FSB_CAP: u32 = 0x8000;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_TN_ROUTE_SHIFT: u32 = 9;

/* Max HPET Period is 10^8 femto sec as in HPET spec */
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_MAX_PERIOD: u64 = 100000000;
/*
 * Min HPET period is 10^5 femto sec just for safety. If it is less than this,
 * then 32 bit HPET counter wrapsaround in less than 0.5 sec.
 */
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_MIN_PERIOD: u64 = 100000;

#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_ADDR: usize = 0x20000;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_MMIO_ADDR: u64 = 0x90000e0000020000;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_FREQ: u64 = 14318780;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_COMPARE_VAL: u64 = (HPET_FREQ + HZ / 2) / HZ;
#[cfg(feature = "CONFIG_RS780_HPET")]
pub const HPET_T0_IRQ: u32 = 0;

#[cfg(feature = "CONFIG_RS780_HPET")]
// C declaration: extern void __init setup_hpet_timer(void);
unsafe extern "C" {
    pub fn setup_hpet_timer();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
