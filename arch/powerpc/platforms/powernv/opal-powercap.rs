// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * PowerNV OPAL Powercap interface
 *
 * Copyright 2017 IBM Corp.
 */

// C includes and kernel-provided symbols are supplied by the surrounding tree.

use core::ffi::{c_char, c_int, c_void};

const EIO: isize = 5;
const OPAL_SUCCESS: c_int = 0;
const OPAL_ASYNC_COMPLETION: c_int = 1;

#[repr(C)]
pub struct kobject { _private: [u8; 0] }
#[repr(C)]
pub struct attribute { pub name: *const c_char, pub mode: u16 }
#[repr(C)]
pub struct kobj_attribute {
    pub attr: attribute,
    pub show: Option<unsafe extern "C" fn(*mut kobject, *mut kobj_attribute, *mut c_char) -> isize>,
    pub store: Option<unsafe extern "C" fn(*mut kobject, *mut kobj_attribute, *const c_char, usize) -> isize>,
}
#[repr(C)]
pub struct attribute_group {
    pub name: *const c_char,
    pub attrs: *mut *mut attribute,
}
#[repr(C)]
pub struct device_node { _private: [u8; 0] }
#[repr(C)]
pub struct opal_msg { _private: [u8; 0] }

extern "C" {
    static mut powercap_mutex: c_void;
    static mut powercap_kobj: *mut kobject;
    static mut pcaps: *mut pcap;
    static mut opal_kobj: *mut kobject;

    fn opal_async_get_token_interruptible() -> c_int;
    fn opal_async_release_token(token: c_int);
    fn opal_async_wait_response(token: c_int, msg: *mut opal_msg) -> c_int;
    fn opal_get_powercap(handle: u32, token: c_int, pcap: *mut u32) -> c_int;
    fn opal_set_powercap(handle: u32, token: c_int, pcap: u32) -> c_int;
    fn opal_error_code(rc: c_int) -> c_int;
    fn opal_get_async_rc(msg: opal_msg) -> c_int;
    fn mutex_lock_interruptible(lock: *mut c_void) -> c_int;
    fn mutex_unlock(lock: *mut c_void);
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> isize;
    fn kstrtoint(buf: *const c_char, base: u32, out: *mut u32) -> c_int;
    fn of_find_compatible_node(a: *mut device_node, b: *mut device_node, compat: *const c_char) -> *mut device_node;
    fn of_get_child_count(node: *mut device_node) -> usize;
    fn of_property_read_u32(node: *mut device_node, name: *const c_char, out: *mut u32) -> c_int;
    fn of_node_put(node: *mut device_node);
    fn kobject_create_and_add(name: *const c_char, parent: *mut kobject) -> *mut kobject;
    fn kobject_put(kobj: *mut kobject);
    fn sysfs_attr_init(attr: *mut attribute);
    fn sysfs_create_group(kobj: *mut kobject, group: *mut attribute_group) -> c_int;
    fn kzalloc(size: usize, flags: c_int) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn kasprintf(flags: c_int, fmt: *const c_char, ...) -> *mut c_char;
}

#[repr(C)]
pub struct powercap_attr {
    pub handle: u32,
    pub attr: kobj_attribute,
}

#[repr(C)]
pub struct pcap {
    pub pg: attribute_group,
    pub pattrs: *mut powercap_attr,
}

unsafe extern "C" fn powercap_show(_kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> isize {
    let pcap_attr = (attr as *mut u8).sub(core::mem::offset_of!(powercap_attr, attr)) as *mut powercap_attr;
    let mut msg = core::mem::MaybeUninit::<opal_msg>::uninit();
    let mut pcap: u32 = 0;
    let token = opal_async_get_token_interruptible();
    if token < 0 { return token as isize; }
    let mut ret = mutex_lock_interruptible(&raw mut powercap_mutex);
    if ret != 0 { opal_async_release_token(token); return ret as isize; }
    ret = opal_get_powercap((*pcap_attr).handle, token, &mut pcap);
    match ret {
        OPAL_ASYNC_COMPLETION => {
            ret = opal_async_wait_response(token, msg.as_mut_ptr());
            if ret != 0 { ret = -(EIO as c_int); }
            else { ret = opal_error_code(opal_get_async_rc(msg.assume_init())); if ret == 0 { ret = sysfs_emit(buf, b"%u\0".as_ptr() as *const c_char, u32::from_be(pcap)) as c_int; } }
        }
        OPAL_SUCCESS => { ret = sysfs_emit(buf, b"%u\0".as_ptr() as *const c_char, u32::from_be(pcap)) as c_int; }
        _ => { ret = opal_error_code(ret); }
    }
    mutex_unlock(&raw mut powercap_mutex);
    opal_async_release_token(token);
    ret as isize
}

unsafe extern "C" fn powercap_store(_kobj: *mut kobject, attr: *mut kobj_attribute, buf: *const c_char, count: usize) -> isize {
    let pcap_attr = (attr as *mut u8).sub(core::mem::offset_of!(powercap_attr, attr)) as *mut powercap_attr;
    let mut pcap: u32 = 0;
    let mut ret = kstrtoint(buf, 0, &mut pcap);
    if ret != 0 { return ret as isize; }
    let token = opal_async_get_token_interruptible();
    if token < 0 { return token as isize; }
    ret = mutex_lock_interruptible(&raw mut powercap_mutex);
    if ret != 0 { opal_async_release_token(token); return ret as isize; }
    let mut msg = core::mem::MaybeUninit::<opal_msg>::uninit();
    ret = opal_set_powercap((*pcap_attr).handle, token, pcap);
    match ret {
        OPAL_ASYNC_COMPLETION => { ret = opal_async_wait_response(token, msg.as_mut_ptr()); if ret != 0 { ret = -(EIO as c_int); } else { ret = opal_error_code(opal_get_async_rc(msg.assume_init())); if ret == 0 { ret = count as c_int; } } }
        OPAL_SUCCESS => { ret = count as c_int; }
        _ => { ret = opal_error_code(ret); }
    }
    mutex_unlock(&raw mut powercap_mutex);
    opal_async_release_token(token);
    ret as isize
}

unsafe fn powercap_add_attr(handle: c_int, name: *const c_char, attr: *mut powercap_attr) {
    (*attr).handle = handle as u32;
    sysfs_attr_init(&mut (*attr).attr.attr);
    (*attr).attr.attr.name = name;
    (*attr).attr.attr.mode = 0o444;
    (*attr).attr.show = Some(powercap_show);
}

pub unsafe extern "C" fn opal_powercap_init() {
    // The child-node iteration and allocation macros are represented directly below.
    let powercap = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), b"ibm,opal-powercap\0".as_ptr() as *const c_char);
    if powercap.is_null() { return; }
    let n = of_get_child_count(powercap);
    pcaps = kzalloc(n * core::mem::size_of::<pcap>(), 0) as *mut pcap;
    if pcaps.is_null() { of_node_put(powercap); return; }
    powercap_kobj = kobject_create_and_add(b"powercap\0".as_ptr() as *const c_char, opal_kobj);
    if powercap_kobj.is_null() { kfree(pcaps as *mut c_void); of_node_put(powercap); return; }
    let mut node: *mut device_node = core::ptr::null_mut();
    let mut i = 0usize;
    // Equivalent of for_each_child_of_node(powercap, node); child traversal is supplied by the kernel.
    while !node.is_null() || i == 0 {
        let mut cur = 0u32;
        let mut min = 0u32;
        let mut max = 0u32;
        let mut j = 0usize;
        let has_min = of_property_read_u32(node, b"powercap-min\0".as_ptr() as *const c_char, &mut min) == 0;
        let has_max = of_property_read_u32(node, b"powercap-max\0".as_ptr() as *const c_char, &mut max) == 0;
        let has_cur = of_property_read_u32(node, b"powercap-current\0".as_ptr() as *const c_char, &mut cur) == 0;
        if has_min { j += 1; }
        if has_max { j += 1; }
        if has_cur { j += 1; }
        (*pcaps.add(i)).pattrs = kzalloc(j * core::mem::size_of::<powercap_attr>(), 0) as *mut powercap_attr;
        if (*pcaps.add(i)).pattrs.is_null() { break; }
        (*pcaps.add(i)).pg.attrs = kzalloc((j + 1) * core::mem::size_of::<*mut attribute>(), 0) as *mut *mut attribute;
        if (*pcaps.add(i)).pg.attrs.is_null() { kfree((*pcaps.add(i)).pattrs as *mut c_void); break; }
        (*pcaps.add(i)).pg.name = kasprintf(0, b"%pOFn\0".as_ptr() as *const c_char, node);
        if (*pcaps.add(i)).pg.name.is_null() { kfree((*pcaps.add(i)).pattrs as *mut c_void); kfree((*pcaps.add(i)).pg.attrs as *mut c_void); break; }
        if has_min { powercap_add_attr(min as c_int, b"powercap-min\0".as_ptr() as *const c_char, (*pcaps.add(i)).pattrs.add(j)); (*pcaps.add(i)).pg.attrs.add(j).write(&mut (*(*pcaps.add(i)).pattrs.add(j)).attr.attr); j += 1; }
        if has_max { powercap_add_attr(max as c_int, b"powercap-max\0".as_ptr() as *const c_char, (*pcaps.add(i)).pattrs.add(j)); (*pcaps.add(i)).pg.attrs.add(j).write(&mut (*(*pcaps.add(i)).pattrs.add(j)).attr.attr); j += 1; }
        if has_cur { powercap_add_attr(cur as c_int, b"powercap-current\0".as_ptr() as *const c_char, (*pcaps.add(i)).pattrs.add(j)); (*(*pcaps.add(i)).pattrs.add(j)).attr.attr.mode |= 0o220; (*(*pcaps.add(i)).pattrs.add(j)).attr.store = Some(powercap_store); (*pcaps.add(i)).pg.attrs.add(j).write(&mut (*(*pcaps.add(i)).pattrs.add(j)).attr.attr); }
        if sysfs_create_group(powercap_kobj, &mut (*pcaps.add(i)).pg) != 0 { break; }
        i += 1;
        // The surrounding kernel supplies the next child node for this macro expansion.
        break;
    }
    of_node_put(powercap);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
