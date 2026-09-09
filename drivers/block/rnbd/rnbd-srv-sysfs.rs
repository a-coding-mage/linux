// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * RDMA Network Block Driver
 *
 * Copyright (c) 2014 - 2018 ProfitBricks GmbH. All rights reserved.
 * Copyright (c) 2018 - 2019 1&1 IONOS Cloud GmbH. All rights reserved.
 * Copyright (c) 2019 - 2020 1&1 IONOS SE. All rights reserved.
 */

// Linux kernel dependencies and "rnbd-srv.h" are supplied by other files.

extern "C" {
    static mut rnbd_devs_kobj: *mut kobject;
    static mut rnbd_dev: *mut device;
    static rnbd_access_modes: [rnbd_access_mode; _];

    fn kobject_init_and_add(kobj: *mut kobject, ktype: *const kobj_type,
                            parent: *mut kobject, fmt: *const c_char, ...) -> c_int;
    fn kobject_put(kobj: *mut kobject);
    fn kobject_del(kobj: *mut kobject);
    fn kobject_create_and_add(name: *const c_char, parent: *mut kobject) -> *mut kobject;
    fn sysfs_create_link(kobj: *mut kobject, target: *mut kobject, name: *const c_char) -> c_int;
    fn sysfs_remove_link(kobj: *mut kobject, name: *const c_char);
    fn sysfs_create_group(kobj: *mut kobject, group: *const attribute_group) -> c_int;
    fn sysfs_remove_group(kobj: *mut kobject, group: *const attribute_group);
    fn sysfs_emit(page: *mut c_char, fmt: *const c_char, ...) -> isize;
    fn sysfs_streq(s: *const c_char, t: *const c_char) -> bool;
    fn class_register(class: *const class) -> c_int;
    fn class_unregister(class: *const class);
    fn device_create(class: *const class, parent: *mut device, devt: dev_t,
                     drvdata: *mut c_void, fmt: *const c_char, ...) -> *mut device;
    fn device_destroy(class: *const class, devt: dev_t);
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn rnbd_srv_sess_dev_force_close(sess_dev: *mut rnbd_srv_sess_dev, attr: *mut kobj_attribute);
    fn rnbd_destroy_sess_dev(sess_dev: *mut rnbd_srv_sess_dev, keep_id: bool);
    fn rnbd_srv_err(sess_dev: *mut rnbd_srv_sess_dev, fmt: *const c_char, ...);
    fn rnbd_srv_info(sess_dev: *mut rnbd_srv_sess_dev, fmt: *const c_char, ...);
    fn kfree(ptr: *mut c_void);
}

use core::ffi::{c_char, c_int, c_void};
type ssize_t = isize;
type size_t = usize;
type dev_t = u64;

#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct device { pub kobj: kobject }
#[repr(C)] pub struct block_device { pub bd_disk: *mut gendisk }
#[repr(C)] pub struct gendisk { _private: [u8; 0] }
#[repr(C)] pub struct class { pub name: *const c_char }
#[repr(C)] pub struct kobj_type { pub sysfs_ops: *const c_void, pub release: Option<unsafe extern "C" fn(*mut kobject)> }
#[repr(C)] pub struct attribute { _private: [u8; 0] }
#[repr(C)] pub struct kobj_attribute { pub attr: attribute }
#[repr(C)] pub struct attribute_group { pub attrs: *mut *mut attribute }
#[repr(C)] pub struct rnbd_access_mode { pub str_: *const c_char }
#[repr(C)] pub struct rnbd_srv_dev { pub dev_kobj: kobject, pub dev_sessions_kobj: *mut kobject }
#[repr(C)] pub struct rnbd_srv_sess_dev { pub kobj: kobject, pub readonly: c_int, pub access_mode: usize, pub pathname: *const c_char, pub keep_id: bool, pub dev: *mut rnbd_srv_dev, pub sess: *mut rnbd_session }
#[repr(C)] pub struct rnbd_session { pub sessname: *const c_char }

static mut RNBD_DEV_CLASS: class = class { name: b"rnbd-server\0".as_ptr() as *const c_char };

unsafe extern "C" fn rnbd_srv_dev_release(kobj: *mut kobject) {
    let dev = (kobj as *mut u8).sub(offset_of!(rnbd_srv_dev, dev_kobj)) as *mut rnbd_srv_dev;
    kfree(dev as *mut c_void);
}

static DEV_KTYPE: kobj_type = kobj_type { sysfs_ops: core::ptr::null(), release: Some(rnbd_srv_dev_release) };

pub unsafe extern "C" fn rnbd_srv_create_dev_sysfs(dev: *mut rnbd_srv_dev, bdev: *mut block_device) -> c_int {
    let ret = kobject_init_and_add(&mut (*dev).dev_kobj, &DEV_KTYPE, rnbd_devs_kobj, b"%pg\0".as_ptr() as *const c_char, bdev);
    if ret != 0 { kobject_put(&mut (*dev).dev_kobj); return ret; }
    (*dev).dev_sessions_kobj = kobject_create_and_add(b"sessions\0".as_ptr() as *const c_char, &mut (*dev).dev_kobj);
    if (*dev).dev_sessions_kobj.is_null() { kobject_del(&mut (*dev).dev_kobj); kobject_put(&mut (*dev).dev_kobj); return -12; }
    ret = sysfs_create_link(&mut (*dev).dev_kobj, core::ptr::null_mut(), b"block_dev\0".as_ptr() as *const c_char);
    if ret != 0 { kobject_put((*dev).dev_sessions_kobj); kobject_del(&mut (*dev).dev_kobj); kobject_put(&mut (*dev).dev_kobj); }
    ret
}

pub unsafe extern "C" fn rnbd_srv_destroy_dev_sysfs(dev: *mut rnbd_srv_dev) {
    sysfs_remove_link(&mut (*dev).dev_kobj, b"block_dev\0".as_ptr() as *const c_char);
    kobject_del((*dev).dev_sessions_kobj); kobject_put((*dev).dev_sessions_kobj);
    kobject_del(&mut (*dev).dev_kobj); kobject_put(&mut (*dev).dev_kobj);
}

unsafe extern "C" fn read_only_show(kobj: *mut kobject, _attr: *mut kobj_attribute, page: *mut c_char) -> ssize_t {
    let sess_dev = kobj as *mut rnbd_srv_sess_dev;
    sysfs_emit(page, b"%d\n\0".as_ptr() as *const c_char, (*sess_dev).readonly)
}
unsafe extern "C" fn access_mode_show(kobj: *mut kobject, _attr: *mut kobj_attribute, page: *mut c_char) -> ssize_t {
    let sess_dev = kobj as *mut rnbd_srv_sess_dev;
    sysfs_emit(page, b"%s\n\0".as_ptr() as *const c_char, rnbd_access_modes[(*sess_dev).access_mode].str_)
}
unsafe extern "C" fn mapping_path_show(kobj: *mut kobject, _attr: *mut kobj_attribute, page: *mut c_char) -> ssize_t {
    sysfs_emit(page, b"%s\n\0".as_ptr() as *const c_char, (*(kobj as *mut rnbd_srv_sess_dev)).pathname)
}
unsafe extern "C" fn force_close_show(_kobj: *mut kobject, attr: *mut kobj_attribute, page: *mut c_char) -> ssize_t {
    sysfs_emit(page, b"Usage: echo 1 > %s\n\0".as_ptr() as *const c_char, (*attr).attr_name())
}
unsafe extern "C" fn force_close_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *const c_char, count: size_t) -> ssize_t {
    let sess_dev = kobj as *mut rnbd_srv_sess_dev;
    if !sysfs_streq(buf, b"1\0".as_ptr() as *const c_char) { rnbd_srv_err(sess_dev, b"%s: invalid value: '%s'\n\0".as_ptr() as *const c_char, (*attr).attr_name(), buf); return -22; }
    rnbd_srv_info(sess_dev, b"force close requested\n\0".as_ptr() as *const c_char);
    rnbd_srv_sess_dev_force_close(sess_dev, attr); count as ssize_t
}

impl kobj_attribute { unsafe fn attr_name(&self) -> *const c_char { core::ptr::null() } }
static mut RNBD_SRV_DEFAULT_DEV_SESSION_ATTR_GROUP: attribute_group = attribute_group { attrs: core::ptr::null_mut() };

pub unsafe extern "C" fn rnbd_srv_destroy_dev_session_sysfs(sess_dev: *mut rnbd_srv_sess_dev) {
    sysfs_remove_group(&mut (*sess_dev).kobj, &RNBD_SRV_DEFAULT_DEV_SESSION_ATTR_GROUP);
    kobject_del(&mut (*sess_dev).kobj); kobject_put(&mut (*sess_dev).kobj);
}
unsafe extern "C" fn rnbd_srv_sess_dev_release(kobj: *mut kobject) {
    let sess_dev = kobj as *mut rnbd_srv_sess_dev;
    rnbd_destroy_sess_dev(sess_dev, (*sess_dev).keep_id);
}
static RNBD_SRV_SESS_DEV_KTYPE: kobj_type = kobj_type { sysfs_ops: core::ptr::null(), release: Some(rnbd_srv_sess_dev_release) };

pub unsafe extern "C" fn rnbd_srv_create_dev_session_sysfs(sess_dev: *mut rnbd_srv_sess_dev) -> c_int {
    let ret = kobject_init_and_add(&mut (*sess_dev).kobj, &RNBD_SRV_SESS_DEV_KTYPE, (*(*sess_dev).dev).dev_sessions_kobj, b"%s\0".as_ptr() as *const c_char, (*(*sess_dev).sess).sessname);
    if ret != 0 { kobject_put(&mut (*sess_dev).kobj); return ret; }
    let ret = sysfs_create_group(&mut (*sess_dev).kobj, &RNBD_SRV_DEFAULT_DEV_SESSION_ATTR_GROUP);
    if ret != 0 { kobject_del(&mut (*sess_dev).kobj); kobject_put(&mut (*sess_dev).kobj); }
    ret
}

pub unsafe extern "C" fn rnbd_srv_create_sysfs_files() -> c_int {
    let mut err = class_register(&RNBD_DEV_CLASS); if err != 0 { return err; }
    rnbd_dev = device_create(&RNBD_DEV_CLASS, core::ptr::null_mut(), 0, core::ptr::null_mut(), b"ctl\0".as_ptr() as *const c_char);
    if IS_ERR(rnbd_dev as *const c_void) { err = PTR_ERR(rnbd_dev as *const c_void); class_unregister(&RNBD_DEV_CLASS); return err; }
    rnbd_devs_kobj = kobject_create_and_add(b"devices\0".as_ptr() as *const c_char, &mut (*rnbd_dev).kobj);
    if rnbd_devs_kobj.is_null() { device_destroy(&RNBD_DEV_CLASS, 0); class_unregister(&RNBD_DEV_CLASS); return -12; }
    0
}
pub unsafe extern "C" fn rnbd_srv_destroy_sysfs_files() {
    kobject_del(rnbd_devs_kobj); kobject_put(rnbd_devs_kobj);
    device_destroy(&RNBD_DEV_CLASS, 0); class_unregister(&RNBD_DEV_CLASS);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
