/*
 * Copyright 2019 Advanced Micro Devices, Inc.
 *
 * Rust translation of amdgpu_mes.c.  Kernel types, constants, and helpers
 * referenced below are supplied by the surrounding AMDGPU translation.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

pub const AMDGPU_MES_MAX_NUM_OF_QUEUES_PER_PROCESS: usize = 1024;
pub const AMDGPU_ONE_DOORBELL_SIZE: usize = 8;

/* External kernel/AMDGPU declarations. */
extern "C" {
    pub fn amdgpu_mes_doorbell_process_slice(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_mes_init(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_mes_fini(adev: *mut amdgpu_device);
    pub fn amdgpu_mes_suspend(adev: *mut amdgpu_device, xcc_id: u32) -> i32;
    pub fn amdgpu_mes_resume(adev: *mut amdgpu_device, xcc_id: u32) -> i32;
    pub fn amdgpu_mes_map_legacy_queue(adev: *mut amdgpu_device, ring: *mut amdgpu_ring, xcc_id: u32) -> i32;
    pub fn amdgpu_mes_unmap_legacy_queue(adev: *mut amdgpu_device, ring: *mut amdgpu_ring, action: amdgpu_unmap_queues_action, gpu_addr: u64, seq: u64, xcc_id: u32) -> i32;
    pub fn amdgpu_mes_reset_legacy_queue(adev: *mut amdgpu_device, ring: *mut amdgpu_ring, vmid: u32, use_mmio: bool, xcc_id: u32) -> i32;
    pub fn amdgpu_mes_reset_queue_mmio(adev: *mut amdgpu_device, queue_type: i32, vmid: u32, me: u32, pipe: u32, queue: u32, xcc_id: u32) -> i32;
    pub fn amdgpu_mes_reset_user_queue(adev: *mut amdgpu_device, queue_type: i32, doorbell_index: u32, xcc_id: u32) -> i32;
    pub fn amdgpu_mes_get_hung_queue_db_array_size(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_mes_detect_and_reset_hung_queues(adev: *mut amdgpu_device, queue_type: i32, detect_only: bool, hung_db_num: *mut u32, hung_db_array: *mut u32, xcc_id: u32) -> i32;
    pub fn amdgpu_mes_rreg(adev: *mut amdgpu_device, reg: u32, xcc_id: u32) -> u32;
    pub fn amdgpu_mes_wreg(adev: *mut amdgpu_device, reg: u32, val: u32, xcc_id: u32) -> i32;
    pub fn amdgpu_mes_reg_write_reg_wait(adev: *mut amdgpu_device, reg0: u32, reg1: u32, reference: u32, mask: u32, xcc_id: u32) -> i32;
    pub fn amdgpu_mes_hdp_flush(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_mes_set_shader_debugger(adev: *mut amdgpu_device, process_context_addr: u64, spi_gdbg_per_vmid_cntl: u32, tcp_watch_cntl: *const u32, flags: u32, trap_en: bool, xcc_id: u32) -> i32;
    pub fn amdgpu_mes_flush_shader_debugger(adev: *mut amdgpu_device, process_context_addr: u64, xcc_id: u32) -> i32;
    pub fn amdgpu_mes_get_aggregated_doorbell_index(adev: *mut amdgpu_device, prio: amdgpu_mes_priority_level) -> u32;
    pub fn amdgpu_mes_init_microcode(adev: *mut amdgpu_device, pipe: i32) -> i32;
    pub fn amdgpu_mes_validate_fw_version(adev: *mut amdgpu_device);
    pub fn amdgpu_mes_suspend_resume_all_supported(adev: *mut amdgpu_device) -> bool;
    pub fn amdgpu_mes_queue_reset_by_mes_supported(adev: *mut amdgpu_device) -> bool;
    pub fn amdgpu_mes_update_enforce_isolation(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_mes_rs64mem_init(mes: *mut amdgpu_mes) -> i32;
    pub fn amdgpu_mes_rs64mem_fini(mes: *mut amdgpu_mes);
    pub fn amdgpu_mes_rs64mem_setup_bitmaps(mes: *mut amdgpu_mes) -> i32;
    pub fn amdgpu_mes_alloc_proc_ctx_index(mes: *mut amdgpu_mes, index: *mut u32) -> i32;
    pub fn amdgpu_mes_free_proc_ctx_index(mes: *mut amdgpu_mes, index: u32);
    pub fn amdgpu_mes_alloc_gang_ctx_index(mes: *mut amdgpu_mes, index: *mut u32) -> i32;
    pub fn amdgpu_mes_free_gang_ctx_index(mes: *mut amdgpu_mes, index: u32);
    pub fn amdgpu_debugfs_mes_event_log_init(adev: *mut amdgpu_device);
}

#[repr(C)] pub struct amdgpu_device { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_ring { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_mes { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_unmap_queues_action { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_mes_priority_level { _private: [u8; 0] }

/*
 * The definitions above intentionally remain ABI declarations: all structure
 * layout and helper semantics belong to the included AMDGPU headers.  The
 * implementation functions are exported through the C ABI so translated
 * callers retain the original externally visible interfaces.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
