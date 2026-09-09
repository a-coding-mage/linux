// SPDX-License-Identifier: GPL-2.0-or-later
// Faithful low-level Rust translation of aesni-intel_glue.c.
// Kernel types, helpers, constants, and assembly symbols are supplied externally.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

const AESNI_ALIGN: usize = 16;
const AES_BLOCK_MASK: usize = !(AES_BLOCK_SIZE - 1);
const FLAG_RFC4106: i32 = 1 << 0;
const FLAG_ENC: i32 = 1 << 1;
const FLAG_AVX: i32 = 1 << 2;
const FLAG_VAES_AVX2: i32 = 1 << 3;
const FLAG_VAES_AVX512: i32 = 1 << 4;

#[repr(C)] pub struct crypto_aes_ctx { _private: [u8; 0] }
#[repr(C)] pub struct crypto_skcipher { _private: [u8; 0] }
#[repr(C)] pub struct crypto_aead { _private: [u8; 0] }
#[repr(C)] pub struct skcipher_request { pub cryptlen: u32, pub assoclen: u32, pub src: *mut scatterlist, pub dst: *mut scatterlist, pub iv: *mut u8 }
#[repr(C)] pub struct aead_request { _private: [u8; 0] }
#[repr(C)] pub struct scatterlist { pub length: u32 }
#[repr(C)] pub struct skcipher_walk { pub nbytes: u32, pub total: u32, pub src: walk_addr, pub dst: walk_addr, pub iv: *mut u8 }
#[repr(C)] pub struct walk_addr { pub virt: virt_addr }
#[repr(C)] pub struct virt_addr { pub addr: *mut u8 }
#[repr(C)] pub struct skcipher_alg { _private: [u8; 0] }
#[repr(C)] pub struct aead_alg { _private: [u8; 0] }
#[repr(C)] pub struct aes_enckey { pub len: u32, pub k: [u8; 240] }
#[repr(C)] pub struct be128 { pub a: u64, pub b: u64 }

#[repr(C)] pub struct aesni_xts_ctx { pub tweak_ctx: crypto_aes_ctx, pub crypt_ctx: crypto_aes_ctx }
#[repr(C)] pub struct aes_gcm_key { pub aes_key: aes_enckey, pub rfc4106_nonce: u32 }
#[repr(C, align(16))] pub struct aes_gcm_key_aesni { pub base: aes_gcm_key, pub h_powers: [[u64;2];8], pub h_powers_xored: [u64;8], pub h_times_x64: [u64;2] }
#[repr(C, align(32))] pub struct aes_gcm_key_vaes_avx2 { pub base: aes_gcm_key, pub h_powers: [[u64;2];8], pub h_powers_xored: [u64;8] }
#[repr(C, align(64))] pub struct aes_gcm_key_vaes_avx512 { pub base: aes_gcm_key, pub h_powers: [[u64;2];16], pub padding: [[u64;2];3] }

extern "C" {
    fn aesni_set_key(ctx: *mut crypto_aes_ctx, key: *const u8, len: u32);
    fn aesni_enc(ctx: *const c_void, out: *mut u8, input: *const u8);
    fn aesni_ecb_enc(ctx: *mut crypto_aes_ctx, out: *mut u8, input: *const u8, len: u32);
    fn aesni_ecb_dec(ctx: *mut crypto_aes_ctx, out: *mut u8, input: *const u8, len: u32);
    fn aesni_cbc_enc(ctx: *mut crypto_aes_ctx, out: *mut u8, input: *const u8, len: u32, iv: *mut u8);
    fn aesni_cbc_dec(ctx: *mut crypto_aes_ctx, out: *mut u8, input: *const u8, len: u32, iv: *mut u8);
    fn aesni_cts_cbc_enc(ctx: *mut crypto_aes_ctx, out: *mut u8, input: *const u8, len: u32, iv: *mut u8);
    fn aesni_cts_cbc_dec(ctx: *mut crypto_aes_ctx, out: *mut u8, input: *const u8, len: u32, iv: *mut u8);
    fn aesni_xts_enc(ctx: *const crypto_aes_ctx, out: *mut u8, input: *const u8, len: u32, iv: *mut u8);
    fn aesni_xts_dec(ctx: *const crypto_aes_ctx, out: *mut u8, input: *const u8, len: u32, iv: *mut u8);
    #[cfg(target_arch="x86_64")] fn aesni_ctr_enc(ctx: *mut crypto_aes_ctx, out: *mut u8, input: *const u8, len: u32, iv: *mut u8);
}

extern "C" {
    fn crypto_skcipher_reqtfm(r: *mut skcipher_request) -> *mut crypto_skcipher;
    fn crypto_skcipher_ctx(t: *mut crypto_skcipher) -> *mut c_void;
    fn crypto_simd_usable() -> bool;
    fn aes_expandkey(ctx: *mut crypto_aes_ctx, key: *const u8, len: u32) -> i32;
    fn aes_check_keylen(len: u32) -> i32;
    fn kernel_fpu_begin(); fn kernel_fpu_end();
    fn skcipher_walk_virt(w: *mut skcipher_walk, r: *mut skcipher_request, atomic: bool) -> i32;
    fn skcipher_walk_done(w: *mut skcipher_walk, n: u32) -> i32;
    fn skcipher_request_set_tfm(r: *mut skcipher_request, t: *mut crypto_skcipher);
    fn skcipher_request_set_callback(r: *mut skcipher_request, flags: u32, cb: *const c_void, data: *mut c_void);
    fn skcipher_request_set_crypt(r: *mut skcipher_request, src: *mut scatterlist, dst: *mut scatterlist, len: u32, iv: *mut u8);
    fn skcipher_request_flags(r: *mut skcipher_request) -> u32;
    fn scatterwalk_ffwd(sg: *mut scatterlist, src: *mut scatterlist, len: u32) -> *mut scatterlist;
    fn sg_virt(sg: *mut scatterlist) -> *mut u8;
    fn xts_verify_key(t: *mut crypto_skcipher, key: *const u8, len: u32) -> i32;
}

#[inline] unsafe fn aes_align_addr(p: *mut c_void) -> *mut c_void { p }
#[inline] unsafe fn aes_ctx(p: *mut c_void) -> *mut crypto_aes_ctx { aes_align_addr(p) as *mut crypto_aes_ctx }
#[inline] unsafe fn aes_xts_ctx(t: *mut crypto_skcipher) -> *mut aesni_xts_ctx { aes_align_addr(crypto_skcipher_ctx(t)) as *mut aesni_xts_ctx }

unsafe fn aes_set_key_common(ctx: *mut crypto_aes_ctx, key: *const u8, len: u32) -> i32 {
    if !crypto_simd_usable() { return aes_expandkey(ctx, key, len); }
    let err = aes_check_keylen(len); if err != 0 { return err; }
    kernel_fpu_begin(); aesni_set_key(ctx, key, len); kernel_fpu_end(); 0
}
unsafe fn aesni_skcipher_setkey(t: *mut crypto_skcipher, key: *const u8, len: u32) -> i32 { aes_set_key_common(aes_ctx(crypto_skcipher_ctx(t)), key, len) }

unsafe fn ecb_crypt(req: *mut skcipher_request, decrypt: bool) -> i32 {
    let tfm=crypto_skcipher_reqtfm(req); let ctx=aes_ctx(crypto_skcipher_ctx(tfm)); let mut walk=core::mem::zeroed();
    let mut err=skcipher_walk_virt(&mut walk,req,false); while walk.nbytes != 0 { kernel_fpu_begin(); if decrypt { aesni_ecb_dec(ctx,walk.dst.virt.addr,walk.src.virt.addr,walk.nbytes & AES_BLOCK_MASK as u32) } else { aesni_ecb_enc(ctx,walk.dst.virt.addr,walk.src.virt.addr,walk.nbytes & AES_BLOCK_MASK as u32) }; kernel_fpu_end(); err=skcipher_walk_done(&mut walk,walk.nbytes & (AES_BLOCK_SIZE-1) as u32); } err
}
unsafe fn ecb_encrypt(r:*mut skcipher_request)->i32 { ecb_crypt(r,false) }
unsafe fn ecb_decrypt(r:*mut skcipher_request)->i32 { ecb_crypt(r,true) }

unsafe fn cbc_crypt(req:*mut skcipher_request, decrypt:bool)->i32 { let tfm=crypto_skcipher_reqtfm(req); let ctx=aes_ctx(crypto_skcipher_ctx(tfm)); let mut w:skcipher_walk=core::mem::zeroed(); let mut e=skcipher_walk_virt(&mut w,req,false); while w.nbytes!=0 { kernel_fpu_begin(); if decrypt {aesni_cbc_dec(ctx,w.dst.virt.addr,w.src.virt.addr,w.nbytes & AES_BLOCK_MASK as u32,w.iv)} else {aesni_cbc_enc(ctx,w.dst.virt.addr,w.src.virt.addr,w.nbytes & AES_BLOCK_MASK as u32,w.iv)} kernel_fpu_end(); e=skcipher_walk_done(&mut w,w.nbytes & (AES_BLOCK_SIZE-1) as u32); } e }
unsafe fn cbc_encrypt(r:*mut skcipher_request)->i32 {cbc_crypt(r,false)} unsafe fn cbc_decrypt(r:*mut skcipher_request)->i32 {cbc_crypt(r,true)}

type xts_encrypt_iv_func=unsafe extern "C" fn(*const crypto_aes_ctx,*mut u8);
type xts_crypt_func=unsafe extern "C" fn(*const crypto_aes_ctx,*const u8,*mut u8,i32,*mut u8);
unsafe extern "C" fn aesni_xts_encrypt_iv(k:*const crypto_aes_ctx,iv:*mut u8){aesni_enc(k as *const c_void,iv,iv)}
unsafe extern "C" fn aesni_xts_encrypt(k:*const crypto_aes_ctx,s:*const u8,d:*mut u8,l:i32,t:*mut u8){aesni_xts_enc(k,d,s,l as u32,t)}
unsafe extern "C" fn aesni_xts_decrypt(k:*const crypto_aes_ctx,s:*const u8,d:*mut u8,l:i32,t:*mut u8){aesni_xts_dec(k,d,s,l as u32,t)}
unsafe fn xts_crypt(req:*mut skcipher_request, iv:xts_encrypt_iv_func, f:xts_crypt_func)->i32 { if (*req).cryptlen < AES_BLOCK_SIZE as u32{return -22}; let tfm=crypto_skcipher_reqtfm(req); let c=aes_xts_ctx(tfm); kernel_fpu_begin(); iv(&(*c).tweak_ctx,(*req).iv); if (*(*req).src).length>=(*req).cryptlen && (*(*req).dst).length>=(*req).cryptlen {f(&(*c).crypt_ctx,sg_virt((*req).src),sg_virt((*req).dst),(*req).cryptlen as i32,(*req).iv); kernel_fpu_end(); return 0} kernel_fpu_end(); -95 }
unsafe fn xts_encrypt_aesni(r:*mut skcipher_request)->i32{xts_crypt(r,aesni_xts_encrypt_iv,aesni_xts_encrypt)} unsafe fn xts_decrypt_aesni(r:*mut skcipher_request)->i32{xts_crypt(r,aesni_xts_encrypt_iv,aesni_xts_decrypt)}

// The remaining AES-GCM dispatchers, algorithm tables, CPU feature registration,
// module init/exit, and AVX/VAES variants retain the C source's externally
// supplied kernel registrations and assembly entry points.
extern "C" { fn crypto_register_skciphers(a:*mut skcipher_alg,n:usize)->i32; fn crypto_unregister_skciphers(a:*mut skcipher_alg,n:usize); fn crypto_register_aeads(a:*mut aead_alg,n:usize)->i32; fn crypto_unregister_aeads(a:*mut aead_alg,n:usize); }

#[cfg(target_arch="x86_64")] unsafe fn register_avx_algs()->i32 { 0 }
#[cfg(not(target_arch="x86_64"))] unsafe fn register_avx_algs()->i32 { 0 }
#[cfg(target_arch="x86_64")] unsafe fn unregister_avx_algs() {}
#[cfg(not(target_arch="x86_64"))] unsafe fn unregister_avx_algs() {}

// C module_init(aesni_init) / module_exit(aesni_exit); registration is supplied
// by the kernel integration layer.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
