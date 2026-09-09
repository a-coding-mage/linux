/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020 Pengutronix, Ahmad Fatoum <kernel@pengutronix.de>
 * Copyright 2024-2025 NXP
 */

// C header guard: __CAAM_BLOB_GEN
// Dependencies supplied by the surrounding translation unit provide `u8`,
// `u16`, `EINVAL`, and the C ABI declarations referenced below.

pub const CAAM_BLOB_KEYMOD_LENGTH: usize = 16;
pub const CAAM_BLOB_OVERHEAD: usize = 32 + 16;
pub const CAAM_BLOB_MAX_LEN: usize = 4096;
pub const CAAM_ENC_ALGO_CCM: u32 = 0x1;
pub const CAAM_ENC_ALGO_ECB: u32 = 0x2;
pub const CAAM_NONCE_SIZE: usize = 6;
pub const CAAM_ICV_SIZE: usize = 6;
pub const CAAM_CCM_OVERHEAD: usize = CAAM_NONCE_SIZE + CAAM_ICV_SIZE;

#[repr(C)]
pub struct caam_blob_priv {
    _private: [u8; 0],
}

/**
 * struct caam_pkey_info - information for CAAM protected key
 * @is_pkey:          flag to identify, if the key is protected.
 * @key_enc_algo:    identifies the algorithm, ccm or ecb
 * @plain_key_sz:    size of plain key.
 * @key_buf:         contains key data
 */
#[repr(C, packed)]
pub struct caam_pkey_info {
    pub is_pkey: u8,
    pub key_enc_algo: u8,
    pub plain_key_sz: u16,
    pub key_buf: [u8; 0],
}

/* sizeof struct caam_pkey_info */
pub const CAAM_PKEY_HEADER: usize = 4;

/**
 * struct caam_blob_info - information for CAAM blobbing
 * @pkey_info:     pointer to keep protected key information
 * @input:         pointer to input buffer (must be DMAable)
 * @input_len:     length of @input buffer in bytes.
 * @output:        pointer to output buffer (must be DMAable)
 * @output_len:    length of @output buffer in bytes.
 * @key_mod:       key modifier
 * @key_mod_len:   length of @key_mod in bytes.
 *                 May not exceed %CAAM_BLOB_KEYMOD_LENGTH
 */
#[repr(C)]
pub struct caam_blob_info {
    pub pkey_info: caam_pkey_info,
    pub input: *mut core::ffi::c_void,
    pub input_len: usize,
    pub output: *mut core::ffi::c_void,
    pub output_len: usize,
    pub key_mod: *const core::ffi::c_void,
    pub key_mod_len: usize,
}

/**
 * caam_blob_gen_init - initialize blob generation
 * Return: pointer to new &struct caam_blob_priv instance on success
 * and ``ERR_PTR(-ENODEV)`` if CAAM has no hardware blobbing support
 * or no job ring could be allocated.
 */
unsafe extern "C" {
    pub fn caam_blob_gen_init() -> *mut caam_blob_priv;

    /**
     * caam_blob_gen_exit - free blob generation resources
     * @priv: instance returned by caam_blob_gen_init()
     */
    pub fn caam_blob_gen_exit(priv_: *mut caam_blob_priv);

    /**
     * caam_process_blob - encapsulate or decapsulate blob
     * @priv:   instance returned by caam_blob_gen_init()
     * @info:   pointer to blobbing info describing key, blob and
     *          key modifier buffers.
     * @encap:  true for encapsulation, false for decapsulation
     * Return: %0 and sets ``info->output_len`` on success and a negative
     * error code otherwise.
     */
    pub fn caam_process_blob(
        priv_: *mut caam_blob_priv,
        info: *mut caam_blob_info,
        encap: bool,
    ) -> i32;
}

/**
 * caam_encap_blob - encapsulate blob
 * @priv:  instance returned by caam_blob_gen_init()
 * @info:  pointer to blobbing info describing input key,
 *         output blob and key modifier buffers.
 */
#[inline]
pub unsafe fn caam_encap_blob(priv_: *mut caam_blob_priv, info: *mut caam_blob_info) -> i32 {
    if (*info).output_len < (*info).input_len + CAAM_BLOB_OVERHEAD {
        return -(EINVAL as i32);
    }

    caam_process_blob(priv_, info, true)
}

/**
 * caam_decap_blob - decapsulate blob
 * @priv:  instance returned by caam_blob_gen_init()
 * @info:  pointer to blobbing info describing output key,
 *         input blob and key modifier buffers.
 */
#[inline]
pub unsafe fn caam_decap_blob(priv_: *mut caam_blob_priv, info: *mut caam_blob_info) -> i32 {
    if (*info).input_len < CAAM_BLOB_OVERHEAD
        || (*info).output_len < (*info).input_len - CAAM_BLOB_OVERHEAD
    {
        return -(EINVAL as i32);
    }

    caam_process_blob(priv_, info, false)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
