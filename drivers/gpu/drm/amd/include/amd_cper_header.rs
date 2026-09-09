/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2025 Advanced Micro Devices, Inc.
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

pub const CPER_HDR_REV_1: u32 = 0x100;
pub const CPER_SEC_MINOR_REV_1: u8 = 0x01;
pub const CPER_SEC_MAJOR_REV_22: u8 = 0x22;
pub const CPER_MAX_OAM_COUNT: usize = 8;
pub const CPER_CTX_TYPE_CRASH: u32 = 1;
pub const CPER_CTX_TYPE_BOOT: u32 = 9;
pub const CPER_CREATOR_ID_AMDGPU: &str = "amdgpu";

// GUID_INIT and guid_t are supplied by the surrounding environment.
pub const CPER_NOTIFY_MCE: guid_t = GUID_INIT!(0xE8F56FFE, 0x919C, 0x4cc5, 0xBA, 0x88, 0x65, 0xAB, 0xE1, 0x49, 0x13, 0xBB);
pub const CPER_NOTIFY_CMC: guid_t = GUID_INIT!(0x2DCE8BB1, 0xBDD7, 0x450e, 0xB9, 0xAD, 0x9C, 0xF4, 0xEB, 0xD4, 0xF8, 0x90);
pub const BOOT_TYPE: guid_t = GUID_INIT!(0x3D61A466, 0xAB40, 0x409a, 0xA6, 0x98, 0xF3, 0x62, 0xD4, 0x64, 0xB3, 0x8F);
pub const AMD_CRASHDUMP: guid_t = GUID_INIT!(0x32AC0C78, 0x2623, 0x48F6, 0xB0, 0xD0, 0x73, 0x65, 0x72, 0x5F, 0xD6, 0xAE);
pub const AMD_GPU_NONSTANDARD_ERROR: guid_t = GUID_INIT!(0x32AC0C78, 0x2623, 0x48F6, 0x81, 0xA2, 0xAC, 0x69, 0x17, 0x80, 0x55, 0x1D);
pub const PROC_ERR_SECTION_TYPE: guid_t = GUID_INIT!(0xDC3EA0B0, 0xA144, 0x4797, 0xB9, 0x5B, 0x53, 0xFA, 0x24, 0x2B, 0x6E, 0x1D);

#[repr(i32)]
pub enum cper_error_severity { CPER_SEV_NON_FATAL_UNCORRECTED = 0, CPER_SEV_FATAL_UNCORRECTED = 1, CPER_SEV_NON_FATAL_CORRECTED = 2, CPER_SEV_NUM = 3, CPER_SEV_UNUSED = 10 }

#[repr(i32)]
pub enum cper_aca_reg { CPER_ACA_REG_CTL_LO = 0, CPER_ACA_REG_CTL_HI = 1, CPER_ACA_REG_STATUS_LO = 2, CPER_ACA_REG_STATUS_HI = 3, CPER_ACA_REG_ADDR_LO = 4, CPER_ACA_REG_ADDR_HI = 5, CPER_ACA_REG_MISC0_LO = 6, CPER_ACA_REG_MISC0_HI = 7, CPER_ACA_REG_CONFIG_LO = 8, CPER_ACA_REG_CONFIG_HI = 9, CPER_ACA_REG_IPID_LO = 10, CPER_ACA_REG_IPID_HI = 11, CPER_ACA_REG_SYND_LO = 12, CPER_ACA_REG_SYND_HI = 13, CPER_ACA_REG_COUNT = 32 }

#[repr(C, packed)]
pub struct cper_timestamp { pub seconds: u8, pub minutes: u8, pub hours: u8, pub flag: u8, pub day: u8, pub month: u8, pub year: u8, pub century: u8 }

#[repr(C, packed)]
pub struct cper_hdr { pub signature: [std::ffi::c_char; 4], pub revision: u16, pub signature_end: u32, pub sec_cnt: u16, pub error_severity: cper_error_severity, pub valid_bits: cper_hdr_valid, pub record_length: u32, pub timestamp: cper_timestamp, pub platform_id: [std::ffi::c_char; 16], pub partition_id: guid_t, pub creator_id: [std::ffi::c_char; 16], pub notify_type: guid_t, pub record_id: [std::ffi::c_char; 8], pub flags: u32, pub persistence_info: u64, pub reserved: [u8; 12] }
#[repr(C)] pub union cper_hdr_valid { pub valid_bits: cper_hdr_valid_bits, pub valid_mask: u32 }
#[repr(C)] pub struct cper_hdr_valid_bits { pub platform_id: u32, pub timestamp: u32, pub partition_id: u32, pub reserved: u32 }

#[repr(C, packed)]
pub struct cper_sec_desc { pub sec_offset: u32, pub sec_length: u32, pub revision_minor: u8, pub revision_major: u8, pub valid_bits: cper_sec_valid, pub reserved: u8, pub flag_bits: cper_sec_flags, pub sec_type: guid_t, pub fru_id: [std::ffi::c_char; 16], pub severity: cper_error_severity, pub fru_text: [std::ffi::c_char; 20] }
#[repr(C)] pub union cper_sec_valid { pub valid_bits: cper_sec_valid_bits, pub valid_mask: u8 }
#[repr(C)] pub struct cper_sec_valid_bits { pub fru_id: u8, pub fru_text: u8, pub reserved: u8 }
#[repr(C)] pub union cper_sec_flags { pub flag_bits: cper_sec_flag_bits, pub flag_mask: u32 }
#[repr(C)] pub struct cper_sec_flag_bits { pub primary: u32, pub reserved1: u32, pub exceed_err_threshold: u32, pub latent_err: u32, pub reserved2: u32 }

#[repr(C, packed)] pub struct cper_sec_nonstd_err_hdr { pub valid_bits: cper_nonstd_hdr_valid, pub apic_id: u64, pub fw_id: [std::ffi::c_char; 48] }
#[repr(C)] pub union cper_nonstd_hdr_valid { pub valid_bits: cper_nonstd_hdr_valid_bits, pub valid_mask: u64 }
#[repr(C)] pub struct cper_nonstd_hdr_valid_bits { pub apic_id: u64, pub fw_id: u64, pub err_info_cnt: u64, pub err_context_cnt: u64 }
#[repr(C, packed)] pub struct cper_sec_nonstd_err_info { pub error_type: guid_t, pub valid_bits: cper_nonstd_info_valid, pub ms_chk_bits: cper_ms_chk, pub target_addr_id: u64, pub req_id: u64, pub resp_id: u64, pub instr_ptr: u64 }
#[repr(C)] pub union cper_nonstd_info_valid { pub valid_bits: cper_nonstd_info_valid_bits, pub valid_mask: u64 }
#[repr(C)] pub struct cper_nonstd_info_valid_bits { pub ms_chk: u64, pub target_addr_id: u64, pub req_id: u64, pub resp_id: u64, pub instr_ptr: u64, pub reserved: u64 }
#[repr(C)] pub union cper_ms_chk { pub ms_chk_bits: cper_ms_chk_bits, pub ms_chk_mask: u64 }
#[repr(C)] pub struct cper_ms_chk_bits { pub err_type_valid: u64, pub pcc_valid: u64, pub uncorr_valid: u64, pub precise_ip_valid: u64, pub restartable_ip_valid: u64, pub overflow_valid: u64, pub reserved1: u64, pub err_type: u64, pub pcc: u64, pub uncorr: u64, pub precised_ip: u64, pub restartable_ip: u64, pub overflow: u64, pub reserved2: u64 }
#[repr(C, packed)] pub struct cper_sec_nonstd_err_ctx { pub reg_ctx_type: u16, pub reg_arr_size: u16, pub msr_addr: u32, pub mm_reg_addr: u64, pub reg_dump: [u32; 32] }
#[repr(C, packed)] pub struct cper_sec_nonstd_err { pub hdr: cper_sec_nonstd_err_hdr, pub info: cper_sec_nonstd_err_info, pub ctx: cper_sec_nonstd_err_ctx }

#[repr(C, packed)] pub struct cper_sec_crashdump_hdr { pub reserved1: u64, pub reserved2: u64, pub fw_id: [std::ffi::c_char; 48], pub reserved3: [u64; 8] }
#[repr(C, packed)] pub struct cper_sec_crashdump_reg_data { pub status_lo: u32, pub status_hi: u32, pub addr_lo: u32, pub addr_hi: u32, pub ipid_lo: u32, pub ipid_hi: u32, pub synd_lo: u32, pub synd_hi: u32 }
#[repr(C, packed)] pub struct cper_sec_crashdump_body_fatal { pub reg_ctx_type: u16, pub reg_arr_size: u16, pub reserved1: u32, pub reserved2: u64, pub data: cper_sec_crashdump_reg_data }
#[repr(C, packed)] pub struct cper_sec_crashdump_body_boot { pub reg_ctx_type: u16, pub reg_arr_size: u16, pub reserved1: u32, pub reserved2: u64, pub msg: [u64; CPER_MAX_OAM_COUNT] }
#[repr(C, packed)] pub struct cper_sec_crashdump_fatal { pub hdr: cper_sec_crashdump_hdr, pub body: cper_sec_crashdump_body_fatal }
#[repr(C, packed)] pub struct cper_sec_crashdump_boot { pub hdr: cper_sec_crashdump_hdr, pub body: cper_sec_crashdump_body_boot }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
