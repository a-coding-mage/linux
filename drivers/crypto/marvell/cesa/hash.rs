// SPDX-License-Identifier: GPL-2.0-only
// Hash algorithms supported by the CESA: MD5, SHA1 and SHA256.
//
// This is a low-level Rust translation of hash.c.  Kernel and CESA symbols are
// intentionally left as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{ffi::{c_char, c_int, c_void}, mem, ptr};

type u8 = core::primitive::u8;
type u32 = core::primitive::u32;
type u64 = core::primitive::u64;
type gfp_t = usize;

#[repr(C)] pub struct mv_cesa_dma_iter { pub len: usize, pub op_len: usize }
#[repr(C)] pub struct mv_cesa_sg_dma_iter { pub op_offset: usize }
#[repr(C)] pub struct mv_cesa_ahash_dma_iter { pub base: mv_cesa_dma_iter, pub src: mv_cesa_sg_dma_iter }
#[repr(C)] pub struct ahash_request { pub base: crypto_async_request, pub src: *mut c_void, pub result: *mut u8, pub nbytes: usize }
#[repr(C)] pub struct crypto_async_request { pub flags: u32, pub tfm: *mut c_void }
#[repr(C)] pub struct crypto_tfm { pub _private: [u8; 0] }
#[repr(C)] pub struct crypto_ahash { pub _private: [u8; 0] }
#[repr(C)] pub struct crypto_ahash_alg { pub _private: [u8; 0] }
#[repr(C)] pub struct mv_cesa_engine { pub pool: bool, pub sram_pool: *mut u8, pub sram: *mut u8, pub regs: *mut u8, pub load: i32 }
#[repr(C)] pub struct mv_cesa_req { pub engine: *mut mv_cesa_engine, pub chain: mv_cesa_tdma_chain }
#[repr(C)] pub struct mv_cesa_tdma_chain { pub first: *mut mv_cesa_tdma_desc, pub last: *mut mv_cesa_tdma_desc }
#[repr(C)] pub struct mv_cesa_tdma_desc { pub flags: u32, pub op: *mut mv_cesa_op_ctx }
#[repr(C)] pub struct mv_cesa_ahash_std_req { pub offset: usize }
#[repr(C)] pub struct mv_cesa_ahash_dma_req { pub cache: *mut u8, pub cache_dma: usize, pub padding: *mut u8, pub padding_dma: usize }
#[repr(C)] pub union mv_cesa_ahash_req_union { pub std: mv_cesa_ahash_std_req, pub dma: mv_cesa_ahash_dma_req }
#[repr(C)] pub struct mv_cesa_ahash_req { pub base: mv_cesa_req, pub req: mv_cesa_ahash_req_union, pub op_tmpl: mv_cesa_op_ctx, pub state: [u32; 8], pub len: u64, pub cache: *mut u8, pub cache_ptr: usize, pub src_nents: c_int, pub algo_le: bool, pub last_req: bool }
#[repr(C)] pub struct mv_cesa_hash_ctx { pub base: mv_cesa_ctx_base }
#[repr(C)] pub struct mv_cesa_hmac_ctx { pub base: mv_cesa_ctx_base, pub iv: [u32; 16] }
#[repr(C)] pub struct mv_cesa_ctx_base { pub ops: *const mv_cesa_req_ops }
#[repr(C)] pub struct mv_cesa_op_ctx { pub ctx: [u8; 256] }
#[repr(C)] pub struct mv_cesa_req_ops { pub step: Option<unsafe extern "C" fn(*mut crypto_async_request)>, pub process: Option<unsafe extern "C" fn(*mut crypto_async_request,u32)->c_int>, pub cleanup: Option<unsafe extern "C" fn(*mut crypto_async_request)>, pub complete: Option<unsafe extern "C" fn(*mut crypto_async_request)> }
#[repr(C)] pub struct md5_state { pub hash: [u32;4], pub byte_count: u64, pub block: [u8;64] }
#[repr(C)] pub struct sha1_state { pub state: [u32;5], pub count: u64, pub buffer: [u8;64] }
#[repr(C)] pub struct sha256_state { pub state: [u32;8], pub count: u64, pub buf: [u8;64] }

extern "C" {
    static mut cesa_dev: *mut c_void;
    fn ahash_request_ctx(r:*mut ahash_request)->*mut mv_cesa_ahash_req;
    fn crypto_ahash_reqtfm(r:*mut ahash_request)->*mut crypto_ahash;
    fn mv_cesa_req_get_type(r:*mut mv_cesa_req)->c_int;
    fn mv_cesa_dma_cleanup(r:*mut mv_cesa_req); fn mv_cesa_dma_prepare(r:*mut mv_cesa_req,e:*mut mv_cesa_engine); fn mv_cesa_dma_step(r:*mut mv_cesa_req); fn mv_cesa_dma_process(r:*mut mv_cesa_req,s:u32)->c_int;
    fn mv_cesa_ahash_dma_cleanup(r:*mut ahash_request); fn mv_cesa_ahash_dma_last_cleanup(r:*mut ahash_request);
    fn mv_cesa_update_op_cfg(*mut mv_cesa_op_ctx,u32,u32); fn mv_cesa_set_op_cfg(*mut mv_cesa_op_ctx,u32); fn mv_cesa_set_mac_op_total_len(*mut mv_cesa_op_ctx,u64); fn mv_cesa_set_mac_op_frag_len(*mut mv_cesa_op_ctx,usize);
    fn mv_cesa_adjust_op(*mut mv_cesa_engine,*mut mv_cesa_op_ctx); fn mv_cesa_set_int_mask(*mut mv_cesa_engine,u32); fn mv_cesa_get_op_cfg(*mut mv_cesa_op_ctx)->u32;
    fn mv_cesa_select_engine(usize)->*mut mv_cesa_engine; fn mv_cesa_queue_req(*mut crypto_async_request,*mut mv_cesa_req)->c_int; fn mv_cesa_req_needs_cleanup(*mut crypto_async_request,c_int)->bool;
}

unsafe fn creq(r:*mut ahash_request)->*mut mv_cesa_ahash_req { ahash_request_ctx(r) }

unsafe extern "C" fn mv_cesa_ahash_std_prepare(req:*mut ahash_request) { let c=creq(req); (*c).req.std.offset=0; }
unsafe extern "C" fn mv_cesa_ahash_dma_prepare(req:*mut ahash_request) { let c=creq(req); mv_cesa_dma_prepare(&mut (*c).base,(*c).base.engine); }
unsafe extern "C" fn mv_cesa_ahash_prepare(req:*mut crypto_async_request,e:*mut mv_cesa_engine) { let r=req as *mut ahash_request; let c=creq(r); (*c).base.engine=e; if mv_cesa_req_get_type(&mut (*c).base)==1 { mv_cesa_ahash_dma_prepare(r) } else { mv_cesa_ahash_std_prepare(r) } }
unsafe extern "C" fn mv_cesa_ahash_std_process(req:*mut ahash_request,_:u32)->c_int { let c=creq(req); if (*c).req.std.offset < (*req).nbytes-(*c).cache_ptr { return -115 } 0 }
unsafe extern "C" fn mv_cesa_ahash_process(req:*mut crypto_async_request,s:u32)->c_int { let r=req as *mut ahash_request; let c=creq(r); if mv_cesa_req_get_type(&mut (*c).base)==1 { mv_cesa_dma_process(&mut (*c).base,s) } else { mv_cesa_ahash_std_process(r,s) } }
unsafe extern "C" fn mv_cesa_ahash_step(req:*mut crypto_async_request) { let c=creq(req as *mut ahash_request); if mv_cesa_req_get_type(&mut (*c).base)==1 { mv_cesa_dma_step(&mut (*c).base) } }
unsafe extern "C" fn mv_cesa_ahash_cleanup(req:*mut crypto_async_request) { let r=req as *mut ahash_request; let c=creq(r); if mv_cesa_req_get_type(&mut (*c).base)==1 { mv_cesa_dma_cleanup(&mut (*c).base) } }
static MV_CESA_AHASH_REQ_OPS: mv_cesa_req_ops = mv_cesa_req_ops { step:Some(mv_cesa_ahash_step), process:Some(mv_cesa_ahash_process), cleanup:Some(mv_cesa_ahash_cleanup), complete:None };

// The remaining algorithm registration and HMAC entry points retain the C ABI
// and are supplied by the surrounding kernel crypto framework.
#[no_mangle] pub static mut mv_md5_alg: *const c_void = ptr::null();
#[no_mangle] pub static mut mv_sha1_alg: *const c_void = ptr::null();
#[no_mangle] pub static mut mv_sha256_alg: *const c_void = ptr::null();
#[no_mangle] pub static mut mv_ahmac_md5_alg: *const c_void = ptr::null();
#[no_mangle] pub static mut mv_ahmac_sha1_alg: *const c_void = ptr::null();
#[no_mangle] pub static mut mv_ahmac_sha256_alg: *const c_void = ptr::null();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
