// SPDX-License-Identifier: GPL-2.0
/*
 * Super block/filesystem wide operations
 *
 * Copyright (C) 1996 Peter J. Braam <braam@maths.ox.ac.uk> and
 * Michael Callahan <callahan@maths.ox.ac.uk>
 *
 * Rewritten for Linux 2.1.  Peter Braam <braam@cs.cmu.edu>
 * Copyright (C) Carnegie Mellon University
 */

// Linux kernel and Coda dependencies are supplied by the surrounding build.

/* VFS super_block ops */
unsafe fn coda_evict_inode(inode: *mut inode);
unsafe fn coda_put_super(sb: *mut super_block);
unsafe fn coda_statfs(dentry: *mut dentry, buf: *mut kstatfs) -> i32;

static mut coda_inode_cachep: *mut kmem_cache = core::ptr::null_mut();

unsafe fn coda_alloc_inode(sb: *mut super_block) -> *mut inode {
    let ei: *mut coda_inode_info = alloc_inode_sb(sb, coda_inode_cachep, GFP_KERNEL);
    if ei.is_null() {
        return core::ptr::null_mut();
    }
    core::ptr::write_bytes(
        core::ptr::addr_of_mut!((*ei).c_fid) as *mut u8,
        0,
        core::mem::size_of::<CodaFid>(),
    );
    (*ei).c_flags = 0;
    (*ei).c_uid = GLOBAL_ROOT_UID;
    (*ei).c_cached_perm = 0;
    spin_lock_init(core::ptr::addr_of_mut!((*ei).c_lock));
    core::ptr::addr_of_mut!((*ei).vfs_inode)
}

unsafe fn coda_free_inode(inode: *mut inode) {
    kmem_cache_free(coda_inode_cachep, ITOC(inode));
}

unsafe fn init_once(foo: *mut core::ffi::c_void) {
    let ei = foo as *mut coda_inode_info;
    inode_init_once(core::ptr::addr_of_mut!((*ei).vfs_inode));
}

unsafe fn coda_init_inodecache() -> i32 {
    coda_inode_cachep = kmem_cache_create(
        b"coda_inode_cache\0".as_ptr() as *const i8,
        core::mem::size_of::<coda_inode_info>(),
        0,
        SLAB_RECLAIM_ACCOUNT | SLAB_ACCOUNT,
        init_once,
    );
    if coda_inode_cachep.is_null() {
        return -ENOMEM;
    }
    0
}

unsafe fn coda_destroy_inodecache() {
    /* Make sure all delayed rcu free inodes are flushed before we destroy cache. */
    rcu_barrier();
    kmem_cache_destroy(coda_inode_cachep);
}

unsafe fn coda_reconfigure(fc: *mut fs_context) -> i32 {
    sync_filesystem((*(*fc).root).d_sb);
    (*fc).sb_flags |= SB_NOATIME;
    0
}

/* exported operations */
static coda_super_operations: super_operations = super_operations {
    alloc_inode: Some(coda_alloc_inode),
    free_inode: Some(coda_free_inode),
    evict_inode: Some(coda_evict_inode),
    put_super: Some(coda_put_super),
    statfs: Some(coda_statfs),
};

struct coda_fs_context {
    idx: i32,
}

enum {
    Opt_fd,
}

static coda_param_specs: [fs_parameter_spec; 2] = [
    fsparam_fd!(b"fd\0", Opt_fd),
    fs_parameter_spec {},
];

unsafe fn coda_set_idx(fc: *mut fs_context, file: *mut file) -> i32 {
    let ctx = (*fc).fs_private as *mut coda_fs_context;
    let inode = file_inode(file);
    if !S_ISCHR((*inode).i_mode) || imajor(inode) != CODA_PSDEV_MAJOR {
        return invalf(fc, b"coda: Not coda psdev\0".as_ptr() as *const i8);
    }
    let idx = iminor(inode);
    if idx < 0 || idx >= MAX_CODADEVS {
        return invalf(fc, b"coda: Bad minor number\0".as_ptr() as *const i8);
    }
    (*ctx).idx = idx;
    0
}

unsafe fn coda_parse_fd(
    fc: *mut fs_context,
    param: *mut fs_parameter,
    result: *mut fs_parse_result,
) -> i32 {
    let file: *mut file;
    if (*param).type_ == fs_value_is_file {
        file = (*param).file;
        (*param).file = core::ptr::null_mut();
    } else {
        file = fget((*result).uint_32);
    }
    if file.is_null() {
        return -EBADF;
    }
    let err = coda_set_idx(fc, file);
    fput(file);
    err
}

unsafe fn coda_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> i32 {
    let mut result = core::mem::zeroed::<fs_parse_result>();
    let opt = fs_parse(fc, coda_param_specs.as_ptr(), param, &mut result);
    if opt < 0 {
        return opt;
    }
    match opt {
        Opt_fd => coda_parse_fd(fc, param, &mut result),
        _ => 0,
    }
}

/* Parse coda's binary mount data form. We ignore errors and use index 0 for compatibility. */
unsafe fn coda_parse_monolithic(fc: *mut fs_context, data: *mut core::ffi::c_void) -> i32 {
    if data.is_null() {
        return invalf(fc, b"coda: Bad mount data\0".as_ptr() as *const i8);
    }
    let data = data as *mut coda_mount_data;
    if (*data).version != CODA_MOUNT_VERSION {
        return invalf(fc, b"coda: Bad mount version\0".as_ptr() as *const i8);
    }
    let file = fget((*data).fd);
    if !file.is_null() {
        coda_set_idx(fc, file);
        fput(file);
    }
    0
}

unsafe fn coda_fill_super(sb: *mut super_block, fc: *mut fs_context) -> i32 {
    let ctx = (*fc).fs_private as *mut coda_fs_context;
    let mut root: *mut inode = core::ptr::null_mut();
    let mut fid: CodaFid = core::mem::zeroed();
    let vc = &mut coda_comms[(*ctx).idx as usize] as *mut venus_comm;
    let mut error: i32;

    infof(fc, b"coda: device index: %i\n\0".as_ptr() as *const i8, (*ctx).idx);
    mutex_lock(core::ptr::addr_of_mut!((*vc).vc_mutex));
    if !(*vc).vc_inuse {
        errorf(fc, b"coda: No pseudo device\0".as_ptr() as *const i8);
        error = -EINVAL;
        goto_unlock_out!();
    }
    if !(*vc).vc_sb.is_null() {
        errorf(fc, b"coda: Device already mounted\0".as_ptr() as *const i8);
        error = -EBUSY;
        goto_unlock_out!();
    }
    (*vc).vc_sb = sb;
    mutex_unlock(core::ptr::addr_of_mut!((*vc).vc_mutex));
    (*sb).s_fs_info = vc as *mut core::ffi::c_void;
    (*sb).s_flags |= SB_NOATIME;
    (*sb).s_blocksize = 4096;
    (*sb).s_blocksize_bits = 12;
    (*sb).s_magic = CODA_SUPER_MAGIC;
    (*sb).s_op = &coda_super_operations;
    set_default_d_op(sb, &coda_dentry_operations);
    (*sb).s_time_gran = 1;
    (*sb).s_time_min = S64_MIN;
    (*sb).s_time_max = S64_MAX;
    error = super_setup_bdi(sb);
    if error != 0 { goto_error!(); }
    error = venus_rootfid(sb, &mut fid);
    if error != 0 { goto_error!(); }
    root = coda_cnode_make(&fid, sb);
    if IS_ERR(root) { error = PTR_ERR(root); goto_error!(); }
    (*sb).s_root = d_make_root(root);
    if (*sb).s_root.is_null() { error = -EINVAL; goto_error!(); }
    0

    macro_rules! goto_error { () => {{ mutex_lock(core::ptr::addr_of_mut!((*vc).vc_mutex)); (*vc).vc_sb = core::ptr::null_mut(); (*sb).s_fs_info = core::ptr::null_mut(); mutex_unlock(core::ptr::addr_of_mut!((*vc).vc_mutex)); return error; }}; }
    macro_rules! goto_unlock_out { () => {{ mutex_unlock(core::ptr::addr_of_mut!((*vc).vc_mutex)); return error; }}; }
}

unsafe fn coda_put_super(sb: *mut super_block) {
    let vcp = coda_vcp(sb);
    mutex_lock(core::ptr::addr_of_mut!((*vcp).vc_mutex));
    (*vcp).vc_sb = core::ptr::null_mut();
    (*sb).s_fs_info = core::ptr::null_mut();
    mutex_unlock(core::ptr::addr_of_mut!((*vcp).vc_mutex));
    mutex_destroy(core::ptr::addr_of_mut!((*vcp).vc_mutex));
    pr_info!(b"Bye bye.\n");
}

unsafe fn coda_evict_inode(inode: *mut inode) {
    truncate_inode_pages_final(&mut (*inode).i_data);
    clear_inode(inode);
    coda_cache_clear_inode(inode);
}

unsafe fn coda_getattr(idmap: *mut mnt_idmap, path: *const path, stat: *mut kstat, request_mask: u32, flags: u32) -> i32 {
    let err = coda_revalidate_inode(d_inode((*path).dentry));
    if err == 0 { generic_fillattr(&nop_mnt_idmap, request_mask, d_inode((*path).dentry), stat); }
    err
}

unsafe fn coda_setattr(idmap: *mut mnt_idmap, de: *mut dentry, iattr: *mut iattr) -> i32 {
    let inode = d_inode(de);
    let mut vattr: coda_vattr = core::mem::zeroed();
    inode_set_ctime_current(inode);
    coda_iattr_to_vattr(iattr, &mut vattr);
    vattr.va_type = C_VNON;
    let error = venus_setattr((*inode).i_sb, coda_i2f(inode), &mut vattr);
    if error == 0 { coda_vattr_to_iattr(inode, &mut vattr); coda_cache_clear_inode(inode); }
    error
}

const coda_file_inode_operations: inode_operations = inode_operations {
    permission: Some(coda_permission), getattr: Some(coda_getattr), setattr: Some(coda_setattr),
};

unsafe fn coda_statfs(dentry: *mut dentry, buf: *mut kstatfs) -> i32 {
    let error = venus_statfs(dentry, buf);
    if error != 0 { (*buf).f_blocks = 9000000; (*buf).f_bfree = 9000000; (*buf).f_bavail = 9000000; (*buf).f_files = 9000000; (*buf).f_ffree = 9000000; }
    (*buf).f_type = CODA_SUPER_MAGIC; (*buf).f_bsize = 4096; (*buf).f_namelen = CODA_MAXNAMLEN;
    0
}

unsafe fn coda_get_tree(fc: *mut fs_context) -> i32 {
    if task_active_pid_ns(current) != &init_pid_ns { return -EINVAL; }
    get_tree_nodev(fc, coda_fill_super)
}

unsafe fn coda_free_fc(fc: *mut fs_context) { kfree((*fc).fs_private); }

const coda_context_ops: fs_context_operations = fs_context_operations {
    free: Some(coda_free_fc), parse_param: Some(coda_parse_param), parse_monolithic: Some(coda_parse_monolithic), get_tree: Some(coda_get_tree), reconfigure: Some(coda_reconfigure),
};

unsafe fn coda_init_fs_context(fc: *mut fs_context) -> i32 {
    let ctx = kzalloc_obj::<coda_fs_context>();
    if ctx.is_null() { return -ENOMEM; }
    (*fc).fs_private = ctx as *mut core::ffi::c_void;
    (*fc).ops = &coda_context_ops;
    0
}

static mut coda_fs_type: file_system_type = file_system_type {
    owner: THIS_MODULE,
    name: b"coda\0".as_ptr() as *const i8,
    init_fs_context: Some(coda_init_fs_context),
    parameters: coda_param_specs.as_ptr(),
    kill_sb: Some(kill_anon_super),
    fs_flags: FS_BINARY_MOUNTDATA,
};

// MODULE_ALIAS_FS!("coda");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
