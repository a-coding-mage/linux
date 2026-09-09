/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2010-2014, The Linux Foundation. All rights reserved.
 */

// Dependencies supplied by the surrounding crypto and QCE code are referenced
// here but are not defined by this header translation.

pub const QCE_SHA_MAX_BLOCKSIZE: usize = SHA256_BLOCK_SIZE as usize;
pub const QCE_SHA_MAX_DIGESTSIZE: usize = SHA256_DIGEST_SIZE as usize;

#[repr(C)]
pub struct qce_sha_ctx {
    pub authkey: [u8; QCE_SHA_MAX_BLOCKSIZE],
}

/**
 * struct qce_sha_reqctx - holds private ahash objects per request
 * @buf: used during update, import and export
 * @tmpbuf: buffer for internal use
 * @digest: calculated digest buffer
 * @buflen: length of the buffer
 * @flags: operation flags
 * @src_orig: original request sg list
 * @nbytes_orig: original request number of bytes
 * @src_nents: source number of entries
 * @byte_count: byte count
 * @count: save count in states during update, import and export
 * @first_blk: is it the first block
 * @last_blk: is it the last block
 * @sg: used to chain sg lists
 * @authkey: pointer to auth key in sha ctx
 * @authklen: auth key length
 * @result_sg: scatterlist used for result buffer
 */
#[repr(C)]
pub struct qce_sha_reqctx {
    pub buf: [u8; QCE_SHA_MAX_BLOCKSIZE],
    pub tmpbuf: [u8; QCE_SHA_MAX_BLOCKSIZE],
    pub digest: [u8; QCE_SHA_MAX_DIGESTSIZE],
    pub buflen: core::ffi::c_uint,
    pub flags: core::ffi::c_ulong,
    pub src_orig: *mut scatterlist,
    pub nbytes_orig: core::ffi::c_uint,
    pub src_nents: core::ffi::c_int,
    pub byte_count: [u32; 2],
    pub count: u64,
    pub first_blk: bool,
    pub last_blk: bool,
    pub sg: [scatterlist; 2],
    pub authkey: *mut u8,
    pub authklen: core::ffi::c_uint,
    pub result_sg: scatterlist,
}

#[inline]
pub unsafe fn to_ahash_tmpl(tfm: *mut crypto_tfm) -> *mut qce_alg_template {
    let ahash: *mut crypto_ahash = __crypto_ahash_cast(tfm);
    let alg: *mut ahash_alg = container_of!(
        crypto_hash_alg_common(ahash),
        ahash_alg,
        halg
    );

    container_of!(alg, qce_alg_template, alg.ahash)
}

extern "C" {
    pub static ahash_ops: qce_algo_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
