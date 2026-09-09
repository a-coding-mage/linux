// SPDX-License-Identifier: GPL-2.0-only
/* fs/crypto/hooks.c -- Encryption hooks for higher-level filesystem operations. */

// Kernel dependencies supplied by the surrounding fscrypt/VFS implementation.

pub unsafe fn fscrypt_file_open(inode: *mut inode, filp: *mut file) -> c_int {
    let mut err: c_int;
    let dentry: *mut dentry;
    let mut dentry_parent: *mut dentry;
    let inode_parent: *mut inode;

    err = fscrypt_require_key(inode);
    if err != 0 { return err; }
    dentry = file_dentry(filp);

    rcu_read_lock();
    dentry_parent = core::ptr::read_volatile(&(*dentry).d_parent);
    inode_parent = d_inode_rcu(dentry_parent);
    if !inode_parent.is_null() && !IS_ENCRYPTED(inode_parent) {
        rcu_read_unlock();
        return 0;
    }
    rcu_read_unlock();

    dentry_parent = dget_parent(dentry);
    if !fscrypt_has_permitted_context(d_inode(dentry_parent), inode) {
        fscrypt_warn(inode, "Inconsistent encryption context (parent directory: %llu)", (*d_inode(dentry_parent)).i_ino);
        err = -EPERM;
    }
    dput(dentry_parent);
    err
}

pub unsafe fn __fscrypt_prepare_link(inode: *mut inode, dir: *mut inode, dentry: *mut dentry) -> c_int {
    if fscrypt_is_nokey_name(dentry) { return -ENOKEY; }
    if !fscrypt_has_permitted_context(dir, inode) { return -EXDEV; }
    0
}

pub unsafe fn __fscrypt_prepare_rename(old_dir: *mut inode, old_dentry: *mut dentry,
                                        new_dir: *mut inode, new_dentry: *mut dentry,
                                        flags: c_uint) -> c_int {
    if fscrypt_is_nokey_name(old_dentry) || fscrypt_is_nokey_name(new_dentry) { return -ENOKEY; }
    if old_dir != new_dir {
        if IS_ENCRYPTED(new_dir) && !fscrypt_has_permitted_context(new_dir, d_inode(old_dentry)) { return -EXDEV; }
        if (flags & RENAME_EXCHANGE) != 0 && IS_ENCRYPTED(old_dir) &&
            !fscrypt_has_permitted_context(old_dir, d_inode(new_dentry)) { return -EXDEV; }
    }
    0
}

pub unsafe fn __fscrypt_prepare_lookup(dir: *mut inode, dentry: *mut dentry, fname: *mut fscrypt_name) -> c_int {
    let err = fscrypt_setup_filename(dir, &(*dentry).d_name, 1, fname);
    if err != 0 && err != -ENOENT { return err; }
    fscrypt_prepare_dentry(dentry, (*fname).is_nokey_name);
    err
}

pub unsafe fn fscrypt_prepare_lookup_partial(dir: *mut inode, dentry: *mut dentry) -> c_int {
    let err = fscrypt_get_encryption_info(dir, true);
    let is_nokey_name = err == 0 && !fscrypt_has_encryption_key(dir);
    fscrypt_prepare_dentry(dentry, is_nokey_name);
    err
}

pub unsafe fn __fscrypt_prepare_readdir(dir: *mut inode) -> c_int { fscrypt_get_encryption_info(dir, true) }

pub unsafe fn __fscrypt_prepare_setattr(dentry: *mut dentry, attr: *mut iattr) -> c_int {
    if ((*attr).ia_valid & ATTR_SIZE) != 0 { return fscrypt_require_key(d_inode(dentry)); }
    0
}

pub unsafe fn fscrypt_prepare_setflags(inode: *mut inode, oldflags: c_uint, flags: c_uint) -> c_int {
    if IS_ENCRYPTED(inode) && (flags & !oldflags & FS_CASEFOLD_FL) != 0 {
        let err = fscrypt_require_key(inode);
        if err != 0 { return err; }
        let ci = fscrypt_get_inode_info_raw(inode);
        if (*ci).ci_policy.version != FSCRYPT_POLICY_V2 { return -EINVAL; }
        let mk = (*ci).ci_master_key;
        down_read(&(*mk).mk_sem);
        let result = if (*mk).mk_present { fscrypt_derive_dirhash_key(ci, mk); 0 } else { -ENOKEY };
        up_read(&(*mk).mk_sem);
        return result;
    }
    0
}

pub unsafe fn fscrypt_prepare_symlink(dir: *mut inode, target: *const c_char, len: c_uint,
                                      max_len: c_uint, disk_link: *mut fscrypt_str) -> c_int {
    let policy = fscrypt_policy_to_inherit(dir);
    if policy.is_null() {
        (*disk_link).name = target as *mut u8;
        (*disk_link).len = len + 1;
        return if (*disk_link).len > max_len { -ENAMETOOLONG } else { 0 };
    }
    if IS_ERR(policy) { return PTR_ERR(policy); }
    if !__fscrypt_fname_encrypted_size(policy, len, max_len - core::mem::size_of::<fscrypt_symlink_data>() as u32 - 1, &mut (*disk_link).len) { return -ENAMETOOLONG; }
    (*disk_link).len += core::mem::size_of::<fscrypt_symlink_data>() as u32 + 1;
    (*disk_link).name = core::ptr::null_mut();
    0
}

pub unsafe fn __fscrypt_encrypt_symlink(inode: *mut inode, target: *const c_char, len: c_uint,
                                        disk_link: *mut fscrypt_str) -> c_int {
    let iname = QSTR_INIT(target, len);
    if WARN_ON_ONCE(!fscrypt_has_encryption_key(inode)) { return -ENOKEY; }
    let sd: *mut fscrypt_symlink_data = if !(*disk_link).name.is_null() {
        (*disk_link).name as *mut fscrypt_symlink_data
    } else {
        let p = kmalloc((*disk_link).len, GFP_NOFS) as *mut fscrypt_symlink_data;
        if p.is_null() { return -ENOMEM; }
        p
    };
    let ciphertext_len = (*disk_link).len - core::mem::size_of::<fscrypt_symlink_data>() as u32 - 1;
    (*sd).len = cpu_to_le16(ciphertext_len as u16);
    let err = fscrypt_fname_encrypt(inode, &iname, (*sd).encrypted_path, ciphertext_len);
    if err != 0 { if (*disk_link).name.is_null() { kfree(sd as *mut c_void); } return err; }
    (*sd).encrypted_path[ciphertext_len as usize] = 0;
    (*inode).i_link = kmemdup(target as *const c_void, len + 1, GFP_NOFS) as *const c_char;
    if (*inode).i_link.is_null() { if (*disk_link).name.is_null() { kfree(sd as *mut c_void); } return -ENOMEM; }
    if (*disk_link).name.is_null() { (*disk_link).name = sd as *mut u8; }
    0
}

pub unsafe fn fscrypt_get_symlink(inode: *mut inode, caddr: *const c_void, max_size: c_uint,
                                  done: *mut delayed_call) -> *const c_char {
    if WARN_ON_ONCE(!IS_ENCRYPTED(inode)) { return ERR_PTR(-EINVAL); }
    let mut pstr = fscrypt_str { name: core::ptr::read_volatile(&(*inode).i_link), len: 0 };
    if !pstr.name.is_null() { return pstr.name as *const c_char; }
    let err = fscrypt_get_encryption_info(inode, false); if err != 0 { return ERR_PTR(err); }
    let has_key = fscrypt_has_encryption_key(inode);
    if max_size < core::mem::size_of::<fscrypt_symlink_data>() as u32 + 1 { return ERR_PTR(-EUCLEAN); }
    let sd = caddr as *const fscrypt_symlink_data;
    let cstr = fscrypt_str { name: (*sd).encrypted_path as *mut u8, len: le16_to_cpu((*sd).len) as u32 };
    if cstr.len == 0 || cstr.len + core::mem::size_of::<fscrypt_symlink_data>() as u32 > max_size { return ERR_PTR(-EUCLEAN); }
    let mut out = fscrypt_str { name: core::ptr::null_mut(), len: 0 };
    let mut err = fscrypt_fname_alloc_buffer(cstr.len, &mut out); if err != 0 { return ERR_PTR(err); }
    err = fscrypt_fname_disk_to_usr(inode, 0, 0, &cstr, &mut out);
    if err != 0 || out.name[0] == 0 { kfree(out.name as *mut c_void); return ERR_PTR(if err != 0 { err } else { -EUCLEAN }); }
    out.name[out.len as usize] = 0;
    if !has_key || cmpxchg_release(&mut (*inode).i_link, core::ptr::null(), out.name as *const c_char) != core::ptr::null() { set_delayed_call(done, kfree_link, out.name); }
    out.name as *const c_char
}

pub unsafe fn fscrypt_symlink_getattr(path: *const path, stat: *mut kstat) -> c_int {
    let dentry = (*path).dentry; let inode = d_inode(dentry); let mut done = DELAYED_CALL_INIT();
    let mut link = core::ptr::read_volatile(&(*inode).i_link);
    if link.is_null() { link = (*(*inode).i_op).get_link(dentry, inode, &mut done); if IS_ERR(link) { return PTR_ERR(link); } }
    (*stat).size = strlen(link as *const c_char); do_delayed_call(&mut done); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
