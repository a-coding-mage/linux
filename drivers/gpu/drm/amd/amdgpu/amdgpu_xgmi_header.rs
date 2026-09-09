/*
 * Copyright 2016 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies supplied by the surrounding kernel/Rust translation.

#[repr(C)]
pub enum amdgpu_xgmi_pstate {
    AMDGPU_XGMI_PSTATE_MIN,
    AMDGPU_XGMI_PSTATE_MAX_VEGA20,
    AMDGPU_XGMI_PSTATE_UNKNOWN,
}

#[repr(C)]
pub struct amdgpu_hive_info {
    pub kobj: kobject,
    pub hive_id: u64,
    pub device_list: list_head,
    pub node: list_head,
    pub number_devices: atomic_t,
    pub hive_lock: mutex,
    pub hi_req_count: ::core::ffi::c_int,
    pub hi_req_gpu: *mut amdgpu_device,
    pub tb: task_barrier,
    pub pstate: amdgpu_xgmi_pstate,
    pub reset_domain: *mut amdgpu_reset_domain,
    pub ras_recovery: atomic_t,
    pub event_mgr: ras_event_manager,
    pub reset_on_init_work: work_struct,
    pub requested_nps_mode: atomic_t,
}

#[repr(C)]
pub struct amdgpu_pcs_ras_field {
    pub err_name: *const ::core::ffi::c_char,
    pub pcs_err_mask: u32,
    pub pcs_err_shift: u32,
}

/**
 * Bandwidth range reporting comes in two modes.
 *
 * PER_LINK - range for any xgmi link
 * PER_PEER - range of max of single xgmi link to max of multiple links based on source peer
 */
#[repr(C)]
pub enum amdgpu_xgmi_bw_mode {
    AMDGPU_XGMI_BW_MODE_PER_LINK = 0,
    AMDGPU_XGMI_BW_MODE_PER_PEER,
}

#[repr(C)]
pub enum amdgpu_xgmi_bw_unit {
    AMDGPU_XGMI_BW_UNIT_GBYTES = 0,
    AMDGPU_XGMI_BW_UNIT_MBYTES,
}

#[repr(C)]
pub struct amdgpu_xgmi_ras {
    pub ras_block: amdgpu_ras_block_object,
}

extern "C" {
    pub static mut xgmi_ras: amdgpu_xgmi_ras;
}

#[repr(C)]
pub struct amdgpu_xgmi {
    // from psp
    pub node_id: u64,
    pub hive_id: u64,
    // fixed per family
    pub node_segment_size: u64,
    // physical node (0-3)
    pub physical_node_id: ::core::ffi::c_uint,
    // number of nodes (0-4)
    pub num_physical_nodes: ::core::ffi::c_uint,
    // gpu list in the same hive
    pub head: list_head,
    pub supported: bool,
    pub ras_if: *mut ras_common_if,
    pub connected_to_cpu: bool,
    pub ras: *mut amdgpu_xgmi_ras,
    pub max_speed: u16,
    pub max_width: u8,
}

extern "C" {
    pub fn amdgpu_get_xgmi_hive(adev: *mut amdgpu_device) -> *mut amdgpu_hive_info;
    pub fn amdgpu_put_xgmi_hive(hive: *mut amdgpu_hive_info);
    pub fn amdgpu_xgmi_update_topology(hive: *mut amdgpu_hive_info, adev: *mut amdgpu_device) -> ::core::ffi::c_int;
    pub fn amdgpu_xgmi_add_device(adev: *mut amdgpu_device) -> ::core::ffi::c_int;
    pub fn amdgpu_xgmi_remove_device(adev: *mut amdgpu_device) -> ::core::ffi::c_int;
    pub fn amdgpu_xgmi_set_pstate(adev: *mut amdgpu_device, pstate: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn amdgpu_xgmi_get_hops_count(adev: *mut amdgpu_device, peer_adev: *mut amdgpu_device) -> ::core::ffi::c_int;
    pub fn amdgpu_xgmi_get_bandwidth(adev: *mut amdgpu_device, peer_adev: *mut amdgpu_device, bw_mode: amdgpu_xgmi_bw_mode, bw_unit: amdgpu_xgmi_bw_unit, min_bw: *mut u32, max_bw: *mut u32) -> ::core::ffi::c_int;
    pub fn amdgpu_xgmi_get_is_sharing_enabled(adev: *mut amdgpu_device, peer_adev: *mut amdgpu_device) -> bool;
    pub fn amdgpu_xgmi_get_relative_phy_addr(adev: *mut amdgpu_device, addr: u64) -> u64;
    pub fn amdgpu_xgmi_same_hive(adev: *mut amdgpu_device, bo_adev: *mut amdgpu_device) -> bool;
    pub fn amdgpu_xgmi_ras_sw_init(adev: *mut amdgpu_device) -> ::core::ffi::c_int;
    pub fn amdgpu_xgmi_reset_on_init(adev: *mut amdgpu_device) -> ::core::ffi::c_int;
    pub fn amdgpu_xgmi_request_nps_change(adev: *mut amdgpu_device, hive: *mut amdgpu_hive_info, req_nps_mode: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn amdgpu_get_xgmi_link_status(adev: *mut amdgpu_device, global_link_num: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn amdgpu_xgmi_get_ext_link(adev: *mut amdgpu_device, link_num: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn amdgpu_xgmi_early_init(adev: *mut amdgpu_device);
    pub fn amdgpu_xgmi_get_max_bandwidth(adev: *mut amdgpu_device) -> u32;
    pub fn amgpu_xgmi_set_max_speed_width(adev: *mut amdgpu_device, max_speed: u16, max_width: u8);
}

// Cleanup macro for use with __free(xgmi_put_hive):
// DEFINE_FREE(xgmi_put_hive, struct amdgpu_hive_info *, if (_T) amdgpu_put_xgmi_hive(_T))

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
