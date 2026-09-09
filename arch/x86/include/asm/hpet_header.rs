/* SPDX-License-Identifier: GPL-2.0 */

/* C header guard and includes omitted; external kernel types are dependencies. */

#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_MMAP_SIZE: usize = 1024;

#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_ID: usize = 0x000;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_PERIOD: usize = 0x004;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_CFG: usize = 0x010;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_STATUS: usize = 0x020;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_COUNTER: usize = 0x0f0;

#[cfg(feature = "CONFIG_HPET_TIMER")]
#[inline]
pub const fn HPET_Tn_CFG(n: usize) -> usize { 0x100 + 0x20 * n }
#[cfg(feature = "CONFIG_HPET_TIMER")]
#[inline]
pub const fn HPET_Tn_CMP(n: usize) -> usize { 0x108 + 0x20 * n }
#[cfg(feature = "CONFIG_HPET_TIMER")]
#[inline]
pub const fn HPET_Tn_ROUTE(n: usize) -> usize { 0x110 + 0x20 * n }

#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_T0_CFG: usize = 0x100;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_T0_CMP: usize = 0x108;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_T0_ROUTE: usize = 0x110;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_T1_CFG: usize = 0x120;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_T1_CMP: usize = 0x128;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_T1_ROUTE: usize = 0x130;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_T2_CFG: usize = 0x140;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_T2_CMP: usize = 0x148;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_T2_ROUTE: usize = 0x150;

#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_ID_REV: u32 = 0x000000ff;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_ID_NUMBER: u32 = 0x00001f00;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_ID_64BIT: u32 = 0x00002000;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_ID_LEGSUP: u32 = 0x00008000;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_ID_VENDOR: u32 = 0xffff0000;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_ID_NUMBER_SHIFT: u32 = 8;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_ID_VENDOR_SHIFT: u32 = 16;

#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_CFG_ENABLE: u32 = 0x001;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_CFG_LEGACY: u32 = 0x002;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_LEGACY_8254: u32 = 2;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_LEGACY_RTC: u32 = 8;

#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_TN_LEVEL: u32 = 0x0002;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_TN_ENABLE: u32 = 0x0004;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_TN_PERIODIC: u32 = 0x0008;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_TN_PERIODIC_CAP: u32 = 0x0010;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_TN_64BIT_CAP: u32 = 0x0020;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_TN_SETVAL: u32 = 0x0040;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_TN_32BIT: u32 = 0x0100;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_TN_ROUTE: u32 = 0x3e00;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_TN_FSB: u32 = 0x4000;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_TN_FSB_CAP: u32 = 0x8000;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_TN_ROUTE_SHIFT: u32 = 9;

/* Max HPET Period is 10^8 femto sec as in HPET spec */
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_MAX_PERIOD: usize = 100000000;
/* Min HPET period is 10^5 femto sec just for safety. If it is less than this,
 * then 32 bit HPET counter wrapsaround in less than 0.5 sec. */
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub const HPET_MIN_PERIOD: usize = 100000;

#[cfg(feature = "CONFIG_HPET_TIMER")]
pub static mut hpet_address: usize = 0;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub static mut force_hpet_address: usize = 0;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub static mut boot_hpet_disable: bool = false;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub static mut hpet_blockid: u8 = 0;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub static mut hpet_force_user: bool = false;
#[cfg(feature = "CONFIG_HPET_TIMER")]
pub static mut hpet_msi_disable: bool = false;

#[cfg(feature = "CONFIG_HPET_TIMER")]
extern "C" {
    pub fn is_hpet_enabled() -> i32;
    pub fn hpet_enable() -> i32;
    pub fn hpet_disable();
    pub fn hpet_readl(a: u32) -> u32;
    pub fn force_hpet_resume();
}

#[cfg(all(feature = "CONFIG_HPET_TIMER", feature = "CONFIG_HPET_EMULATE_RTC"))]
pub type rtc_irq_handler = unsafe extern "C" fn(interrupt: i32, cookie: *mut core::ffi::c_void) -> irqreturn_t;

#[cfg(all(feature = "CONFIG_HPET_TIMER", feature = "CONFIG_HPET_EMULATE_RTC"))]
extern "C" {
    pub fn hpet_mask_rtc_irq_bit(bit_mask: usize) -> i32;
    pub fn hpet_set_rtc_irq_bit(bit_mask: usize) -> i32;
    pub fn hpet_set_alarm_time(hrs: u8, min: u8, sec: u8) -> i32;
    pub fn hpet_set_periodic_freq(freq: usize) -> i32;
    pub fn hpet_rtc_timer_init() -> i32;
    pub fn hpet_rtc_interrupt(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t;
    pub fn hpet_register_irq_handler(handler: rtc_irq_handler) -> i32;
    pub fn hpet_unregister_irq_handler(handler: rtc_irq_handler);
}

#[cfg(not(feature = "CONFIG_HPET_TIMER"))]
#[inline]
pub fn hpet_enable() -> i32 { 0 }
#[cfg(not(feature = "CONFIG_HPET_TIMER"))]
#[inline]
pub fn is_hpet_enabled() -> i32 { 0 }
#[cfg(not(feature = "CONFIG_HPET_TIMER"))]
#[inline]
pub const fn hpet_readl(_a: u32) -> u32 { 0 }
#[cfg(not(feature = "CONFIG_HPET_TIMER"))]
pub const DEFAULT_SETUP_HPET_MSI: *const core::ffi::c_void = core::ptr::null();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
