// SPDX-License-Identifier: GPL-2.0-or-later
/* Glue code for AES implementation for SPE instructions (PPC). */

// C dependencies supplied by the surrounding kernel translation.
use core::ptr;

const MAX_BYTES: usize = 768;
const AES_BLOCK_SIZE: usize = 16;
const AES_MAX_KEYLENGTH_U32: usize = 60;
const AES_KEYSIZE_128: usize = 16;
const AES_KEYSIZE_192: usize = 24;
const AES_KEYSIZE_256: usize = 32;
const AES_MIN_KEY_SIZE: usize = AES_KEYSIZE_128;
const AES_MAX_KEY_SIZE: usize = AES_KEYSIZE_256;

#[repr(C)]
pub struct ppc_aes_ctx {
    pub key_enc: [u32; AES_MAX_KEYLENGTH_U32],
    pub key_dec: [u32; AES_MAX_KEYLENGTH_U32],
    pub rounds: u32,
}

#[repr(C)]
pub struct ppc_xts_ctx {
    pub key_enc: [u32; AES_MAX_KEYLENGTH_U32],
    pub key_dec: [u32; AES_MAX_KEYLENGTH_U32],
    pub key_twk: [u32; AES_MAX_KEYLENGTH_U32],
    pub rounds: u32,
}

#[repr(C)]
pub struct crypto_skcipher { _private: [u8; 0] }
#[repr(C)]
pub struct skcipher_request { pub cryptlen: usize, pub src: *mut u8, pub dst: *mut u8, pub iv: *mut u8 }
#[repr(C)]
pub struct skcipher_walk { pub nbytes: usize, pub total: usize, pub iv: *mut u8, pub dst: skcipher_walk_buf, pub src: skcipher_walk_buf }
#[repr(C)]
pub struct skcipher_walk_buf { pub virt: skcipher_walk_addr }
#[repr(C)]
pub struct skcipher_walk_addr { pub addr: *mut u8 }
#[repr(C)]
pub struct le128 { pub b: [u8; 16] }

extern "C" {
    fn preempt_disable(); fn enable_kernel_spe(); fn disable_kernel_spe(); fn preempt_enable();
    fn crypto_skcipher_reqtfm(req: *mut skcipher_request) -> *mut crypto_skcipher;
    fn crypto_skcipher_ctx(tfm: *mut crypto_skcipher) -> *mut u8;
    fn ppc_expand_key_128(dst: *mut u32, key: *const u8);
    fn ppc_expand_key_192(dst: *mut u32, key: *const u8);
    fn ppc_expand_key_256(dst: *mut u32, key: *const u8);
    fn ppc_generate_decrypt_key(dst: *mut u32, src: *const u32, len: usize);
    fn skcipher_walk_virt(walk: *mut skcipher_walk, req: *mut skcipher_request, atomic: bool) -> i32;
    fn skcipher_walk_done(walk: *mut skcipher_walk, nbytes: usize) -> i32;
    fn ppc_encrypt_ecb(dst: *mut u8, src: *mut u8, key: *const u32, rounds: u32, nbytes: usize);
    fn ppc_decrypt_ecb(dst: *mut u8, src: *mut u8, key: *const u32, rounds: u32, nbytes: usize);
    fn ppc_encrypt_cbc(dst: *mut u8, src: *mut u8, key: *const u32, rounds: u32, nbytes: usize, iv: *mut u8);
    fn ppc_decrypt_cbc(dst: *mut u8, src: *mut u8, key: *const u32, rounds: u32, nbytes: usize, iv: *mut u8);
    fn ppc_crypt_ctr(dst: *mut u8, src: *mut u8, key: *const u32, rounds: u32, nbytes: usize, iv: *mut u8);
    fn ppc_encrypt_xts(dst: *mut u8, src: *mut u8, key: *const u32, rounds: u32, nbytes: usize, iv: *mut u8, twk: *mut u32);
    fn ppc_decrypt_xts(dst: *mut u8, src: *mut u8, key: *const u32, rounds: u32, nbytes: usize, iv: *mut u8, twk: *mut u32);
    fn xts_verify_key(tfm: *mut crypto_skcipher, key: *const u8, len: usize) -> i32;
    fn scatterwalk_map_and_copy(to: *mut u8, sg: *mut u8, offset: isize, len: usize, out: i32);
    fn skcipher_request_set_crypt(req: *mut skcipher_request, src: *mut u8, dst: *mut u8, len: usize, iv: *mut u8);
    fn gf128mul_x_ble(out: *mut le128, in_: *const le128);
    fn crypto_register_skciphers(algs: *mut skcipher_alg, count: usize) -> i32;
    fn crypto_unregister_skciphers(algs: *mut skcipher_alg, count: usize);
}

unsafe fn spe_begin() { preempt_disable(); enable_kernel_spe(); }
unsafe fn spe_end() { disable_kernel_spe(); preempt_enable(); }

unsafe fn ppc_aes_setkey_skcipher(tfm: *mut crypto_skcipher, in_key: *const u8, key_len: usize) -> i32 {
    let ctx = crypto_skcipher_ctx(tfm) as *mut ppc_aes_ctx;
    match key_len {
        AES_KEYSIZE_128 => { (*ctx).rounds = 4; ppc_expand_key_128((*ctx).key_enc.as_mut_ptr(), in_key); }
        AES_KEYSIZE_192 => { (*ctx).rounds = 5; ppc_expand_key_192((*ctx).key_enc.as_mut_ptr(), in_key); }
        AES_KEYSIZE_256 => { (*ctx).rounds = 6; ppc_expand_key_256((*ctx).key_enc.as_mut_ptr(), in_key); }
        _ => return -22,
    }
    ppc_generate_decrypt_key((*ctx).key_dec.as_mut_ptr(), (*ctx).key_enc.as_ptr(), key_len); 0
}

unsafe fn ppc_xts_setkey(tfm: *mut crypto_skcipher, in_key: *const u8, mut key_len: usize) -> i32 {
    let ctx = crypto_skcipher_ctx(tfm) as *mut ppc_xts_ctx;
    let err = xts_verify_key(tfm, in_key, key_len); if err != 0 { return err; }
    key_len >>= 1;
    let twk = in_key.add(key_len);
    match key_len {
        AES_KEYSIZE_128 => { (*ctx).rounds=4; ppc_expand_key_128((*ctx).key_enc.as_mut_ptr(),in_key); ppc_expand_key_128((*ctx).key_twk.as_mut_ptr(),twk); }
        AES_KEYSIZE_192 => { (*ctx).rounds=5; ppc_expand_key_192((*ctx).key_enc.as_mut_ptr(),in_key); ppc_expand_key_192((*ctx).key_twk.as_mut_ptr(),twk); }
        AES_KEYSIZE_256 => { (*ctx).rounds=6; ppc_expand_key_256((*ctx).key_enc.as_mut_ptr(),in_key); ppc_expand_key_256((*ctx).key_twk.as_mut_ptr(),twk); }
        _ => return -22,
    }
    ppc_generate_decrypt_key((*ctx).key_dec.as_mut_ptr(), (*ctx).key_enc.as_ptr(), key_len); 0
}

unsafe fn ppc_ecb_crypt(req: *mut skcipher_request, enc: bool) -> i32 {
    let tfm=crypto_skcipher_reqtfm(req); let ctx=crypto_skcipher_ctx(tfm) as *mut ppc_aes_ctx; let mut walk=core::mem::zeroed();
    let mut err=skcipher_walk_virt(&mut walk,false as *mut skcipher_request, false); while walk.nbytes != 0 { let n=core::cmp::min(walk.nbytes,MAX_BYTES)&!(AES_BLOCK_SIZE-1); spe_begin(); if enc { ppc_encrypt_ecb(walk.dst.virt.addr,walk.src.virt.addr,(*ctx).key_enc.as_ptr(),(*ctx).rounds,n) } else { ppc_decrypt_ecb(walk.dst.virt.addr,walk.src.virt.addr,(*ctx).key_dec.as_ptr(),(*ctx).rounds,n) } spe_end(); err=skcipher_walk_done(&mut walk,walk.nbytes-n); } err
}
unsafe fn ppc_ecb_encrypt(r:*mut skcipher_request)->i32 { ppc_ecb_crypt(r,true) }
unsafe fn ppc_ecb_decrypt(r:*mut skcipher_request)->i32 { ppc_ecb_crypt(r,false) }

unsafe fn ppc_cbc_crypt(req:*mut skcipher_request, enc:bool)->i32 { let tfm=crypto_skcipher_reqtfm(req); let ctx=crypto_skcipher_ctx(tfm) as *mut ppc_aes_ctx; let mut walk:skcipher_walk=core::mem::zeroed(); let mut err=skcipher_walk_virt(&mut walk,req,false); while walk.nbytes!=0 { let n=core::cmp::min(walk.nbytes,MAX_BYTES)&!(AES_BLOCK_SIZE-1); spe_begin(); if enc { ppc_encrypt_cbc(walk.dst.virt.addr,walk.src.virt.addr,(*ctx).key_enc.as_ptr(),(*ctx).rounds,n,walk.iv) } else { ppc_decrypt_cbc(walk.dst.virt.addr,walk.src.virt.addr,(*ctx).key_dec.as_ptr(),(*ctx).rounds,n,walk.iv) } spe_end(); err=skcipher_walk_done(&mut walk,walk.nbytes-n); } err }
unsafe fn ppc_cbc_encrypt(r:*mut skcipher_request)->i32 { ppc_cbc_crypt(r,true) }
unsafe fn ppc_cbc_decrypt(r:*mut skcipher_request)->i32 { ppc_cbc_crypt(r,false) }
unsafe fn ppc_ctr_crypt(req:*mut skcipher_request)->i32 { let tfm=crypto_skcipher_reqtfm(req); let ctx=crypto_skcipher_ctx(tfm) as *mut ppc_aes_ctx; let mut walk:skcipher_walk=core::mem::zeroed(); let mut err=skcipher_walk_virt(&mut walk,req,false); while walk.nbytes!=0 { let mut n=core::cmp::min(walk.nbytes,MAX_BYTES); if n<walk.total { n&=!(AES_BLOCK_SIZE-1); } spe_begin(); ppc_crypt_ctr(walk.dst.virt.addr,walk.src.virt.addr,(*ctx).key_enc.as_ptr(),(*ctx).rounds,n,walk.iv); spe_end(); err=skcipher_walk_done(&mut walk,walk.nbytes-n); } err }
unsafe fn ppc_xts_crypt(req:*mut skcipher_request, enc:bool)->i32 { let tfm=crypto_skcipher_reqtfm(req); let ctx=crypto_skcipher_ctx(tfm) as *mut ppc_xts_ctx; let mut walk:skcipher_walk=core::mem::zeroed(); let mut err=skcipher_walk_virt(&mut walk,req,false); let mut twk=(*ctx).key_twk.as_mut_ptr(); while walk.nbytes!=0 { let n=core::cmp::min(walk.nbytes,MAX_BYTES)&!(AES_BLOCK_SIZE-1); spe_begin(); if enc { ppc_encrypt_xts(walk.dst.virt.addr,walk.src.virt.addr,(*ctx).key_enc.as_ptr(),(*ctx).rounds,n,walk.iv,twk) } else { ppc_decrypt_xts(walk.dst.virt.addr,walk.src.virt.addr,(*ctx).key_dec.as_ptr(),(*ctx).rounds,n,walk.iv,twk) } spe_end(); twk=ptr::null_mut(); err=skcipher_walk_done(&mut walk,walk.nbytes-n); } err }
unsafe fn ppc_xts_encrypt(r:*mut skcipher_request)->i32 { ppc_xts_crypt(r,true) }
unsafe fn ppc_xts_decrypt(r:*mut skcipher_request)->i32 { ppc_xts_crypt(r,false) }

#[repr(C)] pub struct skcipher_alg { pub opaque: [usize; 32] }
#[no_mangle] pub static mut aes_skcipher_algs: [skcipher_alg; 4] = [skcipher_alg{opaque:[0;32]};4];

#[no_mangle] pub unsafe extern "C" fn ppc_aes_mod_init() -> i32 { crypto_register_skciphers(aes_skcipher_algs.as_mut_ptr(),4) }
#[no_mangle] pub unsafe extern "C" fn ppc_aes_mod_fini() { crypto_unregister_skciphers(aes_skcipher_algs.as_mut_ptr(),4); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
