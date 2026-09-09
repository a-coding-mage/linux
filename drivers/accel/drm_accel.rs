// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright 2022 HabanaLabs, Ltd.
 * All Rights Reserved.
 *
 */

// Linux and DRM declarations used by this implementation are supplied by the
// surrounding kernel bindings.

use crate::*;

pub static mut accel_minors_xa: XArray = unsafe { core::mem::zeroed() };

static accel_sysfs_device_minor: DeviceType = DeviceType { name: "accel_minor" };

unsafe extern "C" fn accel_devnode(dev: *const Device, mode: *mut UmodeT) -> *mut c_char {
    kasprintf(GFP_KERNEL, c"accel/%s".as_ptr(), dev_name(dev))
}

static accel_class: Class = Class {
    name: "accel",
    devnode: Some(accel_devnode),
};

unsafe fn accel_sysfs_init() -> c_int {
    class_register(&accel_class)
}

unsafe fn accel_sysfs_destroy() {
    class_unregister(&accel_class);
}

unsafe extern "C" fn accel_name_info(m: *mut SeqFile, _data: *mut c_void) -> c_int {
    let node = (*m).private as *mut DrmInfoNode;
    let minor = (*node).minor;
    let dev = (*minor).dev;
    let mut master: *mut DrmMaster;

    mutex_lock(&mut (*dev).master_mutex);
    master = (*dev).master;
    seq_printf(m, c"%s".as_ptr(), (*(*dev).driver).name);
    if !(*dev).dev.is_null() {
        seq_printf(m, c" dev=%s".as_ptr(), dev_name((*dev).dev));
    }
    if !master.is_null() && !(*master).unique.is_null() {
        seq_printf(m, c" master=%s".as_ptr(), (*master).unique);
    }
    if !(*dev).unique.is_null() {
        seq_printf(m, c" unique=%s".as_ptr(), (*dev).unique);
    }
    seq_puts(m, c"\n".as_ptr());
    mutex_unlock(&mut (*dev).master_mutex);

    0
}

static accel_debugfs_list: [DrmInfoList; 1] = [DrmInfoList {
    name: c"name".as_ptr(),
    show: Some(accel_name_info),
    driver_features: 0,
}];
const ACCEL_DEBUGFS_ENTRIES: usize = accel_debugfs_list.len();

/// Register debugfs for device.
///
/// Creates common files for accelerators.
#[no_mangle]
pub unsafe extern "C" fn accel_debugfs_register(dev: *mut DrmDevice) {
    let minor = (*dev).accel;

    (*minor).debugfs_root = (*dev).debugfs_root;
    drm_debugfs_create_files(
        accel_debugfs_list.as_ptr(),
        ACCEL_DEBUGFS_ENTRIES,
        (*dev).debugfs_root,
        minor,
    );
}

/// Set some device parameters for accel device.
#[no_mangle]
pub unsafe extern "C" fn accel_set_device_instance_params(kdev: *mut Device, index: c_int) {
    (*kdev).devt = mkdev(ACCEL_MAJOR, index as _);
    (*kdev).class = &accel_class;
    (*kdev).type_ = &accel_sysfs_device_minor;
}

/// Open method for ACCEL files.
#[no_mangle]
pub unsafe extern "C" fn accel_open(inode: *mut Inode, filp: *mut File) -> c_int {
    let minor = drm_minor_acquire(&mut accel_minors_xa, iminor(inode));
    if is_err(minor as *const c_void) {
        return ptr_err(minor as *const c_void);
    }

    let dev = (*minor).dev;
    atomic_fetch_inc(&mut (*dev).open_count);
    // Share address_space across all char-devs of a single device.
    (*filp).f_mapping = (*(*dev).anon_inode).i_mapping;

    let retcode = drm_open_helper(filp, minor);
    if retcode != 0 {
        atomic_dec(&mut (*dev).open_count);
        drm_minor_release(minor);
        return retcode;
    }
    0
}

unsafe extern "C" fn accel_stub_open(inode: *mut Inode, filp: *mut File) -> c_int {
    let minor = drm_minor_acquire(&mut accel_minors_xa, iminor(inode));
    if is_err(minor as *const c_void) {
        return ptr_err(minor as *const c_void);
    }

    let new_fops = fops_get((*(*minor).dev).driver.fops);
    if new_fops.is_null() {
        drm_minor_release(minor);
        return -ENODEV;
    }

    replace_fops(filp, new_fops);
    let err = if let Some(open) = (*(*filp).f_op).open {
        open(inode, filp)
    } else {
        0
    };
    drm_minor_release(minor);
    err
}

static accel_stub_fops: FileOperations = FileOperations {
    owner: THIS_MODULE,
    open: Some(accel_stub_open),
    llseek: Some(noop_llseek),
};

#[no_mangle]
pub unsafe extern "C" fn accel_core_exit() {
    unregister_chrdev(ACCEL_MAJOR, c"accel".as_ptr());
    accel_sysfs_destroy();
    warn_on(!xa_empty(&accel_minors_xa));
}

#[no_mangle]
pub unsafe extern "C" fn accel_core_init() -> c_int {
    let mut ret = accel_sysfs_init();
    if ret < 0 {
        drm_error(c"Cannot create ACCEL class: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = register_chrdev(ACCEL_MAJOR, c"accel".as_ptr(), &accel_stub_fops);
    if ret < 0 {
        drm_error(c"Cannot register ACCEL major: %d\n".as_ptr(), ret);
    }
    // Cleanup due to errors is performed by drm_core_exit(), which calls
    // accel_core_exit().
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
