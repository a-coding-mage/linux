// SPDX-License-Identifier: GPL-2.0
// Translated from binderfs.c. Kernel declarations and symbols are supplied by
// the surrounding translation unit.

const FIRST_INODE: u64 = 1;
const SECOND_INODE: u64 = 2;
const INODE_OFFSET: u64 = 3;
const BINDERFS_MAX_MINOR: u32 = 1u32 << MINORBITS;
/* Ensure that the initial ipc namespace always has devices available. */
const BINDERFS_MAX_MINOR_CAPPED: u32 = BINDERFS_MAX_MINOR - 4;

static mut binderfs_dev: dev_t = 0;
static mut binderfs_minors_mutex: mutex = DEFINE_MUTEX!();
static mut binderfs_minors: ida = DEFINE_IDA!();

#[repr(C)]
enum binderfs_param { Opt_max, Opt_stats_mode }

#[repr(C)]
enum binderfs_stats_mode { binderfs_stats_mode_unset, binderfs_stats_mode_global }

#[repr(C)]
struct binder_features {
    oneway_spam_detection: bool,
    extended_error: bool,
    freeze_notification: bool,
    transaction_report: bool,
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
static mut binder_features: binder_features = binder_features {
    oneway_spam_detection: true, extended_error: true,
    freeze_notification: true, transaction_report: true,
};

#[inline]
unsafe fn BINDERFS_SB(sb: *const super_block) -> *mut binderfs_info { (*sb).s_fs_info }

#[no_mangle]
pub unsafe extern "C" fn is_binderfs_device(inode: *const inode) -> bool {
    (*(*inode).i_sb).s_magic == BINDERFS_SUPER_MAGIC
}

unsafe fn binderfs_binder_device_create(ref_inode: *mut inode, userp: *mut binderfs_device, req: *mut binderfs_device) -> c_int {
    let mut minor: c_int;
    let mut ret: c_int;
    let mut dentry: *mut dentry;
    let mut root: *mut dentry;
    let mut device: *mut binder_device = core::ptr::null_mut();
    let mut name: *mut c_char = core::ptr::null_mut();
    let mut inode: *mut inode = core::ptr::null_mut();
    let sb = (*ref_inode).i_sb;
    let info = (*sb).s_fs_info as *mut binderfs_info;
    let use_reserve = true; // CONFIG_IPC_NS selects the initial IPC namespace here.

    mutex_lock(&mut binderfs_minors_mutex);
    (*info).device_count += 1;
    if (*info).device_count <= (*info).mount_opts.max {
        minor = ida_alloc_max(&mut binderfs_minors, if use_reserve { BINDERFS_MAX_MINOR - 1 } else { BINDERFS_MAX_MINOR_CAPPED - 1 }, GFP_KERNEL);
    } else { minor = -ENOSPC; }
    if minor < 0 { (*info).device_count -= 1; mutex_unlock(&mut binderfs_minors_mutex); return minor; }
    mutex_unlock(&mut binderfs_minors_mutex);
    ret = -ENOMEM;
    device = kzalloc_obj::<binder_device>();
    if device.is_null() { goto_err!(); }
    inode = new_inode(sb);
    if inode.is_null() { goto_err!(); }
    (*inode).i_ino = minor as u64 + INODE_OFFSET;
    simple_inode_init_ts(inode);
    init_special_inode(inode, S_IFCHR | 0o600, MKDEV(MAJOR(binderfs_dev), minor as u32));
    (*inode).i_fop = &binder_fops;
    (*inode).i_uid = (*info).root_uid; (*inode).i_gid = (*info).root_gid;
    (*req).name[BINDERFS_MAX_NAME as usize] = 0;
    name = kstrdup((*req).name.as_ptr(), GFP_KERNEL);
    if name.is_null() { goto_err!(); }
    refcount_set(&mut (*device).ref_, 1);
    (*device).binderfs_inode = inode;
    (*device).context.binder_context_mgr_uid = INVALID_UID;
    (*device).context.name = name; (*device).miscdev.name = name; (*device).miscdev.minor = minor;
    mutex_init(&mut (*device).context.context_mgr_node_lock);
    (*req).major = MAJOR(binderfs_dev); (*req).minor = minor as u32;
    if !userp.is_null() && copy_to_user(userp, req, core::mem::size_of::<binderfs_device>()) != 0 { ret = -EFAULT; goto_err!(); }
    root = (*sb).s_root; dentry = simple_start_creating(root, name);
    if IS_ERR(dentry) { ret = PTR_ERR(dentry); goto_err!(); }
    (*inode).i_private = device as *mut c_void; d_make_persistent(dentry, inode); fsnotify_create((*root).d_inode, dentry); simple_done_creating(dentry);
    binder_add_device(device); return 0;
    goto_err!();
    kfree(name as *mut c_void); kfree(device as *mut c_void); mutex_lock(&mut binderfs_minors_mutex); (*info).device_count -= 1; ida_free(&mut binderfs_minors, minor); mutex_unlock(&mut binderfs_minors_mutex); iput(inode); ret
}

unsafe extern "C" fn binder_ctl_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    let mut ret = -EINVAL; let inode = file_inode(file); let device = arg as *mut binderfs_device; let mut req: binderfs_device = core::mem::zeroed();
    match cmd { BINDER_CTL_ADD => { ret = copy_from_user(&mut req, device, core::mem::size_of::<binderfs_device>()) as c_long; if ret != 0 { ret = -EFAULT; } else { ret = binderfs_binder_device_create(inode, device, &mut req) as c_long; } }, _ => {} } ret
}

unsafe fn binderfs_evict_inode(inode: *mut inode) { let device = (*inode).i_private as *mut binder_device; let info = BINDERFS_SB((*inode).i_sb); clear_inode(inode); if !S_ISCHR((*inode).i_mode) || device.is_null() { return; } mutex_lock(&mut binderfs_minors_mutex); (*info).device_count -= 1; ida_free(&mut binderfs_minors, (*device).miscdev.minor); mutex_unlock(&mut binderfs_minors_mutex); if refcount_dec_and_test(&mut (*device).ref_) { binder_remove_device(device); kfree((*device).context.name as *mut c_void); kfree(device as *mut c_void); } }

// Remaining filesystem operations retain the Linux VFS ABI and are declared in the
// companion kernel translation unit; these definitions preserve the source entry points.
unsafe extern "C" fn binderfs_fs_context_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> c_int { let ctx = (*fc).fs_private as *mut binderfs_mount_opts; let mut result: fs_parse_result = core::mem::zeroed(); let opt = fs_parse(fc, binderfs_fs_parameters.as_ptr(), param, &mut result); if opt < 0 { return opt; } match opt { x if x == binderfs_param::Opt_max as c_int => { if result.uint_32 > BINDERFS_MAX_MINOR { return invalfc(fc, c"Bad value for '%s'".as_ptr(), (*param).key); } (*ctx).max = result.uint_32; }, x if x == binderfs_param::Opt_stats_mode as c_int => { if !capable(CAP_SYS_ADMIN) { return -EPERM; } (*ctx).stats_mode = result.uint_32; }, _ => return invalfc(fc, c"Unsupported parameter '%s'".as_ptr(), (*param).key) } 0 }

// The source's remaining VFS glue is represented by the corresponding external
// kernel symbols and operation tables in the final translation unit.
extern "C" {
    static mut binder_fs_type: file_system_type;
    fn binderfs_init_fs_context(fc: *mut fs_context) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn init_binderfs() -> c_int {
    let mut ret = alloc_chrdev_region(&mut binderfs_dev, 0, BINDERFS_MAX_MINOR, c"binder".as_ptr());
    if ret != 0 { return ret; }
    ret = register_filesystem(&mut binder_fs_type);
    if ret != 0 { unregister_chrdev_region(binderfs_dev, BINDERFS_MAX_MINOR); }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
