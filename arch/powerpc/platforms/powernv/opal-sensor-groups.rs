// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * PowerNV OPAL Sensor-groups interface
 *
 * Copyright 2017 IBM Corp.
 */

// Translated from the Linux kernel implementation. External kernel and OPAL
// declarations are supplied by the surrounding Rust translation environment.

use core::ffi::c_char;

extern "C" {
    static mut sg_mutex: mutex;
    static mut sg_kobj: *mut kobject;
    static mut sgs: *mut sensor_group;

    fn opal_async_get_token_interruptible() -> i32;
    fn opal_sensor_group_enable(handle: u32, token: i32, enable: bool) -> i32;
    fn opal_async_wait_response(token: i32, msg: *mut opal_msg) -> i32;
    fn opal_error_code(rc: i32) -> i32;
    fn opal_get_async_rc(msg: opal_msg) -> i32;
    fn opal_async_release_token(token: i32);
    fn opal_sensor_group_clear(handle: u32, token: i32) -> i32;
    fn mutex_lock_interruptible(lock: *mut mutex) -> i32;
    fn mutex_unlock(lock: *mut mutex);
    fn kstrtoint(buf: *const c_char, base: u32, out: *mut u32) -> i32;
    fn sysfs_attr_init(attr: *mut attribute);
    fn sysfs_create_group(kobj: *mut kobject, group: *mut attribute_group) -> i32;
    fn of_find_compatible_node(
        from: *mut device_node,
        typ: *const c_char,
        compatible: *const c_char,
    ) -> *mut device_node;
    fn of_get_child_count(node: *mut device_node) -> usize;
    fn of_get_property(node: *mut device_node, name: *const c_char, len: *mut u32) -> *const u32;
    fn of_property_read_u32(node: *mut device_node, name: *const c_char, out: *mut u32) -> i32;
    fn of_node_put(node: *mut device_node);
    fn kobject_create_and_add(name: *const c_char, parent: *mut kobject) -> *mut kobject;
    fn kobject_put(kobj: *mut kobject);
    fn kfree(ptr: *mut core::ffi::c_void);
    fn sprintf(dst: *mut c_char, fmt: *const c_char, ...) -> i32;

    static mut opal_kobj: *mut kobject;
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kobject {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct opal_msg {
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
    pub show: Option<unsafe extern "C" fn(*mut kobject, *mut kobj_attribute, *mut c_char) -> isize>,
    pub store: Option<unsafe extern "C" fn(*mut kobject, *mut kobj_attribute, *const c_char, usize) -> isize>,
}

#[repr(C)]
pub struct attribute_group {
    pub name: *const c_char,
    pub attrs: *mut *mut attribute,
}

#[repr(C)]
pub struct sg_attr {
    pub handle: u32,
    pub attr: kobj_attribute,
}

#[repr(C)]
pub struct sensor_group {
    pub name: [c_char; 20],
    pub sg: attribute_group,
    pub sgattrs: *mut sg_attr,
}

#[repr(C)]
struct sg_ops_info {
    opal_no: i32,
    attr_name: *const c_char,
    store: Option<unsafe extern "C" fn(*mut kobject, *mut kobj_attribute, *const c_char, usize) -> isize>,
}

const OPAL_SUCCESS: i32 = 0;
const OPAL_ASYNC_COMPLETION: i32 = -2;
const EINVAL: i32 = 22;
const EIO: i32 = 5;
const OPAL_SENSOR_GROUP_CLEAR: i32 = 0;

static mut OPS_INFO: [sg_ops_info; 1] = [sg_ops_info {
    opal_no: OPAL_SENSOR_GROUP_CLEAR,
    attr_name: b"clear\0".as_ptr() as *const c_char,
    store: Some(sg_store),
}];

pub unsafe extern "C" fn sensor_group_enable(handle: u32, enable: bool) -> i32 {
    let mut msg = core::mem::MaybeUninit::<opal_msg>::uninit();
    let token = opal_async_get_token_interruptible();
    if token < 0 {
        return token;
    }

    let mut ret = opal_sensor_group_enable(handle, token, enable);
    if ret == OPAL_ASYNC_COMPLETION {
        ret = opal_async_wait_response(token, msg.as_mut_ptr());
        if ret != 0 {
            ret = -EIO;
        } else {
            ret = opal_error_code(opal_get_async_rc(msg.assume_init()));
        }
    } else {
        ret = opal_error_code(ret);
    }
    opal_async_release_token(token);
    ret
}

unsafe extern "C" fn sg_store(
    _kobj: *mut kobject,
    attr: *mut kobj_attribute,
    buf: *const c_char,
    count: usize,
) -> isize {
    let sattr = (attr as *mut u8).sub(core::mem::offset_of!(sg_attr, attr)) as *mut sg_attr;
    let mut data = 0u32;
    let mut ret = kstrtoint(buf, 0, &mut data);
    if ret != 0 {
        return ret as isize;
    }
    if data != 1 {
        return -EINVAL as isize;
    }

    let token = opal_async_get_token_interruptible();
    if token < 0 {
        return token as isize;
    }
    ret = mutex_lock_interruptible(&mut sg_mutex);
    if ret != 0 {
        opal_async_release_token(token);
        return ret as isize;
    }

    let mut msg = core::mem::MaybeUninit::<opal_msg>::uninit();
    ret = opal_sensor_group_clear((*sattr).handle, token);
    if ret == OPAL_ASYNC_COMPLETION {
        ret = opal_async_wait_response(token, msg.as_mut_ptr());
        if ret == 0 {
            ret = opal_error_code(opal_get_async_rc(msg.assume_init()));
            if ret == 0 { ret = count as i32; }
        } else { ret = -EIO; }
    } else if ret == OPAL_SUCCESS {
        ret = count as i32;
    } else {
        ret = opal_error_code(ret);
    }
    mutex_unlock(&mut sg_mutex);
    opal_async_release_token(token);
    ret as isize
}

unsafe fn add_attr(handle: i32, attr: *mut sg_attr, index: usize) {
    (*attr).handle = handle as u32;
    sysfs_attr_init(&mut (*attr).attr.attr);
    (*attr).attr.attr.name = OPS_INFO[index].attr_name;
    (*attr).attr.attr.mode = 0o220;
    (*attr).attr.store = OPS_INFO[index].store;
}

unsafe fn add_attr_group(ops: *const u32, len: i32, sg: *mut sensor_group, handle: u32) -> i32 {
    let mut count = 0usize;
    for i in 0..len as usize {
        for j in 0..OPS_INFO.len() {
            if u32::from_be(*ops.add(i)) as i32 == OPS_INFO[j].opal_no {
                add_attr(handle as i32, (*sg).sgattrs.add(count), j);
                *(*sg).sg.attrs.add(count) = &mut (*(*sg).sgattrs.add(count)).attr.attr;
                count += 1;
            }
        }
    }
    sysfs_create_group(sg_kobj, &mut (*sg).sg)
}

unsafe fn get_nr_attrs(ops: *const u32, len: i32) -> usize {
    let mut nr_attrs = 0usize;
    for i in 0..len as usize {
        for j in 0..OPS_INFO.len() {
            if u32::from_be(*ops.add(i)) as i32 == OPS_INFO[j].opal_no {
                nr_attrs += 1;
            }
        }
    }
    nr_attrs
}

pub unsafe extern "C" fn opal_sensor_groups_init() {
    let sg = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"ibm,opal-sensor-group\0".as_ptr() as *const c_char);
    if sg.is_null() {
        return;
    }

    let child_count = of_get_child_count(sg);
    sgs = kzalloc_sensor_groups(child_count);
    if sgs.is_null() { of_node_put(sg); return; }

    sg_kobj = kobject_create_and_add(b"sensor_groups\0".as_ptr() as *const c_char, opal_kobj);
    if sg_kobj.is_null() { kfree(sgs as *mut _); of_node_put(sg); return; }

    let mut i = 0usize;
    let mut node = of_first_child(sg);
    while !node.is_null() {
        let mut len = 0u32;
        let ops = of_get_property(node, b"ops\0".as_ptr() as *const c_char, &mut len);
        if !ops.is_null() {
            let nr_attrs = get_nr_attrs(ops, len as i32);
            if nr_attrs != 0 {
                (*sgs.add(i)).sgattrs = kzalloc_sg_attrs(nr_attrs);
                if (*sgs.add(i)).sgattrs.is_null() { break; }
                (*sgs.add(i)).sg.attrs = kzalloc_attrs(nr_attrs + 1);
                if (*sgs.add(i)).sg.attrs.is_null() {
                    kfree((*sgs.add(i)).sgattrs as *mut _); break;
                }
                let mut sgid = 0u32;
                if of_property_read_u32(node, b"sensor-group-id\0".as_ptr() as *const c_char, &mut sgid) != 0 { break; }
                let mut chipid = 0u32;
                if of_property_read_u32(node, b"ibm,chip-id\0".as_ptr() as *const c_char, &mut chipid) == 0 {
                    sprintf((*sgs.add(i)).name.as_mut_ptr(), b"%pOFn%d\0".as_ptr() as *const c_char, node, chipid);
                } else {
                    sprintf((*sgs.add(i)).name.as_mut_ptr(), b"%pOFn\0".as_ptr() as *const c_char, node);
                }
                (*sgs.add(i)).sg.name = (*sgs.add(i)).name.as_ptr();
                if add_attr_group(ops, len as i32, sgs.add(i), sgid) != 0 { break; }
                i += 1;
            }
        }
        node = of_next_child(node, sg);
    }
    of_node_put(sg);
}

extern "C" {
    fn kzalloc_sensor_groups(count: usize) -> *mut sensor_group;
    fn kzalloc_sg_attrs(count: usize) -> *mut sg_attr;
    fn kzalloc_attrs(count: usize) -> *mut *mut attribute;
    fn of_first_child(node: *mut device_node) -> *mut device_node;
    fn of_next_child(node: *mut device_node, parent: *mut device_node) -> *mut device_node;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
