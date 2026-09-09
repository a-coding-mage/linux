/*
 * Defines, structures, APIs for edac_device
 *
 * C header translated to Rust. The included Linux types and symbols are
 * supplied by other translation units.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

/* Linux/kernel types supplied by other files. */
pub use crate::{
    attribute, bus_type, delayed_work, device, edac_device_ctl_info,
    kobject, list_head, module, ssize_t,
};

pub const EDAC_DEVICE_NAME_LEN: usize = crate::EDAC_DEVICE_NAME_LEN as usize;

#[repr(C)]
pub struct edac_device_counter {
    pub ue_count: u32,
    pub ce_count: u32,
}

/* forward reference */
/* struct edac_device_ctl_info; */
/* struct edac_device_block; */

#[repr(C)]
pub struct edac_dev_sysfs_attribute {
    pub attr: attribute,
    pub show: Option<unsafe extern "C" fn(*mut edac_device_ctl_info, *mut c_char) -> ssize_t>,
    pub store: Option<unsafe extern "C" fn(*mut edac_device_ctl_info, *const c_char, usize) -> ssize_t>,
}

#[repr(C)]
pub struct edac_dev_sysfs_block_attribute {
    pub attr: attribute,
    pub show: Option<unsafe extern "C" fn(*mut kobject, *mut attribute, *mut c_char) -> ssize_t>,
}

#[repr(C)]
pub struct edac_device_block {
    pub instance: *mut edac_device_instance,
    pub name: [c_char; EDAC_DEVICE_NAME_LEN + 1],
    pub counters: edac_device_counter,
    pub nr_attribs: c_int,
    pub block_attributes: *mut edac_dev_sysfs_block_attribute,
    pub kobj: kobject,
}

#[repr(C)]
pub struct edac_device_instance {
    pub ctl: *mut edac_device_ctl_info,
    pub name: [c_char; EDAC_DEVICE_NAME_LEN + 4],
    pub counters: edac_device_counter,
    pub nr_blocks: u32,
    pub blocks: *mut edac_device_block,
    pub kobj: kobject,
}

#[repr(C)]
pub struct edac_device_ctl_info {
    pub link: list_head,
    pub owner: *mut module,
    pub dev_idx: c_int,
    pub log_ue: c_int,
    pub log_ce: c_int,
    pub panic_on_ue: c_int,
    pub poll_msec: c_uint,
    pub delay: c_ulong,
    pub sysfs_attributes: *const edac_dev_sysfs_attribute,
    pub edac_subsys: *const bus_type,
    pub op_state: c_int,
    pub work: delayed_work,
    pub edac_check: Option<unsafe extern "C" fn(*mut edac_device_ctl_info)>,
    pub dev: *mut device,
    pub mod_name: *const c_char,
    pub ctl_name: *const c_char,
    pub dev_name: *const c_char,
    pub pvt_info: *mut c_void,
    pub start_time: c_ulong,
    pub name: [c_char; EDAC_DEVICE_NAME_LEN + 1],
    pub nr_instances: u32,
    pub instances: *mut edac_device_instance,
    pub blocks: *mut edac_device_block,
    pub counters: edac_device_counter,
    pub kobj: kobject,
}

/* To get from the instance's workqueue to the beginning of the ctl structure. */
/* #define to_edac_mem_ctl_work(w) container_of(w, struct mem_ctl_info, work) */
/* #define to_edac_device_ctl_work(w) container_of(w, struct edac_device_ctl_info, work) */

extern "C" {
    pub fn edac_device_alloc_ctl_info(
        sizeof_private: c_uint,
        edac_device_name: *mut c_char,
        nr_instances: c_uint,
        edac_block_name: *mut c_char,
        nr_blocks: c_uint,
        offset_value: c_uint,
        device_index: c_int,
    ) -> *mut edac_device_ctl_info;

    pub fn edac_device_free_ctl_info(ctl_info: *mut edac_device_ctl_info);
    pub fn edac_device_add_device(edac_dev: *mut edac_device_ctl_info) -> c_int;
    pub fn edac_device_del_device(dev: *mut device) -> *mut edac_device_ctl_info;
    pub fn edac_device_handle_ce_count(
        edac_dev: *mut edac_device_ctl_info, count: c_uint,
        inst_nr: c_int, block_nr: c_int, msg: *const c_char,
    );
    pub fn edac_device_handle_ue_count(
        edac_dev: *mut edac_device_ctl_info, count: c_uint,
        inst_nr: c_int, block_nr: c_int, msg: *const c_char,
    );
    pub fn edac_device_alloc_index() -> c_int;
    pub static mut edac_layer_name: *const *const c_char;
}

pub const BLOCK_OFFSET_VALUE_OFF: c_uint = c_uint::MAX;

pub unsafe inline fn edac_device_handle_ce(
    edac_dev: *mut edac_device_ctl_info, inst_nr: c_int,
    block_nr: c_int, msg: *const c_char,
) {
    edac_device_handle_ce_count(edac_dev, 1, inst_nr, block_nr, msg);
}

pub unsafe inline fn edac_device_handle_ue(
    edac_dev: *mut edac_device_ctl_info, inst_nr: c_int,
    block_nr: c_int, msg: *const c_char,
) {
    edac_device_handle_ue_count(edac_dev, 1, inst_nr, block_nr, msg);
}

extern "C" {
    fn kfree(ptr: *mut c_void);
}

pub unsafe inline fn __edac_device_free_ctl_info(ci: *mut edac_device_ctl_info) {
    if !ci.is_null() {
        kfree((*ci).pvt_info);
        kfree((*ci).blocks.cast());
        kfree((*ci).instances.cast());
        kfree(ci.cast());
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
