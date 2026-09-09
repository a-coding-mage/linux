// SPDX-License-Identifier: GPL-2.0
/*
 * fs-verity hash algorithms
 *
 * Copyright 2019 Google LLC
 */

// Dependency declarations supplied by fsverity_private.h and other files are
// intentionally left external to this translation unit.

/* The hash algorithms supported by fs-verity */
pub static fsverity_hash_algs: [fsverity_hash_alg; 2] = [
    fsverity_hash_alg {
        name: b"sha256\0".as_ptr() as *const i8,
        digest_size: SHA256_DIGEST_SIZE,
        block_size: SHA256_BLOCK_SIZE,
        algo_id: HASH_ALGO_SHA256,
    },
    fsverity_hash_alg {
        name: b"sha512\0".as_ptr() as *const i8,
        digest_size: SHA512_DIGEST_SIZE,
        block_size: SHA512_BLOCK_SIZE,
        algo_id: HASH_ALGO_SHA512,
    },
];

/**
 * fsverity_get_hash_alg() - get a hash algorithm by number
 * @inode: optional inode for logging purposes
 * @num: the hash algorithm number
 *
 * Get the struct fsverity_hash_alg for the given hash algorithm number.
 *
 * Return: pointer to the hash alg if it's known, otherwise NULL.
 */
pub unsafe fn fsverity_get_hash_alg(
    inode: *const inode,
    num: u32,
) -> *const fsverity_hash_alg {
    if num as usize >= fsverity_hash_algs.len()
        || (*fsverity_hash_algs.as_ptr().add(num as usize)).name.is_null()
    {
        fsverity_warn(inode, b"Unknown hash algorithm number: %u\0".as_ptr() as *const i8, num);
        return core::ptr::null();
    }
    fsverity_hash_algs.as_ptr().add(num as usize)
}

/**
 * fsverity_prepare_hash_state() - precompute the initial hash state
 * @alg: hash algorithm
 * @salt: a salt which is to be prepended to all data to be hashed
 * @salt_size: salt size in bytes
 *
 * Return: the kmalloc()'ed initial hash state, or NULL if out of memory.
 */
pub unsafe fn fsverity_prepare_hash_state(
    alg: *const fsverity_hash_alg,
    salt: *const u8,
    salt_size: usize,
) -> *mut fsverity_hash_ctx {
    let mut padded_salt: *mut u8 = core::ptr::null_mut();
    let padded_salt_size: usize;
    let mut ctx: fsverity_hash_ctx = core::mem::zeroed();
    let mut res: *mut core::ffi::c_void = core::ptr::null_mut();

    padded_salt_size = round_up(salt_size, (*alg).block_size);
    padded_salt = kzalloc(padded_salt_size, GFP_KERNEL);
    if padded_salt.is_null() {
        return core::ptr::null_mut();
    }
    core::ptr::copy_nonoverlapping(salt, padded_salt, salt_size);

    match (*alg).algo_id {
        HASH_ALGO_SHA256 => {
            sha256_init(&mut ctx.sha256);
            sha256_update(&mut ctx.sha256, padded_salt, padded_salt_size);
            res = kmemdup(&ctx.sha256 as *const _ as *const core::ffi::c_void,
                          core::mem::size_of_val(&ctx.sha256), GFP_KERNEL);
        }
        HASH_ALGO_SHA512 => {
            sha512_init(&mut ctx.sha512);
            sha512_update(&mut ctx.sha512, padded_salt, padded_salt_size);
            res = kmemdup(&ctx.sha512 as *const _ as *const core::ffi::c_void,
                          core::mem::size_of_val(&ctx.sha512), GFP_KERNEL);
        }
        _ => WARN_ON_ONCE(1),
    }
    kfree(padded_salt as *mut core::ffi::c_void);
    res as *mut fsverity_hash_ctx
}

/** Hash a single data or hash block. */
pub unsafe fn fsverity_hash_block(
    params: *const merkle_tree_params,
    data: *const core::ffi::c_void,
    out: *mut u8,
) {
    let mut ctx: fsverity_hash_ctx = core::mem::zeroed();
    if (*params).hashstate.is_null() {
        fsverity_hash_buffer((*params).hash_alg, data, (*params).block_size, out);
        return;
    }
    match (*(*params).hash_alg).algo_id {
        HASH_ALGO_SHA256 => {
            ctx.sha256 = (*(*params).hashstate).sha256;
            sha256_update(&mut ctx.sha256, data, (*params).block_size);
            sha256_final(&mut ctx.sha256, out);
        }
        HASH_ALGO_SHA512 => {
            ctx.sha512 = (*(*params).hashstate).sha512;
            sha512_update(&mut ctx.sha512, data, (*params).block_size);
            sha512_final(&mut ctx.sha512, out);
        }
        _ => BUG(),
    }
}

/** Hash some data. */
pub unsafe fn fsverity_hash_buffer(
    alg: *const fsverity_hash_alg,
    data: *const core::ffi::c_void,
    size: usize,
    out: *mut u8,
) {
    match (*alg).algo_id {
        HASH_ALGO_SHA256 => sha256(data, size, out),
        HASH_ALGO_SHA512 => sha512(data, size, out),
        _ => BUG(),
    }
}

pub unsafe fn fsverity_check_hash_algs() {
    let mut i = 0usize;
    while i < fsverity_hash_algs.len() {
        let alg = &fsverity_hash_algs[i];
        if alg.name.is_null() {
            i += 1;
            continue;
        }
        BUG_ON(i == 0);
        BUG_ON(alg.digest_size > FS_VERITY_MAX_DIGEST_SIZE);
        BUG_ON(!is_power_of_2(alg.digest_size));
        BUG_ON(!is_power_of_2(alg.block_size));
        BUG_ON(alg.algo_id == 0);
        BUG_ON(alg.digest_size != hash_digest_size[alg.algo_id as usize]);
        i += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
