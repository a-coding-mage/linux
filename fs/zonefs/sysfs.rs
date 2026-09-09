// SPDX-License-Identifier: GPL-2.0
/*
 * Simple file system for zoned block devices exposing zones as files.
 *
 * Copyright (C) 2022 Western Digital Corporation or its affiliates.
 */

// The Linux kernel headers and zonefs.h are external dependencies of this translation.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct zonefs_sysfs_attr {
    pub attr: attribute,
    pub show: Option<unsafe extern "C" fn(*mut zonefs_sb_info, *mut c_char) -> isize>,
}

// External kernel and zonefs types/functions supplied by other translation units.
#[repr(C)]
pub struct attribute {
    _private: [u8; 0],
}
#[repr(C)]
pub struct kobject {
    _private: [u8; 0],
}
#[repr(C)]
pub struct super_block {
    _private: [u8; 0],
}
#[repr(C)]
pub struct zonefs_sb_info {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sysfs_ops {
    pub show: Option<unsafe extern "C" fn(*mut kobject, *mut attribute, *mut c_char) -> isize>,
}
#[repr(C)]
pub struct kobj_type {
    pub default_groups: *mut *mut attribute_group,
    pub sysfs_ops: *const sysfs_ops,
    pub release: Option<unsafe extern "C" fn(*mut kobject)>,
}
#[repr(C)]
pub struct attribute_group {
    _private: [u8; 0],
}

extern "C" {
    static mut fs_kobj: *mut kobject;
    fn super_set_sysfs_name_id(sb: *mut super_block);
    fn init_completion(completion: *mut c_void);
    fn kobject_init_and_add(
        kobj: *mut kobject,
        ktype: *const kobj_type,
        parent: *mut kobject,
        fmt: *const c_char,
        ...,
    ) -> c_int;
    fn kobject_put(kobj: *mut kobject);
    fn wait_for_completion(completion: *mut c_void);
    fn kobject_del(kobj: *mut kobject);
    fn kobject_create_and_add(name: *const c_char, parent: *mut kobject) -> *mut kobject;
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> isize;
    fn atomic_read(v: *const c_void) -> c_int;
    fn complete(completion: *mut c_void);
}

extern "C" {
    fn zonefs_sb_from_super(sb: *mut super_block) -> *mut zonefs_sb_info;
}

unsafe extern "C" fn zonefs_sysfs_attr_show(
    _kobj: *mut kobject,
    _attr: *mut attribute,
    _buf: *mut c_char,
) -> isize {
    // container_of accesses and field offsets are supplied by zonefs.h/kernel definitions.
    // TODO: provide the kernel-layout-dependent container_of translation.
    0
}

unsafe extern "C" fn max_wro_seq_files_show(
    _sbi: *mut zonefs_sb_info,
    _buf: *mut c_char,
) -> isize {
    // sysfs_emit(buf, "%u\n", sbi->s_max_wro_seq_files)
    0
}

unsafe extern "C" fn nr_wro_seq_files_show(
    _sbi: *mut zonefs_sb_info,
    _buf: *mut c_char,
) -> isize {
    // sysfs_emit(buf, "%d\n", atomic_read(&sbi->s_wro_seq_files))
    0
}

unsafe extern "C" fn max_active_seq_files_show(
    _sbi: *mut zonefs_sb_info,
    _buf: *mut c_char,
) -> isize {
    // sysfs_emit(buf, "%u\n", sbi->s_max_active_seq_files)
    0
}

unsafe extern "C" fn nr_active_seq_files_show(
    _sbi: *mut zonefs_sb_info,
    _buf: *mut c_char,
) -> isize {
    // sysfs_emit(buf, "%d\n", atomic_read(&sbi->s_active_seq_files))
    0
}

static mut zonefs_sysfs_attrs: [*mut attribute; 5] = [
    core::ptr::null_mut(),
    core::ptr::null_mut(),
    core::ptr::null_mut(),
    core::ptr::null_mut(),
    core::ptr::null_mut(),
];
static mut zonefs_sysfs_groups: *mut attribute_group = core::ptr::null_mut();

unsafe extern "C" fn zonefs_sysfs_sb_release(_kobj: *mut kobject) {
    // complete(&sbi->s_kobj_unregister)
}

static zonefs_sysfs_attr_ops: sysfs_ops = sysfs_ops {
    show: Some(zonefs_sysfs_attr_show),
};

static zonefs_sb_ktype: kobj_type = kobj_type {
    default_groups: unsafe { &mut zonefs_sysfs_groups },
    sysfs_ops: &zonefs_sysfs_attr_ops,
    release: Some(zonefs_sysfs_sb_release),
};

static mut zonefs_sysfs_root: *mut kobject = core::ptr::null_mut();

#[no_mangle]
pub unsafe extern "C" fn zonefs_sysfs_register(sb: *mut super_block) -> c_int {
    let sbi = zonefs_sb_from_super(sb);
    let ret: c_int;

    super_set_sysfs_name_id(sb);
    // init_completion(&sbi->s_kobj_unregister)
    ret = kobject_init_and_add(
        sbi as *mut kobject,
        &zonefs_sb_ktype,
        zonefs_sysfs_root,
        b"%s\0".as_ptr() as *const c_char,
    );
    if ret != 0 {
        kobject_put(sbi as *mut kobject);
        // wait_for_completion(&sbi->s_kobj_unregister)
        return ret;
    }

    // sbi->s_sysfs_registered = true
    0
}

#[no_mangle]
pub unsafe extern "C" fn zonefs_sysfs_unregister(sb: *mut super_block) {
    let sbi = zonefs_sb_from_super(sb);

    if sbi.is_null() {
        return;
    }
    // if (!sbi->s_sysfs_registered) return;
    kobject_del(sbi as *mut kobject);
    kobject_put(sbi as *mut kobject);
    // wait_for_completion(&sbi->s_kobj_unregister)
}

#[no_mangle]
pub unsafe extern "C" fn zonefs_sysfs_init() -> c_int {
    zonefs_sysfs_root = kobject_create_and_add(b"zonefs\0".as_ptr() as *const c_char, fs_kobj);
    if zonefs_sysfs_root.is_null() {
        return -12; // -ENOMEM
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn zonefs_sysfs_exit() {
    kobject_put(zonefs_sysfs_root);
    zonefs_sysfs_root = core::ptr::null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
