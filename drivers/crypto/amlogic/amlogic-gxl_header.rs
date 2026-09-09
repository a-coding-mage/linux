/* SPDX-License-Identifier: GPL-2.0 */
/*
 * amlogic.h - hardware cryptographic offloader for Amlogic SoC
 *
 * Copyright (C) 2018-2019 Corentin LABBE <clabbe@baylibre.com>
 */
// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

pub const MODE_KEY: u32 = 1;
pub const MODE_AES_128: u32 = 0x8;
pub const MODE_AES_192: u32 = 0x9;
pub const MODE_AES_256: u32 = 0xa;

pub const MESON_DECRYPT: u32 = 0;
pub const MESON_ENCRYPT: u32 = 1;

pub const MESON_OPMODE_ECB: u32 = 0;
pub const MESON_OPMODE_CBC: u32 = 1;

pub const MAXFLOW: usize = 2;
pub const MAXDESC: usize = 64;

pub const DESC_LAST: u32 = 1u32 << 18;
pub const DESC_ENCRYPTION: u32 = 1u32 << 28;
pub const DESC_OWN: u32 = 1u32 << 31;

/*
 * struct meson_desc - Descriptor for DMA operations
 * Note that without datasheet, some are unknown
 * @t_status: Descriptor of the cipher operation (see description below)
 * @t_src: Physical address of data to read
 * @t_dst: Physical address of data to write
 * t_status is segmented like this:
 * @len: 0-16 length of data to operate
 * @irq: 17 Ignored by hardware
 * @eoc: 18 End means the descriptor is the last
 * @loop: 19 Unknown
 * @mode: 20-23 Type of algorithm (AES, SHA)
 * @begin: 24 Unknown
 * @end: 25 Unknown
 * @op_mode: 26-27 Blockmode (CBC, ECB)
 * @enc: 28 0 means decryption, 1 is for encryption
 * @block: 29 Unknown
 * @error: 30 Unknown
 * @owner: 31 owner of the descriptor, 1 own by HW
 */
#[repr(C)]
pub struct meson_desc {
    pub t_status: u32,
    pub t_src: u32,
    pub t_dst: u32,
}

/* struct meson_flow - Information used by each flow */
#[repr(C)]
pub struct meson_flow {
    pub engine: *mut crypto_engine,
    pub complete: completion,
    pub status: ::core::ffi::c_int,
    pub keylen: u32,
    pub t_phy: dma_addr_t,
    pub tl: *mut meson_desc,
    #[cfg(CONFIG_CRYPTO_DEV_AMLOGIC_GXL_DEBUG)]
    pub stat_req: ::core::ffi::c_ulong,
}

/* struct meson_dev - main container for all this driver information */
#[repr(C)]
pub struct meson_dev {
    pub base: *mut ::core::ffi::c_void,
    pub busclk: *mut clk,
    pub dev: *mut device,
    pub chanlist: *mut meson_flow,
    pub flow: atomic_t,
    pub irqs: [::core::ffi::c_int; MAXFLOW],
    #[cfg(CONFIG_CRYPTO_DEV_AMLOGIC_GXL_DEBUG)]
    pub dbgfs_dir: *mut dentry,
}

/* struct meson_cipher_req_ctx - context for a skcipher request */
#[repr(C)]
pub struct meson_cipher_req_ctx {
    pub op_dir: u32,
    pub flow: ::core::ffi::c_int,
    pub fallback_req: skcipher_request, // keep at the end
}

/* struct meson_cipher_tfm_ctx - context for a skcipher TFM */
#[repr(C)]
pub struct meson_cipher_tfm_ctx {
    pub key: *mut u32,
    pub keylen: u32,
    pub keymode: u32,
    pub mc: *mut meson_dev,
    pub fallback_tfm: *mut crypto_skcipher,
}

/* struct meson_alg_template - crypto_alg template */
#[repr(C)]
pub union meson_alg_template_alg {
    pub skcipher: skcipher_engine_alg,
}

#[repr(C)]
pub struct meson_alg_template {
    pub type_: u32,
    pub blockmode: u32,
    pub alg: meson_alg_template_alg,
    pub mc: *mut meson_dev,
    #[cfg(CONFIG_CRYPTO_DEV_AMLOGIC_GXL_DEBUG)]
    pub stat_req: ::core::ffi::c_ulong,
    #[cfg(CONFIG_CRYPTO_DEV_AMLOGIC_GXL_DEBUG)]
    pub stat_fb: ::core::ffi::c_ulong,
}

extern "C" {
    pub fn meson_aes_setkey(
        tfm: *mut crypto_skcipher,
        key: *const u8,
        keylen: u32,
    ) -> ::core::ffi::c_int;
    pub fn meson_cipher_init(tfm: *mut crypto_tfm) -> ::core::ffi::c_int;
    pub fn meson_cipher_exit(tfm: *mut crypto_tfm);
    pub fn meson_skdecrypt(areq: *mut skcipher_request) -> ::core::ffi::c_int;
    pub fn meson_skencrypt(areq: *mut skcipher_request) -> ::core::ffi::c_int;
    pub fn meson_handle_cipher_request(
        engine: *mut crypto_engine,
        areq: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
