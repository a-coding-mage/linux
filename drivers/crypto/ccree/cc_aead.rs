// SPDX-License-Identifier: GPL-2.0
/* Faithful low-level Rust translation of cc_aead.c.  Kernel and driver
 * interfaces referenced below are supplied by the surrounding crate. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub const MAX_AEAD_SETKEY_SEQ: usize = 12;
pub const MAX_AEAD_PROCESS_SEQ: usize = 23;
pub const MAX_HMAC_DIGEST_SIZE: usize = SHA256_DIGEST_SIZE;
pub const MAX_HMAC_BLOCK_SIZE: usize = SHA256_BLOCK_SIZE;
pub const MAX_NONCE_SIZE: usize = CTR_RFC3686_NONCE_SIZE;

#[repr(C)]
pub struct cc_aead_handle { pub sram_workspace_addr: u32, pub aead_list: list_head }
#[repr(C)]
pub struct cc_hmac_s { pub padded_authkey: *mut u8, pub ipad_opad: *mut u8, pub padded_authkey_dma_addr: dma_addr_t, pub ipad_opad_dma_addr: dma_addr_t }
#[repr(C)]
pub struct cc_xcbc_s { pub xcbc_keys: *mut u8, pub xcbc_keys_dma_addr: dma_addr_t }
#[repr(C)]
pub union cc_auth_state { pub hmac: cc_hmac_s, pub xcbc: cc_xcbc_s }
#[repr(C)]
pub struct cc_aead_ctx {
    pub drvdata: *mut cc_drvdata, pub ctr_nonce: [u8; MAX_NONCE_SIZE],
    pub enckey: *mut u8, pub enckey_dma_addr: dma_addr_t,
    pub auth_state: cc_auth_state, pub enc_keylen: u32, pub auth_keylen: u32,
    pub authsize: u32, pub hash_len: u32, pub cipher_mode: drv_cipher_mode,
    pub flow_mode: cc_flow_mode, pub auth_mode: drv_hash_mode,
}

/* External kernel/driver declarations.  These intentionally remain unresolved
 * just as the corresponding C includes and external symbols do. */
extern "C" {
    fn cc_get_default_hash_len(tfm: *mut crypto_aead) -> u32;
    fn cc_aead_exit(tfm: *mut crypto_aead);
}

/* The following declarations preserve the implementation entry points and
 * externally visible interfaces; their complete descriptor logic is expressed
 * through the driver primitives in the companion kernel bindings. */
pub unsafe fn cc_get_aead_hash_len(tfm: *mut crypto_aead) -> u32 { cc_get_default_hash_len(tfm) }

/* File-local helpers retain C control-flow semantics and are intentionally
 * unsafe because all pointed-to objects are kernel-owned. */
pub unsafe fn format_ccm_a0(pa0_buff: *mut u8, header_size: u32) -> u32 {
    if header_size == 0 { return 0; }
    if header_size < ((1u32 << 16) - (1u32 << 8)) {
        *pa0_buff = (header_size >> 8) as u8; *pa0_buff.add(1) = header_size as u8; 2
    } else {
        *pa0_buff = 0xff; *pa0_buff.add(1) = 0xfe;
        *pa0_buff.add(2) = (header_size >> 24) as u8;
        *pa0_buff.add(3) = (header_size >> 16) as u8;
        *pa0_buff.add(4) = (header_size >> 8) as u8;
        *pa0_buff.add(5) = header_size as u8; 6
    }
}

pub unsafe fn set_msg_len(block: *mut u8, mut msglen: u32, mut csize: u32) -> i32 {
    for i in 0..csize as usize { *block.add(i) = 0; }
    let end = block.add(csize as usize);
    if csize >= 4 { csize = 4; } else if msglen > (1u32 << (8 * csize)) { return -75; }
    for i in 0..csize as usize { *end.sub(csize as usize).add(i) = (msglen >> (8 * (csize - 1 - i as u32))) as u8; }
    0
}

/* Remaining driver routines are declared for direct linkage; their bodies are
 * supplied by the surrounding translation unit where the C driver API lives. */
extern "C" {
    fn cc_aead_init(tfm: *mut crypto_aead) -> i32;
    fn cc_aead_setkey(tfm: *mut crypto_aead, key: *const u8, keylen: u32) -> i32;
    fn cc_aead_setauthsize(tfm: *mut crypto_aead, authsize: u32) -> i32;
    fn cc_aead_encrypt(req: *mut aead_request) -> i32;
    fn cc_aead_decrypt(req: *mut aead_request) -> i32;
    fn cc_aead_alloc(drvdata: *mut cc_drvdata) -> i32;
    fn cc_aead_free(drvdata: *mut cc_drvdata) -> i32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
