// SPDX-License-Identifier: GPL-2.0
// Rust translation of debugfs/inode.c. Kernel dependencies are external.

const DEBUGFS_DEFAULT_MODE: umode_t = 0o700;

static mut debugfs_mount: *mut vfsmount = core::ptr::null_mut();
static mut debugfs_mount_count: i32 = 0;
static mut debugfs_registered: bool = false;
static mut debugfs_enabled: bool = cfg!(feature = "CONFIG_DEBUG_FS_ALLOW_ALL");

unsafe fn debugfs_setattr(idmap: *mut mnt_idmap, dentry: *mut dentry, ia: *mut iattr) -> i32 {
    if (*ia).ia_valid & (ATTR_MODE | ATTR_UID | ATTR_GID) != 0 {
        let ret = security_locked_down(LOCKDOWN_DEBUGFS);
        if ret != 0 { return ret; }
    }
    simple_setattr(&nop_mnt_idmap, dentry, ia)
}

static debugfs_file_inode_operations: inode_operations = inode_operations { setattr: Some(debugfs_setattr), ..inode_operations::default() };
static debugfs_dir_inode_operations: inode_operations = inode_operations { lookup: Some(simple_lookup), setattr: Some(debugfs_setattr), ..inode_operations::default() };
static debugfs_symlink_inode_operations: inode_operations = inode_operations { get_link: Some(simple_get_link), setattr: Some(debugfs_setattr), ..inode_operations::default() };

unsafe fn debugfs_get_inode(sb: *mut super_block) -> *mut inode {
    let inode = new_inode(sb);
    if !inode.is_null() { (*inode).i_ino = get_next_ino(); simple_inode_init_ts(inode); }
    inode
}

#[repr(C)]
struct debugfs_fs_info { uid: kuid_t, gid: kgid_t, mode: umode_t, opts: u32 }

#[repr(i32)]
enum debugfs_opt { Opt_uid, Opt_gid, Opt_mode, Opt_source }

static debugfs_param_specs: [fs_parameter_spec; 5] = [
    fsparam_gid!("gid", Opt_gid), fsparam_u32oct!("mode", Opt_mode),
    fsparam_uid!("uid", Opt_uid), fsparam_string!("source", Opt_source), fsparam_end!()
];

unsafe fn debugfs_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> i32 {
    let opts = (*fc).s_fs_info as *mut debugfs_fs_info;
    let mut result = fs_parse_result::default();
    let opt = fs_parse(fc, &debugfs_param_specs, param, &mut result);
    if opt < 0 { if opt == -ENOPARAM { return 0; } return opt; }
    match opt {
        x if x == Opt_uid as i32 => (*opts).uid = result.uid,
        x if x == Opt_gid as i32 => (*opts).gid = result.gid,
        x if x == Opt_mode as i32 => (*opts).mode = result.uint_32 & S_IALLUGO,
        x if x == Opt_source as i32 => { if !(*fc).source.is_null() { return invalfc(fc, c"Multiple sources specified"); } (*fc).source = (*param).string; (*param).string = core::ptr::null_mut(); },
        _ => {}
    }
    (*opts).opts |= 1u32.wrapping_shl(opt as u32); 0
}

unsafe fn _debugfs_apply_options(sb: *mut super_block, remount: bool) {
    let fsi = (*sb).s_fs_info as *mut debugfs_fs_info;
    let inode = d_inode((*sb).s_root);
    if !remount || (*fsi).opts & (1 << Opt_mode as i32) != 0 { (*inode).i_mode &= !S_IALLUGO; (*inode).i_mode |= (*fsi).mode; }
    if !remount || (*fsi).opts & (1 << Opt_uid as i32) != 0 { (*inode).i_uid = (*fsi).uid; }
    if !remount || (*fsi).opts & (1 << Opt_gid as i32) != 0 { (*inode).i_gid = (*fsi).gid; }
}
unsafe fn debugfs_apply_options(sb: *mut super_block) { _debugfs_apply_options(sb, false); }
unsafe fn debugfs_apply_options_remount(sb: *mut super_block) { _debugfs_apply_options(sb, true); }

unsafe fn debugfs_reconfigure(fc: *mut fs_context) -> i32 {
    let sb = (*(*fc).root).d_sb; let old = (*sb).s_fs_info as *mut debugfs_fs_info; let new = (*fc).s_fs_info as *mut debugfs_fs_info;
    if new.is_null() { return 0; } sync_filesystem(sb); *old = *new; debugfs_apply_options_remount(sb); 0
}

unsafe fn debugfs_show_options(m: *mut seq_file, root: *mut dentry) -> i32 {
    let fsi = (*(*root).d_sb).s_fs_info as *mut debugfs_fs_info;
    if !uid_eq((*fsi).uid, GLOBAL_ROOT_UID) { seq_printf(m, c",uid=%u", from_kuid_munged(&init_user_ns, (*fsi).uid)); }
    if !gid_eq((*fsi).gid, GLOBAL_ROOT_GID) { seq_printf(m, c",gid=%u", from_kgid_munged(&init_user_ns, (*fsi).gid)); }
    if (*fsi).mode != DEBUGFS_DEFAULT_MODE { seq_printf(m, c",mode=%o", (*fsi).mode); } 0
}

static mut debugfs_inode_cachep: *mut kmem_cache = core::ptr::null_mut();
unsafe fn init_once(foo: *mut core::ffi::c_void) { inode_init_once(&mut (*(foo as *mut debugfs_inode_info)).vfs_inode); }
unsafe fn debugfs_alloc_inode(sb: *mut super_block) -> *mut inode { let info = alloc_inode_sb(sb, debugfs_inode_cachep, GFP_KERNEL); if info.is_null() { core::ptr::null_mut() } else { &mut (*(info as *mut debugfs_inode_info)).vfs_inode } }
unsafe fn debugfs_free_inode(inode: *mut inode) { if S_ISLNK((*inode).i_mode) { kfree((*inode).i_link as *mut _); } kmem_cache_free(debugfs_inode_cachep, DEBUGFS_I(inode)); }

unsafe fn debugfs_release_dentry(d: *mut dentry) { let fsd = (*d).d_fsdata as *mut debugfs_fsdata; if !fsd.is_null() { WARN_ON(!list_empty(&(*fsd).cancellations)); mutex_destroy(&mut (*fsd).cancellations_mtx); } kfree(fsd as *mut _); }
unsafe fn debugfs_automount(path: *mut path) -> *mut vfsmount { let inode = (*(*path).dentry).d_inode; (DEBUGFS_I(inode).automount.unwrap())((*path).dentry, (*inode).i_private) }

unsafe fn debugfs_fill_super(sb: *mut super_block, _fc: *mut fs_context) -> i32 { let files = [tree_descr::empty()]; let err = simple_fill_super(sb, DEBUGFS_MAGIC, &files); if err != 0 { return err; } (*sb).s_op = &debugfs_super_operations; set_default_d_op(sb, &debugfs_dops); (*sb).s_d_flags |= DCACHE_DONTCACHE; debugfs_apply_options(sb); 0 }
unsafe fn debugfs_get_tree(fc: *mut fs_context) -> i32 { let err = get_tree_single(fc, debugfs_fill_super); if err != 0 { return err; } debugfs_reconfigure(fc) }
unsafe fn debugfs_free_fc(fc: *mut fs_context) { kfree((*fc).s_fs_info); }
unsafe fn debugfs_init_fs_context(fc: *mut fs_context) -> i32 { let fsi = kzalloc_obj::<debugfs_fs_info>(); if fsi.is_null() { return -ENOMEM; } (*fsi).mode = DEBUGFS_DEFAULT_MODE; (*fc).s_fs_info = fsi as *mut _; (*fc).ops = &debugfs_context_ops; 0 }

pub unsafe fn debugfs_initialized() -> bool { debugfs_registered }

pub unsafe fn debugfs_lookup(name: *const i8, mut parent: *mut dentry) -> *mut dentry { if !debugfs_initialized() || IS_ERR_OR_NULL(name) || IS_ERR(parent) { return core::ptr::null_mut(); } if parent.is_null() { parent = (*debugfs_mount).mnt_root; } let d = lookup_noperm_positive_unlocked(&QSTR(name), parent); if IS_ERR(d) { return core::ptr::null_mut(); } d }

unsafe fn debugfs_start_creating(name: *const i8, mut parent: *mut dentry) -> *mut dentry { if !debugfs_enabled { return ERR_PTR(-EPERM); } if !debugfs_initialized() { pr_err!(c"Unable to create file '%s', debugfs is not initialized yet\n", name); return ERR_PTR(-ENOENT); } pr_debug!(c"creating file '%s'\n", name); if IS_ERR(parent) { return parent; } let e = simple_pin_fs(&debug_fs_type, &mut debugfs_mount, &mut debugfs_mount_count); if e != 0 { return ERR_PTR(e); } if parent.is_null() { parent = (*debugfs_mount).mnt_root; } let d = simple_start_creating(parent, name); if IS_ERR(d) { simple_release_fs(&mut debugfs_mount, &mut debugfs_mount_count); } d }
unsafe fn debugfs_failed_creating(d: *mut dentry) -> *mut dentry { simple_done_creating(d); simple_release_fs(&mut debugfs_mount, &mut debugfs_mount_count); ERR_PTR(-ENOMEM) }
unsafe fn debugfs_end_creating(d: *mut dentry) -> *mut dentry { simple_done_creating(d); d }

pub unsafe fn debugfs_create_file_unsafe(name: *const i8, mode: umode_t, parent: *mut dentry, data: *mut core::ffi::c_void, fops: *const file_operations) -> *mut dentry { __debugfs_create_file(name, mode, parent, data, core::ptr::null(), &debugfs_open_proxy_file_operations, fops) }
pub unsafe fn debugfs_create_file_size(name: *const i8, mode: umode_t, parent: *mut dentry, data: *mut core::ffi::c_void, fops: *const file_operations, file_size: loff_t) { let de = debugfs_create_file(name, mode, parent, data, fops); if !IS_ERR(de) { (*d_inode(de)).i_size = file_size; } }

pub unsafe fn debugfs_create_dir(name: *const i8, parent: *mut dentry) -> *mut dentry { let d = debugfs_start_creating(name, parent); if IS_ERR(d) { return d; } let i = debugfs_get_inode((*d).d_sb); if i.is_null() { return debugfs_failed_creating(d); } (*i).i_mode = S_IFDIR | S_IRWXU | S_IRUGO | S_IXUGO; (*i).i_op = &debugfs_dir_inode_operations; (*i).i_fop = &simple_dir_operations; inc_nlink(i); d_make_persistent(d, i); inc_nlink(d_inode((*d).d_parent)); fsnotify_mkdir(d_inode((*d).d_parent), d); debugfs_end_creating(d) }

pub unsafe fn debugfs_create_symlink(name: *const i8, parent: *mut dentry, target: *const i8) -> *mut dentry { let link = kstrdup(target, GFP_KERNEL); if link.is_null() { return ERR_PTR(-ENOMEM); } let d = debugfs_start_creating(name, parent); if IS_ERR(d) { kfree(link as *mut _); return d; } let i = debugfs_get_inode((*d).d_sb); if i.is_null() { kfree(link as *mut _); return debugfs_failed_creating(d); } (*i).i_mode = S_IFLNK | S_IRWXUGO; (*i).i_op = &debugfs_symlink_inode_operations; (*i).i_link = link; d_make_persistent(d, i); debugfs_end_creating(d) }

unsafe fn remove_one(victim: *mut dentry) { if d_is_reg(victim) { __debugfs_file_removed(victim); } simple_release_fs(&mut debugfs_mount, &mut debugfs_mount_count); }
pub unsafe fn debugfs_remove(d: *mut dentry) { if IS_ERR_OR_NULL(d) { return; } simple_pin_fs(&debug_fs_type, &mut debugfs_mount, &mut debugfs_mount_count); simple_recursive_removal(d, remove_one); simple_release_fs(&mut debugfs_mount, &mut debugfs_mount_count); }
pub unsafe fn debugfs_lookup_and_remove(name: *const i8, parent: *mut dentry) { let d = debugfs_lookup(name, parent); if !d.is_null() { debugfs_remove(d); dput(d); } }

unsafe fn __debugfs_file_removed(d: *mut dentry) { let fsd = READ_ONCE((*d).d_fsdata) as *mut debugfs_fsdata; if fsd.is_null() { return; } while refcount_read(&(*fsd).active_users) != 0 { mutex_lock(&mut (*fsd).cancellations_mtx); while let Some(c) = list_first_entry_or_null(&(*fsd).cancellations, debugfs_cancellation::default()) { list_del_init(&mut (*c).list); ((*c).cancel.unwrap())(d, (*c).cancel_data); } mutex_unlock(&mut (*fsd).cancellations_mtx); wait_for_completion(&(*fsd).active_users_drained); } }

unsafe fn __debugfs_create_file(name: *const i8, mut mode: umode_t, parent: *mut dentry, data: *mut core::ffi::c_void, aux: *const core::ffi::c_void, proxy_fops: *const file_operations, real_fops: *const file_operations) -> *mut dentry {
    if mode & S_IFMT == 0 { mode |= S_IFREG; }
    BUG_ON(!S_ISREG(mode)); let d = debugfs_start_creating(name, parent); if IS_ERR(d) { return d; }
    let i = debugfs_get_inode((*d).d_sb); if i.is_null() { return debugfs_failed_creating(d); }
    (*i).i_mode = mode; (*i).i_private = data; (*i).i_op = &debugfs_file_inode_operations;
    let pf = if real_fops.is_null() { &debugfs_noop_file_operations } else { proxy_fops }; (*i).i_fop = pf;
    (*DEBUGFS_I(i)).raw = real_fops; (*DEBUGFS_I(i)).aux = aux as *mut _;
    d_make_persistent(d, i); fsnotify_create(d_inode((*d).d_parent), d); debugfs_end_creating(d)
}
pub unsafe fn debugfs_create_file_full(name: *const i8, mode: umode_t, parent: *mut dentry, data: *mut core::ffi::c_void, aux: *const core::ffi::c_void, fops: *const file_operations) -> *mut dentry { __debugfs_create_file(name, mode, parent, data, aux, &debugfs_full_proxy_file_operations, fops) }
pub unsafe fn debugfs_create_file_short(name: *const i8, mode: umode_t, parent: *mut dentry, data: *mut core::ffi::c_void, aux: *const core::ffi::c_void, fops: *const debugfs_short_fops) -> *mut dentry { __debugfs_create_file(name, mode, parent, data, aux, &debugfs_full_short_proxy_file_operations, fops as *const _) }

pub unsafe fn debugfs_create_automount(name: *const i8, parent: *mut dentry, f: debugfs_automount_t, data: *mut core::ffi::c_void) -> *mut dentry { let d = debugfs_start_creating(name, parent); if IS_ERR(d) { return d; } let i = debugfs_get_inode((*d).d_sb); if i.is_null() { return debugfs_failed_creating(d); } make_empty_dir_inode(i); (*i).i_flags |= S_AUTOMOUNT; (*i).i_private = data; (*DEBUGFS_I(i)).automount = Some(f); inc_nlink(i); d_make_persistent(d, i); inc_nlink(d_inode((*d).d_parent)); fsnotify_mkdir(d_inode((*d).d_parent), d); debugfs_end_creating(d) }

pub unsafe fn debugfs_change_name(dentry: *mut dentry, _fmt: *const i8, _args: ...) -> i32 { if IS_ERR_OR_NULL(dentry) { return 0; } -ENOSYS }

unsafe fn debugfs_kernel(str_: *mut i8) -> i32 { if !str_.is_null() { if !strcmp(str_, c"on") { debugfs_enabled = true; } else if !strcmp(str_, c"off") || !strcmp(str_, c"no-mount") { debugfs_enabled = false; } } 0 }
unsafe fn debugfs_init() -> i32 { if !debugfs_enabled { return -EPERM; } let r = sysfs_create_mount_point(kernel_kobj, c"debug"); if r != 0 { return r; } debugfs_inode_cachep = kmem_cache_create(c"debugfs_inode_cache", size_of::<debugfs_inode_info>(), 0, SLAB_RECLAIM_ACCOUNT | SLAB_ACCOUNT, init_once); if debugfs_inode_cachep.is_null() { sysfs_remove_mount_point(kernel_kobj, c"debug"); return -ENOMEM; } let r = register_filesystem(&debug_fs_type); if r != 0 { sysfs_remove_mount_point(kernel_kobj, c"debug"); kmem_cache_destroy(debugfs_inode_cachep); return r; } debugfs_registered = true; 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
