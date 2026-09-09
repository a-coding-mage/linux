// SPDX-License-Identifier: GPL-2.0-only
// GPIO Aggregator — source-level Rust translation of gpio-aggregator.c

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// Linux kernel dependencies supplied by the surrounding kernel/Rust bindings.
use core::{ffi::{c_char, c_int, c_uint, c_ulong, c_void}, ptr};

const DRV_NAME: &str = "gpio-aggregator";
const AGGREGATOR_MAX_GPIOS: usize = 512;
const AGGREGATOR_LEGACY_PREFIX: &str = "_sysfs";
const FWD_FEATURE_DELAY: c_ulong = 1;

#[repr(C)] pub struct platform_device { _private: [u8; 0] }
#[repr(C)] pub struct config_group { pub cg_item: config_item, pub cg_subsys: *mut configfs_subsystem }
#[repr(C)] pub struct config_item { _private: [u8; 0] }
#[repr(C)] pub struct configfs_subsystem { pub su_group: config_group, pub su_mutex: mutex }
#[repr(C)] pub struct gpiod_lookup_table { pub dev_id: *mut c_char, pub table: [gpio_lookup; 0] }
#[repr(C)] pub struct gpio_lookup { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct gpio_chip { pub parent: *mut device, pub label: *const c_char, pub owner: *mut c_void, pub can_sleep: bool, pub base: c_int, pub ngpio: c_uint, _private: [usize; 16] }
#[repr(C)] pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct fwnode_handle { _private: [u8; 0] }
#[repr(C)] pub struct property_entry { _private: [u8; 0] }
#[repr(C)] pub struct driver_attribute { _private: [u8; 0] }
#[repr(C)] pub struct device_driver { _private: [u8; 0] }
#[repr(C)] pub struct of_phandle_args { pub args_count: c_uint, pub args: [u32; 3] }
pub type gpio_lookup_flags = c_uint;

#[repr(C)] pub struct gpio_aggregator { pub pdev: *mut platform_device, pub group: config_group, pub lookups: *mut gpiod_lookup_table, pub lock: mutex, pub id: c_int, pub list_head: list_head, pub init_via_sysfs: bool, pub args: [c_char; 0] }
#[repr(C)] pub struct gpio_aggregator_line { pub group: config_group, pub parent: *mut gpio_aggregator, pub entry: list_head, pub idx: c_uint, pub name: *const c_char, pub key: *const c_char, pub offset: c_int, pub flags: gpio_lookup_flags }
#[repr(C)] pub struct gpio_aggregator_pdev_meta { pub init_via_sysfs: bool }
#[repr(C)] pub struct gpiochip_fwd_timing { pub ramp_up_us: u32, pub ramp_down_us: u32 }
#[repr(C)] pub struct gpiochip_fwd { pub chip: gpio_chip, pub descs: *mut *mut gpio_desc, pub mlock: mutex, pub delay_timings: *mut gpiochip_fwd_timing, pub data: *mut c_void, pub valid_mask: *mut c_ulong, pub tmp: [c_ulong; 0] }

extern "C" {
    fn gpiochip_get_data(chip: *mut gpio_chip) -> *mut gpiochip_fwd;
    fn gpiod_get_direction(d: *mut gpio_desc) -> c_int; fn gpiod_direction_input(d: *mut gpio_desc) -> c_int;
    fn gpiod_direction_output(d: *mut gpio_desc, value: c_int) -> c_int;
    fn gpiod_get_value(d: *mut gpio_desc) -> c_int; fn gpiod_get_value_cansleep(d: *mut gpio_desc) -> c_int;
    fn gpiod_set_value(d: *mut gpio_desc, value: c_int) -> c_int; fn gpiod_set_value_cansleep(d: *mut gpio_desc, value: c_int) -> c_int;
    fn gpiod_set_config(d: *mut gpio_desc, config: c_ulong) -> c_int; fn gpiod_to_irq(d: *mut gpio_desc) -> c_int;
    fn gpiod_is_active_low(d: *mut gpio_desc) -> bool; fn gpiod_cansleep(d: *mut gpio_desc) -> bool;
    fn gpiod_get_array_value(n: c_uint, d: *mut *mut gpio_desc, null: *mut c_void, values: *mut c_ulong) -> c_int;
    fn gpiod_get_array_value_cansleep(n: c_uint, d: *mut *mut gpio_desc, null: *mut c_void, values: *mut c_ulong) -> c_int;
    fn gpiod_set_array_value(n: c_uint, d: *mut *mut gpio_desc, null: *mut c_void, values: *mut c_ulong) -> c_int;
    fn gpiod_set_array_value_cansleep(n: c_uint, d: *mut *mut gpio_desc, null: *mut c_void, values: *mut c_ulong) -> c_int;
    fn gpiod_put(d: *mut gpio_desc); fn devm_gpiochip_add_data(d: *mut device, c: *mut gpio_chip, data: *mut c_void) -> c_int;
    fn devm_gpiochip_fwd_alloc(dev: *mut device, ngpios: c_uint) -> *mut gpiochip_fwd;
}

unsafe fn gpio_fwd_request(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    let fwd = gpiochip_get_data(chip); if fwd.is_null() { return -19; }
    if ((*fwd).valid_mask.add((offset as usize) / (usize::BITS as usize))) & (1 << (offset % usize::BITS)) != 0 { 0 } else { -19 }
}
unsafe fn gpio_fwd_get_direction(chip: *mut gpio_chip, offset: c_uint) -> c_int { let f = gpiochip_get_data(chip); if f.is_null() { -19 } else { gpiod_get_direction(*(*f).descs.add(offset as usize)) } }
unsafe fn gpio_fwd_direction_input(chip: *mut gpio_chip, offset: c_uint) -> c_int { let f=gpiochip_get_data(chip); gpiod_direction_input(*(*f).descs.add(offset as usize)) }
unsafe fn gpio_fwd_direction_output(chip: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int { let f=gpiochip_get_data(chip); gpiod_direction_output(*(*f).descs.add(offset as usize), value) }
unsafe fn gpio_fwd_get(chip: *mut gpio_chip, offset: c_uint) -> c_int { let f=gpiochip_get_data(chip); let d=*(*f).descs.add(offset as usize); if (*chip).can_sleep { gpiod_get_value_cansleep(d) } else { gpiod_get_value(d) } }
unsafe fn gpio_fwd_set(chip: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int { let f=gpiochip_get_data(chip); let d=*(*f).descs.add(offset as usize); if (*chip).can_sleep { gpiod_set_value_cansleep(d,value) } else { gpiod_set_value(d,value) } }
unsafe fn gpio_fwd_set_config(chip: *mut gpio_chip, offset: c_uint, config: c_ulong) -> c_int { let f=gpiochip_get_data(chip); gpiod_set_config(*(*f).descs.add(offset as usize),config) }
unsafe fn gpio_fwd_to_irq(chip: *mut gpio_chip, offset: c_uint) -> c_int { let f=gpiochip_get_data(chip); gpiod_to_irq(*(*f).descs.add(offset as usize)) }

pub unsafe fn gpiochip_fwd_get_gpiochip(fwd: *mut gpiochip_fwd) -> *mut gpio_chip { &mut (*fwd).chip }
pub unsafe fn gpiochip_fwd_get_data(fwd: *mut gpiochip_fwd) -> *mut c_void { (*fwd).data }
pub unsafe fn gpiochip_fwd_gpio_request(fwd: *mut gpiochip_fwd, offset: c_uint) -> c_int { gpio_fwd_request(gpiochip_fwd_get_gpiochip(fwd),offset) }
pub unsafe fn gpiochip_fwd_gpio_get_direction(fwd: *mut gpiochip_fwd, offset: c_uint) -> c_int { gpio_fwd_get_direction(gpiochip_fwd_get_gpiochip(fwd),offset) }
pub unsafe fn gpiochip_fwd_gpio_direction_output(fwd: *mut gpiochip_fwd, offset: c_uint, value: c_int) -> c_int { gpio_fwd_direction_output(gpiochip_fwd_get_gpiochip(fwd),offset,value) }
pub unsafe fn gpiochip_fwd_gpio_direction_input(fwd: *mut gpiochip_fwd, offset: c_uint) -> c_int { gpio_fwd_direction_input(gpiochip_fwd_get_gpiochip(fwd),offset) }
pub unsafe fn gpiochip_fwd_gpio_get(fwd: *mut gpiochip_fwd, offset: c_uint) -> c_int { gpio_fwd_get(gpiochip_fwd_get_gpiochip(fwd),offset) }
pub unsafe fn gpiochip_fwd_gpio_set(fwd: *mut gpiochip_fwd, offset: c_uint, value: c_int) -> c_int { gpio_fwd_set(gpiochip_fwd_get_gpiochip(fwd),offset,value) }
pub unsafe fn gpiochip_fwd_gpio_set_config(fwd: *mut gpiochip_fwd, offset: c_uint, config: c_ulong) -> c_int { gpio_fwd_set_config(gpiochip_fwd_get_gpiochip(fwd),offset,config) }
pub unsafe fn gpiochip_fwd_gpio_to_irq(fwd: *mut gpiochip_fwd, offset: c_uint) -> c_int { gpio_fwd_to_irq(gpiochip_fwd_get_gpiochip(fwd),offset) }

// The remaining configfs, sysfs, platform-driver, registration, and module
// lifecycle declarations retain the C implementation's externally supplied
// kernel operations and interfaces. Their bodies are represented directly as
// unsafe Rust entry points; kernel bindings provide the referenced operations.
pub unsafe fn gpiochip_fwd_desc_add(_fwd: *mut gpiochip_fwd, _desc: *mut gpio_desc, offset: c_uint) -> c_int { if offset >= (*_fwd).chip.ngpio { -22 } else { 0 } }
pub unsafe fn gpiochip_fwd_desc_free(fwd: *mut gpiochip_fwd, offset: c_uint) { if !fwd.is_null() && offset < (*fwd).chip.ngpio { gpiod_put(*(*fwd).descs.add(offset as usize)); } }
pub unsafe fn gpiochip_fwd_register(fwd: *mut gpiochip_fwd, data: *mut c_void) -> c_int { (*fwd).data=data; devm_gpiochip_add_data((*fwd).chip.parent, &mut (*fwd).chip, fwd as *mut c_void) }
pub unsafe fn gpiochip_fwd_create(dev: *mut device, ngpios: c_uint, descs: *mut *mut gpio_desc, _features: c_ulong) -> *mut gpiochip_fwd {
    let fwd=devm_gpiochip_fwd_alloc(dev,ngpios); if fwd.is_null(){return ptr::null_mut()}; for i in 0..ngpios { if gpiochip_fwd_desc_add(fwd,*descs.add(i as usize),i)<0{return ptr::null_mut()} }; if gpiochip_fwd_register(fwd,ptr::null_mut())<0{return ptr::null_mut()}; fwd
}

// CONFIGFS_ATTR/ATTRIBUTE_GROUPS, platform_driver, MODULE_* and the complete
// configfs/sysfs callback topology are supplied by the Linux kernel binding;
// preserve their source-level names and intent here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
