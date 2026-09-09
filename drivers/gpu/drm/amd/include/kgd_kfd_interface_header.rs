/*
 * Copyright 2014 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

/* Private interface between the AMD kernel graphics drivers and AMD KFD. */

use core::ffi::c_void;

pub type u8 = core::primitive::u8;
pub type u16 = core::primitive::u16;
pub type u32 = core::primitive::u32;
pub type u64 = core::primitive::u64;
pub type phys_addr_t = usize;
pub type size_t = usize;

#[repr(C)]
pub struct pci_dev { _private: [u8; 0] }
#[repr(C)]
pub struct amdgpu_device { _private: [u8; 0] }
#[repr(C)]
pub struct kfd_dev { _private: [u8; 0] }
#[repr(C)]
pub struct kgd_mem { _private: [u8; 0] }
#[repr(C)]
pub struct mm_struct { _private: [u8; 0] }
#[repr(C)]
pub enum amdgpu_ptl_fmt { }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum kfd_preempt_type {
    KFD_PREEMPT_TYPE_WAVEFRONT_DRAIN = 0,
    KFD_PREEMPT_TYPE_WAVEFRONT_RESET,
    KFD_PREEMPT_TYPE_WAVEFRONT_SAVE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_vm_fault_info {
    pub page_addr: u64,
    pub vmid: u32,
    pub mc_id: u32,
    pub status: u32,
    pub prot_valid: bool,
    pub prot_read: bool,
    pub prot_write: bool,
    pub prot_exec: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_local_mem_info {
    pub local_mem_size_private: u64,
    pub local_mem_size_public: u64,
    pub vram_width: u32,
    pub mem_clk_max: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum kgd_memory_pool {
    KGD_POOL_SYSTEM_CACHEABLE = 1,
    KGD_POOL_SYSTEM_WRITECOMBINE = 2,
    KGD_POOL_FRAMEBUFFER = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_cu_occupancy {
    pub wave_cnt: u32,
    pub doorbell_off: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum kfd_sched_policy {
    KFD_SCHED_POLICY_HWS = 0,
    KFD_SCHED_POLICY_HWS_NO_OVERSUBSCRIPTION,
    KFD_SCHED_POLICY_NO_HWS,
}

#[repr(C)]
pub struct kgd2kfd_shared_resources {
    pub compute_vmid_bitmap: u32,
    pub num_pipe_per_mec: u32,
    pub num_queue_per_pipe: u32,
    /* DECLARE_BITMAP(cp_queue_bitmap, AMDGPU_MAX_QUEUES), AMDGPU_MAX_QUEUES == 4096. */
    pub cp_queue_bitmap: [usize; 64],
    pub sdma_doorbell_idx: *mut u32,
    pub non_cp_doorbells_start: u32,
    pub non_cp_doorbells_end: u32,
    pub doorbell_physical_address: phys_addr_t,
    pub doorbell_aperture_size: size_t,
    pub doorbell_start_offset: size_t,
    pub gpuvm_size: u64,
    pub drm_render_minor: i32,
    pub enable_mes: bool,
}

#[repr(C)]
pub struct tile_config {
    pub tile_config_ptr: *mut u32,
    pub macro_tile_config_ptr: *mut u32,
    pub num_tile_configs: u32,
    pub num_macro_tile_configs: u32,
    pub gb_addr_config: u32,
    pub num_banks: u32,
    pub num_ranks: u32,
}

pub const KFD_MAX_NUM_OF_QUEUES_PER_DEVICE_DEFAULT: u32 = 4096;

#[repr(C)]
pub struct kfd2kgd_calls {
    pub program_sh_mem_settings: Option<unsafe extern "C" fn(*mut amdgpu_device, u32, u32, u32, u32, u32, u32)>,
    pub set_pasid_vmid_mapping: Option<unsafe extern "C" fn(*mut amdgpu_device, u32, u32, u32) -> i32>,
    pub init_interrupts: Option<unsafe extern "C" fn(*mut amdgpu_device, u32, u32) -> i32>,
    pub hqd_load: Option<unsafe extern "C" fn(*mut amdgpu_device, *mut c_void, u32, u32, *mut u32, u32, u32, *mut mm_struct, u32) -> i32>,
    pub hiq_mqd_load: Option<unsafe extern "C" fn(*mut amdgpu_device, *mut c_void, u32, u32, u32, u32) -> i32>,
    pub hqd_sdma_load: Option<unsafe extern "C" fn(*mut amdgpu_device, *mut c_void, *mut u32, *mut mm_struct) -> i32>,
    pub hqd_dump: Option<unsafe extern "C" fn(*mut amdgpu_device, u32, u32, *mut *mut [u32; 2], *mut u32, u32) -> i32>,
    pub hqd_sdma_dump: Option<unsafe extern "C" fn(*mut amdgpu_device, u32, u32, *mut *mut [u32; 2], *mut u32) -> i32>,
    pub hqd_is_occupied: Option<unsafe extern "C" fn(*mut amdgpu_device, u64, u32, u32, u32) -> bool>,
    pub hqd_destroy: Option<unsafe extern "C" fn(*mut amdgpu_device, *mut c_void, kfd_preempt_type, u32, u32, u32, u32) -> i32>,
    pub hqd_sdma_is_occupied: Option<unsafe extern "C" fn(*mut amdgpu_device, *mut c_void) -> bool>,
    pub hqd_sdma_destroy: Option<unsafe extern "C" fn(*mut amdgpu_device, *mut c_void, u32) -> i32>,
    pub wave_control_execute: Option<unsafe extern "C" fn(*mut amdgpu_device, u32, u32, u32) -> i32>,
    pub get_atc_vmid_pasid_mapping_info: Option<unsafe extern "C" fn(*mut amdgpu_device, u8, *mut u16) -> bool>,
    pub set_scratch_backing_va: Option<unsafe extern "C" fn(*mut amdgpu_device, u64, u32)>,
    pub set_vm_context_page_table_base: Option<unsafe extern "C" fn(*mut amdgpu_device, u32, u64)>,
    pub read_vmid_from_vmfault_reg: Option<unsafe extern "C" fn(*mut amdgpu_device) -> u32>,
    pub enable_debug_trap: Option<unsafe extern "C" fn(*mut amdgpu_device, bool, u32) -> u32>,
    pub disable_debug_trap: Option<unsafe extern "C" fn(*mut amdgpu_device, bool, u32) -> u32>,
    pub validate_trap_override_request: Option<unsafe extern "C" fn(*mut amdgpu_device, u32, *mut u32) -> i32>,
    pub set_wave_launch_trap_override: Option<unsafe extern "C" fn(*mut amdgpu_device, u32, u32, u32, u32, *mut u32, u32) -> u32>,
    pub set_wave_launch_mode: Option<unsafe extern "C" fn(*mut amdgpu_device, u8, u32) -> u32>,
    pub set_address_watch: Option<unsafe extern "C" fn(*mut amdgpu_device, u64, u32, u32, u32, u32, u32) -> u32>,
    pub clear_address_watch: Option<unsafe extern "C" fn(*mut amdgpu_device, u32) -> u32>,
    pub get_iq_wait_times: Option<unsafe extern "C" fn(*mut amdgpu_device, *mut u32, u32)>,
    pub build_dequeue_wait_counts_packet_info: Option<unsafe extern "C" fn(*mut amdgpu_device, u32, u32, u32, *mut u32, *mut u32)>,
    pub get_cu_occupancy: Option<unsafe extern "C" fn(*mut amdgpu_device, *mut kfd_cu_occupancy, *mut i32, u32)>,
    pub program_trap_handler_settings: Option<unsafe extern "C" fn(*mut amdgpu_device, u32, u64, u64, u32)>,
    pub hqd_get_pq_addr: Option<unsafe extern "C" fn(*mut amdgpu_device, u32, u32, u32) -> u64>,
    pub hqd_reset: Option<unsafe extern "C" fn(*mut amdgpu_device, u32, u32, u32, u32) -> u64>,
    pub hqd_sdma_get_doorbell: Option<unsafe extern "C" fn(*mut amdgpu_device, i32, i32) -> u32>,
    pub ptl_ctrl: Option<unsafe extern "C" fn(*mut amdgpu_device, u32, *mut u32, *mut amdgpu_ptl_fmt, *mut amdgpu_ptl_fmt) -> u32>,
    pub hqd_sdma_get_counter: Option<unsafe extern "C" fn(*mut amdgpu_device, *mut c_void, u32, *mut u64) -> i32>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
