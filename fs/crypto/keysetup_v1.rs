// SPDX-License-Identifier: GPL-2.0
/*
 * Key setup for v1 encryption policies
 *
 * Copyright 2015, 2019 Google LLC
 */

/*
 * Compatibility functions for the original encryption policy version (v1).
 * The kernel and fscrypt definitions referenced here are supplied externally.
 */

// Table of keys referenced by DIRECT_KEY policies.
static mut FSCRYPT_DIRECT_KEYS: HashTable = DEFINE_HASHTABLE!(6);
static mut FSCRYPT_DIRECT_KEYS_LOCK: SpinLock = DEFINE_SPINLOCK!();

/*
 * Search the current task's subscribed keyrings for a "logon" key with
 * description prefix:descriptor, and if found acquire a read lock on it and
 * return a pointer to its validated payload in `payload_ret`.
 */
unsafe fn find_and_lock_process_key(
    prefix: *const c_char,
    descriptor: *const u8,
    min_keysize: c_uint,
    payload_ret: *mut *const fscrypt_key,
) -> *mut key {
    let description: *mut c_char = kasprintf(
        GFP_KERNEL,
        b"%s%*phN\0".as_ptr() as *const c_char,
        prefix,
        FSCRYPT_KEY_DESCRIPTOR_SIZE,
        descriptor,
    );
    if description.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    let key = request_key(&key_type_logon, description, core::ptr::null());
    kfree(description as *mut c_void);
    if IS_ERR(key) {
        return key;
    }

    down_read(&mut (*key).sem);
    let ukp = user_key_payload_locked(key);
    if ukp.is_null() {
        up_read(&mut (*key).sem);
        key_put(key);
        return ERR_PTR(-ENOKEY);
    }

    let payload = (*ukp).data as *const fscrypt_key;
    if (*ukp).datalen != core::mem::size_of::<fscrypt_key>()
        || (*payload).size < 1
        || (*payload).size > core::mem::size_of_val(&(*payload).raw)
    {
        fscrypt_warn(
            core::ptr::null(),
            b"key with description '%s' has invalid payload\0".as_ptr() as *const c_char,
            (*key).description,
        );
        up_read(&mut (*key).sem);
        key_put(key);
        return ERR_PTR(-ENOKEY);
    }

    if (*payload).size < min_keysize {
        fscrypt_warn(
            core::ptr::null(),
            b"key with description '%s' is too short (got %u bytes, need %u+ bytes)\0".as_ptr()
                as *const c_char,
            (*key).description,
            (*payload).size,
            min_keysize,
        );
        up_read(&mut (*key).sem);
        key_put(key);
        return ERR_PTR(-ENOKEY);
    }

    *payload_ret = payload;
    key
}

/* Master key referenced by DIRECT_KEY policy. */
#[repr(C)]
struct fscrypt_direct_key {
    dk_sb: *mut super_block,
    dk_node: hlist_node,
    dk_refcount: refcount_t,
    dk_mode: *const fscrypt_mode,
    dk_key: fscrypt_prepared_key,
    dk_descriptor: [u8; FSCRYPT_KEY_DESCRIPTOR_SIZE],
    dk_raw: [u8; FSCRYPT_MAX_RAW_KEY_SIZE],
}

unsafe fn free_direct_key(dk: *mut fscrypt_direct_key) {
    if !dk.is_null() {
        fscrypt_destroy_prepared_key((*dk).dk_sb, &mut (*dk).dk_key);
        kfree_sensitive(dk as *mut c_void);
    }
}

unsafe fn fscrypt_put_direct_key(dk: *mut fscrypt_direct_key) {
    if !refcount_dec_and_lock(&mut (*dk).dk_refcount, &mut FSCRYPT_DIRECT_KEYS_LOCK) {
        return;
    }
    hash_del(&mut (*dk).dk_node);
    spin_unlock(&mut FSCRYPT_DIRECT_KEYS_LOCK);
    free_direct_key(dk);
}

unsafe fn find_or_insert_direct_key(
    to_insert: *mut fscrypt_direct_key,
    raw_key: *const u8,
    ci: *const fscrypt_inode_info,
) -> *mut fscrypt_direct_key {
    let mut hash_key: c_ulong = 0;
    core::ptr::copy_nonoverlapping(
        (*ci).ci_policy.v1.master_key_descriptor.as_ptr(),
        &mut hash_key as *mut c_ulong as *mut u8,
        core::mem::size_of::<c_ulong>(),
    );

    spin_lock(&mut FSCRYPT_DIRECT_KEYS_LOCK);
    let mut dk: *mut fscrypt_direct_key = core::ptr::null_mut();
    hash_for_each_possible!(FSCRYPT_DIRECT_KEYS, dk, dk_node, hash_key, {
        if libc_memcmp(
            (*ci).ci_policy.v1.master_key_descriptor.as_ptr() as *const c_void,
            (*dk).dk_descriptor.as_ptr() as *const c_void,
            FSCRYPT_KEY_DESCRIPTOR_SIZE,
        ) != 0
            || (*ci).ci_inode.as_ref().unwrap().i_sb != (*dk).dk_sb
            || (*ci).ci_mode != (*dk).dk_mode
            || !fscrypt_is_key_prepared(&(*dk).dk_key, ci)
            || crypto_memneq(raw_key, (*dk).dk_raw.as_ptr(), (*ci).ci_mode.as_ref().unwrap().keysize)
        {
            continue;
        }
        refcount_inc(&mut (*dk).dk_refcount);
        spin_unlock(&mut FSCRYPT_DIRECT_KEYS_LOCK);
        free_direct_key(to_insert);
        return dk;
    });
    if !to_insert.is_null() {
        hash_add!(FSCRYPT_DIRECT_KEYS, &mut (*to_insert).dk_node, hash_key);
    }
    spin_unlock(&mut FSCRYPT_DIRECT_KEYS_LOCK);
    to_insert
}

unsafe fn fscrypt_get_direct_key(
    ci: *const fscrypt_inode_info,
    raw_key: *const u8,
) -> *mut fscrypt_direct_key {
    let dk = find_or_insert_direct_key(core::ptr::null_mut(), raw_key, ci);
    if !dk.is_null() {
        return dk;
    }
    let dk = kzalloc_obj::<fscrypt_direct_key>();
    if dk.is_null() {
        return ERR_PTR(-ENOMEM);
    }
    (*dk).dk_sb = (*ci).ci_inode.as_ref().unwrap().i_sb;
    refcount_set(&mut (*dk).dk_refcount, 1);
    (*dk).dk_mode = (*ci).ci_mode;
    let err = fscrypt_prepare_key(&mut (*dk).dk_key, raw_key, ci);
    if err != 0 {
        free_direct_key(dk);
        return ERR_PTR(err);
    }
    core::ptr::copy_nonoverlapping(
        (*ci).ci_policy.v1.master_key_descriptor.as_ptr(),
        (*dk).dk_descriptor.as_mut_ptr(),
        FSCRYPT_KEY_DESCRIPTOR_SIZE,
    );
    core::ptr::copy_nonoverlapping(
        raw_key,
        (*dk).dk_raw.as_mut_ptr(),
        (*ci).ci_mode.as_ref().unwrap().keysize,
    );
    find_or_insert_direct_key(dk, raw_key, ci)
}

unsafe fn setup_v1_file_key_direct(
    ci: *mut fscrypt_inode_info,
    raw_master_key: *const u8,
) -> c_int {
    let dk = fscrypt_get_direct_key(ci, raw_master_key);
    if IS_ERR(dk) {
        return PTR_ERR(dk);
    }
    (*ci).ci_direct_key = dk;
    (*ci).ci_enc_key = (*dk).dk_key;
    0
}

unsafe fn setup_v1_file_key_derived(
    ci: *mut fscrypt_inode_info,
    raw_master_key: *const u8,
) -> c_int {
    let derived_keysize = (*ci).ci_mode.as_ref().unwrap().keysize;
    let mut derived_key = [0u8; FSCRYPT_MAX_RAW_KEY_SIZE];
    let mut aes: aes_enckey = core::mem::zeroed();

    if WARN_ON_ONCE(
        derived_keysize > FSCRYPT_MAX_RAW_KEY_SIZE || derived_keysize % AES_BLOCK_SIZE != 0,
    ) {
        return -EINVAL;
    }
    aes_prepareenckey(&mut aes, (*ci).ci_nonce.as_ptr(), FSCRYPT_FILE_NONCE_SIZE);
    let mut i = 0;
    while i < derived_keysize {
        aes_encrypt(&aes, derived_key.as_mut_ptr().add(i), raw_master_key.add(i));
        i += AES_BLOCK_SIZE;
    }
    let err = fscrypt_set_per_file_enc_key(ci, derived_key.as_ptr());
    memzero_explicit(derived_key.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&derived_key));
    err
}

unsafe fn fscrypt_setup_v1_file_key(
    ci: *mut fscrypt_inode_info,
    raw_master_key: *const u8,
) -> c_int {
    if (*ci).ci_policy.v1.flags & FSCRYPT_POLICY_FLAG_DIRECT_KEY != 0 {
        setup_v1_file_key_direct(ci, raw_master_key)
    } else {
        setup_v1_file_key_derived(ci, raw_master_key)
    }
}

unsafe fn fscrypt_setup_v1_file_key_via_subscribed_keyrings(
    ci: *mut fscrypt_inode_info,
) -> c_int {
    let sb = (*ci).ci_inode.as_ref().unwrap().i_sb;
    let mut payload: *const fscrypt_key = core::ptr::null();
    let mut key = find_and_lock_process_key(
        FSCRYPT_KEY_DESC_PREFIX.as_ptr() as *const c_char,
        (*ci).ci_policy.v1.master_key_descriptor.as_ptr(),
        (*ci).ci_mode.as_ref().unwrap().keysize,
        &mut payload,
    );
    if key == ERR_PTR(-ENOKEY) && !(*sb).s_cop.as_ref().unwrap().legacy_key_prefix.is_null() {
        key = find_and_lock_process_key(
            (*sb).s_cop.as_ref().unwrap().legacy_key_prefix,
            (*ci).ci_policy.v1.master_key_descriptor.as_ptr(),
            (*ci).ci_mode.as_ref().unwrap().keysize,
            &mut payload,
        );
    }
    if IS_ERR(key) {
        return PTR_ERR(key);
    }
    let err = fscrypt_setup_v1_file_key(ci, (*payload).raw.as_ptr());
    up_read(&mut (*key).sem);
    key_put(key);
    err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
