/* SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause) */
/*
 * Shared descriptors for ahash algorithms
 *
 * Copyright 2017 NXP
 */

/* Dependency-provided C constants and types: CAAM_CMD_SZ, OP_ALG_ALGSEL_MASK,
 * OP_ALG_AAI_MASK, OP_ALG_ALGSEL_AES, OP_ALG_AAI_XCBC_MAC, and alginfo. */

/* length of descriptors text */
pub const DESC_AHASH_BASE: usize = 3 * CAAM_CMD_SZ;
pub const DESC_AHASH_UPDATE_LEN: usize = 6 * CAAM_CMD_SZ;
pub const DESC_AHASH_UPDATE_FIRST_LEN: usize = DESC_AHASH_BASE + 4 * CAAM_CMD_SZ;
pub const DESC_AHASH_FINAL_LEN: usize = DESC_AHASH_BASE + 5 * CAAM_CMD_SZ;
pub const DESC_AHASH_DIGEST_LEN: usize = DESC_AHASH_BASE + 4 * CAAM_CMD_SZ;

pub fn is_xcbc_aes(algtype: u32) -> bool {
    (algtype & (OP_ALG_ALGSEL_MASK | OP_ALG_AAI_MASK))
        == (OP_ALG_ALGSEL_AES | OP_ALG_AAI_XCBC_MAC)
}

extern "C" {
    pub fn cnstr_shdsc_ahash(
        desc: *mut u32,
        adata: *mut alginfo,
        state: u32,
        digestsize: i32,
        ctx_len: i32,
        import_ctx: bool,
        era: i32,
    );

    pub fn cnstr_shdsc_sk_hash(
        desc: *mut u32,
        adata: *mut alginfo,
        state: u32,
        digestsize: i32,
        ctx_len: i32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
