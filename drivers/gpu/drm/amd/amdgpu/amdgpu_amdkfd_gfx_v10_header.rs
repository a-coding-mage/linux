/*
 * Copyright 2023 Advanced Micro Devices, Inc.
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
    pub fn kgd_gfx_v10_enable_debug_trap(
        adev: *mut amdgpu_device,
        restore_dbg_registers: bool,
        vmid: u32,
    ) -> u32;
    pub fn kgd_gfx_v10_disable_debug_trap(
        adev: *mut amdgpu_device,
        keep_trap_enabled: bool,
        vmid: u32,
    ) -> u32;
    pub fn kgd_gfx_v10_validate_trap_override_request(
        adev: *mut amdgpu_device,
        trap_override: u32,
        trap_mask_supported: *mut u32,
    ) -> i32;
    pub fn kgd_gfx_v10_set_wave_launch_trap_override(
        adev: *mut amdgpu_device,
        vmid: u32,
        trap_override: u32,
        trap_mask_bits: u32,
        trap_mask_request: u32,
        trap_mask_prev: *mut u32,
        kfd_dbg_trap_cntl_prev: u32,
    ) -> u32;
    pub fn kgd_gfx_v10_set_wave_launch_mode(
        adev: *mut amdgpu_device,
        wave_launch_mode: u8,
        vmid: u32,
    ) -> u32;
    pub fn kgd_gfx_v10_set_address_watch(
        adev: *mut amdgpu_device,
        watch_address: u64,
        watch_address_mask: u32,
        watch_id: u32,
        watch_mode: u32,
        debug_vmid: u32,
        inst: u32,
    ) -> u32;
    pub fn kgd_gfx_v10_clear_address_watch(adev: *mut amdgpu_device, watch_id: u32) -> u32;
    pub fn kgd_gfx_v10_get_iq_wait_times(
        adev: *mut amdgpu_device,
        wait_times: *mut u32,
        inst: u32,
    );
    pub fn kgd_gfx_v10_build_dequeue_wait_counts_packet_info(
        adev: *mut amdgpu_device,
        wait_times: u32,
        sch_wave: u32,
        que_sleep: u32,
        reg_offset: *mut u32,
        reg_data: *mut u32,
    );
    pub fn kgd_gfx_v10_hqd_get_pq_addr(
        adev: *mut amdgpu_device,
        pipe_id: u32,
        queue_id: u32,
        inst: u32,
    ) -> u64;
    pub fn kgd_gfx_v10_hqd_reset(
        adev: *mut amdgpu_device,
        pipe_id: u32,
        queue_id: u32,
        inst: u32,
        utimeout: u32,
    ) -> u64;
    pub fn kgd_gfx_v10_hqd_sdma_get_doorbell(
        adev: *mut amdgpu_device,
        engine: i32,
        queue: i32,
    ) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
