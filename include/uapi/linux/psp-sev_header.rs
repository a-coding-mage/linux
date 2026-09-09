/* SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note */
/*
 * Userspace interface for AMD Secure Encrypted Virtualization (SEV)
 * platform management commands.
 *
 * Copyright (C) 2016-2017 Advanced Micro Devices, Inc.
 *
 * Author: Brijesh Singh <brijesh.singh@amd.com>
 *
 * SEV API specification is available at: https://developer.amd.com/sev/
 */

/* Dependency: Linux __u8/__u32/__u64 types. */

/** SEV platform commands */
pub const SEV_FACTORY_RESET: u32 = 0;
pub const SEV_PLATFORM_STATUS: u32 = 1;
pub const SEV_PEK_GEN: u32 = 2;
pub const SEV_PEK_CSR: u32 = 3;
pub const SEV_PDH_GEN: u32 = 4;
pub const SEV_PDH_CERT_EXPORT: u32 = 5;
pub const SEV_PEK_CERT_IMPORT: u32 = 6;
pub const SEV_GET_ID: u32 = 7; /* This command is deprecated, use SEV_GET_ID2 */
pub const SEV_GET_ID2: u32 = 8;
pub const SNP_PLATFORM_STATUS: u32 = 9;
pub const SNP_COMMIT: u32 = 10;
pub const SNP_SET_CONFIG: u32 = 11;
pub const SNP_VLEK_LOAD: u32 = 12;
pub const SEV_MAX: u32 = 13;

/** SEV Firmware status code */
pub type sev_ret_code = i32;
pub const SEV_RET_NO_FW_CALL: sev_ret_code = -1;
pub const SEV_RET_SUCCESS: sev_ret_code = 0;
pub const SEV_RET_INVALID_PLATFORM_STATE: sev_ret_code = 0x0001;
pub const SEV_RET_INVALID_GUEST_STATE: sev_ret_code = 0x0002;
pub const SEV_RET_INAVLID_CONFIG: sev_ret_code = 0x0003;
pub const SEV_RET_INVALID_CONFIG: sev_ret_code = SEV_RET_INAVLID_CONFIG;
pub const SEV_RET_INVALID_LEN: sev_ret_code = 0x0004;
pub const SEV_RET_ALREADY_OWNED: sev_ret_code = 0x0005;
pub const SEV_RET_INVALID_CERTIFICATE: sev_ret_code = 0x0006;
pub const SEV_RET_POLICY_FAILURE: sev_ret_code = 0x0007;
pub const SEV_RET_INACTIVE: sev_ret_code = 0x0008;
pub const SEV_RET_INVALID_ADDRESS: sev_ret_code = 0x0009;
pub const SEV_RET_BAD_SIGNATURE: sev_ret_code = 0x000A;
pub const SEV_RET_BAD_MEASUREMENT: sev_ret_code = 0x000B;
pub const SEV_RET_ASID_OWNED: sev_ret_code = 0x000C;
pub const SEV_RET_INVALID_ASID: sev_ret_code = 0x000D;
pub const SEV_RET_WBINVD_REQUIRED: sev_ret_code = 0x000E;
pub const SEV_RET_DFFLUSH_REQUIRED: sev_ret_code = 0x000F;
pub const SEV_RET_INVALID_GUEST: sev_ret_code = 0x0010;
pub const SEV_RET_INVALID_COMMAND: sev_ret_code = 0x0011;
pub const SEV_RET_ACTIVE: sev_ret_code = 0x0012;
pub const SEV_RET_HWSEV_RET_PLATFORM: sev_ret_code = 0x0013;
pub const SEV_RET_HWSEV_RET_UNSAFE: sev_ret_code = 0x0014;
pub const SEV_RET_UNSUPPORTED: sev_ret_code = 0x0015;
pub const SEV_RET_INVALID_PARAM: sev_ret_code = 0x0016;
pub const SEV_RET_RESOURCE_LIMIT: sev_ret_code = 0x0017;
pub const SEV_RET_SECURE_DATA_INVALID: sev_ret_code = 0x0018;
pub const SEV_RET_INVALID_PAGE_SIZE: sev_ret_code = 0x0019;
pub const SEV_RET_INVALID_PAGE_STATE: sev_ret_code = 0x001A;
pub const SEV_RET_INVALID_MDATA_ENTRY: sev_ret_code = 0x001B;
pub const SEV_RET_INVALID_PAGE_OWNER: sev_ret_code = 0x001C;
pub const SEV_RET_AEAD_OFLOW: sev_ret_code = 0x001D;
pub const SEV_RET_EXIT_RING_BUFFER: sev_ret_code = 0x001F;
pub const SEV_RET_RMP_INIT_REQUIRED: sev_ret_code = 0x0020;
pub const SEV_RET_BAD_SVN: sev_ret_code = 0x0021;
pub const SEV_RET_BAD_VERSION: sev_ret_code = 0x0022;
pub const SEV_RET_SHUTDOWN_REQUIRED: sev_ret_code = 0x0023;
pub const SEV_RET_UPDATE_FAILED: sev_ret_code = 0x0024;
pub const SEV_RET_RESTORE_REQUIRED: sev_ret_code = 0x0025;
pub const SEV_RET_RMP_INITIALIZATION_FAILED: sev_ret_code = 0x0026;
pub const SEV_RET_INVALID_KEY: sev_ret_code = 0x0027;
pub const SEV_RET_SHUTDOWN_INCOMPLETE: sev_ret_code = 0x0028;
pub const SEV_RET_INCORRECT_BUFFER_LENGTH: sev_ret_code = 0x0030;
pub const SEV_RET_EXPAND_BUFFER_LENGTH_REQUEST: sev_ret_code = 0x0031;
pub const SEV_RET_SPDM_REQUEST: sev_ret_code = 0x0032;
pub const SEV_RET_SPDM_ERROR: sev_ret_code = 0x0033;
pub const SEV_RET_SEV_STATUS_ERR_IN_DEV_CONN: sev_ret_code = 0x0035;
pub const SEV_RET_SEV_STATUS_INVALID_DEV_CTX: sev_ret_code = 0x0036;
pub const SEV_RET_SEV_STATUS_INVALID_TDI_CTX: sev_ret_code = 0x0037;
pub const SEV_RET_SEV_STATUS_INVALID_TDI: sev_ret_code = 0x0038;
pub const SEV_RET_SEV_STATUS_RECLAIM_REQUIRED: sev_ret_code = 0x0039;
pub const SEV_RET_IN_USE: sev_ret_code = 0x003A;
pub const SEV_RET_SEV_STATUS_INVALID_DEV_STATE: sev_ret_code = 0x003B;
pub const SEV_RET_SEV_STATUS_INVALID_TDI_STATE: sev_ret_code = 0x003C;
pub const SEV_RET_SEV_STATUS_DEV_CERT_CHANGED: sev_ret_code = 0x003D;
pub const SEV_RET_SEV_STATUS_RESYNC_REQ: sev_ret_code = 0x003E;
pub const SEV_RET_SEV_STATUS_RESPONSE_TOO_LARGE: sev_ret_code = 0x003F;
pub const SEV_RET_MAX: sev_ret_code = 0x0040;

#[repr(C, packed)]
pub struct sev_user_data_status { pub api_major: u8, pub api_minor: u8, pub state: u8, pub flags: u32, pub build: u8, pub guest_count: u32 }
pub const SEV_STATUS_FLAGS_CONFIG_ES: u32 = 0x0100;

#[repr(C, packed)]
pub struct sev_user_data_pek_csr { pub address: u64, pub length: u32 }
#[repr(C, packed)]
pub struct sev_user_data_pek_cert_import { pub pek_cert_address: u64, pub pek_cert_len: u32, pub oca_cert_address: u64, pub oca_cert_len: u32 }
#[repr(C, packed)]
pub struct sev_user_data_pdh_cert_export { pub pdh_cert_address: u64, pub pdh_cert_len: u32, pub cert_chain_address: u64, pub cert_chain_len: u32 }
#[repr(C, packed)]
pub struct sev_user_data_get_id { pub socket1: [u8; 64], pub socket2: [u8; 64] }
#[repr(C, packed)]
pub struct sev_user_data_get_id2 { pub address: u64, pub length: u32 }

#[repr(C, packed)]
pub struct sev_user_data_snp_status {
    pub api_major: u8, pub api_minor: u8, pub state: u8, pub is_rmp_initialized: u8,
    pub rsvd: u8, pub build_id: u32, pub mask_chip_id: u32, pub mask_chip_key: u32,
    pub vlek_en: u32, pub feature_info: u32, pub rapl_dis: u32,
    pub ciphertext_hiding_cap: u32, pub ciphertext_hiding_en: u32, pub rsvd1: u32,
    pub guest_count: u32, pub current_tcb_version: u64, pub reported_tcb_version: u64,
}
#[repr(C, packed)]
pub struct sev_user_data_snp_config {
    pub reported_tcb: u64, pub mask_chip_id: u32, pub mask_chip_key: u32,
    pub rsvd: u32, pub rsvd1: [u8; 52],
}
#[repr(C, packed)]
pub struct sev_user_data_snp_vlek_load { pub len: u32, pub vlek_wrapped_version: u8, pub rsvd: [u8; 3], pub vlek_wrapped_address: u64 }
#[repr(C, packed)]
pub struct sev_user_data_snp_wrapped_vlek_hashstick { pub data: [u8; 432] }
#[repr(C, packed)]
pub struct sev_issue_cmd { pub cmd: u32, pub data: u64, pub error: u32 }

pub const SEV_IOC_TYPE: u8 = b'S';
/* _IOWR(SEV_IOC_TYPE, 0x0, struct sev_issue_cmd); ioctl encoding is supplied by the platform ABI. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
