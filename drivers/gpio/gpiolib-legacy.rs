// SPDX-License-Identifier: GPL-2.0
// Translated from gpiolib-legacy.c. Kernel include dependencies are supplied
// by the surrounding translation unit.

use core::ffi::{c_char, c_int, c_void};

// External kernel types and functions supplied by the GPIO/device subsystem.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn gpio_to_desc(gpio: u32) -> *mut gpio_desc;
    fn gpiod_free(desc: *mut gpio_desc);
    fn gpio_direction_input(gpio: u32) -> c_int;
    fn gpio_direction_output(gpio: u32, value: c_int) -> c_int;
    fn gpiod_request(desc: *mut gpio_desc, label: *const c_char) -> c_int;
    fn devm_add_action_or_reset(
        dev: *mut device,
        action: unsafe extern "C" fn(*mut c_void),
        data: *mut c_void,
    ) -> c_int;
}

// GPIOF_* and errno values are provided by the kernel GPIO headers.
const GPIOF_IN: u64 = 1 << 0;
const GPIOF_OUT_INIT_HIGH: u64 = 1 << 1;
const EPROBE_DEFER: c_int = 517;

/*
 * **DEPRECATED** This function is deprecated and must not be used in new code.
 */
#[no_mangle]
pub unsafe extern "C" fn gpio_free(gpio: u32) {
    gpiod_free(gpio_to_desc(gpio));
}

/**
 * gpio_request_one - request a single GPIO with initial configuration
 * @gpio: the GPIO number
 * @flags: GPIO configuration as specified by GPIOF_*
 * @label: a literal description string of this GPIO
 *
 * **DEPRECATED** This function is deprecated and must not be used in new code.
 *
 * Returns:
 * 0 on success, or negative errno on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn gpio_request_one(
    gpio: u32,
    flags: u64,
    label: *const c_char,
) -> c_int {
    let mut err: c_int;

    err = gpio_request(gpio, label);
    if err != 0 {
        return err;
    }

    if flags & GPIOF_IN != 0 {
        err = gpio_direction_input(gpio);
    } else {
        err = gpio_direction_output(gpio, if flags & GPIOF_OUT_INIT_HIGH != 0 { 1 } else { 0 });
    }

    if err != 0 {
        gpio_free(gpio);
    }

    err
}

/*
 * **DEPRECATED** This function is deprecated and must not be used in new code.
 */
#[no_mangle]
pub unsafe extern "C" fn gpio_request(gpio: u32, label: *const c_char) -> c_int {
    let desc: *mut gpio_desc;

    /* Compatibility: assume unavailable "valid" GPIOs will appear later */
    desc = gpio_to_desc(gpio);
    if desc.is_null() {
        return -EPROBE_DEFER;
    }

    gpiod_request(desc, label)
}

unsafe extern "C" fn devm_gpio_release(gpio: *mut c_void) {
    gpio_free(gpio as usize as u32);
}

/**
 * devm_gpio_request_one - request a single GPIO with initial setup
 * @dev: device to request for
 * @gpio: the GPIO number
 * @flags: GPIO configuration as specified by GPIOF_*
 * @label: a literal description string of this GPIO
 *
 * **DEPRECATED** This function is deprecated and must not be used in new code.
 *
 * Returns:
 * 0 on success, or negative errno on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn devm_gpio_request_one(
    dev: *mut device,
    gpio: u32,
    flags: u64,
    label: *const c_char,
) -> c_int {
    let mut rc: c_int;

    rc = gpio_request(gpio, label);
    if rc != 0 {
        return rc;
    }

    if flags & GPIOF_IN != 0 {
        rc = gpio_direction_input(gpio);
    } else {
        rc = gpio_direction_output(gpio, if flags & GPIOF_OUT_INIT_HIGH != 0 { 1 } else { 0 });
    }

    if rc != 0 {
        gpio_free(gpio);
        return rc;
    }

    devm_add_action_or_reset(dev, devm_gpio_release, gpio as usize as *mut c_void)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
