/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/nvmem-consumer.h. */

use core::ffi::c_void;

#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct device_node { _private: [u8; 0] }
#[repr(C)]
pub struct nvmem_cell { _private: [u8; 0] }
#[repr(C)]
pub struct nvmem_device { _private: [u8; 0] }
#[repr(C)]
pub struct nvmem_cell_info { _private: [u8; 0] }
#[repr(C)]
pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)]
pub struct notifier_block { _private: [u8; 0] }

#[repr(C)]
pub struct nvmem_cell_lookup {
    pub nvmem_name: *const core::ffi::c_char,
    pub cell_name: *const core::ffi::c_char,
    pub dev_id: *const core::ffi::c_char,
    pub con_id: *const core::ffi::c_char,
    pub node: list_head,
}

pub const NVMEM_ADD: u32 = 1;
pub const NVMEM_REMOVE: u32 = 2;
pub const NVMEM_CELL_ADD: u32 = 3;
pub const NVMEM_CELL_REMOVE: u32 = 4;
pub const NVMEM_LAYOUT_ADD: u32 = 5;
pub const NVMEM_LAYOUT_REMOVE: u32 = 6;

#[cfg(feature = "CONFIG_NVMEM")]
extern "C" {
    pub fn nvmem_cell_get(dev: *mut device, id: *const core::ffi::c_char) -> *mut nvmem_cell;
    pub fn devm_nvmem_cell_get(dev: *mut device, id: *const core::ffi::c_char) -> *mut nvmem_cell;
    pub fn nvmem_cell_put(cell: *mut nvmem_cell);
    pub fn devm_nvmem_cell_put(dev: *mut device, cell: *mut nvmem_cell);
    pub fn nvmem_cell_read(cell: *mut nvmem_cell, len: *mut usize) -> *mut c_void;
    pub fn nvmem_cell_write(cell: *mut nvmem_cell, buf: *mut c_void, len: usize) -> i32;
    pub fn nvmem_cell_read_u8(dev: *mut device, cell_id: *const core::ffi::c_char, val: *mut u8) -> i32;
    pub fn nvmem_cell_read_u16(dev: *mut device, cell_id: *const core::ffi::c_char, val: *mut u16) -> i32;
    pub fn nvmem_cell_read_u32(dev: *mut device, cell_id: *const core::ffi::c_char, val: *mut u32) -> i32;
    pub fn nvmem_cell_read_u64(dev: *mut device, cell_id: *const core::ffi::c_char, val: *mut u64) -> i32;
    pub fn nvmem_cell_read_variable_le_u32(dev: *mut device, cell_id: *const core::ffi::c_char, val: *mut u32) -> i32;
    pub fn nvmem_cell_read_variable_le_u64(dev: *mut device, cell_id: *const core::ffi::c_char, val: *mut u64) -> i32;
    pub fn nvmem_device_get(dev: *mut device, name: *const core::ffi::c_char) -> *mut nvmem_device;
    pub fn devm_nvmem_device_get(dev: *mut device, name: *const core::ffi::c_char) -> *mut nvmem_device;
    pub fn nvmem_device_put(nvmem: *mut nvmem_device);
    pub fn devm_nvmem_device_put(dev: *mut device, nvmem: *mut nvmem_device);
    pub fn nvmem_device_read(nvmem: *mut nvmem_device, offset: u32, bytes: usize, buf: *mut c_void) -> i32;
    pub fn nvmem_device_write(nvmem: *mut nvmem_device, offset: u32, bytes: usize, buf: *mut c_void) -> i32;
    pub fn nvmem_device_cell_read(nvmem: *mut nvmem_device, info: *mut nvmem_cell_info, buf: *mut c_void) -> isize;
    pub fn nvmem_device_cell_write(nvmem: *mut nvmem_device, info: *mut nvmem_cell_info, buf: *mut c_void) -> i32;
    pub fn nvmem_dev_name(nvmem: *mut nvmem_device) -> *const core::ffi::c_char;
    pub fn nvmem_dev_size(nvmem: *mut nvmem_device) -> usize;
    pub fn nvmem_add_cell_lookups(entries: *mut nvmem_cell_lookup, nentries: usize);
    pub fn nvmem_del_cell_lookups(entries: *mut nvmem_cell_lookup, nentries: usize);
    pub fn nvmem_register_notifier(nb: *mut notifier_block) -> i32;
    pub fn nvmem_unregister_notifier(nb: *mut notifier_block) -> i32;
    pub fn nvmem_device_find(data: *mut c_void, m: Option<unsafe extern "C" fn(*mut device, *const c_void) -> i32>) -> *mut nvmem_device;
}

#[cfg(not(feature = "CONFIG_NVMEM"))]
pub unsafe fn nvmem_cell_get(_: *mut device, _: *const core::ffi::c_char) -> *mut nvmem_cell { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_NVMEM"))]
pub unsafe fn devm_nvmem_cell_get(_: *mut device, _: *const core::ffi::c_char) -> *mut nvmem_cell { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_NVMEM"))]
pub unsafe fn nvmem_cell_put(_: *mut nvmem_cell) {}
#[cfg(not(feature = "CONFIG_NVMEM"))]
pub unsafe fn devm_nvmem_cell_put(_: *mut device, _: *mut nvmem_cell) {}
#[cfg(not(feature = "CONFIG_NVMEM"))]
pub unsafe fn nvmem_cell_read(_: *mut nvmem_cell, _: *mut usize) -> *mut c_void { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_NVMEM"))]
pub unsafe fn nvmem_cell_write(_: *mut nvmem_cell, _: *mut c_void, _: usize) -> i32 { -95 }
#[cfg(not(feature = "CONFIG_NVMEM"))]
pub unsafe fn nvmem_cell_read_u8(_: *mut device, _: *const core::ffi::c_char, _: *mut u8) -> i32 { -95 }
#[cfg(not(feature = "CONFIG_NVMEM"))]
pub unsafe fn nvmem_cell_read_u16(_: *mut device, _: *const core::ffi::c_char, _: *mut u16) -> i32 { -95 }
#[cfg(not(feature = "CONFIG_NVMEM"))]
pub unsafe fn nvmem_cell_read_u32(_: *mut device, _: *const core::ffi::c_char, _: *mut u32) -> i32 { -95 }
#[cfg(not(feature = "CONFIG_NVMEM"))]
pub unsafe fn nvmem_cell_read_u64(_: *mut device, _: *const core::ffi::c_char, _: *mut u64) -> i32 { -95 }
#[cfg(not(feature = "CONFIG_NVMEM"))]
pub unsafe fn nvmem_cell_read_variable_le_u32(_: *mut device, _: *const core::ffi::c_char, _: *mut u32) -> i32 { -95 }
#[cfg(not(feature = "CONFIG_NVMEM"))]
pub unsafe fn nvmem_cell_read_variable_le_u64(_: *mut device, _: *const core::ffi::c_char, _: *mut u64) -> i32 { -95 }
#[cfg(not(feature = "CONFIG_NVMEM"))]
pub unsafe fn nvmem_device_get(_: *mut device, _: *const core::ffi::c_char) -> *mut nvmem_device { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_NVMEM"))]
pub unsafe fn devm_nvmem_device_get(_: *mut device, _: *const core::ffi::c_char) -> *mut nvmem_device { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_NVMEM"))]
pub unsafe fn nvmem_device_put(_: *mut nvmem_device) {}
#[cfg(not(feature = "CONFIG_NVMEM"))]
pub unsafe fn devm_nvmem_device_put(_: *mut device, _: *mut nvmem_device) {}
#[cfg(not(feature = "CONFIG_NVMEM"))]
pub unsafe fn nvmem_device_cell_read(_: *mut nvmem_device, _: *mut nvmem_cell_info, _: *mut c_void) -> isize { -95 }
#[cfg(not(feature = "CONFIG_NVMEM"))]
pub unsafe fn nvmem_device_cell_write(_: *mut nvmem_device, _: *mut nvmem_cell_info, _: *mut c_void) -> i32 { -95 }
#[cfg(not(feature = "CONFIG_NVMEM"))]
pub unsafe fn nvmem_device_read(_: *mut nvmem_device, _: u32, _: usize, _: *mut c_void) -> i32 { -95 }
#[cfg(not(feature = "CONFIG_NVMEM"))]
pub unsafe fn nvmem_device_write(_: *mut nvmem_device, _: u32, _: usize, _: *mut c_void) -> i32 { -95 }
#[cfg(not(feature = "CONFIG_NVMEM"))]
pub unsafe fn nvmem_dev_name(_: *mut nvmem_device) -> *const core::ffi::c_char { core::ptr::null() }
#[cfg(not(feature = "CONFIG_NVMEM"))]
pub unsafe fn nvmem_dev_size(_: *mut nvmem_device) -> usize { 0 }
#[cfg(not(feature = "CONFIG_NVMEM"))]
pub unsafe fn nvmem_add_cell_lookups(_: *mut nvmem_cell_lookup, _: usize) {}
#[cfg(not(feature = "CONFIG_NVMEM"))]
pub unsafe fn nvmem_del_cell_lookups(_: *mut nvmem_cell_lookup, _: usize) {}
#[cfg(not(feature = "CONFIG_NVMEM"))]
pub unsafe fn nvmem_register_notifier(_: *mut notifier_block) -> i32 { -95 }
#[cfg(not(feature = "CONFIG_NVMEM"))]
pub unsafe fn nvmem_unregister_notifier(_: *mut notifier_block) -> i32 { -95 }
#[cfg(not(feature = "CONFIG_NVMEM"))]
pub unsafe fn nvmem_device_find(_: *mut c_void, _: Option<unsafe extern "C" fn(*mut device, *const c_void) -> i32>) -> *mut nvmem_device { core::ptr::null_mut() }

#[cfg(all(feature = "CONFIG_NVMEM", feature = "CONFIG_OF"))]
extern "C" {
    pub fn of_nvmem_cell_get(np: *mut device_node, id: *const core::ffi::c_char) -> *mut nvmem_cell;
    pub fn of_nvmem_device_get(np: *mut device_node, name: *const core::ffi::c_char) -> *mut nvmem_device;
}

#[cfg(not(all(feature = "CONFIG_NVMEM", feature = "CONFIG_OF")))]
pub unsafe fn of_nvmem_cell_get(_: *mut device_node, _: *const core::ffi::c_char) -> *mut nvmem_cell { core::ptr::null_mut() }
#[cfg(not(all(feature = "CONFIG_NVMEM", feature = "CONFIG_OF")))]
pub unsafe fn of_nvmem_device_get(_: *mut device_node, _: *const core::ffi::c_char) -> *mut nvmem_device { core::ptr::null_mut() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
