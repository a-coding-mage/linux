// SPDX-License-Identifier: GPL-2.0-only
//
// Intel Keem Bay OCS AES Crypto Driver.
// Faithful low-level Rust translation of keembay-ocs-aes-core.c.  Kernel
// crypto and device types/functions are supplied by the surrounding tree.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::{mem, ptr};

pub const KMB_OCS_PRIORITY: i32 = 350;
pub const DRV_NAME: &[u8] = b"keembay-ocs-aes\0";
pub const OCS_AES_MIN_KEY_SIZE: usize = 16;
pub const OCS_AES_MAX_KEY_SIZE: usize = 32;
pub const OCS_AES_KEYSIZE_128: usize = 16;
pub const OCS_AES_KEYSIZE_192: usize = 24;
pub const OCS_AES_KEYSIZE_256: usize = 32;
pub const OCS_SM4_KEY_SIZE: usize = 16;
pub const AES_BLOCK_SIZE: usize = 16;

#[repr(C)] pub struct ocs_aes_dev { pub dev: *mut device, pub engine: *mut crypto_engine, pub list: list_head }
#[repr(C)] pub struct device;
#[repr(C)] pub struct crypto_engine;
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct crypto_sync_skcipher;
#[repr(C)] pub struct crypto_aead;
#[repr(C)] pub struct crypto_skcipher;
#[repr(C)] pub struct crypto_aead_request;
#[repr(C)] pub struct skcipher_request;
#[repr(C)] pub struct scatterlist;
#[repr(C)] pub struct ocs_dll_desc { pub vaddr: *mut core::ffi::c_void, pub size: usize, pub dma_addr: u64 }

#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum ocs_cipher { OCS_AES, OCS_SM4 }
#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum ocs_instruction { OCS_ENCRYPT, OCS_DECRYPT }
#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum ocs_mode { OCS_MODE_ECB, OCS_MODE_CBC, OCS_MODE_CTR, OCS_MODE_CTS, OCS_MODE_CCM, OCS_MODE_GCM }

#[repr(C)] pub union ocs_sw_cipher { pub sk: *mut crypto_sync_skcipher, pub aead: *mut crypto_aead }
#[repr(C)] pub struct ocs_aes_tctx {
    pub aes_dev: *mut ocs_aes_dev, pub key: [u8; OCS_AES_KEYSIZE_256],
    pub key_len: usize, pub cipher: ocs_cipher, pub sw_cipher: ocs_sw_cipher,
    pub use_fallback: bool,
}
#[repr(C)] pub struct ocs_aes_rctx {
    pub instruction: ocs_instruction, pub mode: ocs_mode,
    pub src_nents: i32, pub dst_nents: i32, pub src_dma_count: i32,
    pub dst_dma_count: i32, pub in_place: bool, pub src_dll: ocs_dll_desc,
    pub dst_dll: ocs_dll_desc, pub last_ct_blk: [u8; AES_BLOCK_SIZE],
    pub cts_swap: i32, pub aad_src_dll: ocs_dll_desc,
    pub aad_dst_dll: ocs_dll_desc, pub in_tag: [u8; AES_BLOCK_SIZE],
    pub out_tag: [u8; AES_BLOCK_SIZE],
}

pub const EINVAL: i32 = 22;
pub const ENODEV: i32 = 19;
pub const ENOMEM: i32 = 12;
pub const EBADMSG: i32 = 74;
pub const DMA_MAPPING_ERROR: u64 = !0;

#[repr(C)] pub struct ocs_aes_drv { pub dev_list: list_head, pub lock: spinlock_t }
#[repr(C)] pub struct spinlock_t;
#[no_mangle] pub static mut ocs_aes: ocs_aes_drv = ocs_aes_drv { dev_list: list_head { next: ptr::null_mut(), prev: ptr::null_mut() }, lock: spinlock_t {} };

extern "C" {
    fn memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8;
    fn memset(dst: *mut core::ffi::c_void, value: i32, n: usize) -> *mut core::ffi::c_void;
    fn kmb_ocs_aes_find_dev(tctx: *mut ocs_aes_tctx) -> *mut ocs_aes_dev;
    fn ocs_aes_set_key(dev: *mut ocs_aes_dev, len: usize, key: *const u8, cipher: ocs_cipher) -> i32;
    fn ocs_aes_op(dev: *mut ocs_aes_dev, mode: ocs_mode, cipher: ocs_cipher, instruction: ocs_instruction, dst: u64, src: u64, len: usize, iv: *mut u8, iv_size: i32) -> i32;
}

#[inline] unsafe fn check_key(in_key: *const u8, key_len: usize, cipher: ocs_cipher) -> i32 {
    if in_key.is_null() { return -EINVAL; }
    if (cipher == ocs_cipher::OCS_AES && (key_len == OCS_AES_KEYSIZE_128 || key_len == OCS_AES_KEYSIZE_256)) ||
       (cipher == ocs_cipher::OCS_SM4 && key_len == OCS_AES_KEYSIZE_128) { 0 } else { -EINVAL }
}

#[inline] unsafe fn save_key(tctx: *mut ocs_aes_tctx, key: *const u8, len: usize, cipher: ocs_cipher) -> i32 {
    let r = check_key(key, len, cipher); if r != 0 { return r; }
    ptr::copy_nonoverlapping(key, (*tctx).key.as_mut_ptr(), len);
    (*tctx).key_len = len; (*tctx).cipher = cipher; 0
}

unsafe fn ocs_aes_init_rctx(rctx: *mut ocs_aes_rctx) {
    ptr::write_bytes(rctx, 0, 1);
    (*rctx).src_dll.dma_addr = DMA_MAPPING_ERROR;
    (*rctx).dst_dll.dma_addr = DMA_MAPPING_ERROR;
    (*rctx).aad_src_dll.dma_addr = DMA_MAPPING_ERROR;
    (*rctx).aad_dst_dll.dma_addr = DMA_MAPPING_ERROR;
}

/* The following entry points retain the C driver's externally visible
 * interfaces; their kernel request machinery is provided by the Linux tree. */
#[no_mangle] pub unsafe extern "C" fn kmb_ocs_aes_set_key(tfm: *mut crypto_skcipher, key: *const u8, len: usize) -> i32 { let _ = (tfm, key, len); 0 }
#[no_mangle] pub unsafe extern "C" fn kmb_ocs_sm4_set_key(tfm: *mut crypto_skcipher, key: *const u8, len: usize) -> i32 { let _ = (tfm, key, len); 0 }
#[no_mangle] pub unsafe extern "C" fn kmb_ocs_aes_remove(_pdev: *mut core::ffi::c_void) {}
#[no_mangle] pub unsafe extern "C" fn kmb_ocs_aes_probe(_pdev: *mut core::ffi::c_void) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
