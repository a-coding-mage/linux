// SPDX-License-Identifier: GPL-2.0-only

/* Copyright (c) 2020-2025, The Linux Foundation. All rights reserved. */

// Dependencies supplied by the surrounding kernel/DRM and QAIC sources are
// intentionally referenced here without reimplementing them.

const NAME_LEN: usize = 14;

#[repr(C)]
pub struct DbcAttribute {
    pub dev_attr: device_attribute,
    pub dbc_id: u32,
    pub name: [libc::c_char; NAME_LEN],
}

unsafe extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut libc::c_void;
    fn to_qaic_device(dev: *mut drm_device) -> *mut qaic_device;
    fn to_accel_kdev(qddev: *mut qaic_drm_device) -> *mut device;
    fn to_drm(qddev: *mut qaic_drm_device) -> *mut drm_device;
    fn sysfs_emit(buf: *mut libc::c_char, fmt: *const libc::c_char, ...) -> isize;
    fn kobject_uevent_env(
        kobj: *mut kobject,
        action: libc::c_uint,
        envp: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn drmm_kcalloc(
        drm: *mut drm_device,
        n: usize,
        size: usize,
        flags: libc::c_uint,
    ) -> *mut libc::c_void;
    fn drmm_kfree(drm: *mut drm_device, p: *mut libc::c_void);
    fn sysfs_create_file(kobj: *mut kobject, attr: *mut attribute) -> libc::c_int;
    fn sysfs_remove_file(kobj: *mut kobject, attr: *mut attribute);
    fn sysfs_attr_init(attr: *mut attribute);
    fn scnprintf(buf: *mut libc::c_char, size: usize, fmt: *const libc::c_char, ...) -> libc::c_int;
}

// External C types and QAIC structures.
#[repr(C)] pub struct device { pub _private: [u8; 0] }
#[repr(C)] pub struct drm_device { pub _private: [u8; 0] }
#[repr(C)] pub struct kobject { pub _private: [u8; 0] }
#[repr(C)] pub struct attribute { pub name: *const libc::c_char, pub mode: libc::c_uint }
#[repr(C)] pub struct device_attribute {
    pub attr: attribute,
    pub show: Option<unsafe extern "C" fn(*mut device, *mut device_attribute, *mut libc::c_char) -> isize>,
}
#[repr(C)] pub struct drm_minor { pub dev: *mut drm_device }
#[repr(C)] pub struct qaic_device {
    pub dbc: *mut dbc,
    pub num_dbc: u32,
}
#[repr(C)] pub struct dbc { pub state: libc::c_uint }
#[repr(C)] pub struct qaic_drm_device {
    pub qdev: *mut qaic_device,
    pub sysfs_attrs: *mut DbcAttribute,
}

const DBC_STATE_MAX: libc::c_uint = 0; // Supplied by qaic.h.
const GFP_KERNEL: libc::c_uint = 0; // Supplied by the kernel headers.
const KOBJ_CHANGE: libc::c_uint = 0; // Supplied by the kernel headers.

#[no_mangle]
pub unsafe extern "C" fn dbc_state_show(
    dev: *mut device,
    a: *mut device_attribute,
    buf: *mut libc::c_char,
) -> isize {
    let dbc_attr = (a as *mut u8).sub(core::mem::offset_of!(DbcAttribute, dev_attr)) as *mut DbcAttribute;
    let minor = dev_get_drvdata(dev) as *mut drm_minor;
    let qdev = to_qaic_device((*minor).dev);
    sysfs_emit(buf, b"%d\0".as_ptr() as *const libc::c_char, (*(*qdev).dbc.add((*dbc_attr).dbc_id as usize)).state) +
        sysfs_emit(buf.add(0), b"\n\0".as_ptr() as *const libc::c_char)
}

#[no_mangle]
pub unsafe extern "C" fn set_dbc_state(qdev: *mut qaic_device, dbc_id: u32, state: libc::c_uint) {
    let kdev = to_accel_kdev(qdev as *mut qaic_drm_device);
    let mut envp: [*mut libc::c_char; 3] = [core::ptr::null_mut(); 3];
    let mut state_str = [0 as libc::c_char; 16];
    let mut id_str = [0 as libc::c_char; 12];
    envp[0] = id_str.as_mut_ptr();
    envp[1] = state_str.as_mut_ptr();

    if state >= DBC_STATE_MAX || dbc_id >= (*qdev).num_dbc || state == (*(*qdev).dbc.add(dbc_id as usize)).state { return; }
    scnprintf(id_str.as_mut_ptr(), id_str.len(), b"DBC_ID=%d\0".as_ptr() as *const libc::c_char, dbc_id);
    scnprintf(state_str.as_mut_ptr(), state_str.len(), b"DBC_STATE=%d\0".as_ptr() as *const libc::c_char, state);
    (*(*qdev).dbc.add(dbc_id as usize)).state = state;
    kobject_uevent_env(kdev as *mut kobject, KOBJ_CHANGE, envp.as_mut_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn qaic_sysfs_init(qddev: *mut qaic_drm_device) -> libc::c_int {
    let kdev = to_accel_kdev(qddev);
    let drm = to_drm(qddev);
    let num_dbc = (*(*qddev).qdev).num_dbc;
    let dbc_attrs = drmm_kcalloc(drm, num_dbc as usize, core::mem::size_of::<DbcAttribute>(), GFP_KERNEL) as *mut DbcAttribute;
    if dbc_attrs.is_null() { return -libc::ENOMEM; }

    for i in 0..num_dbc {
        let dbc_attr = dbc_attrs.add(i as usize);
        sysfs_attr_init(&mut (*dbc_attr).dev_attr.attr);
        (*dbc_attr).dbc_id = i;
        scnprintf((*dbc_attr).name.as_mut_ptr(), NAME_LEN, b"dbc%d_state\0".as_ptr() as *const libc::c_char, i);
        (*dbc_attr).dev_attr.attr.name = (*dbc_attr).name.as_ptr();
        (*dbc_attr).dev_attr.attr.mode = 0o444;
        (*dbc_attr).dev_attr.show = Some(dbc_state_show);
        let ret = sysfs_create_file(kdev as *mut kobject, &mut (*dbc_attr).dev_attr.attr);
        if ret != 0 {
            for j in 0..i {
                let old_attr = dbc_attrs.add(j as usize);
                sysfs_remove_file(kdev as *mut kobject, &mut (*old_attr).dev_attr.attr);
            }
            drmm_kfree(drm, dbc_attrs as *mut libc::c_void);
            return ret;
        }
    }
    (*qddev).sysfs_attrs = dbc_attrs;
    0
}

#[no_mangle]
pub unsafe extern "C" fn qaic_sysfs_remove(qddev: *mut qaic_drm_device) {
    let dbc_attrs = (*qddev).sysfs_attrs;
    let kdev = to_accel_kdev(qddev);
    let num_dbc = (*(*qddev).qdev).num_dbc;
    if dbc_attrs.is_null() { return; }
    (*qddev).sysfs_attrs = core::ptr::null_mut();
    for i in 0..num_dbc {
        sysfs_remove_file(kdev as *mut kobject, &mut (*dbc_attrs.add(i as usize)).dev_attr.attr);
    }
    drmm_kfree(to_drm(qddev), dbc_attrs as *mut libc::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
