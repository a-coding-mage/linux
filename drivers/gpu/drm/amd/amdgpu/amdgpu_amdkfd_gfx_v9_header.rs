/*
 * Copyright 2019 Advanced Micro Devices, Inc.
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

extern "C" {
    pub fn kgd_gfx_v9_program_sh_mem_settings(adev: *mut amdgpu_device, vmid: u32, sh_mem_config: u32, sh_mem_ape1_base: u32, sh_mem_ape1_limit: u32, sh_mem_bases: u32, inst: u32);
    pub fn kgd_gfx_v9_set_pasid_vmid_mapping(adev: *mut amdgpu_device, pasid: u32, vmid: ::core::ffi::c_uint, inst: u32) -> i32;
    pub fn kgd_gfx_v9_init_interrupts(adev: *mut amdgpu_device, pipe_id: u32, inst: u32) -> i32;
    pub fn kgd_gfx_v9_hqd_load(adev: *mut amdgpu_device, mqd: *mut ::core::ffi::c_void, pipe_id: u32, queue_id: u32, wptr: *mut u32, wptr_shift: u32, wptr_mask: u32, mm: *mut mm_struct, inst: u32) -> i32;
    pub fn kgd_gfx_v9_hiq_mqd_load(adev: *mut amdgpu_device, mqd: *mut ::core::ffi::c_void, pipe_id: u32, queue_id: u32, doorbell_off: u32, inst: u32) -> i32;
    pub fn kgd_gfx_v9_hqd_dump(adev: *mut amdgpu_device, pipe_id: u32, queue_id: u32, dump: *mut *mut [u32; 2], n_regs: *mut u32, inst: u32) -> i32;
    pub fn kgd_gfx_v9_hqd_is_occupied(adev: *mut amdgpu_device, queue_address: u64, pipe_id: u32, queue_id: u32, inst: u32) -> bool;
    pub fn kgd_gfx_v9_hqd_destroy(adev: *mut amdgpu_device, mqd: *mut ::core::ffi::c_void, reset_type: kfd_preempt_type, utimeout: ::core::ffi::c_uint, pipe_id: u32, queue_id: u32, inst: u32) -> i32;
    pub fn kgd_gfx_v9_wave_control_execute(adev: *mut amdgpu_device, gfx_index_val: u32, sq_cmd: u32, inst: u32) -> i32;
    pub fn kgd_gfx_v9_get_atc_vmid_pasid_mapping_info(adev: *mut amdgpu_device, vmid: u8, p_pasid: *mut u16) -> bool;
    pub fn kgd_gfx_v9_set_vm_context_page_table_base(adev: *mut amdgpu_device, vmid: u32, page_table_base: u64);
    pub fn kgd_gfx_v9_get_cu_occupancy(adev: *mut amdgpu_device, cu_occupancy: *mut kfd_cu_occupancy, max_waves_per_cu: *mut i32, inst: u32);
    pub fn kgd_gfx_v9_program_trap_handler_settings(adev: *mut amdgpu_device, vmid: u32, tba_addr: u64, tma_addr: u64, inst: u32);
    pub fn kgd_gfx_v9_acquire_queue(adev: *mut amdgpu_device, pipe_id: u32, queue_id: u32, inst: u32);
    pub fn kgd_gfx_v9_get_queue_mask(adev: *mut amdgpu_device, pipe_id: u32, queue_id: u32) -> u64;
    pub fn kgd_gfx_v9_release_queue(adev: *mut amdgpu_device, inst: u32);
    pub fn kgd_gfx_v9_set_wave_launch_stall(adev: *mut amdgpu_device, vmid: u32, stall: bool);
    pub fn kgd_gfx_v9_enable_debug_trap(adev: *mut amdgpu_device, restore_dbg_registers: bool, vmid: u32) -> u32;
    pub fn kgd_gfx_v9_disable_debug_trap(adev: *mut amdgpu_device, keep_trap_enabled: bool, vmid: u32) -> u32;
    pub fn kgd_gfx_v9_validate_trap_override_request(adev: *mut amdgpu_device, trap_override: u32, trap_mask_supported: *mut u32) -> i32;
    pub fn kgd_gfx_v9_set_wave_launch_mode(adev: *mut amdgpu_device, wave_launch_mode: u8, vmid: u32) -> u32;
    pub fn kgd_gfx_v9_set_wave_launch_trap_override(adev: *mut amdgpu_device, vmid: u32, trap_override: u32, trap_mask_bits: u32, trap_mask_request: u32, trap_mask_prev: *mut u32, kfd_dbg_trap_cntl_prev: u32) -> u32;
    pub fn kgd_gfx_v9_set_address_watch(adev: *mut amdgpu_device, watch_address: u64, watch_address_mask: u32, watch_id: u32, watch_mode: u32, debug_vmid: u32, inst: u32) -> u32;
    pub fn kgd_gfx_v9_clear_address_watch(adev: *mut amdgpu_device, watch_id: u32) -> u32;
    pub fn kgd_gfx_v9_get_iq_wait_times(adev: *mut amdgpu_device, wait_times: *mut u32, inst: u32);
    pub fn kgd_gfx_v9_build_dequeue_wait_counts_packet_info(adev: *mut amdgpu_device, wait_times: u32, sch_wave: u32, que_sleep: u32, reg_offset: *mut u32, reg_data: *mut u32);
    pub fn kgd_gfx_v9_hqd_get_pq_addr(adev: *mut amdgpu_device, pipe_id: u32, queue_id: u32, inst: u32) -> u64;
    pub fn kgd_gfx_v9_hqd_reset(adev: *mut amdgpu_device, pipe_id: u32, queue_id: u32, inst: u32, utimeout: ::core::ffi::c_uint) -> u64;
    pub fn kgd_gfx_v9_hqd_sdma_get_doorbell(adev: *mut amdgpu_device, engine: i32, queue: i32) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
