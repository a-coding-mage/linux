// SPDX-License-Identifier: GPL-2.0
/*
 * Sample kset and ktype implementation
 *
 * Copyright (C) 2004-2007 Greg Kroah-Hartman <greg@kroah.com>
 * Copyright (C) 2007 Novell Inc.
 */
// Dependencies supplied by the Linux kernel headers are intentionally external.

/*
 * This module shows how to create a kset in sysfs called
 * /sys/kernel/kset_example
 * Then three kobjects are created and assigned to this kset, "foo", "baz",
 * and "bar".  In those kobjects, attributes of the same name are also
 * created and if an integer is written to these files, it can be later
 * read out of it.
 */

#[repr(C)]
pub struct kobject {
    pub kset: *mut kset,
    _private: [u8; 0],
}
#[repr(C)]
pub struct kset {
    _private: [u8; 0],
}
#[repr(C)]
pub struct attribute {
    pub name: *const ::std::os::raw::c_char,
    pub mode: umode_t,
}
#[repr(C)]
pub struct sysfs_ops {
    pub show: Option<unsafe extern "C" fn(*mut kobject, *mut attribute, *mut ::std::os::raw::c_char) -> ssize_t>,
    pub store: Option<unsafe extern "C" fn(*mut kobject, *mut attribute, *const ::std::os::raw::c_char, usize) -> ssize_t>,
}
#[repr(C)]
pub struct attribute_group {
    pub attrs_const: *const *const attribute,
    pub is_visible_const: Option<unsafe extern "C" fn(*mut kobject, *const attribute, i32) -> umode_t>,
}
#[repr(C)]
pub struct kobj_type {
    pub sysfs_ops: *const sysfs_ops,
    pub release: Option<unsafe extern "C" fn(*mut kobject)>,
    pub default_groups: *const *const attribute_group,
}

pub type ssize_t = isize;
pub type umode_t = u16;

extern "C" {
    fn kfree(ptr: *mut ::std::ffi::c_void);
    fn kstrtoint(buf: *const ::std::os::raw::c_char, base: u32, out: *mut i32) -> i32;
    fn sysfs_emit(buf: *mut ::std::os::raw::c_char, fmt: *const ::std::os::raw::c_char, ...) -> ssize_t;
    fn strcmp(a: *const ::std::os::raw::c_char, b: *const ::std::os::raw::c_char) -> i32;
    fn kobject_name(kobj: *const kobject) -> *const ::std::os::raw::c_char;
    fn kobject_init_and_add(kobj: *mut kobject, ktype: *const kobj_type, parent: *mut kobject, fmt: *const ::std::os::raw::c_char, ...) -> i32;
    fn kobject_put(kobj: *mut kobject);
    fn kobject_uevent(kobj: *mut kobject, action: i32) -> i32;
    fn kset_create_and_add(name: *const ::std::os::raw::c_char, uevent_ops: *const ::std::ffi::c_void, parent: *mut kobject) -> *mut kset;
    fn kset_unregister(kset: *mut kset);
    static mut kernel_kobj: *mut kobject;
}

const EIO: i32 = 5;
const ENOMEM: i32 = 12;
const EINVAL: i32 = 22;
const KOBJ_ADD: i32 = 0;

/*
 * This is our "object" that we will create a few of and register them with
 * sysfs.
 */
#[repr(C)]
pub struct foo_obj {
    pub kobj: kobject,
    pub foo: i32,
    pub baz: i32,
    pub bar: i32,
}

/* a custom attribute that works just for a struct foo_obj. */
#[repr(C)]
pub struct foo_attribute {
    pub attr: attribute,
    pub show: Option<unsafe extern "C" fn(*mut foo_obj, *const foo_attribute, *mut ::std::os::raw::c_char) -> ssize_t>,
    pub store: Option<unsafe extern "C" fn(*mut foo_obj, *const foo_attribute, *const ::std::os::raw::c_char, usize) -> ssize_t>,
}

/* The default show function that must be passed to sysfs. */
unsafe extern "C" fn foo_attr_show(kobj: *mut kobject, attr: *mut attribute, buf: *mut ::std::os::raw::c_char) -> ssize_t {
    let attribute = (attr as *const u8).sub(std::mem::offset_of!(foo_attribute, attr)) as *const foo_attribute;
    let foo = (kobj as *mut u8).sub(std::mem::offset_of!(foo_obj, kobj)) as *mut foo_obj;
    match (*attribute).show { Some(show) => show(foo, attribute, buf), None => -(EIO as ssize_t) }
}

/* Just like the default show function above, but for sysfs store requests. */
unsafe extern "C" fn foo_attr_store(kobj: *mut kobject, attr: *mut attribute, buf: *const ::std::os::raw::c_char, len: usize) -> ssize_t {
    let attribute = (attr as *const u8).sub(std::mem::offset_of!(foo_attribute, attr)) as *const foo_attribute;
    let foo = (kobj as *mut u8).sub(std::mem::offset_of!(foo_obj, kobj)) as *mut foo_obj;
    match (*attribute).store { Some(store) => store(foo, attribute, buf, len), None => -(EIO as ssize_t) }
}

/* Our custom sysfs_ops that we associate with our ktype. */
static foo_sysfs_ops: sysfs_ops = sysfs_ops { show: Some(foo_attr_show), store: Some(foo_attr_store) };

/* The release function for our object. */
unsafe extern "C" fn foo_release(kobj: *mut kobject) {
    let foo = (kobj as *mut u8).sub(std::mem::offset_of!(foo_obj, kobj)) as *mut foo_obj;
    kfree(foo as *mut ::std::ffi::c_void);
}

unsafe extern "C" fn foo_show(foo_obj: *mut foo_obj, _attr: *const foo_attribute, buf: *mut ::std::os::raw::c_char) -> ssize_t {
    static FMT: &[u8] = b"%d\n\0";
    sysfs_emit(buf, FMT.as_ptr() as *const _, (*foo_obj).foo)
}
unsafe extern "C" fn foo_store(foo_obj: *mut foo_obj, _attr: *const foo_attribute, buf: *const ::std::os::raw::c_char, count: usize) -> ssize_t {
    let ret = kstrtoint(buf, 10, &mut (*foo_obj).foo);
    if ret < 0 { return ret as ssize_t; }
    count as ssize_t
}

/* Sysfs attributes cannot be world-writable. */
static foo_attribute: foo_attribute = foo_attribute { attr: attribute { name: b"foo\0".as_ptr() as *const _, mode: 0o664 }, show: Some(foo_show), store: Some(foo_store) };

unsafe extern "C" fn b_show(foo_obj: *mut foo_obj, attr: *const foo_attribute, buf: *mut ::std::os::raw::c_char) -> ssize_t {
    let baz = strcmp((*attr).attr.name, b"baz\0".as_ptr() as *const _) == 0;
    let var = if baz { (*foo_obj).baz } else { (*foo_obj).bar };
    static FMT: &[u8] = b"%d\n\0";
    sysfs_emit(buf, FMT.as_ptr() as *const _, var)
}
unsafe extern "C" fn b_store(foo_obj: *mut foo_obj, attr: *const foo_attribute, buf: *const ::std::os::raw::c_char, count: usize) -> ssize_t {
    let mut var = 0;
    let ret = kstrtoint(buf, 10, &mut var);
    if ret < 0 { return ret as ssize_t; }
    if strcmp((*attr).attr.name, b"baz\0".as_ptr() as *const _) == 0 { (*foo_obj).baz = var; } else { (*foo_obj).bar = var; }
    count as ssize_t
}

static baz_attribute: foo_attribute = foo_attribute { attr: attribute { name: b"baz\0".as_ptr() as *const _, mode: 0o664 }, show: Some(b_show), store: Some(b_store) };
static bar_attribute: foo_attribute = foo_attribute { attr: attribute { name: b"bar\0".as_ptr() as *const _, mode: 0o664 }, show: Some(b_show), store: Some(b_store) };

/* Create a group of attributes so that we can create and destroy them all at once. */
static foo_default_attrs: [*const attribute; 4] = [
    &foo_attribute.attr, &baz_attribute.attr, &bar_attribute.attr, std::ptr::null(),
];

unsafe extern "C" fn foo_default_attrs_is_visible(kobj: *mut kobject, attr: *const attribute, _n: i32) -> umode_t {
    if strcmp(kobject_name(kobj), (*attr).name) == 0 { 0 } else { (*attr).mode }
}
static foo_default_group: attribute_group = attribute_group { attrs_const: foo_default_attrs.as_ptr(), is_visible_const: Some(foo_default_attrs_is_visible) };
static foo_default_groups: [*const attribute_group; 2] = [&foo_default_group, std::ptr::null()];

static foo_ktype: kobj_type = kobj_type { sysfs_ops: &foo_sysfs_ops, release: Some(foo_release), default_groups: foo_default_groups.as_ptr() };

static mut example_kset: *mut kset = std::ptr::null_mut();
static mut foo_obj: *mut foo_obj = std::ptr::null_mut();
static mut bar_obj: *mut foo_obj = std::ptr::null_mut();
static mut baz_obj: *mut foo_obj = std::ptr::null_mut();

// Allocation and GFP_KERNEL are supplied by the kernel environment.
unsafe fn create_foo_obj(name: *const ::std::os::raw::c_char) -> *mut foo_obj {
    let foo = kzalloc(std::mem::size_of::<foo_obj>(), GFP_KERNEL) as *mut foo_obj;
    if foo.is_null() { return std::ptr::null_mut(); }
    (*foo).kobj.kset = example_kset;
    let mut retval = kobject_init_and_add(&mut (*foo).kobj, &foo_ktype, std::ptr::null_mut(), b"%s\0".as_ptr() as *const _, name);
    if retval != 0 { kobject_put(&mut (*foo).kobj); return std::ptr::null_mut(); }
    retval = kobject_uevent(&mut (*foo).kobj, KOBJ_ADD);
    let _ = retval;
    foo
}

unsafe fn destroy_foo_obj(foo: *mut foo_obj) { kobject_put(&mut (*foo).kobj); }

unsafe fn example_init() -> i32 {
    example_kset = kset_create_and_add(b"kset_example\0".as_ptr() as *const _, std::ptr::null(), kernel_kobj);
    if example_kset.is_null() { return -ENOMEM; }
    foo_obj = create_foo_obj(b"foo\0".as_ptr() as *const _);
    if foo_obj.is_null() { kset_unregister(example_kset); return -EINVAL; }
    bar_obj = create_foo_obj(b"bar\0".as_ptr() as *const _);
    if bar_obj.is_null() { destroy_foo_obj(foo_obj); kset_unregister(example_kset); return -EINVAL; }
    baz_obj = create_foo_obj(b"baz\0".as_ptr() as *const _);
    if baz_obj.is_null() { destroy_foo_obj(bar_obj); destroy_foo_obj(foo_obj); kset_unregister(example_kset); return -EINVAL; }
    0
}

unsafe fn example_exit() {
    destroy_foo_obj(baz_obj); destroy_foo_obj(bar_obj); destroy_foo_obj(foo_obj); kset_unregister(example_kset);
}

extern "C" {
    fn kzalloc(size: usize, flags: usize) -> *mut ::std::ffi::c_void;
}
const GFP_KERNEL: usize = 0;
// module_init(example_init); module_exit(example_exit);
// MODULE_DESCRIPTION("Sample kset and ktype implementation");
// MODULE_LICENSE("GPL v2");
// MODULE_AUTHOR("Greg Kroah-Hartman <greg@kroah.com>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
