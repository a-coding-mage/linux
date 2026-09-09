// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2019 IBM Corporation <nayna@linux.ibm.com>
 *
 * This code exposes secure variables to user via sysfs
 */

// C dependencies supplied by the kernel and architecture headers are external
// to this translation unit.

const NAME_MAX_SIZE: usize = 1024;

extern "C" {
    static mut secvar_kobj: *mut kobject;
    static mut secvar_kset: *mut kset;
    static mut secvar_ops: *mut secvar_operations;
    static mut firmware_kobj: *mut kobject;
    static kobj_sysfs_ops: kobj_type;
    static secvar_attr_groups: *const *const attribute_group;

    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> isize;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_warn(fmt: *const c_char, ...);
    fn pr_warn_ratelimited(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn kzalloc(size: usize, flags: gfp_t) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn kobject_init(kobj: *mut kobject, ktype: *const kobj_type);
    fn kobject_add(kobj: *mut kobject, parent: *mut kobject, fmt: *const c_char, ...)
        -> c_int;
    fn kobject_put(kobj: *mut kobject);
    fn kobject_uevent(kobj: *mut kobject, action: c_int) -> c_int;
    fn kobject_create_and_add(name: *const c_char, parent: *mut kobject) -> *mut kobject;
    fn kset_create_and_add(
        name: *const c_char,
        kset: *mut kset,
        parent: *mut kobject,
    ) -> *mut kset;
    fn sysfs_create_file(kobj: *mut kobject, attr: *const attribute) -> c_int;
    fn memory_read_from_buffer(
        to: *mut c_void,
        count: usize,
        ppos: *mut loff_t,
        from: *const c_void,
        available: usize,
    ) -> isize;
    fn strlen(s: *const c_char) -> usize;
    fn plpks_config_create_softlink(kobj: *mut kobject) -> c_int;
}

type c_char = i8;
type c_int = i32;
type c_void = core::ffi::c_void;
type loff_t = i64;
type gfp_t = usize;
type ssize_t = isize;
type size_t = usize;
type u64 = u64;

#[repr(C)]
struct kobject {
    name: *const c_char,
}
#[repr(C)] struct kset { kobj: kobject }
#[repr(C)] struct kobj_attribute { attr: attribute }
#[repr(C)] struct bin_attribute { size: u64 }
#[repr(C)] struct attribute { _private: usize }
#[repr(C)] struct attribute_group { attrs: *mut *mut attribute, bin_attrs: *const *const bin_attribute }
#[repr(C)] struct kobj_type { sysfs_ops: *const c_void, default_groups: *const *const attribute_group }
#[repr(C)] struct file;
#[repr(C)] struct secvar_operations {
    format: unsafe extern "C" fn(*mut c_char, usize) -> ssize_t,
    get: unsafe extern "C" fn(*const c_char, usize, *mut c_char, *mut u64) -> c_int,
    set: unsafe extern "C" fn(*const c_char, usize, *mut c_char, usize) -> c_int,
    max_size: unsafe extern "C" fn(*mut u64) -> c_int,
    get_next: Option<unsafe extern "C" fn(*mut c_char, *mut u64, usize) -> c_int>,
    var_names: *const *const c_char,
}

const GFP_KERNEL: gfp_t = 0;
const EIO: c_int = 5;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const ENOENT: c_int = 2;
const PAGE_SIZE: u64 = 4096;
const KOBJ_ADD: c_int = 0;

static mut format_attr: kobj_attribute = kobj_attribute { attr: attribute { _private: 0 } };
static mut size_attr: kobj_attribute = kobj_attribute { attr: attribute { _private: 0 } };
static mut data_attr: bin_attribute = bin_attribute { size: 0 };
static mut update_attr: bin_attribute = bin_attribute { size: 0 };
static mut secvar_bin_attrs: [*const bin_attribute; 3] = [core::ptr::null(), core::ptr::null(), core::ptr::null()];
static mut secvar_attrs: [*mut attribute; 2] = [core::ptr::null_mut(), core::ptr::null_mut()];
static secvar_attr_group: attribute_group = attribute_group { attrs: core::ptr::null_mut(), bin_attrs: core::ptr::null() };
static secvar_ktype: kobj_type = kobj_type { sysfs_ops: core::ptr::null(), default_groups: core::ptr::null() };

unsafe extern "C" fn format_show(_kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut tmp = [0i8; 32];
    let len = ((*secvar_ops).format)(tmp.as_mut_ptr(), tmp.len());
    if len > 0 { return sysfs_emit(buf, b"%s\n\0".as_ptr() as *const c_char, tmp.as_ptr()); }
    if len < 0 { pr_err(b"Error %zd reading format string\n\0".as_ptr() as *const c_char, len); }
    else { pr_err(b"Got empty format string from backend\n\0".as_ptr() as *const c_char); }
    -(EIO as ssize_t)
}

unsafe extern "C" fn size_show(kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut dsize = 0u64;
    let rc = ((*secvar_ops).get)((*kobj).name, strlen((*kobj).name) + 1, core::ptr::null_mut(), &mut dsize);
    if rc != 0 { if rc != -ENOENT { pr_err(b"Error retrieving %s variable size %d\n\0".as_ptr() as *const c_char, (*kobj).name, rc); } return rc as ssize_t; }
    sysfs_emit(buf, b"%llu\n\0".as_ptr() as *const c_char, dsize)
}

unsafe extern "C" fn data_read(_filep: *mut file, kobj: *mut kobject, _attr: *const bin_attribute, buf: *mut c_char, mut off: loff_t, count: size_t) -> ssize_t {
    let mut dsize = 0u64;
    let mut rc = ((*secvar_ops).get)((*kobj).name, strlen((*kobj).name) + 1, core::ptr::null_mut(), &mut dsize);
    if rc != 0 { if rc != -ENOENT { pr_err(b"Error getting %s variable size %d\n\0".as_ptr() as *const c_char, (*kobj).name, rc); } return rc as ssize_t; }
    pr_debug(b"dsize is %llu\n\0".as_ptr() as *const c_char, dsize);
    let data = kzalloc(dsize as usize, GFP_KERNEL);
    if data.is_null() { return -(ENOMEM as ssize_t); }
    rc = ((*secvar_ops).get)((*kobj).name, strlen((*kobj).name) + 1, data as *mut c_char, &mut dsize);
    if rc != 0 { pr_err(b"Error getting %s variable %d\n\0".as_ptr() as *const c_char, (*kobj).name, rc); kfree(data); return rc as ssize_t; }
    let ret = memory_read_from_buffer(buf as *mut c_void, count, &mut off, data, dsize as usize);
    kfree(data);
    ret
}

unsafe extern "C" fn update_write(_filep: *mut file, kobj: *mut kobject, _attr: *const bin_attribute, buf: *mut c_char, _off: loff_t, count: size_t) -> ssize_t {
    pr_debug(b"count is %ld\n\0".as_ptr() as *const c_char, count);
    let rc = ((*secvar_ops).set)((*kobj).name, strlen((*kobj).name) + 1, buf, count);
    if rc != 0 { pr_err(b"Error setting the %s variable %d\n\0".as_ptr() as *const c_char, (*kobj).name, rc); return rc as ssize_t; }
    count as ssize_t
}

unsafe extern "C" fn update_kobj_size() -> c_int { let mut varsize = 0u64; let rc = ((*secvar_ops).max_size)(&mut varsize); if rc != 0 { return rc; } data_attr.size = varsize; update_attr.size = varsize; 0 }

unsafe extern "C" fn add_var(name: *const c_char) -> c_int {
    let kobj = kzalloc(core::mem::size_of::<kobject>(), GFP_KERNEL) as *mut kobject;
    if kobj.is_null() { return -ENOMEM; }
    kobject_init(kobj, &secvar_ktype);
    let rc = kobject_add(kobj, &mut (*secvar_kset).kobj, b"%s\0".as_ptr() as *const c_char, name);
    if rc != 0 { pr_warn(b"kobject_add error %d for attribute: %s\n\0".as_ptr() as *const c_char, rc, name); kobject_put(kobj); return rc; }
    kobject_uevent(kobj, KOBJ_ADD); 0
}

unsafe extern "C" fn secvar_sysfs_load() -> c_int {
    let mut namesize = 0u64; let name = kzalloc(NAME_MAX_SIZE, GFP_KERNEL) as *mut c_char; if name.is_null() { return -ENOMEM; }
    let rc = loop { let mut r = ((*secvar_ops).get_next.unwrap())(name, &mut namesize, NAME_MAX_SIZE); if r != 0 { if r != -ENOENT { pr_err(b"error getting secvar from firmware %d\n\0".as_ptr() as *const c_char, r); } else { r = 0; } break r; } r = add_var(name); if r != 0 { break r; } };
    kfree(name as *mut c_void); rc
}

unsafe extern "C" fn secvar_sysfs_load_static() -> c_int { let mut name_ptr = (*secvar_ops).var_names; while !(*name_ptr).is_null() { let rc = add_var(*name_ptr); if rc != 0 { return rc; } name_ptr = name_ptr.add(1); } 0 }

unsafe extern "C" fn secvar_sysfs_init() -> c_int {
    if secvar_ops.is_null() { pr_warn(b"Failed to retrieve secvar operations\n\0".as_ptr() as *const c_char); return -ENODEV; }
    secvar_kobj = kobject_create_and_add(b"secvar\0".as_ptr() as *const c_char, firmware_kobj); if secvar_kobj.is_null() { pr_err(b"Failed to create firmware kobj\n\0".as_ptr() as *const c_char); return -ENOMEM; }
    let mut rc = sysfs_create_file(secvar_kobj, &format_attr.attr); if rc != 0 { pr_err(b"Failed to create format object\n\0".as_ptr() as *const c_char); rc = -ENOMEM; kobject_put(secvar_kobj); return rc; }
    secvar_kset = kset_create_and_add(b"vars\0".as_ptr() as *const c_char, core::ptr::null_mut(), secvar_kobj); if secvar_kset.is_null() { pr_err(b"sysfs kobject registration failed\n\0".as_ptr() as *const c_char); kobject_put(secvar_kobj); return -ENOMEM; }
    rc = update_kobj_size(); if rc != 0 { pr_err(b"Cannot read the size of the attribute\n\0".as_ptr() as *const c_char); kobject_put(secvar_kobj); return rc; }
    rc = plpks_config_create_softlink(secvar_kobj); if rc != 0 { pr_err(b"Failed to create softlink to PLPKS config directory\0".as_ptr() as *const c_char); kobject_put(secvar_kobj); return rc; }
    pr_info(b"/sys/firmware/secvar/config is now deprecated.\n\0".as_ptr() as *const c_char); pr_info(b"Will be removed in future versions.\n\0".as_ptr() as *const c_char);
    rc = if (*secvar_ops).get_next.is_some() { secvar_sysfs_load() } else { secvar_sysfs_load_static() }; if rc != 0 { pr_err(b"Failed to create variable attributes\n\0".as_ptr() as *const c_char); kobject_put(secvar_kobj); return rc; }
    let mut max_size = 0u64; ((*secvar_ops).max_size)(&mut max_size); if max_size > PAGE_SIZE { pr_warn_ratelimited(b"PAGE_SIZE (%lu) is smaller than maximum object size (%llu), writes are limited to PAGE_SIZE\n\0".as_ptr() as *const c_char, PAGE_SIZE, max_size); } 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
