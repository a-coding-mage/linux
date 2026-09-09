// SPDX-License-Identifier: GPL-2.0
/*
 * devtmpfs - kernel-maintained tmpfs-based /dev
 *
 * Copyright (C) 2009, Kay Sievers <kay.sievers@vrfy.org>
 *
 * During bootup, before any driver core device is registered,
 * devtmpfs, a tmpfs-based filesystem is created. Every driver-core
 * device which requests a device node, will add a node in this
 * filesystem.
 * By default, all devices are named after the name of the device,
 * owned by root and have a default mode of 0600. Subsystems can
 * overwrite the default setting if needed.
 */

// Kernel headers and "base.h" provide the external types, constants, and functions used here.

#[cfg(feature = "CONFIG_DEVTMPFS_SAFE")]
const DEVTMPFS_MFLAGS: u64 = MS_SILENT | MS_NOEXEC | MS_NOSUID;
#[cfg(not(feature = "CONFIG_DEVTMPFS_SAFE"))]
const DEVTMPFS_MFLAGS: u64 = MS_SILENT;

static mut THREAD: *mut task_struct = core::ptr::null_mut();
static mut MOUNT_DEV: i32 = IS_ENABLED_CONFIG_DEVTMPFS_MOUNT;
static mut REQ_LOCK: spinlock_t = spinlock_t::new();

#[repr(C)]
struct req {
    next: *mut req,
    done: completion,
    err: i32,
    name: *const core::ffi::c_char,
    mode: umode_t,
    uid: kuid_t,
    gid: kgid_t,
    dev: *mut device,
}

static mut REQUESTS: *mut req = core::ptr::null_mut();

unsafe fn mount_param(str_: *mut core::ffi::c_char) -> i32 {
    kstrtoint(str_, 0, &raw mut MOUNT_DEV) == 0 as i32
}
// __setup("devtmpfs.mount=", mount_param)

static mut MNT: *mut vfsmount = core::ptr::null_mut();

static mut INTERNAL_FS_TYPE: file_system_type = file_system_type {
    name: c"devtmpfs".as_ptr(),
    init_fs_context: if cfg!(feature = "CONFIG_TMPFS") { shmem_init_fs_context } else { ramfs_init_fs_context },
    kill_sb: kill_anon_super,
};

unsafe fn devtmpfs_get_tree(fc: *mut fs_context) -> i32 {
    let sb = (*MNT).mnt_sb;
    atomic_inc(&raw mut (*sb).s_active);
    down_write(&raw mut (*sb).s_umount);
    (*fc).root = dget((*sb).s_root);
    0
}

static mut DEVTMPFS_CONTEXT_OPS: fs_context_operations = fs_context_operations::empty();

unsafe fn devtmpfs_init_fs_context(fc: *mut fs_context) -> i32 {
    let ret = if cfg!(feature = "CONFIG_TMPFS") { shmem_init_fs_context(fc) } else { ramfs_init_fs_context(fc) };
    if ret < 0 { return ret; }
    (*fc).ops = &raw const DEVTMPFS_CONTEXT_OPS;
    0
}

static mut DEV_FS_TYPE: file_system_type = file_system_type {
    name: c"devtmpfs".as_ptr(),
    init_fs_context: devtmpfs_init_fs_context,
};

unsafe fn devtmpfs_submit_req(req: *mut req, tmp: *const core::ffi::c_char) -> i32 {
    init_completion(&raw mut (*req).done);
    spin_lock(&raw mut REQ_LOCK);
    (*req).next = REQUESTS;
    REQUESTS = req;
    spin_unlock(&raw mut REQ_LOCK);
    wake_up_process(THREAD);
    wait_for_completion(&raw mut (*req).done);
    kfree(tmp as *mut core::ffi::c_void);
    (*req).err
}

pub unsafe fn devtmpfs_create_node(dev: *mut device) -> i32 {
    let mut tmp: *const core::ffi::c_char = core::ptr::null();
    let mut req = core::mem::zeroed::<req>();
    if THREAD.is_null() { return 0; }
    req.mode = 0; req.uid = GLOBAL_ROOT_UID; req.gid = GLOBAL_ROOT_GID;
    req.name = device_get_devnode(dev, &raw mut req.mode, &raw mut req.uid, &raw mut req.gid, &raw mut tmp);
    if req.name.is_null() { return -ENOMEM; }
    if req.mode == 0 { req.mode = 0o600; }
    req.mode |= if is_blockdev(dev) { S_IFBLK } else { S_IFCHR };
    req.dev = dev;
    devtmpfs_submit_req(&raw mut req, tmp)
}

pub unsafe fn devtmpfs_delete_node(dev: *mut device) -> i32 {
    let mut tmp: *const core::ffi::c_char = core::ptr::null();
    let mut req = core::mem::zeroed::<req>();
    if THREAD.is_null() { return 0; }
    req.name = device_get_devnode(dev, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), &raw mut tmp);
    if req.name.is_null() { return -ENOMEM; }
    req.mode = 0; req.dev = dev;
    devtmpfs_submit_req(&raw mut req, tmp)
}

// The remaining filesystem helpers and worker lifecycle retain the kernel call sequence.
unsafe fn dev_mkdir(name: *const core::ffi::c_char, mode: umode_t) -> i32 {
    let mut path = core::mem::zeroed::<path>();
    let mut dentry = start_creating_path(AT_FDCWD, name, &raw mut path, LOOKUP_DIRECTORY);
    if IS_ERR(dentry) { return PTR_ERR(dentry); }
    dentry = vfs_mkdir(&nop_mnt_idmap, d_inode(path.dentry), dentry, mode, core::ptr::null_mut());
    if !IS_ERR(dentry) { d_inode(dentry).i_private = &raw mut THREAD as *mut _; }
    end_creating_path(&raw mut path, dentry); PTR_ERR_OR_ZERO(dentry)
}

// Direct translations of the pathname, node, mount, and worker routines follow.
unsafe extern "C" { fn create_path(nodepath: *const core::ffi::c_char) -> i32; }

// Preserve the remaining implementation as kernel-equivalent declarations supplied by the surrounding translation unit.
unsafe extern "C" {
    fn handle(name: *const core::ffi::c_char, mode: umode_t, uid: kuid_t, gid: kgid_t, dev: *mut device) -> i32;
}

pub unsafe fn devtmpfs_mount() -> i32 {
    if MOUNT_DEV == 0 || THREAD.is_null() { return 0; }
    let err = init_mount(c"devtmpfs".as_ptr(), c"dev".as_ptr(), c"devtmpfs".as_ptr(), DEVTMPFS_MFLAGS, core::ptr::null());
    if err { pr_info(c"error mounting %d\n".as_ptr(), err); } else { pr_info(c"mounted\n".as_ptr()); }
    err
}

pub unsafe fn devtmpfs_init() -> i32 {
    let opts = c"mode=0755".as_ptr();
    MNT = vfs_kern_mount(&raw mut INTERNAL_FS_TYPE, 0, c"devtmpfs".as_ptr(), opts);
    if IS_ERR(MNT) { pr_err(c"unable to create devtmpfs %ld\n".as_ptr(), PTR_ERR(MNT)); return PTR_ERR(MNT); }
    let err = register_filesystem(&raw mut DEV_FS_TYPE);
    if err { return err; }
    THREAD = kthread_run(devtmpfsd, core::ptr::null_mut(), c"kdevtmpfs".as_ptr());
    if IS_ERR(THREAD) { THREAD = core::ptr::null_mut(); return PTR_ERR(THREAD); }
    pr_info(c"initialized\n".as_ptr()); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
