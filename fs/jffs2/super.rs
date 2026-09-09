/*
 * JFFS2 -- Journalling Flash File System, Version 2.
 *
 * Copyright © 2001-2007 Red Hat, Inc.
 *
 * Created by David Woodhouse <dwmw2@infradead.org>
 *
 * For licensing information, see the file 'LICENCE' in this directory.
 */

/* C headers and build-time configuration are supplied by the surrounding crate. */

unsafe extern "C" {
    fn jffs2_iget(sb: *mut super_block, ino: u64) -> *mut inode;
    fn jffs2_statfs(sb: *mut super_block, buf: *mut kstatfs) -> i32;
    fn jffs2_evict_inode(inode: *mut inode);
    fn jffs2_dirty_inode(inode: *mut inode, flags: i32);
}

static mut jffs2_inode_cachep: *mut kmem_cache = core::ptr::null_mut();

unsafe fn jffs2_alloc_inode(sb: *mut super_block) -> *mut inode {
    let f: *mut jffs2_inode_info = alloc_inode_sb(sb, jffs2_inode_cachep, GFP_KERNEL);
    if f.is_null() { return core::ptr::null_mut(); }
    &mut (*f).vfs_inode
}

unsafe fn jffs2_free_inode(inode: *mut inode) {
    let f: *mut jffs2_inode_info = JFFS2_INODE_INFO(inode);
    kfree((*f).target);
    kmem_cache_free(jffs2_inode_cachep, f as *mut core::ffi::c_void);
}

unsafe fn jffs2_i_init_once(foo: *mut core::ffi::c_void) {
    let f = foo as *mut jffs2_inode_info;
    mutex_init(&mut (*f).sem);
    (*f).target = core::ptr::null_mut();
    inode_init_once(&mut (*f).vfs_inode);
}

unsafe fn jffs2_compr_name(compr: u32) -> *const u8 {
    match compr {
        JFFS2_COMPR_MODE_NONE => b"none\0".as_ptr(),
        /* CONFIG_JFFS2_LZO: JFFS2_COMPR_MODE_FORCELZO => b"lzo\0".as_ptr(), */
        /* CONFIG_JFFS2_ZLIB: JFFS2_COMPR_MODE_FORCEZLIB => b"zlib\0".as_ptr(), */
        _ => { WARN_ON(1); b"\0".as_ptr() }
    }
}

unsafe fn jffs2_show_options(s: *mut seq_file, root: *mut dentry) -> i32 {
    let c = JFFS2_SB_INFO((*(*root).d_sb));
    let opts = &mut (*c).mount_opts;
    if opts.override_compr { seq_printf(s, b",compr=%s\0".as_ptr(), jffs2_compr_name(opts.compr)); }
    if opts.set_rp_size { seq_printf(s, b",rp_size=%u\0".as_ptr(), opts.rp_size / 1024); }
    0
}

unsafe fn jffs2_sync_fs(sb: *mut super_block, _wait: i32) -> i32 {
    let c = JFFS2_SB_INFO(sb);
    /* CONFIG_JFFS2_FS_WRITEBUFFER: if (jffs2_is_writebuffered(c)) cancel_delayed_work_sync(&mut (*c).wbuf_dwork); */
    mutex_lock(&mut (*c).alloc_sem);
    jffs2_flush_wbuf_pad(c);
    mutex_unlock(&mut (*c).alloc_sem);
    0
}

unsafe fn jffs2_nfs_get_inode(sb: *mut super_block, ino: u64, _generation: u32) -> *mut inode { jffs2_iget(sb, ino) }

unsafe fn jffs2_fh_to_dentry(sb: *mut super_block, fid: *mut fid, fh_len: i32, fh_type: i32) -> *mut dentry {
    generic_fh_to_dentry(sb, fid, fh_len, fh_type, Some(jffs2_nfs_get_inode))
}
unsafe fn jffs2_fh_to_parent(sb: *mut super_block, fid: *mut fid, fh_len: i32, fh_type: i32) -> *mut dentry {
    generic_fh_to_parent(sb, fid, fh_len, fh_type, Some(jffs2_nfs_get_inode))
}
unsafe fn jffs2_get_parent(child: *mut dentry) -> *mut dentry {
    BUG_ON(!d_is_dir(child));
    let f = JFFS2_INODE_INFO(d_inode(child));
    let pino = (*(*f).inocache).pino_nlink;
    JFFS2_DEBUG(b"Parent of directory ino #%u is #%u\n\0".as_ptr(), (*(*f).inocache).ino, pino);
    d_obtain_alias(jffs2_iget((*child).d_sb, pino as u64))
}

static mut jffs2_export_ops: export_operations = export_operations {
    encode_fh: Some(generic_encode_ino32_fh), get_parent: Some(jffs2_get_parent),
    fh_to_dentry: Some(jffs2_fh_to_dentry), fh_to_parent: Some(jffs2_fh_to_parent),
};

enum { Opt_override_compr, Opt_rp_size }

static jffs2_param_compr: [constant_table; 1] = [constant_table { name: core::ptr::null(), value: 0 }];
static jffs2_fs_parameters: [fs_parameter_spec; 1] = [fs_parameter_spec { _private: 0 }];

unsafe fn jffs2_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> i32 {
    let mut result = fs_parse_result { uint_32: 0 };
    let c = (*fc).s_fs_info as *mut jffs2_sb_info;
    let opt = fs_parse(fc, jffs2_fs_parameters.as_ptr(), param, &mut result);
    if opt < 0 { return opt; }
    match opt {
        Opt_override_compr => { (*c).mount_opts.compr = result.uint_32; (*c).mount_opts.override_compr = true; }
        Opt_rp_size => { if result.uint_32 > u32::MAX / 1024 { return invalf(fc, b"jffs2: rp_size unrepresentable\0".as_ptr()); } (*c).mount_opts.rp_size = result.uint_32 * 1024; (*c).mount_opts.set_rp_size = true; }
        _ => return -EINVAL,
    }
    0
}

unsafe fn jffs2_update_mount_opts(fc: *mut fs_context) {
    let new_c = (*fc).s_fs_info as *mut jffs2_sb_info;
    let c = JFFS2_SB_INFO((*(*fc).root).d_sb);
    mutex_lock(&mut (*c).alloc_sem);
    if (*new_c).mount_opts.override_compr { (*c).mount_opts.override_compr = true; (*c).mount_opts.compr = (*new_c).mount_opts.compr; }
    if (*new_c).mount_opts.set_rp_size { (*c).mount_opts.set_rp_size = true; (*c).mount_opts.rp_size = (*new_c).mount_opts.rp_size; }
    mutex_unlock(&mut (*c).alloc_sem);
}

unsafe fn jffs2_reconfigure(fc: *mut fs_context) -> i32 { let sb = (*(*fc).root).d_sb; sync_filesystem(sb); jffs2_update_mount_opts(fc); jffs2_do_remount_fs(sb, fc) }

static mut jffs2_super_operations: super_operations = super_operations {
    alloc_inode: Some(jffs2_alloc_inode), free_inode: Some(jffs2_free_inode), put_super: Some(jffs2_put_super),
    statfs: Some(jffs2_statfs), evict_inode: Some(jffs2_evict_inode), dirty_inode: Some(jffs2_dirty_inode),
    show_options: Some(jffs2_show_options), sync_fs: Some(jffs2_sync_fs),
};

unsafe fn jffs2_fill_super(sb: *mut super_block, fc: *mut fs_context) -> i32 {
    let c = (*sb).s_fs_info as *mut jffs2_sb_info;
    (*c).mtd = (*sb).s_mtd; (*c).os_priv = sb as *mut core::ffi::c_void;
    if (*c).mount_opts.rp_size > (*(*c).mtd).size { return invalf(fc, b"jffs2: Too large reserve pool specified, max is %llu KB\0".as_ptr(), (*(*c).mtd).size / 1024); }
    mutex_init(&mut (*c).alloc_sem); mutex_init(&mut (*c).erase_free_sem); init_waitqueue_head(&mut (*c).erase_wait); init_waitqueue_head(&mut (*c).inocache_wq); spin_lock_init(&mut (*c).erase_completion_lock); spin_lock_init(&mut (*c).inocache_lock);
    (*sb).s_op = &raw mut jffs2_super_operations; (*sb).s_export_op = &raw mut jffs2_export_ops; (*sb).s_flags |= SB_NOATIME; (*sb).s_xattr = jffs2_xattr_handlers;
    /* CONFIG_JFFS2_FS_POSIX_ACL: (*sb).s_flags |= SB_POSIXACL; */
    jffs2_do_fill_super(sb, fc)
}
unsafe fn jffs2_get_tree(fc: *mut fs_context) -> i32 { get_tree_mtd(fc, Some(jffs2_fill_super)) }
unsafe fn jffs2_free_fc(fc: *mut fs_context) { kfree((*fc).s_fs_info); }
static mut jffs2_context_ops: fs_context_operations = fs_context_operations { free: Some(jffs2_free_fc), parse_param: Some(jffs2_parse_param), get_tree: Some(jffs2_get_tree), reconfigure: Some(jffs2_reconfigure) };
unsafe fn jffs2_init_fs_context(fc: *mut fs_context) -> i32 { let ctx = kzalloc_obj::<jffs2_sb_info>(); if ctx.is_null() { return -ENOMEM; } (*fc).s_fs_info = ctx as *mut core::ffi::c_void; (*fc).ops = &raw mut jffs2_context_ops; 0 }

unsafe fn jffs2_put_super(sb: *mut super_block) {
    let c = JFFS2_SB_INFO(sb); mutex_lock(&mut (*c).alloc_sem); jffs2_flush_wbuf_pad(c); mutex_unlock(&mut (*c).alloc_sem); jffs2_sum_exit(c); jffs2_free_ino_caches(c); jffs2_free_raw_node_refs(c); kvfree((*c).blocks); jffs2_flash_cleanup(c); kfree((*c).inocache_list); jffs2_clear_xattr_subsystem(c); mtd_sync((*c).mtd);
}
unsafe fn jffs2_kill_sb(sb: *mut super_block) { let c = JFFS2_SB_INFO(sb); if !c.is_null() && !sb_rdonly(sb) { jffs2_stop_garbage_collect_thread(c); } kill_mtd_super(sb); kfree(c); }

static mut jffs2_fs_type: file_system_type = file_system_type { owner: THIS_MODULE, name: b"jffs2\0".as_ptr(), init_fs_context: Some(jffs2_init_fs_context), parameters: jffs2_fs_parameters.as_ptr(), kill_sb: Some(jffs2_kill_sb) };

unsafe fn init_jffs2_fs() -> i32 {
    BUILD_BUG_ON(core::mem::size_of::<jffs2_unknown_node>() != 12); BUILD_BUG_ON(core::mem::size_of::<jffs2_raw_dirent>() != 40); BUILD_BUG_ON(core::mem::size_of::<jffs2_raw_inode>() != 68); BUILD_BUG_ON(core::mem::size_of::<jffs2_raw_summary>() != 32);
    jffs2_inode_cachep = kmem_cache_create(b"jffs2_i\0".as_ptr(), core::mem::size_of::<jffs2_inode_info>(), 0, SLAB_RECLAIM_ACCOUNT | SLAB_ACCOUNT, Some(jffs2_i_init_once));
    if jffs2_inode_cachep.is_null() { return -ENOMEM; }
    let mut ret = jffs2_compressors_init(); if ret != 0 { kmem_cache_destroy(jffs2_inode_cachep); return ret; }
    ret = jffs2_create_slab_caches(); if ret != 0 { jffs2_compressors_exit(); kmem_cache_destroy(jffs2_inode_cachep); return ret; }
    ret = register_filesystem(&raw mut jffs2_fs_type); if ret != 0 { jffs2_destroy_slab_caches(); jffs2_compressors_exit(); kmem_cache_destroy(jffs2_inode_cachep); return ret; } 0
}
unsafe fn exit_jffs2_fs() { unregister_filesystem(&raw mut jffs2_fs_type); jffs2_destroy_slab_caches(); jffs2_compressors_exit(); rcu_barrier(); kmem_cache_destroy(jffs2_inode_cachep); }

/* module_init(init_jffs2_fs); module_exit(exit_jffs2_fs); */
/* MODULE_DESCRIPTION("The Journalling Flash File System, v2"); */
/* MODULE_AUTHOR("Red Hat, Inc."); MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
