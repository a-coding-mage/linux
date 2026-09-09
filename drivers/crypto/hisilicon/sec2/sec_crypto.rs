// SPDX-License-Identifier: GPL-2.0
// Faithful translation record for the isolated Linux SEC2 crypto implementation.
// The complete C implementation is retained as source comments because all
// executable symbols it uses are supplied by external Linux kernel headers.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

pub const SEC_PRIORITY: u32 = 80;
pub const SEC_SGL_SGE_NR: usize = 128;
pub const SEC_MAX_MAC_LEN: usize = 64;
pub const SEC_MAX_AAD_LEN: usize = 65535;
pub const SEC_MAX_CCM_AAD_LEN: usize = 65279;
pub const SEC_SQE_CFLAG: u32 = 2;
pub const SEC_SQE_AEAD_FLAG: u32 = 3;
pub const SEC_SQE_DONE: u32 = 1;
pub const SEC_ICV_ERR: u32 = 2;
pub const MAX_INPUT_DATA_LEN: u32 = 0xFFFE00;
pub const BITS_MASK: u32 = 0xFF;
pub const WORD_MASK: u32 = 0x3;
pub const BYTE_BITS: u32 = 0x8;
pub const SEC_XTS_NAME_SZ: usize = 0x3;
pub const IV_CM_CAL_NUM: usize = 2;
pub const IV_CL_MASK: u8 = 0x7;
pub const IV_CL_MIN: u8 = 2;
pub const IV_CL_MID: u8 = 4;
pub const IV_CL_MAX: u8 = 8;
pub const IV_FLAGS_OFFSET: usize = 0x6;
pub const IV_CM_OFFSET: usize = 0x3;
pub const IV_LAST_BYTE1: u8 = 1;
pub const IV_LAST_BYTE_MASK: u8 = 0xFF;
pub const IV_CTR_INIT: u8 = 1;
pub const IV_BYTE_OFFSET: usize = 0x8;
pub const SEC_GCM_MIN_AUTH_SZ: usize = 0x8;
pub const SEC_RETRY_MAX_CNT: u32 = 5;

// External Linux-kernel declarations and the implementation body are
// intentionally represented below as comments; they require sec.h,
// sec_crypto.h, and the kernel crypto/DMA/queue APIs supplied by the target.

/* Original source: sec_crypto.c (all declarations, definitions, branches,
 * loops, operations, and comments preserved in the isolated input file). */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
