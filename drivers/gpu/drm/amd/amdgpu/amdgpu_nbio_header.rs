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

/* amdgpu nbio functions */

#[repr(C)]
pub struct nbio_hdp_flush_reg {
    pub ref_and_mask_cp0: u32,
    pub ref_and_mask_cp1: u32,
    pub ref_and_mask_cp2: u32,
    pub ref_and_mask_cp3: u32,
    pub ref_and_mask_cp4: u32,
    pub ref_and_mask_cp5: u32,
    pub ref_and_mask_cp6: u32,
    pub ref_and_mask_cp7: u32,
    pub ref_and_mask_cp8: u32,
    pub ref_and_mask_cp9: u32,
    pub ref_and_mask_sdma0: u32,
    pub ref_and_mask_sdma1: u32,
    pub ref_and_mask_sdma2: u32,
    pub ref_and_mask_sdma3: u32,
    pub ref_and_mask_sdma4: u32,
    pub ref_and_mask_sdma5: u32,
    pub ref_and_mask_sdma6: u32,
    pub ref_and_mask_sdma7: u32,
}

#[repr(C)]
pub struct amdgpu_nbio_ras {
    pub ras_block: amdgpu_ras_block_object,
    pub handle_ras_controller_intr_no_bifring: Option<unsafe extern "C" fn(*mut amdgpu_device)>,
    pub handle_ras_err_event_athub_intr_no_bifring: Option<unsafe extern "C" fn(*mut amdgpu_device)>,
    pub init_ras_controller_interrupt: Option<unsafe extern "C" fn(*mut amdgpu_device) -> i32>,
    pub init_ras_err_event_athub_interrupt: Option<unsafe extern "C" fn(*mut amdgpu_device) -> i32>,
}

#[repr(C)]
pub struct amdgpu_nbio_funcs {
    pub hdp_flush_reg: *const nbio_hdp_flush_reg,
    pub get_hdp_flush_req_offset: Option<unsafe extern "C" fn(*mut amdgpu_device) -> u32>,
    pub get_hdp_flush_done_offset: Option<unsafe extern "C" fn(*mut amdgpu_device) -> u32>,
    pub get_pcie_index_offset: Option<unsafe extern "C" fn(*mut amdgpu_device) -> u32>,
    pub get_pcie_data_offset: Option<unsafe extern "C" fn(*mut amdgpu_device) -> u32>,
    pub get_pcie_index_hi_offset: Option<unsafe extern "C" fn(*mut amdgpu_device) -> u32>,
    pub get_pcie_port_index_offset: Option<unsafe extern "C" fn(*mut amdgpu_device) -> u32>,
    pub get_pcie_port_data_offset: Option<unsafe extern "C" fn(*mut amdgpu_device) -> u32>,
    pub get_rev_id: Option<unsafe extern "C" fn(*mut amdgpu_device) -> u32>,
    pub mc_access_enable: Option<unsafe extern "C" fn(*mut amdgpu_device, bool)>,
    pub get_memsize: Option<unsafe extern "C" fn(*mut amdgpu_device) -> u32>,
    pub sdma_doorbell_range: Option<unsafe extern "C" fn(*mut amdgpu_device, i32, bool, i32, i32)>,
    pub vpe_doorbell_range: Option<unsafe extern "C" fn(*mut amdgpu_device, i32, bool, i32, i32)>,
    pub vcn_doorbell_range: Option<unsafe extern "C" fn(*mut amdgpu_device, bool, i32, i32)>,
    pub gc_doorbell_init: Option<unsafe extern "C" fn(*mut amdgpu_device)>,
    pub enable_doorbell_aperture: Option<unsafe extern "C" fn(*mut amdgpu_device, bool)>,
    pub enable_doorbell_selfring_aperture: Option<unsafe extern "C" fn(*mut amdgpu_device, bool)>,
    pub ih_doorbell_range: Option<unsafe extern "C" fn(*mut amdgpu_device, bool, i32)>,
    pub enable_doorbell_interrupt: Option<unsafe extern "C" fn(*mut amdgpu_device, bool)>,
    pub update_medium_grain_clock_gating: Option<unsafe extern "C" fn(*mut amdgpu_device, bool)>,
    pub update_medium_grain_light_sleep: Option<unsafe extern "C" fn(*mut amdgpu_device, bool)>,
    pub get_clockgating_state: Option<unsafe extern "C" fn(*mut amdgpu_device, *mut u64)>,
    pub ih_control: Option<unsafe extern "C" fn(*mut amdgpu_device)>,
    pub init_registers: Option<unsafe extern "C" fn(*mut amdgpu_device)>,
    pub remap_hdp_registers: Option<unsafe extern "C" fn(*mut amdgpu_device)>,
    pub enable_aspm: Option<unsafe extern "C" fn(*mut amdgpu_device, bool)>,
    pub program_aspm: Option<unsafe extern "C" fn(*mut amdgpu_device)>,
    pub apply_lc_spc_mode_wa: Option<unsafe extern "C" fn(*mut amdgpu_device)>,
    pub apply_l1_link_width_reconfig_wa: Option<unsafe extern "C" fn(*mut amdgpu_device)>,
    pub clear_doorbell_interrupt: Option<unsafe extern "C" fn(*mut amdgpu_device)>,
    pub get_rom_offset: Option<unsafe extern "C" fn(*mut amdgpu_device) -> u32>,
    pub get_compute_partition_mode: Option<unsafe extern "C" fn(*mut amdgpu_device) -> i32>,
    pub get_memory_partition_mode: Option<unsafe extern "C" fn(*mut amdgpu_device, *mut u32) -> u32>,
    pub is_nps_switch_requested: Option<unsafe extern "C" fn(*mut amdgpu_device) -> bool>,
    pub get_pcie_replay_count: Option<unsafe extern "C" fn(*mut amdgpu_device) -> u64>,
    pub set_reg_remap: Option<unsafe extern "C" fn(*mut amdgpu_device)>,
}

#[repr(C)]
pub struct amdgpu_nbio {
    pub hdp_flush_reg: *const nbio_hdp_flush_reg,
    pub ras_controller_irq: amdgpu_irq_src,
    pub ras_err_event_athub_irq: amdgpu_irq_src,
    pub ras_if: *mut ras_common_if,
    pub funcs: *const amdgpu_nbio_funcs,
    pub ras: *mut amdgpu_nbio_ras,
}

extern "C" {
    pub fn amdgpu_nbio_ras_sw_init(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_nbio_ras_late_init(adev: *mut amdgpu_device, ras_block: *mut ras_common_if) -> i32;
    pub fn amdgpu_nbio_get_pcie_replay_count(adev: *mut amdgpu_device) -> u64;
    pub fn amdgpu_nbio_is_replay_cnt_supported(adev: *mut amdgpu_device) -> bool;
    pub fn amdgpu_nbio_program_aspm(adev: *mut amdgpu_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
