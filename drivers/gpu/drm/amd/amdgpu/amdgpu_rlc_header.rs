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

// Dependency supplied by clearstate_defs.h is intentionally not implemented here.

pub const AMDGPU_MAX_RLC_INSTANCES: usize = 8;

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FIRMWARE_ID {
    FIRMWARE_ID_INVALID = 0, FIRMWARE_ID_RLC_G_UCODE = 1, FIRMWARE_ID_RLC_TOC = 2,
    FIRMWARE_ID_RLCG_SCRATCH = 3, FIRMWARE_ID_RLC_SRM_ARAM = 4,
    FIRMWARE_ID_RLC_SRM_INDEX_ADDR = 5, FIRMWARE_ID_RLC_SRM_INDEX_DATA = 6,
    FIRMWARE_ID_RLC_P_UCODE = 7, FIRMWARE_ID_RLC_V_UCODE = 8, FIRMWARE_ID_RLX6_UCODE = 9,
    FIRMWARE_ID_RLX6_DRAM_BOOT = 10, FIRMWARE_ID_GLOBAL_TAP_DELAYS = 11,
    FIRMWARE_ID_SE0_TAP_DELAYS = 12, FIRMWARE_ID_SE1_TAP_DELAYS = 13,
    FIRMWARE_ID_GLOBAL_SE0_SE1_SKEW_DELAYS = 14, FIRMWARE_ID_SDMA0_UCODE = 15,
    FIRMWARE_ID_SDMA0_JT = 16, FIRMWARE_ID_SDMA1_UCODE = 17, FIRMWARE_ID_SDMA1_JT = 18,
    FIRMWARE_ID_CP_CE = 19, FIRMWARE_ID_CP_PFP = 20, FIRMWARE_ID_CP_ME = 21,
    FIRMWARE_ID_CP_MEC = 22, FIRMWARE_ID_CP_MES = 23, FIRMWARE_ID_MES_STACK = 24,
    FIRMWARE_ID_RLC_SRM_DRAM_SR = 25, FIRMWARE_ID_RLCG_SCRATCH_SR = 26,
    FIRMWARE_ID_RLCP_SCRATCH_SR = 27, FIRMWARE_ID_RLCV_SCRATCH_SR = 28,
    FIRMWARE_ID_RLX6_DRAM_SR = 29, FIRMWARE_ID_SDMA0_PG_CONTEXT = 30,
    FIRMWARE_ID_SDMA1_PG_CONTEXT = 31, FIRMWARE_ID_GLOBAL_MUX_SELECT_RAM = 32,
    FIRMWARE_ID_SE0_MUX_SELECT_RAM = 33, FIRMWARE_ID_SE1_MUX_SELECT_RAM = 34,
    FIRMWARE_ID_ACCUM_CTRL_RAM = 35, FIRMWARE_ID_RLCP_CAM = 36,
    FIRMWARE_ID_RLC_SPP_CAM_EXT = 37, FIRMWARE_ID_MAX = 38,
}

// SOC21_FIRMWARE_ID and SOC24_FIRMWARE_ID use the same numeric layout for 0..36.
// The remaining source-level identifiers are retained as constants.
pub const SOC21_FIRMWARE_ID_INVALID: u32 = 0;
pub const SOC21_FIRMWARE_ID_RLC_G_UCODE: u32 = 1;
pub const SOC21_FIRMWARE_ID_RLC_TOC: u32 = 2;
pub const SOC21_FIRMWARE_ID_RLCG_SCRATCH: u32 = 3;
pub const SOC21_FIRMWARE_ID_RLC_SRM_ARAM: u32 = 4;
pub const SOC21_FIRMWARE_ID_RLC_P_UCODE: u32 = 5;
pub const SOC21_FIRMWARE_ID_RLC_V_UCODE: u32 = 6;
pub const SOC21_FIRMWARE_ID_RLX6_UCODE: u32 = 7;
pub const SOC21_FIRMWARE_ID_RLX6_UCODE_CORE1: u32 = 8;
pub const SOC21_FIRMWARE_ID_RLX6_DRAM_BOOT: u32 = 9;
pub const SOC21_FIRMWARE_ID_RLX6_DRAM_BOOT_CORE1: u32 = 10;
pub const SOC21_FIRMWARE_ID_SDMA_UCODE_TH0: u32 = 11;
pub const SOC21_FIRMWARE_ID_SDMA_UCODE_TH1: u32 = 12;
pub const SOC21_FIRMWARE_ID_CP_PFP: u32 = 13;
pub const SOC21_FIRMWARE_ID_CP_ME: u32 = 14;
pub const SOC21_FIRMWARE_ID_CP_MEC: u32 = 15;
pub const SOC21_FIRMWARE_ID_RS64_MES_P0: u32 = 16;
pub const SOC21_FIRMWARE_ID_RS64_MES_P1: u32 = 17;
pub const SOC21_FIRMWARE_ID_RS64_PFP: u32 = 18;
pub const SOC21_FIRMWARE_ID_RS64_ME: u32 = 19;
pub const SOC21_FIRMWARE_ID_RS64_MEC: u32 = 20;
pub const SOC21_FIRMWARE_ID_RS64_MES_P0_STACK: u32 = 21;
pub const SOC21_FIRMWARE_ID_RS64_MES_P1_STACK: u32 = 22;
pub const SOC21_FIRMWARE_ID_RS64_PFP_P0_STACK: u32 = 23;
pub const SOC21_FIRMWARE_ID_RS64_PFP_P1_STACK: u32 = 24;
pub const SOC21_FIRMWARE_ID_RS64_ME_P0_STACK: u32 = 25;
pub const SOC21_FIRMWARE_ID_RS64_ME_P1_STACK: u32 = 26;
pub const SOC21_FIRMWARE_ID_RS64_MEC_P0_STACK: u32 = 27;
pub const SOC21_FIRMWARE_ID_RS64_MEC_P1_STACK: u32 = 28;
pub const SOC21_FIRMWARE_ID_RS64_MEC_P2_STACK: u32 = 29;
pub const SOC21_FIRMWARE_ID_RS64_MEC_P3_STACK: u32 = 30;
pub const SOC21_FIRMWARE_ID_RLC_SRM_DRAM_SR: u32 = 31;
pub const SOC21_FIRMWARE_ID_RLCG_SCRATCH_SR: u32 = 32;
pub const SOC21_FIRMWARE_ID_RLCP_SCRATCH_SR: u32 = 33;
pub const SOC21_FIRMWARE_ID_RLCV_SCRATCH_SR: u32 = 34;
pub const SOC21_FIRMWARE_ID_RLX6_DRAM_SR: u32 = 35;
pub const SOC21_FIRMWARE_ID_RLX6_DRAM_SR_CORE1: u32 = 36;
pub const SOC21_FIRMWARE_ID_MAX: u32 = 37;

pub const SOC24_FIRMWARE_ID_INVALID: u32 = 0;
pub const SOC24_FIRMWARE_ID_RLC_G_UCODE: u32 = 1;
pub const SOC24_FIRMWARE_ID_RLC_TOC: u32 = 2;
pub const SOC24_FIRMWARE_ID_RLCG_SCRATCH: u32 = 3;
pub const SOC24_FIRMWARE_ID_RLC_SRM_ARAM: u32 = 4;
pub const SOC24_FIRMWARE_ID_RLC_P_UCODE: u32 = 5;
pub const SOC24_FIRMWARE_ID_RLC_V_UCODE: u32 = 6;
pub const SOC24_FIRMWARE_ID_RLX6_UCODE: u32 = 7;
pub const SOC24_FIRMWARE_ID_RLX6_UCODE_CORE1: u32 = 8;
pub const SOC24_FIRMWARE_ID_RLX6_DRAM_BOOT: u32 = 9;
pub const SOC24_FIRMWARE_ID_RLX6_DRAM_BOOT_CORE1: u32 = 10;
pub const SOC24_FIRMWARE_ID_SDMA_UCODE_TH0: u32 = 11;
pub const SOC24_FIRMWARE_ID_SDMA_UCODE_TH1: u32 = 12;
pub const SOC24_FIRMWARE_ID_CP_PFP: u32 = 13;
pub const SOC24_FIRMWARE_ID_CP_ME: u32 = 14;
pub const SOC24_FIRMWARE_ID_CP_MEC: u32 = 15;
pub const SOC24_FIRMWARE_ID_RS64_MES_P0: u32 = 16;
pub const SOC24_FIRMWARE_ID_RS64_MES_P1: u32 = 17;
pub const SOC24_FIRMWARE_ID_RS64_PFP: u32 = 18;
pub const SOC24_FIRMWARE_ID_RS64_ME: u32 = 19;
pub const SOC24_FIRMWARE_ID_RS64_MEC: u32 = 20;
pub const SOC24_FIRMWARE_ID_RS64_MES_P0_STACK: u32 = 21;
pub const SOC24_FIRMWARE_ID_RS64_MES_P1_STACK: u32 = 22;
pub const SOC24_FIRMWARE_ID_RS64_PFP_P0_STACK: u32 = 23;
pub const SOC24_FIRMWARE_ID_RS64_PFP_P1_STACK: u32 = 24;
pub const SOC24_FIRMWARE_ID_RS64_ME_P0_STACK: u32 = 25;
pub const SOC24_FIRMWARE_ID_RS64_ME_P1_STACK: u32 = 26;
pub const SOC24_FIRMWARE_ID_RS64_MEC_P0_STACK: u32 = 27;
pub const SOC24_FIRMWARE_ID_RS64_MEC_P1_STACK: u32 = 28;
pub const SOC24_FIRMWARE_ID_RS64_MEC_P2_STACK: u32 = 29;
pub const SOC24_FIRMWARE_ID_RS64_MEC_P3_STACK: u32 = 30;
pub const SOC24_FIRMWARE_ID_RLC_SRM_DRAM_SR: u32 = 31;
pub const SOC24_FIRMWARE_ID_RLCG_SCRATCH_SR: u32 = 32;
pub const SOC24_FIRMWARE_ID_RLCP_SCRATCH_SR: u32 = 33;
pub const SOC24_FIRMWARE_ID_RLCV_SCRATCH_SR: u32 = 34;
pub const SOC24_FIRMWARE_ID_RLX6_DRAM_SR: u32 = 35;
pub const SOC24_FIRMWARE_ID_RLX6_DRAM_SR_CORE1: u32 = 36;
pub const SOC24_FIRMWARE_ID_RLCDEBUGLOG: u32 = 37;
pub const SOC24_FIRMWARE_ID_SRIOV_DEBUG: u32 = 38;
pub const SOC24_FIRMWARE_ID_SRIOV_CSA_RLC: u32 = 39;
pub const SOC24_FIRMWARE_ID_SRIOV_CSA_SDMA: u32 = 40;
pub const SOC24_FIRMWARE_ID_SRIOV_CSA_CP: u32 = 41;
pub const SOC24_FIRMWARE_ID_UMF_ZONE_PAD: u32 = 42;
pub const SOC24_FIRMWARE_ID_MAX: u32 = 43;

#[repr(C)]
pub union RLC_TABLE_OF_CONTENT { pub DW0: u32, pub DW1: u32, pub DW2: u32, pub DW3: u32 }
#[repr(C)]
pub union RLC_TABLE_OF_CONTENT_V2 { pub DW0: u32, pub DW1: u32 }
pub const RLC_TOC_MAX_SIZE: usize = 64;

// C bitfield masks, preserving the packed 32-bit word representation.
pub const RLC_TOC_DW0_OFFSET_MASK: u32 = 0x01ff_ffff;
pub const RLC_TOC_DW0_ID_MASK: u32 = 0xfe00_0000;

#[repr(C)]
pub struct amdgpu_rlc_funcs {
    pub is_rlc_enabled: Option<unsafe extern "C" fn(*mut amdgpu_device) -> bool>,
    pub set_safe_mode: Option<unsafe extern "C" fn(*mut amdgpu_device, i32)>,
    pub unset_safe_mode: Option<unsafe extern "C" fn(*mut amdgpu_device, i32)>,
    pub init: Option<unsafe extern "C" fn(*mut amdgpu_device) -> i32>,
    pub get_csb_size: Option<unsafe extern "C" fn(*mut amdgpu_device) -> u32>,
    pub get_csb_buffer: Option<unsafe extern "C" fn(*mut amdgpu_device, *mut u32)>,
    pub get_cp_table_num: Option<unsafe extern "C" fn(*mut amdgpu_device) -> i32>,
    pub resume: Option<unsafe extern "C" fn(*mut amdgpu_device) -> i32>,
    pub stop: Option<unsafe extern "C" fn(*mut amdgpu_device)>,
    pub reset: Option<unsafe extern "C" fn(*mut amdgpu_device)>,
    pub start: Option<unsafe extern "C" fn(*mut amdgpu_device)>,
    pub update_spm_vmid: Option<unsafe extern "C" fn(*mut amdgpu_device, i32, *mut amdgpu_ring, u32)>,
    pub is_rlcg_access_range: Option<unsafe extern "C" fn(*mut amdgpu_device, u32) -> bool>,
}

#[repr(C)]
pub struct amdgpu_rlc_reg_funcs {
    pub rreg32: Option<unsafe extern "C" fn(*mut amdgpu_device, u32, u32, u32, u32) -> u32>,
    pub wreg32: Option<unsafe extern "C" fn(*mut amdgpu_device, u32, u32, u32, u32, u32)>,
}

#[repr(C)]
pub struct amdgpu_rlcg_reg_access_ctrl {
    pub scratch_reg0: u32, pub scratch_reg1: u32, pub scratch_reg2: u32, pub scratch_reg3: u32,
    pub grbm_cntl: u32, pub grbm_idx: u32, pub spare_int: u32,
    pub vfi_cmd: u32, pub vfi_stat: u32, pub vfi_addr: u32, pub vfi_data: u32,
    pub vfi_grbm_cntl: u32, pub vfi_grbm_idx: u32, pub vfi_grbm_cntl_data: u32,
    pub vfi_grbm_idx_data: u32,
}

// External types are supplied by other translated headers.
#[allow(non_camel_case_types)] pub enum amdgpu_device {}
#[allow(non_camel_case_types)] pub enum amdgpu_bo {}
#[allow(non_camel_case_types)] pub enum amdgpu_ring {}
#[allow(non_camel_case_types)] pub enum cs_section_def {}

#[repr(C)]
pub struct amdgpu_rlc {
    pub save_restore_obj: *mut amdgpu_bo, pub save_restore_gpu_addr: u64, pub sr_ptr: *mut u32,
    pub reg_list: *const u32, pub reg_list_size: u32, pub clear_state_obj: *mut amdgpu_bo,
    pub clear_state_gpu_addr: u64, pub cs_ptr: *mut u32, pub cs_data: *const cs_section_def,
    pub clear_state_size: u32, pub cp_table_obj: *mut amdgpu_bo, pub cp_table_gpu_addr: u64,
    pub cp_table_ptr: *mut u32, pub cp_table_size: u32,
    pub in_safe_mode: [bool; AMDGPU_MAX_RLC_INSTANCES], pub funcs: *const amdgpu_rlc_funcs,
    pub reg_funcs: *const amdgpu_rlc_reg_funcs,
    pub save_and_restore_offset: u32, pub clear_state_descriptor_offset: u32,
    pub avail_scratch_ram_locations: u32, pub reg_restore_list_size: u32,
    pub reg_list_format_start: u32, pub reg_list_format_separate_start: u32,
    pub starting_offsets_start: u32, pub reg_list_format_size_bytes: u32,
    pub reg_list_size_bytes: u32, pub reg_list_format_direct_reg_list_length: u32,
    pub save_restore_list_cntl_size_bytes: u32, pub save_restore_list_gpm_size_bytes: u32,
    pub save_restore_list_srm_size_bytes: u32, pub rlc_iram_ucode_size_bytes: u32,
    pub rlc_dram_ucode_size_bytes: u32, pub rlc_1_iram_ucode_size_bytes: u32,
    pub rlc_1_dram_ucode_size_bytes: u32, pub rlcp_ucode_size_bytes: u32,
    pub rlcv_ucode_size_bytes: u32, pub global_tap_delays_ucode_size_bytes: u32,
    pub se0_tap_delays_ucode_size_bytes: u32, pub se1_tap_delays_ucode_size_bytes: u32,
    pub se2_tap_delays_ucode_size_bytes: u32, pub se3_tap_delays_ucode_size_bytes: u32,
    pub register_list_format: *mut u32, pub register_restore: *mut u32,
    pub save_restore_list_cntl: *mut u8, pub save_restore_list_gpm: *mut u8,
    pub save_restore_list_srm: *mut u8, pub rlc_iram_ucode: *mut u8, pub rlc_dram_ucode: *mut u8,
    pub rlc_1_iram_ucode: *mut u8, pub rlc_1_dram_ucode: *mut u8, pub rlcp_ucode: *mut u8,
    pub rlcv_ucode: *mut u8, pub global_tap_delays_ucode: *mut u8, pub se0_tap_delays_ucode: *mut u8,
    pub se1_tap_delays_ucode: *mut u8, pub se2_tap_delays_ucode: *mut u8, pub se3_tap_delays_ucode: *mut u8,
    pub is_rlc_v2_1: bool, pub rlc_autoload_bo: *mut amdgpu_bo, pub rlc_autoload_gpu_addr: u64,
    pub rlc_autoload_ptr: *mut core::ffi::c_void, pub rlc_toc_bo: *mut amdgpu_bo,
    pub rlc_toc_gpu_addr: u64, pub rlc_toc_buf: *mut core::ffi::c_void,
    pub rlcg_reg_access_supported: bool,
    pub reg_access_ctrl: [amdgpu_rlcg_reg_access_ctrl; AMDGPU_MAX_RLC_INSTANCES],
}

extern "C" {
    pub fn amdgpu_gfx_rlc_enter_safe_mode(adev: *mut amdgpu_device, xcc_id: i32);
    pub fn amdgpu_gfx_rlc_exit_safe_mode(adev: *mut amdgpu_device, xcc_id: i32);
    pub fn amdgpu_gfx_rlc_init_sr(adev: *mut amdgpu_device, dws: u32) -> i32;
    pub fn amdgpu_gfx_rlc_init_csb(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_gfx_rlc_init_cpt(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_gfx_rlc_setup_cp_table(adev: *mut amdgpu_device);
    pub fn amdgpu_gfx_rlc_fini(adev: *mut amdgpu_device);
    pub fn amdgpu_gfx_rlc_init_microcode(adev: *mut amdgpu_device, version_major: u16, version_minor: u16) -> i32;
    pub fn amdgpu_early_init_rlc_reg_funcs(adev: *mut amdgpu_device);
    pub fn amdgpu_init_rlc_reg_funcs(adev: *mut amdgpu_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
