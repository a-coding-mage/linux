// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Asynchronous Cryptographic Hash operations.
 *
 * Rust translation of ahash.c. Kernel-provided types, constants, macros, and
 * functions referenced below are supplied externally by the surrounding
 * kernel bindings.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

const CRYPTO_ALG_TYPE_AHASH_MASK: u32 = 0x0000000e;

// External kernel declarations (provided by the kernel translation unit).
extern "C" {
    fn crypto_ahash_alg(tfm: *mut crypto_ahash) -> *mut ahash_alg;
    fn crypto_ahash_ctx(tfm: *mut crypto_ahash) -> *mut c_void;
    fn crypto_ahash_fb(tfm: *mut crypto_ahash) -> *mut crypto_ahash;
    fn crypto_ahash_reqtfm(req: *mut ahash_request) -> *mut crypto_ahash;
    fn ahash_request_ctx(req: *mut ahash_request) -> *mut u8;
    fn ahash_request_isvirt(req: *mut ahash_request) -> bool;
    fn ahash_req_on_stack(req: *mut ahash_request) -> bool;
    fn ahash_is_async(tfm: *mut crypto_ahash) -> bool;
    fn crypto_ahash_req_virt(tfm: *mut crypto_ahash) -> bool;
    fn crypto_ahash_statesize(tfm: *mut crypto_ahash) -> usize;
    fn crypto_ahash_reqsize(tfm: *mut crypto_ahash) -> usize;
    fn crypto_ahash_blocksize(tfm: *mut crypto_ahash) -> i32;
    fn crypto_ahash_get_flags(tfm: *mut crypto_ahash) -> u32;
    fn crypto_ahash_set_flags(tfm: *mut crypto_ahash, flags: u32);
    fn crypto_ahash_clear_flags(tfm: *mut crypto_ahash, flags: u32);
    fn crypto_ahash_set_statesize(tfm: *mut crypto_ahash, size: usize);
    fn crypto_ahash_set_reqsize(tfm: *mut crypto_ahash, size: usize);
    fn crypto_ahash_alg_name(tfm: *mut crypto_ahash) -> *const i8;
    fn crypto_ahash_tfm(tfm: *mut crypto_ahash) -> *mut crypto_tfm;
    fn __crypto_ahash_cast(tfm: *mut crypto_tfm) -> *mut crypto_ahash;
    fn __crypto_shash_alg(alg: *mut crypto_alg) -> *mut crypto_shash_alg;
    fn crypto_shash_type() -> crypto_type;
    fn crypto_shash_alg_has_setkey(alg: *mut crypto_shash_alg) -> bool;
    fn crypto_shash_get_flags(tfm: *mut crypto_shash) -> u32;
    fn crypto_shash_setkey(tfm: *mut crypto_shash, key: *const u8, len: u32) -> i32;
    fn crypto_shash_init(desc: *mut shash_desc) -> i32;
    fn crypto_shash_update(desc: *mut shash_desc, data: *mut u8, len: i32) -> i32;
    fn crypto_shash_finup(desc: *mut shash_desc, data: *mut u8, len: i32, out: *mut u8) -> i32;
    fn crypto_shash_final(desc: *mut shash_desc, out: *mut u8) -> i32;
    fn crypto_shash_digest(desc: *mut shash_desc, data: *const u8, len: u32, out: *mut u8) -> i32;
    fn crypto_shash_export(desc: *mut shash_desc, out: *mut c_void) -> i32;
    fn crypto_shash_import(desc: *mut shash_desc, input: *const c_void) -> i32;
    fn crypto_shash_export_core(desc: *mut c_void, out: *mut c_void) -> i32;
    fn crypto_shash_import_core(desc: *mut shash_desc, input: *const c_void) -> i32;
    fn crypto_create_tfm(alg: *mut crypto_alg, typ: *const crypto_type) -> *mut crypto_shash;
    fn crypto_free_shash(tfm: *mut crypto_shash);
    fn crypto_mod_get(alg: *mut crypto_alg) -> bool;
    fn crypto_mod_put(alg: *mut crypto_alg);
    fn crypto_register_alg(alg: *mut crypto_alg) -> i32;
    fn crypto_unregister_alg(alg: *mut crypto_alg);
    fn crypto_alloc_tfm(name: *const i8, typ: *const crypto_type, ty: u32, mask: u32) -> *mut crypto_ahash;
    fn crypto_type_has_alg(name: *const i8, typ: *const crypto_type, ty: u32, mask: u32) -> i32;
    fn crypto_alloc_ahash(name: *const i8, ty: u32, mask: u32) -> *mut crypto_ahash;
    fn crypto_free_ahash(tfm: *mut crypto_ahash);
    fn crypto_grab_spawn(spawn: *mut crypto_spawn, inst: *mut crypto_instance, name: *const i8, ty: u32, mask: u32) -> i32;
    fn crypto_register_instance(tmpl: *mut crypto_template, inst: *mut crypto_instance) -> i32;
    fn crypto_drop_spawn(spawn: *mut crypto_spawn);
    fn crypto_unregister_ahashes(algs: *mut ahash_alg, count: i32);
    fn crypto_yield(flags: u32);
    fn ahash_request_complete(req: *mut ahash_request, err: i32);
    fn ahash_request_zero(req: *mut ahash_request);
    fn ahash_request_set_callback(req: *mut ahash_request, flags: u32, complete: Option<unsafe extern "C" fn(*mut c_void, i32)>, data: *mut c_void);
    fn ahash_request_set_virt(req: *mut ahash_request, data: *const u8, out: *mut u8, len: u32);
    fn crypto_hash_no_export_core(tfm: *mut crypto_ahash) -> bool;
    fn crypto_ahash_export(req: *mut ahash_request, out: *mut u8) -> i32;
    fn crypto_ahash_import(req: *mut ahash_request, input: *const u8) -> i32;
}

#[repr(C)] pub struct crypto_ahash { pub using_shash: bool, pub fb: *mut crypto_tfm }
#[repr(C)] pub struct crypto_tfm { pub __crt_alg: *mut crypto_alg, pub exit: Option<unsafe extern "C" fn(*mut crypto_tfm)> }
#[repr(C)] pub struct crypto_alg { pub cra_flags: u32, pub cra_blocksize: u32, pub cra_reqsize: u32, pub cra_type: *const crypto_type, pub cra_init: Option<unsafe extern "C" fn(*mut crypto_tfm) -> i32>, pub cra_exit: Option<unsafe extern "C" fn(*mut crypto_tfm)> }
#[repr(C)] pub struct crypto_type { pub extsize: Option<unsafe extern "C" fn(*mut crypto_alg)->usize>, pub init_tfm: Option<unsafe extern "C" fn(*mut crypto_tfm)->i32>, pub free: Option<unsafe extern "C" fn(*mut crypto_instance)> }
#[repr(C)] pub struct hash_alg_common { pub base: crypto_alg, pub statesize: usize, pub digestsize: u32 }
#[repr(C)] pub struct ahash_alg { pub halg: hash_alg_common, pub setkey: Option<unsafe extern "C" fn(*mut crypto_ahash,*const u8,u32)->i32>, pub init: Option<unsafe extern "C" fn(*mut ahash_request)->i32>, pub update: Option<unsafe extern "C" fn(*mut ahash_request)->i32>, pub final_: Option<unsafe extern "C" fn(*mut ahash_request)->i32>, pub finup: Option<unsafe extern "C" fn(*mut ahash_request)->i32>, pub digest: Option<unsafe extern "C" fn(*mut ahash_request)->i32>, pub export: Option<unsafe extern "C" fn(*mut ahash_request,*mut c_void)->i32>, pub import: Option<unsafe extern "C" fn(*mut ahash_request,*const c_void)->i32>, pub export_core: Option<unsafe extern "C" fn(*mut ahash_request,*mut c_void)->i32>, pub import_core: Option<unsafe extern "C" fn(*mut ahash_request,*const c_void)->i32>, pub init_tfm: Option<unsafe extern "C" fn(*mut crypto_ahash)->i32>, pub exit_tfm: Option<unsafe extern "C" fn(*mut crypto_ahash)> }
#[repr(C)] pub struct crypto_shash { _private: [u8;0] }
#[repr(C)] pub struct crypto_shash_alg { pub halg: hash_alg_common, pub setkey: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct shash_desc { pub tfm: *mut crypto_shash }
#[repr(C)] pub struct ahash_request { pub base: crypto_async_request, pub nbytes: u32, pub src: *mut scatterlist, pub result: *mut u8, pub svirt: *const u8, pub sg_head: *mut scatterlist, pub saved_complete: Option<unsafe extern "C" fn(*mut c_void,i32)>, pub saved_data: *mut c_void }
#[repr(C)] pub struct crypto_async_request { pub flags:u32, pub complete:Option<unsafe extern "C" fn(*mut c_void,i32)>, pub data:*mut c_void }
#[repr(C)] pub struct crypto_hash_walk { pub total:u32,pub entrylen:u32,pub offset:u32,pub flags:u32,pub data:*mut u8,pub sg:*mut scatterlist,pub pg:*mut page }
#[repr(C)] pub struct scatterlist { pub offset:u32,pub length:u32 }
#[repr(C)] pub struct page { _private:[u8;0] }
#[repr(C)] pub struct crypto_instance { _private:[u8;0] }
#[repr(C)] pub struct ahash_instance { pub alg:ahash_alg, pub free:Option<unsafe extern "C" fn(*mut ahash_instance)> }
#[repr(C)] pub struct crypto_spawn { pub frontend:*const crypto_type }
#[repr(C)] pub struct crypto_ahash_spawn { pub base:crypto_spawn }
#[repr(C)] pub struct crypto_template { _private:[u8;0] }

const EAGAIN:i32=11; const EINPROGRESS:i32=115; const EBUSY:i32=16; const ENOSYS:i32=38; const ENOKEY:i32=126; const EINVAL:i32=22; const EOVERFLOW:i32=75;
const CRYPTO_AHASH_ALG_BLOCK_ONLY:u32=1<<0; const CRYPTO_AHASH_ALG_FINAL_NONZERO:u32=1<<1; const CRYPTO_ALG_NEED_FALLBACK:u32=1<<2; const CRYPTO_TFM_REQ_MAY_SLEEP:u32=1<<3; const CRYPTO_AHASH_REQ_VIRT:u32=1<<4; const CRYPTO_TFM_NEED_KEY:u32=1<<5;
const PAGE_SIZE:usize=4096; const PAGE_SHIFT:u32=12; const HASH_MAX_STATESIZE:usize=512; const MAX_SYNC_HASH_REQSIZE:usize=512; const HASH_MAX_DESCSIZE:usize=256; const MAX_ALGAPI_BLOCKSIZE:u32=256;

#[inline] unsafe fn crypto_ahash_block_only(t:*mut crypto_ahash)->bool { ((*crypto_ahash_alg(t)).halg.base.cra_flags & CRYPTO_AHASH_ALG_BLOCK_ONLY)!=0 }
#[inline] unsafe fn crypto_ahash_final_nonzero(t:*mut crypto_ahash)->bool { ((*crypto_ahash_alg(t)).halg.base.cra_flags & CRYPTO_AHASH_ALG_FINAL_NONZERO)!=0 }
#[inline] unsafe fn crypto_ahash_need_fallback(t:*mut crypto_ahash)->bool { ((*crypto_ahash_alg(t)).halg.base.cra_flags & CRYPTO_ALG_NEED_FALLBACK)!=0 }

#[inline] unsafe fn ahash_op_done(data:*mut c_void, mut err:i32, finish:unsafe fn(*mut ahash_request,i32)->i32) { let r=data as *mut ahash_request; let c=(*r).saved_complete; let d=(*r).saved_data; if err== -EINPROGRESS { if let Some(f)=c {f(d,err)}; return } (*r).base.flags &= !CRYPTO_TFM_REQ_MAY_SLEEP; err=finish(r,err); if err==-EINPROGRESS || err==-EBUSY{return} if let Some(f)=c {f(d,err)} }

unsafe fn hash_walk_next(w:*mut crypto_hash_walk)->i32 { let n=core::cmp::min((*w).entrylen, PAGE_SIZE as u32-(*w).offset); (*w).data=core::ptr::null_mut(); (*w).data=(*w).data.add((*w).offset as usize); (*w).entrylen-=n; n as i32 }
unsafe fn hash_walk_new_entry(w:*mut crypto_hash_walk)->i32 { let sg=(*w).sg; (*w).offset=(*sg).offset; (*w).offset &= (PAGE_SIZE as u32)-1; (*w).entrylen=(*sg).length; if (*w).entrylen>(*w).total {(*w).entrylen=(*w).total} (*w).total-=(*w).entrylen; hash_walk_next(w) }

#[no_mangle] pub unsafe extern "C" fn crypto_hash_walk_first(req:*mut ahash_request,w:*mut crypto_hash_walk)->i32 { (*w).total=(*req).nbytes;(*w).entrylen=0;if (*w).total==0{return 0} (*w).flags=(*req).base.flags;if ahash_request_isvirt(req){(*w).data=(*req).svirt as *mut u8;(*w).total=0;return (*req).nbytes as i32} (*w).sg=(*req).src;hash_walk_new_entry(w) }
#[no_mangle] pub unsafe extern "C" fn crypto_hash_walk_done(w:*mut crypto_hash_walk,err:i32)->i32 { if (*w).flags&CRYPTO_AHASH_REQ_VIRT!=0{return err} crypto_yield((*w).flags);if err!=0{return err}if (*w).entrylen!=0 {(*w).offset=0;return hash_walk_next(w)}if (*w).total==0{return 0} (*w).sg=(*w).sg.add(1);hash_walk_new_entry(w) }

#[inline] unsafe fn ahash_to_shash(t:*mut crypto_ahash)->*mut crypto_shash { *(crypto_ahash_ctx(t) as *mut *mut crypto_shash) }
#[inline] unsafe fn prepare_shash_desc(r:*mut ahash_request,t:*mut crypto_ahash)->*mut shash_desc { let d=ahash_request_ctx(r) as *mut shash_desc;(*d).tfm=ahash_to_shash(t);d }

#[no_mangle] pub unsafe extern "C" fn shash_ahash_update(r:*mut ahash_request,d:*mut shash_desc)->i32 { let mut w=core::mem::zeroed();let mut n=crypto_hash_walk_first(r,&mut w);while n>0 {n=crypto_shash_update(d,w.data,n);if n>0{n=crypto_hash_walk_done(&mut w,n)}}n }
#[no_mangle] pub unsafe extern "C" fn shash_ahash_finup(r:*mut ahash_request,d:*mut shash_desc)->i32 { let mut w=core::mem::zeroed();let mut n=crypto_hash_walk_first(r,&mut w);if n==0{return crypto_shash_final(d,(*r).result)} loop {n=if (*w).entrylen==0 {crypto_shash_finup(d,w.data,n,(*r).result)} else {crypto_shash_update(d,w.data,n)};n=crypto_hash_walk_done(&mut w,n);if n<=0{return n}} }
#[no_mangle] pub unsafe extern "C" fn shash_ahash_digest(r:*mut ahash_request,d:*mut shash_desc)->i32 {crypto_shash_digest(d,(*r).svirt,(*r).nbytes,(*r).result)}

#[no_mangle] pub unsafe extern "C" fn crypto_ahash_init(r:*mut ahash_request)->i32 {let t=crypto_ahash_reqtfm(r);if (*t).using_shash{return crypto_shash_init(prepare_shash_desc(r,t))}if crypto_ahash_get_flags(t)&CRYPTO_TFM_NEED_KEY!=0{return -ENOKEY}if ahash_req_on_stack(r)&&ahash_is_async(t){return -EAGAIN}((*crypto_ahash_alg(t)).init.unwrap())(r)}
#[no_mangle] pub unsafe extern "C" fn crypto_ahash_update(r:*mut ahash_request)->i32 {let t=crypto_ahash_reqtfm(r);if (*t).using_shash{return shash_ahash_update(r,ahash_request_ctx(r) as *mut shash_desc)}if let Some(f)=(*crypto_ahash_alg(t)).update{f(r)}else{-ENOSYS}}
#[no_mangle] pub unsafe extern "C" fn crypto_ahash_finup(r:*mut ahash_request)->i32 {let t=crypto_ahash_reqtfm(r);if (*t).using_shash{return shash_ahash_finup(r,ahash_request_ctx(r) as *mut shash_desc)}if let Some(f)=(*crypto_ahash_alg(t)).finup{f(r)}else{crypto_ahash_update(r)}}
#[no_mangle] pub unsafe extern "C" fn crypto_ahash_digest(r:*mut ahash_request)->i32 {let t=crypto_ahash_reqtfm(r);if (*t).using_shash{return shash_ahash_digest(r,prepare_shash_desc(r,t))}if crypto_ahash_get_flags(t)&CRYPTO_TFM_NEED_KEY!=0{return -ENOKEY}((*crypto_ahash_alg(t)).digest.unwrap())(r)}

#[no_mangle] pub unsafe extern "C" fn crypto_ahash_setkey(t:*mut crypto_ahash,key:*const u8,len:u32)->i32 {if (*t).using_shash {let e=crypto_shash_setkey(ahash_to_shash(t),key,len);if e!=0{return e}} else if let Some(f)=(*crypto_ahash_alg(t)).setkey {let e=f(t,key,len);if e!=0{return e}} crypto_ahash_clear_flags(t,CRYPTO_TFM_NEED_KEY);0}
#[no_mangle] pub unsafe extern "C" fn crypto_ahash_export_core(r:*mut ahash_request,out:*mut c_void)->i32 {let t=crypto_ahash_reqtfm(r);if (*t).using_shash{return crypto_shash_export_core(ahash_request_ctx(r) as *mut c_void,out)}((*crypto_ahash_alg(t)).export_core.unwrap())(r,out)}
#[no_mangle] pub unsafe extern "C" fn crypto_ahash_import_core(r:*mut ahash_request,input:*const c_void)->i32 {let t=crypto_ahash_reqtfm(r);if (*t).using_shash{return crypto_shash_import_core(prepare_shash_desc(r,t),input)}((*crypto_ahash_alg(t)).import_core.unwrap())(r,input)}
#[no_mangle] pub unsafe extern "C" fn crypto_ahash_export(r:*mut ahash_request,out:*mut c_void)->i32 {let t=crypto_ahash_reqtfm(r);if (*t).using_shash{return crypto_shash_export(ahash_request_ctx(r) as *mut shash_desc,out)}((*crypto_ahash_alg(t)).export.unwrap())(r,out)}
#[no_mangle] pub unsafe extern "C" fn crypto_ahash_import(r:*mut ahash_request,input:*const c_void)->i32 {let t=crypto_ahash_reqtfm(r);if (*t).using_shash{return crypto_shash_import(prepare_shash_desc(r,t),input)}((*crypto_ahash_alg(t)).import.unwrap())(r,input)}

#[no_mangle] pub unsafe extern "C" fn crypto_hash_alg_has_setkey(h:*mut hash_alg_common)->bool {(*h).base.cra_type==crypto_shash_type() as *const _ || (*__crypto_ahash_alg(h as *mut crypto_alg)).setkey.is_some()}
#[no_mangle] pub unsafe extern "C" fn crypto_register_ahash(a:*mut ahash_alg)->i32 {crypto_register_alg(&mut (*a).halg.base)}
#[no_mangle] pub unsafe extern "C" fn crypto_unregister_ahash(a:*mut ahash_alg){crypto_unregister_alg(&mut (*a).halg.base)}
#[no_mangle] pub unsafe extern "C" fn crypto_register_ahashes(a:*mut ahash_alg,n:i32)->i32 {for i in 0..n {let e=crypto_register_ahash(a.add(i as usize));if e!=0{return e}}0}
#[no_mangle] pub unsafe extern "C" fn crypto_unregister_ahashes_local(a:*mut ahash_alg,n:i32){let mut i=n;while i>0{i-=1;crypto_unregister_ahash(a.add(i as usize))}}
#[no_mangle] pub unsafe extern "C" fn ahash_request_free(r:*mut ahash_request){if !r.is_null()&&!ahash_req_on_stack(r){/* kernel kfree(req) */}}
#[no_mangle] pub unsafe extern "C" fn crypto_hash_digest(t:*mut crypto_ahash,data:*const u8,len:u32,out:*mut u8)->i32 {let mut r:ahash_request=core::mem::zeroed();ahash_request_set_callback(&mut r,0,None,core::ptr::null_mut());ahash_request_set_virt(&mut r,data,out,len);crypto_ahash_digest(&mut r)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
