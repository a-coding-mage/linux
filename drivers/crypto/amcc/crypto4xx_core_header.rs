/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * AMCC SoC PPC4xx Crypto Driver
 *
 * Copyright (c) 2008 Applied Micro Circuits Corporation.
 * All rights reserved. James Hsiao <jhsiao@amcc.com>
 *
 * This is the header file for AMCC Crypto offload Linux device driver for
 * use with Linux CryptoAPI.
 */

// Dependencies supplied by the surrounding translation unit and Linux headers
// are intentionally left external.

pub const PPC460SX_SDR0_SRST: u32 = 0x201;
pub const PPC405EX_SDR0_SRST: u32 = 0x200;
pub const PPC460EX_SDR0_SRST: u32 = 0x201;
pub const PPC460EX_CE_RESET: u32 = 0x08000000;
pub const PPC460SX_CE_RESET: u32 = 0x20000000;
pub const PPC405EX_CE_RESET: u32 = 0x00000008;

pub const CRYPTO4XX_CRYPTO_PRIORITY: i32 = 300;
pub const PPC4XX_NUM_PD: u32 = 256;
pub const PPC4XX_LAST_PD: u32 = PPC4XX_NUM_PD - 1;
pub const PPC4XX_NUM_GD: u32 = 1024;
pub const PPC4XX_LAST_GD: u32 = PPC4XX_NUM_GD - 1;
pub const PPC4XX_NUM_SD: u32 = 256;
pub const PPC4XX_LAST_SD: u32 = PPC4XX_NUM_SD - 1;
pub const PPC4XX_SD_BUFFER_SIZE: u32 = 2048;

pub const PD_ENTRY_BUSY: u32 = 1 << 1;
pub const PD_ENTRY_INUSE: u32 = 1 << 0;
pub const PD_ENTRY_FREE: u32 = 0;
pub const ERING_WAS_FULL: u32 = 0xffff_ffff;

pub struct crypto4xx_device;

#[repr(C, packed)]
pub union shadow_sa_buf {
    pub sa: dynamic_sa_ctl,
    /// alloc 256 bytes which is enough for any kind of dynamic sa
    pub buf: [u8; 256],
}

#[repr(C)]
pub struct pd_uinfo {
    pub dev: *mut crypto4xx_device,
    pub state: u32,
    /// first gather discriptor used by this packet
    pub first_gd: u32,
    /// number of gather discriptor used by this packet
    pub num_gd: u32,
    /// first scatter discriptor used by this packet
    pub first_sd: u32,
    /// number of scatter discriptors used by this packet
    pub num_sd: u32,
    /// shadow sa
    pub sa_va: *mut dynamic_sa_ctl,
    /// state record for shadow sa
    pub sr_va: *mut sa_state_record,
    pub sr_pa: u32,
    pub dest_va: *mut scatterlist,
    /// base crypto request for this packet
    pub async_req: *mut crypto_async_request,
}

#[repr(C)]
pub struct crypto4xx_device {
    pub core_dev: *mut crypto4xx_core_device,
    pub ce_base: *mut core::ffi::c_void,
    pub trng_base: *mut core::ffi::c_void,
    /// base address of packet descriptor ring
    pub pdr: *mut ce_pd,
    /// physical address of pdr_base_register
    pub pdr_pa: dma_addr_t,
    /// gather descriptor ring
    pub gdr: *mut ce_gd,
    /// physical address of gdr_base_register
    pub gdr_pa: dma_addr_t,
    /// scatter descriptor ring
    pub sdr: *mut ce_sd,
    /// physical address of sdr_base_register
    pub sdr_pa: dma_addr_t,
    pub scatter_buffer_va: *mut core::ffi::c_void,
    pub scatter_buffer_pa: dma_addr_t,
    pub shadow_sa_pool: *mut shadow_sa_buf,
    pub shadow_sa_pool_pa: dma_addr_t,
    pub shadow_sr_pool: *mut sa_state_record,
    pub shadow_sr_pool_pa: dma_addr_t,
    pub pdr_tail: u32,
    pub pdr_head: u32,
    pub gdr_tail: u32,
    pub gdr_head: u32,
    pub sdr_tail: u32,
    pub sdr_head: u32,
    pub pdr_uinfo: *mut pd_uinfo,
    /// List of algorithm supported by this device
    pub alg_list: list_head,
    pub aead_ratelimit: ratelimit_state,
    pub is_revb: bool,
}

#[repr(C)]
pub struct crypto4xx_core_device {
    pub device: *mut device,
    pub ofdev: *mut platform_device,
    pub dev: *mut crypto4xx_device,
    pub trng: *mut hwrng,
    pub int_status: u32,
    pub irq: i32,
    pub tasklet: tasklet_struct,
    pub lock: spinlock_t,
}

#[repr(C)]
pub struct crypto4xx_ctx {
    pub dev: *mut crypto4xx_device,
    pub sa_in: *mut dynamic_sa_ctl,
    pub sa_out: *mut dynamic_sa_ctl,
    pub iv_nonce: __le32,
    pub sa_len: u32,
    #[repr(C)]
    pub sw_cipher: crypto4xx_ctx_sw_cipher,
}

#[repr(C)]
pub union crypto4xx_ctx_sw_cipher {
    pub cipher: *mut crypto_sync_skcipher,
    pub aead: *mut crypto_aead,
}

#[repr(C)]
pub struct crypto4xx_aead_reqctx {
    pub dst: [scatterlist; 2],
}

#[repr(C)]
pub struct crypto4xx_alg_common {
    pub type_: u32,
    #[repr(C)]
    pub u: crypto4xx_alg_common_u,
}

#[repr(C)]
pub union crypto4xx_alg_common_u {
    pub cipher: skcipher_alg,
    pub aead: aead_alg,
}

#[repr(C)]
pub struct crypto4xx_alg {
    pub entry: list_head,
    pub alg: crypto4xx_alg_common,
    pub dev: *mut crypto4xx_device,
}

// BUILD_PD_ACCESS is a compiler attribute when GCC supports access annotations.
// The attribute has no direct file-local Rust equivalent.

extern "C" {
    pub fn crypto4xx_alloc_sa(ctx: *mut crypto4xx_ctx, size: u32) -> i32;
    pub fn crypto4xx_free_sa(ctx: *mut crypto4xx_ctx);
    pub fn crypto4xx_build_pd(
        req: *mut crypto_async_request, ctx: *mut crypto4xx_ctx,
        src: *mut scatterlist, dst: *mut scatterlist, datalen: u32,
        iv: *const core::ffi::c_void, iv_len: u32,
        sa: *const dynamic_sa_ctl, sa_len: u32, assoclen: u32,
        dst_tmp: *mut scatterlist,
    ) -> i32;
    pub fn crypto4xx_setkey_aes_cbc(cipher: *mut crypto_skcipher, key: *const u8, keylen: u32) -> i32;
    pub fn crypto4xx_setkey_aes_ctr(cipher: *mut crypto_skcipher, key: *const u8, keylen: u32) -> i32;
    pub fn crypto4xx_setkey_aes_ecb(cipher: *mut crypto_skcipher, key: *const u8, keylen: u32) -> i32;
    pub fn crypto4xx_setkey_rfc3686(cipher: *mut crypto_skcipher, key: *const u8, keylen: u32) -> i32;
    pub fn crypto4xx_encrypt_ctr(req: *mut skcipher_request) -> i32;
    pub fn crypto4xx_decrypt_ctr(req: *mut skcipher_request) -> i32;
    pub fn crypto4xx_encrypt_iv_stream(req: *mut skcipher_request) -> i32;
    pub fn crypto4xx_decrypt_iv_stream(req: *mut skcipher_request) -> i32;
    pub fn crypto4xx_encrypt_iv_block(req: *mut skcipher_request) -> i32;
    pub fn crypto4xx_decrypt_iv_block(req: *mut skcipher_request) -> i32;
    pub fn crypto4xx_encrypt_noiv_block(req: *mut skcipher_request) -> i32;
    pub fn crypto4xx_decrypt_noiv_block(req: *mut skcipher_request) -> i32;
    pub fn crypto4xx_rfc3686_encrypt(req: *mut skcipher_request) -> i32;
    pub fn crypto4xx_rfc3686_decrypt(req: *mut skcipher_request) -> i32;

    pub fn crypto4xx_setauthsize_aead(ciper: *mut crypto_aead, authsize: u32) -> i32;
    pub fn crypto4xx_setkey_aes_ccm(cipher: *mut crypto_aead, key: *const u8, keylen: u32) -> i32;
    pub fn crypto4xx_encrypt_aes_ccm(req: *mut aead_request) -> i32;
    pub fn crypto4xx_decrypt_aes_ccm(req: *mut aead_request) -> i32;
    pub fn crypto4xx_setkey_aes_gcm(cipher: *mut crypto_aead, key: *const u8, keylen: u32) -> i32;
    pub fn crypto4xx_encrypt_aes_gcm(req: *mut aead_request) -> i32;
    pub fn crypto4xx_decrypt_aes_gcm(req: *mut aead_request) -> i32;
}

/// Note: Only use this function to copy items that is word aligned.
pub unsafe fn crypto4xx_memcpy_swab32(dst: *mut u32, buf: *const core::ffi::c_void, mut len: usize) {
    let mut dst = dst;
    let mut buf = buf as *const u8;
    while len >= 4 {
        let v = core::ptr::read_unaligned(buf as *const u32);
        core::ptr::write(dst, v.swap_bytes());
        dst = dst.add(1);
        buf = buf.add(4);
        len -= 4;
    }
    if len != 0 {
        let value = match len {
            3 => (*buf.add(2) as u32) << 16 | (*buf.add(1) as u32) << 8 | *buf as u32,
            2 => (*buf.add(1) as u32) << 8 | *buf as u32,
            1 => *buf as u32,
            _ => 0,
        };
        if len <= 3 { core::ptr::write(dst, value); }
    }
}

pub unsafe fn crypto4xx_memcpy_from_le32(dst: *mut u32, buf: *const core::ffi::c_void, len: usize) {
    crypto4xx_memcpy_swab32(dst, buf, len);
}

pub unsafe fn crypto4xx_memcpy_to_le32(dst: *mut __le32, buf: *const core::ffi::c_void, len: usize) {
    crypto4xx_memcpy_swab32(dst as *mut u32, buf, len);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
