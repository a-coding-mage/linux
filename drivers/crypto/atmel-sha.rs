// SPDX-License-Identifier: GPL-2.0
/* Source-level Rust translation of crypto/atmel-sha.c.  Kernel symbols are
 * intentionally left as external dependencies, as in the original driver. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub const ATMEL_SHA_PRIORITY: u32 = 300;
pub const SHA_FLAGS_BUSY: usize = 1 << 0;
pub const SHA_FLAGS_FINAL: usize = 1 << 1;
pub const SHA_FLAGS_DMA_ACTIVE: usize = 1 << 2;
pub const SHA_FLAGS_OUTPUT_READY: usize = 1 << 3;
pub const SHA_FLAGS_INIT: usize = 1 << 4;
pub const SHA_FLAGS_CPU: usize = 1 << 5;
pub const SHA_FLAGS_DMA_READY: usize = 1 << 6;
pub const SHA_FLAGS_DUMP_REG: usize = 1 << 7;
pub const SHA_FLAGS_FINUP: usize = 1 << 16;
pub const SHA_FLAGS_SG: usize = 1 << 17;
pub const SHA_FLAGS_ERROR: usize = 1 << 23;
pub const SHA_FLAGS_PAD: usize = 1 << 24;
pub const SHA_FLAGS_RESTORE: usize = 1 << 25;
pub const SHA_FLAGS_IDATAR0: usize = 1 << 26;
pub const SHA_FLAGS_WAIT_DATARDY: usize = 1 << 27;
pub const SHA_OP_INIT: usize = 0;
pub const SHA_OP_UPDATE: usize = 1;
pub const SHA_OP_FINAL: usize = 2;
pub const SHA_OP_DIGEST: usize = 3;
pub const SHA_BUFFER_LEN: usize = 4096 / 16;
pub const ATMEL_SHA_DMA_THRESHOLD: usize = 56;

pub type u8 = core::ffi::c_uchar;
pub type u32 = core::ffi::c_uint;
pub type u64 = core::ffi::c_ulonglong;
pub type dma_addr_t = usize;
pub type atmel_sha_fn_t = unsafe extern "C" fn(*mut atmel_sha_dev) -> i32;

#[repr(C)] pub struct atmel_sha_caps { pub has_dma: bool, pub has_dualbuff: bool, pub has_sha224: bool, pub has_sha_384_512: bool, pub has_uihv: bool, pub has_hmac: bool }
#[repr(C)] pub struct atmel_sha_reqctx {
    pub dd: *mut atmel_sha_dev, pub flags: usize, pub op: usize,
    pub digest: [u8; 64], pub digcnt: [u64; 2], pub bufcnt: usize, pub buflen: usize,
    pub dma_addr: dma_addr_t, pub sg: *mut scatterlist, pub offset: u32, pub total: u32,
    pub block_size: usize, pub hash_size: usize, pub buffer: [u8; SHA_BUFFER_LEN + 128],
}
#[repr(C)] pub struct atmel_sha_ctx { pub dd: *mut atmel_sha_dev, pub start: Option<atmel_sha_fn_t>, pub flags: usize }
#[repr(C)] pub struct atmel_sha_dma { pub chan: *mut dma_chan, pub dma_conf: dma_slave_config, pub sg: *mut scatterlist, pub nents: i32, pub last_sg_length: u32 }
#[repr(C)] pub struct atmel_sha_dev {
    pub list: list_head, pub phys_base: usize, pub dev: *mut device, pub iclk: *mut clk, pub irq: i32,
    pub io_base: *mut c_void, pub lock: spinlock_t, pub done_task: tasklet_struct, pub queue_task: tasklet_struct,
    pub flags: usize, pub queue: crypto_queue, pub req: *mut ahash_request, pub is_async: bool, pub force_complete: bool,
    pub resume: Option<atmel_sha_fn_t>, pub cpu_transfer_complete: Option<atmel_sha_fn_t>, pub dma_lch_in: atmel_sha_dma,
    pub caps: atmel_sha_caps, pub tmp: scatterlist, pub hw_version: u32,
}
#[repr(C)] pub struct atmel_sha_drv { pub dev_list: list_head, pub lock: spinlock_t }
#[repr(C)] pub struct atmel_sha_hmac_key { pub valid: bool, pub keylen: u32, pub buffer: [u8; 128], pub keydup: *mut u8 }
#[repr(C)] pub struct atmel_sha_hmac_ctx { pub base: atmel_sha_ctx, pub hkey: atmel_sha_hmac_key, pub ipad: [u32; 32], pub opad: [u32; 32], pub resume: Option<atmel_sha_fn_t> }
#[repr(C)] pub struct atmel_sha_authenc_ctx { pub tfm: *mut crypto_ahash }
#[repr(C)] pub struct atmel_sha_authenc_reqctx { pub base: atmel_sha_reqctx, pub cb: Option<unsafe extern "C" fn(*mut atmel_aes_dev, i32, bool)>, pub aes_dev: *mut atmel_aes_dev, pub assoc: *mut scatterlist, pub assoclen: u32, pub textlen: u32, pub digest: *mut u32, pub digestlen: u32 }

#[repr(C)] pub struct list_head { _private: [usize; 2] }
#[repr(C)] pub struct spinlock_t { _private: [usize; 4] }
#[repr(C)] pub struct tasklet_struct { _private: [usize; 8] }
#[repr(C)] pub struct dma_slave_config { pub src_maxburst: u32, pub dst_maxburst: u32, _private: [usize; 8] }
#[repr(C)] pub struct scatterlist { pub offset: u32, pub length: u32 }
#[repr(C)] pub struct dma_chan { _private: [usize; 1] }
#[repr(C)] pub struct device { _private: [usize; 1] }
#[repr(C)] pub struct clk { _private: [usize; 1] }
#[repr(C)] pub struct crypto_queue { _private: [usize; 4] }
#[repr(C)] pub struct ahash_request { pub nbytes: u32, pub src: *mut scatterlist, pub result: *mut u8 }
#[repr(C)] pub struct crypto_ahash { _private: [usize; 1] }
#[repr(C)] pub struct crypto_tfm { _private: [usize; 1] }
#[repr(C)] pub struct ahash_alg { _private: [usize; 1] }
#[repr(C)] pub struct platform_device { _private: [usize; 1] }
#[repr(C)] pub struct atmel_aes_dev { _private: [usize; 1] }

extern "C" {
    fn atmel_sha_read(dd: *mut atmel_sha_dev, offset: u32) -> u32;
    fn atmel_sha_write(dd: *mut atmel_sha_dev, offset: u32, value: u32);
    fn atmel_sha_complete(dd: *mut atmel_sha_dev, err: i32) -> i32;
    fn atmel_sha_init(req: *mut ahash_request) -> i32;
    fn atmel_sha_update(req: *mut ahash_request) -> i32;
    fn atmel_sha_final(req: *mut ahash_request) -> i32;
    fn atmel_sha_finup(req: *mut ahash_request) -> i32;
    fn atmel_sha_digest(req: *mut ahash_request) -> i32;
    fn atmel_sha_export(req: *mut ahash_request, out: *mut c_void) -> i32;
    fn atmel_sha_import(req: *mut ahash_request, input: *const c_void) -> i32;
    fn atmel_sha_hmac_init(req: *mut ahash_request) -> i32;
    fn atmel_sha_hmac_digest(req: *mut ahash_request) -> i32;
    fn atmel_sha_hmac_setkey(tfm: *mut crypto_ahash, key: *const u8, keylen: u32) -> i32;
    fn atmel_sha_probe(pdev: *mut platform_device) -> i32;
    fn atmel_sha_remove(pdev: *mut platform_device);
}

pub unsafe fn atmel_sha_authenc_is_ready() -> bool { false }
pub unsafe fn atmel_sha_authenc_get_reqsize() -> usize { core::mem::size_of::<atmel_sha_authenc_reqctx>() }
pub unsafe fn atmel_sha_authenc_free(_auth: *mut atmel_sha_authenc_ctx) {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
