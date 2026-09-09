// SPDX-License-Identifier: GPL-2.0
/*
 * fs/sysfs/dir.c - sysfs core and dir operation implementation
 *
 * Copyright (c) 2001-3 Patrick Mochel
 * Copyright (c) 2007 SUSE Linux Products GmbH
 * Copyright (c) 2007 Tejun Heo <teheo@suse.de>
 *
 * Please see Documentation/filesystems/sysfs.rst for more information.
 */

// pr_fmt(fmt) is "sysfs: " fmt.

use core::ffi::{c_char, c_void};

// Types, constants, functions, and globals supplied by the surrounding kernel bindings.
extern "C" {
    static mut sysfs_root_kn: *mut kernfs_node;
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn kernfs_path(parent: *mut kernfs_node, buf: *mut c_char, length: usize);
    fn dump_stack();
    fn kobject_get_ownership(kobj: *mut kobject, uid: *mut kuid_t, gid: *mut kgid_t);
    fn kobject_name(kobj: *mut kobject) -> *const c_char;
    fn kernfs_create_dir_ns(
        parent: *mut kernfs_node,
        name: *const c_char,
        mode: u32,
        uid: kuid_t,
        gid: kgid_t,
        priv_: *mut c_void,
        ns: *const ns_common,
    ) -> *mut kernfs_node;
    fn kernfs_type(kn: *mut kernfs_node) -> u32;
    fn kernfs_remove(kn: *mut kernfs_node);
    fn kernfs_get_parent(kn: *mut kernfs_node) -> *mut kernfs_node;
    fn kernfs_rename_ns(
        kn: *mut kernfs_node,
        parent: *mut kernfs_node,
        new_name: *const c_char,
        new_ns: *const ns_common,
    ) -> i32;
    fn kernfs_put(kn: *mut kernfs_node);
    fn kernfs_create_empty_dir(parent: *mut kernfs_node, name: *const c_char) -> *mut kernfs_node;
    fn kernfs_remove_by_name_ns(
        parent: *mut kernfs_node,
        name: *const c_char,
        ns: *const ns_common,
    );
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
}

#[repr(C)]
pub struct kernfs_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct kobject {
    pub parent: *mut kobject,
    pub sd: *mut kernfs_node,
}
#[repr(C)]
pub struct ns_common {
    _private: [u8; 0],
}
#[repr(C)]
pub struct kuid_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct kgid_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

pub const GFP_KERNEL: u32 = 0;
pub const PATH_MAX: usize = 4096;
pub const KERNFS_DIR: u32 = 0x0000_0001;

#[no_mangle]
pub static mut sysfs_symlink_target_lock: spinlock_t = spinlock_t { _private: [] };

pub unsafe fn sysfs_warn_dup(parent: *mut kernfs_node, name: *const c_char) {
    let buf = kzalloc(PATH_MAX, GFP_KERNEL) as *mut c_char;

    if !buf.is_null() {
        kernfs_path(parent, buf, PATH_MAX);
    }

    // pr_warn("cannot create duplicate filename '%s/%s'\n", buf, name);
    dump_stack();

    kfree(buf as *mut c_void);
}

/**
 * sysfs_create_dir_ns - create a directory for an object with a namespace tag
 * @kobj: object we're creating directory for
 * @ns: the namespace tag to use
 */
pub unsafe fn sysfs_create_dir_ns(kobj: *mut kobject, ns: *const ns_common) -> i32 {
    let parent: *mut kernfs_node;
    let kn: *mut kernfs_node;
    let mut uid = kuid_t { _private: [] };
    let mut gid = kgid_t { _private: [] };

    if kobj.is_null() {
        return -22;
    }

    if !(*kobj).parent.is_null() {
        parent = (*(*kobj).parent).sd;
    } else {
        parent = sysfs_root_kn;
    }

    if parent.is_null() {
        return -2;
    }

    kobject_get_ownership(kobj, &mut uid, &mut gid);

    kn = kernfs_create_dir_ns(parent, kobject_name(kobj), 0o755, uid, gid, kobj as *mut c_void, ns);
    if (kn as isize) < 0 {
        if kn as isize == -17 {
            sysfs_warn_dup(parent, kobject_name(kobj));
        }
        return kn as isize as i32;
    }

    (*kobj).sd = kn;
    0
}

/**
 * sysfs_remove_dir - remove an object's directory.
 * @kobj: object.
 */
pub unsafe fn sysfs_remove_dir(kobj: *mut kobject) {
    let kn = (*kobj).sd;

    spin_lock(&mut sysfs_symlink_target_lock);
    (*kobj).sd = core::ptr::null_mut();
    spin_unlock(&mut sysfs_symlink_target_lock);

    if !kn.is_null() {
        kernfs_remove(kn);
    }
}

pub unsafe fn sysfs_rename_dir_ns(
    kobj: *mut kobject,
    new_name: *const c_char,
    new_ns: *const ns_common,
) -> i32 {
    let parent = kernfs_get_parent((*kobj).sd);
    let ret = kernfs_rename_ns((*kobj).sd, parent, new_name, new_ns);
    kernfs_put(parent);
    ret
}

pub unsafe fn sysfs_move_dir_ns(
    kobj: *mut kobject,
    new_parent_kobj: *mut kobject,
    new_ns: *const ns_common,
) -> i32 {
    let kn = (*kobj).sd;
    let new_parent = if !new_parent_kobj.is_null() && !(*new_parent_kobj).sd.is_null() {
        (*new_parent_kobj).sd
    } else {
        sysfs_root_kn
    };

    kernfs_rename_ns(kn, new_parent, core::ptr::null(), new_ns)
}

/**
 * sysfs_create_mount_point - create an always empty directory
 * @parent_kobj: kobject that will contain this always empty directory
 * @name: The name of the always empty directory to add
 */
pub unsafe fn sysfs_create_mount_point(parent_kobj: *mut kobject, name: *const c_char) -> i32 {
    let parent = (*parent_kobj).sd;
    let kn = kernfs_create_empty_dir(parent, name);

    if (kn as isize) < 0 {
        if kn as isize == -17 {
            sysfs_warn_dup(parent, name);
        }
        return kn as isize as i32;
    }

    0
}

// EXPORT_SYMBOL_GPL(sysfs_create_mount_point);

/**
 * sysfs_remove_mount_point - remove an always empty directory.
 * @parent_kobj: kobject that will contain this always empty directory
 * @name: The name of the always empty directory to remove
 */
pub unsafe fn sysfs_remove_mount_point(parent_kobj: *mut kobject, name: *const c_char) {
    let parent = (*parent_kobj).sd;
    kernfs_remove_by_name_ns(parent, name, core::ptr::null());
}

// EXPORT_SYMBOL_GPL(sysfs_remove_mount_point);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
