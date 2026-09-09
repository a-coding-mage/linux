/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: GPIOLIB_CDEV_H
// Dependency intent: <linux/types.h>

pub struct gpio_device;

extern "C" {
    pub fn gpiolib_cdev_register(gc: *mut gpio_chip, devt: dev_t) -> ::core::ffi::c_int;
    pub fn gpiolib_cdev_unregister(gdev: *mut gpio_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
