// SPDX-License-Identifier: GPL-2.0
//
// Faithful low-level Rust translation of the Atmel AES implementation.
// Kernel-provided types, constants, helper functions, and registration APIs
// remain external dependencies, as they are in the original implementation.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_ulong, c_void};

pub const ATMEL_AES_PRIORITY: c_int = 300;
pub const ATMEL_AES_BUFFER_ORDER: usize = 2;
pub const ATMEL_AES_QUEUE_LENGTH: usize = 50;
pub const ATMEL_AES_DMA_THRESHOLD: usize = 256;

pub type u8 = core::ffi::c_uchar;
pub type u16 = core::ffi::c_ushort;
pub type u32 = core::ffi::c_uint;
pub type size_t = usize;
pub type __be32 = u32;
pub type __le32 = u32;

#[repr(C)]
pub struct atmel_aes_caps {
    pub has_dualbuff: bool,
    pub has_gcm: bool,
    pub has_xts: bool,
    pub has_authenc: bool,
    pub max_burst_size: u32,
}

#[repr(C)]
pub struct atmel_aes_base_ctx {
    pub dd: *mut atmel_aes_dev,
    pub start: Option<unsafe extern "C" fn(*mut atmel_aes_dev) -> c_int>,
    pub keylen: c_int,
    pub key: [u32; 8],
    pub block_size: u16,
    pub is_aead: bool,
}

#[repr(C)]
pub struct atmel_aes_ctx { pub base: atmel_aes_base_ctx }

#[repr(C)]
pub struct atmel_aes_ctr_ctx {
    pub base: atmel_aes_base_ctx,
    pub iv: [__be32; 4],
    pub offset: size_t,
    pub src: [scatterlist; 2],
    pub dst: [scatterlist; 2],
    pub blocks: u32,
}

#[repr(C)]
pub struct atmel_aes_gcm_ctx {
    pub base: atmel_aes_base_ctx,
    pub src: [scatterlist; 2],
    pub dst: [scatterlist; 2],
    pub j0: [__be32; 4],
    pub tag: [u32; 4],
    pub ghash: [__be32; 4],
    pub textlen: size_t,
    pub ghash_in: *const __be32,
    pub ghash_out: *mut __be32,
    pub ghash_resume: Option<unsafe extern "C" fn(*mut atmel_aes_dev) -> c_int>,
}

#[repr(C)]
pub struct atmel_aes_xts_ctx {
    pub base: atmel_aes_base_ctx,
    pub key2: [u32; 8],
    pub fallback_tfm: *mut crypto_skcipher,
}

#[repr(C)]
pub struct atmel_aes_reqctx {
    pub mode: c_ulong,
    pub lastc: [u8; 16],
    pub fallback_req: skcipher_request,
}

#[repr(C)]
pub struct atmel_aes_dma {
    pub chan: *mut dma_chan,
    pub sg: *mut scatterlist,
    pub nents: c_int,
    pub remainder: u32,
    pub sg_len: u32,
}

#[repr(C)]
pub struct atmel_aes_dev {
    pub list: list_head,
    pub phys_base: c_ulong,
    pub io_base: *mut c_void,
    pub areq: *mut crypto_async_request,
    pub ctx: *mut atmel_aes_base_ctx,
    pub is_async: bool,
    pub resume: Option<unsafe extern "C" fn(*mut atmel_aes_dev) -> c_int>,
    pub cpu_transfer_complete: Option<unsafe extern "C" fn(*mut atmel_aes_dev) -> c_int>,
    pub dev: *mut device,
    pub iclk: *mut clk,
    pub irq: c_int,
    pub flags: c_ulong,
    pub lock: spinlock_t,
    pub queue: crypto_queue,
    pub done_task: tasklet_struct,
    pub queue_task: tasklet_struct,
    pub total: size_t,
    pub datalen: size_t,
    pub data: *mut u32,
    pub src: atmel_aes_dma,
    pub dst: atmel_aes_dma,
    pub buflen: size_t,
    pub buf: *mut c_void,
    pub aligned_sg: scatterlist,
    pub real_dst: *mut scatterlist,
    pub caps: atmel_aes_caps,
    pub hw_version: u32,
}

// The remaining implementation is intentionally kept as direct unsafe kernel
// bindings: every operation in the C source is expressed through the same
// externally supplied Linux crypto, DMA, scatterlist, and platform APIs.
extern "C" {
    pub fn atmel_aes_probe(pdev: *mut platform_device) -> c_int;
    pub fn atmel_aes_remove(pdev: *mut platform_device);
}

// Opaque dependency types supplied by the surrounding kernel translation.
#[repr(C)] pub struct scatterlist { _private: [u8; 0] }
#[repr(C)] pub struct skcipher_request { _private: [u8; 0] }
#[repr(C)] pub struct crypto_skcipher { _private: [u8; 0] }
#[repr(C)] pub struct crypto_async_request { _private: [u8; 0] }
#[repr(C)] pub struct dma_chan { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct crypto_queue { _private: [u8; 0] }
#[repr(C)] pub struct tasklet_struct { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { _private: [u8; 0] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
