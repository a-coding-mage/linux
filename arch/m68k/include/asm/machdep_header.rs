/* SPDX-License-Identifier: GPL-2.0 */

/* Linux header dependencies are supplied by the surrounding translation. */

use core::ffi::{c_char, c_int, c_uint, c_ulong};

#[repr(C)]
pub struct seq_file;
#[repr(C)]
pub struct pt_regs;
#[repr(C)]
pub struct mktime;
#[repr(C)]
pub struct rtc_time;
#[repr(C)]
pub struct rtc_pll_info;
#[repr(C)]
pub struct buffer_head;

pub static mut mach_sched_init: Option<unsafe extern "C" fn()>;

/* machine dependent irq functions */
pub static mut mach_init_IRQ: Option<unsafe extern "C" fn()>;
pub static mut mach_get_model: Option<unsafe extern "C" fn(*mut c_char)>;
pub static mut mach_get_hardware_list: Option<unsafe extern "C" fn(*mut seq_file)>;

/* machine dependent timer functions */
pub static mut mach_hwclk: Option<unsafe extern "C" fn(c_int, *mut rtc_time) -> c_int>;
pub static mut mach_get_rtc_pll:
    Option<unsafe extern "C" fn(*mut rtc_pll_info) -> c_int>;
pub static mut mach_set_rtc_pll:
    Option<unsafe extern "C" fn(*mut rtc_pll_info) -> c_int>;
pub static mut mach_reset: Option<unsafe extern "C" fn()>;
pub static mut mach_halt: Option<unsafe extern "C" fn()>;
pub static mut mach_hd_init:
    Option<unsafe extern "C" fn(c_ulong, c_ulong) -> c_ulong>;
pub static mut mach_hd_setup: Option<unsafe extern "C" fn(*mut c_char, *mut c_int)>;
pub static mut mach_heartbeat: Option<unsafe extern "C" fn(c_int)>;
pub static mut mach_l2_flush: Option<unsafe extern "C" fn(c_int)>;
pub static mut mach_beep: Option<unsafe extern "C" fn(c_uint, c_uint)>;

/* Hardware clock functions */
unsafe extern "C" {
    pub fn hw_timer_init();
}

/* CONFIG_HEARTBEAT selects the external implementation at build time. */
#[cfg(feature = "CONFIG_HEARTBEAT")]
unsafe extern "C" {
    pub fn timer_heartbeat();
}

#[cfg(not(feature = "CONFIG_HEARTBEAT"))]
#[inline]
pub fn timer_heartbeat() {}

unsafe extern "C" {
    pub fn config_BSP(command: *mut c_char, len: c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
