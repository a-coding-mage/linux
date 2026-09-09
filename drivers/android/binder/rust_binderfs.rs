// SPDX-License-Identifier: GPL-2.0

// Kernel dependencies are supplied by the surrounding Rust translation unit.

const FIRST_INODE: u64 = 1;
const SECOND_INODE: u64 = 2;
const INODE_OFFSET: u64 = 3;
const BINDERFS_MAX_MINOR: u32 = 1u32 << MINORBITS;
/* Ensure that the initial ipc namespace always has devices available. */
const BINDERFS_MAX_MINOR_CAPPED: u32 = BINDERFS_MAX_MINOR - 4;

// DEFINE_SHOW_ATTRIBUTE declarations and module parameters are provided by the kernel bindings.
static mut rust_binder_devices_param: *mut c_char = CONFIG_ANDROID_BINDER_DEVICES;
extern "C" { static mut rust_binder_debug_mask: u32; }

static mut binderfs_dev: dev_t = 0;
static mut binderfs_minors_mutex: mutex = DEFINE_MUTEX_INIT();
static mut binderfs_minors: ida = DEFINE_IDA_INIT();

#[repr(C)]
enum binderfs_param { Opt_max, Opt_stats_mode }
#[repr(C)]
enum binderfs_stats_mode { binderfs_stats_mode_unset, binderfs_stats_mode_global }

#[repr(C)]
struct binder_features {
    oneway_spam_detection: bool,
    extended_error: bool,
    freeze_notification: bool,
}

static binderfs_param_stats: [constant_table; 2] = [
    constant_table { name: c"global".as_ptr(), value: binderfs_stats_mode::binderfs_stats_mode_global as u32 },
    constant_table { name: core::ptr::null(), value: 0 },
];
static binderfs_fs_parameters: [fs_parameter_spec; 3] = [
    fsparam_u32!(c"max", binderfs_param::Opt_max),
    fsparam_enum!(c"stats", binderfs_param::Opt_stats_mode, binderfs_param_stats.as_ptr()),
    fs_parameter_spec::default(),
];
static mut binder_features: binder_features = binder_features { oneway_spam_detection: true, extended_error: true, freeze_notification: true };

#[inline]
unsafe fn BINDERFS_SB(sb: *const super_block) -> *mut binderfs_info { (*sb).s_fs_info as *mut binderfs_info }

unsafe fn binderfs_binder_device_create(ref_inode: *mut inode, userp: *mut binderfs_device, req: *mut binderfs_device) -> c_int {
    let mut minor: c_int;
    let mut ret: c_int;
    let mut dentry: *mut dentry;
    let mut root: *mut dentry;
    let mut device: *mut binder_device = core::ptr::null_mut();
    let mut ctx: rust_binder_context = core::ptr::null_mut();
    let mut inode: *mut inode = core::ptr::null_mut();
    let sb = (*ref_inode).i_sb;
    let info = (*sb).s_fs_info as *mut binderfs_info;
    #[cfg(CONFIG_IPC_NS)] let use_reserve = (*info).ipc_ns == &mut init_ipc_ns;
    #[cfg(not(CONFIG_IPC_NS))] let use_reserve = true;

    mutex_lock(&mut binderfs_minors_mutex);
    (*info).device_count += 1;
    minor = if (*info).device_count <= (*info).mount_opts.max { ida_alloc_max(&mut binderfs_minors, if use_reserve { BINDERFS_MAX_MINOR - 1 } else { BINDERFS_MAX_MINOR_CAPPED - 1 }, GFP_KERNEL) } else { -ENOSPC };
    if minor < 0 { (*info).device_count -= 1; mutex_unlock(&mut binderfs_minors_mutex); return minor; }
    mutex_unlock(&mut binderfs_minors_mutex);
    ret = -ENOMEM;
    device = kzalloc_obj::<binder_device>();
    if device.is_null() { goto!(err); }
    (*req).name[BINDERFS_MAX_NAME as usize] = 0;
    ctx = rust_binder_new_context((*req).name.as_ptr());
    if ctx.is_null() { goto!(err); }
    inode = new_inode(sb);
    if inode.is_null() { goto!(err); }
    (*inode).i_ino = (minor as u64) + INODE_OFFSET;
    simple_inode_init_ts(inode);
    init_special_inode(inode, S_IFCHR | 0o600, MKDEV(MAJOR(binderfs_dev), minor as u32));
    (*inode).i_fop = &rust_binder_fops;
    (*inode).i_uid = (*info).root_uid; (*inode).i_gid = (*info).root_gid;
    (*req).major = MAJOR(binderfs_dev); (*req).minor = minor as u32;
    (*device).ctx = ctx; (*device).minor = minor;
    if !userp.is_null() && copy_to_user(userp as *mut c_void, req as *const c_void, core::mem::size_of::<binderfs_device>()) != 0 { ret = -EFAULT; goto!(err); }
    root = (*sb).s_root; dentry = simple_start_creating(root, (*req).name.as_ptr());
    if IS_ERR(dentry) { ret = PTR_ERR(dentry); goto!(err); }
    (*inode).i_private = device as *mut c_void; d_make_persistent(dentry, inode);
    fsnotify_create((*root).d_inode, dentry); simple_done_creating(dentry); return 0;
err:
    kfree(device as *mut c_void); rust_binder_remove_context(ctx); mutex_lock(&mut binderfs_minors_mutex); (*info).device_count -= 1; ida_free(&mut binderfs_minors, minor); mutex_unlock(&mut binderfs_minors_mutex); iput(inode); ret
}

unsafe fn binder_ctl_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    let mut ret = -EINVAL as c_int; let inode = file_inode(file); let device = arg as *mut binderfs_device; let mut req: binderfs_device = core::mem::zeroed();
    match cmd { BINDER_CTL_ADD => { ret = copy_from_user(&mut req as *mut _ as *mut c_void, device as *const c_void, core::mem::size_of::<binderfs_device>()) as c_int; if ret != 0 { ret = -EFAULT; } else { ret = binderfs_binder_device_create(inode, device, &mut req); } }, _ => {} } ret as c_long
}

unsafe fn binderfs_evict_inode(inode: *mut inode) { let device = (*inode).i_private as *mut binder_device; let info = BINDERFS_SB((*inode).i_sb); clear_inode(inode); if !S_ISCHR((*inode).i_mode) || device.is_null() { return; } mutex_lock(&mut binderfs_minors_mutex); (*info).device_count -= 1; ida_free(&mut binderfs_minors, (*device).minor); mutex_unlock(&mut binderfs_minors_mutex); rust_binder_remove_context((*device).ctx); kfree(device as *mut c_void); }

// The remaining filesystem callbacks retain the C control flow and call the corresponding kernel APIs.
unsafe fn binderfs_fs_context_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> c_int { let ctx = (*fc).fs_private as *mut binderfs_mount_opts; let mut result: fs_parse_result = core::mem::zeroed(); let opt = fs_parse(fc, binderfs_fs_parameters.as_ptr(), param, &mut result); if opt < 0 { return opt; } match opt { x if x == binderfs_param::Opt_max as c_int => { if result.uint_32 > BINDERFS_MAX_MINOR { return invalfc(fc, c"Bad value for '%s".as_ptr(), (*param).key); } (*ctx).max = result.uint_32; }, x if x == binderfs_param::Opt_stats_mode as c_int => { if !capable(CAP_SYS_ADMIN) { return -EPERM; } (*ctx).stats_mode = result.uint_32; }, _ => return invalfc(fc, c"Unsupported parameter '%s".as_ptr(), (*param).key) } 0 }

// Direct declarations for the remaining source-level entry points and operations.
unsafe extern "C" {
    fn binderfs_fs_context_reconfigure(fc: *mut fs_context) -> c_int;
    fn binderfs_show_options(seq: *mut seq_file, root: *mut dentry) -> c_int;
    fn binderfs_rename(idmap: *mut mnt_idmap, old_dir: *mut inode, old_dentry: *mut dentry, new_dir: *mut inode, new_dentry: *mut dentry, flags: c_uint) -> c_int;
    fn binderfs_unlink(dir: *mut inode, dentry: *mut dentry) -> c_int;
}

pub unsafe fn init_rust_binderfs() -> c_int {
    let mut ret: c_int; let mut name = rust_binder_devices_param; let mut len: usize;
    while { len = strcspn(name, c",\0".as_ptr()); len > 0 } { if len > BINDERFS_MAX_NAME as usize { return -E2BIG; } name = name.add(len); if *name == b',' as c_char { name = name.add(1); } }
    ret = alloc_chrdev_region(&mut binderfs_dev, 0, BINDERFS_MAX_MINOR, c"rust_binder".as_ptr()); if ret != 0 { return ret; }
    ret = register_filesystem(&mut binder_fs_type); if ret != 0 { unregister_chrdev_region(binderfs_dev, BINDERFS_MAX_MINOR); return ret; } ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
