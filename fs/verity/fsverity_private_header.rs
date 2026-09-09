/* SPDX-License-Identifier: GPL-2.0 */
/*
 * fs-verity: read-only file-based authenticity protection
 *
 * Copyright 2019 Google LLC
 */

/* pr_fmt(fmt) expands to "fs-verity: " fmt. */

/* Implementation limit: maximum depth of the Merkle tree. */
pub const FS_VERITY_MAX_LEVELS: usize = 8;

/* A hash algorithm supported by fs-verity. */
#[repr(C)]
pub struct fsverity_hash_alg {
    pub name: *const ::core::ffi::c_char,
    pub digest_size: ::core::ffi::c_uint,
    pub block_size: ::core::ffi::c_uint,
    pub algo_id: hash_algo,
}

#[repr(C)]
pub union fsverity_hash_ctx {
    pub sha256: sha256_ctx,
    pub sha512: sha512_ctx,
}

/* Merkle tree parameters: hash algorithm, initial hash state, and topology. */
#[repr(C)]
pub struct merkle_tree_params {
    pub hash_alg: *const fsverity_hash_alg,
    pub hashstate: *const fsverity_hash_ctx,
    pub digest_size: ::core::ffi::c_uint,
    pub block_size: ::core::ffi::c_uint,
    pub hashes_per_block: ::core::ffi::c_uint,
    pub blocks_per_page: ::core::ffi::c_uint,
    pub log_digestsize: u8,
    pub log_blocksize: u8,
    pub log_arity: u8,
    pub log_blocks_per_page: u8,
    pub num_levels: ::core::ffi::c_uint,
    pub tree_size: u64,
    pub tree_pages: ::core::ffi::c_ulong,
    pub zero_digest: [u8; FS_VERITY_MAX_DIGEST_SIZE as usize],
    pub level_start: [::core::ffi::c_ulong; FS_VERITY_MAX_LEVELS],
}

/* Cached verity metadata for an inode. */
#[repr(C)]
pub struct fsverity_info {
    pub rhash_head: rhash_head,
    pub tree_params: merkle_tree_params,
    pub root_hash: [u8; FS_VERITY_MAX_DIGEST_SIZE as usize],
    pub file_digest: [u8; FS_VERITY_MAX_DIGEST_SIZE as usize],
    pub inode: *mut inode,
    pub hash_block_verified: *mut ::core::ffi::c_ulong,
}

pub const FS_VERITY_MAX_SIGNATURE_SIZE: usize =
    FS_VERITY_MAX_DESCRIPTOR_SIZE as usize - core::mem::size_of::<fsverity_descriptor>();

extern "C" {
    pub static fsverity_hash_algs: fsverity_hash_alg;

    pub fn fsverity_get_hash_alg(
        inode: *const inode,
        num: ::core::ffi::c_uint,
    ) -> *const fsverity_hash_alg;
    pub fn fsverity_prepare_hash_state(
        alg: *const fsverity_hash_alg,
        salt: *const u8,
        salt_size: usize,
    ) -> *mut fsverity_hash_ctx;
    pub fn fsverity_hash_block(
        params: *const merkle_tree_params,
        data: *const ::core::ffi::c_void,
        out: *mut u8,
    );
    pub fn fsverity_hash_buffer(
        alg: *const fsverity_hash_alg,
        data: *const ::core::ffi::c_void,
        size: usize,
        out: *mut u8,
    );
    pub fn fsverity_check_hash_algs();

    pub fn fsverity_msg(
        inode: *const inode,
        level: *const ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...,
    );

    pub fn fsverity_init_merkle_tree_params(
        params: *mut merkle_tree_params,
        inode: *const inode,
        hash_algorithm: ::core::ffi::c_uint,
        log_blocksize: ::core::ffi::c_uint,
        salt: *const u8,
        salt_size: usize,
    ) -> ::core::ffi::c_int;
    pub fn fsverity_create_info(
        inode: *mut inode,
        desc: *mut fsverity_descriptor,
    ) -> *mut fsverity_info;
    pub fn fsverity_set_info(vi: *mut fsverity_info) -> ::core::ffi::c_int;
    pub fn fsverity_free_info(vi: *mut fsverity_info);
    pub fn fsverity_remove_info(vi: *mut fsverity_info);
    pub fn fsverity_get_descriptor(
        inode: *mut inode,
        desc_ret: *mut *mut fsverity_descriptor,
    ) -> ::core::ffi::c_int;
    pub fn fsverity_init_info_cache();

    pub fn fsverity_verify_signature(
        vi: *const fsverity_info,
        signature: *const u8,
        sig_size: usize,
    ) -> ::core::ffi::c_int;
    pub fn fsverity_init_signature();
    pub fn fsverity_init_workqueue();
}

/* fsverity_warn(inode, fmt, ...) and fsverity_err(inode, fmt, ...) are
 * forwarding macros to fsverity_msg with KERN_WARNING and KERN_ERR. */

#[cfg(not(CONFIG_BPF_SYSCALL))]
#[inline]
pub fn fsverity_init_bpf() {}

#[cfg(not(CONFIG_FS_VERITY_BUILTIN_SIGNATURES))]
#[inline]
pub unsafe fn fsverity_verify_signature_stub(
    _vi: *const fsverity_info,
    _signature: *const u8,
    _sig_size: usize,
) -> ::core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_FS_VERITY_BUILTIN_SIGNATURES))]
#[inline]
pub fn fsverity_init_signature_stub() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
