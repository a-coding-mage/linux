// SPDX-License-Identifier: GPL-2.0
/* HCTR2 length-preserving encryption mode */

// The following types and functions are supplied by the surrounding kernel
// crypto implementation.
use core::ffi::{c_char, c_int, c_uint, c_void};

const BLOCKCIPHER_BLOCK_SIZE: usize = 16;
const TWEAK_SIZE: usize = 32;
const POLYVAL_BLOCK_SIZE: usize = 16;
const POLYVAL_DIGEST_SIZE: usize = 16;

#[repr(C)] pub struct crypto_cipher_spawn { _private: [u8; 0] }
#[repr(C)] pub struct crypto_skcipher_spawn { _private: [u8; 0] }
#[repr(C)] pub struct crypto_cipher { _private: [u8; 0] }
#[repr(C)] pub struct crypto_skcipher { _private: [u8; 0] }
#[repr(C)] pub struct polyval_key { _private: [u8; 0] }
#[repr(C)] pub struct polyval_elem { _private: [u8; 16] }
#[repr(C)] pub struct polyval_ctx { _private: [u8; 0] }
#[repr(C)] pub struct scatterlist { _private: [u8; 0] }
#[repr(C)] pub struct sg_mapping_iter { addr: *mut u8, length: usize }
#[repr(C)] pub struct crypto_template { _private: [u8; 0] }
#[repr(C)] pub struct rtattr { _private: [u8; 0] }
#[repr(C)] pub struct crypto_alg { cra_name: [c_char; 64], cra_driver_name: [c_char; 64], cra_blocksize: u32, cra_alignmask: u32, cra_priority: i32 }
#[repr(C)] pub struct skcipher_alg_common { base: crypto_alg, min_keysize: u32, max_keysize: u32 }
#[repr(C)] pub struct skcipher_instance { alg: skcipher_alg, free: Option<unsafe extern "C" fn(*mut skcipher_instance)> }
#[repr(C)] pub struct skcipher_alg { base: crypto_alg, setkey: Option<unsafe extern "C" fn(*mut skcipher_request,*const u8,u32)->i32>, encrypt: Option<unsafe extern "C" fn(*mut skcipher_request)->i32>, decrypt: Option<unsafe extern "C" fn(*mut skcipher_request)->i32>, init: Option<unsafe extern "C" fn(*mut crypto_skcipher)->i32>, exit: Option<unsafe extern "C" fn(*mut crypto_skcipher)>, min_keysize: u32, max_keysize: u32, ivsize: u32 }
#[repr(C)] pub struct skcipher_request { base: request_base, cryptlen: usize, src: *mut scatterlist, dst: *mut scatterlist, iv: *mut u8 }
#[repr(C)] pub struct request_base { flags: u32 }
#[repr(C)] pub union hctr2_union { poly_ctx: polyval_ctx, xctr_req: skcipher_request }

#[repr(C)] pub struct hctr2_instance_ctx { blockcipher_spawn: crypto_cipher_spawn, xctr_spawn: crypto_skcipher_spawn }
#[repr(C)] pub struct hctr2_tfm_ctx { blockcipher: *mut crypto_cipher, xctr: *mut crypto_skcipher, poly_key: polyval_key, hashed_tweaklens: [polyval_elem; 2], l: [u8; BLOCKCIPHER_BLOCK_SIZE] }
#[repr(C)] pub struct hctr2_request_ctx { first_block: [u8; BLOCKCIPHER_BLOCK_SIZE], xctr_iv: [u8; BLOCKCIPHER_BLOCK_SIZE], bulk_part_dst: *mut scatterlist, bulk_part_src: *mut scatterlist, sg_src: [scatterlist; 2], sg_dst: [scatterlist; 2], hashed_tweak: polyval_elem, u: hctr2_union }

extern "C" {
    fn polyval_init(*mut polyval_ctx, *const polyval_key);
    fn polyval_update(*mut polyval_ctx, *const u8, usize);
    fn polyval_export_blkaligned(*mut polyval_ctx, *mut polyval_elem);
    fn polyval_import_blkaligned(*mut polyval_ctx, *const polyval_key, *const polyval_elem);
    fn polyval_preparekey(*mut polyval_key, *const u8);
    fn polyval_final(*mut polyval_ctx, *mut u8);
    fn crypto_skcipher_ctx(*mut crypto_skcipher) -> *mut hctr2_tfm_ctx;
    fn crypto_skcipher_reqtfm(*mut skcipher_request) -> *mut crypto_skcipher;
    fn skcipher_request_ctx(*mut skcipher_request) -> *mut hctr2_request_ctx;
    fn crypto_cipher_setkey(*mut crypto_cipher,*const u8,u32)->i32;
    fn crypto_skcipher_setkey(*mut crypto_skcipher,*const u8,u32)->i32;
    fn crypto_cipher_encrypt_one(*mut crypto_cipher,*mut u8,*const u8);
    fn crypto_cipher_decrypt_one(*mut crypto_cipher,*mut u8,*const u8);
    fn crypto_xor(*mut u8,*const u8,usize);
    fn crypto_xor_cpy(*mut u8,*const u8,*const u8,usize);
    fn crypto_skcipher_encrypt(*mut skcipher_request)->i32;
    fn skcipher_request_complete(*mut skcipher_request,i32);
    fn skcipher_request_set_tfm(*mut skcipher_request,*mut crypto_skcipher);
    fn skcipher_request_set_crypt(*mut skcipher_request,*mut scatterlist,*mut scatterlist,usize,*mut u8);
    fn skcipher_request_set_callback(*mut skcipher_request,u32,Option<unsafe extern "C" fn(*mut c_void,i32)>,*mut skcipher_request);
    fn scatterwalk_map_and_copy(*mut u8,*mut scatterlist,usize,usize,i32);
    fn scatterwalk_ffwd(*mut scatterlist,*mut scatterlist,usize)->*mut scatterlist;
    fn sg_miter_start(*mut sg_mapping_iter,*mut scatterlist,usize,u32);
    fn sg_miter_next(*mut sg_mapping_iter)->i32;
    fn sg_miter_stop(*mut sg_mapping_iter);
    fn sg_nents(*mut scatterlist)->usize;
    fn crypto_spawn_skcipher(*mut crypto_skcipher_spawn)->*mut crypto_skcipher;
    fn crypto_spawn_cipher(*mut crypto_cipher_spawn)->*mut crypto_cipher;
    fn crypto_free_skcipher(*mut crypto_skcipher);
    fn crypto_free_cipher(*mut crypto_cipher);
    fn crypto_register_templates(*mut crypto_template,usize)->i32;
    fn crypto_unregister_templates(*mut crypto_template,usize)->i32;
    fn memzero_explicit(*mut c_void,usize);
}

unsafe fn hctr2_hash_tweaklens(tctx: *mut hctr2_tfm_ctx) {
    let mut ctx = core::mem::MaybeUninit::<polyval_ctx>::zeroed().assume_init();
    for has_remainder in 0..2 {
        let tweak_length_block: [u64; 2] = [(TWEAK_SIZE * 8 * 2 + 2 + has_remainder) as u64, 0];
        polyval_init(&mut ctx, &(*tctx).poly_key);
        polyval_update(&mut ctx, tweak_length_block.as_ptr() as *const u8, core::mem::size_of_val(&tweak_length_block));
        polyval_export_blkaligned(&mut ctx, &mut (*tctx).hashed_tweaklens[has_remainder]);
    }
    memzero_explicit(&mut ctx as *mut _ as *mut c_void, core::mem::size_of::<polyval_ctx>());
}

unsafe extern "C" fn hctr2_setkey(tfm: *mut skcipher_request, key: *const u8, keylen: u32) -> i32 {
    let tctx = crypto_skcipher_ctx(crypto_skcipher_reqtfm(tfm));
    let mut hbar = [0u8; BLOCKCIPHER_BLOCK_SIZE];
    let err = crypto_cipher_setkey((*tctx).blockcipher, key, keylen); if err != 0 { return err; }
    let err = crypto_skcipher_setkey((*tctx).xctr, key, keylen); if err != 0 { return err; }
    crypto_cipher_encrypt_one((*tctx).blockcipher, hbar.as_mut_ptr(), hbar.as_ptr());
    (*tctx).l = [0; BLOCKCIPHER_BLOCK_SIZE]; (*tctx).l[0] = 1;
    crypto_cipher_encrypt_one((*tctx).blockcipher, (*tctx).l.as_mut_ptr(), (*tctx).l.as_ptr());
    polyval_preparekey(&mut (*tctx).poly_key, hbar.as_ptr());
    memzero_explicit(hbar.as_mut_ptr() as *mut c_void, hbar.len());
    hctr2_hash_tweaklens(tctx); 0
}

unsafe fn hctr2_hash_tweak(req: *mut skcipher_request) {
    let tctx = crypto_skcipher_ctx(crypto_skcipher_reqtfm(req)); let rctx = skcipher_request_ctx(req);
    let p = &mut (*rctx).u.poly_ctx; let rem = ((*req).cryptlen % POLYVAL_BLOCK_SIZE) != 0;
    polyval_import_blkaligned(p, &(*tctx).poly_key, &(*tctx).hashed_tweaklens[rem as usize]);
    polyval_update(p, (*req).iv, TWEAK_SIZE); polyval_export_blkaligned(p, &mut (*rctx).hashed_tweak);
}

unsafe fn hctr2_hash_message(req: *mut skcipher_request, sgl: *mut scatterlist, digest: *mut u8) {
    let rctx = skcipher_request_ctx(req); let p = &mut (*rctx).u.poly_ctx;
    let bulk_len = (*req).cryptlen - BLOCKCIPHER_BLOCK_SIZE; let mut m = core::mem::zeroed::<sg_mapping_iter>(); let mut n = 0usize; let mut i = 0usize;
    sg_miter_start(&mut m, sgl, sg_nents(sgl), 0x03);
    while i < bulk_len { sg_miter_next(&mut m); n = core::cmp::min(m.length, bulk_len-i); polyval_update(p, m.addr, n); i += n; }
    sg_miter_stop(&mut m); if (*req).cryptlen % BLOCKCIPHER_BLOCK_SIZE != 0 { let padding=1u8; polyval_update(p,&padding,1); } polyval_final(p,digest);
}

unsafe fn hctr2_finish(req: *mut skcipher_request) -> i32 {
    let tctx=crypto_skcipher_ctx(crypto_skcipher_reqtfm(req)); let rctx=skcipher_request_ctx(req); let mut digest=[0u8;POLYVAL_DIGEST_SIZE];
    polyval_import_blkaligned(&mut (*rctx).u.poly_ctx,&(*tctx).poly_key,&(*rctx).hashed_tweak); hctr2_hash_message(req,(*rctx).bulk_part_dst,digest.as_mut_ptr()); crypto_xor((*rctx).first_block.as_mut_ptr(),digest.as_ptr(),BLOCKCIPHER_BLOCK_SIZE); scatterwalk_map_and_copy((*rctx).first_block.as_mut_ptr(),(*req).dst,0,BLOCKCIPHER_BLOCK_SIZE,1); 0
}

unsafe extern "C" fn hctr2_xctr_done(data: *mut c_void, mut err: i32) { let req=data as *mut skcipher_request; if err==0 { err=hctr2_finish(req); } skcipher_request_complete(req,err); }

unsafe fn hctr2_crypt(req:*mut skcipher_request, enc:bool)->i32 {
    let tctx=crypto_skcipher_ctx(crypto_skcipher_reqtfm(req)); let rctx=skcipher_request_ctx(req); let mut digest=[0u8;POLYVAL_DIGEST_SIZE]; let bulk_len=(*req).cryptlen-BLOCKCIPHER_BLOCK_SIZE;
    if (*req).cryptlen < BLOCKCIPHER_BLOCK_SIZE { return -22; }
    scatterwalk_map_and_copy((*rctx).first_block.as_mut_ptr(),(*req).src,0,BLOCKCIPHER_BLOCK_SIZE,0);
    (*rctx).bulk_part_src=scatterwalk_ffwd((*rctx).sg_src.as_mut_ptr(),(*req).src,BLOCKCIPHER_BLOCK_SIZE); (*rctx).bulk_part_dst=scatterwalk_ffwd((*rctx).sg_dst.as_mut_ptr(),(*req).dst,BLOCKCIPHER_BLOCK_SIZE);
    hctr2_hash_tweak(req); hctr2_hash_message(req,(*rctx).bulk_part_src,digest.as_mut_ptr()); crypto_xor(digest.as_mut_ptr(),(*rctx).first_block.as_ptr(),BLOCKCIPHER_BLOCK_SIZE);
    if enc { crypto_cipher_encrypt_one((*tctx).blockcipher,(*rctx).first_block.as_mut_ptr(),digest.as_ptr()); } else { crypto_cipher_decrypt_one((*tctx).blockcipher,(*rctx).first_block.as_mut_ptr(),digest.as_ptr()); }
    crypto_xor(digest.as_mut_ptr(),(*rctx).first_block.as_ptr(),BLOCKCIPHER_BLOCK_SIZE); crypto_xor_cpy((*rctx).xctr_iv.as_mut_ptr(),digest.as_ptr(),(*tctx).l.as_ptr(),BLOCKCIPHER_BLOCK_SIZE);
    skcipher_request_set_tfm(&mut (*rctx).u.xctr_req,(*tctx).xctr); skcipher_request_set_crypt(&mut (*rctx).u.xctr_req,(*rctx).bulk_part_src,(*rctx).bulk_part_dst,bulk_len,(*rctx).xctr_iv.as_mut_ptr()); skcipher_request_set_callback(&mut (*rctx).u.xctr_req,(*req).base.flags,Some(hctr2_xctr_done),req as *mut c_void);
    let err=crypto_skcipher_encrypt(&mut (*rctx).u.xctr_req); if err != 0 { err } else { hctr2_finish(req) }
}
unsafe extern "C" fn hctr2_encrypt(req:*mut skcipher_request)->i32 { hctr2_crypt(req,true) }
unsafe extern "C" fn hctr2_decrypt(req:*mut skcipher_request)->i32 { hctr2_crypt(req,false) }

unsafe extern "C" {
    fn hctr2_init_tfm(*mut crypto_skcipher)->i32;
    fn hctr2_exit_tfm(*mut crypto_skcipher);
    fn hctr2_create(*mut crypto_template,*mut *mut rtattr)->i32;
    fn hctr2_create_base(*mut crypto_template,*mut *mut rtattr)->i32;
}

/* The kernel's instance allocation and attribute helpers are external. */
#[no_mangle] pub static mut hctr2_tmpls: [crypto_template; 2] = [crypto_template { _private: [] }, crypto_template { _private: [] }];

unsafe extern "C" fn hctr2_module_init() -> i32 {
    crypto_register_templates(hctr2_tmpls.as_mut_ptr(), 2)
}

unsafe extern "C" fn hctr2_module_exit() {
    let _ = crypto_unregister_templates(hctr2_tmpls.as_mut_ptr(), 2);
}

// module_init(hctr2_module_init);
// module_exit(hctr2_module_exit);
// MODULE_DESCRIPTION("HCTR2 length-preserving encryption mode");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS_CRYPTO("hctr2");
// MODULE_IMPORT_NS("CRYPTO_INTERNAL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
