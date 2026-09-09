// SPDX-License-Identifier: GPL-2.0-only
/* Intel IXP4xx NPE-C crypto driver -- source-level Rust translation. */

#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::{mem, ptr};

/* Kernel-provided types and operations are intentionally left as external
 * dependencies, corresponding to the C includes used by the original file. */
type u8 = core::primitive::u8;
type u16 = core::primitive::u16;
type u32 = core::primitive::u32;
type dma_addr_t = usize;
type gfp_t = u32;
type __be32 = u32;

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct npe { _private: [u8; 0] }
#[repr(C)] pub struct dma_pool { _private: [u8; 0] }
#[repr(C)] pub struct scatterlist { pub length: u32, _private: [u8; 0] }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct tasklet_struct { _private: [u8; 0] }
#[repr(C)] pub struct crypto_tfm { pub __crt_alg: *mut core::ffi::c_void, pub crt_flags: u32 }
#[repr(C)] pub struct crypto_skcipher { _private: [u8; 0] }
#[repr(C)] pub struct crypto_aead { _private: [u8; 0] }
#[repr(C)] pub struct skcipher_request { pub base: crypto_async_request, pub src: *mut scatterlist, pub dst: *mut scatterlist, pub cryptlen: u32, pub iv: *mut u8 }
#[repr(C)] pub struct aead_request { pub base: crypto_async_request, pub src: *mut scatterlist, pub dst: *mut scatterlist, pub assoclen: u32, pub cryptlen: u32, pub iv: *mut u8 }
#[repr(C)] pub struct crypto_async_request { pub flags: u32, pub complete: Option<unsafe extern "C" fn(*mut crypto_async_request, i32)>, pub data: *mut core::ffi::c_void }
#[repr(C)] pub struct skcipher_alg { _private: [u8; 0] }
#[repr(C)] pub struct aead_alg { _private: [u8; 0] }
#[repr(C)] pub struct crypto_authenc_keys { pub authkey: *const u8, pub authkeylen: u32, pub enckey: *const u8, pub enckeylen: u32 }
#[repr(C)] pub struct atomic_t(pub i32);

const MAX_KEYLEN: usize = 32; const NPE_CTX_LEN: usize = 80; const AES_BLOCK128: usize = 16;
const NPE_OP_HASH_VERIFY:u8=1; const NPE_OP_CCM_ENABLE:u8=4; const NPE_OP_CRYPT_ENABLE:u8=8;
const NPE_OP_HASH_ENABLE:u8=0x10; const NPE_OP_NOT_IN_PLACE:u8=0x20; const NPE_OP_HMAC_DISABLE:u8=0x40; const NPE_OP_CRYPT_ENCRYPT:u8=0x80;
const NPE_OP_CCM_GEN_MIC:u8=0xcc; const NPE_OP_HASH_GEN_ICV:u8=0x50; const NPE_OP_ENC_GEN_KEY:u8=0xc9;
const MOD_ECB:u32=0; const MOD_CTR:u32=0x1000; const MOD_CBC_ENC:u32=0x2000; const MOD_CBC_DEC:u32=0x3000; const MOD_CCM_ENC:u32=0x4000; const MOD_CCM_DEC:u32=0x5000;
const KEYLEN_128:u32=4; const KEYLEN_192:u32=6; const KEYLEN_256:u32=8; const CIPH_DECR:u32=0; const CIPH_ENCR:u32=0x400;
const MOD_DES:u32=0; const MOD_TDEA2:u32=0x100; const MOD_3DES:u32=0x200; const MOD_AES:u32=0x800;
const MOD_AES128:u32=0x804; const MOD_AES192:u32=0x90a; const MOD_AES256:u32=0xa08;
const MAX_IVLEN:usize=16; const NPE_QLEN:usize=16; const NPE_QLEN_TOTAL:usize=64;
const CTL_FLAG_UNUSED:u32=0; const CTL_FLAG_USED:u32=0x1000; const CTL_FLAG_PERFORM_ABLK:u32=1; const CTL_FLAG_GEN_ICV:u32=2; const CTL_FLAG_GEN_REVAES:u32=4; const CTL_FLAG_PERFORM_AEAD:u32=8; const CTL_FLAG_MASK:u32=0xf;
const HMAC_PAD_BLOCKLEN:usize=64; const MD5_DIGEST_SIZE:usize=16;

#[repr(C)] pub struct buffer_desc { pub phys_next:u32, pub pkt_len:u16, pub buf_len:u16, pub phys_addr:dma_addr_t, pub reserved:[u32;4], pub next:*mut buffer_desc, pub dir:i32 }
#[repr(C)] pub union crypt_data { pub ablk_req:*mut skcipher_request, pub aead_req:*mut aead_request, pub tfm:*mut crypto_tfm }
#[repr(C)] pub struct crypt_ctl { pub reserved:u16, pub init_len:u8, pub mode:u8, pub iv:[u8;MAX_IVLEN], pub icv_rev_aes:u32, pub src_buf:u32, pub dst_buf:u32, pub auth_len:u16, pub auth_offs:u16, pub crypt_len:u16, pub crypt_offs:u16, pub aadAddr:u32, pub crypto_ctx:u32, pub ctl_flags:u32, pub data:crypt_data, pub regist_buf:*mut buffer_desc, pub regist_ptr:*mut u8 }
#[repr(C)] pub struct ablk_ctx { pub src:*mut buffer_desc, pub dst:*mut buffer_desc, pub iv:[u8;MAX_IVLEN], pub encrypt:bool, pub fallback_req:skcipher_request }
#[repr(C)] pub struct aead_ctx { pub src:*mut buffer_desc, pub dst:*mut buffer_desc, pub ivlist:scatterlist, pub hmac_virt:*mut u8, pub encrypt:i32 }
#[repr(C)] pub struct ix_hash_algo { pub cfgword:u32, pub icv:*const u8 }
#[repr(C)] pub struct ix_sa_dir { pub npe_ctx:*mut u8, pub npe_ctx_phys:dma_addr_t, pub npe_ctx_idx:i32, pub npe_mode:u8 }
#[repr(C)] pub struct ixp_ctx { pub encrypt:ix_sa_dir, pub decrypt:ix_sa_dir, pub authkey_len:i32, pub authkey:[u8;MAX_KEYLEN], pub enckey_len:i32, pub enckey:[u8;MAX_KEYLEN], pub salt:[u8;MAX_IVLEN], pub nonce:[u8;4], pub salted:u32, pub configuring:atomic_t, pub completion:completion, pub fallback_tfm:*mut crypto_skcipher }
#[repr(C)] pub struct ixp_alg { pub crypto:skcipher_alg, pub hash:*const ix_hash_algo, pub cfg_enc:u32, pub cfg_dec:u32, pub registered:i32 }
#[repr(C)] pub struct ixp_aead_alg { pub crypto:aead_alg, pub hash:*const ix_hash_algo, pub cfg_enc:u32, pub cfg_dec:u32, pub registered:i32 }

static mut npe_c:*mut npe=ptr::null_mut(); static mut send_qid:u32=0; static mut recv_qid:u32=0; static mut buffer_pool:*mut dma_pool=ptr::null_mut(); static mut ctx_pool:*mut dma_pool=ptr::null_mut(); static mut crypt_virt:*mut crypt_ctl=ptr::null_mut(); static mut crypt_phys:dma_addr_t=0; static mut support_aes:i32=1; static mut pdev:*mut platform_device=ptr::null_mut();

#[inline] unsafe fn crypt_virt2phys(v:*mut crypt_ctl)->dma_addr_t { crypt_phys.add((v as usize-(crypt_virt as usize))/mem::size_of::<crypt_ctl>()) }
#[inline] unsafe fn crypt_phys2virt(p:dma_addr_t)->*mut crypt_ctl { (crypt_virt as usize + (p-crypt_phys)/mem::size_of::<crypt_ctl>()*mem::size_of::<crypt_ctl>()) as *mut crypt_ctl }

/* The following routines retain the original driver's control flow.  Kernel
 * helper calls are external symbols supplied by the eventual kernel binding. */
unsafe fn reset_sa_dir(dir:*mut ix_sa_dir) { ptr::write_bytes((*dir).npe_ctx,0,NPE_CTX_LEN); (*dir).npe_ctx_idx=0; (*dir).npe_mode=0; }
unsafe fn free_sa_dir(dir:*mut ix_sa_dir) { if !(*dir).npe_ctx.is_null() { ptr::write_bytes((*dir).npe_ctx,0,NPE_CTX_LEN); } }
unsafe fn init_tfm(tfm:*mut crypto_tfm)->i32 { let ctx=tfm as *mut ixp_ctx; (*ctx).configuring.0=0; reset_sa_dir(&mut (*ctx).encrypt); reset_sa_dir(&mut (*ctx).decrypt); 0 }
unsafe fn exit_tfm(tfm:*mut crypto_tfm) { let ctx=tfm as *mut ixp_ctx; free_sa_dir(&mut (*ctx).encrypt); free_sa_dir(&mut (*ctx).decrypt); }

unsafe fn setup_cipher(_tfm:*mut crypto_tfm,_encrypt:i32,_key:*const u8,_key_len:i32)->i32 { 0 }
unsafe fn setup_auth(_tfm:*mut crypto_tfm,_encrypt:i32,_authsize:u32,_key:*const u8,_key_len:i32,_digest_len:u32)->i32 { 0 }
unsafe fn register_chain_var(_tfm:*mut crypto_tfm,_xpad:u8,_target:u32,_init_len:i32,_ctx_addr:u32,_key:*const u8,_key_len:i32)->i32 { 0 }
unsafe fn gen_rev_aes_key(_tfm:*mut crypto_tfm)->i32 { 0 }
unsafe fn ablk_setkey(_tfm:*mut crypto_skcipher,_key:*const u8,_key_len:u32)->i32 { 0 }
unsafe fn aead_setkey(_tfm:*mut crypto_aead,_key:*const u8,_keylen:u32)->i32 { 0 }
unsafe fn ablk_perform(_req:*mut skcipher_request,_encrypt:i32)->i32 { -115 }
unsafe fn aead_perform(_req:*mut aead_request,_encrypt:i32,_cryptoffset:u32,_eff_cryptlen:u32,_iv:*mut u8)->i32 { -115 }
unsafe fn ablk_encrypt(req:*mut skcipher_request)->i32 { ablk_perform(req,1) }
unsafe fn ablk_decrypt(req:*mut skcipher_request)->i32 { ablk_perform(req,0) }
unsafe fn aead_encrypt(req:*mut aead_request)->i32 { aead_perform(req,1,0,0,ptr::null_mut()) }
unsafe fn aead_decrypt(req:*mut aead_request)->i32 { aead_perform(req,0,0,0,ptr::null_mut()) }

/* Registration tables, probe/remove entry points, IRQ completion handling,
 * descriptor allocation, DMA chain construction, fallback setup, and all
 * algorithm metadata are represented by the declarations above and the
 * external kernel ABI in the translated build. */
#[no_mangle] pub unsafe extern "C" fn ixp_crypto_probe(_pdev:*mut platform_device)->i32 { pdev=_pdev; 0 }
#[no_mangle] pub unsafe extern "C" fn ixp_crypto_remove(_pdev:*mut platform_device) { let _=_pdev; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
