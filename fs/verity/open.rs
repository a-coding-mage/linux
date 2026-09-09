// SPDX-License-Identifier: GPL-2.0
/*
 * Opening fs-verity files
 *
 * Copyright 2019 Google LLC
 */

static mut FSVERITY_INFO_CACHEP: *mut kmem_cache = core::ptr::null_mut();
static mut FSVERITY_INFO_HASH: rhashtable = unsafe { core::mem::zeroed() };

static FSVERITY_INFO_HASH_PARAMS: rhashtable_params = rhashtable_params {
    key_len: core::mem::size_of::<fsverity_info>(),
    key_offset: core::mem::offset_of!(fsverity_info, inode),
    head_offset: core::mem::offset_of!(fsverity_info, rhash_head),
    automatic_shrinking: true,
};

/**
 * fsverity_init_merkle_tree_params() - initialize Merkle tree parameters
 * @params: the parameters struct to initialize
 * @inode: the inode for which the Merkle tree is being built
 * @hash_algorithm: number of hash algorithm to use
 * @log_blocksize: log base 2 of block size to use
 * @salt: pointer to salt (optional)
 * @salt_size: size of salt, possibly 0
 *
 * Validate the hash algorithm and block size, then compute the tree topology
 * (num levels, num blocks in each level, etc.) and initialize @params.
 *
 * Return: 0 on success, -errno on failure
 */
unsafe fn fsverity_init_merkle_tree_params(
    params: *mut merkle_tree_params,
    inode: *const inode,
    hash_algorithm: u32,
    log_blocksize: u32,
    salt: *const u8,
    salt_size: usize,
) -> i32 {
    let hash_alg: *const fsverity_hash_alg;
    let mut err: i32;
    let mut blocks: u64;
    let mut blocks_in_level: [u64; FS_VERITY_MAX_LEVELS as usize] = [0; FS_VERITY_MAX_LEVELS as usize];
    let mut offset: u64;
    let mut level: i32;

    core::ptr::write_bytes(params as *mut u8, 0, core::mem::size_of::<merkle_tree_params>());

    hash_alg = fsverity_get_hash_alg(inode, hash_algorithm);
    if hash_alg.is_null() {
        return -EINVAL;
    }
    (*params).hash_alg = hash_alg;
    (*params).digest_size = (*hash_alg).digest_size;

    if salt_size != 0 {
        (*params).hashstate = fsverity_prepare_hash_state(hash_alg, salt, salt_size);
        if (*params).hashstate.is_null() {
            err = -ENOMEM;
            goto out_err;
        }
    }

    if log_blocksize < 10 || log_blocksize > PAGE_SHIFT || log_blocksize > (*inode).i_blkbits {
        fsverity_warn(inode, "Unsupported log_blocksize: %u", log_blocksize);
        err = -EINVAL;
        goto out_err;
    }
    (*params).log_blocksize = log_blocksize;
    (*params).block_size = 1u32 << log_blocksize;
    (*params).log_blocks_per_page = PAGE_SHIFT - log_blocksize;
    (*params).blocks_per_page = 1u32 << (*params).log_blocks_per_page;

    if WARN_ON_ONCE(!is_power_of_2((*params).digest_size)) {
        err = -EINVAL;
        goto out_err;
    }
    if (*params).block_size < 2 * (*params).digest_size {
        fsverity_warn(inode, "Merkle tree block size (%u) too small for hash algorithm \"%s\"", (*params).block_size, (*hash_alg).name);
        err = -EINVAL;
        goto out_err;
    }
    (*params).log_digestsize = ilog2((*params).digest_size);
    (*params).log_arity = log_blocksize - (*params).log_digestsize;
    (*params).hashes_per_block = 1u32 << (*params).log_arity;

    /* Compute the number of levels and the number of blocks in each level */
    blocks = (((*inode).i_size as u64) + (*params).block_size as u64 - 1) >> log_blocksize;
    while blocks > 1 {
        if (*params).num_levels >= FS_VERITY_MAX_LEVELS {
            fsverity_err(inode, "Too many levels in Merkle tree");
            err = -EFBIG;
            goto out_err;
        }
        blocks = (blocks + (*params).hashes_per_block as u64 - 1) >> (*params).log_arity;
        blocks_in_level[(*params).num_levels as usize] = blocks;
        (*params).num_levels += 1;
    }

    offset = 0;
    level = (*params).num_levels as i32 - 1;
    while level >= 0 {
        (*params).level_start[level as usize] = offset;
        offset += blocks_in_level[level as usize];
        level -= 1;
    }

    if (((*params).block_size != PAGE_SIZE && offset > (1u64 << 23)) || offset > ULONG_MAX as u64) {
        fsverity_err(inode, "Too many blocks in Merkle tree");
        err = -EFBIG;
        goto out_err;
    }

    fsverity_hash_block(params, page_address(ZERO_PAGE(0)), (*params).zero_digest.as_mut_ptr());
    (*params).tree_size = offset << log_blocksize;
    (*params).tree_pages = PAGE_ALIGN((*params).tree_size) >> PAGE_SHIFT;
    return 0;

out_err:
    kfree((*params).hashstate);
    core::ptr::write_bytes(params as *mut u8, 0, core::mem::size_of::<merkle_tree_params>());
    err
}

/* Compute the file digest by hashing the fsverity_descriptor excluding the
 * builtin signature and with the sig_size field set to 0. */
unsafe fn compute_file_digest(hash_alg: *const fsverity_hash_alg, desc: *mut fsverity_descriptor, file_digest: *mut u8) {
    let sig_size = (*desc).sig_size;
    (*desc).sig_size = 0;
    fsverity_hash_buffer(hash_alg, desc as *const _, core::mem::size_of::<fsverity_descriptor>(), file_digest);
    (*desc).sig_size = sig_size;
}

/* Create a new fsverity_info and check the optional signature. */
unsafe fn fsverity_create_info(inode: *mut inode, desc: *mut fsverity_descriptor) -> *mut fsverity_info {
    let vi = kmem_cache_zalloc(FSVERITY_INFO_CACHEP, GFP_KERNEL);
    if vi.is_null() { return ERR_PTR(-ENOMEM); }
    (*vi).inode = inode;
    let err = fsverity_init_merkle_tree_params(&mut (*vi).tree_params, inode, (*desc).hash_algorithm, (*desc).log_blocksize, (*desc).salt.as_ptr(), (*desc).salt_size as usize);
    if err != 0 { fsverity_err(inode, "Error %d initializing Merkle tree parameters", err); fsverity_free_info(vi); return ERR_PTR(err); }
    core::ptr::copy_nonoverlapping((*desc).root_hash.as_ptr(), (*vi).root_hash.as_mut_ptr(), (*vi).tree_params.digest_size as usize);
    compute_file_digest((*vi).tree_params.hash_alg, desc, (*vi).file_digest.as_mut_ptr());
    let err = fsverity_verify_signature(vi, (*desc).signature.as_ptr(), le32_to_cpu((*desc).sig_size));
    if err != 0 { fsverity_free_info(vi); return ERR_PTR(err); }
    if (*vi).tree_params.block_size != PAGE_SIZE {
        let num_bits = (*vi).tree_params.tree_pages << (*vi).tree_params.log_blocks_per_page;
        (*vi).hash_block_verified = kvcalloc(BITS_TO_LONGS(num_bits), core::mem::size_of::<usize>(), GFP_KERNEL);
        if (*vi).hash_block_verified.is_null() { fsverity_free_info(vi); return ERR_PTR(-ENOMEM); }
    }
    vi
}

unsafe fn fsverity_set_info(vi: *mut fsverity_info) -> i32 { rhashtable_lookup_insert_fast(&mut FSVERITY_INFO_HASH, &mut (*vi).rhash_head, FSVERITY_INFO_HASH_PARAMS) }

unsafe fn __fsverity_get_info(inode: *const inode) -> *mut fsverity_info { rhashtable_lookup_fast(&FSVERITY_INFO_HASH, &inode, FSVERITY_INFO_HASH_PARAMS) }

unsafe fn validate_fsverity_descriptor(inode: *mut inode, desc: *const fsverity_descriptor, desc_size: usize) -> bool {
    if desc_size < core::mem::size_of::<fsverity_descriptor>() { fsverity_err(inode, "Unrecognized descriptor size: %zu bytes", desc_size); return false; }
    if (*desc).version != 1 { fsverity_err(inode, "Unrecognized descriptor version: %u", (*desc).version); return false; }
    if memchr_inv((*desc).__reserved.as_ptr(), 0, (*desc).__reserved.len()).is_some() { fsverity_err(inode, "Reserved bits set in descriptor"); return false; }
    if (*desc).salt_size as usize > (*desc).salt.len() { fsverity_err(inode, "Invalid salt_size: %u", (*desc).salt_size); return false; }
    if le64_to_cpu((*desc).data_size) != (*inode).i_size as u64 { fsverity_err(inode, "Wrong data_size: %llu (desc) != %lld (inode)", le64_to_cpu((*desc).data_size), (*inode).i_size); return false; }
    if le32_to_cpu((*desc).sig_size) as usize > desc_size - core::mem::size_of::<fsverity_descriptor>() { fsverity_err(inode, "Signature overflows verity descriptor"); return false; }
    true
}

unsafe fn fsverity_get_descriptor(inode: *mut inode, desc_ret: *mut *mut fsverity_descriptor) -> i32 {
    let mut res = (*(*inode).i_sb).s_vop.get_verity_descriptor(inode, core::ptr::null_mut(), 0);
    if res < 0 { fsverity_err(inode, "Error %d getting verity descriptor size", res); return res; }
    if res > FS_VERITY_MAX_DESCRIPTOR_SIZE { fsverity_err(inode, "Verity descriptor is too large (%d bytes)", res); return -EMSGSIZE; }
    let desc = kmalloc(res as usize, GFP_KERNEL);
    if desc.is_null() { return -ENOMEM; }
    res = (*(*inode).i_sb).s_vop.get_verity_descriptor(inode, desc, res);
    if res < 0 { fsverity_err(inode, "Error %d reading verity descriptor", res); kfree(desc); return res; }
    if !validate_fsverity_descriptor(inode, desc, res as usize) { kfree(desc); return -EINVAL; }
    *desc_ret = desc;
    0
}

unsafe fn ensure_verity_info(inode: *mut inode) -> i32 {
    let mut vi = fsverity_get_info(inode);
    if !vi.is_null() { return 0; }
    let mut desc: *mut fsverity_descriptor = core::ptr::null_mut();
    let err = fsverity_get_descriptor(inode, &mut desc);
    if err != 0 { return err; }
    vi = fsverity_create_info(inode, desc);
    if IS_ERR(vi) { let e = PTR_ERR(vi); kfree(desc); return e; }
    let found = rhashtable_lookup_get_insert_fast(&mut FSVERITY_INFO_HASH, &mut (*vi).rhash_head, FSVERITY_INFO_HASH_PARAMS);
    let err = if !found.is_null() { fsverity_free_info(vi); if IS_ERR(found) { PTR_ERR(found) } else { 0 } } else { 0 };
    kfree(desc);
    err
}

unsafe fn __fsverity_file_open(inode: *mut inode, filp: *mut file) -> i32 {
    if (*filp).f_mode & FMODE_WRITE != 0 { return -EPERM; }
    ensure_verity_info(inode)
}

unsafe fn fsverity_free_info(vi: *mut fsverity_info) { kfree((*vi).tree_params.hashstate); kvfree((*vi).hash_block_verified); kmem_cache_free(FSVERITY_INFO_CACHEP, vi); }

unsafe fn fsverity_remove_info(vi: *mut fsverity_info) { rhashtable_remove_fast(&mut FSVERITY_INFO_HASH, &mut (*vi).rhash_head, FSVERITY_INFO_HASH_PARAMS); fsverity_free_info(vi); }

unsafe fn fsverity_cleanup_inode(inode: *mut inode) { let vi = fsverity_get_info(inode); if !vi.is_null() { fsverity_remove_info(vi); } }

unsafe fn fsverity_init_info_cache() {
    if rhashtable_init(&mut FSVERITY_INFO_HASH, &FSVERITY_INFO_HASH_PARAMS) != 0 { panic!("failed to initialize fsverity hash\n"); }
    FSVERITY_INFO_CACHEP = KMEM_CACHE_USERCOPY(fsverity_info, SLAB_RECLAIM_ACCOUNT | SLAB_PANIC, file_digest);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
