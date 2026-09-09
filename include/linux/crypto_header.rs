/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Scatterlist Cryptographic API. */

// Dependencies supplied by the surrounding kernel translation.

pub const CRYPTO_ALG_TYPE_MASK: u32 = 0x0000000f;
pub const CRYPTO_ALG_TYPE_CIPHER: u32 = 0x00000001;
pub const CRYPTO_ALG_TYPE_AEAD: u32 = 0x00000003;
pub const CRYPTO_ALG_TYPE_LSKCIPHER: u32 = 0x00000004;
pub const CRYPTO_ALG_TYPE_SKCIPHER: u32 = 0x00000005;
pub const CRYPTO_ALG_TYPE_AKCIPHER: u32 = 0x00000006;
pub const CRYPTO_ALG_TYPE_SIG: u32 = 0x00000007;
pub const CRYPTO_ALG_TYPE_KPP: u32 = 0x00000008;
pub const CRYPTO_ALG_TYPE_ACOMPRESS: u32 = 0x0000000a;
pub const CRYPTO_ALG_TYPE_SCOMPRESS: u32 = 0x0000000b;
pub const CRYPTO_ALG_TYPE_RNG: u32 = 0x0000000c;
pub const CRYPTO_ALG_TYPE_HASH: u32 = 0x0000000e;
pub const CRYPTO_ALG_TYPE_SHASH: u32 = 0x0000000e;
pub const CRYPTO_ALG_TYPE_AHASH: u32 = 0x0000000f;
pub const CRYPTO_ALG_TYPE_ACOMPRESS_MASK: u32 = 0x0000000e;
pub const CRYPTO_ALG_LARVAL: u32 = 0x00000010;
pub const CRYPTO_ALG_DEAD: u32 = 0x00000020;
pub const CRYPTO_ALG_DYING: u32 = 0x00000040;
pub const CRYPTO_ALG_ASYNC: u32 = 0x00000080;
pub const CRYPTO_ALG_NEED_FALLBACK: u32 = 0x00000100;
pub const CRYPTO_ALG_DUP_FIRST: u32 = 0x00000200;
pub const CRYPTO_ALG_TESTED: u32 = 0x00000400;
pub const CRYPTO_ALG_INSTANCE: u32 = 0x00000800;
pub const CRYPTO_ALG_KERN_DRIVER_ONLY: u32 = 0x00001000;
pub const CRYPTO_ALG_INTERNAL: u32 = 0x00002000;
pub const CRYPTO_ALG_OPTIONAL_KEY: u32 = 0x00004000;
pub const CRYPTO_NOLOAD: u32 = 0x00008000;
pub const CRYPTO_ALG_ALLOCATES_MEMORY: u32 = 0x00010000;
pub const CRYPTO_ALG_FIPS_INTERNAL: u32 = 0x00020000;
pub const CRYPTO_ALG_REQ_VIRT: u32 = 0x00040000;
pub const CRYPTO_ALG_NO_FALLBACK: u32 = 0x00080000;

pub const CRYPTO_TFM_NEED_KEY: u32 = 0x00000001;
pub const CRYPTO_TFM_REQ_MASK: u32 = 0x000fff00;
pub const CRYPTO_TFM_REQ_FORBID_WEAK_KEYS: u32 = 0x00000100;
pub const CRYPTO_TFM_REQ_MAY_SLEEP: u32 = 0x00000200;
pub const CRYPTO_TFM_REQ_MAY_BACKLOG: u32 = 0x00000400;
pub const CRYPTO_TFM_REQ_ON_STACK: u32 = 0x00000800;
pub const CRYPTO_MAX_ALG_NAME: usize = 128;

pub type CryptoCompletionT = unsafe extern "C" fn(req: *mut core::ffi::c_void, err: i32);

#[repr(C)]
pub struct CryptoAsyncRequest {
    pub list: ListHead,
    pub complete: Option<CryptoCompletionT>,
    pub data: *mut core::ffi::c_void,
    pub tfm: *mut CryptoTfm,
    pub flags: u32,
}

#[repr(C)]
pub struct CipherAlg {
    pub cia_min_keysize: u32,
    pub cia_max_keysize: u32,
    pub cia_setkey: Option<unsafe extern "C" fn(*mut CryptoTfm, *const u8, u32) -> i32>,
    pub cia_encrypt: Option<unsafe extern "C" fn(*mut CryptoTfm, *mut u8, *const u8)>,
    pub cia_decrypt: Option<unsafe extern "C" fn(*mut CryptoTfm, *mut u8, *const u8)>,
}

#[repr(C)]
pub union CryptoAlgUnion {
    pub cipher: CipherAlg,
}

#[repr(C)]
pub struct CryptoAlg {
    pub cra_list: ListHead,
    pub cra_users: ListHead,
    pub cra_flags: u32,
    pub cra_blocksize: u32,
    pub cra_ctxsize: u32,
    pub cra_alignmask: u32,
    pub cra_reqsize: u32,
    pub cra_priority: i32,
    pub cra_refcnt: RefcountT,
    pub cra_name: [core::ffi::c_char; CRYPTO_MAX_ALG_NAME],
    pub cra_driver_name: [core::ffi::c_char; CRYPTO_MAX_ALG_NAME],
    pub cra_type: *const CryptoType,
    pub cra_u: CryptoAlgUnion,
    pub cra_init: Option<unsafe extern "C" fn(*mut CryptoTfm) -> i32>,
    pub cra_exit: Option<unsafe extern "C" fn(*mut CryptoTfm)>,
    pub cra_destroy: Option<unsafe extern "C" fn(*mut CryptoAlg)>,
    pub cra_module: *mut Module,
}

#[repr(C)]
pub struct CryptoWait {
    pub completion: Completion,
    pub err: i32,
}

extern "C" {
    pub fn crypto_req_done(req: *mut core::ffi::c_void, err: i32);
    pub fn crypto_has_alg(name: *const core::ffi::c_char, type_: u32, mask: u32) -> i32;
    pub fn crypto_alloc_base(name: *const core::ffi::c_char, type_: u32, mask: u32) -> *mut CryptoTfm;
    pub fn crypto_destroy_tfm(mem: *mut core::ffi::c_void, tfm: *mut CryptoTfm);
    pub fn wait_for_completion(completion: *mut Completion);
    pub fn reinit_completion(completion: *mut Completion);
    pub fn init_completion(completion: *mut Completion);
    pub fn crypto_request_clone(req: *mut CryptoAsyncRequest, total: usize, gfp: GfpT) -> *mut CryptoAsyncRequest;
}

#[repr(C)]
pub struct CryptoTfm {
    pub crt_flags: u32,
    pub node: i32,
    pub fb: *mut CryptoTfm,
    pub exit: Option<unsafe extern "C" fn(*mut CryptoTfm)>,
    pub __crt_alg: *mut CryptoAlg,
    pub __crt_ctx: [u8; 0],
}

pub unsafe fn crypto_free_tfm(tfm: *mut CryptoTfm) {
    crypto_destroy_tfm(tfm as *mut core::ffi::c_void, tfm);
}

pub unsafe fn crypto_tfm_alg_name(tfm: *mut CryptoTfm) -> *const core::ffi::c_char { (*(*tfm).__crt_alg).cra_name.as_ptr() }
pub unsafe fn crypto_tfm_alg_driver_name(tfm: *mut CryptoTfm) -> *const core::ffi::c_char { (*(*tfm).__crt_alg).cra_driver_name.as_ptr() }
pub unsafe fn crypto_tfm_alg_blocksize(tfm: *mut CryptoTfm) -> u32 { (*(*tfm).__crt_alg).cra_blocksize }
pub unsafe fn crypto_tfm_alg_alignmask(tfm: *mut CryptoTfm) -> u32 { (*(*tfm).__crt_alg).cra_alignmask }
pub unsafe fn crypto_tfm_alg_reqsize(tfm: *mut CryptoTfm) -> u32 { (*(*tfm).__crt_alg).cra_reqsize }
pub unsafe fn crypto_tfm_get_flags(tfm: *mut CryptoTfm) -> u32 { (*tfm).crt_flags }
pub unsafe fn crypto_tfm_set_flags(tfm: *mut CryptoTfm, flags: u32) { (*tfm).crt_flags |= flags; }
pub unsafe fn crypto_tfm_clear_flags(tfm: *mut CryptoTfm, flags: u32) { (*tfm).crt_flags &= !flags; }
pub unsafe fn crypto_tfm_is_async(tfm: *mut CryptoTfm) -> bool { (*(*tfm).__crt_alg).cra_flags & CRYPTO_ALG_ASYNC != 0 }
pub unsafe fn crypto_req_on_stack(req: *mut CryptoAsyncRequest) -> bool { (*req).flags & CRYPTO_TFM_REQ_ON_STACK != 0 }

pub unsafe fn crypto_request_set_callback(req: *mut CryptoAsyncRequest, flags: u32, compl: Option<CryptoCompletionT>, data: *mut core::ffi::c_void) {
    (*req).complete = compl;
    (*req).data = data;
    (*req).flags &= CRYPTO_TFM_REQ_ON_STACK;
    (*req).flags |= flags & !CRYPTO_TFM_REQ_ON_STACK;
}

pub unsafe fn crypto_request_set_tfm(req: *mut CryptoAsyncRequest, tfm: *mut CryptoTfm) {
    (*req).tfm = tfm;
    (*req).flags &= !CRYPTO_TFM_REQ_ON_STACK;
}

pub unsafe fn crypto_stack_request_init(req: *mut CryptoAsyncRequest, tfm: *mut CryptoTfm) {
    (*req).flags = 0;
    crypto_request_set_tfm(req, tfm);
    (*req).flags |= CRYPTO_TFM_REQ_ON_STACK;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
