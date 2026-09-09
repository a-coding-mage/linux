/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_chip {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpiochip_fwd {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn devm_gpiochip_fwd_alloc(
        dev: *mut device,
        ngpios: c_uint,
    ) -> *mut gpiochip_fwd;

    pub fn gpiochip_fwd_desc_add(
        fwd: *mut gpiochip_fwd,
        desc: *mut gpio_desc,
        offset: c_uint,
    ) -> c_int;

    pub fn gpiochip_fwd_desc_free(fwd: *mut gpiochip_fwd, offset: c_uint);

    pub fn gpiochip_fwd_register(fwd: *mut gpiochip_fwd, data: *mut c_void) -> c_int;

    pub fn gpiochip_fwd_get_gpiochip(fwd: *mut gpiochip_fwd) -> *mut gpio_chip;

    pub fn gpiochip_fwd_get_data(fwd: *mut gpiochip_fwd) -> *mut c_void;

    pub fn gpiochip_fwd_gpio_request(fwd: *mut gpiochip_fwd, offset: c_uint) -> c_int;

    pub fn gpiochip_fwd_gpio_get_direction(
        fwd: *mut gpiochip_fwd,
        offset: c_uint,
    ) -> c_int;

    pub fn gpiochip_fwd_gpio_direction_input(
        fwd: *mut gpiochip_fwd,
        offset: c_uint,
    ) -> c_int;

    pub fn gpiochip_fwd_gpio_direction_output(
        fwd: *mut gpiochip_fwd,
        offset: c_uint,
        value: c_int,
    ) -> c_int;

    pub fn gpiochip_fwd_gpio_get(fwd: *mut gpiochip_fwd, offset: c_uint) -> c_int;

    pub fn gpiochip_fwd_gpio_get_multiple(
        fwd: *mut gpiochip_fwd,
        mask: *mut c_ulong,
        bits: *mut c_ulong,
    ) -> c_int;

    pub fn gpiochip_fwd_gpio_set(
        fwd: *mut gpiochip_fwd,
        offset: c_uint,
        value: c_int,
    ) -> c_int;

    pub fn gpiochip_fwd_gpio_set_multiple(
        fwd: *mut gpiochip_fwd,
        mask: *mut c_ulong,
        bits: *mut c_ulong,
    ) -> c_int;

    pub fn gpiochip_fwd_gpio_set_config(
        fwd: *mut gpiochip_fwd,
        offset: c_uint,
        config: c_ulong,
    ) -> c_int;

    pub fn gpiochip_fwd_gpio_to_irq(fwd: *mut gpiochip_fwd, offset: c_uint) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
