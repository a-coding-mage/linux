// SPDX-License-Identifier: MIT
/*
 * VirtualBox Guest Shared Folders support: Virtual File System.
 *
 * Module initialization/finalization
 * File system registration/deregistration
 * Superblock reading
 * Few utility functions
 *
 * Copyright (C) 2006-2018 Oracle Corporation
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

pub const VBOXSF_SUPER_MAGIC: u32 = 0x786f4256; // 'VBox' little endian

static VBSF_MOUNT_SIGNATURE: [u8; 4] = [0, 0xff, 0xfe, 0xfd];

static mut follow_symlinks: ::core::ffi::c_int = 0;

static mut vboxsf_bdi_ida: ida = DEFINE_IDA!();
static mut vboxsf_setup_mutex: mutex = DEFINE_MUTEX!();
static mut vboxsf_setup_done: bool = false;
static mut vboxsf_super_ops: super_operations = unsafe { core::mem::zeroed() };
static mut vboxsf_inode_cachep: *mut kmem_cache = core::ptr::null_mut();

static mut vboxsf_default_nls: *mut ::core::ffi::c_char = CONFIG_NLS_DEFAULT;

#[repr(i32)]
enum VboxsfOption {
    OptNls,
    OptUid,
    OptGid,
    OptTtl,
    OptDmode,
    OptFmode,
    OptDmask,
    OptFmask,
}

static vboxsf_fs_parameters: [fs_parameter_spec; 9] = [
    fsparam_string!("nls", VboxsfOption::OptNls),
    fsparam_uid!("uid", VboxsfOption::OptUid),
    fsparam_gid!("gid", VboxsfOption::OptGid),
    fsparam_u32!("ttl", VboxsfOption::OptTtl),
    fsparam_u32oct!("dmode", VboxsfOption::OptDmode),
    fsparam_u32oct!("fmode", VboxsfOption::OptFmode),
    fsparam_u32oct!("dmask", VboxsfOption::OptDmask),
    fsparam_u32oct!("fmask", VboxsfOption::OptFmask),
    fs_parameter_spec::default(),
];

unsafe fn vboxsf_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> i32 {
    let ctx = (*fc).fs_private as *mut vboxsf_fs_context;
    let mut result: fs_parse_result = core::mem::zeroed();
    let opt = fs_parse(fc, vboxsf_fs_parameters.as_ptr(), param, &mut result);
    if opt < 0 { return opt; }

    match opt {
        x if x == VboxsfOption::OptNls as i32 => {
            if !(*ctx).nls_name.is_null() || (*fc).purpose != FS_CONTEXT_FOR_MOUNT {
                vbg_err!("vboxsf: Cannot reconfigure nls option\n");
                return -EINVAL;
            }
            (*ctx).nls_name = (*param).string;
            (*param).string = core::ptr::null_mut();
        }
        x if x == VboxsfOption::OptUid as i32 => (*ctx).o.uid = result.uid,
        x if x == VboxsfOption::OptGid as i32 => (*ctx).o.gid = result.gid,
        x if x == VboxsfOption::OptTtl as i32 => (*ctx).o.ttl = msecs_to_jiffies(result.uint_32),
        x if x == VboxsfOption::OptDmode as i32 => {
            if result.uint_32 & !0o777 != 0 { return -EINVAL; }
            (*ctx).o.dmode = result.uint_32;
            (*ctx).o.dmode_set = true;
        }
        x if x == VboxsfOption::OptFmode as i32 => {
            if result.uint_32 & !0o777 != 0 { return -EINVAL; }
            (*ctx).o.fmode = result.uint_32;
            (*ctx).o.fmode_set = true;
        }
        x if x == VboxsfOption::OptDmask as i32 => {
            if result.uint_32 & !0o7777 != 0 { return -EINVAL; }
            (*ctx).o.dmask = result.uint_32;
        }
        x if x == VboxsfOption::OptFmask as i32 => {
            if result.uint_32 & !0o7777 != 0 { return -EINVAL; }
            (*ctx).o.fmask = result.uint_32;
        }
        _ => return -EINVAL,
    }
    0
}

unsafe fn vboxsf_fill_super(sb: *mut super_block, fc: *mut fs_context) -> i32 {
    let ctx = (*fc).fs_private as *mut vboxsf_fs_context;
    let mut folder_name: *mut shfl_string;
    let mut root_path: shfl_string = core::mem::zeroed();
    let mut sbi: *mut vboxsf_sbi;
    let mut droot: *mut dentry;
    let mut iroot: *mut inode;
    let mut nls_name: *mut ::core::ffi::c_char;
    let mut size: usize;
    let mut err: i32;

    if (*fc).source.is_null() { return -EINVAL; }
    sbi = kzalloc_obj!(*sbi);
    if sbi.is_null() { return -ENOMEM; }
    (*sbi).o = (*ctx).o;
    idr_init(&mut (*sbi).ino_idr);
    spin_lock_init(&mut (*sbi).ino_idr_lock);
    (*sbi).next_generation = 1;
    (*sbi).bdi_id = -1;

    nls_name = if !(*ctx).nls_name.is_null() { (*ctx).nls_name } else { vboxsf_default_nls };
    if strcmp(nls_name, c"utf8".as_ptr()) != 0 {
        (*sbi).nls = if nls_name == vboxsf_default_nls { load_nls_default() } else { load_nls(nls_name) };
        if (*sbi).nls.is_null() {
            vbg_err!("vboxsf: Count not load '%s' nls\n", nls_name);
            err = -EINVAL;
            goto fail_destroy_idr;
        }
    }
    (*sbi).bdi_id = ida_alloc(&mut vboxsf_bdi_ida, GFP_KERNEL);
    if (*sbi).bdi_id < 0 { err = (*sbi).bdi_id; goto fail_free; }
    err = super_setup_bdi_name(sb, c"vboxsf-%d".as_ptr(), (*sbi).bdi_id);
    if err != 0 { goto fail_free; }
    (*(*sb).s_bdi).ra_pages = 0;
    (*(*sb).s_bdi).io_pages = 0;

    size = strlen((*fc).source) + 1;
    folder_name = kmalloc(SHFLSTRING_HEADER_SIZE + size, GFP_KERNEL);
    if folder_name.is_null() { err = -ENOMEM; goto fail_free; }
    (*folder_name).size = size;
    (*folder_name).length = size - 1;
    strscpy((*folder_name).string.utf8.as_mut_ptr(), (*fc).source, size);
    err = vboxsf_map_folder(folder_name, &mut (*sbi).root);
    kfree(folder_name as *mut core::ffi::c_void);
    if err != 0 { vbg_err!("vboxsf: Host rejected mount of '%s' with error %d\n", (*fc).source, err); goto fail_free; }

    root_path.length = 1;
    root_path.size = 2;
    root_path.string.utf8[0] = b'/';
    root_path.string.utf8[1] = 0;
    err = vboxsf_stat(sbi, &root_path, &mut (*sbi).root_info);
    if err != 0 { goto fail_unmap; }
    // A failed query is advisory; preserve the default case-sensitive behavior.
    vboxsf_query_case_sensitive(sbi);
    (*sb).s_magic = VBOXSF_SUPER_MAGIC;
    (*sb).s_blocksize = 1024;
    (*sb).s_maxbytes = MAX_LFS_FILESIZE;
    (*sb).s_op = &raw mut vboxsf_super_ops;
    set_default_d_op(sb, &vboxsf_dentry_ops);
    iroot = iget_locked(sb, 0);
    if iroot.is_null() { err = -ENOMEM; goto fail_unmap; }
    vboxsf_init_inode(sbi, iroot, &(*sbi).root_info, false);
    unlock_new_inode(iroot);
    droot = d_make_root(iroot);
    if droot.is_null() { err = -ENOMEM; goto fail_unmap; }
    (*sb).s_root = droot;
    (*sb).s_fs_info = sbi as *mut core::ffi::c_void;
    return 0;

fail_unmap:
    vboxsf_unmap_folder((*sbi).root);
fail_free:
    if (*sbi).bdi_id >= 0 { ida_free(&mut vboxsf_bdi_ida, (*sbi).bdi_id); }
    if !(*sbi).nls.is_null() { unload_nls((*sbi).nls); }
fail_destroy_idr:
    idr_destroy(&mut (*sbi).ino_idr);
    kfree(sbi as *mut core::ffi::c_void);
    err
}

unsafe fn vboxsf_inode_init_once(data: *mut core::ffi::c_void) {
    let sf_i = data as *mut vboxsf_inode;
    mutex_init(&mut (*sf_i).handle_list_mutex);
    inode_init_once(&mut (*sf_i).vfs_inode);
}

unsafe fn vboxsf_alloc_inode(sb: *mut super_block) -> *mut inode {
    let sf_i = alloc_inode_sb(sb, vboxsf_inode_cachep, GFP_NOFS);
    if sf_i.is_null() { return core::ptr::null_mut(); }
    (*sf_i).force_restat = 0;
    INIT_LIST_HEAD(&mut (*sf_i).handle_list);
    &mut (*sf_i).vfs_inode
}

unsafe fn vboxsf_free_inode(inode: *mut inode) {
    let sbi = VBOXSF_SBI((*inode).i_sb);
    let mut flags: ulong = 0;
    spin_lock_irqsave(&mut (*sbi).ino_idr_lock, &mut flags);
    idr_remove(&mut (*sbi).ino_idr, (*inode).i_ino);
    spin_unlock_irqrestore(&mut (*sbi).ino_idr_lock, flags);
    kmem_cache_free(vboxsf_inode_cachep, VBOXSF_I(inode));
}

unsafe fn vboxsf_put_super(sb: *mut super_block) {
    let sbi = VBOXSF_SBI(sb);
    vboxsf_unmap_folder((*sbi).root);
    if (*sbi).bdi_id >= 0 { ida_free(&mut vboxsf_bdi_ida, (*sbi).bdi_id); }
    if !(*sbi).nls.is_null() { unload_nls((*sbi).nls); }
    rcu_barrier();
    idr_destroy(&mut (*sbi).ino_idr);
    kfree(sbi as *mut core::ffi::c_void);
}

unsafe fn vboxsf_statfs(dentry: *mut dentry, stat: *mut kstatfs) -> i32 {
    let sb = (*dentry).d_sb;
    let mut vi: shfl_volinfo = core::mem::zeroed();
    let sbi = VBOXSF_SBI(sb);
    let mut buf_len = core::mem::size_of::<shfl_volinfo>() as u32;
    let err = vboxsf_fsinfo((*sbi).root, 0, SHFL_INFO_GET | SHFL_INFO_VOLUME, &mut buf_len, &mut vi);
    if err != 0 { return err; }
    (*stat).f_type = VBOXSF_SUPER_MAGIC;
    (*stat).f_bsize = vi.bytes_per_allocation_unit;
    vi.total_allocation_bytes /= vi.bytes_per_allocation_unit;
    (*stat).f_blocks = vi.total_allocation_bytes;
    vi.available_allocation_bytes /= vi.bytes_per_allocation_unit;
    (*stat).f_bfree = vi.available_allocation_bytes;
    (*stat).f_bavail = vi.available_allocation_bytes;
    (*stat).f_files = 1000;
    (*stat).f_ffree = 1000000;
    (*stat).f_fsid.val[0] = 0;
    (*stat).f_fsid.val[1] = 0;
    (*stat).f_namelen = 255;
    0
}

static mut vboxsf_super_ops: super_operations = super_operations {
    alloc_inode: Some(vboxsf_alloc_inode),
    free_inode: Some(vboxsf_free_inode),
    put_super: Some(vboxsf_put_super),
    statfs: Some(vboxsf_statfs),
    ..unsafe { core::mem::zeroed() }
};

unsafe fn vboxsf_setup() -> i32 {
    let mut err: i32;
    mutex_lock(&mut vboxsf_setup_mutex);
    if vboxsf_setup_done { goto_success!(); }
    vboxsf_inode_cachep = kmem_cache_create(c"vboxsf_inode_cache".as_ptr(), core::mem::size_of::<vboxsf_inode>(), 0, SLAB_RECLAIM_ACCOUNT | SLAB_ACCOUNT, Some(vboxsf_inode_init_once));
    if vboxsf_inode_cachep.is_null() { err = -ENOMEM; goto fail_nomem; }
    err = vboxsf_connect();
    if err != 0 { vbg_err!("vboxsf: err %d connecting to guest PCI-device\n", err); vbg_err!("vboxsf: make sure you are inside a VirtualBox VM\n"); vbg_err!("vboxsf: and check dmesg for vboxguest errors\n"); goto fail_free_cache; }
    err = vboxsf_set_utf8();
    if err != 0 { vbg_err!("vboxsf_setutf8 error %d\n", err); goto fail_disconnect; }
    if !follow_symlinks { err = vboxsf_set_symlinks(); if err != 0 { vbg_warn!("vboxsf: Unable to show symlinks: %d\n", err); } }
    vboxsf_setup_done = true;
success:
    mutex_unlock(&mut vboxsf_setup_mutex); return 0;
fail_disconnect:
    vboxsf_disconnect();
fail_free_cache:
    kmem_cache_destroy(vboxsf_inode_cachep);
fail_nomem:
    mutex_unlock(&mut vboxsf_setup_mutex); return err;
}

unsafe fn vboxsf_parse_monolithic(fc: *mut fs_context, data: *mut core::ffi::c_void) -> i32 {
    if !data.is_null() && memcmp(data, VBSF_MOUNT_SIGNATURE.as_ptr() as *const _, 4) == 0 { vbg_err!("vboxsf: Old binary mount data not supported, remove obsolete mount.vboxsf and/or update your VBoxService.\n"); return -EINVAL; }
    generic_parse_monolithic(fc, data)
}

unsafe fn vboxsf_get_tree(fc: *mut fs_context) -> i32 { let err = vboxsf_setup(); if err != 0 { return err; } get_tree_nodev(fc, Some(vboxsf_fill_super)) }

unsafe fn vboxsf_reconfigure(fc: *mut fs_context) -> i32 {
    let sbi = VBOXSF_SBI((*(*fc).root).d_sb);
    let ctx = (*fc).fs_private as *mut vboxsf_fs_context;
    let iroot = (*(*(*fc).root).d_sb).s_root.cast::<dentry>().as_ref().unwrap().d_inode;
    (*sbi).o = (*ctx).o;
    vboxsf_init_inode(sbi, iroot, &(*sbi).root_info, true);
    0
}

unsafe fn vboxsf_free_fc(fc: *mut fs_context) { let ctx = (*fc).fs_private as *mut vboxsf_fs_context; kfree((*ctx).nls_name as *mut _); kfree(ctx as *mut _); }

static vboxsf_context_ops: fs_context_operations = fs_context_operations { free: Some(vboxsf_free_fc), parse_param: Some(vboxsf_parse_param), parse_monolithic: Some(vboxsf_parse_monolithic), get_tree: Some(vboxsf_get_tree), reconfigure: Some(vboxsf_reconfigure), ..unsafe { core::mem::zeroed() } };

unsafe fn vboxsf_init_fs_context(fc: *mut fs_context) -> i32 {
    let ctx = kzalloc_obj!(*ctx);
    if ctx.is_null() { return -ENOMEM; }
    current_uid_gid(&mut (*ctx).o.uid, &mut (*ctx).o.gid);
    (*fc).fs_private = ctx as *mut _;
    (*fc).ops = &vboxsf_context_ops;
    0
}

static mut vboxsf_fs_type: file_system_type = file_system_type { owner: THIS_MODULE, name: c"vboxsf".as_ptr(), init_fs_context: Some(vboxsf_init_fs_context), kill_sb: Some(kill_anon_super), ..unsafe { core::mem::zeroed() } };

unsafe fn vboxsf_init() -> i32 { register_filesystem(&mut vboxsf_fs_type) }

unsafe fn vboxsf_fini() {
    unregister_filesystem(&mut vboxsf_fs_type);
    mutex_lock(&mut vboxsf_setup_mutex);
    if vboxsf_setup_done { vboxsf_disconnect(); rcu_barrier(); kmem_cache_destroy(vboxsf_inode_cachep); }
    mutex_unlock(&mut vboxsf_setup_mutex);
}

// module_init(vboxsf_init); module_exit(vboxsf_fini);
// MODULE_DESCRIPTION("Oracle VM VirtualBox Module for Host File System Access");
// MODULE_AUTHOR("Oracle Corporation");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS_FS("vboxsf");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
