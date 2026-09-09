/* SPDX-License-Identifier: GPL-2.0 */

// The definition of `rtc_time` is supplied by the corresponding dependency.
#[repr(C)]
pub struct rtc_time {
    _private: [u8; 0],
}

extern "C" {
    pub fn sun3x_hwclk(set: i32, t: *mut rtc_time) -> i32;
    pub fn sun3x_sched_init();
}

#[repr(C)]
pub struct mostek_dt {
    pub csr: core::cell::UnsafeCell<u8>,
    pub sec: core::cell::UnsafeCell<u8>,
    pub min: core::cell::UnsafeCell<u8>,
    pub hour: core::cell::UnsafeCell<u8>,
    pub wday: core::cell::UnsafeCell<u8>,
    pub mday: core::cell::UnsafeCell<u8>,
    pub month: core::cell::UnsafeCell<u8>,
    pub year: core::cell::UnsafeCell<u8>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
