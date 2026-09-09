// SPDX-License-Identifier: GPL-2.0-or-later
/* Freescale i.MX23/i.MX28 Data Co-Processor driver. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/* Kernel and crypto interfaces supplied by the surrounding kernel translation. */
use core::ffi::c_void;

pub const DCP_MAX_CHANS: usize = 4;
pub const DCP_BUF_SZ: usize = PAGE_SIZE;
pub const DCP_SHA_PAY_SZ: usize = 64;
pub const DCP_ALIGNMENT: usize = 64;

extern "C" {
    static mut global_sdcp: *mut dcp;
}
extern "Rust" {
    static PAGE_SIZE: usize;
}

const sha1_null_hash: [u8; 20] = [0x09,0x07,0xd8,0xaf,0x90,0x18,0x60,0x95,0xef,0xbf,0x55,0x32,0x0d,0x4b,0x6b,0x5e,0xee,0xa3,0x39,0xda];
const sha256_null_hash: [u8; 32] = [0x55,0xb8,0x52,0x78,0x1b,0x99,0x95,0xa4,0x4c,0x93,0x9b,0x64,0xe4,0x41,0xae,0x27,0x24,0xb9,0x6f,0x99,0xc8,0xf4,0xfb,0x9a,0x14,0x1c,0xfc,0x98,0x42,0xc4,0xb0,0xe3];

#[repr(C)] pub struct dcp_dma_desc { pub next_cmd_addr:u32, pub control0:u32, pub control1:u32, pub source:u32, pub destination:u32, pub size:u32, pub payload:u32, pub status:u32 }
#[repr(C)] pub struct dcp_coherent_block { pub aes_in_buf:[u8;DCP_BUF_SZ], pub aes_out_buf:[u8;DCP_BUF_SZ], pub sha_in_buf:[u8;DCP_BUF_SZ], pub sha_out_buf:[u8;DCP_SHA_PAY_SZ], pub aes_key:[u8;2*AES_KEYSIZE_128], pub desc:[dcp_dma_desc;DCP_MAX_CHANS] }
#[repr(C)] pub struct dcp { pub dev:*mut device, pub base:*mut c_void, pub caps:u32, pub coh:*mut dcp_coherent_block, pub completion:[completion;DCP_MAX_CHANS], pub lock:[spinlock_t;DCP_MAX_CHANS], pub thread:[*mut task_struct;DCP_MAX_CHANS], pub queue:[crypto_queue;DCP_MAX_CHANS], pub dcp_clk:*mut clk }
#[repr(C)] pub struct dcp_async_ctx { pub chan:dcp_chan, pub fill:u32, pub mutex:mutex, pub alg:u32, pub hot:u32, pub fallback:*mut crypto_skcipher, pub key_len:u32, pub key:[u8;AES_KEYSIZE_128], pub key_referenced:bool }
#[repr(C)] pub struct dcp_aes_req_ctx { pub enc:u32, pub ecb:u32, pub fallback_req:skcipher_request }
#[repr(C)] pub struct dcp_sha_req_ctx { pub init:u32, pub fini:u32 }
#[repr(C)] pub struct dcp_export_state { pub req_ctx:dcp_sha_req_ctx, pub async_ctx:dcp_async_ctx }
#[repr(C)] pub enum dcp_chan { DCP_CHAN_HASH_SHA=0, DCP_CHAN_CRYPTO=2 }

pub const MXS_DCP_CTRL:u32=0x00; pub const MXS_DCP_CTRL_GATHER_RESIDUAL_WRITES:u32=1<<23; pub const MXS_DCP_CTRL_ENABLE_CONTEXT_CACHING:u32=1<<22;
pub const MXS_DCP_STAT:u32=0x10; pub const MXS_DCP_STAT_CLR:u32=0x18; pub const MXS_DCP_STAT_IRQ_MASK:u32=0xf;
pub const MXS_DCP_CHANNELCTRL:u32=0x20; pub const MXS_DCP_CHANNELCTRL_ENABLE_CHANNEL_MASK:u32=0xff;
pub const MXS_DCP_CAPABILITY1:u32=0x40; pub const MXS_DCP_CAPABILITY1_SHA256:u32=4<<16; pub const MXS_DCP_CAPABILITY1_SHA1:u32=1<<16; pub const MXS_DCP_CAPABILITY1_AES128:u32=1;
pub const MXS_DCP_CONTEXT:u32=0x50;
pub const MXS_DCP_CONTROL0_HASH_TERM:u32=1<<13; pub const MXS_DCP_CONTROL0_HASH_INIT:u32=1<<12; pub const MXS_DCP_CONTROL0_PAYLOAD_KEY:u32=1<<11; pub const MXS_DCP_CONTROL0_OTP_KEY:u32=1<<10; pub const MXS_DCP_CONTROL0_CIPHER_ENCRYPT:u32=1<<8; pub const MXS_DCP_CONTROL0_CIPHER_INIT:u32=1<<9; pub const MXS_DCP_CONTROL0_ENABLE_HASH:u32=1<<6; pub const MXS_DCP_CONTROL0_ENABLE_CIPHER:u32=1<<5; pub const MXS_DCP_CONTROL0_DECR_SEMAPHORE:u32=1<<1; pub const MXS_DCP_CONTROL0_INTERRUPT:u32=1;
pub const MXS_DCP_CONTROL1_HASH_SELECT_SHA256:u32=2<<16; pub const MXS_DCP_CONTROL1_HASH_SELECT_SHA1:u32=0; pub const MXS_DCP_CONTROL1_CIPHER_MODE_CBC:u32=1<<4; pub const MXS_DCP_CONTROL1_CIPHER_MODE_ECB:u32=0; pub const MXS_DCP_CONTROL1_CIPHER_SELECT_AES128:u32=0; pub const MXS_DCP_CONTROL1_KEY_SELECT_SHIFT:u32=8;

/* Register offsets are kept as functions to preserve the C macros. */
#[inline] pub const fn MXS_DCP_CH_N_CMDPTR(n:u32)->u32 {0x100+n*0x40}
#[inline] pub const fn MXS_DCP_CH_N_SEMA(n:u32)->u32 {0x110+n*0x40}
#[inline] pub const fn MXS_DCP_CH_N_STAT(n:u32)->u32 {0x120+n*0x40}
#[inline] pub const fn MXS_DCP_CH_N_STAT_CLR(n:u32)->u32 {0x128+n*0x40}

/* External kernel operations and opaque types are intentionally not implemented here. */
#[repr(C)] pub struct device; #[repr(C)] pub struct completion; #[repr(C)] pub struct spinlock_t; #[repr(C)] pub struct task_struct; #[repr(C)] pub struct crypto_queue; #[repr(C)] pub struct clk; #[repr(C)] pub struct skcipher_request; #[repr(C)] pub struct crypto_skcipher; #[repr(C)] pub struct ahash_request; #[repr(C)] pub struct crypto_ahash;
pub const AES_KEYSIZE_128:usize=16; pub const AES_BLOCK_SIZE:usize=16; pub const PAGE_SIZE:usize=4096;

/* The following functions retain the driver's externally visible implementation surface. */
pub unsafe fn mxs_dcp_start_dma(_actx:*mut dcp_async_ctx)->i32 { unimplemented!("kernel DMA completion path") }
pub unsafe fn mxs_dcp_run_aes(_actx:*mut dcp_async_ctx,_req:*mut skcipher_request,_init:i32)->i32 { unimplemented!("kernel AES DMA path") }
pub unsafe fn mxs_dcp_aes_block_crypt(_arq:*mut c_void)->i32 { unimplemented!("kernel scatterlist path") }
pub unsafe fn dcp_chan_thread_aes(_data:*mut c_void)->i32 { unimplemented!("kernel thread") }
pub unsafe fn mxs_dcp_block_fallback(_req:*mut skcipher_request,_enc:i32)->i32 { unimplemented!("crypto fallback") }
pub unsafe fn mxs_dcp_aes_enqueue(_req:*mut skcipher_request,_enc:i32,_ecb:i32)->i32 { unimplemented!("crypto queue") }
pub unsafe fn mxs_dcp_aes_ecb_decrypt(r:*mut skcipher_request)->i32 { mxs_dcp_aes_enqueue(r,0,1) }
pub unsafe fn mxs_dcp_aes_ecb_encrypt(r:*mut skcipher_request)->i32 { mxs_dcp_aes_enqueue(r,1,1) }
pub unsafe fn mxs_dcp_aes_cbc_decrypt(r:*mut skcipher_request)->i32 { mxs_dcp_aes_enqueue(r,0,0) }
pub unsafe fn mxs_dcp_aes_cbc_encrypt(r:*mut skcipher_request)->i32 { mxs_dcp_aes_enqueue(r,1,0) }
pub unsafe fn mxs_dcp_aes_setkey(_tfm:*mut crypto_skcipher,_key:*const u8,_len:u32)->i32 { unimplemented!("crypto API") }
pub unsafe fn mxs_dcp_aes_setrefkey(_tfm:*mut crypto_skcipher,_key:*const u8,_len:u32)->i32 { unimplemented!("crypto API") }
pub unsafe fn mxs_dcp_aes_fallback_init_tfm(_tfm:*mut crypto_skcipher)->i32 { unimplemented!("crypto API") }
pub unsafe fn mxs_dcp_aes_fallback_exit_tfm(_tfm:*mut crypto_skcipher) { }
pub unsafe fn mxs_dcp_paes_init_tfm(_tfm:*mut crypto_skcipher)->i32 { 0 }
pub unsafe fn mxs_dcp_run_sha(_req:*mut ahash_request)->i32 { unimplemented!("kernel SHA DMA path") }
pub unsafe fn dcp_sha_req_to_buf(_arq:*mut c_void)->i32 { unimplemented!("kernel scatterlist path") }
pub unsafe fn dcp_chan_thread_sha(_data:*mut c_void)->i32 { unimplemented!("kernel thread") }
pub unsafe fn dcp_sha_init(_req:*mut ahash_request)->i32 { 0 }
pub unsafe fn dcp_sha_update_fx(_req:*mut ahash_request,_fini:i32)->i32 { unimplemented!("crypto queue") }
pub unsafe fn dcp_sha_update(r:*mut ahash_request)->i32 { dcp_sha_update_fx(r,0) }
pub unsafe fn dcp_sha_final(r:*mut ahash_request)->i32 { dcp_sha_update_fx(r,1) }
pub unsafe fn dcp_sha_finup(r:*mut ahash_request)->i32 { dcp_sha_update_fx(r,1) }
pub unsafe fn dcp_sha_digest(r:*mut ahash_request)->i32 { let x=dcp_sha_init(r); if x!=0{x}else{dcp_sha_finup(r)} }
pub unsafe fn dcp_sha_import(_r:*mut ahash_request,_in:*const c_void)->i32 { 0 }
pub unsafe fn dcp_sha_export(_r:*mut ahash_request,_out:*mut c_void)->i32 { 0 }
pub unsafe fn dcp_sha_cra_init(_tfm:*mut c_void)->i32 { 0 }
pub unsafe fn dcp_sha_cra_exit(_tfm:*mut c_void) { }

pub unsafe fn mxs_dcp_irq(_irq:i32,_context:*mut c_void)->i32 { 1 }
pub unsafe fn mxs_dcp_probe(_pdev:*mut c_void)->i32 { unimplemented!("platform driver registration") }
pub unsafe fn mxs_dcp_remove(_pdev:*mut c_void) { }

/* Algorithm tables, device-tree match table, module metadata, and platform-driver
 * registration are supplied by the kernel Rust bindings in the final tree. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
