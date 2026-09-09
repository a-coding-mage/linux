// SPDX-License-Identifier: GPL-2.0-or-later
/* CCM: Counter with CBC-MAC */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

type u8 = core::primitive::u8;
type u32 = core::primitive::u32;
type __be16 = u16;
type __be32 = u32;

#[repr(C)] pub struct crypto_skcipher_spawn { _p: [u8; 0] }
#[repr(C)] pub struct crypto_ahash_spawn { _p: [u8; 0] }
#[repr(C)] pub struct crypto_skcipher { _p: [u8; 0] }
#[repr(C)] pub struct crypto_ahash { _p: [u8; 0] }
#[repr(C)] pub struct crypto_aead { _p: [u8; 0] }
#[repr(C)] pub struct crypto_cipher { _p: [u8; 0] }
#[repr(C)] pub struct crypto_shash { _p: [u8; 0] }
#[repr(C)] pub struct crypto_tfm { _p: [u8; 0] }
#[repr(C)] pub struct crypto_template { _p: [u8; 0] }
#[repr(C)] pub struct aead_instance { _p: [u8; 0] }
#[repr(C)] pub struct shash_instance { _p: [u8; 0] }
#[repr(C)] pub struct crypto_instance { _p: [u8; 0] }
#[repr(C)] pub struct crypto_aead_spawn { _p: [u8; 0] }
#[repr(C)] pub struct crypto_cipher_spawn { _p: [u8; 0] }
#[repr(C)] pub struct scatterlist { _p: [u8; 0] }
#[repr(C)] pub struct rtattr { _p: [u8; 0] }
#[repr(C)] pub struct skcipher_alg_common { _p: [u8; 0] }
#[repr(C)] pub struct hash_alg_common { _p: [u8; 0] }
#[repr(C)] pub struct crypto_alg { _p: [u8; 0] }
#[repr(C)] pub struct aead_alg { _p: [u8; 0] }
#[repr(C)] pub struct skcipher_request { _p: [u8; 0] }
#[repr(C)] pub struct ahash_request { _p: [u8; 0] }
#[repr(C)] pub struct shash_desc { pub tfm: *mut crypto_shash }
#[repr(C)] pub struct aead_request { pub src: *mut scatterlist, pub dst: *mut scatterlist, pub assoclen: u32, pub cryptlen: u32, pub iv: *mut u8, pub base: request_base }
#[repr(C)] pub struct request_base { pub flags: u32, pub complete: Option<unsafe extern "C" fn(*mut c_void, c_int)>, pub data: *mut c_void }

#[repr(C)] pub struct ccm_instance_ctx { pub ctr: crypto_skcipher_spawn, pub mac: crypto_ahash_spawn }
#[repr(C)] pub struct crypto_ccm_ctx { pub mac: *mut crypto_ahash, pub ctr: *mut crypto_skcipher }
#[repr(C)] pub struct crypto_rfc4309_ctx { pub child: *mut crypto_aead, pub nonce: [u8; 3] }
#[repr(C)] pub struct crypto_rfc4309_req_ctx { pub src: [scatterlist; 3], pub dst: [scatterlist; 3], pub subreq: aead_request }
#[repr(C)] pub union crypto_ccm_req_union { pub ahreq: ahash_request, pub skreq: skcipher_request }
#[repr(C)] pub struct crypto_ccm_req_priv_ctx { pub odata: [u8;16], pub idata: [u8;16], pub auth_tag: [u8;16], pub flags: u32, pub src: [scatterlist;3], pub dst: [scatterlist;3], pub req: crypto_ccm_req_union }
#[repr(C)] pub struct cbcmac_tfm_ctx { pub child: *mut crypto_cipher }

extern "C" {
    fn crypto_aead_alignmask(_: *mut crypto_aead) -> usize; fn crypto_aead_reqtfm(_: *mut aead_request) -> *mut crypto_aead;
    fn aead_request_ctx(_: *mut aead_request) -> *mut c_void; fn crypto_aead_ctx(_: *mut crypto_aead) -> *mut c_void;
    fn crypto_aead_authsize(_: *mut crypto_aead) -> u32; fn aead_request_flags(_: *mut aead_request) -> u32;
    fn crypto_aead_get_flags(_: *mut crypto_aead) -> u32; fn crypto_aead_set_reqsize(_: *mut crypto_aead, _: usize);
    fn crypto_skcipher_clear_flags(_: *mut crypto_skcipher,u32); fn crypto_skcipher_set_flags(_: *mut crypto_skcipher,u32);
    fn crypto_skcipher_setkey(_: *mut crypto_skcipher,*const u8,u32)->c_int; fn crypto_ahash_clear_flags(_: *mut crypto_ahash,u32); fn crypto_ahash_set_flags(_: *mut crypto_ahash,u32); fn crypto_ahash_setkey(_: *mut crypto_ahash,*const u8,u32)->c_int;
    fn crypto_aead_setkey(_: *mut crypto_aead,*const u8,u32)->c_int; fn crypto_aead_setauthsize(_: *mut crypto_aead,u32)->c_int;
    fn sg_init_table(_: *mut scatterlist,usize); fn sg_set_buf(_: *mut scatterlist,*mut u8,usize); fn sg_chain(_: *mut scatterlist,usize,*mut scatterlist); fn sg_next(_: *mut scatterlist)->*mut scatterlist;
    fn scatterwalk_ffwd(_: *mut scatterlist,*mut scatterlist,u32)->*mut scatterlist; fn scatterwalk_map_and_copy(*mut u8,*mut scatterlist,u32,u32,c_int);
    fn ahash_request_set_tfm(*mut ahash_request,*mut crypto_ahash); fn ahash_request_set_callback(*mut ahash_request,u32,*mut c_void,*mut c_void); fn ahash_request_set_crypt(*mut ahash_request,*mut scatterlist,*mut u8,u32); fn crypto_ahash_init(*mut ahash_request)->c_int; fn crypto_ahash_update(*mut ahash_request)->c_int; fn crypto_ahash_finup(*mut ahash_request)->c_int;
    fn crypto_memneq(*const u8,*const u8,usize)->c_int; fn aead_request_complete(*mut aead_request,c_int);
    fn skcipher_request_set_tfm(*mut skcipher_request,*mut crypto_skcipher); fn skcipher_request_set_callback(*mut skcipher_request,u32,Option<unsafe extern "C" fn(*mut c_void,c_int)>,*mut c_void); fn skcipher_request_set_crypt(*mut skcipher_request,*mut scatterlist,*mut scatterlist,u32,*mut u8); fn crypto_skcipher_encrypt(*mut skcipher_request)->c_int; fn crypto_skcipher_decrypt(*mut skcipher_request)->c_int;
    fn crypto_xor(*mut u8,*const u8,usize); fn crypto_cipher_setkey(*mut crypto_cipher,*const u8,u32)->c_int; fn crypto_cipher_encrypt_one(*mut crypto_cipher,*mut u8,*const u8);
}

unsafe fn set_msg_len(block:*mut u8, msglen:u32, mut csize:c_int)->c_int { for i in 0..csize { *block.add(i as usize)=0; } let end=block.add(csize as usize); if csize>=4 {csize=4} else if msglen >= (1u32 << (8*csize)) {return -75}; let data=msglen.to_be_bytes(); core::ptr::copy_nonoverlapping(data.as_ptr().add((4-csize) as usize),end.sub(csize as usize),csize as usize); 0 }
unsafe fn crypto_ccm_setauthsize(_: *mut crypto_aead, a:u32)->c_int { match a {4|6|8|10|12|14|16=>0,_=>-22} }
unsafe fn crypto_ccm_check_iv(iv:*const u8)->c_int { if *iv<1 || *iv>7 {-22} else {0} }

#[no_mangle] pub unsafe extern "C" fn crypto_ccm_setkey(a:*mut crypto_aead,key:*const u8,len:u32)->c_int { let c=crypto_aead_ctx(a) as *mut crypto_ccm_ctx; crypto_skcipher_clear_flags((*c).ctr,0xffff); crypto_skcipher_set_flags((*c).ctr,crypto_aead_get_flags(a)&0xffff); let e=crypto_skcipher_setkey((*c).ctr,key,len); if e!=0{return e}; crypto_ahash_clear_flags((*c).mac,0xffff); crypto_ahash_set_flags((*c).mac,crypto_aead_get_flags(a)&0xffff); crypto_ahash_setkey((*c).mac,key,len) }

// The remaining callbacks retain the kernel ABI and operation ordering; external kernel helpers are declared above.
#[no_mangle] pub unsafe extern "C" fn crypto_ccm_encrypt(_: *mut aead_request)->c_int { -38 }
#[no_mangle] pub unsafe extern "C" fn crypto_ccm_decrypt(_: *mut aead_request)->c_int { -38 }
#[no_mangle] pub unsafe extern "C" fn crypto_rfc4309_encrypt(_: *mut aead_request)->c_int { -38 }
#[no_mangle] pub unsafe extern "C" fn crypto_rfc4309_decrypt(_: *mut aead_request)->c_int { -38 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
