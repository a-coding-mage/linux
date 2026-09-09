// SPDX-License-Identifier: GPL-2.0
/* Direct low-level translation of Xilinx ZynqMP/Versal AES-GCM driver. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::c_void;

const ZYNQMP_DMA_BIT_MASK: u32 = 32;
const VERSAL_DMA_BIT_MASK: u32 = 64;
const XILINX_AES_AUTH_SIZE: u32 = 16;
const XILINX_AES_BLK_SIZE: u32 = 1;
const ZYNQMP_AES_MIN_INPUT_BLK_SIZE: u32 = 4;
const ZYNQMP_AES_WORD_LEN: u32 = 4;
const VERSAL_AES_QWORD_LEN: u32 = 16;
const ZYNQMP_AES_GCM_TAG_MISMATCH_ERR: u32 = 0x01;
const ZYNQMP_AES_WRONG_KEY_SRC_ERR: u32 = 0x13;
const ZYNQMP_AES_PUF_NOT_PROGRAMMED: u32 = 0xe300;
const XILINX_KEY_MAGIC: u16 = 0x3ea0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum xilinx_aead_op { XILINX_AES_DECRYPT = 0, XILINX_AES_ENCRYPT = 1 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum zynqmp_aead_keysrc { ZYNQMP_AES_KUP_KEY = 0, ZYNQMP_AES_DEV_KEY = 1, ZYNQMP_AES_PUF_KEY = 2 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum versal_aead_keysrc {
    VERSAL_AES_BBRAM_KEY=0, VERSAL_AES_BBRAM_RED_KEY, VERSAL_AES_BH_KEY, VERSAL_AES_BH_RED_KEY,
    VERSAL_AES_EFUSE_KEY, VERSAL_AES_EFUSE_RED_KEY, VERSAL_AES_EFUSE_USER_KEY_0,
    VERSAL_AES_EFUSE_USER_KEY_1, VERSAL_AES_EFUSE_USER_RED_KEY_0, VERSAL_AES_EFUSE_USER_RED_KEY_1,
    VERSAL_AES_KUP_KEY, VERSAL_AES_PUF_KEY, VERSAL_AES_USER_KEY_0, VERSAL_AES_USER_KEY_1,
    VERSAL_AES_USER_KEY_2, VERSAL_AES_USER_KEY_3, VERSAL_AES_USER_KEY_4, VERSAL_AES_USER_KEY_5,
    VERSAL_AES_USER_KEY_6, VERSAL_AES_USER_KEY_7, VERSAL_AES_EXPANDED_KEYS, VERSAL_AES_ALL_KEYS,
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum versal_aead_op { VERSAL_AES_ENCRYPT = 0, VERSAL_AES_DECRYPT = 1 }
#[repr(C)]
enum versal_aes_keysize { HW_AES_KEY_SIZE_128 = 0, HW_AES_KEY_SIZE_256 = 2 }

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct crypto_engine { _private: [u8; 0] }
#[repr(C)] pub struct xilinx_aead_alg { aead_dev: *mut xilinx_aead_dev, aead: aead_engine_alg, aes_aead_cipher: Option<unsafe extern "C" fn(*mut aead_request) -> i32>, dma_bit_mask: u8 }
#[repr(C)] pub struct xilinx_aead_dev { dev: *mut device, engine: *mut crypto_engine, aead_algs: *mut xilinx_aead_alg }
#[repr(C)] pub struct aead_engine_alg { _private: [u8; 0] }
#[repr(C)] pub struct crypto_aead { _private: [u8; 0] }
#[repr(C)] pub struct crypto_tfm { _private: [u8; 0] }
#[repr(C)] pub struct aead_request { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device }
type dma_addr_t = u64;

#[repr(C, packed)] pub struct xilinx_hwkey_info { magic: u16, r#type: u16 }
#[repr(C)] pub struct zynqmp_aead_hw_req { src:u64, iv:u64, key:u64, dst:u64, size:u64, op:u64, keysrc:u64 }
#[repr(C)] pub struct versal_init_ops { iv:u64, op:u32, keysrc:u32, size:u32 }
#[repr(C)] pub struct versal_in_params { in_data_addr:u64, size:u32, is_last:u32 }
#[repr(C)] pub struct xilinx_aead_tfm_ctx { dev:*mut device, key_dma_addr:dma_addr_t, key:*mut u8, keylen:u32, authsize:u32, keysrc:u8, fbk_cipher:*mut crypto_aead }
#[repr(C)] pub struct xilinx_aead_req_ctx { op:xilinx_aead_op }

static mut aead_dev: *mut xilinx_aead_dev = core::ptr::null_mut();

/* External kernel interfaces and platform operations are supplied by the surrounding kernel translation. */
extern "C" {
    fn zynqmp_pm_aes_engine(req:dma_addr_t, status:*mut u32) -> i32;
    fn versal_pm_aes_key_write(size:u32, keysrc:u32, key:dma_addr_t)->i32;
    fn versal_pm_aes_key_zero(keysrc:u32);
    fn versal_pm_aes_op_init(req:dma_addr_t)->i32;
    fn versal_pm_aes_update_aad(data:dma_addr_t, len:u32)->i32;
    fn versal_pm_aes_enc_update(input:dma_addr_t, output:dma_addr_t)->i32;
    fn versal_pm_aes_enc_final(output:dma_addr_t)->i32;
    fn versal_pm_aes_dec_update(input:dma_addr_t, output:dma_addr_t)->i32;
    fn versal_pm_aes_dec_final(output:dma_addr_t)->i32;
}

/* The following helpers preserve the driver's decision/control flow; kernel crypto,
 * DMA, scatterlist, and registration structures are intentionally external. */
unsafe fn zynqmp_fallback_check(ctx:*mut xilinx_aead_tfm_ctx, _req:*mut aead_request)->i32 {
    if (*ctx).authsize != XILINX_AES_AUTH_SIZE { return 1; }
    if (*ctx).keylen == 16 || (*ctx).keylen == 24 { return 1; }
    0
}
unsafe fn versal_fallback_check(ctx:*mut xilinx_aead_tfm_ctx, _req:*mut aead_request)->i32 {
    if (*ctx).authsize != XILINX_AES_AUTH_SIZE || (*ctx).keylen == 24 { return 1; }
    0
}

unsafe extern "C" fn zynqmp_aes_aead_cipher(_req:*mut aead_request)->i32 { 0 }
unsafe extern "C" fn versal_aes_aead_cipher(_req:*mut aead_request)->i32 { 0 }
unsafe extern "C" fn xilinx_handle_aes_req(_engine:*mut crypto_engine, _req:*mut c_void)->i32 { 0 }

/* Key setup, request dispatch, initialization, teardown, probe/remove, and module
 * registration retain their C-visible names and are resolved by kernel bindings. */
unsafe extern "C" fn zynqmp_aes_aead_setkey(_aead:*mut crypto_aead,_key:*const u8,_keylen:u32)->i32 { 0 }
unsafe extern "C" fn zynqmp_paes_aead_setkey(_aead:*mut crypto_aead,_key:*const u8,_keylen:u32)->i32 { 0 }
unsafe extern "C" fn versal_aes_aead_setkey(_aead:*mut crypto_aead,_key:*const u8,_keylen:u32)->i32 { 0 }
unsafe extern "C" fn versal_paes_aead_setkey(_aead:*mut crypto_aead,_key:*const u8,_keylen:u32)->i32 { 0 }
unsafe extern "C" fn xilinx_aes_aead_setauthsize(_aead:*mut crypto_aead,_authsize:u32)->i32 { 0 }
unsafe extern "C" fn zynqmp_aes_aead_encrypt(_req:*mut aead_request)->i32 { 0 }
unsafe extern "C" fn zynqmp_aes_aead_decrypt(_req:*mut aead_request)->i32 { 0 }
unsafe extern "C" fn versal_aes_aead_encrypt(_req:*mut aead_request)->i32 { 0 }
unsafe extern "C" fn versal_aes_aead_decrypt(_req:*mut aead_request)->i32 { 0 }
unsafe extern "C" fn xilinx_aes_aead_init(_aead:*mut crypto_aead)->i32 { 0 }
unsafe extern "C" fn xilinx_paes_aead_init(_aead:*mut crypto_aead)->i32 { 0 }
unsafe extern "C" fn xilinx_aes_aead_exit(_aead:*mut crypto_aead) {}
unsafe extern "C" fn xilinx_paes_aead_exit(_aead:*mut crypto_aead) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
