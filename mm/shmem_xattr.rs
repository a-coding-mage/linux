// SPDX-License-Identifier: GPL-2.0-only
#[export_name = "rust_shmem_owner_shmem_fileattr_get"]
unsafe extern "C" fn shmem_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> c_int {
    fileattr_fill_flags(
        fa,
        (*SHMEM_I(d_inode(dentry))).fsflags & RUST_SHMEM_SHMEM_FL_USER_VISIBLE as c_uint,
    );
    0
}
#[export_name = "rust_shmem_owner_shmem_fileattr_set"]
unsafe extern "C" fn shmem_fileattr_set(
    _idmap: *mut mnt_idmap,
    dentry: *mut dentry,
    fa: *mut file_kattr,
) -> c_int {
    let inode = d_inode(dentry);
    let info = SHMEM_I(inode);
    if fileattr_has_fsx(fa) || (*fa).flags & !(RUST_SHMEM_SHMEM_FL_USER_MODIFIABLE as c_uint) != 0 {
        return -(RUST_SHMEM_EOPNOTSUPP as c_int);
    }
    let flags = ((*info).fsflags & !(RUST_SHMEM_SHMEM_FL_USER_MODIFIABLE as c_uint))
        | ((*fa).flags & RUST_SHMEM_SHMEM_FL_USER_MODIFIABLE as c_uint);
    let ret = shmem_set_inode_flags(inode, flags, dentry);
    if ret != 0 {
        return ret;
    }
    (*info).fsflags = flags;
    inode_set_ctime_current(inode);
    inode_inc_iversion(inode);
    0
}
#[export_name = "rust_shmem_owner_shmem_initxattrs"]
unsafe extern "C" fn shmem_initxattrs(
    inode: *mut inode,
    array: *const xattr,
    _fs_info: *mut c_void,
) -> c_int {
    let info = SHMEM_I(inode);
    let sb = SHMEM_SB((*inode).i_sb);
    let mut space: usize = 0;
    if (*sb).max_inodes != 0 {
        let mut attr = array;
        while !(*attr).name.is_null() {
            space = space.wrapping_add(simple_xattr_space(
                (*attr).name,
                (*attr)
                    .value_len
                    .wrapping_add(RUST_SHMEM_XATTR_SECURITY_PREFIX_LEN as usize),
            ));
            attr = attr.add(1);
        }
        if space != 0 {
            rust_shmem_raw_spin_lock(addr_of_mut!((*sb).stat_lock));
            if (*sb).free_ispace < space as c_ulong {
                space = 0;
            } else {
                (*sb).free_ispace -= space as c_ulong;
            }
            rust_shmem_raw_spin_unlock(addr_of_mut!((*sb).stat_lock));
            if space == 0 {
                return -(RUST_SHMEM_ENOSPC as c_int);
            }
        }
    }
    let mut attr = array;
    while !(*attr).name.is_null() {
        let new = simple_xattr_alloc((*attr).value, (*attr).value_len);
        if IS_ERR(new.cast()) {
            break;
        }
        (*new).name = kasprintf(
            RUST_SHMEM_GFP_KERNEL_ACCOUNT as gfp_t,
            c"security.%s".as_char_ptr(),
            (*attr).name,
        );
        if (*new).name.is_null() {
            simple_xattr_free(new);
            break;
        }
        if simple_xattr_add(
            addr_of_mut!((*sb).xa_cache),
            addr_of_mut!((*info).xattrs),
            new,
        ) != 0
        {
            simple_xattr_free(new);
            break;
        }
        if (*sb).max_inodes != 0 {
            space = space.wrapping_sub(simple_xattr_space((*new).name, (*new).size));
        }
        attr = attr.add(1);
    }
    if !(*attr).name.is_null() {
        if space != 0 {
            rust_shmem_raw_spin_lock(addr_of_mut!((*sb).stat_lock));
            (*sb).free_ispace = (*sb).free_ispace.wrapping_add(space as c_ulong);
            rust_shmem_raw_spin_unlock(addr_of_mut!((*sb).stat_lock));
        }
        return -(RUST_SHMEM_ENOMEM as c_int);
    }
    rust_shmem_warn_xattr_space(space != 0);
    0
}
#[export_name = "rust_shmem_owner_shmem_xattr_handler_get"]
unsafe extern "C" fn shmem_xattr_handler_get(
    handler: *const xattr_handler,
    _unused: *mut dentry,
    inode: *mut inode,
    name: *const c_char,
    buffer: *mut c_void,
    size: usize,
) -> c_int {
    let sb = SHMEM_SB((*inode).i_sb);
    simple_xattr_get(
        addr_of_mut!((*sb).xa_cache),
        addr_of_mut!((*SHMEM_I(inode)).xattrs),
        xattr_full_name(handler, name),
        buffer,
        size,
    )
}
#[export_name = "rust_shmem_owner_shmem_xattr_handler_set"]
unsafe extern "C" fn shmem_xattr_handler_set(
    handler: *const xattr_handler,
    _idmap: *mut mnt_idmap,
    _unused: *mut dentry,
    inode: *mut inode,
    name: *const c_char,
    value: *const c_void,
    size: usize,
    flags: c_int,
) -> c_int {
    let info = SHMEM_I(inode);
    let sb = SHMEM_SB((*inode).i_sb);
    let name = xattr_full_name(handler, name);
    let mut space: usize = 0;
    if !value.is_null() && (*sb).max_inodes != 0 {
        space = simple_xattr_space(name, size);
        rust_shmem_raw_spin_lock(addr_of_mut!((*sb).stat_lock));
        if (*sb).free_ispace < space as c_ulong {
            space = 0;
        } else {
            (*sb).free_ispace -= space as c_ulong;
        }
        rust_shmem_raw_spin_unlock(addr_of_mut!((*sb).stat_lock));
        if space == 0 {
            return -(RUST_SHMEM_ENOSPC as c_int);
        }
    }
    let mut old = simple_xattr_set(
        addr_of_mut!((*sb).xa_cache),
        addr_of_mut!((*info).xattrs),
        name,
        value,
        size,
        flags,
    );
    if !IS_ERR(old.cast()) {
        space = 0;
        if !old.is_null() && (*sb).max_inodes != 0 {
            space = simple_xattr_space((*old).name, (*old).size);
        }
        simple_xattr_free_rcu(old);
        old = null_mut();
        inode_set_ctime_current(inode);
        inode_inc_iversion(inode);
    }
    if space != 0 {
        rust_shmem_raw_spin_lock(addr_of_mut!((*sb).stat_lock));
        (*sb).free_ispace = (*sb).free_ispace.wrapping_add(space as c_ulong);
        rust_shmem_raw_spin_unlock(addr_of_mut!((*sb).stat_lock));
    }
    ptr_err(old)
}
#[export_name = "rust_shmem_owner_shmem_listxattr"]
unsafe extern "C" fn shmem_listxattr(
    dentry: *mut dentry,
    buffer: *mut c_char,
    size: usize,
) -> ssize_t {
    simple_xattr_list(
        d_inode(dentry),
        addr_of_mut!((*SHMEM_I(d_inode(dentry))).xattrs),
        buffer,
        size,
    )
}
