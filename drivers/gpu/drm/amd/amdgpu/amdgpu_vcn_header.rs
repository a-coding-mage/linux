/*
 * Copyright 2016-2024 Advanced Micro Devices, Inc. All rights reserved.
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

// Dependency: amdgpu_ras.h

pub const AMDGPU_VCN_STACK_SIZE: usize = 128 * 1024;
pub const AMDGPU_VCN_CONTEXT_SIZE: usize = 512 * 1024;
pub const AMDGPU_VCN_FIRMWARE_OFFSET: u32 = 256;
pub const AMDGPU_VCN_MAX_ENC_RINGS: usize = 3;
pub const AMDGPU_MAX_VCN_INSTANCES: usize = 4;
pub const AMDGPU_MAX_VCN_ENC_RINGS: usize = AMDGPU_VCN_MAX_ENC_RINGS * AMDGPU_MAX_VCN_INSTANCES;
pub const AMDGPU_VCN_HARVEST_VCN0: u32 = 1 << 0;
pub const AMDGPU_VCN_HARVEST_VCN1: u32 = 1 << 1;

pub const VCN_DEC_KMD_CMD: u32 = 0x80000000;
pub const VCN_DEC_CMD_FENCE: u32 = 0x00000000;
pub const VCN_DEC_CMD_TRAP: u32 = 0x00000001;
pub const VCN_DEC_CMD_WRITE_REG: u32 = 0x00000004;
pub const VCN_DEC_CMD_REG_READ_COND_WAIT: u32 = 0x00000006;
pub const VCN_DEC_CMD_PACKET_START: u32 = 0x0000000a;
pub const VCN_DEC_CMD_PACKET_END: u32 = 0x0000000b;
pub const VCN_DEC_SW_CMD_NO_OP: u32 = 0;
pub const VCN_DEC_SW_CMD_END: u32 = 1;
pub const VCN_DEC_SW_CMD_IB: u32 = 2;
pub const VCN_DEC_SW_CMD_FENCE: u32 = 3;
pub const VCN_DEC_SW_CMD_TRAP: u32 = 4;
pub const VCN_DEC_SW_CMD_IB_AUTO: u32 = 5;
pub const VCN_DEC_SW_CMD_SEMAPHORE: u32 = 6;
pub const VCN_DEC_SW_CMD_PREEMPT_FENCE: u32 = 9;
pub const VCN_DEC_SW_CMD_REG_WRITE: u32 = 11;
pub const VCN_DEC_SW_CMD_REG_WAIT: u32 = 12;
pub const VCN_ENC_CMD_NO_OP: u32 = 0;
pub const VCN_ENC_CMD_END: u32 = 1;
pub const VCN_ENC_CMD_IB: u32 = 2;
pub const VCN_ENC_CMD_FENCE: u32 = 3;
pub const VCN_ENC_CMD_TRAP: u32 = 4;
pub const VCN_ENC_CMD_REG_WRITE: u32 = 11;
pub const VCN_ENC_CMD_REG_WAIT: u32 = 12;
pub const VCN_AON_SOC_ADDRESS_2_0: u32 = 0x1f800;
pub const VCN_VID_IP_ADDRESS_2_0: u32 = 0;
pub const VCN_AON_IP_ADDRESS_2_0: u32 = 0x30000;
pub const mmUVD_RBC_XX_IB_REG_CHECK: u32 = 0x026b;
pub const mmUVD_RBC_XX_IB_REG_CHECK_BASE_IDX: u32 = 1;
pub const mmUVD_REG_XX_MASK: u32 = 0x026c;
pub const mmUVD_REG_XX_MASK_BASE_IDX: u32 = 1;
// 1 second timeout; msecs_to_jiffies(1000)
pub const VCN_IDLE_TIMEOUT: u32 = 1000;

// The following register-access macros depend on kernel/device symbols supplied by other files.
#[macro_export]
macro_rules! RREG32_SOC15_DPG_MODE_1_0 { ($($arg:tt)*) => {{ WREG32_SOC15!($($arg)*); RREG32_SOC15!($($arg)*) }} }
#[macro_export]
macro_rules! WREG32_SOC15_DPG_MODE_1_0 { ($($arg:tt)*) => {{ WREG32_SOC15!($($arg)*); WREG32_SOC15!($($arg)*); WREG32_SOC15!($($arg)*); }} }
#[macro_export]
macro_rules! SOC15_DPG_MODE_OFFSET { ($($arg:tt)*) => {{ let mut internal_reg_offset: u32 = 0; let mut addr: u32 = 0; let video_range = false; let video1_range = false; let aon_range = false; let aon1_range = false; let _ = (&mut addr, video_range, video1_range, aon_range, aon1_range); internal_reg_offset >>= 2; internal_reg_offset }} }
#[macro_export]
macro_rules! RREG32_SOC15_DPG_MODE { ($($arg:tt)*) => {{ WREG32_SOC15!($($arg)*); RREG32_SOC15!($($arg)*) }} }
#[macro_export]
macro_rules! WREG32_SOC15_DPG_MODE { ($($arg:tt)*) => {{ if !$arg { WREG32_SOC15!($($arg)*); WREG32_SOC15!($($arg)*); } else { } }} }
#[macro_export]
macro_rules! SOC24_DPG_MODE_OFFSET { ($($arg:tt)*) => {{ let mut internal_reg_offset: u32 = 0; let mut addr: u32 = 0; let video_range = false; let video1_range = false; let aon_range = false; let aon1_range = false; let _ = (&mut addr, video_range, video1_range, aon_range, aon1_range); internal_reg_offset >>= 2; internal_reg_offset }} }
#[macro_export]
macro_rules! WREG32_SOC24_DPG_MODE { ($($arg:tt)*) => {{ if !$arg { WREG32_SOC15!($($arg)*); WREG32_SOC15!($($arg)*); } else { } }} }

pub const AMDGPU_FW_SHARED_FLAG_0_UNIFIED_QUEUE: u32 = 1 << 2;
pub const AMDGPU_FW_SHARED_FLAG_0_DRM_KEY_INJECT: u32 = 1 << 4;
pub const AMDGPU_VCN_FW_SHARED_FLAG_0_RB: u32 = 1 << 6;
pub const AMDGPU_VCN_MULTI_QUEUE_FLAG: u32 = 1 << 8;
pub const AMDGPU_VCN_SW_RING_FLAG: u32 = 1 << 9;
pub const AMDGPU_VCN_FW_LOGGING_FLAG: u32 = 1 << 10;
pub const AMDGPU_VCN_SMU_VERSION_INFO_FLAG: u32 = 1 << 11;
pub const AMDGPU_VCN_SMU_DPM_INTERFACE_FLAG: u32 = 1 << 11;
pub const AMDGPU_VCN_VF_RB_SETUP_FLAG: u32 = 1 << 14;
pub const AMDGPU_VCN_VF_RB_DECOUPLE_FLAG: u32 = 1 << 15;
pub const MAX_NUM_VCN_RB_SETUP: usize = 4;
pub const AMDGPU_VCN_IB_FLAG_DECODE_BUFFER: u32 = 1;
pub const AMDGPU_VCN_CMD_FLAG_MSG_BUFFER: u32 = 1;
pub const VCN_CODEC_DISABLE_MASK_AV1: u32 = 1 << 0;
pub const VCN_CODEC_DISABLE_MASK_VP9: u32 = 1 << 1;
pub const VCN_CODEC_DISABLE_MASK_HEVC: u32 = 1 << 2;
pub const VCN_CODEC_DISABLE_MASK_H264: u32 = 1 << 3;
pub const AMDGPU_VCN_SMU_DPM_INTERFACE_DGPU: u32 = 0;
pub const AMDGPU_VCN_SMU_DPM_INTERFACE_APU: u32 = 1;
pub const AMDGPU_DRM_KEY_INJECT_WORKAROUND_VCNFW_ASD_HANDSHAKING: u32 = 2;

pub struct amdgpu_hwip_reg_entry;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum amdgpu_vcn_caps { AMDGPU_VCN_RRMT_ENABLED }
#[macro_export]
macro_rules! AMDGPU_VCN_CAPS { ($caps:ident) => { BIT!(concat_idents!(AMDGPU_VCN_, $caps)) } }
#[repr(C)]
pub enum fw_queue_mode { FW_QUEUE_RING_RESET = 1, FW_QUEUE_DPG_HOLD_OFF = 2 }
#[repr(C)]
pub enum engine_status_constants {
    UVD_PGFSM_STATUS__UVDM_UVDU_PWR_ON = 0x2AAAA0, UVD_PGFSM_STATUS__UVDM_UVDU_PWR_ON_2_0 = 0xAAAA0,
    UVD_PGFSM_STATUS__UVDM_UVDU_UVDLM_PWR_ON_3_0 = 0x2A2A8AA0, UVD_PGFSM_CONFIG__UVDM_UVDU_PWR_ON = 2,
    UVD_STATUS__UVD_BUSY = 4, GB_ADDR_CONFIG_DEFAULT = 0x26010011, UVD_STATUS__IDLE = 2, UVD_STATUS__BUSY = 5,
    UVD_POWER_STATUS__UVD_POWER_STATUS_TILES_OFF = 1, UVD_STATUS__RBC_BUSY = 1, UVD_PGFSM_STATUS_UVDJ_PWR_ON = 0,
}
#[repr(C)] pub enum internal_dpg_state { VCN_DPG_STATE__UNPAUSE = 0, VCN_DPG_STATE__PAUSE }

#[repr(C)] pub struct dpg_pause_state { pub fw_based: internal_dpg_state, pub jpeg: internal_dpg_state }
#[repr(C)] pub struct amdgpu_vcn_reg { pub data0: u32, pub data1: u32, pub cmd: u32, pub nop: u32, pub context_id: u32, pub ib_vmid: u32, pub ib_bar_low: u32, pub ib_bar_high: u32, pub ib_size: u32, pub gp_scratch8: u32, pub scratch9: u32 }
#[repr(C)] pub struct amdgpu_vcn_fw_shared { pub cpu_addr: *mut core::ffi::c_void, pub gpu_addr: u64, pub mem_size: u32, pub log_offset: u32 }

#[repr(C)] pub struct amdgpu_vcn_inst {
    pub adev: *mut amdgpu_device, pub inst: i32, pub vcpu_bo: *mut amdgpu_bo, pub cpu_addr: *mut core::ffi::c_void, pub gpu_addr: u64, pub saved_bo: *mut core::ffi::c_void,
    pub ring_dec: amdgpu_ring, pub ring_enc: [amdgpu_ring; AMDGPU_VCN_MAX_ENC_RINGS], pub sched_score: atomic_t, pub irq: amdgpu_irq_src, pub ras_poison_irq: amdgpu_irq_src,
    pub external: amdgpu_vcn_reg, pub dpg_sram_bo: *mut amdgpu_bo, pub pause_state: dpg_pause_state, pub dpg_sram_cpu_addr: *mut core::ffi::c_void, pub dpg_sram_gpu_addr: u64,
    pub dpg_sram_curr_addr: *mut u32, pub dpg_enc_submission_cnt: atomic_t, pub fw_shared: amdgpu_vcn_fw_shared, pub aid_id: u8, pub fw: *const firmware, pub vcn_config: u8,
    pub vcn_codec_disable_mask: u32, pub total_submission_cnt: atomic_t, pub vcn_pg_lock: mutex, pub cur_state: amd_powergating_state, pub idle_work: delayed_work,
    pub fw_version: u32, pub num_enc_rings: u32, pub indirect_sram: bool, pub internal: amdgpu_vcn_reg, pub vcn1_jpeg1_workaround: mutex,
    pub pause_dpg_mode: Option<unsafe extern "C" fn(*mut amdgpu_vcn_inst, *mut dpg_pause_state) -> i32>, pub set_pg_state: Option<unsafe extern "C" fn(*mut amdgpu_vcn_inst, amd_powergating_state) -> i32>,
    pub reset: Option<unsafe extern "C" fn(*mut amdgpu_vcn_inst) -> i32>, pub using_unified_queue: bool, pub engine_reset_mutex: mutex,
}
#[repr(C)] pub struct amdgpu_vcn_ras { pub ras_block: amdgpu_ras_block_object }
#[repr(C)] pub struct amdgpu_vcn { pub num_vcn_inst: u8, pub inst: [amdgpu_vcn_inst; AMDGPU_MAX_VCN_INSTANCES], pub harvest_config: u32, pub ras_if: *mut ras_common_if, pub ras: *mut amdgpu_vcn_ras, pub inst_mask: u16, pub num_inst_per_aid: u8, pub ip_dump: *mut u32, pub supported_reset: u32, pub caps: u32, pub per_inst_fw: bool, pub fw_version: u32, pub workload_profile_active: bool, pub workload_profile_mutex: mutex, pub reg_count: u32, pub reg_list: *const amdgpu_hwip_reg_entry, pub disable_uq: bool, pub disable_kq: bool }
#[repr(C)] pub struct amdgpu_fw_shared_rb_ptrs_struct { pub rptr: u32, pub wptr: u32 }
#[repr(C)] pub struct amdgpu_fw_shared_multi_queue { pub decode_queue_mode: u8, pub encode_generalpurpose_queue_mode: u8, pub encode_lowlatency_queue_mode: u8, pub encode_realtime_queue_mode: u8, pub padding: [u8; 4] }
#[repr(C)] pub struct amdgpu_fw_shared_sw_ring { pub is_enabled: u8, pub padding: [u8; 3] }
#[repr(C)] pub struct amdgpu_fw_shared_unified_queue_struct { pub is_enabled: u8, pub queue_mode: u8, pub queue_status: u8, pub padding: [u8; 5] }
#[repr(C)] pub struct amdgpu_fw_shared_fw_logging { pub is_enabled: u8, pub addr_lo: u32, pub addr_hi: u32, pub size: u32 }
#[repr(C)] pub struct amdgpu_fw_shared_smu_interface_info { pub smu_interface_type: u8, pub padding: [u8; 3] }
#[repr(C)] pub struct amdgpu_fw_shared { pub present_flag_0: u32, pub pad: [u8; 44], pub rb: amdgpu_fw_shared_rb_ptrs_struct, pub pad1: [u8; 1], pub multi_queue: amdgpu_fw_shared_multi_queue, pub sw_ring: amdgpu_fw_shared_sw_ring, pub fw_log: amdgpu_fw_shared_fw_logging, pub smu_interface_info: amdgpu_fw_shared_smu_interface_info }
#[repr(C)] pub struct amdgpu_vcn_rb_setup_info { pub rb_addr_lo: u32, pub rb_addr_hi: u32, pub rb_size: u32 }
#[repr(C)] pub union amdgpu_fw_shared_rb_setup_union { pub direct: [u32; 12], pub rb_info: [amdgpu_vcn_rb_setup_info; MAX_NUM_VCN_RB_SETUP] }
#[repr(C)] pub struct amdgpu_fw_shared_rb_setup { pub is_rb_enabled_flags: u32, pub data: amdgpu_fw_shared_rb_setup_union }
#[repr(C)] pub struct amdgpu_fw_shared_drm_key_wa { pub method: u8, pub reserved: [u8; 3] }
#[repr(C)] pub struct amdgpu_fw_shared_queue_decouple { pub is_enabled: u8, pub reserved: [u8; 7] }
#[repr(C)] pub struct amdgpu_vcn4_fw_shared { pub present_flag_0: u32, pub pad: [u8; 12], pub sq: amdgpu_fw_shared_unified_queue_struct, pub pad1: [u8; 8], pub fw_log: amdgpu_fw_shared_fw_logging, pub pad2: [u8; 20], pub rb_setup: amdgpu_fw_shared_rb_setup, pub smu_dpm_interface: amdgpu_fw_shared_smu_interface_info, pub drm_key_wa: amdgpu_fw_shared_drm_key_wa, pub pad3: [u8; 9], pub decouple: amdgpu_fw_shared_queue_decouple }
#[repr(C)] pub struct amdgpu_vcn_fwlog { pub rptr: u32, pub wptr: u32, pub buffer_size: u32, pub header_size: u32, pub wrapped: u8 }
#[repr(C)] pub struct amdgpu_vcn_decode_buffer { pub valid_buf_flag: u32, pub msg_buffer_address_hi: u32, pub msg_buffer_address_lo: u32, pub pad: [u32; 30] }
#[repr(C)] pub struct amdgpu_vcn_rb_metadata { pub size: u32, pub present_flag_0: u32, pub version: u8, pub ring_id: u8, pub pad: [u8; 26] }
#[repr(C)] pub struct amdgpu_vcn5_fw_shared { pub present_flag_0: u32, pub pad: [u8; 12], pub sq: amdgpu_fw_shared_unified_queue_struct, pub pad1: [u8; 8], pub fw_log: amdgpu_fw_shared_fw_logging, pub pad2: [u8; 20], pub rb_setup: amdgpu_fw_shared_rb_setup, pub smu_dpm_interface: amdgpu_fw_shared_smu_interface_info, pub drm_key_wa: amdgpu_fw_shared_drm_key_wa, pub pad3: [u8; 404] }
pub const VCN_BLOCK_ENCODE_DISABLE_MASK: u32 = 0x80;
pub const VCN_BLOCK_DECODE_DISABLE_MASK: u32 = 0x40;
pub const VCN_BLOCK_QUEUE_DISABLE_MASK: u32 = 0xC0;
#[repr(C)] pub enum vcn_ring_type { VCN_ENCODE_RING, VCN_DECODE_RING, VCN_UNIFIED_RING }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
