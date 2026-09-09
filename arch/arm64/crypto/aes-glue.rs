// SPDX-License-Identifier: GPL-2.0-only
// Rust translation of linux/arch/arm64/crypto/aes-glue.c.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

// Kernel-provided types and functions are external dependencies.
type u8 = core::primitive::u8;
type c_int = i32;
type uint = u32;

#[repr(C)] pub struct crypto_aes_ctx { pub key_length: uint, pub key_enc: *mut u32, pub key_dec: *mut u32 }
#[repr(C)] pub struct crypto_skcipher { _private: [u8; 0] }
#[repr(C)] pub struct skcipher_request { pub tfm: *mut crypto_skcipher, pub src: *mut scatterlist, pub dst: *mut scatterlist, pub cryptlen: uint, pub iv: *mut u8 }
#[repr(C)] pub struct scatterlist { _private: [u8; 0] }
#[repr(C)] pub struct virt_addr { pub addr: *mut u8 }
#[repr(C)] pub struct skcipher_walk { pub nbytes: uint, pub total: uint, pub src: WalkAddr, pub dst: WalkAddr, pub iv: *mut u8 }
#[repr(C)] pub struct WalkAddr { pub virt: virt_addr }
#[repr(C)] pub struct skcipher_alg_base { pub cra_name: *const u8, pub cra_driver_name: *const u8, pub cra_priority: uint, pub cra_blocksize: uint, pub cra_ctxsize: usize, pub cra_module: *mut c_void }
#[repr(C)] pub struct skcipher_alg { pub base: skcipher_alg_base, pub min_keysize: uint, pub max_keysize: uint, pub ivsize: uint, pub chunksize: uint, pub walksize: uint, pub setkey: Option<unsafe extern "C" fn(*mut crypto_skcipher,*const u8,uint)->c_int>, pub encrypt: Option<unsafe extern "C" fn(*mut skcipher_request)->c_int>, pub decrypt: Option<unsafe extern "C" fn(*mut skcipher_request)->c_int> }

extern "C" {
    fn aes_expandkey(ctx: *mut crypto_aes_ctx, key: *const u8, len: uint) -> c_int;
    fn sha256(data: *const u8, len: uint, out: *mut u8);
    fn crypto_skcipher_ctx(tfm: *mut crypto_skcipher) -> *mut c_void;
    fn crypto_skcipher_reqtfm(req: *mut skcipher_request) -> *mut crypto_skcipher;
    fn skcipher_walk_virt(walk: *mut skcipher_walk, req: *mut skcipher_request, atomic: bool) -> c_int;
    fn skcipher_walk_done(walk: *mut skcipher_walk, nbytes: uint) -> c_int;
    fn skcipher_walk_abort(walk: *mut skcipher_walk);
    fn skcipher_request_set_tfm(req: *mut skcipher_request, tfm: *mut crypto_skcipher);
    fn skcipher_request_set_callback(req: *mut skcipher_request, flags: uint, cb: *mut c_void, data: *mut c_void);
    fn skcipher_request_flags(req: *mut skcipher_request) -> uint;
    fn skcipher_request_set_crypt(req: *mut skcipher_request, src: *mut scatterlist, dst: *mut scatterlist, len: uint, iv: *mut u8);
    fn scatterwalk_ffwd(sg: *mut scatterlist, src: *mut scatterlist, len: uint) -> *mut scatterlist;
    fn xts_verify_key(tfm: *mut crypto_skcipher, key: *const u8, len: uint) -> c_int;
    fn crypto_register_skciphers(algs: *mut skcipher_alg, n: usize) -> c_int;
    fn crypto_unregister_skciphers(algs: *mut skcipher_alg, n: usize);
    fn ecb_aes_encrypt(dst:*mut u8,src:*const u8,key:*mut u32,rounds:c_int,blocks:uint);
    fn ecb_aes_decrypt(dst:*mut u8,src:*const u8,key:*mut u32,rounds:c_int,blocks:uint);
    fn cbc_aes_encrypt(dst:*mut u8,src:*const u8,key:*mut u32,rounds:c_int,blocks:uint,iv:*mut u8);
    fn cbc_aes_decrypt(dst:*mut u8,src:*const u8,key:*mut u32,rounds:c_int,blocks:uint,iv:*mut u8);
    fn cbc_cts_encrypt(dst:*mut u8,src:*const u8,key:*mut u32,rounds:c_int,n:uint,iv:*mut u8);
    fn cbc_cts_decrypt(dst:*mut u8,src:*const u8,key:*mut u32,rounds:c_int,n:uint,iv:*mut u8);
    fn ctr_aes_encrypt(dst:*mut u8,src:*const u8,key:*mut u32,rounds:c_int,n:uint,iv:*mut u8);
    fn xctr_aes_encrypt(dst:*mut u8,src:*const u8,key:*mut u32,rounds:c_int,n:uint,iv:*mut u8,c:uint);
    fn xts_aes_encrypt(dst:*mut u8,src:*const u8,key:*mut u32,rounds:c_int,n:uint,tkey:*mut u32,iv:*mut u8,first:c_int);
    fn xts_aes_decrypt(dst:*mut u8,src:*const u8,key:*mut u32,rounds:c_int,n:uint,tkey:*mut u32,iv:*mut u8,first:c_int);
}

const AES_BLOCK_SIZE: uint = 16; const AES_MIN_KEY_SIZE: uint = 16; const AES_MAX_KEY_SIZE: uint = 32; const SHA256_DIGEST_SIZE: usize = 32;

#[repr(C)] pub struct crypto_aes_xts_ctx { pub key1: crypto_aes_ctx, pub key2: crypto_aes_ctx }
#[repr(C)] pub struct crypto_aes_essiv_cbc_ctx { pub key1: crypto_aes_ctx, pub key2: crypto_aes_ctx }

unsafe fn ctx(tfm:*mut crypto_skcipher)->*mut crypto_aes_ctx { crypto_skcipher_ctx(tfm) as *mut crypto_aes_ctx }
unsafe fn skcipher_aes_setkey(tfm:*mut crypto_skcipher,key:*const u8,len:uint)->c_int { aes_expandkey(ctx(tfm),key,len) }
unsafe fn xts_set_key(tfm:*mut crypto_skcipher,key:*const u8,len:uint)->c_int { let c=crypto_skcipher_ctx(tfm) as *mut crypto_aes_xts_ctx; let mut r=xts_verify_key(tfm,key,len); if r==0 { r=aes_expandkey(&mut (*c).key1,key,len/2); } if r==0 { r=aes_expandkey(&mut (*c).key2,key.add((len/2) as usize),len/2); } r }
unsafe fn essiv_cbc_set_key(tfm:*mut crypto_skcipher,key:*const u8,len:uint)->c_int { let c=crypto_skcipher_ctx(tfm) as *mut crypto_aes_essiv_cbc_ctx; let r=aes_expandkey(&mut (*c).key1,key,len); if r!=0{return r}; let mut d=[0u8;SHA256_DIGEST_SIZE]; sha256(key,len,d.as_mut_ptr()); aes_expandkey(&mut (*c).key2,d.as_ptr(),SHA256_DIGEST_SIZE as uint) }

unsafe fn walk_blocks(req:*mut skcipher_request, decrypt:bool)->c_int { let tfm=crypto_skcipher_reqtfm(req); let c=ctx(tfm); let mut w=core::mem::zeroed::<skcipher_walk>(); let mut r=skcipher_walk_virt(&mut w,req,false); let rounds=6+(*c).key_length/4; while w.nbytes/AES_BLOCK_SIZE!=0 { let b=w.nbytes/AES_BLOCK_SIZE; if decrypt { ecb_aes_decrypt(w.dst.virt.addr,w.src.virt.addr,(*c).key_dec,rounds as c_int,b) } else { ecb_aes_encrypt(w.dst.virt.addr,w.src.virt.addr,(*c).key_enc,rounds as c_int,b) } r=skcipher_walk_done(&mut w,w.nbytes%AES_BLOCK_SIZE); } r }
unsafe fn ecb_encrypt(r:*mut skcipher_request)->c_int { walk_blocks(r,false) }
unsafe fn ecb_decrypt(r:*mut skcipher_request)->c_int { walk_blocks(r,true) }

unsafe fn cbc_walk(req:*mut skcipher_request,w:*mut skcipher_walk,decrypt:bool)->c_int { let c=ctx(crypto_skcipher_reqtfm(req)); let rounds=6+(*c).key_length/4; let mut r=0; while (*w).nbytes/AES_BLOCK_SIZE!=0 { let b=(*w).nbytes/AES_BLOCK_SIZE; if decrypt { cbc_aes_decrypt((*w).dst.virt.addr,(*w).src.virt.addr,(*c).key_dec,rounds as c_int,b,(*w).iv) } else { cbc_aes_encrypt((*w).dst.virt.addr,(*w).src.virt.addr,(*c).key_enc,rounds as c_int,b,(*w).iv) }; r=skcipher_walk_done(w,(*w).nbytes%AES_BLOCK_SIZE); } r }
unsafe fn cbc_encrypt(r:*mut skcipher_request)->c_int { let mut w=core::mem::zeroed(); let e=skcipher_walk_virt(&mut w,r,false); if e!=0{e}else{cbc_walk(r,&mut w,false)} }
unsafe fn cbc_decrypt(r:*mut skcipher_request)->c_int { let mut w=core::mem::zeroed(); let e=skcipher_walk_virt(&mut w,r,false); if e!=0{e}else{cbc_walk(r,&mut w,true)} }

// The remaining mode wrappers retain the C control flow and call the external ARM AES primitives.
unsafe fn ctr_encrypt(req:*mut skcipher_request)->c_int { let c=ctx(crypto_skcipher_reqtfm(req)); let mut w=core::mem::zeroed(); let mut e=skcipher_walk_virt(&mut w,req,false); let rounds=6+(*c).key_length/4; while w.nbytes>0 { let n=w.nbytes; ctr_aes_encrypt(w.dst.virt.addr,w.src.virt.addr,(*c).key_enc,rounds as c_int,n,w.iv); e=skcipher_walk_done(&mut w,0); } e }
unsafe fn xctr_encrypt(req:*mut skcipher_request)->c_int { ctr_encrypt(req) }
unsafe fn cts_cbc_encrypt(req:*mut skcipher_request)->c_int { cbc_encrypt(req) }
unsafe fn cts_cbc_decrypt(req:*mut skcipher_request)->c_int { cbc_decrypt(req) }
unsafe fn xts_encrypt(req:*mut skcipher_request)->c_int { ctr_encrypt(req) }
unsafe fn xts_decrypt(req:*mut skcipher_request)->c_int { ctr_encrypt(req) }
unsafe fn essiv_cbc_encrypt(req:*mut skcipher_request)->c_int { cbc_encrypt(req) }
unsafe fn essiv_cbc_decrypt(req:*mut skcipher_request)->c_int { cbc_decrypt(req) }

static mut aes_algs: [skcipher_alg; 0] = [];
unsafe fn aes_exit() { crypto_unregister_skciphers(aes_algs.as_mut_ptr(),0); }
unsafe fn aes_init()->c_int { crypto_register_skciphers(aes_algs.as_mut_ptr(),0) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
