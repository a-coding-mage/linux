// SPDX-License-Identifier: GPL-2.0-only
#[export_name = "rust_shmem_owner_shmem_put_super"]
unsafe extern "C" fn shmem_put_super(sb: *mut super_block) {
    let info = SHMEM_SB(sb);
    #[cfg(CONFIG_UNICODE)]
    if !(*sb).s_encoding.is_null() {
        utf8_unload((*sb).s_encoding);
    }
    #[cfg(CONFIG_TMPFS_QUOTA)]
    shmem_disable_quotas(sb);
    free_percpu((*info).ino_batch.cast());
    percpu_counter_destroy(addr_of_mut!((*info).used_blocks));
    mpol_put((*info).mpol);
    #[cfg(CONFIG_TMPFS_XATTR)]
    simple_xattr_cache_cleanup(addr_of_mut!((*info).xa_cache));
    kfree(info.cast());
    (*sb).s_fs_info = null_mut();
}
#[export_name = "rust_shmem_owner_shmem_fill_super"]
unsafe extern "C" fn shmem_fill_super(sb: *mut super_block, fc: *mut fs_context) -> c_int {
    let ctx = (*fc).fs_private as *mut shmem_options;
    let info = rust_shmem_alloc_sbinfo();
    let mut error = -(RUST_SHMEM_ENOMEM as c_int);
    if info.is_null() {
        return error;
    }
    (*sb).s_fs_info = info.cast();
    'fill: {
        #[cfg(CONFIG_TMPFS)]
        {
            if (*sb).s_flags & RUST_SHMEM_SB_KERNMOUNT as c_ulong == 0 {
                if (*ctx).seen & RUST_SHMEM_SHMEM_SEEN_BLOCKS == 0 {
                    (*ctx).blocks = shmem_default_max_blocks() as u64;
                }
                if (*ctx).seen & RUST_SHMEM_SHMEM_SEEN_INODES == 0 {
                    (*ctx).inodes = shmem_default_max_inodes() as u64;
                }
                if (*ctx).seen & RUST_SHMEM_SHMEM_SEEN_INUMS == 0 {
                    (*ctx).full_inums = cfg!(CONFIG_TMPFS_INODE64);
                }
                (*info).noswap = (*ctx).noswap;
            } else {
                (*sb).s_flags |= RUST_SHMEM_SB_NOUSER as c_ulong;
            }
            (*sb).s_export_op = addr_of!(rust_shmem_data_shmem_export_ops);
            (*sb).s_flags |= RUST_SHMEM_SB_NOSEC as c_ulong;
            #[cfg(CONFIG_UNICODE)]
            {
                if (*ctx).encoding.is_null() && (*ctx).strict_encoding {
                    rust_shmem_pr_err(
                        c"tmpfs: strict_encoding option without encoding is forbidden\n"
                            .as_char_ptr(),
                    );
                    error = -(RUST_SHMEM_EINVAL as c_int);
                    break 'fill;
                }
                if !(*ctx).encoding.is_null() {
                    (*sb).s_encoding = (*ctx).encoding;
                    set_default_d_op(sb, addr_of!(rust_shmem_data_shmem_ci_dentry_ops));
                    if (*ctx).strict_encoding {
                        (*sb).s_encoding_flags = RUST_SHMEM_SB_ENC_STRICT_MODE_FL as _;
                    }
                }
            }
        }
        #[cfg(not(CONFIG_TMPFS))]
        {
            (*sb).s_flags |= RUST_SHMEM_SB_NOUSER as c_ulong;
        }
        (*sb).s_d_flags |= DCACHE_DONTCACHE as c_uint;
        (*info).max_blocks = (*ctx).blocks as c_ulong;
        (*info).max_inodes = (*ctx).inodes as c_ulong;
        (*info).free_ispace = (*info)
            .max_inodes
            .wrapping_mul(RUST_SHMEM_BOGO_INODE_SIZE as c_ulong);
        if (*sb).s_flags & RUST_SHMEM_SB_KERNMOUNT as c_ulong != 0 {
            (*info).ino_batch = rust_shmem_alloc_ino_batch();
            if (*info).ino_batch.is_null() {
                break 'fill;
            }
        }
        (*info).uid = (*ctx).uid;
        (*info).gid = (*ctx).gid;
        (*info).full_inums = (*ctx).full_inums;
        (*info).mode = (*ctx).mode;
        #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
        {
            (*info).huge = if (*ctx).seen & RUST_SHMEM_SHMEM_SEEN_HUGE != 0 {
                (*ctx).huge as u8
            } else {
                rust_shmem_data_tmpfs_huge as u8
            };
        }
        (*info).mpol = (*ctx).mpol;
        (*ctx).mpol = null_mut();
        rust_shmem_init_stat_lock(info);
        if rust_shmem_init_blocks(info) != 0 {
            break 'fill;
        }
        rust_shmem_init_shrink_lock(info);
        INIT_LIST_HEAD(addr_of_mut!((*info).shrinklist));
        (*sb).s_maxbytes = RUST_SHMEM_MAX_LFS_FILESIZE as loff_t;
        (*sb).s_blocksize = RUST_SHMEM_PAGE_SIZE as c_ulong;
        (*sb).s_blocksize_bits = RUST_SHMEM_PAGE_SHIFT as _;
        (*sb).s_magic = RUST_SHMEM_TMPFS_MAGIC as c_ulong;
        (*sb).s_op = addr_of!(rust_shmem_data_shmem_ops);
        (*sb).s_time_gran = 1;
        #[cfg(CONFIG_TMPFS_XATTR)]
        {
            (*sb).s_xattr = rust_shmem_data_shmem_xattr_handlers.as_ptr();
        }
        #[cfg(CONFIG_TMPFS_POSIX_ACL)]
        {
            (*sb).s_flags |= RUST_SHMEM_SB_POSIXACL as c_ulong;
        }
        let mut uuid = MaybeUninit::<uuid_t>::uninit();
        uuid_gen(uuid.as_mut_ptr());
        super_set_uuid(sb, (*uuid.as_ptr()).b.as_ptr(), size_of::<uuid_t>() as _);
        #[cfg(CONFIG_TMPFS_QUOTA)]
        if (*ctx).seen & RUST_SHMEM_SHMEM_SEEN_QUOTA != 0 {
            (*sb).dq_op = addr_of!(shmem_quota_operations);
            (*sb).s_qcop = addr_of!(dquot_quotactl_sysfile_ops);
            (*sb).s_quota_types = (RUST_SHMEM_QTYPE_MASK_USR | RUST_SHMEM_QTYPE_MASK_GRP) as _;
            core::ptr::copy_nonoverlapping(
                addr_of!((*ctx).qlimits),
                addr_of_mut!((*info).qlimits),
                1,
            );
            if shmem_enable_quotas(sb, (*ctx).quota_types) != 0 {
                break 'fill;
            }
        }
        let inode = shmem_get_inode(
            addr_of_mut!(nop_mnt_idmap),
            sb,
            null_mut(),
            RUST_SHMEM_S_IFDIR as umode_t | (*info).mode,
            0,
            rust_shmem_noreserve_flags(),
        );
        if IS_ERR(inode.cast()) {
            error = ptr_err(inode);
            break 'fill;
        }
        (*inode).i_uid = (*info).uid;
        (*inode).i_gid = (*info).gid;
        (*sb).s_root = d_make_root(inode);
        if (*sb).s_root.is_null() {
            break 'fill;
        }
        return 0;
    }
    shmem_put_super(sb);
    error
}
#[export_name = "rust_shmem_owner_shmem_get_tree"]
unsafe extern "C" fn shmem_get_tree(fc: *mut fs_context) -> c_int {
    get_tree_nodev(fc, Some(shmem_fill_super))
}
#[export_name = "rust_shmem_owner_shmem_free_fc"]
unsafe extern "C" fn shmem_free_fc(fc: *mut fs_context) {
    let ctx = (*fc).fs_private as *mut shmem_options;
    if !ctx.is_null() {
        mpol_put((*ctx).mpol);
        kfree(ctx.cast());
    }
}
#[export_name = "rust_shmem_owner_shmem_alloc_inode"]
unsafe extern "C" fn shmem_alloc_inode(sb: *mut super_block) -> *mut inode {
    let info = rust_shmem_alloc_inode_sb(
        sb,
        rust_shmem_data_shmem_inode_cachep,
        RUST_SHMEM_GFP_KERNEL as gfp_t,
    ) as *mut shmem_inode_info;
    if info.is_null() {
        null_mut()
    } else {
        addr_of_mut!((*info).vfs_inode)
    }
}
#[export_name = "rust_shmem_owner_shmem_free_in_core_inode"]
unsafe extern "C" fn shmem_free_in_core_inode(inode: *mut inode) {
    if rust_shmem_islnk((*inode).i_mode) {
        kfree(rust_shmem_inode_link(inode).cast());
    }
    kmem_cache_free(rust_shmem_data_shmem_inode_cachep, SHMEM_I(inode).cast());
}
#[export_name = "rust_shmem_owner_shmem_destroy_inode"]
unsafe extern "C" fn shmem_destroy_inode(inode: *mut inode) {
    if rust_shmem_isreg((*inode).i_mode) {
        mpol_free_shared_policy(addr_of_mut!((*SHMEM_I(inode)).policy));
    }
    if rust_shmem_isdir((*inode).i_mode) {
        simple_offset_destroy(shmem_get_offset_ctx(inode));
    }
}
#[export_name = "rust_shmem_owner_shmem_init_inode"]
unsafe extern "C" fn shmem_init_inode(foo: *mut c_void) {
    inode_init_once(addr_of_mut!((*(foo as *mut shmem_inode_info)).vfs_inode));
}
#[link_section = ".init.text"]
#[cfg_attr(RUST_SHMEM_INIT_COLD, cold)]
unsafe fn shmem_init_inodecache() {
    rust_shmem_data_shmem_inode_cachep = rust_shmem_kmem_cache_create(
        c"shmem_inode_cache".as_char_ptr(),
        size_of::<shmem_inode_info>() as _,
        0,
        (RUST_SHMEM_SLAB_PANIC | RUST_SHMEM_SLAB_ACCOUNT) as slab_flags_t,
        Some(shmem_init_inode),
    );
}
#[link_section = ".init.text"]
#[cfg_attr(RUST_SHMEM_INIT_COLD, cold)]
unsafe fn shmem_destroy_inodecache() {
    kmem_cache_destroy(rust_shmem_data_shmem_inode_cachep);
}
#[export_name = "rust_shmem_owner_shmem_error_remove_folio"]
unsafe extern "C" fn shmem_error_remove_folio(
    _mapping: *mut address_space,
    _folio: *mut folio,
) -> c_int {
    0
}
#[no_mangle]
pub unsafe extern "C" fn shmem_init_fs_context(fc: *mut fs_context) -> c_int {
    let ctx = rust_shmem_alloc_options();
    if ctx.is_null() {
        return -(RUST_SHMEM_ENOMEM as c_int);
    }
    (*ctx).mode = 0o777 | RUST_SHMEM_S_ISVTX as umode_t;
    (*ctx).uid = rust_shmem_current_fsuid();
    (*ctx).gid = rust_shmem_current_fsgid();
    #[cfg(CONFIG_UNICODE)]
    {
        (*ctx).encoding = null_mut();
    }
    (*fc).fs_private = ctx.cast();
    (*fc).ops = addr_of!(rust_shmem_data_shmem_fs_context_ops);
    #[cfg(CONFIG_TMPFS)]
    {
        (*fc).sb_flags |= RUST_SHMEM_SB_I_VERSION as c_uint;
    }
    0
}
#[cfg(all(CONFIG_SYSFS, CONFIG_TMPFS, CONFIG_UNICODE))]
#[export_name = "rust_shmem_owner_casefold_show"]
unsafe extern "C" fn casefold_show(
    _kobj: *mut kobject,
    _attr: *mut kobj_attribute,
    buf: *mut c_char,
) -> ssize_t {
    sysfs_emit(buf, c"supported\n".as_char_ptr()) as ssize_t
}
#[cfg(all(CONFIG_SYSFS, CONFIG_TMPFS))]
#[link_section = ".init.text"]
#[cfg_attr(RUST_SHMEM_INIT_COLD, cold)]
unsafe fn tmpfs_sysfs_init() -> c_int {
    rust_shmem_data_tmpfs_kobj = kobject_create_and_add(c"tmpfs".as_char_ptr(), fs_kobj);
    if rust_shmem_data_tmpfs_kobj.is_null() {
        return -(RUST_SHMEM_ENOMEM as c_int);
    }
    let ret = sysfs_create_group(
        rust_shmem_data_tmpfs_kobj,
        addr_of!(rust_shmem_data_tmpfs_attribute_group),
    );
    if ret != 0 {
        kobject_put(rust_shmem_data_tmpfs_kobj);
    }
    ret
}
#[no_mangle]
#[link_section = ".init.text"]
#[cfg_attr(RUST_SHMEM_INIT_COLD, cold)]
pub unsafe extern "C" fn shmem_init() {
    shmem_init_inodecache();
    #[cfg(CONFIG_TMPFS_QUOTA)]
    register_quota_format(addr_of_mut!(shmem_quota_format));
    let mut error = register_filesystem(addr_of_mut!(rust_shmem_data_shmem_fs_type));
    'init: {
        if error != 0 {
            rust_shmem_pr_err(c"Could not register tmpfs\n".as_char_ptr());
            break 'init;
        }
        'registered: {
            rust_shmem_data_shm_mnt = kern_mount(addr_of_mut!(rust_shmem_data_shmem_fs_type));
            if IS_ERR(rust_shmem_data_shm_mnt.cast()) {
                error = ptr_err(rust_shmem_data_shm_mnt);
                rust_shmem_pr_err(c"Could not kern_mount tmpfs\n".as_char_ptr());
                break 'registered;
            }
            #[cfg(all(CONFIG_SYSFS, CONFIG_TMPFS))]
            {
                error = tmpfs_sysfs_init();
                if error != 0 {
                    rust_shmem_pr_err(c"Could not init tmpfs sysfs\n".as_char_ptr());
                    break 'registered;
                }
            }
            #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
            {
                if rust_shmem_has_transparent_hugepage()
                    && rust_shmem_data_shmem_huge > RUST_SHMEM_SHMEM_HUGE_DENY
                {
                    (*SHMEM_SB((*rust_shmem_data_shm_mnt).mnt_sb)).huge =
                        rust_shmem_data_shmem_huge as u8;
                } else {
                    rust_shmem_data_shmem_huge = RUST_SHMEM_SHMEM_HUGE_NEVER;
                }
                if !rust_shmem_data_shmem_orders_configured {
                    rust_shmem_data_huge_shmem_orders_inherit = 1 << RUST_SHMEM_HPAGE_PMD_ORDER;
                }
            }
            return;
        }
        unregister_filesystem(addr_of_mut!(rust_shmem_data_shmem_fs_type));
    }
    #[cfg(CONFIG_TMPFS_QUOTA)]
    unregister_quota_format(addr_of_mut!(shmem_quota_format));
    shmem_destroy_inodecache();
    rust_shmem_data_shm_mnt = err_ptr(error);
}
