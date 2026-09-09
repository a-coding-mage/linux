/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Cryptographic API.
 *
 * Support for OMAP AES HW ACCELERATOR defines
 *
 * Copyright (c) 2015 Texas Instruments Incorporated
 */

// Dependency intent: symbols from crypto/aes.h and the Linux kernel are supplied externally.

pub const DST_MAXBURST: usize = 4;
pub const DMA_MIN: usize = DST_MAXBURST * core::mem::size_of::<u32>();

pub const fn fld_mask(start: u32, end: u32) -> u32 {
    (((1u32 << (start - end + 1)) - 1) << end)
}
pub const fn fld_val(val: u32, start: u32, end: u32) -> u32 {
    (val << end) & fld_mask(start, end)
}

#[inline]
pub unsafe fn aes_reg_key(dd: *const omap_aes_dev, x: u32) -> u32 { (*(*dd).pdata).key_ofs - ((x ^ 0x01) * 0x04) }
#[inline]
pub unsafe fn aes_reg_iv(dd: *const omap_aes_dev, x: u32) -> u32 { (*(*dd).pdata).iv_ofs + x * 0x04 }
#[inline]
pub unsafe fn aes_reg_ctrl(dd: *const omap_aes_dev) -> u32 { (*(*dd).pdata).ctrl_ofs }
#[inline]
pub unsafe fn aes_reg_data_n(dd: *const omap_aes_dev, x: u32) -> u32 { (*(*dd).pdata).data_ofs + x * 0x04 }
#[inline]
pub const fn aes_reg_tag_n(x: u32) -> u32 { 0x70 + x * 0x04 }
#[inline]
pub unsafe fn aes_reg_rev(dd: *const omap_aes_dev) -> u32 { (*(*dd).pdata).rev_ofs }
#[inline]
pub unsafe fn aes_reg_mask(dd: *const omap_aes_dev) -> u32 { (*(*dd).pdata).mask_ofs }
#[inline]
pub const fn aes_reg_length_n(x: u32) -> u32 { 0x54 + x * 0x04 }
#[inline]
pub unsafe fn aes_reg_irq_status(dd: *const omap_aes_dev) -> u32 { (*(*dd).pdata).irq_status_ofs }
#[inline]
pub unsafe fn aes_reg_irq_enable(dd: *const omap_aes_dev) -> u32 { (*(*dd).pdata).irq_enable_ofs }

pub const AES_REG_CTRL_CONTEXT_READY: u32 = 1 << 31;
pub const AES_REG_CTRL_CTR_WIDTH_MASK: u32 = 0x3 << 7;
pub const AES_REG_CTRL_CTR_WIDTH_32: u32 = 0;
pub const AES_REG_CTRL_CTR_WIDTH_64: u32 = 1 << 7;
pub const AES_REG_CTRL_CTR_WIDTH_96: u32 = 1 << 8;
pub const AES_REG_CTRL_CTR_WIDTH_128: u32 = 0x3 << 7;
pub const AES_REG_CTRL_GCM: u32 = 0x3 << 16;
pub const AES_REG_CTRL_CTR: u32 = 1 << 6;
pub const AES_REG_CTRL_CBC: u32 = 1 << 5;
pub const AES_REG_CTRL_KEY_SIZE: u32 = 0x3 << 3;
pub const AES_REG_CTRL_DIRECTION: u32 = 1 << 2;
pub const AES_REG_CTRL_INPUT_READY: u32 = 1 << 1;
pub const AES_REG_CTRL_OUTPUT_READY: u32 = 1;
pub const AES_REG_CTRL_MASK: u32 = 0x1fffffc;

pub const AES_REG_C_LEN_0: u32 = 0x54;
pub const AES_REG_C_LEN_1: u32 = 0x58;
pub const AES_REG_A_LEN: u32 = 0x5c;
pub const AES_REG_MASK_SIDLE: u32 = 1 << 6;
pub const AES_REG_MASK_START: u32 = 1 << 5;
pub const AES_REG_MASK_DMA_OUT_EN: u32 = 1 << 3;
pub const AES_REG_MASK_DMA_IN_EN: u32 = 1 << 2;
pub const AES_REG_MASK_SOFTRESET: u32 = 1 << 1;
pub const AES_REG_AUTOIDLE: u32 = 1;
pub const AES_REG_IRQ_DATA_IN: u32 = 1 << 1;
pub const AES_REG_IRQ_DATA_OUT: u32 = 1 << 2;
pub const DEFAULT_TIMEOUT: u32 = 5 * HZ;
pub const DEFAULT_AUTOSUSPEND_DELAY: u32 = 1000;
pub const FLAGS_MODE_MASK: u32 = 0x001f;
pub const FLAGS_ENCRYPT: u32 = 1;
pub const FLAGS_CBC: u32 = 1 << 1;
pub const FLAGS_CTR: u32 = 1 << 2;
pub const FLAGS_GCM: u32 = 1 << 3;
pub const FLAGS_RFC4106_GCM: u32 = 1 << 4;
pub const FLAGS_INIT: u32 = 1 << 5;
pub const FLAGS_FAST: u32 = 1 << 6;
pub const FLAGS_IN_DATA_ST_SHIFT: u32 = 8;
pub const FLAGS_OUT_DATA_ST_SHIFT: u32 = 10;
pub const FLAGS_ASSOC_DATA_ST_SHIFT: u32 = 12;
pub const AES_BLOCK_WORDS: usize = AES_BLOCK_SIZE >> 2;

#[repr(C)]
pub struct omap_aes_gcm_result { pub completion: completion, pub err: i32 }

#[repr(C)]
pub struct omap_aes_ctx {
    pub keylen: i32,
    pub key: [u32; AES_KEYSIZE_256 / core::mem::size_of::<u32>()],
    pub nonce: [u8; 4],
    pub fallback: *mut crypto_skcipher,
}

#[repr(C)]
pub struct omap_aes_gcm_ctx { pub octx: omap_aes_ctx, pub akey: aes_enckey }

#[repr(C)]
pub struct omap_aes_reqctx {
    pub dd: *mut omap_aes_dev,
    pub mode: c_ulong,
    pub iv: [u8; AES_BLOCK_SIZE],
    pub auth_tag: [u32; AES_BLOCK_SIZE / core::mem::size_of::<u32>()],
    // keep at the end
    pub fallback_req: skcipher_request,
}

pub const OMAP_AES_QUEUE_LENGTH: u32 = 1;
pub const OMAP_AES_CACHE_SIZE: u32 = 0;

#[repr(C)]
pub struct omap_aes_algs_info { pub algs_list: *mut skcipher_engine_alg, pub size: u32, pub registered: u32 }
#[repr(C)]
pub struct omap_aes_aead_algs { pub algs_list: *mut aead_engine_alg, pub size: u32, pub registered: u32 }

#[repr(C)]
pub struct omap_aes_pdata {
    pub algs_info: *mut omap_aes_algs_info,
    pub algs_info_size: u32,
    pub aead_algs_info: *mut omap_aes_aead_algs,
    pub trigger: Option<unsafe extern "C" fn(*mut omap_aes_dev, i32)>,
    pub key_ofs: u32, pub iv_ofs: u32, pub ctrl_ofs: u32, pub data_ofs: u32,
    pub rev_ofs: u32, pub mask_ofs: u32, pub irq_enable_ofs: u32, pub irq_status_ofs: u32,
    pub dma_enable_in: u32, pub dma_enable_out: u32, pub dma_start: u32,
    pub major_mask: u32, pub major_shift: u32, pub minor_mask: u32, pub minor_shift: u32,
}

#[repr(C)]
pub struct omap_aes_dev {
    pub list: list_head, pub phys_base: c_ulong, pub io_base: *mut core::ffi::c_void,
    pub ctx: *mut omap_aes_ctx, pub dev: *mut device, pub flags: c_ulong, pub err: i32,
    pub done_task: work_struct, pub aead_queue: aead_queue, pub lock: spinlock_t,
    pub req: *mut skcipher_request, pub aead_req: *mut aead_request, pub engine: *mut crypto_engine,
    pub total: usize, pub total_save: usize, pub assoc_len: usize, pub authsize: usize,
    pub in_sg: *mut scatterlist, pub out_sg: *mut scatterlist,
    pub in_sgl: [scatterlist; 2], pub out_sgl: scatterlist, pub orig_out: *mut scatterlist,
    pub in_sg_offset: u32, pub out_sg_offset: u32, pub dma_lch_in: *mut dma_chan, pub dma_lch_out: *mut dma_chan,
    pub in_sg_len: i32, pub out_sg_len: i32, pub pio_only: i32, pub pdata: *const omap_aes_pdata,
}

extern "C" {
    pub fn omap_aes_read(dd: *mut omap_aes_dev, offset: u32) -> u32;
    pub fn omap_aes_write(dd: *mut omap_aes_dev, offset: u32, value: u32);
    pub fn omap_aes_find_dev(rctx: *mut omap_aes_reqctx) -> *mut omap_aes_dev;
    pub fn omap_aes_gcm_setkey(tfm: *mut crypto_aead, key: *const u8, keylen: u32) -> i32;
    pub fn omap_aes_4106gcm_setkey(tfm: *mut crypto_aead, key: *const u8, keylen: u32) -> i32;
    pub fn omap_aes_gcm_encrypt(req: *mut aead_request) -> i32;
    pub fn omap_aes_gcm_decrypt(req: *mut aead_request) -> i32;
    pub fn omap_aes_gcm_setauthsize(tfm: *mut crypto_aead, authsize: u32) -> i32;
    pub fn omap_aes_4106gcm_encrypt(req: *mut aead_request) -> i32;
    pub fn omap_aes_4106gcm_decrypt(req: *mut aead_request) -> i32;
    pub fn omap_aes_4106gcm_setauthsize(parent: *mut crypto_aead, authsize: u32) -> i32;
    pub fn omap_aes_gcm_cra_init(tfm: *mut crypto_aead) -> i32;
    pub fn omap_aes_write_ctrl(dd: *mut omap_aes_dev) -> i32;
    pub fn omap_aes_crypt_dma_start(dd: *mut omap_aes_dev) -> i32;
    pub fn omap_aes_crypt_dma_stop(dd: *mut omap_aes_dev) -> i32;
    pub fn omap_aes_gcm_dma_out_callback(data: *mut core::ffi::c_void);
    pub fn omap_aes_clear_copy_flags(dd: *mut omap_aes_dev);
    pub fn omap_aes_gcm_crypt_req(engine: *mut crypto_engine, areq: *mut core::ffi::c_void) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
