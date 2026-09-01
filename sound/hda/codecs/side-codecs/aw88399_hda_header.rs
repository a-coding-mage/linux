/* SPDX-License-Identifier: GPL-2.0-only
 *
 * AW88399 HDA side codec driver
 */

// C dependencies:
// #include <linux/device.h>
// #include <linux/gpio/consumer.h>
// #include <sound/aw88399.h>

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct aw88399 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct aw_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct aw88399_hda {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub reset_gpio: *mut gpio_desc,
    pub aw_dev: *mut aw_device,
    pub core: *mut aw88399,
    pub bsts_unreliable: bool,

    pub acpi_subsystem_id: *const c_char,
    pub index: c_int,
    pub channel: c_int,

    pub playing: bool,
}

unsafe extern "C" {
    pub fn aw88399_hda_probe(dev: *mut device, regmap: *mut regmap) -> c_int;
    pub fn aw88399_hda_remove(dev: *mut device);

    pub static aw88399_hda_pm_ops: dev_pm_ops;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
