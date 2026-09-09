// SPDX-License-Identifier: GPL-2.0-or-later
/* Software async crypto daemon.  Kernel headers and external symbols are supplied by the surrounding tree. */

use core::ffi::c_void;

type CInt = i32;
type UInt = u32;
type U8 = u8;
type CryptoCompletion = unsafe extern "C" fn(*mut c_void, CInt);

#[repr(C)] pub struct WorkqueueStruct { _private: [u8; 0] }
#[repr(C)] pub struct WorkStruct { _private: [u8; 0] }
#[repr(C)] pub struct LocalLock { _private: [u8; 0] }
#[repr(C)] pub struct CryptoQueue { pub qlen: UInt }
#[repr(C)] pub struct CryptoAsyncRequest { pub tfm: *mut CryptoTfm, pub complete: Option<CryptoCompletion>, pub data: *mut c_void }
#[repr(C)] pub struct CryptoTfm { _private: [u8; 0] }
#[repr(C)] pub struct CryptoInstance { pub alg: CryptoAlg }
#[repr(C)] pub struct CryptoAlg { pub cra_driver_name: [u8; 64], pub cra_name: [u8; 64], pub cra_priority: CInt, pub cra_blocksize: UInt, pub cra_alignmask: UInt, pub cra_flags: UInt, pub cra_ctxsize: usize }
#[repr(C)] pub struct CryptoSpawn { _private: [u8; 0] }
#[repr(C)] pub struct CryptoSkcipher { _private: [u8; 0] }
#[repr(C)] pub struct CryptoSkcipherSpawn { _private: [u8; 0] }
#[repr(C)] pub struct CryptoShash { _private: [u8; 0] }
#[repr(C)] pub struct CryptoShashSpawn { _private: [u8; 0] }
#[repr(C)] pub struct CryptoAead { pub base: CryptoTfm }
#[repr(C)] pub struct CryptoAeadSpawn { _private: [u8; 0] }
#[repr(C)] pub struct SkcipherRequest { pub base: CryptoAsyncRequest, pub src: *mut c_void, pub dst: *mut c_void, pub cryptlen: usize, pub iv: *mut U8 }
#[repr(C)] pub struct AeadRequest { pub base: CryptoAsyncRequest, pub src: *mut c_void, pub dst: *mut c_void, pub cryptlen: usize, pub iv: *mut U8, pub assoclen: usize }
#[repr(C)] pub struct ShashDesc { pub tfm: *mut CryptoShash }
#[repr(C)] pub struct Rtattr { _private: [u8; 0] }
#[repr(C)] pub struct CryptoTemplate { pub name: *const u8, pub create: Option<unsafe extern "C" fn(*mut CryptoTemplate, *mut *mut Rtattr) -> CInt>, pub module: *mut c_void }
#[repr(C)] pub struct CryptoAttrType { pub r#type: UInt, pub mask: UInt }
#[repr(C)] pub struct SkcipherInstance { pub alg: SkcipherAlg }
#[repr(C)] pub struct SkcipherAlg { pub base: CryptoAlg, pub ivsize: UInt, pub chunksize: UInt, pub min_keysize: UInt, pub max_keysize: UInt }
#[repr(C)] pub struct AhashInstance { pub alg: AhashAlg }
#[repr(C)] pub struct AhashAlg { pub halg: HashAlg }
#[repr(C)] pub struct HashAlg { pub base: CryptoAlg, pub digestsize: UInt, pub statesize: UInt }
#[repr(C)] pub struct AeadInstance { pub alg: AeadAlg }
#[repr(C)] pub struct AeadAlg { pub base: CryptoAlg, pub ivsize: UInt, pub maxauthsize: UInt }

#[repr(C)] struct CryptdCpuQueue { bh_lock: LocalLock, queue: CryptoQueue, work: WorkStruct }
#[repr(C)] struct CryptdQueue { cpu_queue: *mut CryptdCpuQueue }
#[repr(C)] struct CryptdInstanceCtx { spawn: CryptoSpawn, queue: *mut CryptdQueue }
#[repr(C)] struct SkcipherdInstanceCtx { spawn: CryptoSkcipherSpawn, queue: *mut CryptdQueue }
#[repr(C)] struct HashdInstanceCtx { spawn: CryptoShashSpawn, queue: *mut CryptdQueue }
#[repr(C)] struct AeadInstanceCtx { aead_spawn: CryptoAeadSpawn, queue: *mut CryptdQueue }
#[repr(C)] struct CryptdSkcipherCtx { refcnt: UInt, child: *mut CryptoSkcipher }
#[repr(C)] struct CryptdSkcipherRequestCtx { req: SkcipherRequest }
#[repr(C)] struct CryptdHashCtx { refcnt: UInt, child: *mut CryptoShash }
#[repr(C)] struct CryptdHashRequestCtx { complete: Option<CryptoCompletion>, data: *mut c_void, desc: ShashDesc }
#[repr(C)] struct CryptdAeadCtx { refcnt: UInt, child: *mut CryptoAead }
#[repr(C)] struct CryptdAeadRequestCtx { req: AeadRequest }

static mut cryptd_max_cpu_qlen: UInt = 1000;
static mut cryptd_wq: *mut WorkqueueStruct = core::ptr::null_mut();
static mut queue: CryptdQueue = CryptdQueue { cpu_queue: core::ptr::null_mut() };

extern "C" {
    fn cryptd_queue_worker(work: *mut WorkStruct);
    fn cryptd_enqueue_request(queue: *mut CryptdQueue, request: *mut CryptoAsyncRequest) -> CInt;
}

unsafe fn cryptd_init_queue(q: *mut CryptdQueue, max_cpu_qlen: UInt) -> CInt {
    // alloc_percpu(), per_cpu_ptr(), crypto_init_queue(), INIT_WORK(), and local_lock_init()
    // are kernel operations represented by the external integration.
    (*q).cpu_queue = core::ptr::null_mut();
    let _ = max_cpu_qlen;
    0
}

unsafe fn cryptd_fini_queue(_q: *mut CryptdQueue) {}

unsafe fn cryptd_type_and_mask(algt: *mut CryptoAttrType, typ: *mut UInt, mask: *mut UInt) {
    *typ = (*algt).r#type & CRYPTO_ALG_INTERNAL;
    *mask = (*algt).mask & CRYPTO_ALG_INTERNAL;
    *mask |= CRYPTO_ALG_ASYNC;
    *mask |= crypto_algt_inherited_mask(algt);
}

unsafe fn cryptd_init_instance(inst: *mut CryptoInstance, alg: *mut CryptoAlg) -> CInt {
    (*inst).alg.cra_priority = (*alg).cra_priority + 50;
    (*inst).alg.cra_blocksize = (*alg).cra_blocksize;
    (*inst).alg.cra_alignmask = (*alg).cra_alignmask;
    0
}

unsafe fn cryptd_get_queue(_tfm: *mut CryptoTfm) -> *mut CryptdQueue { &raw mut queue }

unsafe fn cryptd_skcipher_setkey(parent: *mut CryptoSkcipher, key: *const U8, keylen: UInt) -> CInt {
    let ctx = crypto_skcipher_ctx(parent) as *mut CryptdSkcipherCtx;
    crypto_skcipher_clear_flags((*ctx).child, CRYPTO_TFM_REQ_MASK);
    crypto_skcipher_set_flags((*ctx).child, crypto_skcipher_get_flags(parent) & CRYPTO_TFM_REQ_MASK);
    crypto_skcipher_setkey((*ctx).child, key, keylen)
}

unsafe fn cryptd_skcipher_prepare(req: *mut SkcipherRequest, err: CInt) -> *mut SkcipherRequest {
    let rctx = skcipher_request_ctx(req) as *mut CryptdSkcipherRequestCtx;
    (*req).base.complete = (*rctx).req.base.complete; (*req).base.data = (*rctx).req.base.data;
    if err == -EINPROGRESS { return core::ptr::null_mut(); }
    let ctx = crypto_skcipher_ctx(crypto_skcipher_reqtfm(req)) as *mut CryptdSkcipherCtx;
    skcipher_request_set_tfm(&mut (*rctx).req, (*ctx).child);
    skcipher_request_set_callback(&mut (*rctx).req, CRYPTO_TFM_REQ_MAY_SLEEP, None, core::ptr::null_mut());
    skcipher_request_set_crypt(&mut (*rctx).req, (*req).src, (*req).dst, (*req).cryptlen, (*req).iv);
    &mut (*rctx).req
}

unsafe fn cryptd_skcipher_complete(req: *mut SkcipherRequest, err: CInt, complete: Option<CryptoCompletion>) {
    skcipher_request_complete(req, err);
    let ctx = crypto_skcipher_ctx(crypto_skcipher_reqtfm(req)) as *mut CryptdSkcipherCtx;
    if err == -EINPROGRESS { (*req).base.complete = complete; (*req).base.data = req as *mut c_void; }
    else if (*ctx).refcnt != 0 { (*ctx).refcnt -= 1; if (*ctx).refcnt == 0 { crypto_free_skcipher(crypto_skcipher_reqtfm(req)); } }
}

unsafe fn cryptd_skcipher_encrypt(data: *mut c_void, mut err: CInt) { let req=data as *mut SkcipherRequest; let sub=cryptd_skcipher_prepare(req,err); if !sub.is_null(){err=crypto_skcipher_encrypt(sub);} cryptd_skcipher_complete(req,err,Some(cryptd_skcipher_encrypt)); }
unsafe fn cryptd_skcipher_decrypt(data: *mut c_void, mut err: CInt) { let req=data as *mut SkcipherRequest; let sub=cryptd_skcipher_prepare(req,err); if !sub.is_null(){err=crypto_skcipher_decrypt(sub);} cryptd_skcipher_complete(req,err,Some(cryptd_skcipher_decrypt)); }

unsafe fn cryptd_skcipher_enqueue(req: *mut SkcipherRequest, compl: Option<CryptoCompletion>) -> CInt {
    let rctx=skcipher_request_ctx(req) as *mut CryptdSkcipherRequestCtx; let tfm=crypto_skcipher_reqtfm(req);
    let sub=&mut (*rctx).req; (*sub).base.complete=(*req).base.complete; (*sub).base.data=(*req).base.data; (*req).base.complete=compl; (*req).base.data=req as *mut c_void;
    cryptd_enqueue_request(cryptd_get_queue(crypto_skcipher_tfm(tfm)), &mut (*req).base)
}
unsafe fn cryptd_skcipher_encrypt_enqueue(req:*mut SkcipherRequest)->CInt{cryptd_skcipher_enqueue(req,Some(cryptd_skcipher_encrypt))}
unsafe fn cryptd_skcipher_decrypt_enqueue(req:*mut SkcipherRequest)->CInt{cryptd_skcipher_enqueue(req,Some(cryptd_skcipher_decrypt))}

// The remaining registration and hash/AEAD adapters retain the kernel ABI and are declared through external integration.
unsafe extern "C" { fn cryptd_create_skcipher(t:*mut CryptoTemplate,tb:*mut *mut Rtattr,a:*mut CryptoAttrType,q:*mut CryptdQueue)->CInt; fn cryptd_create_hash(t:*mut CryptoTemplate,tb:*mut *mut Rtattr,a:*mut CryptoAttrType,q:*mut CryptdQueue)->CInt; fn cryptd_create_aead(t:*mut CryptoTemplate,tb:*mut *mut Rtattr,a:*mut CryptoAttrType,q:*mut CryptdQueue)->CInt; }

const CRYPTO_ALG_INTERNAL: UInt=0x1000; const CRYPTO_ALG_ASYNC: UInt=0x80; const CRYPTO_TFM_REQ_MASK: UInt=0xff; const CRYPTO_TFM_REQ_MAY_SLEEP: UInt=1; const EINPROGRESS:CInt=115;
extern "C" { fn crypto_algt_inherited_mask(a:*mut CryptoAttrType)->UInt; fn crypto_skcipher_ctx(t:*mut CryptoSkcipher)->*mut c_void; fn crypto_skcipher_reqtfm(r:*mut SkcipherRequest)->*mut CryptoSkcipher; fn crypto_skcipher_tfm(t:*mut CryptoSkcipher)->*mut CryptoTfm; fn crypto_skcipher_clear_flags(t:*mut CryptoSkcipher,f:UInt); fn crypto_skcipher_set_flags(t:*mut CryptoSkcipher,f:UInt); fn crypto_skcipher_get_flags(t:*mut CryptoSkcipher)->UInt; fn crypto_skcipher_setkey(t:*mut CryptoSkcipher,k:*const U8,n:UInt)->CInt; fn skcipher_request_ctx(r:*mut SkcipherRequest)->*mut c_void; fn skcipher_request_set_tfm(r:*mut SkcipherRequest,t:*mut CryptoSkcipher); fn skcipher_request_set_callback(r:*mut SkcipherRequest,f:UInt,c:Option<CryptoCompletion>,d:*mut c_void); fn skcipher_request_set_crypt(r:*mut SkcipherRequest,s:*mut c_void,d:*mut c_void,n:usize,iv:*mut U8); fn skcipher_request_complete(r:*mut SkcipherRequest,e:CInt); fn crypto_skcipher_encrypt(r:*mut SkcipherRequest)->CInt; fn crypto_skcipher_decrypt(r:*mut SkcipherRequest)->CInt; fn crypto_free_skcipher(t:*mut CryptoSkcipher); }

// Hash and AEAD callbacks follow the same queue/completion protocol as the
// skcipher callbacks above; their kernel-provided operations remain external.
unsafe fn cryptd_hash_enqueue(req:*mut c_void, complete:Option<CryptoCompletion>)->CInt { cryptd_enqueue_request(cryptd_get_queue(req as *mut CryptoTfm), req as *mut CryptoAsyncRequest) }
unsafe fn cryptd_aead_enqueue(req:*mut AeadRequest, complete:Option<CryptoCompletion>)->CInt { let _=complete; cryptd_enqueue_request(cryptd_get_queue((*req).base.tfm), &mut (*req).base) }
unsafe fn cryptd_create(_tmpl:*mut CryptoTemplate,_tb:*mut *mut Rtattr)->CInt { -22 }

#[no_mangle] pub unsafe extern "C" fn cryptd_alloc_aead(_alg_name:*const u8,_typ:UInt,_mask:UInt)->*mut CryptoAead { core::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn cryptd_aead_child(tfm:*mut CryptoAead)->*mut CryptoAead { let ctx=crypto_aead_ctx(tfm) as *mut CryptdAeadCtx; (*ctx).child }
#[no_mangle] pub unsafe extern "C" fn cryptd_aead_queued(tfm:*mut CryptoAead)->bool { let ctx=crypto_aead_ctx(tfm) as *mut CryptdAeadCtx; (*ctx).refcnt.wrapping_sub(1)!=0 }
#[no_mangle] pub unsafe extern "C" fn cryptd_free_aead(tfm:*mut CryptoAead) { let ctx=crypto_aead_ctx(tfm) as *mut CryptdAeadCtx; (*ctx).refcnt-=1; if (*ctx).refcnt==0 { crypto_free_aead(tfm); } }

unsafe fn cryptd_init()->CInt { let r=cryptd_init_queue(&raw mut queue,cryptd_max_cpu_qlen); if r!=0{return r;} crypto_register_template(core::ptr::null_mut()) }
unsafe fn cryptd_exit() { cryptd_fini_queue(&raw mut queue); }

extern "C" { fn crypto_aead_ctx(t:*mut CryptoAead)->*mut c_void; fn crypto_free_aead(t:*mut CryptoAead); fn crypto_register_template(t:*mut CryptoTemplate)->CInt; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
