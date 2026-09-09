/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: GPIOLIB_OF_H

use core::ffi::{c_char, c_int, c_ulong};

// Forward declarations supplied by other translation units.
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}
#[repr(C)]
pub struct fwnode_reference_args {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gpio_chip {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gpio_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}

// Supplied by the Linux error-pointer and errno definitions.
unsafe extern "C" {
    pub fn ERR_PTR(err: c_int) -> *mut gpio_desc;
    pub static gpio_of_notifier: notifier_block;
}

pub const ENOENT: c_int = 2;

// CONFIG_OF_GPIO controls whether these declarations or the inline stubs are used.
#[cfg(feature = "CONFIG_OF_GPIO")]
unsafe extern "C" {
    pub fn of_find_gpio(
        np: *mut device_node,
        con_id: *const c_char,
        idx: u32,
        lookupflags: *mut c_ulong,
    ) -> *mut gpio_desc;
    pub fn of_gpiochip_add(gc: *mut gpio_chip) -> c_int;
    pub fn of_gpiochip_remove(gc: *mut gpio_chip);
    pub fn of_gpiochip_instance_match(gc: *mut gpio_chip, index: u32) -> bool;
    pub fn of_gpio_count(fwnode: *const fwnode_handle, con_id: *const c_char) -> c_int;
    pub fn of_gpiochip_get_lflags(
        chip: *mut gpio_chip,
        gpiospec: *mut fwnode_reference_args,
        lflags: *mut c_ulong,
    ) -> c_int;
}

#[cfg(not(feature = "CONFIG_OF_GPIO"))]
pub unsafe fn of_find_gpio(
    _np: *mut device_node,
    _con_id: *const c_char,
    _idx: u32,
    _lookupflags: *mut c_ulong,
) -> *mut gpio_desc {
    unsafe { ERR_PTR(-ENOENT) }
}

#[cfg(not(feature = "CONFIG_OF_GPIO"))]
pub unsafe fn of_gpiochip_add(_gc: *mut gpio_chip) -> c_int {
    0
}

#[cfg(not(feature = "CONFIG_OF_GPIO"))]
pub unsafe fn of_gpiochip_remove(_gc: *mut gpio_chip) {}

#[cfg(not(feature = "CONFIG_OF_GPIO"))]
pub unsafe fn of_gpiochip_instance_match(_gc: *mut gpio_chip, _index: u32) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_OF_GPIO"))]
pub unsafe fn of_gpio_count(
    _fwnode: *const fwnode_handle,
    _con_id: *const c_char,
) -> c_int {
    0
}

#[cfg(not(feature = "CONFIG_OF_GPIO"))]
pub unsafe fn of_gpiochip_get_lflags(
    _chip: *mut gpio_chip,
    _gpiospec: *mut fwnode_reference_args,
    _lflags: *mut c_ulong,
) -> c_int {
    -ENOENT
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
