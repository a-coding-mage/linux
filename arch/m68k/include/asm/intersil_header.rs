/* SPDX-License-Identifier: GPL-2.0 */

/* bits 0 and 1 */
pub const INTERSIL_FREQ_32K: u8 = 0x00;
pub const INTERSIL_FREQ_1M: u8 = 0x01;
pub const INTERSIL_FREQ_2M: u8 = 0x02;
pub const INTERSIL_FREQ_4M: u8 = 0x03;

/* bit 2 */
pub const INTERSIL_12H_MODE: u8 = 0x00;
pub const INTERSIL_24H_MODE: u8 = 0x04;

/* bit 3 */
pub const INTERSIL_STOP: u8 = 0x00;
pub const INTERSIL_RUN: u8 = 0x08;

/* bit 4 */
pub const INTERSIL_INT_ENABLE: u8 = 0x10;
pub const INTERSIL_INT_DISABLE: u8 = 0x00;

/* bit 5 */
pub const INTERSIL_MODE_NORMAL: u8 = 0x00;
pub const INTERSIL_MODE_TEST: u8 = 0x20;

pub const INTERSIL_HZ_100_MASK: u8 = 0x02;

#[repr(C)]
pub struct intersil_dt {
    pub csec: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub month: u8,
    pub day: u8,
    pub year: u8,
    pub weekday: u8,
}

#[repr(C)]
pub struct intersil_7170 {
    pub counter: intersil_dt,
    pub alarm: intersil_dt,
    pub int_reg: u8,
    pub cmd_reg: u8,
}

extern "C" {
    pub static mut clock_va: *mut core::ffi::c_char;
}

#[macro_export]
macro_rules! intersil_clock {
    () => {
        unsafe { *(clock_va as *mut intersil_7170) }
    };
}

#[macro_export]
macro_rules! intersil_clear {
    () => {{
        unsafe {
            core::ptr::read_volatile(
                core::ptr::addr_of!((* (clock_va as *mut intersil_7170)).int_reg),
            );
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
