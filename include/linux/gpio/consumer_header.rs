/* SPDX-License-Identifier: GPL-2.0 */
// Translated from linux/gpio/consumer.h. Configuration branches are retained
// as comments because their values are supplied by the surrounding build.

use core::ffi::c_void;

#[repr(C)] pub struct acpi_device { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct fwnode_handle { _private: [u8; 0] }
#[repr(C)] pub struct gpio_array { _private: [u8; 0] }
#[repr(C)] pub struct gpio_desc { _private: [u8; 0] }

#[repr(C)]
pub struct gpio_descs {
    pub info: *mut gpio_array,
    pub ndescs: u32,
    pub desc: [*mut gpio_desc; 0],
}

pub const GPIOD_FLAGS_BIT_DIR_SET: u32 = 1 << 0;
pub const GPIOD_FLAGS_BIT_DIR_OUT: u32 = 1 << 1;
pub const GPIOD_FLAGS_BIT_DIR_VAL: u32 = 1 << 2;
pub const GPIOD_FLAGS_BIT_OPEN_DRAIN: u32 = 1 << 3;
// GPIOD_FLAGS_BIT_NONEXCLUSIVE is DEPRECATED, don't use in new code.
pub const GPIOD_FLAGS_BIT_NONEXCLUSIVE: u32 = 1 << 4;

#[repr(C)] #[derive(Copy, Clone)]
pub enum gpiod_flags {
    GPIOD_ASIS = 0,
    GPIOD_IN = GPIOD_FLAGS_BIT_DIR_SET as isize,
    GPIOD_OUT_LOW = (GPIOD_FLAGS_BIT_DIR_SET | GPIOD_FLAGS_BIT_DIR_OUT) as isize,
    GPIOD_OUT_HIGH = (GPIOD_FLAGS_BIT_DIR_SET | GPIOD_FLAGS_BIT_DIR_OUT | GPIOD_FLAGS_BIT_DIR_VAL) as isize,
    GPIOD_OUT_LOW_OPEN_DRAIN = (GPIOD_FLAGS_BIT_DIR_SET | GPIOD_FLAGS_BIT_DIR_OUT | GPIOD_FLAGS_BIT_OPEN_DRAIN) as isize,
    GPIOD_OUT_HIGH_OPEN_DRAIN = (GPIOD_FLAGS_BIT_DIR_SET | GPIOD_FLAGS_BIT_DIR_OUT | GPIOD_FLAGS_BIT_DIR_VAL | GPIOD_FLAGS_BIT_OPEN_DRAIN) as isize,
}

#[cfg(CONFIG_GPIOLIB)]
extern "C" {
    pub fn gpiod_count(dev: *mut device, con_id: *const i8) -> i32;
    pub fn gpiod_get(dev: *mut device, con_id: *const i8, flags: gpiod_flags) -> *mut gpio_desc;
    pub fn gpiod_get_index(dev: *mut device, con_id: *const i8, idx: u32, flags: gpiod_flags) -> *mut gpio_desc;
    pub fn gpiod_get_optional(dev: *mut device, con_id: *const i8, flags: gpiod_flags) -> *mut gpio_desc;
    pub fn gpiod_get_index_optional(dev: *mut device, con_id: *const i8, index: u32, flags: gpiod_flags) -> *mut gpio_desc;
    pub fn gpiod_get_array(dev: *mut device, con_id: *const i8, flags: gpiod_flags) -> *mut gpio_descs;
    pub fn gpiod_get_array_optional(dev: *mut device, con_id: *const i8, flags: gpiod_flags) -> *mut gpio_descs;
    pub fn gpiod_put(desc: *mut gpio_desc);
    pub fn gpiod_put_array(descs: *mut gpio_descs);
    pub fn devm_gpiod_get(dev: *mut device, con_id: *const i8, flags: gpiod_flags) -> *mut gpio_desc;
    pub fn devm_gpiod_get_index(dev: *mut device, con_id: *const i8, idx: u32, flags: gpiod_flags) -> *mut gpio_desc;
    pub fn devm_gpiod_get_optional(dev: *mut device, con_id: *const i8, flags: gpiod_flags) -> *mut gpio_desc;
    pub fn devm_gpiod_get_index_optional(dev: *mut device, con_id: *const i8, index: u32, flags: gpiod_flags) -> *mut gpio_desc;
    pub fn devm_gpiod_get_array(dev: *mut device, con_id: *const i8, flags: gpiod_flags) -> *mut gpio_descs;
    pub fn devm_gpiod_get_array_optional(dev: *mut device, con_id: *const i8, flags: gpiod_flags) -> *mut gpio_descs;
    pub fn devm_gpiod_put(dev: *mut device, desc: *mut gpio_desc);
    pub fn devm_gpiod_unhinge(dev: *mut device, desc: *mut gpio_desc);
    pub fn devm_gpiod_put_array(dev: *mut device, descs: *mut gpio_descs);
    pub fn gpiod_get_direction(desc: *mut gpio_desc) -> i32;
    pub fn gpiod_is_single_ended(desc: *mut gpio_desc) -> bool;
    pub fn gpiod_direction_input(desc: *mut gpio_desc) -> i32;
    pub fn gpiod_direction_output(desc: *mut gpio_desc, value: i32) -> i32;
    pub fn gpiod_direction_output_raw(desc: *mut gpio_desc, value: i32) -> i32;
    pub fn gpiod_get_value(desc: *const gpio_desc) -> i32;
    pub fn gpiod_get_array_value(array_size: u32, desc_array: *mut *mut gpio_desc, array_info: *mut gpio_array, value_bitmap: *mut usize) -> i32;
    pub fn gpiod_set_value(desc: *mut gpio_desc, value: i32) -> i32;
    pub fn gpiod_set_array_value(array_size: u32, desc_array: *mut *mut gpio_desc, array_info: *mut gpio_array, value_bitmap: *mut usize) -> i32;
    pub fn gpiod_get_raw_value(desc: *const gpio_desc) -> i32;
    pub fn gpiod_get_raw_array_value(array_size: u32, desc_array: *mut *mut gpio_desc, array_info: *mut gpio_array, value_bitmap: *mut usize) -> i32;
    pub fn gpiod_set_raw_value(desc: *mut gpio_desc, value: i32) -> i32;
    pub fn gpiod_set_raw_array_value(array_size: u32, desc_array: *mut *mut gpio_desc, array_info: *mut gpio_array, value_bitmap: *mut usize) -> i32;
    pub fn gpiod_get_value_cansleep(desc: *const gpio_desc) -> i32;
    pub fn gpiod_get_array_value_cansleep(array_size: u32, desc_array: *mut *mut gpio_desc, array_info: *mut gpio_array, value_bitmap: *mut usize) -> i32;
    pub fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: i32) -> i32;
    pub fn gpiod_set_array_value_cansleep(array_size: u32, desc_array: *mut *mut gpio_desc, array_info: *mut gpio_array, value_bitmap: *mut usize) -> i32;
    pub fn gpiod_get_raw_value_cansleep(desc: *const gpio_desc) -> i32;
    pub fn gpiod_get_raw_array_value_cansleep(array_size: u32, desc_array: *mut *mut gpio_desc, array_info: *mut gpio_array, value_bitmap: *mut usize) -> i32;
    pub fn gpiod_set_raw_value_cansleep(desc: *mut gpio_desc, value: i32) -> i32;
    pub fn gpiod_set_raw_array_value_cansleep(array_size: u32, desc_array: *mut *mut gpio_desc, array_info: *mut gpio_array, value_bitmap: *mut usize) -> i32;
    pub fn gpiod_set_config(desc: *mut gpio_desc, config: usize) -> i32;
    pub fn gpiod_set_debounce(desc: *mut gpio_desc, debounce: u32) -> i32;
    pub fn gpiod_toggle_active_low(desc: *mut gpio_desc);
    pub fn gpiod_is_active_low(desc: *const gpio_desc) -> i32;
    pub fn gpiod_cansleep(desc: *const gpio_desc) -> i32;
    pub fn gpiod_to_irq(desc: *const gpio_desc) -> i32;
    pub fn gpiod_set_consumer_name(desc: *mut gpio_desc, name: *const i8) -> i32;
    pub fn gpiod_is_shared(desc: *const gpio_desc) -> bool;
    pub fn gpio_to_desc(gpio: u32) -> *mut gpio_desc;
    pub fn desc_to_gpio(desc: *const gpio_desc) -> i32;
    pub fn gpiod_hwgpio(desc: *const gpio_desc) -> i32;
    pub fn fwnode_gpiod_get_index(fwnode: *mut fwnode_handle, con_id: *const i8, index: i32, flags: gpiod_flags, label: *const i8) -> *mut gpio_desc;
    pub fn devm_fwnode_gpiod_get_index(dev: *mut device, child: *mut fwnode_handle, con_id: *const i8, index: i32, flags: gpiod_flags, label: *const i8) -> *mut gpio_desc;
    pub fn gpiod_is_equal(desc: *const gpio_desc, other: *const gpio_desc) -> bool;
}

#[repr(C)] pub struct acpi_gpio_params { pub crs_entry_index: u32, pub line_index: u16, pub active_low: bool }
#[repr(C)] pub struct acpi_gpio_mapping { pub name: *const i8, pub data: *const acpi_gpio_params, pub size: u32, pub quirks: u32 }
pub const ACPI_GPIO_QUIRK_NO_IO_RESTRICTION: u32 = 1 << 0;
pub const ACPI_GPIO_QUIRK_ONLY_GPIOIO: u32 = 1 << 1;
pub const ACPI_GPIO_QUIRK_ABSOLUTE_NUMBER: u32 = 1 << 2;

pub const ENOSYS: i32 = 38; pub const ENOENT: i32 = 2; pub const ENXIO: i32 = 6; pub const EINVAL: i32 = 22;
#[inline] pub unsafe fn fwnode_gpiod_get(f: *mut fwnode_handle, c: *const i8, x: gpiod_flags, l: *const i8) -> *mut gpio_desc { fwnode_gpiod_get_index(f,c,0,x,l) }
#[inline] pub unsafe fn devm_fwnode_gpiod_get(d: *mut device, f: *mut fwnode_handle, c: *const i8, x: gpiod_flags, l: *const i8) -> *mut gpio_desc { devm_fwnode_gpiod_get_index(d,f,c,0,x,l) }
#[inline] pub unsafe fn devm_fwnode_gpiod_get_optional(d: *mut device, f: *mut fwnode_handle, c: *const i8, x: gpiod_flags, l: *const i8) -> *mut gpio_desc { let p=devm_fwnode_gpiod_get_index(d,f,c,0,x,l); if p as isize == -ENOENT as isize { core::ptr::null_mut() } else { p } }

// CONFIG_GPIOLIB, CONFIG_ACPI, CONFIG_GPIO_SYSFS and CONFIG_HTE conditional
// declarations and fallback inline implementations from the header.
extern "C" {
    pub fn acpi_dev_add_driver_gpios(a: *mut acpi_device, g: *const acpi_gpio_mapping) -> i32;
    pub fn acpi_dev_remove_driver_gpios(a: *mut acpi_device);
    pub fn devm_acpi_dev_add_driver_gpios(d: *mut device, g: *const acpi_gpio_mapping) -> i32;
    pub fn gpiod_export(d: *mut gpio_desc, direction_may_change: bool) -> i32;
    pub fn gpiod_export_link(d: *mut device, name: *const i8, desc: *mut gpio_desc) -> i32;
    pub fn gpiod_unexport(d: *mut gpio_desc);
    pub fn gpiod_enable_hw_timestamp_ns(d: *mut gpio_desc, flags: usize) -> i32;
    pub fn gpiod_disable_hw_timestamp_ns(d: *mut gpio_desc, flags: usize) -> i32;
}

#[inline] pub unsafe fn gpiod_multi_set_value_cansleep(d: *mut gpio_descs, b: *mut usize) -> i32 {
    if d.is_null() { return 0; }
    gpiod_set_array_value_cansleep((*d).ndescs, (*d).desc.as_mut_ptr(), (*d).info, b)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
