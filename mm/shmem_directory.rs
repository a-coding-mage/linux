// SPDX-License-Identifier: GPL-2.0-only
#[export_name = "rust_shmem_owner_shmem_statfs"]
unsafe extern "C" fn shmem_statfs(dentry: *mut dentry, buf: *mut kstatfs) -> c_int {
    let sb = SHMEM_SB((*dentry).d_sb);
    (*buf).f_type = RUST_SHMEM_TMPFS_MAGIC as _;
    (*buf).f_bsize = RUST_SHMEM_PAGE_SIZE as _;
    (*buf).f_namelen = RUST_SHMEM_NAME_MAX as _;
    if (*sb).max_blocks != 0 {
        (*buf).f_blocks = (*sb).max_blocks as _;
        (*buf).f_bfree = ((*sb).max_blocks as u64)
            .wrapping_sub(percpu_counter_sum(addr_of_mut!((*sb).used_blocks)) as u64);
        (*buf).f_bavail = (*buf).f_bfree;
    }
    if (*sb).max_inodes != 0 {
        (*buf).f_files = (*sb).max_inodes as _;
        (*buf).f_ffree = ((*sb).free_ispace / RUST_SHMEM_BOGO_INODE_SIZE as c_ulong) as _;
    }
    (*buf).f_fsid = uuid_to_fsid((*(*dentry).d_sb).s_uuid.b.as_mut_ptr());
    0
}
unsafe fn shmem_security_init(inode: *mut inode, dir: *mut inode, name: *const qstr) -> c_int {
    #[cfg(CONFIG_TMPFS_XATTR)]
    let callback = Some(
        shmem_initxattrs as unsafe extern "C" fn(*mut inode, *const xattr, *mut c_void) -> c_int,
    );
    #[cfg(not(CONFIG_TMPFS_XATTR))]
    let callback = None;
    security_inode_init_security(inode, dir, name, callback, null_mut())
}
#[export_name = "rust_shmem_owner_shmem_mknod"]
unsafe extern "C" fn shmem_mknod(
    idmap: *mut mnt_idmap,
    dir: *mut inode,
    dentry: *mut dentry,
    mode: umode_t,
    dev: dev_t,
) -> c_int {
    if !generic_ci_validate_strict_name(dir, addr_of!((*dentry).d_name)) {
        return -(RUST_SHMEM_EINVAL as c_int);
    }
    let inode = shmem_get_inode(
        idmap,
        (*dir).i_sb,
        dir,
        mode,
        dev,
        rust_shmem_noreserve_flags(),
    );
    if IS_ERR(inode.cast()) {
        return ptr_err(inode);
    }
    let mut error = simple_acl_create(dir, inode);
    if error == 0 {
        error = shmem_security_init(inode, dir, addr_of!((*dentry).d_name));
        if error == 0 || error == -(RUST_SHMEM_EOPNOTSUPP as c_int) {
            error = simple_offset_add(shmem_get_offset_ctx(dir), dentry);
        }
    }
    if error != 0 {
        iput(inode);
        return error;
    }
    (*dir).i_size = (*dir)
        .i_size
        .wrapping_add(RUST_SHMEM_BOGO_DIRENT_SIZE as loff_t);
    inode_set_mtime_to_ts(dir, inode_set_ctime_current(dir));
    inode_inc_iversion(dir);
    d_make_persistent(dentry, inode);
    error
}
#[export_name = "rust_shmem_owner_shmem_tmpfile"]
unsafe extern "C" fn shmem_tmpfile(
    idmap: *mut mnt_idmap,
    dir: *mut inode,
    file: *mut file,
    mode: umode_t,
) -> c_int {
    let inode = shmem_get_inode(
        idmap,
        (*dir).i_sb,
        dir,
        mode,
        0,
        rust_shmem_noreserve_flags(),
    );
    if IS_ERR(inode.cast()) {
        return finish_open_simple(file, ptr_err(inode));
    }
    let mut error = shmem_security_init(inode, dir, null());
    if error == 0 || error == -(RUST_SHMEM_EOPNOTSUPP as c_int) {
        error = simple_acl_create(dir, inode);
    }
    if error != 0 {
        iput(inode);
        return error;
    }
    d_tmpfile(file, inode);
    finish_open_simple(file, error)
}
#[export_name = "rust_shmem_owner_shmem_mkdir"]
unsafe extern "C" fn shmem_mkdir(
    idmap: *mut mnt_idmap,
    dir: *mut inode,
    dentry: *mut dentry,
    mode: umode_t,
) -> *mut dentry {
    let error = shmem_mknod(idmap, dir, dentry, mode | RUST_SHMEM_S_IFDIR as umode_t, 0);
    if error != 0 {
        return err_ptr(error);
    }
    inc_nlink(dir);
    null_mut()
}
#[export_name = "rust_shmem_owner_shmem_create"]
unsafe extern "C" fn shmem_create(
    idmap: *mut mnt_idmap,
    dir: *mut inode,
    dentry: *mut dentry,
    mode: umode_t,
) -> c_int {
    shmem_mknod(idmap, dir, dentry, mode | RUST_SHMEM_S_IFREG as umode_t, 0)
}
#[export_name = "rust_shmem_owner_shmem_link"]
unsafe extern "C" fn shmem_link(old: *mut dentry, dir: *mut inode, new: *mut dentry) -> c_int {
    let inode = d_inode(old);
    if rust_shmem_inode_nlink(inode) != 0 {
        let ret = shmem_reserve_inode((*inode).i_sb, null_mut());
        if ret != 0 {
            return ret;
        }
    }
    let ret = simple_offset_add(shmem_get_offset_ctx(dir), new);
    if ret != 0 {
        if rust_shmem_inode_nlink(inode) != 0 {
            shmem_free_inode((*inode).i_sb, 0);
        }
        return ret;
    }
    (*dir).i_size = (*dir)
        .i_size
        .wrapping_add(RUST_SHMEM_BOGO_DIRENT_SIZE as loff_t);
    inode_inc_iversion(dir);
    simple_link(old, dir, new)
}
#[export_name = "rust_shmem_owner_shmem_unlink"]
unsafe extern "C" fn shmem_unlink(dir: *mut inode, dentry: *mut dentry) -> c_int {
    let inode = d_inode(dentry);
    if rust_shmem_inode_nlink(inode) > 1 && !rust_shmem_isdir((*inode).i_mode) {
        shmem_free_inode((*inode).i_sb, 0);
    }
    simple_offset_remove(shmem_get_offset_ctx(dir), dentry);
    (*dir).i_size = (*dir)
        .i_size
        .wrapping_sub(RUST_SHMEM_BOGO_DIRENT_SIZE as loff_t);
    inode_inc_iversion(dir);
    simple_unlink(dir, dentry);
    #[cfg(CONFIG_UNICODE)]
    if rust_shmem_casefolded(dir) {
        d_invalidate(dentry);
    }
    0
}
#[export_name = "rust_shmem_owner_shmem_rmdir"]
unsafe extern "C" fn shmem_rmdir(dir: *mut inode, dentry: *mut dentry) -> c_int {
    if !rust_shmem_simple_empty(dentry) {
        return -(RUST_SHMEM_ENOTEMPTY as c_int);
    }
    drop_nlink(d_inode(dentry));
    drop_nlink(dir);
    shmem_unlink(dir, dentry)
}
unsafe fn shmem_whiteout(idmap: *mut mnt_idmap, dir: *mut inode, old: *mut dentry) -> c_int {
    let whiteout = d_alloc((*old).d_parent, addr_of!((*old).d_name));
    if whiteout.is_null() {
        return -(RUST_SHMEM_ENOMEM as c_int);
    }
    let ret = shmem_mknod(
        idmap,
        dir,
        whiteout,
        (RUST_SHMEM_S_IFCHR | RUST_SHMEM_WHITEOUT_MODE) as umode_t,
        RUST_SHMEM_WHITEOUT_DEV as dev_t,
    );
    dput(whiteout);
    ret
}
#[export_name = "rust_shmem_owner_shmem_rename2"]
unsafe extern "C" fn shmem_rename2(
    idmap: *mut mnt_idmap,
    old_dir: *mut inode,
    old: *mut dentry,
    new_dir: *mut inode,
    new: *mut dentry,
    flags: c_uint,
) -> c_int {
    let dirs = rust_shmem_isdir((*d_inode(old)).i_mode);
    if flags
        & !((RUST_SHMEM_RENAME_NOREPLACE | RUST_SHMEM_RENAME_EXCHANGE | RUST_SHMEM_RENAME_WHITEOUT)
            as c_uint)
        != 0
    {
        return -(RUST_SHMEM_EINVAL as c_int);
    }
    if flags & RUST_SHMEM_RENAME_EXCHANGE as c_uint != 0 {
        return simple_offset_rename_exchange(old_dir, old, new_dir, new);
    }
    if !rust_shmem_simple_empty(new) {
        return -(RUST_SHMEM_ENOTEMPTY as c_int);
    }
    let error = simple_offset_add(shmem_get_offset_ctx(new_dir), new);
    let had_offset = error == -(RUST_SHMEM_EBUSY as c_int);
    if error != 0 && !had_offset {
        return error;
    }
    if flags & RUST_SHMEM_RENAME_WHITEOUT as c_uint != 0 {
        let err = shmem_whiteout(idmap, old_dir, old);
        if err != 0 {
            if !had_offset {
                simple_offset_remove(shmem_get_offset_ctx(new_dir), new);
            }
            return err;
        }
    }
    simple_offset_rename(old_dir, old, new_dir, new);
    if d_really_is_positive(new) {
        shmem_unlink(new_dir, new);
        if dirs {
            drop_nlink(d_inode(new));
            drop_nlink(old_dir);
        }
    } else if dirs {
        drop_nlink(old_dir);
        inc_nlink(new_dir);
    }
    (*old_dir).i_size = (*old_dir)
        .i_size
        .wrapping_sub(RUST_SHMEM_BOGO_DIRENT_SIZE as loff_t);
    (*new_dir).i_size = (*new_dir)
        .i_size
        .wrapping_add(RUST_SHMEM_BOGO_DIRENT_SIZE as loff_t);
    simple_rename_timestamp(old_dir, old, new_dir, new);
    inode_inc_iversion(old_dir);
    inode_inc_iversion(new_dir);
    0
}
#[export_name = "rust_shmem_owner_shmem_symlink"]
unsafe extern "C" fn shmem_symlink(
    idmap: *mut mnt_idmap,
    dir: *mut inode,
    dentry: *mut dentry,
    name: *const c_char,
) -> c_int {
    let len = rust_shmem_strlen(name).wrapping_add(1) as c_int;
    if len > RUST_SHMEM_PAGE_SIZE as c_int {
        return -(RUST_SHMEM_ENAMETOOLONG as c_int);
    }
    let inode = shmem_get_inode(
        idmap,
        (*dir).i_sb,
        dir,
        RUST_SHMEM_S_IFLNK as umode_t | 0o777,
        0,
        rust_shmem_noreserve_flags(),
    );
    if IS_ERR(inode.cast()) {
        return ptr_err(inode);
    }
    let mut error = shmem_security_init(inode, dir, addr_of!((*dentry).d_name));
    if error != 0 && error != -(RUST_SHMEM_EOPNOTSUPP as c_int) {
        iput(inode);
        return error;
    }
    error = simple_offset_add(shmem_get_offset_ctx(dir), dentry);
    if error != 0 {
        iput(inode);
        return error;
    }
    (*inode).i_size = len as loff_t - 1;
    'create: {
        if len <= RUST_SHMEM_SHORT_SYMLINK_LEN as c_int {
            let link =
                rust_shmem_kmemdup(name.cast(), len as usize, RUST_SHMEM_GFP_KERNEL as gfp_t);
            if link.is_null() {
                error = -(RUST_SHMEM_ENOMEM as c_int);
                break 'create;
            }
            (*inode).i_op = addr_of!(rust_shmem_data_shmem_short_symlink_operations);
            inode_set_cached_link(inode, link.cast(), len as usize - 1);
        } else {
            inode_nohighmem(inode);
            (*(*inode).i_mapping).a_ops = addr_of!(rust_shmem_data_shmem_aops);
            let mut f = null_mut();
            error = shmem_get_folio(inode, 0, 0, &mut f, SGP_WRITE);
            if error != 0 {
                break 'create;
            }
            (*inode).i_op = addr_of!(rust_shmem_data_shmem_symlink_inode_operations);
            rust_shmem_memcpy(folio_address(f), name.cast(), len as usize);
            folio_zero_range(f, len as usize, folio_size(f) - len as usize);
            folio_mark_uptodate(f);
            folio_mark_dirty(f);
            folio_unlock(f);
            folio_put(f);
        }
        (*dir).i_size = (*dir)
            .i_size
            .wrapping_add(RUST_SHMEM_BOGO_DIRENT_SIZE as loff_t);
        inode_set_mtime_to_ts(dir, inode_set_ctime_current(dir));
        inode_inc_iversion(dir);
        d_make_persistent(dentry, inode);
        return 0;
    }
    simple_offset_remove(shmem_get_offset_ctx(dir), dentry);
    iput(inode);
    error
}
#[export_name = "rust_shmem_owner_shmem_put_link"]
unsafe extern "C" fn shmem_put_link(arg: *mut c_void) {
    folio_mark_accessed(arg.cast());
    folio_put(arg.cast());
}
#[export_name = "rust_shmem_owner_shmem_get_link"]
unsafe extern "C" fn shmem_get_link(
    dentry: *mut dentry,
    inode: *mut inode,
    done: *mut delayed_call,
) -> *const c_char {
    let mut f;
    if dentry.is_null() {
        f = filemap_get_folio((*inode).i_mapping, 0);
        if IS_ERR(f.cast()) {
            return err_ptr(-(RUST_SHMEM_ECHILD as c_int));
        }
        if rust_shmem_PageHWPoison(rust_shmem_folio_page(f, 0)) || !folio_test_uptodate(f) {
            folio_put(f);
            return err_ptr(-(RUST_SHMEM_ECHILD as c_int));
        }
    } else {
        f = null_mut();
        let error = shmem_get_folio(inode, 0, 0, &mut f, SGP_READ);
        if error != 0 {
            return err_ptr(error);
        }
        if f.is_null() {
            return err_ptr(-(RUST_SHMEM_ECHILD as c_int));
        }
        if rust_shmem_PageHWPoison(rust_shmem_folio_page(f, 0)) {
            folio_unlock(f);
            folio_put(f);
            return err_ptr(-(RUST_SHMEM_ECHILD as c_int));
        }
        folio_unlock(f);
    }
    set_delayed_call(done, Some(shmem_put_link), f.cast());
    folio_address(f).cast()
}
