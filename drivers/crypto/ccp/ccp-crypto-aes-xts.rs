// SPDX-License-Identifier: GPL-2.0-only
/* AMD Cryptographic Coprocessor (CCP) AES XTS crypto API support */

use core::ffi::c_void;

// Kernel-provided types, constants, and functions from the included headers.
extern "C" {
    fn ccp_version() -> u32;
    fn xts_verify_key(tfm: *mut crypto_skcipher, key: *const u8, key_len: u32) -> i32;
    fn crypto_skcipher_setkey(tfm: *mut crypto_skcipher, key: *const u8, key_len: u32) -> i32;
    fn crypto_skcipher_encrypt(req: *mut skcipher_request) -> i32;
    fn crypto_skcipher_decrypt(req: *mut skcipher_request) -> i32;
    fn ccp_crypto_enqueue_request(req: *mut crypto_async_request, cmd: *mut ccp_cmd) -> i32;
    fn crypto_alloc_skcipher(name: *const i8, type_: u32, mask: u32) -> *mut crypto_skcipher;
    fn crypto_free_skcipher(tfm: *mut crypto_skcipher);
    fn crypto_skcipher_reqsize(tfm: *mut crypto_skcipher) -> usize;
    fn crypto_skcipher_set_reqsize_dma(tfm: *mut crypto_skcipher, size: usize);
    fn crypto_register_skcipher(alg: *mut skcipher_alg) -> i32;
    fn crypto_dma_padding() -> usize;
    fn kfree(ptr: *mut c_void);
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn sg_init_one(sg: *mut scatterlist, buf: *mut c_void, len: usize);
    fn memcpy(dst: *mut c_void, src: *const c_void, len: usize) -> *mut c_void;
    fn memset(dst: *mut c_void, value: i32, len: usize) -> *mut c_void;
    fn strscpy(dst: *mut i8, src: *const i8, size: usize) -> isize;
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_add(new: *mut list_head, head: *mut list_head);
}

const AES_BLOCK_SIZE: usize = 16;
const AES_KEYSIZE_128: u32 = 16;
const AES_KEYSIZE_256: u32 = 32;
const AES_MIN_KEY_SIZE: u32 = 16;
const AES_MAX_KEY_SIZE: u32 = 32;
const CCP_XTS_AES_UNIT_SIZE__LAST: u32 = 5;
const CCP_ENGINE_XTS_AES_128: u32 = 0;
const CCP_AES_TYPE_128: u32 = 0;
const CCP_AES_ACTION_ENCRYPT: u32 = 0;
const CCP_AES_ACTION_DECRYPT: u32 = 1;
const CRYPTO_ALG_NEED_FALLBACK: u32 = 0;
const CCP_CRA_PRIORITY: u32 = 0;

#[repr(C)]
pub struct ccp_aes_xts_def { pub name: *const i8, pub drv_name: *const i8 }
#[repr(C)]
pub struct ccp_unit_size_map { pub size: u32, pub value: u32 }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct scatterlist { _private: [u8; 0] }
#[repr(C)] pub struct crypto_async_request { pub flags: u32, pub complete: Option<unsafe extern "C" fn(*mut crypto_async_request, i32) -> i32>, pub data: *mut c_void }
#[repr(C)] pub struct crypto_skcipher { _private: [u8; 0] }
#[repr(C)] pub struct crypto_tfm_base { pub cra_name: [i8; 64], pub cra_driver_name: [i8; 64], pub cra_flags: u32, pub cra_blocksize: u32, pub cra_ctxsize: usize, pub cra_priority: u32, pub cra_module: *mut c_void }
#[repr(C)] pub struct skcipher_alg { pub base: crypto_tfm_base, pub setkey: Option<unsafe extern "C" fn(*mut crypto_skcipher, *const u8, u32) -> i32>, pub encrypt: Option<unsafe extern "C" fn(*mut skcipher_request) -> i32>, pub decrypt: Option<unsafe extern "C" fn(*mut skcipher_request) -> i32>, pub min_keysize: u32, pub max_keysize: u32, pub ivsize: u32, pub init: Option<unsafe extern "C" fn(*mut crypto_skcipher) -> i32>, pub exit: Option<unsafe extern "C" fn(*mut crypto_skcipher)> }
#[repr(C)] pub struct skcipher_request { pub base: crypto_async_request, pub src: *mut scatterlist, pub dst: *mut scatterlist, pub cryptlen: usize, pub iv: *mut u8 }
#[repr(C)] pub struct ccp_aes_req_ctx { pub iv: [u8; AES_BLOCK_SIZE], pub iv_sg: scatterlist, pub cmd: ccp_cmd, pub fallback_req: skcipher_request }
#[repr(C)] pub struct ccp_ctx { pub complete: Option<unsafe extern "C" fn(*mut crypto_async_request, i32) -> i32>, pub u: ccp_ctx_u }
#[repr(C)] pub union ccp_ctx_u { pub aes: ccp_aes_ctx }
#[repr(C)] pub struct ccp_aes_ctx { pub key: [u8; 64], pub key_len: u32, pub key_sg: scatterlist, pub tfm_skcipher: *mut crypto_skcipher }
#[repr(C)] pub struct ccp_crypto_skcipher_alg { pub entry: list_head, pub alg: skcipher_alg }
#[repr(C)] pub struct ccp_cmd { pub entry: list_head, pub engine: u32, pub xts: ccp_xts_cmd }
#[repr(C)] pub struct ccp_xts_cmd { pub type_: u32, pub action: u32, pub unit_size: u32, pub key: *mut scatterlist, pub key_len: u32, pub iv: *mut scatterlist, pub iv_len: u32, pub src: *mut scatterlist, pub src_len: usize, pub dst: *mut scatterlist }

static AES_XTS_ALGS: [ccp_aes_xts_def; 1] = [ccp_aes_xts_def { name: b"xts(aes)\0".as_ptr() as *const i8, drv_name: b"xts-aes-ccp\0".as_ptr() as *const i8 }];
static mut XTS_UNIT_SIZES: [ccp_unit_size_map; 5] = [
    ccp_unit_size_map { size: 16, value: 0 }, ccp_unit_size_map { size: 512, value: 1 },
    ccp_unit_size_map { size: 1024, value: 2 }, ccp_unit_size_map { size: 2048, value: 3 },
    ccp_unit_size_map { size: 4096, value: 4 },
];

unsafe extern "C" fn ccp_aes_xts_complete(async_req: *mut crypto_async_request, ret: i32) -> i32 { if ret != 0 { return ret; } let req = async_req as *mut skcipher_request; let rctx = ((*req).base.data) as *mut ccp_aes_req_ctx; memcpy((*req).iv as *mut c_void, (*rctx).iv.as_ptr() as *const c_void, AES_BLOCK_SIZE); 0 }
unsafe extern "C" fn ccp_aes_xts_setkey(tfm: *mut crypto_skcipher, key: *const u8, key_len: u32) -> i32 { let ctx = tfm as *mut ccp_ctx; let ret = xts_verify_key(tfm, key, key_len); if ret != 0 { return ret; } let aes = &mut (*ctx).u.aes; match key_len { 32 => { memcpy(aes.key.as_mut_ptr() as *mut c_void, key as *const c_void, key_len as usize); }, 64 => { if ccp_version() > 0x0300 { memcpy(aes.key.as_mut_ptr() as *mut c_void, key as *const c_void, key_len as usize); } }, _ => {} } aes.key_len = key_len / 2; sg_init_one(&mut aes.key_sg, aes.key.as_mut_ptr() as *mut c_void, key_len as usize); crypto_skcipher_setkey(aes.tfm_skcipher, key, key_len) }
unsafe extern "C" fn ccp_aes_xts_crypt(req: *mut skcipher_request, encrypt: u32) -> i32 { let tfm = req as *mut crypto_skcipher; let ctx = tfm as *mut ccp_ctx; let rctx = (*req).base.data as *mut ccp_aes_req_ctx; let aes = &mut (*ctx).u.aes; if aes.key_len == 0 || (*req).iv.is_null() { return -22; } let mut unit_size = CCP_XTS_AES_UNIT_SIZE__LAST; for unit in 0..5 { if (*req).cryptlen == XTS_UNIT_SIZES[unit].size as usize { unit_size = unit as u32; break; } } let fallback = unit_size == CCP_XTS_AES_UNIT_SIZE__LAST || (ccp_version() < 0x0500 && aes.key_len != AES_KEYSIZE_128) || (aes.key_len != AES_KEYSIZE_128 && aes.key_len != AES_KEYSIZE_256); if fallback { return if encrypt != 0 { crypto_skcipher_encrypt(&mut (*rctx).fallback_req) } else { crypto_skcipher_decrypt(&mut (*rctx).fallback_req) }; } memcpy((*rctx).iv.as_mut_ptr() as *mut c_void, (*req).iv as *const c_void, AES_BLOCK_SIZE); sg_init_one(&mut (*rctx).iv_sg, (*rctx).iv.as_mut_ptr() as *mut c_void, AES_BLOCK_SIZE); memset(&mut (*rctx).cmd as *mut _ as *mut c_void, 0, core::mem::size_of::<ccp_cmd>()); INIT_LIST_HEAD(&mut (*rctx).cmd.entry); (*rctx).cmd.engine = CCP_ENGINE_XTS_AES_128; (*rctx).cmd.xts.type_ = CCP_AES_TYPE_128; (*rctx).cmd.xts.action = if encrypt != 0 { CCP_AES_ACTION_ENCRYPT } else { CCP_AES_ACTION_DECRYPT }; (*rctx).cmd.xts.unit_size = unit_size; (*rctx).cmd.xts.key = &mut aes.key_sg; (*rctx).cmd.xts.key_len = aes.key_len; (*rctx).cmd.xts.iv = &mut (*rctx).iv_sg; (*rctx).cmd.xts.iv_len = AES_BLOCK_SIZE as u32; (*rctx).cmd.xts.src = (*req).src; (*rctx).cmd.xts.src_len = (*req).cryptlen; (*rctx).cmd.xts.dst = (*req).dst; ccp_crypto_enqueue_request(&mut (*req).base, &mut (*rctx).cmd) }
unsafe extern "C" fn ccp_aes_xts_encrypt(req: *mut skcipher_request) -> i32 { ccp_aes_xts_crypt(req, 1) }
unsafe extern "C" fn ccp_aes_xts_decrypt(req: *mut skcipher_request) -> i32 { ccp_aes_xts_crypt(req, 0) }
unsafe extern "C" fn ccp_aes_xts_init_tfm(tfm: *mut crypto_skcipher) -> i32 { let ctx = tfm as *mut ccp_ctx; (*ctx).complete = Some(ccp_aes_xts_complete); (*ctx).u.aes.key_len = 0; let fallback = crypto_alloc_skcipher(b"xts(aes)\0".as_ptr() as *const i8, 0, CRYPTO_ALG_NEED_FALLBACK); if fallback.is_null() { return -12; } (*ctx).u.aes.tfm_skcipher = fallback; crypto_skcipher_set_reqsize_dma(tfm, core::mem::size_of::<ccp_aes_req_ctx>() + crypto_skcipher_reqsize(fallback)); 0 }
unsafe extern "C" fn ccp_aes_xts_exit_tfm(tfm: *mut crypto_skcipher) { crypto_free_skcipher((*((tfm as *mut ccp_ctx))).u.aes.tfm_skcipher); }
pub unsafe extern "C" fn ccp_register_aes_xts_algs(head: *mut list_head) -> i32 { let alg = kzalloc(core::mem::size_of::<ccp_crypto_skcipher_alg>(), 0) as *mut ccp_crypto_skcipher_alg; if alg.is_null() { return -12; } INIT_LIST_HEAD(&mut (*alg).entry); (*alg).alg.setkey = Some(ccp_aes_xts_setkey); (*alg).alg.encrypt = Some(ccp_aes_xts_encrypt); (*alg).alg.decrypt = Some(ccp_aes_xts_decrypt); (*alg).alg.init = Some(ccp_aes_xts_init_tfm); (*alg).alg.exit = Some(ccp_aes_xts_exit_tfm); let ret = crypto_register_skcipher(&mut (*alg).alg); if ret != 0 { kfree(alg as *mut c_void); return ret; } list_add(&mut (*alg).entry, head); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
