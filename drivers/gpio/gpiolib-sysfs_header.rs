/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: GPIOLIB_SYSFS_H

#[repr(C)]
pub struct gpio_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_chip {
    _private: [u8; 0],
}

// CONFIG_GPIO_SYSFS is a build-time condition from the C source.
#[cfg(feature = "CONFIG_GPIO_SYSFS")]
extern "C" {
    pub fn gpiochip_sysfs_register(gc: *mut gpio_chip) -> ::std::os::raw::c_int;
    pub fn gpiochip_sysfs_unregister(gc: *mut gpio_chip);
}

// Equivalent to the !CONFIG_GPIO_SYSFS branch.
#[cfg(not(feature = "CONFIG_GPIO_SYSFS"))]
#[inline]
pub unsafe fn gpiochip_sysfs_register(_gc: *mut gpio_chip) -> ::std::os::raw::c_int {
    0
}

#[cfg(not(feature = "CONFIG_GPIO_SYSFS"))]
#[inline]
pub unsafe fn gpiochip_sysfs_unregister(_gc: *mut gpio_chip) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
