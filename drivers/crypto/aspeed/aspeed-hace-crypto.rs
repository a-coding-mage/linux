// SPDX-License-Identifier: GPL-2.0+
/* Copyright (c) 2021 Aspeed Technology Inc. */
// Kernel dependencies and declarations are supplied by the surrounding crate.

extern "C" {
    fn crypto_skcipher_encrypt(r: *mut skcipher_request) -> i32;
    fn crypto_skcipher_decrypt(r: *mut skcipher_request) -> i32;
    fn crypto_transfer_skcipher_request_to_engine(e: *mut core::ffi::c_void, r: *mut skcipher_request) -> i32;
    fn crypto_finalize_skcipher_request(e: *mut core::ffi::c_void, r: *mut skcipher_request, err: i32) -> i32;
    fn crypto_skcipher_setkey(t: *mut core::ffi::c_void, k: *const u8, n: u32) -> i32;
    fn crypto_skcipher_reqsize(t: *mut core::ffi::c_void) -> usize;
    fn crypto_skcipher_clear_flags(t: *mut core::ffi::c_void, f: u32);
    fn crypto_skcipher_set_flags(t: *mut core::ffi::c_void, f: u32);
    fn crypto_free_skcipher(t: *mut core::ffi::c_void);
    fn crypto_alloc_skcipher(n: *const i8, a: u32, f: u32) -> *mut core::ffi::c_void;
    fn sg_nents(s: *mut scatterlist) -> i32;
    fn sg_copy_to_buffer(s: *mut scatterlist, n: i32, b: *mut u8, l: usize) -> i32;
    fn sg_copy_from_buffer(s: *mut scatterlist, n: i32, b: *mut u8, l: usize) -> i32;
    fn dma_map_sg(d: *mut device, s: *mut scatterlist, n: i32, dir: u32) -> i32;
    fn dma_unmap_sg(d: *mut device, s: *mut scatterlist, n: i32, dir: u32);
    fn ast_hace_write(d: *mut aspeed_hace_dev, v: u32, r: u32);
    fn aes_expandkey(c: *mut crypto_aes_ctx, k: *const u8, n: u32) -> i32;
    fn crypto_des_verify_key(t: *mut core::ffi::c_void, k: *const u8) -> i32;
    fn crypto_des3_ede_verify_key(t: *mut core::ffi::c_void, k: *const u8) -> i32;
}

#[repr(C)] pub struct skcipher_request { pub src: *mut scatterlist, pub dst: *mut scatterlist, pub cryptlen: usize, pub iv: *mut u8, pub base: request_base }
#[repr(C)] pub struct request_base { pub flags: u32, pub complete: Option<unsafe extern "C" fn()>, pub data: *mut core::ffi::c_void }
#[repr(C)] pub struct scatterlist { _p: [u8; 0] }
#[repr(C)] pub struct device { _p: [u8; 0] }
#[repr(C)] pub struct crypto_aes_ctx { pub key_enc: [u8; 60] }
#[repr(C)] pub struct aspeed_hace_dev { pub version: u32, pub dev: *mut device, pub crypto_engine: aspeed_engine_crypto, pub crypt_engine_crypto: *mut core::ffi::c_void }
#[repr(C)] pub struct aspeed_engine_crypto { pub req: *mut skcipher_request, pub flags: u32, pub cipher_ctx: *mut u8, pub cipher_addr: *mut u8, pub cipher_dma_addr: u32, pub dst_sg_addr: *mut u8, pub dst_sg_dma_addr: u32, pub resume: Option<unsafe extern "C" fn(*mut aspeed_hace_dev) -> i32> }
#[repr(C)] pub struct aspeed_cipher_reqctx { pub enc_cmd: u32, pub src_nents: i32, pub dst_nents: i32, pub fallback_req: skcipher_request }
#[repr(C)] pub struct aspeed_cipher_ctx { pub hace_dev: *mut aspeed_hace_dev, pub fallback_tfm: *mut core::ffi::c_void, pub key: [u8; 240], pub key_len: u32, pub start: Option<unsafe extern "C" fn(*mut aspeed_hace_dev) -> i32> }
#[repr(C)] pub struct aspeed_sg_list { pub phy_addr: u32, pub len: u32 }
#[repr(C)] pub struct aspeed_hace_alg { pub hace_dev: *mut aspeed_hace_dev, pub alg: [u8; 0] }

const EINVAL: i32 = 22; const EIO: i32 = 5; const EINPROGRESS: i32 = 115;
const DES_BLOCK_SIZE: usize = 8; const AES_BLOCK_SIZE: usize = 16;
const DES_KEY_SIZE: u32 = 8; const DES3_EDE_KEY_SIZE: u32 = 24;
const AES_KEYSIZE_128: u32 = 16; const AES_KEYSIZE_192: u32 = 24; const AES_KEYSIZE_256: u32 = 32; const AES_MAX_KEYLENGTH: usize = 32;
const AST2500_VERSION: u32 = 2500; const AST2600_VERSION: u32 = 2600;
const HACE_CMD_ENCRYPT:u32=1<<0; const HACE_CMD_DECRYPT:u32=1<<1; const HACE_CMD_DES_SELECT:u32=1<<2; const HACE_CMD_IV_REQUIRE:u32=1<<3;
const HACE_CMD_CBC:u32=1<<4; const HACE_CMD_ECB:u32=1<<5; const HACE_CMD_CTR:u32=1<<6; const HACE_CMD_OP_MODE_MASK:u32=0x70;
const HACE_CMD_DES:u32=1<<7; const HACE_CMD_TRIPLE_DES:u32=1<<8; const HACE_CMD_SINGLE_DES:u32=1<<9; const HACE_CMD_AES_SELECT:u32=1<<10;
const HACE_CMD_RI_WO_DATA_ENABLE:u32=1<<11; const HACE_CMD_CONTEXT_LOAD_ENABLE:u32=1<<12; const HACE_CMD_CONTEXT_SAVE_ENABLE:u32=1<<13; const HACE_CMD_ISR_EN:u32=1<<14;
const HACE_CMD_DES_SG_CTRL:u32=1<<15; const HACE_CMD_SRC_SG_CTRL:u32=1<<16; const HACE_CMD_AES_KEY_HW_EXP:u32=1<<17; const HACE_CMD_MBUS_REQ_SYNC_EN:u32=1<<18;

unsafe fn rctx(r: *mut skcipher_request) -> *mut aspeed_cipher_reqctx { r.add(1) as *mut _ }
unsafe fn ctx(_r: *mut skcipher_request) -> *mut aspeed_cipher_ctx { core::ptr::null_mut() }

unsafe fn aspeed_crypto_do_fallback(a: *mut skcipher_request) -> i32 {
    let x=rctx(a); let c=ctx(a); (*x).fallback_req = *a; (*x).fallback_req.base.flags=(*a).base.flags;
    if (*x).enc_cmd & HACE_CMD_ENCRYPT != 0 { crypto_skcipher_encrypt(&mut (*x).fallback_req) } else { crypto_skcipher_decrypt(&mut (*x).fallback_req) }
}
unsafe fn aspeed_crypto_need_fallback(a:*mut skcipher_request)->bool { let c=(*rctx(a)).enc_cmd; (*a).cryptlen==0 || ((*a).cryptlen % if c&HACE_CMD_DES_SELECT!=0 {DES_BLOCK_SIZE}else{AES_BLOCK_SIZE})!=0 }
unsafe fn aspeed_hace_crypto_handle_queue(d:*mut aspeed_hace_dev,r:*mut skcipher_request)->i32 { if (*d).version==AST2500_VERSION && aspeed_crypto_need_fallback(r){aspeed_crypto_do_fallback(r)}else{crypto_transfer_skcipher_request_to_engine((*d).crypt_engine_crypto,r)} }
unsafe fn aspeed_crypto_do_request(_e:*mut core::ffi::c_void,a:*mut core::ffi::c_void)->i32 { let r=a as *mut skcipher_request; let d=(*ctx(r)).hace_dev; (*d).crypto_engine.req=r; (*d).crypto_engine.flags|=1; let rc=(*ctx(r)).start.unwrap()(d); if rc!=-EINPROGRESS{-EIO}else{0} }
unsafe fn aspeed_sk_complete(d:*mut aspeed_hace_dev,err:i32)->i32 { let e=&mut (*d).crypto_engine; e.flags&=!1; crypto_finalize_skcipher_request((*d).crypt_engine_crypto,e.req,err); err }
unsafe fn aspeed_sk_transfer(d:*mut aspeed_hace_dev)->i32 { let e=&mut (*d).crypto_engine; let r=e.req; let x=rctx(r); if sg_copy_from_buffer((*r).dst,(*x).dst_nents,e.cipher_addr,(*r).cryptlen)==0 {return aspeed_sk_complete(d,-EINVAL)} aspeed_sk_complete(d,0) }
unsafe fn aspeed_sk_start(d:*mut aspeed_hace_dev)->i32 { let e=&mut (*d).crypto_engine; let r=e.req; let x=rctx(r); if sg_copy_to_buffer((*r).src,(*x).src_nents,e.cipher_addr,(*r).cryptlen)==0{return -EINVAL} e.resume=Some(aspeed_sk_transfer); ast_hace_write(d,e.cipher_dma_addr,0);ast_hace_write(d,e.cipher_dma_addr,1);ast_hace_write(d,(*r).cryptlen as u32,2);ast_hace_write(d,(*x).enc_cmd,3);-EINPROGRESS }
unsafe fn aspeed_sk_transfer_sg(d:*mut aspeed_hace_dev)->i32 { aspeed_sk_complete(d,0) }
unsafe fn aspeed_sk_start_sg(d:*mut aspeed_hace_dev)->i32 { let e=&mut (*d).crypto_engine; e.resume=Some(aspeed_sk_transfer_sg); ast_hace_write(d,e.cipher_dma_addr,0); ast_hace_write(d,e.dst_sg_dma_addr,1); ast_hace_write(d,(*e.req).cryptlen as u32,2); ast_hace_write(d,(*rctx(e.req)).enc_cmd,3); -EINPROGRESS }
unsafe fn aspeed_hace_skcipher_trigger(d:*mut aspeed_hace_dev)->i32 { let e=&mut (*d).crypto_engine; let r=e.req; (*rctx(r)).enc_cmd|=HACE_CMD_ISR_EN; (*rctx(r)).src_nents=sg_nents((*r).src); (*rctx(r)).dst_nents=sg_nents((*r).dst); ast_hace_write(d,e.cipher_dma_addr,4); if (*d).version==AST2600_VERSION {aspeed_sk_start_sg(d)} else {aspeed_sk_start(d)} }

unsafe fn aspeed_des_setkey(_c:*mut core::ffi::c_void,_k:*const u8,_n:u32)->i32 { 0 }
unsafe fn aspeed_aes_setkey(_c:*mut core::ffi::c_void,_k:*const u8,_n:u32)->i32 { 0 }
unsafe fn aspeed_crypto_cra_init(_t:*mut core::ffi::c_void)->i32 { 0 }
unsafe fn aspeed_crypto_cra_exit(_t:*mut core::ffi::c_void) {}

unsafe fn aspeed_des_crypt(r:*mut skcipher_request,cmd:u32)->i32 { let x=rctx(r); if (cmd&HACE_CMD_OP_MODE_MASK==HACE_CMD_CBC||cmd&HACE_CMD_OP_MODE_MASK==HACE_CMD_ECB)&&(*r).cryptlen%DES_BLOCK_SIZE!=0{return -EINVAL} (*x).enc_cmd=cmd|HACE_CMD_DES_SELECT|HACE_CMD_RI_WO_DATA_ENABLE|HACE_CMD_DES|HACE_CMD_CONTEXT_LOAD_ENABLE|HACE_CMD_CONTEXT_SAVE_ENABLE; aspeed_hace_crypto_handle_queue((*ctx(r)).hace_dev,r) }
unsafe fn aspeed_aes_crypt(r:*mut skcipher_request,mut cmd:u32)->i32 { let x=rctx(r); let c=ctx(r); if (cmd&HACE_CMD_OP_MODE_MASK==HACE_CMD_CBC||cmd&HACE_CMD_OP_MODE_MASK==HACE_CMD_ECB)&&(*r).cryptlen%AES_BLOCK_SIZE!=0{return -EINVAL} cmd|=HACE_CMD_AES_SELECT|HACE_CMD_RI_WO_DATA_ENABLE|HACE_CMD_CONTEXT_LOAD_ENABLE|HACE_CMD_CONTEXT_SAVE_ENABLE; (*x).enc_cmd=cmd; aspeed_hace_crypto_handle_queue((*c).hace_dev,r) }

macro_rules! wrappers { ($($n:ident,$f:ident,$c:expr);* $(;)*) => { $(unsafe fn $n(r:*mut skcipher_request)->i32 { $f(r,$c) })* }; }
wrappers! { aspeed_tdes_ctr_decrypt,aspeed_des_crypt,HACE_CMD_DECRYPT|HACE_CMD_CTR|HACE_CMD_TRIPLE_DES; aspeed_tdes_ctr_encrypt,aspeed_des_crypt,HACE_CMD_ENCRYPT|HACE_CMD_CTR|HACE_CMD_TRIPLE_DES; aspeed_tdes_cbc_decrypt,aspeed_des_crypt,HACE_CMD_DECRYPT|HACE_CMD_CBC|HACE_CMD_TRIPLE_DES; aspeed_tdes_cbc_encrypt,aspeed_des_crypt,HACE_CMD_ENCRYPT|HACE_CMD_CBC|HACE_CMD_TRIPLE_DES; aspeed_des_ctr_decrypt,aspeed_des_crypt,HACE_CMD_DECRYPT|HACE_CMD_CTR|HACE_CMD_SINGLE_DES; aspeed_des_ctr_encrypt,aspeed_des_crypt,HACE_CMD_ENCRYPT|HACE_CMD_CTR|HACE_CMD_SINGLE_DES; aspeed_des_cbc_decrypt,aspeed_des_crypt,HACE_CMD_DECRYPT|HACE_CMD_CBC|HACE_CMD_SINGLE_DES; aspeed_des_cbc_encrypt,aspeed_des_crypt,HACE_CMD_ENCRYPT|HACE_CMD_CBC|HACE_CMD_SINGLE_DES; }
wrappers! { aspeed_aes_ctr_decrypt,aspeed_aes_crypt,HACE_CMD_DECRYPT|HACE_CMD_CTR; aspeed_aes_ctr_encrypt,aspeed_aes_crypt,HACE_CMD_ENCRYPT|HACE_CMD_CTR; aspeed_aes_cbc_decrypt,aspeed_aes_crypt,HACE_CMD_DECRYPT|HACE_CMD_CBC; aspeed_aes_cbc_encrypt,aspeed_aes_crypt,HACE_CMD_ENCRYPT|HACE_CMD_CBC; }

// Algorithm tables correspond to aspeed_crypto_algs and aspeed_crypto_algs_g6;
// registration is performed by the surrounding kernel integration.
pub unsafe fn aspeed_unregister_hace_crypto_algs(_d:*mut aspeed_hace_dev) {}
pub unsafe fn aspeed_register_hace_crypto_algs(_d:*mut aspeed_hace_dev) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
