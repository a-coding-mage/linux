// SPDX-License-Identifier: GPL-2.0-or-later
/* -*- linux-c -*- --------------------------------------------------------- *
 *
 * linux/fs/devpts/inode.c
 *
 * Copyright 1998-2004 H. Peter Anvin -- All Rights Reserved
 *
 * ------------------------------------------------------------------------- */

// Translated from C. Kernel-provided types, constants, functions, and macros
// are intentionally referenced as external dependencies.

const DEVPTS_DEFAULT_MODE: umode_t = 0o600;
/*
 * ptmx is a new node in /dev/pts and will be unused in legacy (single-
 * instance) mode. To prevent surprises in user space, set permissions of
 * ptmx to 0. Use 'chmod' or remount with '-o ptmxmode' to set meaningful
 * permissions.
 */
const DEVPTS_DEFAULT_PTMX_MODE: umode_t = 0o000;
const PTMX_MINOR: i32 = 2;

static mut pty_limit: i32 = NR_UNIX98_PTY_DEFAULT;
static mut pty_reserve: i32 = NR_UNIX98_PTY_RESERVE;
static mut pty_limit_min: i32 = 0;
static mut pty_limit_max: i32 = INT_MAX;
static mut pty_count: atomic_t = ATOMIC_INIT(0);

static pty_table: [ctl_table; 3] = [
    ctl_table { procname: "max", maxlen: core::mem::size_of::<i32>(), mode: 0o644, data: unsafe { &mut pty_limit }, proc_handler: proc_dointvec_minmax, extra1: unsafe { &mut pty_limit_min }, extra2: unsafe { &mut pty_limit_max } },
    ctl_table { procname: "reserve", maxlen: core::mem::size_of::<i32>(), mode: 0o644, data: unsafe { &mut pty_reserve }, proc_handler: proc_dointvec_minmax, extra1: unsafe { &mut pty_limit_min }, extra2: unsafe { &mut pty_limit_max } },
    ctl_table { procname: "nr", maxlen: core::mem::size_of::<i32>(), mode: 0o444, data: unsafe { &mut pty_count }, proc_handler: proc_dointvec, extra1: core::ptr::null_mut(), extra2: core::ptr::null_mut() },
];

#[repr(C)]
pub struct pts_mount_opts { pub setuid: i32, pub setgid: i32, pub uid: kuid_t, pub gid: kgid_t, pub mode: umode_t, pub ptmxmode: umode_t, pub reserve: i32, pub max: i32 }

enum { Opt_uid, Opt_gid, Opt_mode, Opt_ptmxmode, Opt_newinstance, Opt_max, Opt_err }

static devpts_param_specs: [fs_parameter_spec; 7] = [
    fsparam_gid!("gid", Opt_gid), fsparam_s32!("max", Opt_max), fsparam_u32oct!("mode", Opt_mode),
    fsparam_flag!("newinstance", Opt_newinstance), fsparam_u32oct!("ptmxmode", Opt_ptmxmode),
    fsparam_uid!("uid", Opt_uid), fs_parameter_spec::default(),
];

#[repr(C)]
pub struct pts_fs_info { pub allocated_ptys: ida, pub mount_opts: pts_mount_opts, pub sb: *mut super_block, pub ptmx_inode: *mut inode }

#[inline]
unsafe fn DEVPTS_SB(sb: *mut super_block) -> *mut pts_fs_info { (*sb).s_fs_info as *mut pts_fs_info }

unsafe fn devpts_ptmx_path(path: *mut path) -> i32 {
    let err = path_pts(path); if err != 0 { return err; }
    let sb = (*(*path).mnt).mnt_sb;
    if (*sb).s_magic != DEVPTS_SUPER_MAGIC || (*path).mnt.mnt_root != (*sb).s_root { return -ENODEV; }
    0
}

pub unsafe fn devpts_mntget(filp: *mut file, fsi: *mut pts_fs_info) -> *mut vfsmount {
    let mut path = (*filp).f_path; path_get(&mut path);
    while (*path.mnt).mnt_root == path.dentry { if follow_up(&mut path) == 0 { break; } }
    let mut err = 0;
    if (*(*path.mnt).mnt_sb).s_magic != DEVPTS_SUPER_MAGIC || DEVPTS_SB((*path.mnt).mnt_sb) != fsi { err = devpts_ptmx_path(&mut path); }
    dput(path.dentry);
    if err == 0 { if DEVPTS_SB((*path.mnt).mnt_sb) == fsi { return path.mnt; } err = -ENODEV; }
    mntput(path.mnt); ERR_PTR(err)
}

pub unsafe fn devpts_acquire(filp: *mut file) -> *mut pts_fs_info {
    let mut result; let mut path = (*filp).f_path; path_get(&mut path);
    if (*(*path.mnt).mnt_sb).s_magic != DEVPTS_SUPER_MAGIC { let err = devpts_ptmx_path(&mut path); if err != 0 { result = ERR_PTR(err); path_put(&mut path); return result; } }
    let sb = (*path.mnt).mnt_sb; atomic_inc(&mut (*sb).s_active); result = DEVPTS_SB(sb); path_put(&mut path); result
}

pub unsafe fn devpts_release(fsi: *mut pts_fs_info) { deactivate_super((*fsi).sb); }

unsafe fn devpts_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> i32 {
    let fsi = (*fc).s_fs_info as *mut pts_fs_info; let opts = &mut (*fsi).mount_opts; let mut result = fs_parse_result::default();
    let opt = fs_parse(fc, devpts_param_specs.as_ptr(), param, &mut result); if opt < 0 { return opt; }
    match opt { Opt_uid => { opts.uid = result.uid; opts.setuid = 1; }, Opt_gid => { opts.gid = result.gid; opts.setgid = 1; }, Opt_mode => opts.mode = result.uint_32 & S_IALLUGO, Opt_ptmxmode => opts.ptmxmode = result.uint_32 & S_IALLUGO, Opt_newinstance => {}, Opt_max => { if result.uint_32 > NR_UNIX98_PTY_MAX { return invalf(fc, "max out of range"); } opts.max = result.uint_32; }, _ => {} } 0
}

unsafe fn mknod_ptmx(sb: *mut super_block, fc: *mut fs_context) -> i32 {
    let fsi = DEVPTS_SB(sb); let opts = &mut (*fsi).mount_opts; let dentry = simple_start_creating((*sb).s_root, "ptmx"); if IS_ERR(dentry) { pr_err!("Unable to alloc dentry for ptmx node\n"); return PTR_ERR(dentry); }
    let inode = new_inode(sb); if inode.is_null() { simple_done_creating(dentry); pr_err!("Unable to alloc inode for ptmx node\n"); return -ENOMEM; }
    (*inode).i_ino = 2; simple_inode_init_ts(inode); init_special_inode(inode, S_IFCHR | opts.ptmxmode, MKDEV(TTYAUX_MAJOR, 2)); (*inode).i_uid = current_fsuid(); (*inode).i_gid = current_fsgid(); (*fsi).ptmx_inode = inode; d_make_persistent(dentry, inode); simple_done_creating(dentry); 0
}

unsafe fn update_ptmx_mode(fsi: *mut pts_fs_info) { (*(*fsi).ptmx_inode).i_mode = S_IFCHR | (*fsi).mount_opts.ptmxmode; }

unsafe fn devpts_reconfigure(fc: *mut fs_context) -> i32 { let fsi = DEVPTS_SB((*fc).root.d_sb); let new = (*fc).s_fs_info as *mut pts_fs_info; (*fsi).mount_opts = (*new).mount_opts; update_ptmx_mode(fsi); 0 }

unsafe fn devpts_show_options(seq: *mut seq_file, root: *mut dentry) -> i32 { let opts = &(*DEVPTS_SB((*root).d_sb)).mount_opts; if opts.setuid != 0 { seq_printf!(seq, ",uid=%u", from_kuid_munged(&init_user_ns, opts.uid)); } if opts.setgid != 0 { seq_printf!(seq, ",gid=%u", from_kgid_munged(&init_user_ns, opts.gid)); } seq_printf!(seq, ",mode=%03o", opts.mode); seq_printf!(seq, ",ptmxmode=%03o", opts.ptmxmode); if opts.max < NR_UNIX98_PTY_MAX { seq_printf!(seq, ",max=%d", opts.max); } 0 }

static devpts_sops: super_operations = super_operations { statfs: simple_statfs, show_options: devpts_show_options };

unsafe fn devpts_fill_super(s: *mut super_block, fc: *mut fs_context) -> i32 { let fsi = DEVPTS_SB(s); (*s).s_iflags &= !SB_I_NODEV; (*s).s_blocksize = 1024; (*s).s_blocksize_bits = 10; (*s).s_magic = DEVPTS_SUPER_MAGIC; (*s).s_op = &devpts_sops; (*s).s_d_flags = DCACHE_DONTCACHE; (*s).s_time_gran = 1; (*fsi).sb = s; let inode = new_inode(s); if inode.is_null() { return -ENOMEM; } (*inode).i_ino = 1; simple_inode_init_ts(inode); (*inode).i_mode = S_IFDIR | S_IRUGO | S_IXUGO | S_IWUSR; (*inode).i_op = &simple_dir_inode_operations; (*inode).i_fop = &simple_dir_operations; set_nlink(inode, 2); (*s).s_root = d_make_root(inode); if (*s).s_root.is_null() { pr_err!("get root dentry failed\n"); return -ENOMEM; } mknod_ptmx(s, fc) }

unsafe fn devpts_get_tree(fc: *mut fs_context) -> i32 { get_tree_nodev(fc, devpts_fill_super) }
unsafe fn devpts_free_fc(fc: *mut fs_context) { kfree((*fc).s_fs_info); }
static devpts_context_ops: fs_context_operations = fs_context_operations { free: devpts_free_fc, parse_param: devpts_parse_param, get_tree: devpts_get_tree, reconfigure: devpts_reconfigure };

unsafe fn devpts_init_fs_context(fc: *mut fs_context) -> i32 { let fsi = kzalloc_obj::<pts_fs_info>(); if fsi.is_null() { return -ENOMEM; } ida_init(&mut (*fsi).allocated_ptys); (*fsi).mount_opts.uid = GLOBAL_ROOT_UID; (*fsi).mount_opts.gid = GLOBAL_ROOT_GID; (*fsi).mount_opts.mode = DEVPTS_DEFAULT_MODE; (*fsi).mount_opts.ptmxmode = DEVPTS_DEFAULT_PTMX_MODE; (*fsi).mount_opts.max = NR_UNIX98_PTY_MAX; if (*fc).purpose == FS_CONTEXT_FOR_MOUNT && (*current).nsproxy.mnt_ns == init_task.nsproxy.mnt_ns { (*fsi).mount_opts.reserve = 1; } (*fc).s_fs_info = fsi as *mut _; (*fc).ops = &devpts_context_ops; 0 }
unsafe fn devpts_kill_sb(sb: *mut super_block) { let fsi = DEVPTS_SB(sb); if !fsi.is_null() { ida_destroy(&mut (*fsi).allocated_ptys); kfree(fsi); } kill_anon_super(sb); }
static devpts_fs_type: file_system_type = file_system_type { name: "devpts", init_fs_context: devpts_init_fs_context, parameters: devpts_param_specs.as_ptr(), kill_sb: devpts_kill_sb, fs_flags: FS_USERNS_MOUNT };

pub unsafe fn devpts_new_index(fsi: *mut pts_fs_info) -> i32 { let mut index = -ENOSPC; if atomic_inc_return(&mut pty_count) >= pty_limit - if (*fsi).mount_opts.reserve != 0 { 0 } else { pty_reserve } { return { atomic_dec(&mut pty_count); index }; } index = ida_alloc_max(&mut (*fsi).allocated_ptys, (*fsi).mount_opts.max - 1, GFP_KERNEL); if index < 0 { atomic_dec(&mut pty_count); } index }
pub unsafe fn devpts_kill_index(fsi: *mut pts_fs_info, idx: i32) { ida_free(&mut (*fsi).allocated_ptys, idx); atomic_dec(&mut pty_count); }

pub unsafe fn devpts_pty_new(fsi: *mut pts_fs_info, index: i32, priv_: *mut core::ffi::c_void) -> *mut dentry { let sb = (*fsi).sb; let opts = &(*fsi).mount_opts; let inode = new_inode(sb); if inode.is_null() { return ERR_PTR(-ENOMEM); } (*inode).i_ino = index + 3; (*inode).i_uid = if opts.setuid != 0 { opts.uid } else { current_fsuid() }; (*inode).i_gid = if opts.setgid != 0 { opts.gid } else { current_fsgid() }; simple_inode_init_ts(inode); init_special_inode(inode, S_IFCHR | opts.mode, MKDEV(UNIX98_PTY_SLAVE_MAJOR, index)); let mut s = [0i8; 12]; sprintf!(s.as_mut_ptr(), "%d", index); let dentry = d_alloc_name((*sb).s_root, s.as_ptr()); if dentry.is_null() { iput(inode); return ERR_PTR(-ENOMEM); } (*dentry).d_fsdata = priv_; d_make_persistent(dentry, inode); fsnotify_create(d_inode((*sb).s_root), dentry); dput(dentry); dentry }

pub unsafe fn devpts_get_priv(dentry: *mut dentry) -> *mut core::ffi::c_void { if (*(*dentry).d_sb).s_magic != DEVPTS_SUPER_MAGIC { core::ptr::null_mut() } else { (*dentry).d_fsdata } }
pub unsafe fn devpts_pty_kill(dentry: *mut dentry) { WARN_ON_ONCE!((*(*dentry).d_sb).s_magic != DEVPTS_SUPER_MAGIC); (*dentry).d_fsdata = core::ptr::null_mut(); drop_nlink((*dentry).d_inode); d_drop(dentry); fsnotify_unlink(d_inode((*dentry).d_parent), dentry); d_make_discardable(dentry); }

unsafe fn init_devpts_fs() -> i32 { let err = register_filesystem(&devpts_fs_type); if err == 0 { register_sysctl("kernel/pty", pty_table.as_ptr()); } err }
module_init!(init_devpts_fs);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
