/*
 * Copyright 2022 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies: linux/pci.h, linux/xarray.h, and amdgpu_ctx.h.

pub const MAX_XCP: usize = 8;
pub const AMDGPU_XCP_MODE_NONE: i32 = -1;
pub const AMDGPU_XCP_MODE_TRANS: i32 = -2;
pub const AMDGPU_XCP_FL_NONE: u32 = 0;
pub const AMDGPU_XCP_FL_LOCKED: u32 = 1 << 0;
pub const AMDGPU_XCP_NO_PARTITION: u32 = !0;
pub const AMDGPU_XCP_OPS_KFD: u32 = 1 << 0;

pub const fn xcp_inst_mask(num_inst: u32, xcp_id: u32) -> u32 {
    if num_inst != 0 { ((1u32 << num_inst) - 1) << (xcp_id * num_inst) } else { 0 }
}

pub enum amdgpu_fpriv {}
pub enum amdgpu_xcp_mgr {}
pub enum amdgpu_device {}
pub enum drm_device {}
pub enum drm_driver {}
pub enum drm_vma_offset_manager {}
pub enum kobject {}
pub enum mutex {}
pub enum atomic_t {}
pub enum amdgpu_sched {}
pub enum amdgpu_ctx_entity {}
pub enum drm_file {}
pub enum drm_gpu_scheduler {}
pub enum pci_device_id {}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum AMDGPU_XCP_IP_BLOCK { AMDGPU_XCP_GFXHUB, AMDGPU_XCP_GFX, AMDGPU_XCP_SDMA, AMDGPU_XCP_VCN, AMDGPU_XCP_MAX_BLOCKS }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum AMDGPU_XCP_STATE { AMDGPU_XCP_PREPARE_SUSPEND, AMDGPU_XCP_SUSPEND, AMDGPU_XCP_PREPARE_RESUME, AMDGPU_XCP_RESUME }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum amdgpu_xcp_res_id { AMDGPU_XCP_RES_XCC, AMDGPU_XCP_RES_DMA, AMDGPU_XCP_RES_DEC, AMDGPU_XCP_RES_JPEG, AMDGPU_XCP_RES_MAX }

#[repr(C)]
pub struct amdgpu_xcp_res_details { pub id: amdgpu_xcp_res_id, pub num_inst: u8, pub num_shared: u8, pub kobj: kobject }

#[repr(C)]
pub struct amdgpu_xcp_cfg {
    pub mode: u8,
    pub xcp_res: [amdgpu_xcp_res_details; 5],
    pub num_res: u8,
    pub xcp_mgr: *mut amdgpu_xcp_mgr,
    pub kobj: kobject,
    pub compatible_nps_modes: u16,
}

#[repr(C)]
pub struct amdgpu_xcp_ip_funcs {
    pub prepare_suspend: Option<unsafe extern "C" fn(*mut core::ffi::c_void, u32) -> i32>,
    pub suspend: Option<unsafe extern "C" fn(*mut core::ffi::c_void, u32) -> i32>,
    pub prepare_resume: Option<unsafe extern "C" fn(*mut core::ffi::c_void, u32) -> i32>,
    pub resume: Option<unsafe extern "C" fn(*mut core::ffi::c_void, u32) -> i32>,
}

#[repr(C)]
pub struct amdgpu_xcp_ip { pub ip_funcs: *mut amdgpu_xcp_ip_funcs, pub inst_mask: u32, pub ip_id: AMDGPU_XCP_IP_BLOCK, pub valid: bool }

#[repr(C)]
pub struct amdgpu_xcp {
    pub ip: [amdgpu_xcp_ip; 5], pub id: u8, pub mem_id: u8, pub valid: bool, pub ref_cnt: atomic_t,
    pub ddev: *mut drm_device, pub rdev: *mut drm_device, pub pdev: *mut drm_device, pub driver: *mut drm_driver,
    pub vma_offset_manager: *mut drm_vma_offset_manager, pub gpu_sched: [[amdgpu_sched; 8]; 16],
    pub xcp_mgr: *mut amdgpu_xcp_mgr, pub kobj: kobject, pub unique_id: u64,
}

#[repr(C)]
pub struct amdgpu_xcp_mgr {
    pub adev: *mut amdgpu_device, pub xcp_lock: mutex, pub funcs: *mut amdgpu_xcp_mgr_funcs,
    pub xcp: [amdgpu_xcp; MAX_XCP], pub num_xcps: u8, pub mode: i8,
    pub num_xcp_per_mem_partition: u32, pub xcp_cfg: *mut amdgpu_xcp_cfg,
    pub supp_xcp_modes: u32, pub avail_xcp_modes: u32, pub mem_alloc_mode: u32,
}

#[repr(C)]
pub struct amdgpu_xcp_mgr_funcs {
    pub switch_partition_mode: Option<unsafe extern "C" fn(*mut amdgpu_xcp_mgr, i32, *mut i32) -> i32>,
    pub query_partition_mode: Option<unsafe extern "C" fn(*mut amdgpu_xcp_mgr) -> i32>,
    pub get_ip_details: Option<unsafe extern "C" fn(*mut amdgpu_xcp_mgr, i32, AMDGPU_XCP_IP_BLOCK, *mut amdgpu_xcp_ip) -> i32>,
    pub get_xcp_mem_id: Option<unsafe extern "C" fn(*mut amdgpu_xcp_mgr, *mut amdgpu_xcp, *mut u8) -> i32>,
    pub get_xcp_res_info: Option<unsafe extern "C" fn(*mut amdgpu_xcp_mgr, i32, *mut amdgpu_xcp_cfg) -> i32>,
    pub prepare_suspend: Option<unsafe extern "C" fn(*mut amdgpu_xcp_mgr, i32) -> i32>, pub suspend: Option<unsafe extern "C" fn(*mut amdgpu_xcp_mgr, i32) -> i32>,
    pub prepare_resume: Option<unsafe extern "C" fn(*mut amdgpu_xcp_mgr, i32) -> i32>, pub resume: Option<unsafe extern "C" fn(*mut amdgpu_xcp_mgr, i32) -> i32>,
}

extern "C" {
    pub fn amdgpu_xcp_prepare_suspend(xcp_mgr: *mut amdgpu_xcp_mgr, xcp_id: i32) -> i32;
    pub fn amdgpu_xcp_suspend(xcp_mgr: *mut amdgpu_xcp_mgr, xcp_id: i32) -> i32;
    pub fn amdgpu_xcp_prepare_resume(xcp_mgr: *mut amdgpu_xcp_mgr, xcp_id: i32) -> i32;
    pub fn amdgpu_xcp_resume(xcp_mgr: *mut amdgpu_xcp_mgr, xcp_id: i32) -> i32;
    pub fn amdgpu_xcp_mgr_init(adev: *mut amdgpu_device, init_mode: i32, init_xcps: i32, xcp_funcs: *mut amdgpu_xcp_mgr_funcs) -> i32;
    pub fn amdgpu_xcp_init(xcp_mgr: *mut amdgpu_xcp_mgr, num_xcps: i32, mode: i32) -> i32;
    pub fn amdgpu_xcp_query_partition_mode(xcp_mgr: *mut amdgpu_xcp_mgr, flags: u32) -> i32;
    pub fn amdgpu_xcp_switch_partition_mode(xcp_mgr: *mut amdgpu_xcp_mgr, mode: i32) -> i32;
    pub fn amdgpu_xcp_restore_partition_mode(xcp_mgr: *mut amdgpu_xcp_mgr) -> i32;
    pub fn amdgpu_xcp_get_partition(xcp_mgr: *mut amdgpu_xcp_mgr, ip: AMDGPU_XCP_IP_BLOCK, instance: i32) -> i32;
    pub fn amdgpu_xcp_get_inst_details(xcp: *mut amdgpu_xcp, ip: AMDGPU_XCP_IP_BLOCK, inst_mask: *mut u32) -> i32;
    pub fn amdgpu_xcp_dev_register(adev: *mut amdgpu_device, ent: *const pci_device_id) -> i32;
    pub fn amdgpu_xcp_dev_unplug(adev: *mut amdgpu_device);
    pub fn amdgpu_xcp_open_device(adev: *mut amdgpu_device, fpriv: *mut amdgpu_fpriv, file_priv: *mut drm_file) -> i32;
    pub fn amdgpu_xcp_release_sched(adev: *mut amdgpu_device, entity: *mut amdgpu_ctx_entity);
    pub fn amdgpu_xcp_select_scheds(adev: *mut amdgpu_device, hw_ip: u32, hw_prio: u32, fpriv: *mut amdgpu_fpriv, num_scheds: *mut u32, scheds: *mut *mut *mut drm_gpu_scheduler) -> i32;
    pub fn amdgpu_xcp_update_supported_modes(xcp_mgr: *mut amdgpu_xcp_mgr);
    pub fn amdgpu_xcp_update_partition_sched_list(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_xcp_pre_partition_switch(xcp_mgr: *mut amdgpu_xcp_mgr, flags: u32) -> i32;
    pub fn amdgpu_xcp_post_partition_switch(xcp_mgr: *mut amdgpu_xcp_mgr, flags: u32) -> i32;
    pub fn amdgpu_xcp_sysfs_init(adev: *mut amdgpu_device);
    pub fn amdgpu_xcp_sysfs_fini(adev: *mut amdgpu_device);
}

#[inline]
pub unsafe fn amdgpu_xcp_get_num_xcp(xcp_mgr: *mut amdgpu_xcp_mgr) -> u8 { if xcp_mgr.is_null() { 1 } else { (*xcp_mgr).num_xcps } }

#[inline]
pub unsafe fn amdgpu_get_next_xcp(xcp_mgr: *mut amdgpu_xcp_mgr, from: *mut i32) -> *mut amdgpu_xcp {
    if xcp_mgr.is_null() { return core::ptr::null_mut(); }
    while *from < MAX_XCP as i32 { if (*xcp_mgr).xcp[*from as usize].valid { return &mut (*xcp_mgr).xcp[*from as usize]; } *from += 1; }
    core::ptr::null_mut()
}

// C iteration macro: for_each_xcp(xcp_mgr, xcp, i)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
