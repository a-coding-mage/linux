// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2026 Intel Corporation */

// Translated from the Linux kernel implementation.  The declarations supplied
// by the included headers are intentionally left as external dependencies.

unsafe fn enforced_min_show(
    dev: *mut device,
    attr: *mut device_attribute,
    buf: *mut core::ffi::c_char,
) -> ssize_t {
    let accel_dev: *mut adf_accel_dev;
    let mut svn: u8 = 0;

    accel_dev = adf_devmgr_pci_to_accel_dev(to_pci_dev(dev));
    if accel_dev.is_null() {
        return -EINVAL;
    }

    let err = adf_anti_rb_query(accel_dev, ARB_ENFORCED_MIN_SVN, &mut svn);
    if err != 0 {
        return err as ssize_t;
    }

    sysfs_emit(buf, "%u\n", svn as core::ffi::c_uint)
}

static DEVICE_ATTR_RO!(enforced_min, enforced_min_show);

unsafe fn active_show(
    dev: *mut device,
    attr: *mut device_attribute,
    buf: *mut core::ffi::c_char,
) -> ssize_t {
    let accel_dev: *mut adf_accel_dev;
    let mut svn: u8 = 0;

    accel_dev = adf_devmgr_pci_to_accel_dev(to_pci_dev(dev));
    if accel_dev.is_null() {
        return -EINVAL;
    }

    let err = adf_anti_rb_query(accel_dev, ARB_ACTIVE_SVN, &mut svn);
    if err != 0 {
        return err as ssize_t;
    }

    sysfs_emit(buf, "%u\n", svn as core::ffi::c_uint)
}

static DEVICE_ATTR_RO!(active, active_show);

unsafe fn permanent_min_show(
    dev: *mut device,
    attr: *mut device_attribute,
    buf: *mut core::ffi::c_char,
) -> ssize_t {
    let accel_dev: *mut adf_accel_dev;
    let mut svn: u8 = 0;

    accel_dev = adf_devmgr_pci_to_accel_dev(to_pci_dev(dev));
    if accel_dev.is_null() {
        return -EINVAL;
    }

    let err = adf_anti_rb_query(accel_dev, ARB_PERMANENT_MIN_SVN, &mut svn);
    if err != 0 {
        return err as ssize_t;
    }

    sysfs_emit(buf, "%u\n", svn as core::ffi::c_uint)
}

static DEVICE_ATTR_RO!(permanent_min, permanent_min_show);

unsafe fn commit_store(
    dev: *mut device,
    attr: *mut device_attribute,
    buf: *const core::ffi::c_char,
    count: usize,
) -> ssize_t {
    let accel_dev: *mut adf_accel_dev;
    let mut val = false;

    accel_dev = adf_devmgr_pci_to_accel_dev(to_pci_dev(dev));
    if accel_dev.is_null() {
        return -EINVAL;
    }

    let err = kstrtobool(buf, &mut val);
    if err != 0 {
        return err as ssize_t;
    }

    if !val {
        return -EINVAL;
    }

    let err = adf_anti_rb_commit(accel_dev);
    if err != 0 {
        return err as ssize_t;
    }

    count as ssize_t
}

static DEVICE_ATTR_WO!(commit, commit_store);

static mut qat_svn_attrs: [*mut attribute; 5] = [
    unsafe { &mut dev_attr_commit.attr },
    unsafe { &mut dev_attr_active.attr },
    unsafe { &mut dev_attr_enforced_min.attr },
    unsafe { &mut dev_attr_permanent_min.attr },
    core::ptr::null_mut(),
];

static qat_svn_group: attribute_group = attribute_group {
    attrs: unsafe { qat_svn_attrs.as_mut_ptr() },
    name: "qat_svn\0".as_ptr() as *const core::ffi::c_char,
};

pub unsafe fn adf_sysfs_start_arb(accel_dev: *mut adf_accel_dev) {
    let anti_rb: *mut adf_anti_rb_hw_data = GET_ANTI_RB_DATA!(accel_dev);

    if !(*anti_rb).anti_rb_enabled
        || !((*anti_rb).anti_rb_enabled.unwrap())(accel_dev)
    {
        return;
    }

    if device_add_group(&mut GET_DEV!(accel_dev), &qat_svn_group) != 0 {
        dev_warn(
            &mut GET_DEV!(accel_dev),
            "Failed to create qat_svn attribute group\n",
        );
        return;
    }

    (*anti_rb).sysfs_added = true;
}

pub unsafe fn adf_sysfs_stop_arb(accel_dev: *mut adf_accel_dev) {
    let anti_rb: *mut adf_anti_rb_hw_data = GET_ANTI_RB_DATA!(accel_dev);

    if !(*anti_rb).sysfs_added {
        return;
    }

    device_remove_group(&mut GET_DEV!(accel_dev), &qat_svn_group);

    (*anti_rb).sysfs_added = false;
    (*anti_rb).svncheck_retry = 0;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
