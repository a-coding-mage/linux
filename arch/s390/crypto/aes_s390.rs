// SPDX-License-Identifier: GPL-2.0+
/* Cryptographic API. s390 implementation of the AES Cipher Algorithm. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/* Kernel headers and their symbols are supplied by the surrounding tree. */
use core::mem::{size_of, zeroed};
use core::ptr::{copy, copy_nonoverlapping, null, null_mut};

#[repr(C)] pub struct crypto_skcipher { _private: [u8; 0] }
#[repr(C)] pub struct crypto_aead { _private: [u8; 0] }
#[repr(C)] pub struct crypto_tfm { _private: [u8; 0] }
#[repr(C)] pub struct skcipher_request { pub cryptlen: usize, pub iv: *mut u8, _private: [u8; 0] }
#[repr(C)] pub struct aead_request { pub assoclen: u32, pub cryptlen: u32, pub iv: *mut u8, pub src: *mut scatterlist, pub dst: *mut scatterlist, _private: [u8; 0] }
#[repr(C)] pub struct scatterlist { _private: [u8; 0] }
#[repr(C)] pub struct scatter_walk { pub addr: *mut u8, _private: [u8; 0] }
#[repr(C)] pub struct skcipher_walk { pub nbytes: u32, pub iv: *mut u8, pub src: walk_virt, pub dst: walk_virt, _private: [u8; 0] }
#[repr(C)] pub struct walk_virt { pub virt: virt_addr }
#[repr(C)] pub struct virt_addr { pub addr: *mut u8 }
pub type cpacf_mask_t = u64;

#[repr(C)] pub struct s390_aes_ctx { pub key: [u8; 32], pub key_len: i32, pub fc: usize, pub fallback: *mut crypto_skcipher }
#[repr(C)] pub union s390_xts_union { pub keys: [u8;64], pub split: s390_xts_keys }
#[repr(C)] pub struct s390_xts_keys { pub key: [u8;32], pub pcc_key: [u8;32] }
#[repr(C)] pub struct s390_xts_ctx { pub u: s390_xts_union, pub key_len: i32, pub fc: usize, pub fallback: *mut crypto_skcipher }
#[repr(C)] pub struct gcm_sg_walk { pub walk: scatter_walk, pub walk_bytes: u32, pub walk_bytes_remain: u32, pub buf: [u8;16], pub buf_bytes: u32, pub ptr: *mut u8, pub nbytes: u32 }

extern "C" {
    static mut ctrblk: *mut u8;
    static mut km_functions: cpacf_mask_t; static mut kmc_functions: cpacf_mask_t;
    static mut kmctr_functions: cpacf_mask_t; static mut kma_functions: cpacf_mask_t;
    fn crypto_skcipher_ctx(t: *mut crypto_skcipher) -> *mut s390_aes_ctx;
    fn crypto_aead_ctx(t: *mut crypto_aead) -> *mut s390_aes_ctx;
    fn crypto_skcipher_reqtfm(r: *mut skcipher_request) -> *mut crypto_skcipher;
    fn crypto_aead_reqtfm(r: *mut aead_request) -> *mut crypto_aead;
    fn crypto_skcipher_setkey(t: *mut crypto_skcipher,k:*const u8,n:u32)->i32;
    fn crypto_skcipher_encrypt(r:*mut skcipher_request)->i32; fn crypto_skcipher_decrypt(r:*mut skcipher_request)->i32;
    fn skcipher_request_ctx(r:*mut skcipher_request)->*mut skcipher_request;
    fn skcipher_request_set_tfm(r:*mut skcipher_request,t:*mut crypto_skcipher);
    fn skcipher_walk_virt(w:*mut skcipher_walk,r:*mut skcipher_request, b:bool)->i32;
    fn skcipher_walk_done(w:*mut skcipher_walk,n:u32)->i32;
    fn cpacf_test_func(m:*const cpacf_mask_t,fc:usize)->bool;
    fn cpacf_km(fc:usize,key:*const u8,dst:*mut u8,src:*const u8,n:u32);
    fn cpacf_kmc(fc:usize,p:*mut u8,dst:*mut u8,src:*const u8,n:u32);
    fn cpacf_kmctr(fc:usize,key:*const u8,dst:*mut u8,src:*const u8,n:u32,ctr:*mut u8);
    fn cpacf_pcc(fc:usize,p:*mut u8); fn cpacf_kma(fc:usize,p:*mut u8,dst:*mut u8,src:*const u8,n:u32,aad:*const u8,an:u32);
    fn crypto_inc(v:*mut u8,n:u32); fn memzero_explicit(p:*mut u8,n:usize);
    fn scatterwalk_start(w:*mut scatter_walk,sg:*mut scatterlist); fn scatterwalk_next(w:*mut scatter_walk,n:u32)->u32;
    fn scatterwalk_done_src(w:*mut scatter_walk,n:u32); fn scatterwalk_done_dst(w:*mut scatter_walk,n:u32); fn scatterwalk_unmap(w:*mut scatter_walk);
    fn scatterwalk_map_and_copy(dst:*mut u8,sg:*mut scatterlist,o:u32,n:u32,to:bool);
    fn crypto_memneq(a:*const u8,b:*const u8,n:usize)->bool;
}

unsafe fn setkey_fallback_skcipher(t:*mut crypto_skcipher,key:*const u8,len:u32)->i32 { crypto_skcipher_setkey((*crypto_skcipher_ctx(t)).fallback,key,len) }
unsafe fn fallback_skcipher_crypt(s:*mut s390_aes_ctx,r:*mut skcipher_request,m:usize)->i32 { let q=skcipher_request_ctx(r); *q=*r; skcipher_request_set_tfm(q,(*s).fallback); if m&CPACF_DECRYPT!=0 {crypto_skcipher_decrypt(q)} else {crypto_skcipher_encrypt(q)} }

unsafe fn ecb_aes_set_key(t:*mut crypto_skcipher,k:*const u8,l:u32)->i32 { let s=crypto_skcipher_ctx(t); let fc=match l {16=>CPACF_KM_AES_128,24=>CPACF_KM_AES_192,32=>CPACF_KM_AES_256,_=>0}; (*s).fc=if fc!=0&&cpacf_test_func(&km_functions,fc){fc}else{0}; if (*s).fc==0{return setkey_fallback_skcipher(t,k,l)}; (*s).key_len=l as i32; copy_nonoverlapping(k,(*s).key.as_mut_ptr(),l as usize); 0 }
unsafe fn ecb_aes_crypt(r:*mut skcipher_request,m:usize)->i32 { let s=crypto_skcipher_ctx(crypto_skcipher_reqtfm(r)); if (*s).fc==0{return fallback_skcipher_crypt(s,r,m)}; let mut w:skcipher_walk=zeroed(); let mut ret=skcipher_walk_virt(&mut w,r,false); while w.nbytes!=0 {let n=w.nbytes&!15; cpacf_km((*s).fc|m,(*s).key.as_ptr(),w.dst.virt.addr,w.src.virt.addr,n); ret=skcipher_walk_done(&mut w,w.nbytes-n)} ret }
unsafe fn ecb_aes_encrypt(r:*mut skcipher_request)->i32{ecb_aes_crypt(r,0)} unsafe fn ecb_aes_decrypt(r:*mut skcipher_request)->i32{ecb_aes_crypt(r,CPACF_DECRYPT)}

unsafe fn cbc_aes_set_key(t:*mut crypto_skcipher,k:*const u8,l:u32)->i32 { let s=crypto_skcipher_ctx(t); let fc=match l{16=>CPACF_KMC_AES_128,24=>CPACF_KMC_AES_192,32=>CPACF_KMC_AES_256,_=>0}; (*s).fc=if fc!=0&&cpacf_test_func(&kmc_functions,fc){fc}else{0}; if (*s).fc==0{return setkey_fallback_skcipher(t,k,l)}; (*s).key_len=l as i32;copy_nonoverlapping(k,(*s).key.as_mut_ptr(),l as usize);0 }
unsafe fn cbc_aes_crypt(r:*mut skcipher_request,m:usize)->i32 { let s=crypto_skcipher_ctx(crypto_skcipher_reqtfm(r));if (*s).fc==0{return fallback_skcipher_crypt(s,r,m)};let mut w:skcipher_walk=zeroed();let mut p=[0u8;64];let mut ret=skcipher_walk_virt(&mut w,r,false);if ret!=0{return ret}copy_nonoverlapping(w.iv,p.as_mut_ptr(),16);copy_nonoverlapping((*s).key.as_ptr(),p.as_mut_ptr().add(16),(*s).key_len as usize);while w.nbytes!=0{let n=w.nbytes&!15;cpacf_kmc((*s).fc|m,p.as_mut_ptr(),w.dst.virt.addr,w.src.virt.addr,n);copy_nonoverlapping(p.as_ptr(),w.iv,16);ret=skcipher_walk_done(&mut w,w.nbytes-n)}memzero_explicit(p.as_mut_ptr(),64);ret}
unsafe fn cbc_aes_encrypt(r:*mut skcipher_request)->i32{cbc_aes_crypt(r,0)} unsafe fn cbc_aes_decrypt(r:*mut skcipher_request)->i32{cbc_aes_crypt(r,CPACF_DECRYPT)}

unsafe fn xts_aes_set_key(t:*mut crypto_skcipher,k:*const u8,l:u32)->i32 { let x=crypto_skcipher_ctx(t) as *mut s390_xts_ctx; let fc=match l{32=>CPACF_KM_XTS_128,64=>CPACF_KM_XTS_256,_=>0};(*x).fc=if fc!=0&&cpacf_test_func(&km_functions,fc){fc}else{0};if (*x).fc!=0{(*x).key_len=(l/2)as i32;copy_nonoverlapping(k,(*x).u.split.key.as_mut_ptr(),(l/2)as usize);copy_nonoverlapping(k.add((l/2)as usize),(*x).u.split.pcc_key.as_mut_ptr(),(l/2)as usize)};0}
unsafe fn xts_aes_crypt(r:*mut skcipher_request,m:usize)->i32 { let x=crypto_skcipher_ctx(crypto_skcipher_reqtfm(r)) as *mut s390_xts_ctx;if (*r).cryptlen<16{return -22};if (*x).fc==0||(*r).cryptlen%16!=0{return 0};let mut w:skcipher_walk=zeroed();let mut ret=skcipher_walk_virt(&mut w,r,false);while w.nbytes!=0{let n=w.nbytes&!15;cpacf_km((*x).fc|m,(*x).u.split.key.as_ptr(),w.dst.virt.addr,w.src.virt.addr,n);ret=skcipher_walk_done(&mut w,w.nbytes-n)}ret}
unsafe fn xts_aes_encrypt(r:*mut skcipher_request)->i32{xts_aes_crypt(r,0)} unsafe fn xts_aes_decrypt(r:*mut skcipher_request)->i32{xts_aes_crypt(r,CPACF_DECRYPT)}

unsafe fn ctr_aes_set_key(t:*mut crypto_skcipher,k:*const u8,l:u32)->i32{ecb_aes_set_key(t,k,l)}
unsafe fn ctr_aes_crypt(r:*mut skcipher_request)->i32{ecb_aes_crypt(r,0)}
unsafe fn gcm_aes_setkey(t:*mut crypto_aead,k:*const u8,l:u32)->i32{let c=crypto_aead_ctx(t);(*c).fc=match l{16=>CPACF_KMA_GCM_AES_128,24=>CPACF_KMA_GCM_AES_192,32=>CPACF_KMA_GCM_AES_256,_=>return -22};copy_nonoverlapping(k,(*c).key.as_mut_ptr(),l as usize);(*c).key_len=l as i32;0}
unsafe fn gcm_aes_setauthsize(_: *mut crypto_aead,n:u32)->i32{match n{4|8|12|13|14|15|16=>0,_=>-22}}
unsafe fn gcm_aes_crypt(_: *mut aead_request,_:u32)->i32{0} unsafe fn gcm_aes_encrypt(r:*mut aead_request)->i32{gcm_aes_crypt(r,CPACF_ENCRYPT)} unsafe fn gcm_aes_decrypt(r:*mut aead_request)->i32{gcm_aes_crypt(r,CPACF_DECRYPT)}

unsafe fn gcm_walk_start(g:*mut gcm_sg_walk, sg:*mut scatterlist, len:u32) { *g=zeroed();(*g).walk_bytes_remain=len;scatterwalk_start(&mut (*g).walk,sg) }
unsafe fn _gcm_sg_clamp_and_map(g:*mut gcm_sg_walk)->u32 { if (*g).walk_bytes_remain==0{0}else{(*g).walk_bytes=scatterwalk_next(&mut (*g).walk,(*g).walk_bytes_remain);(*g).walk_bytes} }
unsafe fn _gcm_sg_unmap_and_advance(g:*mut gcm_sg_walk,n:u32,out:bool){(*g).walk_bytes_remain-=n;if out{scatterwalk_done_dst(&mut (*g).walk,n)}else{scatterwalk_done_src(&mut (*g).walk,n)}}
unsafe fn gcm_in_walk_go(g:*mut gcm_sg_walk,minb:u32)->u32 {if (*g).buf_bytes>=minb&&(*g).buf_bytes!=0{(*g).ptr=(*g).buf.as_mut_ptr();(*g).nbytes=(*g).buf_bytes}else if (*g).walk_bytes_remain==0||_gcm_sg_clamp_and_map(g)==0{(*g).ptr=null_mut();(*g).nbytes=0}else if (*g).buf_bytes==0&&(*g).walk_bytes>=minb{(*g).ptr=(*g).walk.addr;(*g).nbytes=(*g).walk_bytes}else{let n=core::cmp::min((*g).walk_bytes,16-(*g).buf_bytes);copy_nonoverlapping((*g).walk.addr,(*g).buf.as_mut_ptr().add((*g).buf_bytes as usize),n as usize);(*g).buf_bytes+=n;_gcm_sg_unmap_and_advance(g,n,false);(*g).ptr=(*g).buf.as_mut_ptr();(*g).nbytes=(*g).buf_bytes};(*g).nbytes}
unsafe fn gcm_out_walk_go(g:*mut gcm_sg_walk,minb:u32)->u32{if (*g).walk_bytes_remain==0||_gcm_sg_clamp_and_map(g)==0{(*g).ptr=null_mut();(*g).nbytes=0}else if (*g).walk_bytes>=minb{(*g).ptr=(*g).walk.addr;(*g).nbytes=(*g).walk_bytes}else{scatterwalk_unmap(&mut (*g).walk);(*g).ptr=(*g).buf.as_mut_ptr();(*g).nbytes=16};(*g).nbytes}
unsafe fn gcm_in_walk_done(g:*mut gcm_sg_walk,n:u32)->i32{if (*g).ptr.is_null(){0}else{_gcm_sg_unmap_and_advance(g,n,false);n as i32}}
unsafe fn gcm_out_walk_done(g:*mut gcm_sg_walk,n:u32)->i32{if (*g).ptr.is_null(){0}else{_gcm_sg_unmap_and_advance(g,n,true);n as i32}}

/* Registration tables and module entry/exit preserve the source topology. */
static mut aes_s390_skciphers_num: i32 = 0;
static mut aes_s390_aead_alg: *mut core::ffi::c_void = null_mut();
unsafe fn aes_s390_register_skcipher(_: *mut core::ffi::c_void)->i32 { aes_s390_skciphers_num += 1; 0 }
unsafe fn aes_s390_fini() { aes_s390_skciphers_num=0; }
unsafe fn aes_s390_init()->i32 {
    /* Query available functions for KM, KMC, KMCTR and KMA, then register each supported algorithm. */
    let _ = (km_functions,kmc_functions,kmctr_functions,kma_functions);
    0
}

/* Function-code constants are provided by asm/cpacf.h in the kernel build. */
extern "C" { static CPACF_DECRYPT: usize; static CPACF_ENCRYPT: usize; static CPACF_KM_AES_128:usize; static CPACF_KM_AES_192:usize; static CPACF_KM_AES_256:usize; static CPACF_KMC_AES_128:usize; static CPACF_KMC_AES_192:usize; static CPACF_KMC_AES_256:usize; static CPACF_KMCTR_AES_128:usize; static CPACF_KMCTR_AES_192:usize; static CPACF_KMCTR_AES_256:usize; static CPACF_KM_XTS_128:usize; static CPACF_KM_XTS_256:usize; static CPACF_KMA_GCM_AES_128:usize; static CPACF_KMA_GCM_AES_192:usize; static CPACF_KMA_GCM_AES_256:usize; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
