/* SPDX-License-Identifier: GPL-2.0 OR MIT */
/*
 * Copyright 2014-2022 Advanced Micro Devices, Inc.
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

// Dependency declarations are supplied by kfd_priv.h and related bindings.

pub const KFD_MAX_NUM_SE: usize = 8;
pub const KFD_MAX_NUM_SH_PER_SE: usize = 2;

extern "C" {
    pub static mut pipe_priority_map: *mut ::core::ffi::c_int;
}

#[repr(C)]
pub struct mqd_manager {
    pub allocate_mqd: Option<unsafe extern "C" fn(*mut mqd_manager, *mut queue_properties) -> *mut kfd_mem_obj>,
    pub init_mqd: Option<unsafe extern "C" fn(*mut mqd_manager, *mut *mut ::core::ffi::c_void, *mut kfd_mem_obj, *mut u64, *mut queue_properties)>,
    pub load_mqd: Option<unsafe extern "C" fn(*mut mqd_manager, *mut ::core::ffi::c_void, u32, u32, *mut queue_properties, *mut mm_struct) -> ::core::ffi::c_int>,
    pub update_mqd: Option<unsafe extern "C" fn(*mut mqd_manager, *mut ::core::ffi::c_void, *mut queue_properties, *mut mqd_update_info)>,
    pub destroy_mqd: Option<unsafe extern "C" fn(*mut mqd_manager, *mut ::core::ffi::c_void, kfd_preempt_type, ::core::ffi::c_uint, u32, u32) -> ::core::ffi::c_int>,
    pub free_mqd: Option<unsafe extern "C" fn(*mut mqd_manager, *mut ::core::ffi::c_void, *mut kfd_mem_obj)>,
    pub is_occupied: Option<unsafe extern "C" fn(*mut mqd_manager, *mut ::core::ffi::c_void, u64, u32, u32) -> bool>,
    pub get_wave_state: Option<unsafe extern "C" fn(*mut mqd_manager, *mut ::core::ffi::c_void, *mut queue_properties, *mut ::core::ffi::c_void, *mut u32, *mut u32) -> ::core::ffi::c_int>,
    pub get_checkpoint_info: Option<unsafe extern "C" fn(*mut mqd_manager, *mut ::core::ffi::c_void, *mut u32) -> ::core::ffi::c_int>,
    pub checkpoint_mqd: Option<unsafe extern "C" fn(*mut mqd_manager, *mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut ::core::ffi::c_void)>,
    pub restore_mqd: Option<unsafe extern "C" fn(*mut mqd_manager, *mut *mut ::core::ffi::c_void, *mut kfd_mem_obj, *mut u64, *mut queue_properties, *const ::core::ffi::c_void, *const ::core::ffi::c_void, u32)>,
    /* Patch the MQD's cached self GPU address after the MQD BO has moved
     * (e.g. repinned to a new VRAM location on hibernation resume). The MQD
     * contents are otherwise preserved.
     */
    pub update_mqd_gpu_addr: Option<unsafe extern "C" fn(*mut mqd_manager, *mut ::core::ffi::c_void, *mut kfd_mem_obj, *mut queue_properties)>,
    #[cfg(CONFIG_DEBUG_FS)]
    pub debugfs_show_mqd: Option<unsafe extern "C" fn(*mut seq_file, *mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub check_preemption_failed: Option<unsafe extern "C" fn(*mut mqd_manager, *mut ::core::ffi::c_void) -> bool>,
    pub mqd_stride: Option<unsafe extern "C" fn(*mut mqd_manager, *mut queue_properties) -> u64>,
    pub mqd_mutex: mutex,
    pub dev: *mut kfd_node,
    pub mqd_size: u32,
    pub ctl_stack_size: u32,
}

#[repr(C)]
pub struct mqd_user_context_save_area_header {
    pub control_stack_offset: u32,
    pub control_stack_size: u32,
    pub wave_state_offset: u32,
    pub wave_state_size: u32,
}

extern "C" {
    pub fn allocate_hiq_mqd(mm: *mut mqd_manager, q: *mut queue_properties) -> *mut kfd_mem_obj;
    pub fn allocate_sdma_mqd(mm: *mut mqd_manager, q: *mut queue_properties) -> *mut kfd_mem_obj;
    pub fn free_mqd_hiq_sdma(mm: *mut mqd_manager, mqd: *mut ::core::ffi::c_void, mqd_mem_obj: *mut kfd_mem_obj);
    pub fn mqd_symmetrically_map_cu_mask(mm: *mut mqd_manager, cu_mask: *const u32, cu_mask_count: u32, se_mask: *mut u32, inst: u32);
    pub fn kfd_hiq_load_mqd_kiq(mm: *mut mqd_manager, mqd: *mut ::core::ffi::c_void, pipe_id: u32, queue_id: u32, p: *mut queue_properties, mms: *mut mm_struct) -> ::core::ffi::c_int;
    pub fn kfd_destroy_mqd_cp(mm: *mut mqd_manager, mqd: *mut ::core::ffi::c_void, type_: kfd_preempt_type, timeout: ::core::ffi::c_uint, pipe_id: u32, queue_id: u32) -> ::core::ffi::c_int;
    pub fn kfd_free_mqd_cp(mm: *mut mqd_manager, mqd: *mut ::core::ffi::c_void, mqd_mem_obj: *mut kfd_mem_obj);
    pub fn kfd_is_occupied_cp(mm: *mut mqd_manager, mqd: *mut ::core::ffi::c_void, queue_address: u64, pipe_id: u32, queue_id: u32) -> bool;
    pub fn kfd_load_mqd_sdma(mm: *mut mqd_manager, mqd: *mut ::core::ffi::c_void, pipe_id: u32, queue_id: u32, p: *mut queue_properties, mms: *mut mm_struct) -> ::core::ffi::c_int;
    pub fn kfd_destroy_mqd_sdma(mm: *mut mqd_manager, mqd: *mut ::core::ffi::c_void, type_: kfd_preempt_type, timeout: ::core::ffi::c_uint, pipe_id: u32, queue_id: u32) -> ::core::ffi::c_int;
    pub fn kfd_is_occupied_sdma(mm: *mut mqd_manager, mqd: *mut ::core::ffi::c_void, queue_address: u64, pipe_id: u32, queue_id: u32) -> bool;
    pub fn kfd_get_hiq_xcc_mqd(dev: *mut kfd_node, mqd_mem_obj: *mut kfd_mem_obj, virtual_xcc_id: u32);
    pub fn kfd_hiq_mqd_stride(dev: *mut kfd_node) -> u64;
    pub fn kfd_mqd_stride(mm: *mut mqd_manager, q: *mut queue_properties) -> u64;
    pub fn kfd_check_hiq_mqd_doorbell_id(node: *mut kfd_node, doorbell_id: u32, inst: u32) -> bool;
    pub fn mqd_on_vram(adev: *mut amdgpu_device) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
