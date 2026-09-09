// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the Linux and ext4 headers are intentionally external.

unsafe fn ext4_fname_from_fscrypt_name(
    dst: *mut ext4_filename,
    src: *const fscrypt_name,
) {
    memset(dst as *mut core::ffi::c_void, 0, core::mem::size_of::<ext4_filename>());

    (*dst).usr_fname = (*src).usr_fname;
    (*dst).disk_name = (*src).disk_name;
    (*dst).hinfo.hash = (*src).hash;
    (*dst).hinfo.minor_hash = (*src).minor_hash;
    (*dst).crypto_buf = (*src).crypto_buf;
}

pub unsafe fn ext4_fname_setup_filename(
    dir: *mut inode,
    iname: *const qstr,
    lookup: i32,
    fname: *mut ext4_filename,
) -> i32 {
    let mut name: fscrypt_name = core::mem::zeroed();
    let mut err: i32;

    err = fscrypt_setup_filename(dir, iname, lookup, &mut name);
    if err != 0 {
        return err;
    }

    ext4_fname_from_fscrypt_name(fname, &name);

    err = ext4_fname_setup_ci_filename(dir, iname, fname);
    if err != 0 {
        ext4_fname_free_filename(fname);
    }

    err
}

pub unsafe fn ext4_fname_prepare_lookup(
    dir: *mut inode,
    dentry: *mut dentry,
    fname: *mut ext4_filename,
) -> i32 {
    let mut name: fscrypt_name = core::mem::zeroed();
    let mut err: i32;

    err = fscrypt_prepare_lookup(dir, dentry, &mut name);
    if err != 0 {
        return err;
    }

    ext4_fname_from_fscrypt_name(fname, &name);

    err = ext4_fname_setup_ci_filename(dir, &(*dentry).d_name, fname);
    if err != 0 {
        ext4_fname_free_filename(fname);
    }
    err
}

pub unsafe fn ext4_fname_free_filename(fname: *mut ext4_filename) {
    let mut name: fscrypt_name = core::mem::zeroed();

    name.crypto_buf = (*fname).crypto_buf;
    fscrypt_free_filename(&mut name);

    (*fname).crypto_buf.name = core::ptr::null_mut();
    (*fname).usr_fname = core::ptr::null_mut();
    (*fname).disk_name.name = core::ptr::null_mut();

    ext4_fname_free_ci_filename(fname);
}

unsafe fn uuid_is_zero(u: *const u8) -> bool {
    let mut i = 0;
    while i < 16 {
        if *u.add(i) != 0 {
            return false;
        }
        i += 1;
    }
    true
}

pub unsafe fn ext4_ioctl_get_encryption_pwsalt(
    filp: *mut file,
    arg: *mut core::ffi::c_void,
) -> i32 {
    let sb = (*file_inode(filp)).i_sb;
    let sbi = EXT4_SB(sb);
    let mut err: i32;
    let mut err2: i32;
    let handle: *mut handle_t;

    if !ext4_has_feature_encrypt(sb) {
        return -EOPNOTSUPP;
    }

    if uuid_is_zero((*sbi).s_es.s_encrypt_pw_salt.as_ptr()) {
        err = mnt_want_write_file(filp);
        if err != 0 {
            return err;
        }
        handle = ext4_journal_start_sb(sb, EXT4_HT_MISC, 1);
        if IS_ERR(handle) {
            err = PTR_ERR(handle);
            goto pwsalt_err_exit;
        }
        err = ext4_journal_get_write_access(handle, sb, (*sbi).s_sbh, EXT4_JTR_NONE);
        if err != 0 {
            goto pwsalt_err_journal;
        }
        lock_buffer((*sbi).s_sbh);
        generate_random_uuid((*sbi).s_es.s_encrypt_pw_salt.as_mut_ptr());
        ext4_superblock_csum_set(sb);
        unlock_buffer((*sbi).s_sbh);
        err = ext4_handle_dirty_metadata(handle, core::ptr::null_mut(), (*sbi).s_sbh);
pwsalt_err_journal:
        err2 = ext4_journal_stop(handle);
        if err2 != 0 && err == 0 {
            err = err2;
        }
pwsalt_err_exit:
        mnt_drop_write_file(filp);
        if err != 0 {
            return err;
        }
    }

    if copy_to_user(arg, (*sbi).s_es.s_encrypt_pw_salt.as_ptr() as *const core::ffi::c_void, 16) != 0 {
        return -EFAULT;
    }
    0
}

unsafe fn ext4_get_context(inode: *mut inode, ctx: *mut core::ffi::c_void, len: usize) -> i32 {
    ext4_xattr_get(inode, EXT4_XATTR_INDEX_ENCRYPTION, EXT4_XATTR_NAME_ENCRYPTION_CONTEXT, ctx, len)
}

unsafe fn ext4_set_context(
    inode: *mut inode,
    ctx: *const core::ffi::c_void,
    len: usize,
    fs_data: *mut core::ffi::c_void,
) -> i32 {
    let mut handle = fs_data as *mut handle_t;
    let mut res: i32;
    let mut res2: i32;
    let mut credits: i32;
    let mut retries: u32 = 0;

    // Encrypting the root directory is not allowed because e2fsck expects
    // lost+found to exist and be unencrypted.
    if (*inode).i_ino == EXT4_ROOT_INO {
        return -EPERM;
    }
    if WARN_ON_ONCE(IS_DAX(inode)) {
        return -EINVAL;
    }
    if ext4_test_inode_flag(inode, EXT4_INODE_DAX) {
        return -EOPNOTSUPP;
    }

    res = ext4_convert_inline_data(inode);
    if res != 0 {
        return res;
    }

    if !handle.is_null() {
        if WARN_ON_ONCE(!IS_ENCRYPTED(inode)) ||
            WARN_ON_ONCE(ext4_test_inode_state(inode, EXT4_STATE_MAY_INLINE_DATA)) {
            return -EINVAL;
        }
        return ext4_xattr_set_handle(
            handle, inode, EXT4_XATTR_INDEX_ENCRYPTION,
            EXT4_XATTR_NAME_ENCRYPTION_CONTEXT, ctx, len, XATTR_CREATE,
        );
    }

    res = dquot_initialize(inode);
    if res != 0 {
        return res;
    }
retry:
    res = ext4_xattr_set_credits(inode, len, false, &mut credits);
    if res != 0 {
        return res;
    }
    handle = ext4_journal_start(inode, EXT4_HT_MISC, credits);
    if IS_ERR(handle) {
        return PTR_ERR(handle);
    }
    res = ext4_xattr_set_handle(
        handle, inode, EXT4_XATTR_INDEX_ENCRYPTION,
        EXT4_XATTR_NAME_ENCRYPTION_CONTEXT, ctx, len, 0,
    );
    if res == 0 {
        ext4_set_inode_flag(inode, EXT4_INODE_ENCRYPT);
        ext4_set_inode_flags(inode, false);
        res = ext4_mark_inode_dirty(handle, inode);
        if res != 0 {
            EXT4_ERROR_INODE(inode, "Failed to mark inode dirty");
        }
    }
    res2 = ext4_journal_stop(handle);
    if res == -ENOSPC && ext4_should_retry_alloc((*inode).i_sb, &mut retries) {
        goto retry;
    }
    if res == 0 {
        res = res2;
    }
    res
}

unsafe fn ext4_get_dummy_policy(sb: *mut super_block) -> *const fscrypt_policy {
    &(*EXT4_SB(sb)).s_dummy_enc_policy.policy
}

unsafe fn ext4_has_stable_inodes(sb: *mut super_block) -> bool {
    ext4_has_feature_stable_inodes(sb)
}

pub static ext4_cryptops: fscrypt_operations = fscrypt_operations {
    inode_info_offs: core::mem::offset_of!(ext4_inode_info, i_crypt_info) as i32 -
        core::mem::offset_of!(ext4_inode_info, vfs_inode) as i32,
    is_block_based: 1,
    has_32bit_inodes: 1,
    supports_subblock_data_units: 1,
    legacy_key_prefix: *b"ext4:\0",
    get_context: Some(ext4_get_context),
    set_context: Some(ext4_set_context),
    get_dummy_policy: Some(ext4_get_dummy_policy),
    empty_dir: Some(ext4_empty_dir),
    has_stable_inodes: Some(ext4_has_stable_inodes),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
