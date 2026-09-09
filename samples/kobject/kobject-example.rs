// SPDX-License-Identifier: GPL-2.0
/*
 * Sample kobject implementation
 *
 * Copyright (C) 2004-2007 Greg Kroah-Hartman <greg@kroah.com>
 * Copyright (C) 2007 Novell Inc.
 */

/* Linux kernel headers and their definitions are supplied by the surrounding build. */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct kobject {
    _private: [u8; 0],
}

#[repr(C)]
pub struct attribute {
    pub name: *const c_char,
    pub mode: u16,
}

#[repr(C)]
pub struct kobj_attribute {
    pub attr: attribute,
    pub show: Option<unsafe extern "C" fn(*mut kobject, *const kobj_attribute, *mut c_char) -> isize>,
    pub store: Option<unsafe extern "C" fn(*mut kobject, *const kobj_attribute, *const c_char, usize) -> isize>,
}

#[repr(C)]
pub struct attribute_group {
    pub attrs_const: *const *const attribute,
}

extern "C" {
    static mut kernel_kobj: *mut kobject;
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> isize;
    fn kstrtoint(s: *const c_char, base: u32, res: *mut c_int) -> c_int;
    fn strcmp(lhs: *const c_char, rhs: *const c_char) -> c_int;
    fn kobject_create_and_add(name: *const c_char, parent: *mut kobject) -> *mut kobject;
    fn sysfs_create_group(kobj: *mut kobject, grp: *const attribute_group) -> c_int;
    fn kobject_put(kobj: *mut kobject);
}

const ENOMEM: c_int = 12;

/*
 * This module shows how to create a simple subdirectory in sysfs called
 * /sys/kernel/kobject_example  In that directory, 3 files are created:
 * "foo", "baz", and "bar".  If an integer is written to these files, it can be
 * later read out of it.
 */

static mut foo: c_int = 0;
static mut baz: c_int = 0;
static mut bar: c_int = 0;

/* The "foo" file where a static variable is read from and written to. */
unsafe extern "C" fn foo_show(_kobj: *mut kobject, _attr: *const kobj_attribute, buf: *mut c_char) -> isize {
    sysfs_emit(buf, b"%d\n\0".as_ptr() as *const c_char, foo)
}

unsafe extern "C" fn foo_store(_kobj: *mut kobject, _attr: *const kobj_attribute, buf: *const c_char, count: usize) -> isize {
    let ret = kstrtoint(buf, 10, &mut foo);
    if ret < 0 {
        return ret as isize;
    }
    count as isize
}

/* Sysfs attributes cannot be world-writable. */
static mut foo_attribute: kobj_attribute = kobj_attribute {
    attr: attribute { name: b"foo\0".as_ptr() as *const c_char, mode: 0o664 },
    show: Some(foo_show),
    store: Some(foo_store),
};

/* More complex function where we determine which variable is being accessed. */
unsafe extern "C" fn b_show(_kobj: *mut kobject, attr: *const kobj_attribute, buf: *mut c_char) -> isize {
    let var = if strcmp((*attr).attr.name, b"baz\0".as_ptr() as *const c_char) == 0 { baz } else { bar };
    sysfs_emit(buf, b"%d\n\0".as_ptr() as *const c_char, var)
}

unsafe extern "C" fn b_store(_kobj: *mut kobject, attr: *const kobj_attribute, buf: *const c_char, count: usize) -> isize {
    let mut var: c_int = 0;
    let ret = kstrtoint(buf, 10, &mut var);
    if ret < 0 {
        return ret as isize;
    }
    if strcmp((*attr).attr.name, b"baz\0".as_ptr() as *const c_char) == 0 { baz = var; } else { bar = var; }
    count as isize
}

static mut baz_attribute: kobj_attribute = kobj_attribute {
    attr: attribute { name: b"baz\0".as_ptr() as *const c_char, mode: 0o664 }, show: Some(b_show), store: Some(b_store),
};
static mut bar_attribute: kobj_attribute = kobj_attribute {
    attr: attribute { name: b"bar\0".as_ptr() as *const c_char, mode: 0o664 }, show: Some(b_show), store: Some(b_store),
};

static mut attrs: [*const attribute; 4] = [
    unsafe { &foo_attribute.attr }, unsafe { &baz_attribute.attr }, unsafe { &bar_attribute.attr }, core::ptr::null(),
];

static mut attr_group: attribute_group = attribute_group { attrs_const: unsafe { attrs.as_ptr() } };
static mut example_kobj: *mut kobject = core::ptr::null_mut();

unsafe extern "C" fn example_init() -> c_int {
    example_kobj = kobject_create_and_add(b"kobject_example\0".as_ptr() as *const c_char, kernel_kobj);
    if example_kobj.is_null() { return -ENOMEM; }
    let retval = sysfs_create_group(example_kobj, &attr_group);
    if retval != 0 { kobject_put(example_kobj); }
    retval
}

unsafe extern "C" fn example_exit() {
    kobject_put(example_kobj);
}

/* module_init(example_init); module_exit(example_exit); */
/* MODULE_DESCRIPTION("Sample kobject implementation"); */
/* MODULE_LICENSE("GPL v2"); */
/* MODULE_AUTHOR("Greg Kroah-Hartman <greg@kroah.com>"); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
