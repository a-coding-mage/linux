/* SPDX-License-Identifier: GPL-2.0 */
/* NVMEM framework provider declarations translated from C. */

use core::ffi::{c_char, c_int, c_uint, c_uchar, c_void};

#[repr(C)]
pub struct nvmem_device;
#[repr(C)]
pub struct device;
#[repr(C)]
pub struct device_node;
#[repr(C)]
pub struct module;
#[repr(C)]
pub struct nvmem_layout;
#[repr(C)]
pub struct device_driver;

pub type nvmem_reg_read_t = unsafe extern "C" fn(
    priv_: *mut c_void,
    offset: c_uint,
    val: *mut c_void,
    bytes: usize,
) -> c_int;
pub type nvmem_reg_write_t = unsafe extern "C" fn(
    priv_: *mut c_void,
    offset: c_uint,
    val: *mut c_void,
    bytes: usize,
) -> c_int;
/* Used for vendor specific post processing of cell data. */
pub type nvmem_cell_post_process_t = unsafe extern "C" fn(
    priv_: *mut c_void,
    id: *const c_char,
    index: c_int,
    offset: c_uint,
    buf: *mut c_void,
    bytes: usize,
) -> c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum nvmem_type {
    NVMEM_TYPE_UNKNOWN = 0,
    NVMEM_TYPE_EEPROM,
    NVMEM_TYPE_OTP,
    NVMEM_TYPE_BATTERY_BACKED,
    NVMEM_TYPE_FRAM,
}

pub const NVMEM_DEVID_NONE: c_int = -1;
pub const NVMEM_DEVID_AUTO: c_int = -2;

#[repr(C)]
pub struct nvmem_keepout {
    pub start: c_uint,
    pub end: c_uint,
    pub value: c_uchar,
}

#[repr(C)]
pub struct nvmem_cell_info {
    pub name: *const c_char,
    pub offset: c_uint,
    pub raw_len: usize,
    pub bytes: c_uint,
    pub bit_offset: c_uint,
    pub nbits: c_uint,
    pub np: *mut device_node,
    pub read_post_process: Option<nvmem_cell_post_process_t>,
    pub priv_: *mut c_void,
}

#[repr(C)]
pub struct nvmem_config {
    pub dev: *mut device,
    pub name: *const c_char,
    pub id: c_int,
    pub owner: *mut module,
    pub cells: *const nvmem_cell_info,
    pub ncells: c_int,
    pub add_legacy_fixed_of_cells: bool,
    pub fixup_dt_cell_info: Option<unsafe extern "C" fn(*mut nvmem_device, *mut nvmem_cell_info)>,
    pub keepout: *const nvmem_keepout,
    pub nkeepout: c_uint,
    pub type_: nvmem_type,
    pub read_only: bool,
    pub root_only: bool,
    pub ignore_wp: bool,
    pub layout: *mut nvmem_layout,
    pub of_node: *mut device_node,
    pub reg_read: Option<nvmem_reg_read_t>,
    pub reg_write: Option<nvmem_reg_write_t>,
    pub size: c_int,
    pub word_size: c_int,
    pub stride: c_int,
    pub priv_: *mut c_void,
    pub compat: bool,
    pub base_dev: *mut device,
}

#[repr(C)]
pub struct nvmem_layout {
    pub dev: device,
    pub nvmem: *mut nvmem_device,
    pub add_cells: Option<unsafe extern "C" fn(*mut nvmem_layout) -> c_int>,
}

#[repr(C)]
pub struct nvmem_layout_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut nvmem_layout) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut nvmem_layout)>,
}

/* CONFIG_NVMEM-dependent declarations and inline fallbacks. */
extern "C" {
    pub fn nvmem_register(cfg: *const nvmem_config) -> *mut nvmem_device;
    pub fn nvmem_unregister(nvmem: *mut nvmem_device);
    pub fn devm_nvmem_register(dev: *mut device, cfg: *const nvmem_config) -> *mut nvmem_device;
    pub fn nvmem_add_one_cell(nvmem: *mut nvmem_device, info: *const nvmem_cell_info) -> c_int;
    pub fn nvmem_layout_register(layout: *mut nvmem_layout) -> c_int;
    pub fn nvmem_layout_unregister(layout: *mut nvmem_layout);
    pub fn __nvmem_layout_driver_register(drv: *mut nvmem_layout_driver, owner: *mut module) -> c_int;
    pub fn nvmem_layout_driver_unregister(drv: *mut nvmem_layout_driver);
    pub fn nvmem_add_cells_from_dt(nvmem: *mut nvmem_device, np: *mut device_node) -> c_int;
}

/* When CONFIG_NVMEM is disabled, the C header returns -EOPNOTSUPP (or NULL). */
#[inline]
pub unsafe fn nvmem_register_disabled(_c: *const nvmem_config) -> *mut nvmem_device { core::ptr::null_mut() }
#[inline]
pub unsafe fn devm_nvmem_register_disabled(_dev: *mut device, _c: *const nvmem_config) -> *mut nvmem_device { core::ptr::null_mut() }

/* CONFIG_NVMEM && CONFIG_OF-dependent declaration. */
extern "C" {
    pub fn of_nvmem_layout_get_container(nvmem: *mut nvmem_device) -> *mut device_node;
}

#[inline]
pub unsafe fn of_nvmem_layout_get_container_disabled(_nvmem: *mut nvmem_device) -> *mut device_node {
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
