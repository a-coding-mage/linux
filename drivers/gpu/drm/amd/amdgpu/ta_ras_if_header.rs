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

pub const RAS_TA_HOST_IF_VER: u32 = 0;
pub const RSP_ID_MASK: u32 = 1u32 << 31;
#[inline]
pub const fn RSP_ID(cmd_id: u32) -> u32 {
    cmd_id | RSP_ID_MASK
}
pub const TA_RAS_INV_NODE: u16 = 0xffff;

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ras_command {
    TA_RAS_COMMAND__ENABLE_FEATURES = 0,
    TA_RAS_COMMAND__DISABLE_FEATURES,
    TA_RAS_COMMAND__TRIGGER_ERROR,
    TA_RAS_COMMAND__QUERY_BLOCK_INFO,
    TA_RAS_COMMAND__QUERY_SUB_BLOCK_INFO,
    TA_RAS_COMMAND__QUERY_ADDRESS,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ta_ras_status {
    TA_RAS_STATUS__SUCCESS = 0x0000,
    TA_RAS_STATUS__RESET_NEEDED = 0xA001,
    TA_RAS_STATUS__ERROR_INVALID_PARAMETER = 0xA002,
    TA_RAS_STATUS__ERROR_RAS_NOT_AVAILABLE = 0xA003,
    TA_RAS_STATUS__ERROR_RAS_DUPLICATE_CMD = 0xA004,
    TA_RAS_STATUS__ERROR_INJECTION_FAILED = 0xA005,
    TA_RAS_STATUS__ERROR_ASD_READ_WRITE = 0xA006,
    TA_RAS_STATUS__ERROR_TOGGLE_DF_CSTATE = 0xA007,
    TA_RAS_STATUS__ERROR_TIMEOUT = 0xA008,
    TA_RAS_STATUS__ERROR_BLOCK_DISABLED = 0xA009,
    TA_RAS_STATUS__ERROR_GENERIC = 0xA00A,
    TA_RAS_STATUS__ERROR_RAS_MMHUB_INIT = 0xA00B,
    TA_RAS_STATUS__ERROR_GET_DEV_INFO = 0xA00C,
    TA_RAS_STATUS__ERROR_UNSUPPORTED_DEV = 0xA00D,
    TA_RAS_STATUS__ERROR_NOT_INITIALIZED = 0xA00E,
    TA_RAS_STATUS__ERROR_TEE_INTERNAL = 0xA00F,
    TA_RAS_STATUS__ERROR_UNSUPPORTED_FUNCTION = 0xA010,
    TA_RAS_STATUS__ERROR_SYS_DRV_REG_ACCESS = 0xA011,
    TA_RAS_STATUS__ERROR_RAS_READ_WRITE = 0xA012,
    TA_RAS_STATUS__ERROR_NULL_PTR = 0xA013,
    TA_RAS_STATUS__ERROR_UNSUPPORTED_IP = 0xA014,
    TA_RAS_STATUS__ERROR_PCS_STATE_QUIET = 0xA015,
    TA_RAS_STATUS__ERROR_PCS_STATE_ERROR = 0xA016,
    TA_RAS_STATUS__ERROR_PCS_STATE_HANG = 0xA017,
    TA_RAS_STATUS__ERROR_PCS_STATE_UNKNOWN = 0xA018,
    TA_RAS_STATUS__ERROR_UNSUPPORTED_ERROR_INJ = 0xA019,
    TA_RAS_STATUS__TEE_ERROR_ACCESS_DENIED = 0xA01A,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ta_ras_block {
    TA_RAS_BLOCK__UMC = 0,
    TA_RAS_BLOCK__SDMA,
    TA_RAS_BLOCK__GFX,
    TA_RAS_BLOCK__MMHUB,
    TA_RAS_BLOCK__ATHUB,
    TA_RAS_BLOCK__PCIE_BIF,
    TA_RAS_BLOCK__HDP,
    TA_RAS_BLOCK__XGMI_WAFL,
    TA_RAS_BLOCK__DF,
    TA_RAS_BLOCK__SMN,
    TA_RAS_BLOCK__SEM,
    TA_RAS_BLOCK__MP0,
    TA_RAS_BLOCK__MP1,
    TA_RAS_BLOCK__FUSE,
    TA_RAS_BLOCK__MCA,
    TA_RAS_BLOCK__VCN,
    TA_RAS_BLOCK__JPEG,
    TA_RAS_BLOCK__IH,
    TA_RAS_BLOCK__MPIO,
    TA_RAS_BLOCK__MMSCH,
    TA_NUM_BLOCK_MAX,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ta_ras_mca_block {
    TA_RAS_MCA_BLOCK__MP0 = 0,
    TA_RAS_MCA_BLOCK__MP1 = 1,
    TA_RAS_MCA_BLOCK__MPIO = 2,
    TA_RAS_MCA_BLOCK__IOHC = 3,
    TA_MCA_NUM_BLOCK_MAX,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ta_ras_error_type {
    TA_RAS_ERROR__NONE = 0,
    TA_RAS_ERROR__PARITY = 1,
    TA_RAS_ERROR__SINGLE_CORRECTABLE = 2,
    TA_RAS_ERROR__MULTI_UNCORRECTABLE = 4,
    TA_RAS_ERROR__POISON = 8,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ta_ras_address_type {
    TA_RAS_MCA_TO_PA,
    TA_RAS_PA_TO_MCA,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ta_ras_nps_mode {
    TA_RAS_UNKNOWN_MODE = 0,
    TA_RAS_NPS1_MODE = 1,
    TA_RAS_NPS2_MODE = 2,
    TA_RAS_NPS4_MODE = 4,
    TA_RAS_NPS8_MODE = 8,
}

#[repr(C)]
pub struct ta_ras_enable_features_input { pub block_id: ta_ras_block, pub error_type: ta_ras_error_type }
#[repr(C)]
pub struct ta_ras_disable_features_input { pub block_id: ta_ras_block, pub error_type: ta_ras_error_type }
#[repr(C)]
pub struct ta_ras_trigger_error_input {
    pub block_id: ta_ras_block,
    pub inject_error_type: ta_ras_error_type,
    pub sub_block_index: u32,
    pub address: u64,
    pub value: u64,
}
#[repr(C)]
pub struct ta_ras_init_flags {
    pub poison_mode_en: u8, pub dgpu_mode: u8, pub xcc_mask: u16,
    pub channel_dis_num: u8, pub nps_mode: u8, pub active_umc_mask: u32, pub vram_type: u8,
}
#[repr(C)]
pub struct ta_ras_mca_addr { pub err_addr: u64, pub ch_inst: u32, pub umc_inst: u32, pub node_inst: u32, pub socket_id: u32 }
#[repr(C)]
pub struct ta_ras_phy_addr { pub pa: u64, pub bank: u32, pub channel_idx: u32 }
#[repr(C)]
pub struct ta_ras_query_address_input { pub addr_type: ta_ras_address_type, pub ma: ta_ras_mca_addr, pub pa: ta_ras_phy_addr }
#[repr(C)]
pub struct ta_ras_output_flags { pub ras_init_success_flag: u8, pub err_inject_switch_disable_flag: u8, pub reg_access_failure_flag: u8 }
#[repr(C)]
pub struct ta_ras_query_address_output { pub flags: ta_ras_output_flags, pub ma: ta_ras_mca_addr, pub pa: ta_ras_phy_addr }

#[repr(C)]
pub union ta_ras_cmd_input {
    pub init_flags: ta_ras_init_flags,
    pub enable_features: ta_ras_enable_features_input,
    pub disable_features: ta_ras_disable_features_input,
    pub trigger_error: ta_ras_trigger_error_input,
    pub address: ta_ras_query_address_input,
    pub reserve_pad: [u32; 256],
}

#[repr(C)]
pub union ta_ras_cmd_output {
    pub flags: ta_ras_output_flags,
    pub address: ta_ras_query_address_output,
    pub reserve_pad: [u32; 256],
}

#[repr(C)]
pub struct ta_ras_shared_memory {
    pub cmd_id: u32,
    pub resp_id: u32,
    pub ras_status: u32,
    pub if_version: u32,
    pub ras_in_message: ta_ras_cmd_input,
    pub ras_out_message: ta_ras_cmd_output,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
