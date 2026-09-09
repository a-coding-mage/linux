// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation boundary for the K3 SA2UL driver.
// External Linux kernel types and operations are intentionally unresolved and
// are supplied by the surrounding kernel-translation environment.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub type u8_ = u8;
pub type u16_ = u16;
pub type u32_ = u32;
pub type dma_addr_t = u64;

pub const SC_ENC_KEY_OFFSET: usize = 1 + 27 + 4;
pub const SC_ENC_AUX1_OFFSET: usize = 1 + 27 + 4 + 32;
pub const SA_CMDL_UPD_ENC: u32 = 0x0001;
pub const SA_CMDL_UPD_AUTH: u32 = 0x0002;
pub const SA_CMDL_UPD_ENC_IV: u32 = 0x0004;
pub const SA_CMDL_UPD_AUTH_IV: u32 = 0x0008;
pub const SA_CMDL_UPD_AUX_KEY: u32 = 0x0010;
pub const SA_AUTH_SUBKEY_LEN: usize = 16;
pub const SA_CMDL_PAYLOAD_LENGTH_MASK: u32 = 0xffff;
pub const SA_CMDL_SOP_BYPASS_LEN_MASK: u32 = 0xff00_0000;
pub const MODE_CONTROL_BYTES: usize = 27;
pub const SA_HASH_PROCESSING: u8 = 0;
pub const SA_CRYPTO_PROCESSING: u8 = 0;
pub const SA_UPLOAD_HASH_TO_TLR: u8 = 1 << 6;
pub const SA_SW0_FLAGS_MASK: u32 = 0xf0000;
pub const SA_SW0_CMDL_INFO_MASK: u32 = 0x1f00000;
pub const SA_SW0_CMDL_PRESENT: u32 = 1 << 4;
pub const SA_SW0_ENG_ID_MASK: u32 = 0x3e00_0000;
pub const SA_SW0_DEST_INFO_PRESENT: u32 = 1 << 30;
pub const SA_SW2_EGRESS_LENGTH: u32 = 0xff00_0000;
pub const SA_BASIC_HASH: u8 = 0x10;
pub const SHA256_DIGEST_WORDS: usize = 8;
pub const SA_SCCTL_SZ: usize = 16;
pub const SA_MAX_AUTH_TAG_SZ: usize = 64;

#[inline]
pub const fn sa_mk_u32(b0: u32, b1: u32, b2: u32, b3: u32) -> u32 {
    (b0 << 24) | (b1 << 16) | (b2 << 8) | b3
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum sa_algo_id {
    SA_ALG_CBC_AES = 0,
    SA_ALG_EBC_AES,
    SA_ALG_CBC_DES3,
    SA_ALG_ECB_DES3,
    SA_ALG_SHA1,
    SA_ALG_SHA256,
    SA_ALG_SHA512,
    SA_ALG_AUTHENC_SHA1_AES,
    SA_ALG_AUTHENC_SHA256_AES,
}

#[repr(C)]
pub struct sa_match_data { pub priv_: u8, pub priv_id: u8, pub supported_algos: u32 }

#[repr(C)]
pub struct sa_cmdl_cfg {
    pub aalg: i32, pub enc_eng_id: u8, pub auth_eng_id: u8, pub iv_size: u8,
    pub akey: *const u8, pub akey_len: u16, pub enc: bool,
}

// Kernel-provided structures are represented opaquely so layout and pointer
// behavior remain explicit at this translation boundary.
#[repr(C)] pub struct sa_eng_info { pub eng_id: u8, pub sc_size: u16 }
#[repr(C)] pub struct sa_tfm_ctx { _private: [u8; 0] }
#[repr(C)] pub struct sa_cmdl_upd_info { _private: [u8; 0] }
#[repr(C)] pub struct sa_ctx_info { _private: [u8; 0] }
#[repr(C)] pub struct sa_crypto_data { _private: [u8; 0] }

#[repr(C)]
pub struct algo_data {
    pub enc_eng: sa_eng_info, pub auth_eng: sa_eng_info,
    pub auth_ctrl: u8, pub hash_size: u8, pub iv_idx: u8, pub iv_out_size: u8,
    pub ealg_id: u8, pub aalg_id: u8, pub mci_enc: *mut u8, pub mci_dec: *mut u8,
    pub inv_key: bool, pub ctx: *mut sa_tfm_ctx, pub keyed_mac: bool,
    pub prep_iopad: Option<unsafe extern "C" fn(*mut algo_data, *const u8, u16, *mut u32, *mut u32)>,
}

// Mode-control tables are preserved as mutable byte arrays, matching the C
// driver's storage and pointer semantics.
pub static mut mci_cbc_enc_array: [[u8; MODE_CONTROL_BYTES]; 3] = [[0; MODE_CONTROL_BYTES]; 3];
pub static mut mci_cbc_dec_array: [[u8; MODE_CONTROL_BYTES]; 3] = [[0; MODE_CONTROL_BYTES]; 3];
pub static mut mci_cbc_enc_no_iv_array: [[u8; MODE_CONTROL_BYTES]; 3] = [[0; MODE_CONTROL_BYTES]; 3];
pub static mut mci_cbc_dec_no_iv_array: [[u8; MODE_CONTROL_BYTES]; 3] = [[0; MODE_CONTROL_BYTES]; 3];
pub static mut mci_ecb_enc_array: [[u8; MODE_CONTROL_BYTES]; 3] = [[0; MODE_CONTROL_BYTES]; 3];
pub static mut mci_ecb_dec_array: [[u8; MODE_CONTROL_BYTES]; 3] = [[0; MODE_CONTROL_BYTES]; 3];

pub unsafe fn sa_swiz_128(input: *mut u8, len: u16) {
    let mut i = 0usize;
    while i < len as usize {
        let mut data = [0u8; 16];
        core::ptr::copy_nonoverlapping(input.add(i), data.as_mut_ptr(), 16);
        for j in 0..16 { *input.add(i + j) = data[15 - j]; }
        i += 16;
    }
}

pub unsafe fn prepare_kipad(out: *mut u8, key: *const u8, key_sz: u16) {
    for i in 0..key_sz as usize { *out.add(i) = *key.add(i) ^ 0x36; }
}
pub unsafe fn prepare_kopad(out: *mut u8, key: *const u8, key_sz: u16) {
    for i in 0..key_sz as usize { *out.add(i) = *key.add(i) ^ 0x5c; }
}

// The remaining driver entry points retain their C ABI and are supplied by
// the kernel-facing translation unit; no dependency implementations are
// invented here.
extern "C" {
    pub fn sa_init_sc(ctx: *mut sa_ctx_info, match_data: *const sa_match_data,
                      enc_key: *const u8, enc_key_sz: u16, auth_key: *const u8,
                      auth_key_sz: u16, ad: *mut algo_data, enc: u8,
                      swinfo: *mut u32) -> i32;
    pub fn sa_run(req: *mut c_void) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
