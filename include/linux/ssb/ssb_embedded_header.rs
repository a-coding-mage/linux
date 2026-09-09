/* SPDX-License-Identifier: GPL-2.0 */

// Dependency declarations corresponding to <linux/types.h> and
// <linux/ssb/ssb.h> are supplied by other files.

#[repr(C)]
pub struct ssb_bus {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn ssb_watchdog_timer_set(bus: *mut ssb_bus, ticks: u32) -> i32;

    /* Generic GPIO API */
    pub fn ssb_gpio_in(bus: *mut ssb_bus, mask: u32) -> u32;
    pub fn ssb_gpio_out(bus: *mut ssb_bus, mask: u32, value: u32) -> u32;
    pub fn ssb_gpio_outen(bus: *mut ssb_bus, mask: u32, value: u32) -> u32;
    pub fn ssb_gpio_control(bus: *mut ssb_bus, mask: u32, value: u32) -> u32;
    pub fn ssb_gpio_intmask(bus: *mut ssb_bus, mask: u32, value: u32) -> u32;
    pub fn ssb_gpio_polarity(bus: *mut ssb_bus, mask: u32, value: u32) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
