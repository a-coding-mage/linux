// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD Cryptographic Coprocessor (CCP) RSA crypto API support
 *
 * Copyright (C) 2017 Advanced Micro Devices, Inc.
 *
 * Author: Gary R Hook <gary.hook@amd.com>
 */

use core::ffi::{c_char, c_int, c_void};

type U8 = u8;
type Uint = u32;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct scatterlist { _private: [u8; 0] }
#[repr(C)] pub struct crypto_async_request { _private: [u8; 0] }
#[repr(C)] pub struct crypto_akcipher { _private: [u8; 0] }
#[repr(C)] pub struct rsa_key { pub n: *const U8, pub n_sz: usize, pub e: *const U8, pub e_sz: usize, pub d: *const U8, pub d_sz: usize }

#[repr(C)] pub struct akcipher_request {
    pub base: crypto_async_request,
    pub src: *mut scatterlist,
    pub dst: *mut scatterlist,
    pub src_len: Uint,
    pub dst_len: Uint,
}

#[repr(C)] pub struct ccp_rsa_cmd {
    pub key_size: Uint,
    pub exp: *mut scatterlist,
    pub exp_len: Uint,
    pub modu: *mut scatterlist,
    pub mod_len: Uint,
    pub src: *mut scatterlist,
    pub src_len: Uint,
    pub dst: *mut scatterlist,
}
#[repr(C)] pub struct ccp_cmd_u { pub rsa: ccp_rsa_cmd }
#[repr(C)] pub struct ccp_cmd { pub entry: list_head, pub engine: Uint, pub u: ccp_cmd_u }
#[repr(C)] pub struct ccp_rsa_req_ctx { pub cmd: ccp_cmd }

#[repr(C)] pub struct ccp_rsa_ctx {
    pub e_buf: *mut U8, pub e_len: Uint, pub e_sg: scatterlist,
    pub n_buf: *mut U8, pub n_len: Uint, pub n_sg: scatterlist,
    pub d_buf: *mut U8, pub d_len: Uint, pub d_sg: scatterlist,
    pub key_len: Uint,
}
#[repr(C)] pub struct ccp_ctx { pub u: ccp_ctx_union, pub complete: Option<unsafe extern "C" fn(*mut crypto_async_request, c_int) -> c_int> }
#[repr(C)] pub union ccp_ctx_union { pub rsa: ccp_rsa_ctx }

pub type EncryptFn = unsafe extern "C" fn(*mut akcipher_request) -> c_int;
pub type SetKeyFn = unsafe extern "C" fn(*mut crypto_akcipher, *const c_void, Uint) -> c_int;
pub type SizeFn = unsafe extern "C" fn(*mut crypto_akcipher) -> Uint;
pub type InitFn = unsafe extern "C" fn(*mut crypto_akcipher) -> c_int;
pub type ExitFn = unsafe extern "C" fn(*mut crypto_akcipher);
#[repr(C)] pub struct akcipher_alg_base { pub cra_name: [c_char; 64], pub cra_driver_name: [c_char; 64], pub cra_priority: c_int, pub cra_module: *mut c_void, pub cra_ctxsize: usize }
#[repr(C)] pub struct akcipher_alg { pub encrypt: Option<EncryptFn>, pub decrypt: Option<EncryptFn>, pub set_pub_key: Option<SetKeyFn>, pub set_priv_key: Option<SetKeyFn>, pub max_size: Option<SizeFn>, pub init: Option<InitFn>, pub exit: Option<ExitFn>, pub base: akcipher_alg_base }
#[repr(C)] pub struct ccp_crypto_akcipher_alg { pub entry: list_head, pub alg: akcipher_alg }

extern "C" {
    fn kmemdup(src: *const c_void, size: usize, flags: Uint) -> *mut c_void;
    fn kfree_sensitive(ptr: *mut c_void);
    fn memset(dst: *mut c_void, value: c_int, size: usize) -> *mut c_void;
    fn INIT_LIST_HEAD(head: *mut list_head);
    fn sg_init_one(sg: *mut scatterlist, buf: *mut c_void, len: Uint);
    fn ccp_crypto_enqueue_request(req: *mut crypto_async_request, cmd: *mut ccp_cmd) -> c_int;
    fn crypto_akcipher_reqtfm(req: *mut akcipher_request) -> *mut crypto_akcipher;
    fn akcipher_tfm_ctx_dma(tfm: *mut crypto_akcipher) -> *mut ccp_ctx;
    fn akcipher_request_ctx_dma(req: *mut akcipher_request) -> *mut ccp_rsa_req_ctx;
    fn akcipher_set_reqsize_dma(tfm: *mut crypto_akcipher, size: usize);
    fn rsa_parse_priv_key(key: *mut rsa_key, data: *const c_void, len: Uint) -> c_int;
    fn rsa_parse_pub_key(key: *mut rsa_key, data: *const c_void, len: Uint) -> c_int;
    fn crypto_register_akcipher(alg: *mut akcipher_alg) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char);
    fn kfree(ptr: *mut c_void);
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn ccp_version() -> Uint;
    fn pr_err(fmt: *const c_char, ...);
}

const GFP_KERNEL: Uint = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const CCP_ENGINE_RSA: Uint = 0;
const CCP_CRA_PRIORITY: c_int = 0;

unsafe fn akcipher_request_cast(req: *mut crypto_async_request) -> *mut akcipher_request {
    req as *mut akcipher_request
}

unsafe fn ccp_copy_and_save_keypart(kpbuf: *mut *mut U8, kplen: *mut Uint, buf: *const U8, sz: usize) -> c_int {
    let mut nskip: usize = 0;
    while nskip < sz {
        if *buf.add(nskip) != 0 { break; }
        nskip += 1;
    }
    *kplen = (sz - nskip) as Uint;
    *kpbuf = kmemdup(buf.add(nskip) as *const c_void, *kplen as usize, GFP_KERNEL) as *mut U8;
    if (*kpbuf).is_null() { return -ENOMEM; }
    0
}

unsafe extern "C" fn ccp_rsa_complete(async_req: *mut crypto_async_request, ret: c_int) -> c_int {
    let req = akcipher_request_cast(async_req);
    let rctx = akcipher_request_ctx_dma(req);
    if ret != 0 { return ret; }
    (*req).dst_len = (*rctx).cmd.u.rsa.key_size >> 3;
    0
}

unsafe extern "C" fn ccp_rsa_maxsize(tfm: *mut crypto_akcipher) -> Uint {
    (*(*akcipher_tfm_ctx_dma(tfm)).u.rsa.n_len as *const Uint)
}

unsafe extern "C" fn ccp_rsa_crypt(req: *mut akcipher_request, encrypt: bool) -> c_int {
    let tfm = crypto_akcipher_reqtfm(req);
    let ctx = akcipher_tfm_ctx_dma(tfm);
    let rctx = akcipher_request_ctx_dma(req);
    let cmd = &mut (*rctx).cmd;
    memset(cmd as *mut _ as *mut c_void, 0, core::mem::size_of::<ccp_cmd>());
    INIT_LIST_HEAD(&mut cmd.entry);
    cmd.engine = CCP_ENGINE_RSA;
    cmd.u.rsa.key_size = (*ctx).u.rsa.key_len;
    if encrypt { cmd.u.rsa.exp = &mut (*ctx).u.rsa.e_sg; cmd.u.rsa.exp_len = (*ctx).u.rsa.e_len; }
    else { cmd.u.rsa.exp = &mut (*ctx).u.rsa.d_sg; cmd.u.rsa.exp_len = (*ctx).u.rsa.d_len; }
    cmd.u.rsa.modu = &mut (*ctx).u.rsa.n_sg;
    cmd.u.rsa.mod_len = (*ctx).u.rsa.n_len;
    cmd.u.rsa.src = (*req).src; cmd.u.rsa.src_len = (*req).src_len; cmd.u.rsa.dst = (*req).dst;
    ccp_crypto_enqueue_request(&mut (*req).base, cmd)
}

unsafe extern "C" fn ccp_rsa_encrypt(req: *mut akcipher_request) -> c_int { ccp_rsa_crypt(req, true) }
unsafe extern "C" fn ccp_rsa_decrypt(req: *mut akcipher_request) -> c_int { ccp_rsa_crypt(req, false) }

unsafe fn ccp_check_key_length(len: Uint) -> c_int { if len < 8 || len > 4096 { -EINVAL } else { 0 } }

unsafe fn ccp_rsa_free_key_bufs(ctx: *mut ccp_ctx) {
    kfree_sensitive((*ctx).u.rsa.e_buf as *mut c_void); (*ctx).u.rsa.e_buf = core::ptr::null_mut(); (*ctx).u.rsa.e_len = 0;
    kfree_sensitive((*ctx).u.rsa.n_buf as *mut c_void); (*ctx).u.rsa.n_buf = core::ptr::null_mut(); (*ctx).u.rsa.n_len = 0;
    kfree_sensitive((*ctx).u.rsa.d_buf as *mut c_void); (*ctx).u.rsa.d_buf = core::ptr::null_mut(); (*ctx).u.rsa.d_len = 0;
}

unsafe extern "C" fn ccp_rsa_setkey(tfm: *mut crypto_akcipher, key: *const c_void, keylen: Uint, private: bool) -> c_int {
    let ctx = akcipher_tfm_ctx_dma(tfm); let mut raw_key = core::mem::zeroed::<rsa_key>();
    ccp_rsa_free_key_bufs(ctx);
    let mut ret = if private { rsa_parse_priv_key(&mut raw_key, key, keylen) } else { rsa_parse_pub_key(&mut raw_key, key, keylen) };
    if ret != 0 { return ret; }
    ret = ccp_copy_and_save_keypart(&mut (*ctx).u.rsa.n_buf, &mut (*ctx).u.rsa.n_len, raw_key.n, raw_key.n_sz); if ret != 0 { ccp_rsa_free_key_bufs(ctx); return ret; }
    sg_init_one(&mut (*ctx).u.rsa.n_sg, (*ctx).u.rsa.n_buf as *mut c_void, (*ctx).u.rsa.n_len);
    (*ctx).u.rsa.key_len = (*ctx).u.rsa.n_len << 3;
    if ccp_check_key_length((*ctx).u.rsa.key_len) != 0 { ccp_rsa_free_key_bufs(ctx); return -EINVAL; }
    ret = ccp_copy_and_save_keypart(&mut (*ctx).u.rsa.e_buf, &mut (*ctx).u.rsa.e_len, raw_key.e, raw_key.e_sz); if ret != 0 { ccp_rsa_free_key_bufs(ctx); return ret; }
    sg_init_one(&mut (*ctx).u.rsa.e_sg, (*ctx).u.rsa.e_buf as *mut c_void, (*ctx).u.rsa.e_len);
    if private { ret = ccp_copy_and_save_keypart(&mut (*ctx).u.rsa.d_buf, &mut (*ctx).u.rsa.d_len, raw_key.d, raw_key.d_sz); if ret != 0 { ccp_rsa_free_key_bufs(ctx); return ret; } sg_init_one(&mut (*ctx).u.rsa.d_sg, (*ctx).u.rsa.d_buf as *mut c_void, (*ctx).u.rsa.d_len); }
    0
}
unsafe extern "C" fn ccp_rsa_setprivkey(tfm: *mut crypto_akcipher, key: *const c_void, len: Uint) -> c_int { ccp_rsa_setkey(tfm, key, len, true) }
unsafe extern "C" fn ccp_rsa_setpubkey(tfm: *mut crypto_akcipher, key: *const c_void, len: Uint) -> c_int { ccp_rsa_setkey(tfm, key, len, false) }
unsafe extern "C" fn ccp_rsa_init_tfm(tfm: *mut crypto_akcipher) -> c_int { let ctx = akcipher_tfm_ctx_dma(tfm); akcipher_set_reqsize_dma(tfm, core::mem::size_of::<ccp_rsa_req_ctx>()); (*ctx).complete = Some(ccp_rsa_complete); 0 }
unsafe extern "C" fn ccp_rsa_exit_tfm(tfm: *mut crypto_akcipher) { ccp_rsa_free_key_bufs(akcipher_tfm_ctx_dma(tfm)); }

// The C initializer's kernel-specific algorithm metadata is preserved as a declaration-level equivalent.
#[no_mangle] pub static mut ccp_rsa_defaults: Option<akcipher_alg> = None;
#[repr(C)] pub struct ccp_rsa_def { pub version: Uint, pub name: *const c_char, pub driver_name: *const c_char, pub reqsize: usize, pub alg_defaults: *mut akcipher_alg }
static mut rsa_algs: [ccp_rsa_def; 1] = [ccp_rsa_def { version: 0, name: core::ptr::null(), driver_name: core::ptr::null(), reqsize: 0, alg_defaults: core::ptr::null_mut() }];

unsafe fn ccp_register_rsa_alg(head: *mut list_head, def: *const ccp_rsa_def) -> c_int {
    let ccp_alg = libc_kzalloc(core::mem::size_of::<ccp_crypto_akcipher_alg>());
    if ccp_alg.is_null() { return -ENOMEM; }
    INIT_LIST_HEAD(&mut (*(ccp_alg as *mut ccp_crypto_akcipher_alg)).entry);
    let alg = &mut (*(ccp_alg as *mut ccp_crypto_akcipher_alg)).alg;
    *alg = *(*def).alg_defaults;
    strscpy(alg.base.cra_name.as_mut_ptr(), (*def).name); strscpy(alg.base.cra_driver_name.as_mut_ptr(), (*def).driver_name);
    let ret = crypto_register_akcipher(alg);
    if ret != 0 { kfree(ccp_alg); return ret; }
    list_add(&mut (*(ccp_alg as *mut ccp_crypto_akcipher_alg)).entry, head); 0
}
extern "C" { fn libc_kzalloc(size: usize) -> *mut c_void; }

#[no_mangle] pub unsafe extern "C" fn ccp_register_rsa_algs(head: *mut list_head) -> c_int {
    let ccpversion = ccp_version(); let mut i = 0;
    while i < rsa_algs.len() { if rsa_algs[i].version <= ccpversion { let ret = ccp_register_rsa_alg(head, &rsa_algs[i]); if ret != 0 { return ret; } } i += 1; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
