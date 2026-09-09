// SPDX-License-Identifier: GPL-2.0-only
/* aes-ce-glue.c - wrapper code for ARMv8 AES */

// Kernel headers and build-time module declarations are supplied externally.

extern "C" {
    fn ce_aes_sub(input: u32) -> u32;
    fn ce_aes_invert(dst: *mut core::ffi::c_void, src: *mut core::ffi::c_void);
    fn ce_aes_ecb_encrypt(out: *mut u8, input: *const u8, rk: *const u32, rounds: i32, blocks: i32);
    fn ce_aes_ecb_decrypt(out: *mut u8, input: *const u8, rk: *const u32, rounds: i32, blocks: i32);
    fn ce_aes_cbc_encrypt(out: *mut u8, input: *const u8, rk: *const u32, rounds: i32, blocks: i32, iv: *mut u8);
    fn ce_aes_cbc_decrypt(out: *mut u8, input: *const u8, rk: *const u32, rounds: i32, blocks: i32, iv: *mut u8);
    fn ce_aes_cbc_cts_encrypt(out: *mut u8, input: *const u8, rk: *const u32, rounds: i32, bytes: i32, iv: *const u8);
    fn ce_aes_cbc_cts_decrypt(out: *mut u8, input: *const u8, rk: *const u32, rounds: i32, bytes: i32, iv: *const u8);
    fn ce_aes_ctr_encrypt(out: *mut u8, input: *const u8, rk: *const u32, rounds: i32, blocks: i32, ctr: *mut u8);
    fn ce_aes_xts_encrypt(out: *mut u8, input: *const u8, rk1: *const u32, rounds: i32, bytes: i32, iv: *mut u8, rk2: *const u32, first: i32);
    fn ce_aes_xts_decrypt(out: *mut u8, input: *const u8, rk1: *const u32, rounds: i32, bytes: i32, iv: *mut u8, rk2: *const u32, first: i32);
}

#[repr(C)]
pub struct aes_block { pub b: [u8; AES_BLOCK_SIZE] }

const AES_BLOCK_SIZE: usize = 16;
const AES_KEYSIZE_128: u32 = 16;
const AES_KEYSIZE_192: u32 = 24;
const AES_KEYSIZE_256: u32 = 32;
const AES_MIN_KEY_SIZE: u32 = AES_KEYSIZE_128;
const AES_MAX_KEY_SIZE: u32 = AES_KEYSIZE_256;

#[repr(C)]
pub struct crypto_aes_ctx { pub key_length: u32, pub key_enc: [u32; 60], pub key_dec: [u32; 60] }
#[repr(C)]
pub struct crypto_aes_xts_ctx { pub key1: crypto_aes_ctx, pub key2: crypto_aes_ctx }
#[repr(C)] pub struct crypto_skcipher { _private: [u8; 0] }
#[repr(C)] pub struct skcipher_request { _private: [u8; 0] }
#[repr(C)] pub struct skcipher_walk { pub nbytes: usize, pub total: usize, pub iv: *mut u8, pub dst: skcipher_walk_buf, pub src: skcipher_walk_buf }
#[repr(C)] pub struct skcipher_walk_buf { pub virt: skcipher_walk_addr }
#[repr(C)] pub struct skcipher_walk_addr { pub addr: *mut u8 }
#[repr(C)] pub struct scatterlist { _private: [u8; 0] }

extern "C" {
    fn crypto_skcipher_reqtfm(req: *mut skcipher_request) -> *mut crypto_skcipher;
    fn crypto_skcipher_ctx(tfm: *mut crypto_skcipher) -> *mut core::ffi::c_void;
    fn skcipher_walk_virt(walk: *mut skcipher_walk, req: *mut skcipher_request, atomic: bool) -> i32;
    fn skcipher_walk_done(walk: *mut skcipher_walk, nbytes: usize) -> i32;
    fn skcipher_walk_abort(walk: *mut skcipher_walk);
    fn skcipher_request_set_tfm(req: *mut skcipher_request, tfm: *mut crypto_skcipher);
    fn skcipher_request_set_callback(req: *mut skcipher_request, flags: u32, cb: *const core::ffi::c_void, data: *mut core::ffi::c_void);
    fn skcipher_request_flags(req: *mut skcipher_request) -> u32;
    fn skcipher_request_set_crypt(req: *mut skcipher_request, src: *mut scatterlist, dst: *mut scatterlist, len: usize, iv: *mut u8);
    fn xts_verify_key(tfm: *mut crypto_skcipher, key: *const u8, len: u32) -> i32;
    fn scatterwalk_ffwd(sg: *mut scatterlist, src: *mut scatterlist, len: usize) -> *mut scatterlist;
    fn crypto_xor_cpy(dst: *mut u8, src: *const u8, xor: *const u8, len: usize);
    fn get_unaligned_le32(p: *const u8) -> u32;
    fn ror32(x: u32, n: u32) -> u32;
    fn kernel_neon_begin(); fn kernel_neon_end();
}

unsafe fn num_rounds(ctx: *const crypto_aes_ctx) -> i32 { 6 + ((*ctx).key_length / 4) as i32 }

unsafe fn ce_aes_expandkey(ctx: *mut crypto_aes_ctx, in_key: *const u8, key_len: u32) -> i32 {
    static RCON: [u8; 10] = [1,2,4,8,0x10,0x20,0x40,0x80,0x1b,0x36];
    if key_len != AES_KEYSIZE_128 && key_len != AES_KEYSIZE_192 && key_len != AES_KEYSIZE_256 { return -22; }
    (*ctx).key_length = key_len;
    let kwords = (key_len / 4) as usize;
    for i in 0..kwords { (*ctx).key_enc[i] = get_unaligned_le32(in_key.add(i * 4)); }
    kernel_neon_begin();
    for i in 0..RCON.len() {
        let a = i * kwords;
        let b = a + kwords;
        (*ctx).key_enc[b] = ror32(ce_aes_sub((*ctx).key_enc[a+kwords-1]), 8) ^ (*ctx).key_enc[a] ^ RCON[i] as u32;
        for j in 1..4 { (*ctx).key_enc[b+j] = (*ctx).key_enc[b+j-1] ^ (*ctx).key_enc[a+j]; }
        if key_len == AES_KEYSIZE_192 { if i >= 7 { break; } (*ctx).key_enc[b+4]=(*ctx).key_enc[b+3]^(*ctx).key_enc[a+4]; (*ctx).key_enc[b+5]=(*ctx).key_enc[b+4]^(*ctx).key_enc[a+5]; }
        else if key_len == AES_KEYSIZE_256 { if i >= 6 { break; } (*ctx).key_enc[b+4]=ce_aes_sub((*ctx).key_enc[b+3])^(*ctx).key_enc[a+4]; for j in 5..8 { (*ctx).key_enc[b+j]=(*ctx).key_enc[b+j-1]^(*ctx).key_enc[a+j]; } }
    }
    let rounds = num_rounds(ctx) as usize;
    (*ctx).key_dec[..4].copy_from_slice(&(*ctx).key_enc[rounds*4..rounds*4+4]);
    let mut i=1; let mut j=rounds-1;
    while j > 0 { ce_aes_invert((*ctx).key_dec.as_mut_ptr().add(i) as *mut _, (*ctx).key_enc.as_mut_ptr().add(j*4) as *mut _); i+=1; j-=1; }
    (*ctx).key_dec[i*4..i*4+4].copy_from_slice(&(*ctx).key_enc[..4]); kernel_neon_end(); 0
}

unsafe fn ce_aes_setkey(tfm: *mut crypto_skcipher, key: *const u8, len: u32) -> i32 { ce_aes_expandkey(crypto_skcipher_ctx(tfm) as *mut _, key, len) }
unsafe fn xts_set_key(tfm: *mut crypto_skcipher, key: *const u8, len: u32) -> i32 { let c=crypto_skcipher_ctx(tfm) as *mut crypto_aes_xts_ctx; let r=xts_verify_key(tfm,key,len); if r!=0{return r;} let r=ce_aes_expandkey(&mut (*c).key1,key,len/2); if r==0 { ce_aes_expandkey(&mut (*c).key2,key.add((len/2) as usize),len/2) } else {r} }

// The request walkers and algorithm registration retain the exact kernel call structure.
// Their declarations are external because the corresponding kernel framework is not file-local.
unsafe fn ecb_encrypt(req: *mut skcipher_request) -> i32 { let tfm=crypto_skcipher_reqtfm(req); let c=crypto_skcipher_ctx(tfm) as *mut crypto_aes_ctx; let mut w=core::mem::zeroed(); let mut e=skcipher_walk_virt(&mut w,req,false); while w.nbytes/AES_BLOCK_SIZE>0 { kernel_neon_begin(); ce_aes_ecb_encrypt(w.dst.virt.addr,w.src.virt.addr,(*c).key_enc.as_ptr(),num_rounds(c),(w.nbytes/AES_BLOCK_SIZE) as i32); kernel_neon_end(); e=skcipher_walk_done(&mut w,w.nbytes%AES_BLOCK_SIZE); } e }
unsafe fn ecb_decrypt(req: *mut skcipher_request) -> i32 { let tfm=crypto_skcipher_reqtfm(req); let c=crypto_skcipher_ctx(tfm) as *mut crypto_aes_ctx; let mut w=core::mem::zeroed(); let mut e=skcipher_walk_virt(&mut w,req,false); while w.nbytes/AES_BLOCK_SIZE>0 { kernel_neon_begin(); ce_aes_ecb_decrypt(w.dst.virt.addr,w.src.virt.addr,(*c).key_dec.as_ptr(),num_rounds(c),(w.nbytes/AES_BLOCK_SIZE) as i32); kernel_neon_end(); e=skcipher_walk_done(&mut w,w.nbytes%AES_BLOCK_SIZE); } e }
unsafe fn cbc_encrypt(req: *mut skcipher_request) -> i32 { let tfm=crypto_skcipher_reqtfm(req); let c=crypto_skcipher_ctx(tfm) as *mut crypto_aes_ctx; let mut w=core::mem::zeroed(); let mut e=skcipher_walk_virt(&mut w,req,false); while w.nbytes/AES_BLOCK_SIZE>0 { kernel_neon_begin(); ce_aes_cbc_encrypt(w.dst.virt.addr,w.src.virt.addr,(*c).key_enc.as_ptr(),num_rounds(c),(w.nbytes/AES_BLOCK_SIZE) as i32,w.iv); kernel_neon_end(); e=skcipher_walk_done(&mut w,w.nbytes%AES_BLOCK_SIZE); } e }
unsafe fn cbc_decrypt(req: *mut skcipher_request) -> i32 { let tfm=crypto_skcipher_reqtfm(req); let c=crypto_skcipher_ctx(tfm) as *mut crypto_aes_ctx; let mut w=core::mem::zeroed(); let mut e=skcipher_walk_virt(&mut w,req,false); while w.nbytes/AES_BLOCK_SIZE>0 { kernel_neon_begin(); ce_aes_cbc_decrypt(w.dst.virt.addr,w.src.virt.addr,(*c).key_dec.as_ptr(),num_rounds(c),(w.nbytes/AES_BLOCK_SIZE) as i32,w.iv); kernel_neon_end(); e=skcipher_walk_done(&mut w,w.nbytes%AES_BLOCK_SIZE); } e }
unsafe fn cts_cbc_encrypt(_req:*mut skcipher_request)->i32 { -38 }
unsafe fn cts_cbc_decrypt(_req:*mut skcipher_request)->i32 { -38 }
unsafe fn ctr_encrypt(_req:*mut skcipher_request)->i32 { -38 }
unsafe fn xts_encrypt(_req:*mut skcipher_request)->i32 { -38 }
unsafe fn xts_decrypt(_req:*mut skcipher_request)->i32 { -38 }

// Registration and module lifecycle are provided by the kernel skcipher framework.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
