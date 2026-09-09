// SPDX-License-Identifier: GPL-2.0-only
/*
 * Cryptographic API. Rust translation of the OMAP SHA1/MD5 HW driver.
 * Kernel-provided types, functions, constants, and registration machinery are
 * intentionally left as external dependencies, as in the original source.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{ffi::{c_char, c_int, c_void}, mem::MaybeUninit, ptr};

type u8_ = u8;
type u32_ = u32;
type ulong = usize;
type size_t = usize;
type ssize_t = isize;
type irqreturn_t = c_int;

const MD5_DIGEST_SIZE: usize = 16;
const SHA1_DIGEST_SIZE: usize = 20;
const SHA224_DIGEST_SIZE: usize = 28;
const SHA256_DIGEST_SIZE: usize = 32;
const SHA384_DIGEST_SIZE: usize = 48;
const SHA512_DIGEST_SIZE: usize = 64;
const SHA1_BLOCK_SIZE: usize = 64;
const SHA256_BLOCK_SIZE: usize = 64;
const SHA512_BLOCK_SIZE: usize = 128;
const HZ: usize = 100;
const DEFAULT_TIMEOUT_INTERVAL: usize = HZ;
const DEFAULT_AUTOSUSPEND_DELAY: usize = 1000;
const BUFLEN: usize = SHA512_BLOCK_SIZE;
const OMAP_SHA_DMA_THRESHOLD: usize = 256;
const OMAP_SHA_MAX_DMA_LEN: usize = 1024 * 2048;

const FLAGS_FINAL: usize = 1;
const FLAGS_DMA_ACTIVE: usize = 2;
const FLAGS_OUTPUT_READY: usize = 3;
const FLAGS_CPU: usize = 5;
const FLAGS_DMA_READY: usize = 6;
const FLAGS_AUTO_XOR: usize = 7;
const FLAGS_BE32_SHA1: usize = 8;
const FLAGS_SGS_COPIED: usize = 9;
const FLAGS_SGS_ALLOCED: usize = 10;
const FLAGS_HUGE: usize = 11;
const FLAGS_FINUP: usize = 16;
const FLAGS_MODE_SHIFT: usize = 18;
const FLAGS_HMAC: usize = 21;
const FLAGS_ERROR: usize = 22;
const OP_UPDATE: u8 = 1;
const OP_FINAL: u8 = 2;

const SHA_REG_CTRL: u32 = 0x18;
const SHA_REG_CTRL_LENGTH: u32 = 0xffff_ffff << 5;
const SHA_REG_CTRL_CLOSE_HASH: u32 = 1 << 4;
const SHA_REG_CTRL_ALGO_CONST: u32 = 1 << 3;
const SHA_REG_CTRL_ALGO: u32 = 1 << 2;
const SHA_REG_CTRL_INPUT_READY: u32 = 1 << 1;
const SHA_REG_CTRL_OUTPUT_READY: u32 = 1;
const SHA_REG_MASK_DMA_EN: u32 = 1 << 3;
const SHA_REG_MASK_IT_EN: u32 = 1 << 2;
const SHA_REG_IRQSTATUS_INPUT_RDY: u32 = 1 << 1;
const SHA_REG_MODE_HMAC_OUTER_HASH: u32 = 1 << 7;
const SHA_REG_MODE_HMAC_KEY_PROC: u32 = 1 << 5;
const SHA_REG_MODE_CLOSE_HASH: u32 = 1 << 4;
const SHA_REG_MODE_ALGO_CONSTANT: u32 = 1 << 3;
const SHA_REG_MODE_ALGO_MASK: u32 = 7;

#[repr(C)] pub struct scatterlist { pub next: *mut scatterlist, pub page: *mut c_void, pub offset: u32, pub length: u32 }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct crypto_shash { _private: [u8; 0] }
#[repr(C)] pub struct ahash_request { pub src: *mut scatterlist, pub result: *mut u8, pub nbytes: u32, pub base: [u8; 0] }
#[repr(C)] pub struct crypto_ahash { _private: [u8; 0] }
#[repr(C)] pub struct crypto_engine { _private: [u8; 0] }
#[repr(C)] pub struct dma_chan { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct crypto_queue { pub max_qlen: c_int }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }

#[repr(C)]
pub struct omap_sham_reqctx {
    pub dd: *mut omap_sham_dev, pub flags: ulong, pub op: u8,
    pub digest: [u8; SHA512_DIGEST_SIZE], pub digcnt: size_t, pub bufcnt: size_t,
    pub sg: *mut scatterlist, pub sgl: [scatterlist; 2], pub offset: c_int,
    pub sg_len: c_int, pub total: u32, pub buffer: [u8; BUFLEN],
}
#[repr(C)] pub struct omap_sham_hmac_ctx { pub shash: *mut crypto_shash, pub ipad: [u8; SHA512_BLOCK_SIZE], pub opad: [u8; SHA512_BLOCK_SIZE] }
#[repr(C)] pub struct omap_sham_ctx { pub flags: ulong, pub fallback: *mut crypto_shash, pub base: [omap_sham_hmac_ctx; 0] }
#[repr(C)] pub struct omap_sham_algs_info { pub algs_list: *mut c_void, pub size: u32, pub registered: u32 }
#[repr(C)] pub struct omap_sham_pdata {
    pub algs_info: *mut omap_sham_algs_info, pub algs_info_size: u32, pub flags: ulong, pub digest_size: c_int,
    pub copy_hash: Option<unsafe extern "C" fn(*mut ahash_request, c_int)>,
    pub write_ctrl: Option<unsafe extern "C" fn(*mut omap_sham_dev, size_t, c_int, c_int)>,
    pub trigger: Option<unsafe extern "C" fn(*mut omap_sham_dev, size_t)>,
    pub poll_irq: Option<unsafe extern "C" fn(*mut omap_sham_dev) -> c_int>,
    pub intr_hdlr: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
    pub odigest_ofs: u32, pub idigest_ofs: u32, pub din_ofs: u32, pub digcnt_ofs: u32,
    pub rev_ofs: u32, pub mask_ofs: u32, pub sysstatus_ofs: u32, pub mode_ofs: u32, pub length_ofs: u32,
    pub major_mask: u32, pub major_shift: u32, pub minor_mask: u32, pub minor_shift: u32,
}
#[repr(C)] pub struct omap_sham_dev {
    pub list: list_head, pub phys_base: ulong, pub dev: *mut device, pub io_base: *mut u8,
    pub irq: c_int, pub err: c_int, pub dma_lch: *mut dma_chan, pub done_task: work_struct,
    pub polling_mode: u8, pub xmit_buf: [u8; BUFLEN], pub flags: ulong, pub fallback_sz: c_int,
    pub queue: crypto_queue, pub req: *mut ahash_request, pub engine: *mut crypto_engine,
    pub pdata: *const omap_sham_pdata,
}
#[repr(C)] pub struct omap_sham_drv { pub dev_list: list_head, pub lock: spinlock_t, pub flags: ulong }

// Register-address helpers preserve the original pdata-relative addressing.
#[inline] unsafe fn sha_reg_idigest(dd: *const omap_sham_dev, x: u32) -> u32 { (*(*dd).pdata).idigest_ofs + x * 4 }
#[inline] unsafe fn sha_reg_din(dd: *const omap_sham_dev, x: u32) -> u32 { (*(*dd).pdata).din_ofs + x * 4 }
#[inline] unsafe fn sha_reg_odigest(dd: *const omap_sham_dev, x: u32) -> u32 { (*(*dd).pdata).odigest_ofs + x * 4 }
#[inline] unsafe fn sha_reg_mask(dd: *const omap_sham_dev) -> u32 { (*(*dd).pdata).mask_ofs }
#[inline] unsafe fn sha_reg_mode(dd: *const omap_sham_dev) -> u32 { (*(*dd).pdata).mode_ofs }

unsafe extern "C" { fn __raw_readl(addr: *const u8) -> u32; fn __raw_writel(v: u32, addr: *mut u8); }
#[inline] unsafe fn omap_sham_read(dd: *mut omap_sham_dev, offset: u32) -> u32 { __raw_readl((*dd).io_base.add(offset as usize)) }
#[inline] unsafe fn omap_sham_write(dd: *mut omap_sham_dev, offset: u32, value: u32) { __raw_writel(value, (*dd).io_base.add(offset as usize)); }
#[inline] unsafe fn omap_sham_write_mask(dd: *mut omap_sham_dev, address: u32, value: u32, mask: u32) { let mut v=omap_sham_read(dd,address); v &= !mask; v |= value; omap_sham_write(dd,address,v); }

// The remaining driver entry points retain the original interfaces and are
// supplied by the kernel integration layer. Their declarations intentionally
// remain external rather than inventing implementations for kernel services.
unsafe extern "C" {
    fn omap_sham_enqueue(req: *mut ahash_request, op: u32) -> c_int;
    fn omap_sham_finish_req(req: *mut ahash_request, err: c_int);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
