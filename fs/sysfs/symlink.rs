// SPDX-License-Identifier: GPL-2.0
/*
 * fs/sysfs/symlink.c - sysfs symlink implementation
 *
 * Copyright (c) 2001-3 Patrick Mochel
 * Copyright (c) 2007 SUSE Linux Products GmbH
 * Copyright (c) 2007 Tejun Heo <teheo@suse.de>
 *
 * Please see Documentation/filesystems/sysfs.rst for more information.
 */

// Linux header dependencies are supplied by the surrounding translation unit.

unsafe extern "C" {
    static mut sysfs_symlink_target_lock: spinlock_t;
    static mut sysfs_root_kn: *mut kernfs_node;

    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn kernfs_get(kn: *mut kernfs_node);
    fn kernfs_put(kn: *mut kernfs_node);
    fn kernfs_create_link(parent: *mut kernfs_node, name: *const c_char,
                          target: *mut kernfs_node) -> *mut kernfs_node;
    fn sysfs_warn_dup(parent: *mut kernfs_node, name: *const c_char);
    fn kernfs_ns_enabled(kn: *mut kernfs_node) -> bool;
    fn kernfs_remove_by_name_ns(parent: *mut kernfs_node, name: *const c_char,
                                ns: *const ns_common);
    fn kernfs_remove_by_name(parent: *mut kernfs_node, name: *const c_char);
    fn kernfs_find_and_get_ns(parent: *mut kernfs_node, name: *const c_char,
                              ns: *const ns_common) -> *mut kernfs_node;
    fn kernfs_type(kn: *mut kernfs_node) -> u32;
    fn kernfs_rename_ns(kn: *mut kernfs_node, parent: *mut kernfs_node,
                        name: *const c_char, ns: *const ns_common) -> c_int;
}

unsafe fn sysfs_do_create_link_sd(
    parent: *mut kernfs_node,
    target_kobj: *mut kobject,
    name: *const c_char,
    warn: c_int,
) -> c_int {
    let mut target: *mut kernfs_node = core::ptr::null_mut();

    if name.is_null() || parent.is_null() {
        return -EINVAL;
    }

    spin_lock(&raw mut sysfs_symlink_target_lock);
    if !(*target_kobj).sd.is_null() {
        target = (*target_kobj).sd;
        kernfs_get(target);
    }
    spin_unlock(&raw mut sysfs_symlink_target_lock);

    if target.is_null() {
        return -ENOENT;
    }

    let kn = kernfs_create_link(parent, name, target);
    kernfs_put(target);

    if !IS_ERR(kn) {
        return 0;
    }

    if warn != 0 && PTR_ERR(kn) == -EEXIST {
        sysfs_warn_dup(parent, name);
    }
    PTR_ERR(kn)
}

/// Create symlink to a given object.
pub unsafe fn sysfs_create_link_sd(
    kn: *mut kernfs_node,
    target: *mut kobject,
    name: *const c_char,
) -> c_int {
    sysfs_do_create_link_sd(kn, target, name, 1)
}

unsafe fn sysfs_do_create_link(
    kobj: *mut kobject,
    target: *mut kobject,
    name: *const c_char,
    warn: c_int,
) -> c_int {
    let parent = if kobj.is_null() { sysfs_root_kn } else { (*kobj).sd };

    if parent.is_null() {
        return -EFAULT;
    }

    sysfs_do_create_link_sd(parent, target, name, warn)
}

/// Create symlink between two objects.
pub unsafe fn sysfs_create_link(
    kobj: *mut kobject,
    target: *mut kobject,
    name: *const c_char,
) -> c_int {
    sysfs_do_create_link(kobj, target, name, 1)
}

/// Create symlink between two objects without warning if it already exists.
pub unsafe fn sysfs_create_link_nowarn(
    kobj: *mut kobject,
    target: *mut kobject,
    name: *const c_char,
) -> c_int {
    sysfs_do_create_link(kobj, target, name, 0)
}

/// Remove symlink in object's directory, including tagged directories.
pub unsafe fn sysfs_delete_link(
    kobj: *mut kobject,
    targ: *mut kobject,
    name: *const c_char,
) {
    let mut ns: *const ns_common = core::ptr::null();

    spin_lock(&raw mut sysfs_symlink_target_lock);
    if !(*targ).sd.is_null() && kernfs_ns_enabled((*kobj).sd) {
        ns = (*(*targ).sd).ns;
    }
    spin_unlock(&raw mut sysfs_symlink_target_lock);
    kernfs_remove_by_name_ns((*kobj).sd, name, ns);
}

/// Remove symlink in object's directory.
pub unsafe fn sysfs_remove_link(kobj: *mut kobject, name: *const c_char) {
    let parent = if kobj.is_null() { sysfs_root_kn } else { (*kobj).sd };
    kernfs_remove_by_name(parent, name);
}

/// Rename symlink in object's directory.
pub unsafe fn sysfs_rename_link_ns(
    kobj: *mut kobject,
    targ: *mut kobject,
    old: *const c_char,
    new: *const c_char,
    new_ns: *const ns_common,
) -> c_int {
    let parent = if kobj.is_null() { sysfs_root_kn } else { (*kobj).sd };
    let mut old_ns: *const ns_common = core::ptr::null();
    if !(*targ).sd.is_null() {
        old_ns = (*(*targ).sd).ns;
    }

    let mut result = -ENOENT;
    let kn = kernfs_find_and_get_ns(parent, old, old_ns);
    if kn.is_null() {
        return result;
    }

    result = -EINVAL;
    if kernfs_type(kn) != KERNFS_LINK {
        kernfs_put(kn);
        return result;
    }
    if (*(*kn).symlink.target_kn).priv_ != targ {
        kernfs_put(kn);
        return result;
    }

    result = kernfs_rename_ns(kn, parent, new, new_ns);
    kernfs_put(kn);
    result
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
