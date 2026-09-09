// SPDX-License-Identifier: GPL-2.0-or-later
/* Asynchronous Compression operations */

use core::ffi::c_void;

// Types, constants, macros, and functions supplied by the kernel headers are
// intentionally referenced here as external dependencies.
type U8 = u8;
type U32 = u32;
type GfpT = u32;
type CryptoCompletionT = unsafe extern "C" fn(*mut c_void, i32);

const ACOMP_WALK_SLEEP: u32 = 1 << 0;
const ACOMP_WALK_SRC_LINEAR: u32 = 1 << 1;
const ACOMP_WALK_DST_LINEAR: u32 = 1 << 2;

#[repr(C)] pub struct CryptoType { _private: [u8; 0] }
#[repr(C)] pub struct CryptoAlg { pub cra_type: *const CryptoType, pub cra_flags: u32, pub cra_reqsize: usize }
#[repr(C)] pub struct CryptoTfm { pub __crt_alg: *mut CryptoAlg, pub fb: *mut CryptoTfm, pub base: CryptoTfmBase }
#[repr(C)] pub struct CryptoTfmBase { pub exit: Option<unsafe extern "C" fn(*mut CryptoTfm)> }
#[repr(C)] pub struct CryptoAc​omp { pub base: CryptoTfm, pub fb: *mut CryptoAc​omp, pub compress: Option<unsafe extern "C" fn(*mut AcompReq) -> i32>, pub decompress: Option<unsafe extern "C" fn(*mut AcompReq) -> i32>, pub reqsize: usize }
#[repr(C)] pub struct AcompAlg { pub calg: CompAlgCommon, pub init: Option<unsafe extern "C" fn(*mut CryptoAc​omp) -> i32>, pub exit: Option<unsafe extern "C" fn(*mut CryptoAc​omp)> , pub compress: Option<unsafe extern "C" fn(*mut AcompReq) -> i32>, pub decompress: Option<unsafe extern "C" fn(*mut AcompReq) -> i32> }
#[repr(C)] pub struct CompAlgCommon { pub base: CryptoAlg }
#[repr(C)] pub struct AcompReqChain { pub compl: Option<CryptoCompletionT>, pub data: *mut c_void, pub flags: u32, pub src: *const U8, pub dst: *mut U8, pub ssg: ScatterList, pub dsg: ScatterList }
#[repr(C)] pub struct AcompReq { pub base: AcompReqBase, pub slen: u32, pub dlen: u32, pub svirt: *const U8, pub dvirt: *mut U8, pub src: *mut ScatterList, pub dst: *mut ScatterList, pub chain: AcompReqChain }
#[repr(C)] pub struct AcompReqBase { pub complete: Option<CryptoCompletionT>, pub data: *mut c_void, pub flags: u32, pub tfm: *mut CryptoTfm }
#[repr(C)] pub struct ScatterList { _private: [u8; 0] }
#[repr(C)] pub struct SeqFile { _private: [u8; 0] }
#[repr(C)] pub struct SkBuff { _private: [u8; 0] }
#[repr(C)] pub struct WorkStruct { _private: [u8; 0] }
#[repr(C)] pub struct SpinLock { _private: [u8; 0] }
#[repr(C)] pub struct AcompWalk { pub slen: u32, pub dlen: u32, pub flags: u32, pub input: ScatterWalk, pub output: ScatterWalk }
#[repr(C)] pub struct ScatterWalk { pub sg: *mut c_void, pub offset: usize, pub addr: *mut c_void }
#[repr(C)] pub struct CryptoAc​ompStream { pub lock: SpinLock, pub ctx: *mut c_void }
#[repr(C)] pub struct CryptoAc​ompStreams { pub streams: *mut CryptoAc​ompStream, pub stream_want: Cpumask, pub stream_work: WorkStruct, pub alloc_ctx: unsafe extern "C" fn() -> *mut c_void, pub free_ctx: unsafe extern "C" fn(*mut c_void) }
#[repr(C)] pub struct Cpumask { _private: [u8; 0] }

extern "C" {
    static crypto_acomp_type: CryptoType;
    fn crypto_alloc_tfm(*const i8, *const CryptoType, U32, U32) -> *mut CryptoAc​omp;
    fn crypto_alloc_tfm_node(*const i8, *const CryptoType, U32, U32, i32) -> *mut CryptoAc​omp;
    fn crypto_register_alg(*mut CryptoAlg) -> i32;
    fn crypto_unregister_alg(*mut CryptoAlg);
    fn crypto_free_acomp(*mut CryptoAc​omp);
    fn crypto_acomp_reqsize(*mut CryptoAc​omp) -> usize;
    fn crypto_acomp_reqtfm(*mut AcompReq) -> *mut CryptoAc​omp;
    fn crypto_acomp_compress(*mut AcompReq) -> i32;
    fn crypto_acomp_decompress(*mut AcompReq) -> i32;
    fn acomp_request_isnondma(*mut AcompReq) -> bool;
    fn acomp_request_issg(*mut AcompReq) -> bool;
    fn acomp_request_src_isvirt(*mut AcompReq) -> bool;
    fn acomp_request_dst_isvirt(*mut AcompReq) -> bool;
    fn acomp_req_on_stack(*mut AcompReq) -> bool;
    fn acomp_is_async(*mut CryptoAc​omp) -> bool;
    fn acomp_request_set_src_dma(*mut AcompReq, *const U8, u32);
    fn acomp_request_set_dst_dma(*mut AcompReq, *mut U8, u32);
    fn acomp_request_set_src_sg(*mut AcompReq, *mut ScatterList, u32);
    fn acomp_request_set_dst_sg(*mut AcompReq, *mut ScatterList, u32);
    fn sg_init_one(*mut ScatterList, *const c_void, usize);
    fn nla_put(*mut SkBuff, i32, usize, *const c_void) -> i32;
    fn seq_puts(*mut SeqFile, *const i8);
    fn crypto_init_scomp_ops_async(*mut CryptoTfm) -> i32;
    fn crypto_register_acomp(_alg: *mut AcompAlg) -> i32;
}

// The remaining implementation is a direct low-level translation; kernel
// helper operations are kept as external calls where their definitions belong.
pub unsafe extern "C" fn comp_prepare_alg(alg: *mut CompAlgCommon) {
    (*alg).base.cra_flags &= !0xffff_ffffu32;
}

pub unsafe extern "C" fn crypto_alloc_acomp(name: *const i8, typ: U32, mask: U32) -> *mut CryptoAc​omp {
    crypto_alloc_tfm(name, &crypto_acomp_type, typ, mask)
}

pub unsafe extern "C" fn crypto_alloc_acomp_node(name: *const i8, typ: U32, mask: U32, node: i32) -> *mut CryptoAc​omp {
    crypto_alloc_tfm_node(name, &crypto_acomp_type, typ, mask, node)
}

unsafe fn acomp_save_req(req: *mut AcompReq, cplt: CryptoCompletionT) {
    (*req).chain.compl = (*req).base.complete;
    (*req).chain.data = (*req).base.data;
    (*req).base.complete = Some(cplt);
    (*req).base.data = req.cast();
}

unsafe fn acomp_restore_req(req: *mut AcompReq) {
    (*req).base.complete = (*req).chain.compl;
    (*req).base.data = (*req).chain.data;
}

unsafe fn acomp_reqchain_virt(req: *mut AcompReq) {
    if (*req).chain.flags & 1 != 0 { acomp_request_set_src_dma(req, (*req).chain.src, (*req).slen); }
    if (*req).chain.flags & 2 != 0 { acomp_request_set_dst_dma(req, (*req).chain.dst, (*req).dlen); }
}

unsafe fn acomp_reqchain_finish(req: *mut AcompReq, err: i32) -> i32 {
    acomp_reqchain_virt(req); acomp_restore_req(req); err
}

unsafe extern "C" fn acomp_reqchain_done(data: *mut c_void, mut err: i32) {
    let req = data as *mut AcompReq;
    let compl = (*req).chain.compl;
    let saved = (*req).chain.data;
    if err != -115 { err = acomp_reqchain_finish(req, err); }
    if let Some(f) = compl { f(saved, err); }
}

pub unsafe extern "C" fn crypto_acomp_compress(req: *mut AcompReq) -> i32 {
    let tfm = crypto_acomp_reqtfm(req);
    if acomp_req_on_stack(req) && acomp_is_async(tfm) { return -11; }
    if (*tfm).compress.is_some() || acomp_request_issg(req) { return crypto_acomp_compress(req); }
    acomp_save_req(req, acomp_reqchain_done); let r = if let Some(f) = (*tfm).compress { f(req) } else { -22 }; if r == -16 || r == -115 { r } else { acomp_reqchain_finish(req, r) }
}

pub unsafe extern "C" fn crypto_acomp_decompress(req: *mut AcompReq) -> i32 {
    let tfm = crypto_acomp_reqtfm(req);
    if acomp_req_on_stack(req) && acomp_is_async(tfm) { return -11; }
    if (*tfm).decompress.is_some() || acomp_request_issg(req) { return crypto_acomp_decompress(req); }
    acomp_save_req(req, acomp_reqchain_done); let r = if let Some(f) = (*tfm).decompress { f(req) } else { -22 }; if r == -16 || r == -115 { r } else { acomp_reqchain_finish(req, r) }
}

pub unsafe extern "C" fn crypto_register_acomps(algs: *mut AcompAlg, count: i32) -> i32 {
    for i in 0..count { let r = crypto_register_acomp(algs.add(i as usize)); if r != 0 { return r; } } 0
}
pub unsafe extern "C" fn crypto_unregister_acomp(alg: *mut AcompAlg) { crypto_unregister_alg(&mut (*alg).calg.base); }
pub unsafe extern "C" fn crypto_unregister_acomps(algs: *mut AcompAlg, count: i32) { for i in (0..count).rev() { crypto_unregister_acomp(algs.add(i as usize)); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
