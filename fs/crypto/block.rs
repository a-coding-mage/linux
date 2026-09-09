// SPDX-License-Identifier: GPL-2.0
/*
 * File contents en/decryption on block-based filesystems
 *
 * Copyright 2019 Google LLC
 */

/*
 * This file implements fscrypt's file contents en/decryption using blk-crypto
 * (Documentation/block/inline-encryption.rst).  fscrypt assigns a bio_crypt_ctx
 * with a key and IV to each bio, and the block layer does the en/decryption.
 *
 * This file's exported functions are called only by block-based filesystems.
 */

unsafe fn fscrypt_get_devices(
    sb: *mut super_block,
    devs: *mut *mut block_device,
) -> libc::c_uint {
    if unsafe { (*(*sb).s_cop).get_devices }.is_some() {
        return unsafe { ((*(*sb).s_cop).get_devices.unwrap())(sb, devs) };
    }
    unsafe { *devs = (*sb).s_bdev };
    1
}

unsafe fn fscrypt_get_dun_bytes(ci: *const fscrypt_inode_info) -> libc::c_uint {
    let sb = unsafe { (*(*ci).ci_inode).i_sb };
    let flags = unsafe { fscrypt_policy_flags(&(*ci).ci_policy) };
    let dun_bits: libc::c_int;

    if flags & FSCRYPT_POLICY_FLAG_DIRECT_KEY != 0 {
        return core::mem::offset_of!(fscrypt_iv, nonce) as libc::c_uint;
    }
    if flags & FSCRYPT_POLICY_FLAG_IV_INO_LBLK_64 != 0 {
        return core::mem::size_of::<__le64>() as libc::c_uint;
    }
    if flags & FSCRYPT_POLICY_FLAG_IV_INO_LBLK_32 != 0 {
        return core::mem::size_of::<__le32>() as libc::c_uint;
    }

    // Default case: IVs are just the file data unit index
    dun_bits = unsafe { fscrypt_max_file_dun_bits(sb, (*ci).ci_data_unit_bits) };
    ((dun_bits + 7) / 8) as libc::c_uint
}

unsafe fn fscrypt_log_blk_crypto_impl(
    mode: *mut fscrypt_mode,
    dev: *mut block_device,
    blk_key: *const blk_crypto_key,
) {
    if unsafe { blk_crypto_config_supported_natively(dev, &(*blk_key).crypto_cfg) } {
        if unsafe { xchg(&mut (*mode).logged_blk_crypto_native, 1) } == 0 {
            unsafe { pr_info!("fscrypt: %s using blk-crypto (native)\n", (*mode).friendly_name) };
        }
    } else if unsafe { xchg(&mut (*mode).logged_blk_crypto_fallback, 1) } == 0 {
        unsafe { pr_info!("fscrypt: %s using blk-crypto-fallback\n", (*mode).friendly_name) };
    }
}

pub unsafe fn fscrypt_prepare_inline_crypt_key(
    prep_key: *mut fscrypt_prepared_key,
    key_bytes: *const u8,
    key_size: usize,
    is_hw_wrapped: bool,
    ci: *const fscrypt_inode_info,
) -> libc::c_int {
    let inode = unsafe { (*ci).ci_inode };
    let sb = unsafe { (*inode).i_sb };
    let inlinecrypt = unsafe { (*sb).s_flags & SB_INLINECRYPT != 0 };
    let mode = unsafe { (*ci).ci_mode };
    let key_type = if is_hw_wrapped { BLK_CRYPTO_KEY_TYPE_HW_WRAPPED } else { BLK_CRYPTO_KEY_TYPE_RAW };
    let mut blk_key: *mut blk_crypto_key;
    let mut devs: [*mut block_device; FSCRYPT_MAX_DEVICES] = [core::ptr::null_mut(); FSCRYPT_MAX_DEVICES];
    let mut num_devs: libc::c_uint;
    let mut err: libc::c_int;

    if is_hw_wrapped && !inlinecrypt {
        unsafe { fscrypt_err!(inode, "Hardware-wrapped keys require inline encryption (-o inlinecrypt)") };
        return -EINVAL;
    }

    blk_key = unsafe { kmalloc_obj::<blk_crypto_key>() };
    if blk_key.is_null() { return -ENOMEM; }

    err = unsafe { blk_crypto_init_key(blk_key, key_bytes, key_size, key_type, (*mode).blk_crypto_mode,
        fscrypt_get_dun_bytes(ci), 1u32 << (*ci).ci_data_unit_bits,
        if inlinecrypt { BLK_CRYPTO_CFG_ALLOW_HW } else { 0 }) };
    if err != 0 {
        unsafe { fscrypt_err!(inode, "Error %d initializing blk-crypto key", err) };
        unsafe { kfree_sensitive(blk_key); }
        return err;
    }

    num_devs = unsafe { fscrypt_get_devices(sb, devs.as_mut_ptr()) };
    for i in 0..num_devs as usize {
        err = unsafe { blk_crypto_start_using_key(devs[i], blk_key) };
        if err != 0 { break; }
        unsafe { fscrypt_log_blk_crypto_impl(mode, devs[i], blk_key) };
    }
    if err != 0 {
        if err == -EOPNOTSUPP && is_hw_wrapped {
            unsafe { fscrypt_err!(inode, "Hardware-wrapped key required, but no suitable inline encryption capabilities are available") };
        } else {
            unsafe { fscrypt_err!(inode, "Error %d starting to use blk-crypto", err) };
        }
        unsafe { kfree_sensitive(blk_key); }
        return err;
    }
    unsafe { (*prep_key).blk_key = blk_key; }
    return 0;
}

pub unsafe fn fscrypt_destroy_inline_crypt_key(sb: *mut super_block, prep_key: *mut fscrypt_prepared_key) {
    let blk_key = unsafe { (*prep_key).blk_key };
    if blk_key.is_null() { return; }
    let mut devs: [*mut block_device; FSCRYPT_MAX_DEVICES] = [core::ptr::null_mut(); FSCRYPT_MAX_DEVICES];
    let num_devs = unsafe { fscrypt_get_devices(sb, devs.as_mut_ptr()) };
    for i in 0..num_devs as usize { unsafe { blk_crypto_evict_key(devs[i], blk_key); } }
    unsafe { kfree_sensitive(blk_key); }
}

pub unsafe fn fscrypt_derive_sw_secret(sb: *mut super_block, wrapped_key: *const u8, wrapped_key_size: usize, sw_secret: *mut u8) -> libc::c_int {
    if unsafe { (*sb).s_flags & SB_INLINECRYPT == 0 } {
        unsafe { fscrypt_warn!(core::ptr::null_mut(), "%s: filesystem not mounted with inlinecrypt\n", (*sb).s_id) };
        return -EOPNOTSUPP;
    }
    let err = unsafe { blk_crypto_derive_sw_secret((*sb).s_bdev, wrapped_key, wrapped_key_size, sw_secret) };
    if err == -EOPNOTSUPP { unsafe { fscrypt_warn!(core::ptr::null_mut(), "%s: block device doesn't support hardware-wrapped keys\n", (*sb).s_id) }; }
    err
}

unsafe fn fscrypt_generate_dun(ci: *const fscrypt_inode_info, pos: loff_t, dun: *mut u64) {
    let mut iv: fscrypt_iv = core::mem::zeroed();
    unsafe { fscrypt_generate_iv(&mut iv, pos >> (*ci).ci_data_unit_bits, ci); }
    unsafe { core::ptr::write_bytes(dun, 0, BLK_CRYPTO_MAX_IV_SIZE); }
    for i in 0..(unsafe { (*(*ci).ci_mode).ivsize } / core::mem::size_of::<u64>()) {
        unsafe { *dun.add(i) = le64_to_cpu(iv.dun[i]); }
    }
}

pub unsafe fn fscrypt_set_bio_crypt_ctx(bio: *mut bio, inode: *const inode, pos: loff_t, gfp_mask: gfp_t) {
    if !unsafe { fscrypt_needs_contents_encryption(inode) } { return; }
    let ci = unsafe { fscrypt_get_inode_info_raw(inode) };
    let mut dun: [u64; BLK_CRYPTO_DUN_ARRAY_SIZE] = [0; BLK_CRYPTO_DUN_ARRAY_SIZE];
    unsafe { fscrypt_generate_dun(ci, pos, dun.as_mut_ptr()); bio_crypt_set_ctx(bio, (*ci).ci_enc_key.blk_key, dun.as_mut_ptr(), gfp_mask); }
}

// EXPORT_SYMBOL_GPL(fscrypt_set_bio_crypt_ctx)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
