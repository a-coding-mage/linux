// SPDX-License-Identifier: GPL-2.0
/*
 * fs/sysfs/symlink.c - operations for initializing and mounting sysfs
 *
 * Copyright (c) 2001-3 Patrick Mochel
 * Copyright (c) 2007 SUSE Linux Products GmbH
 * Copyright (c) 2007 Tejun Heo <teheo@suse.de>
 *
 * Please see Documentation/filesystems/sysfs.rst for more information.
 */

// C headers provide the kernel types, constants, and external declarations
// referenced below.

static mut sysfs_root: *mut kernfs_root = core::ptr::null_mut();
pub static mut sysfs_root_kn: *mut kernfs_node = core::ptr::null_mut();

unsafe extern "C" {
    fn kobj_ns_drop(type_: u32, ns: *mut core::ffi::c_void);
    fn kernfs_free_fs_context(fc: *mut fs_context);
    fn kobj_ns_current_may_mount(type_: u32) -> bool;
    fn kzalloc_obj<T>() -> *mut T;
    fn kobj_ns_grab_current(type_: u32) -> *mut ns_common;
    fn to_net_ns(ns: *mut ns_common) -> *mut net;
    fn put_user_ns(ns: *mut user_namespace);
    fn get_user_ns(ns: *mut user_namespace) -> *mut user_namespace;
    fn kernfs_kill_sb(sb: *mut super_block);
    fn kernfs_super_ns(sb: *mut super_block) -> *mut core::ffi::c_void;
    fn kernfs_create_root(ops: *const core::ffi::c_void, flags: u32,
                          data: *mut core::ffi::c_void) -> *mut kernfs_root;
    fn is_err<T>(ptr: *mut T) -> bool;
    fn ptr_err<T>(ptr: *mut T) -> i32;
    fn kernfs_root_to_node(root: *mut kernfs_root) -> *mut kernfs_node;
    fn register_filesystem(fs: *mut file_system_type) -> i32;
    fn kernfs_destroy_root(root: *mut kernfs_root);
    fn kernfs_get_tree(fc: *mut fs_context) -> i32;
}

#[repr(C)]
struct kernfs_root;
#[repr(C)]
struct kernfs_node;
#[repr(C)]
struct fs_context {
    sb_flags: u32,
    fs_private: *mut core::ffi::c_void,
    ops: *const fs_context_operations,
    user_ns: *mut user_namespace,
    global: bool,
}
#[repr(C)]
struct kernfs_fs_context {
    ns_tag: *mut ns_common,
    root: *mut kernfs_root,
    magic: u32,
}
#[repr(C)]
struct ns_common;
#[repr(C)]
struct net {
    user_ns: *mut user_namespace,
}
#[repr(C)]
struct user_namespace;
#[repr(C)]
struct super_block;
#[repr(C)]
struct file_system_type {
    name: *const u8,
    init_fs_context: Option<unsafe extern "C" fn(*mut fs_context) -> i32>,
    kill_sb: Option<unsafe extern "C" fn(*mut super_block)>,
    fs_flags: u32,
}
#[repr(C)]
struct fs_context_operations {
    free: Option<unsafe extern "C" fn(*mut fs_context)>,
    get_tree: Option<unsafe extern "C" fn(*mut fs_context) -> i32>,
}

const KOBJ_NS_TYPE_NET: u32 = 0;
const SB_KERNMOUNT: u32 = 0;
const SYSFS_MAGIC: u32 = 0;
const KERNFS_ROOT_EXTRA_OPEN_PERM_CHECK: u32 = 0;
const FS_USERNS_MOUNT: u32 = 0;
const FS_USERNS_MOUNT_RESTRICTED: u32 = 0;
const EPERM: i32 = 1;
const ENOMEM: i32 = 12;

unsafe extern "C" fn sysfs_fs_context_free(fc: *mut fs_context) {
    let kfc = (*fc).fs_private as *mut kernfs_fs_context;

    if !(*kfc).ns_tag.is_null() {
        kobj_ns_drop(KOBJ_NS_TYPE_NET, (*kfc).ns_tag as *mut core::ffi::c_void);
    }
    kernfs_free_fs_context(fc);
    libc::free(kfc as *mut core::ffi::c_void);
}

static sysfs_fs_context_ops: fs_context_operations = fs_context_operations {
    free: Some(sysfs_fs_context_free),
    get_tree: Some(kernfs_get_tree),
};

unsafe extern "C" fn sysfs_init_fs_context(fc: *mut fs_context) -> i32 {
    let kfc: *mut kernfs_fs_context;
    let ns: *mut ns_common;

    if (*fc).sb_flags & SB_KERNMOUNT == 0 {
        if !kobj_ns_current_may_mount(KOBJ_NS_TYPE_NET) {
            return -EPERM;
        }
    }

    kfc = kzalloc_obj::<kernfs_fs_context>();
    if kfc.is_null() {
        return -ENOMEM;
    }

    ns = kobj_ns_grab_current(KOBJ_NS_TYPE_NET);
    (*kfc).ns_tag = ns;
    (*kfc).root = sysfs_root;
    (*kfc).magic = SYSFS_MAGIC;
    (*fc).fs_private = kfc as *mut core::ffi::c_void;
    (*fc).ops = &sysfs_fs_context_ops;
    if !ns.is_null() {
        let netns = to_net_ns(ns);

        put_user_ns((*fc).user_ns);
        (*fc).user_ns = get_user_ns((*netns).user_ns);
    }
    (*fc).global = true;
    0
}

unsafe extern "C" fn sysfs_kill_sb(sb: *mut super_block) {
    let ns = kernfs_super_ns(sb) as *mut ns_common;

    kernfs_kill_sb(sb);
    kobj_ns_drop(KOBJ_NS_TYPE_NET, ns as *mut core::ffi::c_void);
}

static mut sysfs_fs_type: file_system_type = file_system_type {
    name: b"sysfs\0".as_ptr(),
    init_fs_context: Some(sysfs_init_fs_context),
    kill_sb: Some(sysfs_kill_sb),
    fs_flags: FS_USERNS_MOUNT | FS_USERNS_MOUNT_RESTRICTED,
};

pub unsafe extern "C" fn sysfs_init() -> i32 {
    let mut err: i32;

    sysfs_root = kernfs_create_root(core::ptr::null(),
                                    KERNFS_ROOT_EXTRA_OPEN_PERM_CHECK,
                                    core::ptr::null_mut());
    if is_err(sysfs_root) {
        return ptr_err(sysfs_root);
    }

    sysfs_root_kn = kernfs_root_to_node(sysfs_root);

    err = register_filesystem(&raw mut sysfs_fs_type);
    if err != 0 {
        kernfs_destroy_root(sysfs_root);
        return err;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
