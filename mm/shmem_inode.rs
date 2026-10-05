// SPDX-License-Identifier: GPL-2.0-only
#[cfg(CONFIG_TMPFS_XATTR)]
unsafe fn shmem_inode_casefold_flags(
    inode: *mut inode,
    fsflags: c_uint,
    dentry: *mut dentry,
    flags: *mut c_uint,
) -> c_int {
    #[cfg(CONFIG_UNICODE)]
    {
        let old = (*inode).i_flags;
        if fsflags & RUST_SHMEM_FS_CASEFOLD_FL as c_uint != 0 {
            if old & RUST_SHMEM_S_CASEFOLD as c_uint == 0 {
                if (*(*inode).i_sb).s_encoding.is_null() {
                    return -(RUST_SHMEM_EOPNOTSUPP as c_int);
                }
                if !rust_shmem_isdir((*inode).i_mode) {
                    return -(RUST_SHMEM_ENOTDIR as c_int);
                }
                if !dentry.is_null() && !rust_shmem_simple_empty(dentry) {
                    return -(RUST_SHMEM_ENOTEMPTY as c_int);
                }
            }
            *flags |= RUST_SHMEM_S_CASEFOLD as c_uint;
        } else if old & RUST_SHMEM_S_CASEFOLD as c_uint != 0
            && !dentry.is_null()
            && !rust_shmem_simple_empty(dentry)
        {
            return -(RUST_SHMEM_ENOTEMPTY as c_int);
        }
    }
    #[cfg(not(CONFIG_UNICODE))]
    {
        let _ = (inode, dentry, flags);
        if fsflags & RUST_SHMEM_FS_CASEFOLD_FL as c_uint != 0 {
            return -(RUST_SHMEM_EOPNOTSUPP as c_int);
        }
    }
    0
}
unsafe fn shmem_set_inode_flags(inode: *mut inode, fsflags: c_uint, dentry: *mut dentry) -> c_int {
    #[cfg(CONFIG_TMPFS_XATTR)]
    {
        let mut flags = 0;
        let ret = shmem_inode_casefold_flags(inode, fsflags, dentry, &mut flags);
        if ret != 0 {
            return ret;
        }
        if fsflags & RUST_SHMEM_FS_NOATIME_FL as c_uint != 0 {
            flags |= RUST_SHMEM_S_NOATIME as c_uint;
        }
        if fsflags & RUST_SHMEM_FS_APPEND_FL as c_uint != 0 {
            flags |= RUST_SHMEM_S_APPEND as c_uint;
        }
        if fsflags & RUST_SHMEM_FS_IMMUTABLE_FL as c_uint != 0 {
            flags |= RUST_SHMEM_S_IMMUTABLE as c_uint;
        }
        inode_set_flags(
            inode,
            flags,
            (RUST_SHMEM_S_NOATIME
                | RUST_SHMEM_S_APPEND
                | RUST_SHMEM_S_IMMUTABLE
                | RUST_SHMEM_S_CASEFOLD) as c_uint,
        );
    }
    #[cfg(not(CONFIG_TMPFS_XATTR))]
    {
        let _ = (inode, fsflags, dentry);
    }
    0
}
#[export_name = "rust_shmem_owner_shmem_get_offset_ctx"]
unsafe extern "C" fn shmem_get_offset_ctx(inode: *mut inode) -> *mut offset_ctx {
    rust_shmem_inode_offsets(SHMEM_I(inode))
}
unsafe fn __shmem_get_inode(
    idmap: *mut mnt_idmap,
    sb: *mut super_block,
    dir: *mut inode,
    mode: umode_t,
    dev: dev_t,
    flags: vma_flags_t,
) -> *mut inode {
    let sbinfo = SHMEM_SB(sb);
    let mut ino = 0;
    let err = shmem_reserve_inode(sb, &mut ino);
    if err != 0 {
        return err_ptr(err);
    }
    let inode = new_inode(sb);
    if inode.is_null() {
        shmem_free_inode(sb, 0);
        return err_ptr(-(RUST_SHMEM_ENOSPC as c_int));
    }
    (*inode).i_ino = ino as _;
    inode_init_owner(idmap, inode, dir, mode);
    (*inode).i_blocks = 0;
    simple_inode_init_ts(inode);
    (*inode).i_generation = get_random_u32();
    let info = SHMEM_I(inode);
    rust_shmem_zero_inode_prefix(info, inode);
    INIT_LIST_HEAD_RCU(addr_of_mut!((*info).xattrs));
    rust_shmem_init_inode_lock(info);
    atomic_set(addr_of_mut!((*info).stop_eviction), 0);
    (*info).seals = RUST_SHMEM_F_SEAL_SEAL as c_uint;
    (*info).flags = if vma_flags_test(&flags, VMA_NORESERVE_BIT) {
        RUST_SHMEM_SHMEM_F_NORESERVE as c_ulong
    } else {
        0
    };
    (*info).i_crtime = inode_get_mtime(inode);
    (*info).fsflags = if dir.is_null() {
        0
    } else {
        (*SHMEM_I(dir)).fsflags & RUST_SHMEM_SHMEM_FL_INHERITED as c_uint
    };
    if (*info).fsflags != 0 {
        shmem_set_inode_flags(inode, (*info).fsflags, null_mut());
    }
    INIT_LIST_HEAD(rust_shmem_inode_shrinklist(info));
    INIT_LIST_HEAD(rust_shmem_inode_swaplist(info));
    cache_no_acl(inode);
    if (*sbinfo).noswap {
        mapping_set_unevictable((*inode).i_mapping);
    }
    mapping_set_large_folios((*inode).i_mapping);
    match mode as c_uint & RUST_SHMEM_S_IFMT as c_uint {
        x if x == RUST_SHMEM_S_IFREG as c_uint => {
            (*(*inode).i_mapping).a_ops = addr_of!(rust_shmem_data_shmem_aops);
            (*inode).i_op = addr_of!(rust_shmem_data_shmem_inode_operations);
            (*rust_shmem_inode_fop_ptr(inode)) = addr_of!(rust_shmem_data_shmem_file_operations);
            mpol_shared_policy_init(addr_of_mut!((*info).policy), shmem_get_sbmpol(sbinfo));
        }
        x if x == RUST_SHMEM_S_IFDIR as c_uint => {
            inc_nlink(inode);
            (*inode).i_size = 2 * RUST_SHMEM_BOGO_DIRENT_SIZE as loff_t;
            (*inode).i_op = addr_of!(rust_shmem_data_shmem_dir_inode_operations);
            (*rust_shmem_inode_fop_ptr(inode)) = addr_of!(simple_offset_dir_operations);
            simple_offset_init(shmem_get_offset_ctx(inode));
        }
        x if x == RUST_SHMEM_S_IFLNK as c_uint => {
            mpol_shared_policy_init(addr_of_mut!((*info).policy), null_mut());
        }
        _ => {
            (*inode).i_op = addr_of!(rust_shmem_data_shmem_special_inode_operations);
            init_special_inode(inode, mode, dev);
        }
    }
    lockdep_annotate_inode_mutex_key(inode);
    inode
}
unsafe fn shmem_get_inode(
    idmap: *mut mnt_idmap,
    sb: *mut super_block,
    dir: *mut inode,
    mode: umode_t,
    dev: dev_t,
    flags: vma_flags_t,
) -> *mut inode {
    let inode = __shmem_get_inode(idmap, sb, dir, mode, dev, flags);
    #[cfg(CONFIG_TMPFS_QUOTA)]
    {
        if IS_ERR(inode.cast()) {
            return inode;
        }
        let mut err = dquot_initialize(inode);
        if err == 0 {
            err = dquot_alloc_inode(inode);
            if err != 0 {
                dquot_drop(inode);
            }
        }
        if err != 0 {
            (*inode).i_flags |= RUST_SHMEM_S_NOQUOTA as c_uint;
            iput(inode);
            return err_ptr(err);
        }
    }
    inode
}
