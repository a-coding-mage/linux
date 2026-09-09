/* SPDX-License-Identifier: GPL-2.0 */
/*
 * FPGA Framework
 *
 *  Copyright (C) 2013-2016 Altera Corporation
 *  Copyright (C) 2017 Intel Corporation
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub enum fpga_mgr_states {
    /* default FPGA states */
    FPGA_MGR_STATE_UNKNOWN,
    FPGA_MGR_STATE_POWER_OFF,
    FPGA_MGR_STATE_POWER_UP,
    FPGA_MGR_STATE_RESET,

    /* getting an image for loading */
    FPGA_MGR_STATE_FIRMWARE_REQ,
    FPGA_MGR_STATE_FIRMWARE_REQ_ERR,

    /* write sequence: parse header, init, write, complete */
    FPGA_MGR_STATE_PARSE_HEADER,
    FPGA_MGR_STATE_PARSE_HEADER_ERR,
    FPGA_MGR_STATE_WRITE_INIT,
    FPGA_MGR_STATE_WRITE_INIT_ERR,
    FPGA_MGR_STATE_WRITE,
    FPGA_MGR_STATE_WRITE_ERR,
    FPGA_MGR_STATE_WRITE_COMPLETE,
    FPGA_MGR_STATE_WRITE_COMPLETE_ERR,

    /* fpga is programmed and operating */
    FPGA_MGR_STATE_OPERATING,
}

/* FPGA Manager flags */
pub const FPGA_MGR_PARTIAL_RECONFIG: u32 = 1 << 0;
pub const FPGA_MGR_EXTERNAL_CONFIG: u32 = 1 << 1;
pub const FPGA_MGR_ENCRYPTED_BITSTREAM: u32 = 1 << 2;
pub const FPGA_MGR_BITSTREAM_LSB_FIRST: u32 = 1 << 3;
pub const FPGA_MGR_COMPRESSED_BITSTREAM: u32 = 1 << 4;

#[repr(C)]
pub struct fpga_image_info {
    pub flags: u32,
    pub enable_timeout_us: u32,
    pub disable_timeout_us: u32,
    pub config_complete_timeout_us: u32,
    pub firmware_name: *mut core::ffi::c_char,
    pub sgt: *mut sg_table,
    pub buf: *const core::ffi::c_char,
    pub count: usize,
    pub header_size: usize,
    pub data_size: usize,
    pub region_id: i32,
    pub dev: *mut device,
    // Present only when CONFIG_OF is enabled in the C build.
    #[cfg(CONFIG_OF)]
    pub overlay: *mut device_node,
}

#[repr(C)]
pub struct fpga_compat_id {
    pub id_h: u64,
    pub id_l: u64,
}

#[repr(C)]
pub struct fpga_manager_info {
    pub name: *const core::ffi::c_char,
    pub compat_id: *mut fpga_compat_id,
    pub mops: *const fpga_manager_ops,
    pub priv_: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct fpga_manager_ops {
    pub initial_header_size: usize,
    pub skip_header: bool,
    pub state: Option<unsafe extern "C" fn(*mut fpga_manager) -> fpga_mgr_states>,
    pub status: Option<unsafe extern "C" fn(*mut fpga_manager) -> u64>,
    pub parse_header: Option<unsafe extern "C" fn(*mut fpga_manager, *mut fpga_image_info, *const core::ffi::c_char, usize) -> i32>,
    pub write_init: Option<unsafe extern "C" fn(*mut fpga_manager, *mut fpga_image_info, *const core::ffi::c_char, usize) -> i32>,
    pub write: Option<unsafe extern "C" fn(*mut fpga_manager, *const core::ffi::c_char, usize) -> i32>,
    pub write_sg: Option<unsafe extern "C" fn(*mut fpga_manager, *mut sg_table) -> i32>,
    pub write_complete: Option<unsafe extern "C" fn(*mut fpga_manager, *mut fpga_image_info) -> i32>,
    pub fpga_remove: Option<unsafe extern "C" fn(*mut fpga_manager)>,
    pub groups: *const *const attribute_group,
}

pub const FPGA_MGR_STATUS_OPERATION_ERR: u32 = 1 << 0;
pub const FPGA_MGR_STATUS_CRC_ERR: u32 = 1 << 1;
pub const FPGA_MGR_STATUS_INCOMPATIBLE_IMAGE_ERR: u32 = 1 << 2;
pub const FPGA_MGR_STATUS_IP_PROTOCOL_ERR: u32 = 1 << 3;
pub const FPGA_MGR_STATUS_FIFO_OVERFLOW_ERR: u32 = 1 << 4;

#[repr(C)]
pub struct fpga_manager {
    pub name: *const core::ffi::c_char,
    pub dev: device,
    pub ref_mutex: mutex,
    pub state: fpga_mgr_states,
    pub compat_id: *mut fpga_compat_id,
    pub mops: *const fpga_manager_ops,
    pub mops_owner: *mut module,
    pub priv_: *mut core::ffi::c_void,
}

/* Equivalent to container_of(d, struct fpga_manager, dev). */
#[inline]
pub unsafe fn to_fpga_manager(d: *mut device) -> *mut fpga_manager {
    (d as *mut u8).sub(core::mem::offset_of!(fpga_manager, dev)) as *mut fpga_manager
}

extern "C" {
    pub fn fpga_image_info_alloc(dev: *mut device) -> *mut fpga_image_info;
    pub fn fpga_image_info_free(info: *mut fpga_image_info);
    pub fn fpga_mgr_load(mgr: *mut fpga_manager, info: *mut fpga_image_info) -> i32;
    pub fn fpga_mgr_lock(mgr: *mut fpga_manager) -> i32;
    pub fn fpga_mgr_unlock(mgr: *mut fpga_manager);
    pub fn of_fpga_mgr_get(node: *mut device_node) -> *mut fpga_manager;
    pub fn fpga_mgr_get(dev: *mut device) -> *mut fpga_manager;
    pub fn fpga_mgr_put(mgr: *mut fpga_manager);
    pub fn __fpga_mgr_register_full(parent: *mut device, info: *const fpga_manager_info, owner: *mut module) -> *mut fpga_manager;
}

// fpga_mgr_register_full(parent, info) passes THIS_MODULE as the owner.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
