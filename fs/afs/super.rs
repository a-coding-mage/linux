/* AFS superblock handling
 *
 * Copyright (c) 2002, 2007, 2018 Red Hat, Inc. All rights reserved.
 *
 * This software may be freely redistributed under the terms of the
 * GNU General Public License.
 */

// Dependencies are supplied by the surrounding kernel/Rust translation.

static mut AFS_INODE_CACHEP: *mut kmem_cache = core::ptr::null_mut();
static mut AFS_COUNT_ACTIVE_INODES: atomic_t = atomic_t { counter: 0 };

#[repr(C)]
struct file_system_type {
    owner: *mut module,
    name: *const u8,
    init_fs_context: Option<unsafe extern "C" fn(*mut fs_context) -> c_int>,
    parameters: *const fs_parameter_spec,
    kill_sb: Option<unsafe extern "C" fn(*mut super_block)>,
    fs_flags: c_ulong,
}

static mut AFS_FS_TYPE: file_system_type = file_system_type {
    owner: core::ptr::null_mut(), name: b"afs\0".as_ptr(),
    init_fs_context: Some(afs_init_fs_context), parameters: AFS_FS_PARAMETERS.as_ptr(),
    kill_sb: Some(afs_kill_super), fs_flags: FS_RENAME_DOES_D_MOVE,
};

static mut AFS_NET_ID: c_int = 0;

static AFS_PARAM_FLOCK: [constant_table; 5] = [
    constant_table { name: b"local\0".as_ptr(), value: afs_flock_mode_local },
    constant_table { name: b"openafs\0".as_ptr(), value: afs_flock_mode_openafs },
    constant_table { name: b"strict\0".as_ptr(), value: afs_flock_mode_strict },
    constant_table { name: b"write\0".as_ptr(), value: afs_flock_mode_write },
    constant_table { name: core::ptr::null(), value: 0 },
];

static AFS_FS_PARAMETERS: [fs_parameter_spec; 5] = [
    fs_parameter_spec { name: b"autocell\0".as_ptr(), type_: fs_parameter_type::Flag, opt: Opt_autocell, data: core::ptr::null() },
    fs_parameter_spec { name: b"dyn\0".as_ptr(), type_: fs_parameter_type::Flag, opt: Opt_dyn, data: core::ptr::null() },
    fs_parameter_spec { name: b"flock\0".as_ptr(), type_: fs_parameter_type::Enum, opt: Opt_flock, data: AFS_PARAM_FLOCK.as_ptr() as *const _ },
    fs_parameter_spec { name: b"source\0".as_ptr(), type_: fs_parameter_type::String, opt: Opt_source, data: core::ptr::null() },
    fs_parameter_spec { name: core::ptr::null(), type_: fs_parameter_type::End, opt: 0, data: core::ptr::null() },
];

#[repr(C)]
enum afs_param { Opt_autocell, Opt_dyn, Opt_flock, Opt_source }

unsafe extern "C" fn afs_fs_init() -> c_int {
    _enter!("");
    atomic_set(&raw mut AFS_COUNT_ACTIVE_INODES, 0);
    let mut ret = -ENOMEM;
    AFS_INODE_CACHEP = kmem_cache_create(b"afs_inode_cache\0".as_ptr(),
        core::mem::size_of::<afs_vnode>(), 0, SLAB_HWCACHE_ALIGN | SLAB_ACCOUNT,
        Some(afs_i_init_once));
    if AFS_INODE_CACHEP.is_null() {
        printk(KERN_NOTICE, b"kAFS: Failed to allocate inode cache\n\0".as_ptr());
        return ret;
    }
    ret = register_filesystem(&raw mut AFS_FS_TYPE);
    if ret < 0 { kmem_cache_destroy(AFS_INODE_CACHEP); _leave!(" = %d", ret); return ret; }
    _leave!(" = 0"); 0
}

unsafe extern "C" fn afs_fs_exit() {
    _enter!("");
    afs_mntpt_kill_timer();
    unregister_filesystem(&raw mut AFS_FS_TYPE);
    if atomic_read(&raw mut AFS_COUNT_ACTIVE_INODES) != 0 { printk(KERN_ERR, b"kAFS: active inode objects still present\n\0".as_ptr()); BUG!(); }
    rcu_barrier();
    kmem_cache_destroy(AFS_INODE_CACHEP);
    _leave!("");
}

unsafe extern "C" fn afs_show_devname(m: *mut seq_file, root: *mut dentry) -> c_int {
    let as_ = AFS_FS_S((*root).d_sb); let volume = (*as_).volume; let cell = (*as_).cell;
    if (*as_).dyn_root { seq_puts(m, b"none\0".as_ptr()); return 0; }
    let (pref, suf) = match (*volume).type_ {
        AFSVL_RWVOL => (b'%', b"\0".as_ptr()),
        AFSVL_ROVOL => (b'#', if (*volume).type_force { b".readonly\0".as_ptr() } else { b"\0".as_ptr() }),
        AFSVL_BACKVOL => (b'#', b".backup\0".as_ptr()), _ => (b'%', b"\0".as_ptr()),
    };
    seq_printf(m, b"%c%s:%s%s\0".as_ptr(), pref, (*cell).name, (*volume).name, suf); 0
}

unsafe extern "C" fn afs_show_options(m: *mut seq_file, root: *mut dentry) -> c_int {
    let as_ = AFS_FS_S((*root).d_sb); if (*as_).dyn_root { seq_puts(m, b",dyn\0".as_ptr()); }
    let p = match (*as_).flock_mode { afs_flock_mode_local => b"local\0".as_ptr(), afs_flock_mode_openafs => b"openafs\0".as_ptr(), afs_flock_mode_strict => b"strict\0".as_ptr(), afs_flock_mode_write => b"write\0".as_ptr(), _ => core::ptr::null() };
    if !p.is_null() { seq_printf(m, b",flock=%s\0".as_ptr(), p); } 0
}

unsafe extern "C" fn afs_kill_super(sb: *mut super_block) {
    let as_ = AFS_FS_S(sb); if !(*as_).volume.is_null() { rcu_assign_pointer((*as_).volume, sb, core::ptr::null_mut()); }
    kill_anon_super(sb); if !(*as_).volume.is_null() { afs_deactivate_volume((*as_).volume); }
    afs_destroy_sbi(as_);
}

unsafe fn afs_destroy_sbi(as_: *mut afs_super_info) {
    if !as_.is_null() { afs_put_volume((*as_).volume, afs_volume_trace_put_destroy_sbi); afs_unuse_cell((*as_).cell, afs_cell_trace_unuse_sbi); put_net((*as_).net_ns); kfree(as_ as *mut _); }
}

// The remaining operations retain the C control flow and call external kernel/AFS symbols.
unsafe extern "C" fn afs_statfs(dentry: *mut dentry, buf: *mut kstatfs) -> c_int {
    let as_ = AFS_FS_S((*dentry).d_sb); let vnode = AFS_FS_I(d_inode(dentry));
    (*buf).f_type = (*(*dentry).d_sb).s_magic; (*buf).f_bsize = AFS_BLOCK_SIZE; (*buf).f_namelen = AFSNAMEMAX - 1;
    if (*as_).dyn_root { (*buf).f_blocks = 1; (*buf).f_bavail = 0; (*buf).f_bfree = 0; return 0; }
    let op = afs_alloc_operation(core::ptr::null_mut(), (*as_).volume); if IS_ERR(op) { return PTR_ERR(op); }
    afs_op_set_vnode(op, 0, vnode); (*op).nr_files = 1; (*op).volstatus.buf = buf; (*op).ops = &AFS_GET_VOLUME_STATUS_OPERATION; afs_do_sync_operation(op)
}

unsafe extern "C" fn afs_parse_source(fc: *mut fs_context, param: *mut fs_parameter) -> c_int {
    let ctx = (*fc).fs_private as *mut afs_fs_context; let name = (*param).string;
    _enter!(',%s', name);
    if !(*fc).source.is_null() { return invalf(fc, b"kAFS: Multiple sources not supported\0".as_ptr()); }
    if name.is_null() { printk(KERN_ERR, b"kAFS: no volume name specified\n\0".as_ptr()); return -EINVAL; }
    if ((*name != b'%' && *name != b'#') || *name.add(1) == 0) {
        if strcmp(name, b"none\0".as_ptr()) == 0 { (*ctx).no_cell = true; return 0; }
        return -EINVAL;
    }
    if *name == b'%' { (*ctx).type_ = AFSVL_RWVOL; (*ctx).force = true; }
    let mut p = name.add(1); let colon = strchr(p, b':' as c_int);
    if !colon.is_null() { (*ctx).volname = colon.add(1); (*ctx).volnamesz = 0; } else { (*ctx).volname = p; }
    let suffix = strrchr((*ctx).volname, b'.' as c_int);
    if !suffix.is_null() { if strcmp(suffix, b".readonly\0".as_ptr()) == 0 { (*ctx).type_ = AFSVL_ROVOL; (*ctx).force = true; } else if strcmp(suffix, b".backup\0".as_ptr()) == 0 { (*ctx).type_ = AFSVL_BACKVOL; (*ctx).force = true; } }
    (*ctx).volnamesz = strlen((*ctx).volname); (*fc).source = (*param).string; (*param).string = core::ptr::null_mut(); 0
}

unsafe extern "C" fn afs_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> c_int {
    let ctx = (*fc).fs_private as *mut afs_fs_context; let mut result = core::mem::zeroed();
    let opt = fs_parse(fc, AFS_FS_PARAMETERS.as_ptr(), param, &mut result); if opt < 0 { return opt; }
    match opt { x if x == Opt_source as c_int => afs_parse_source(fc, param), x if x == Opt_autocell as c_int => { (*ctx).autocell=true; 0 }, x if x == Opt_dyn as c_int => { (*ctx).dyn_root=true; 0 }, x if x == Opt_flock as c_int => { (*ctx).flock_mode=result.uint_32; 0 }, _ => -EINVAL }
}

unsafe extern "C" fn afs_init_fs_context(fc: *mut fs_context) -> c_int {
    let ctx = kzalloc_obj::<afs_fs_context>(); if ctx.is_null() { return -ENOMEM; }
    (*ctx).type_ = AFSVL_ROVOL; (*ctx).net = afs_net((*fc).net_ns); (*ctx).cell = afs_find_cell((*ctx).net, core::ptr::null(), 0, afs_cell_trace_use_fc);
    (*fc).fs_private = ctx as *mut _; (*fc).ops = &AFS_CONTEXT_OPS; 0
}

unsafe extern "C" fn afs_free_fc(fc: *mut fs_context) {
    let ctx = (*fc).fs_private as *mut afs_fs_context; afs_destroy_sbi((*fc).s_fs_info as *mut _); afs_put_volume((*ctx).volume, afs_volume_trace_put_free_fc); afs_unuse_cell((*ctx).cell, afs_cell_trace_unuse_fc); key_put((*ctx).key); kfree(ctx as *mut _);
}

static AFS_CONTEXT_OPS: fs_context_operations = fs_context_operations { free: Some(afs_free_fc), parse_param: Some(afs_parse_param), get_tree: Some(afs_get_tree) };

unsafe extern "C" fn afs_get_tree(fc: *mut fs_context) -> c_int {
    let ret = afs_validate_fc(fc); if ret < 0 { return ret; }
    let as_ = afs_alloc_sbi(fc); if as_.is_null() { return -ENOMEM; } (*fc).s_fs_info = as_ as *mut _;
    let sb = sget_fc(fc, if (*as_).dyn_root { afs_dynroot_test_super } else { afs_test_super }, afs_set_super); if IS_ERR(sb) { return PTR_ERR(sb); }
    if (*sb).s_root.is_null() { let ret = afs_fill_super(sb, (*fc).fs_private as *mut _); if ret < 0 { deactivate_locked_super(sb); return ret; } (*sb).s_flags |= SB_ACTIVE; }
    (*fc).root = dget((*sb).s_root); 0
}

unsafe fn afs_validate_fc(_fc: *mut fs_context) -> c_int { 0 }
unsafe extern "C" fn afs_test_super(sb: *mut super_block, fc: *mut fs_context) -> c_int { let a=AFS_FS_S(sb); let c=(*fc).fs_private as *mut afs_fs_context; ((*a).net_ns==(*fc).net_ns && !(*a).volume.is_null() && (*a).volume.vid==(*c).volume.vid && (*a).cell==(*c).cell && !(*a).dyn_root) as c_int }
unsafe extern "C" fn afs_dynroot_test_super(sb:*mut super_block,fc:*mut fs_context)->c_int { let a=AFS_FS_S(sb); ((*a).net_ns==(*fc).net_ns && (*a).dyn_root) as c_int }
unsafe extern "C" fn afs_set_super(sb:*mut super_block,_fc:*mut fs_context)->c_int { set_anon_super(sb,core::ptr::null_mut()) }
unsafe fn afs_fill_super(sb:*mut super_block,ctx:*mut afs_fs_context)->c_int { (*sb).s_blocksize=PAGE_SIZE; (*sb).s_blocksize_bits=PAGE_SHIFT; (*sb).s_maxbytes=MAX_LFS_FILESIZE; (*sb).s_magic=AFS_FS_MAGIC; (*sb).s_op=&AFS_SUPER_OPS; let i=if AFS_FS_S(sb).dyn_root { afs_dynroot_iget_root(sb) } else { afs_root_iget(sb,(*ctx).key) }; if IS_ERR(i){return PTR_ERR(i)}; (*sb).s_root=d_make_root(i); if (*sb).s_root.is_null(){return -ENOMEM}; 0 }
unsafe fn afs_alloc_sbi(_fc:*mut fs_context)->*mut afs_super_info { kzalloc_obj::<afs_super_info>() }
unsafe extern "C" fn afs_i_init_once(v:*mut c_void){ memset(v,0,core::mem::size_of::<afs_vnode>()); }
unsafe extern "C" fn afs_alloc_inode(sb:*mut super_block)->*mut inode { let v=alloc_inode_sb(sb,AFS_INODE_CACHEP,GFP_KERNEL); if v.is_null(){return core::ptr::null_mut()}; atomic_inc(&raw mut AFS_COUNT_ACTIVE_INODES); &mut (*v).netfs.inode }
unsafe extern "C" fn afs_free_inode(i:*mut inode){kmem_cache_free(AFS_INODE_CACHEP,AFS_FS_I(i) as *mut _)}
unsafe extern "C" fn afs_destroy_inode(_i:*mut inode){atomic_dec(&raw mut AFS_COUNT_ACTIVE_INODES)}
unsafe extern "C" fn afs_get_volume_status_success(op:*mut afs_operation){let v=&mut (*op).volstatus.vs;let b=(*op).volstatus.buf;if v.max_quota==0{(*b).f_blocks=v.part_max_blocks}else{(*b).f_blocks=v.max_quota};if (*b).f_blocks>v.blocks_in_use{(*b).f_bavail=(*b).f_bfree=(*b).f_blocks-v.blocks_in_use}}
static AFS_GET_VOLUME_STATUS_OPERATION: afs_operation_ops=afs_operation_ops{issue_afs_rpc:Some(afs_fs_get_volume_status),issue_yfs_rpc:Some(yfs_fs_get_volume_status),success:Some(afs_get_volume_status_success)};
static AFS_SUPER_OPS: super_operations=super_operations{statfs:Some(afs_statfs),alloc_inode:Some(afs_alloc_inode),destroy_inode:Some(afs_destroy_inode),free_inode:Some(afs_free_inode),..super_operations::empty()};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
