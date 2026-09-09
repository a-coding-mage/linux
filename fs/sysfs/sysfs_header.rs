/* SPDX-License-Identifier: GPL-2.0 */
/*
 * fs/sysfs/sysfs.h - sysfs internal header file
 *
 * Copyright (c) 2001-3 Patrick Mochel
 * Copyright (c) 2007 SUSE Linux Products GmbH
 * Copyright (c) 2007 Tejun Heo <teheo@suse.de>
 */

/* Declarations supplied by linux/sysfs.h and related dependencies. */

/*
 * mount.c
 */
extern "C" {
    pub static mut sysfs_root_kn: *mut kernfs_node;
}

/*
 * dir.c
 */
extern "C" {
    pub static mut sysfs_symlink_target_lock: spinlock_t;

    pub fn sysfs_warn_dup(parent: *mut kernfs_node, name: *const ::std::os::raw::c_char);
}

/*
 * file.c
 */
extern "C" {
    pub fn sysfs_add_file_mode_ns(
        parent: *mut kernfs_node,
        attr: *const attribute,
        amode: umode_t,
        uid: kuid_t,
        gid: kgid_t,
        ns: *const ns_common,
    ) -> ::std::os::raw::c_int;

    pub fn sysfs_add_bin_file_mode_ns(
        parent: *mut kernfs_node,
        battr: *const bin_attribute,
        mode: umode_t,
        size: usize,
        uid: kuid_t,
        gid: kgid_t,
        ns: *const ns_common,
    ) -> ::std::os::raw::c_int;
}

/*
 * symlink.c
 */
extern "C" {
    pub fn sysfs_create_link_sd(
        kn: *mut kernfs_node,
        target: *mut kobject,
        name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
