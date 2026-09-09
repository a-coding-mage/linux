// SPDX-License-Identifier: GPL-2.0
/*
 * Common Code for DAMON Sysfs Interface
 */

// Dependencies supplied by the surrounding kernel/Rust translation.

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static mut damon_sysfs_lock: mutex;

    fn kmalloc_obj<T>() -> *mut T;
    fn kfree(ptr: *mut c_void);
    fn kstrtoul(buf: *const c_char, base: c_uint, out: *mut c_ulong) -> c_int;
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> isize;
    fn cgroup_path(cgroup: *mut cgroup, buf: *mut c_char, buflen: c_int) -> c_int;
    fn sysfs_streq(s1: *const c_char, s2: *const c_char) -> bool;
    fn mem_cgroup_iter(
        root: *mut mem_cgroup,
        prev: *mut mem_cgroup,
        cond: *mut c_void,
    ) -> *mut mem_cgroup;
    fn mem_cgroup_online(memcg: *mut mem_cgroup) -> bool;
    fn mem_cgroup_id(memcg: *mut mem_cgroup) -> u64;
    fn mem_cgroup_iter_break(root: *mut mem_cgroup, prev: *mut mem_cgroup);
}

type c_uint = u32;
type c_ulong = usize;

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kobject {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kobj_attribute {
    _private: [u8; 0],
}

#[repr(C)]
pub struct attribute {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kobj_type {
    pub release: Option<unsafe extern "C" fn(*mut kobject)>,
    pub sysfs_ops: *const c_void,
    pub default_groups: *const *const attribute_group,
}

#[repr(C)]
pub struct attribute_group {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cgroup {
    _private: [u8; 0],
}

#[repr(C)]
pub struct css {
    pub cgroup: *mut cgroup,
}

#[repr(C)]
pub struct mem_cgroup {
    pub css: css,
}

#[repr(C)]
pub struct damon_sysfs_ul_range {
    pub kobj: kobject,
    pub min: c_ulong,
    pub max: c_ulong,
}

pub unsafe fn damon_sysfs_ul_range_alloc(
    min: c_ulong,
    max: c_ulong,
) -> *mut damon_sysfs_ul_range {
    let range = kmalloc_obj::<damon_sysfs_ul_range>();

    if range.is_null() {
        return core::ptr::null_mut();
    }
    (*range).kobj = core::mem::zeroed();
    (*range).min = min;
    (*range).max = max;

    range
}

unsafe extern "C" fn min_show(
    kobj: *mut kobject,
    _attr: *mut kobj_attribute,
    buf: *mut c_char,
) -> isize {
    let range = container_of_kobj(kobj);
    sysfs_emit(buf, b"%lu\n\0".as_ptr() as *const c_char, (*range).min) as isize
}

unsafe extern "C" fn min_store(
    kobj: *mut kobject,
    _attr: *mut kobj_attribute,
    buf: *const c_char,
    count: usize,
) -> isize {
    let range = container_of_kobj(kobj);
    let mut min = 0;
    let err = kstrtoul(buf, 0, &mut min);
    if err != 0 {
        return err as isize;
    }
    (*range).min = min;
    count as isize
}

unsafe extern "C" fn max_show(
    kobj: *mut kobject,
    _attr: *mut kobj_attribute,
    buf: *mut c_char,
) -> isize {
    let range = container_of_kobj(kobj);
    sysfs_emit(buf, b"%lu\n\0".as_ptr() as *const c_char, (*range).max) as isize
}

unsafe extern "C" fn max_store(
    kobj: *mut kobject,
    _attr: *mut kobj_attribute,
    buf: *const c_char,
    count: usize,
) -> isize {
    let range = container_of_kobj(kobj);
    let mut max = 0;
    let err = kstrtoul(buf, 0, &mut max);
    if err != 0 {
        return err as isize;
    }
    (*range).max = max;
    count as isize
}

pub unsafe extern "C" fn damon_sysfs_ul_range_release(kobj: *mut kobject) {
    kfree(container_of_kobj(kobj) as *mut c_void);
}

static mut damon_sysfs_ul_range_min_attr: kobj_attribute = kobj_attribute { _private: [] };
static mut damon_sysfs_ul_range_max_attr: kobj_attribute = kobj_attribute { _private: [] };
static mut damon_sysfs_ul_range_attrs: [*mut attribute; 3] = [
    core::ptr::null_mut(),
    core::ptr::null_mut(),
    core::ptr::null_mut(),
];
static mut damon_sysfs_ul_range_groups: *const attribute_group = core::ptr::null();

pub static damon_sysfs_ul_range_ktype: kobj_type = kobj_type {
    release: Some(damon_sysfs_ul_range_release),
    sysfs_ops: core::ptr::null(),
    default_groups: unsafe { &damon_sysfs_ul_range_groups },
};

unsafe fn container_of_kobj(kobj: *mut kobject) -> *mut damon_sysfs_ul_range {
    kobj as *mut damon_sysfs_ul_range
}

unsafe fn damon_sysfs_memcg_path_eq(
    memcg: *mut mem_cgroup,
    memcg_path_buf: *mut c_char,
    path: *mut c_char,
) -> bool {
    // CONFIG_MEMCG condition is supplied by the build configuration.
    cgroup_path((*memcg).css.cgroup, memcg_path_buf, 4096);
    sysfs_streq(memcg_path_buf, path)
}

pub unsafe fn damon_sysfs_memcg_path_to_id(
    memcg_path: *mut c_char,
    id: *mut u64,
) -> c_int {
    let mut memcg: *mut mem_cgroup;
    let path: *mut c_char;
    let mut found = false;

    if memcg_path.is_null() {
        return -22;
    }
    path = kmalloc_obj::<[c_char; 4096]>() as *mut c_char;
    if path.is_null() {
        return -12;
    }

    memcg = mem_cgroup_iter(core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    while !memcg.is_null() {
        // skip offlined memcg
        if !mem_cgroup_online(memcg) {
            memcg = mem_cgroup_iter(core::ptr::null_mut(), memcg, core::ptr::null_mut());
            continue;
        }
        if damon_sysfs_memcg_path_eq(memcg, path, memcg_path) {
            *id = mem_cgroup_id(memcg);
            found = true;
            mem_cgroup_iter_break(core::ptr::null_mut(), memcg);
            break;
        }
        memcg = mem_cgroup_iter(core::ptr::null_mut(), memcg, core::ptr::null_mut());
    }

    kfree(path as *mut c_void);
    if found { 0 } else { -22 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
