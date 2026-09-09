// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2024-2026 Intel Corporation
 */

// The C source includes Linux and IVPU headers; their declarations are supplied
// by the surrounding kernel/Rust bindings.

unsafe fn npu_busy_time_us_show(dev: *mut device, _attr: *mut device_attribute,
                                buf: *mut c_char) -> ssize_t {
    let drm: *mut drm_device = dev_get_drvdata(dev);
    let vdev: *mut ivpu_device = to_ivpu_device(drm);
    let mut total: ktime_t;
    let mut now: ktime_t = 0;

    mutex_lock(&mut (*vdev).submitted_jobs_lock);
    total = (*vdev).busy_time;
    if !xa_empty(&(*vdev).submitted_jobs_xa) {
        now = ktime_sub(ktime_get(), (*vdev).busy_start_ts);
    }
    mutex_unlock(&mut (*vdev).submitted_jobs_lock);

    sysfs_emit(buf, "%lld\n", ktime_to_us(ktime_add(total, now)))
}

unsafe fn npu_memory_utilization_show(dev: *mut device, _attr: *mut device_attribute,
                                      buf: *mut c_char) -> ssize_t {
    let drm: *mut drm_device = dev_get_drvdata(dev);
    let vdev: *mut ivpu_device = to_ivpu_device(drm);
    let mut bo: *mut ivpu_bo;
    let mut total_npu_memory: u64 = 0;

    mutex_lock(&mut (*vdev).bo_list_lock);
    list_for_each_entry!(bo, &(*vdev).bo_list, bo_list_node, {
        if ivpu_bo_is_resident(bo) {
            total_npu_memory += ivpu_bo_size(bo);
        }
    });
    mutex_unlock(&mut (*vdev).bo_list_lock);

    sysfs_emit(buf, "%lld\n", total_npu_memory)
}

unsafe fn sched_mode_show(dev: *mut device, _attr: *mut device_attribute,
                          buf: *mut c_char) -> ssize_t {
    let drm: *mut drm_device = dev_get_drvdata(dev);
    let vdev: *mut ivpu_device = to_ivpu_device(drm);
    sysfs_emit(buf, "%s\n", if (*(*vdev).fw).sched_mode { "HW" } else { "OS" })
}

unsafe fn hw_min_freq_show(dev: *mut device, _attr: *mut device_attribute,
                           buf: *mut c_char) -> ssize_t {
    let vdev = to_ivpu_device(dev_get_drvdata(dev));
    let freq_mhz: u32 = ivpu_hw_btrs_pll_ratio_to_mhz(vdev, (*(*vdev).hw).pll.min_ratio);
    sysfs_emit(buf, "%u\n", freq_mhz)
}

unsafe fn hw_efficient_freq_show(dev: *mut device, _attr: *mut device_attribute,
                                buf: *mut c_char) -> ssize_t {
    let vdev = to_ivpu_device(dev_get_drvdata(dev));
    let freq_mhz: u32 = ivpu_hw_btrs_pll_ratio_to_mhz(vdev, (*(*vdev).hw).pll.pn_ratio);
    sysfs_emit(buf, "%u\n", freq_mhz)
}

unsafe fn hw_max_freq_show(dev: *mut device, _attr: *mut device_attribute,
                           buf: *mut c_char) -> ssize_t {
    let vdev = to_ivpu_device(dev_get_drvdata(dev));
    let freq_mhz: u32 = ivpu_hw_btrs_pll_ratio_to_mhz(vdev, (*(*vdev).hw).pll.max_ratio);
    sysfs_emit(buf, "%u\n", freq_mhz)
}

unsafe fn set_min_freq_show(dev: *mut device, _attr: *mut device_attribute,
                            buf: *mut c_char) -> ssize_t {
    let vdev = to_ivpu_device(dev_get_drvdata(dev));
    let freq_mhz: u32 = ivpu_hw_btrs_pll_ratio_to_mhz(vdev, (*(*vdev).hw).pll.cfg_min_ratio);
    sysfs_emit(buf, "%u\n", freq_mhz)
}

unsafe fn set_min_freq_store(dev: *mut device, _attr: *mut device_attribute,
                             buf: *const c_char, count: usize) -> ssize_t {
    let vdev = to_ivpu_device(dev_get_drvdata(dev));
    let mut freq_mhz: u32 = 0;
    let mut ret = kstrtou32(buf, 10, &mut freq_mhz);
    if ret != 0 { return ret as ssize_t; }
    ret = ivpu_hw_btrs_cfg_min_freq_set(vdev, freq_mhz);
    if ret != 0 { return ret as ssize_t; }
    count as ssize_t
}

unsafe fn set_max_freq_show(dev: *mut device, _attr: *mut device_attribute,
                            buf: *mut c_char) -> ssize_t {
    let vdev = to_ivpu_device(dev_get_drvdata(dev));
    let freq_mhz: u32 = ivpu_hw_btrs_pll_ratio_to_mhz(vdev, (*(*vdev).hw).pll.cfg_max_ratio);
    sysfs_emit(buf, "%u\n", freq_mhz)
}

unsafe fn set_max_freq_store(dev: *mut device, _attr: *mut device_attribute,
                             buf: *const c_char, count: usize) -> ssize_t {
    let vdev = to_ivpu_device(dev_get_drvdata(dev));
    let mut freq_mhz: u32 = 0;
    let mut ret = kstrtou32(buf, 10, &mut freq_mhz);
    if ret != 0 { return ret as ssize_t; }
    // Convert MHz to Hz and set max frequency
    ret = ivpu_hw_btrs_cfg_max_freq_set(vdev, freq_mhz);
    if ret != 0 { return ret as ssize_t; }
    count as ssize_t
}

unsafe fn current_freq_show(dev: *mut device, _attr: *mut device_attribute,
                            buf: *mut c_char) -> ssize_t {
    let vdev = to_ivpu_device(dev_get_drvdata(dev));
    let mut freq_mhz: u32 = 0;
    // Read frequency only if device is active, otherwise frequency is 0
    if pm_runtime_get_if_active((*vdev).drm.dev) > 0 {
        freq_mhz = ivpu_hw_btrs_current_freq_get(vdev);
        pm_runtime_put_autosuspend((*vdev).drm.dev);
    }
    sysfs_emit(buf, "%u\n", freq_mhz)
}

// DEVICE_ATTR_RO/DEVICE_ATTR_RW and __ATTR declarations from the C source.
static mut dev_attr_npu_busy_time_us: device_attribute = DEVICE_ATTR_RO!(npu_busy_time_us);
static mut dev_attr_npu_memory_utilization: device_attribute = DEVICE_ATTR_RO!(npu_memory_utilization);
static mut dev_attr_sched_mode: device_attribute = DEVICE_ATTR_RO!(sched_mode);
static mut dev_attr_hw_min_freq: device_attribute = DEVICE_ATTR_RO!(hw_min_freq);
static mut dev_attr_hw_efficient_freq: device_attribute = DEVICE_ATTR_RO!(hw_efficient_freq);
static mut dev_attr_hw_max_freq: device_attribute = DEVICE_ATTR_RO!(hw_max_freq);
static mut dev_attr_set_min_freq: device_attribute = DEVICE_ATTR_RW!(set_min_freq);
static mut dev_attr_set_max_freq: device_attribute = DEVICE_ATTR_RW!(set_max_freq);
static mut dev_attr_current_freq: device_attribute = DEVICE_ATTR_RO!(current_freq);
static mut dev_attr_npu_max_frequency_mhz: device_attribute =
    __ATTR!(npu_max_frequency_mhz, 0o444, hw_max_freq_show, None);
static mut dev_attr_npu_current_frequency_mhz: device_attribute =
    __ATTR!(npu_current_frequency_mhz, 0o444, current_freq_show, None);

static mut ivpu_freq_attrs: [*mut attribute; 5] = [
    &mut dev_attr_hw_min_freq.attr, &mut dev_attr_hw_efficient_freq.attr,
    &mut dev_attr_hw_max_freq.attr, &mut dev_attr_current_freq.attr, core::ptr::null_mut(),
];
static mut ivpu_freq_attr_group: attribute_group = attribute_group {
    name: "freq", attrs: ivpu_freq_attrs.as_mut_ptr(),
};
static mut ivpu_dev_attrs: [*mut attribute; 6] = [
    &mut dev_attr_npu_busy_time_us.attr, &mut dev_attr_npu_memory_utilization.attr,
    &mut dev_attr_sched_mode.attr, &mut dev_attr_npu_max_frequency_mhz.attr,
    &mut dev_attr_npu_current_frequency_mhz.attr, core::ptr::null_mut(),
];
static mut ivpu_dev_attr_group: attribute_group = attribute_group {
    name: core::ptr::null(), attrs: ivpu_dev_attrs.as_mut_ptr(),
};

pub unsafe fn ivpu_sysfs_init(vdev: *mut ivpu_device) {
    let mut ret = devm_device_add_group((*vdev).drm.dev, &ivpu_dev_attr_group);
    if ret != 0 {
        ivpu_warn(vdev, "Failed to add sysfs group to device, ret %d", ret);
        return;
    }
    ret = devm_device_add_group((*vdev).drm.dev, &ivpu_freq_attr_group);
    if ret != 0 {
        ivpu_warn(vdev, "Failed to add sysfs freq group, ret %d", ret);
        return;
    }
    if ivpu_hw_ip_gen(vdev) >= IVPU_HW_IP_50XX {
        ret = sysfs_add_file_to_group(&mut (*(*vdev).drm.dev).kobj,
                                      &dev_attr_set_min_freq.attr, "freq");
        if ret != 0 {
            ivpu_warn(vdev, "Failed to add sysfs set_min_freq to device, ret %d", ret);
            return;
        }
        ret = sysfs_add_file_to_group(&mut (*(*vdev).drm.dev).kobj,
                                      &dev_attr_set_max_freq.attr, "freq");
        if ret != 0 {
            ivpu_warn(vdev, "Failed to add sysfs set_max_freq to device, ret %d", ret);
            return;
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
