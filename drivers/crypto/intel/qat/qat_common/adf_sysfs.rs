// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2022 Intel Corporation */

// Linux headers and project headers provide the types, constants, macros, and
// external functions referenced below.

const UNSET_RING_NUM: i32 = -1;

static STATE_OPERATIONS: [&'static [u8]; 2] = [b"down\0", b"up\0"];

unsafe fn state_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut u8) -> isize {
    let accel_dev = adf_devmgr_pci_to_accel_dev(to_pci_dev(dev));
    if accel_dev.is_null() { return -EINVAL as isize; }
    sysfs_emit(buf, b"%s\n\0".as_ptr(), str_up_down(adf_dev_started(accel_dev)))
}

unsafe fn state_store(dev: *mut device, _attr: *mut device_attribute, buf: *const u8, count: usize) -> isize {
    let accel_dev = adf_devmgr_pci_to_accel_dev(to_pci_dev(dev));
    if accel_dev.is_null() { return -EINVAL as isize; }
    let accel_id = (*accel_dev).accel_id;
    if adf_devmgr_in_reset(accel_dev) || adf_dev_in_use(accel_dev) {
        dev_info(dev, b"Device qat_dev%d is busy\n\0".as_ptr(), accel_id);
        return -EBUSY as isize;
    }
    let mut ret = sysfs_match_string(STATE_OPERATIONS.as_ptr(), buf);
    if ret < 0 { return ret as isize; }
    match ret {
        DEV_DOWN => {
            dev_info(dev, b"Stopping device qat_dev%d\n\0".as_ptr(), accel_id);
            if !adf_dev_started(accel_dev) {
                dev_info(GET_DEV(accel_dev), b"Device qat_dev%d already down\n\0".as_ptr(), accel_id);
            } else {
                ret = adf_dev_down(accel_dev);
                if ret != 0 { return ret as isize; }
            }
        }
        DEV_UP => {
            dev_info(dev, b"Starting device qat_dev%d\n\0".as_ptr(), accel_id);
            ret = adf_dev_up(accel_dev, true);
            if ret == -EALREADY { } else if ret != 0 {
                dev_err(dev, b"Failed to start device qat_dev%d\n\0".as_ptr(), accel_id);
                adf_dev_down(accel_dev);
                return ret as isize;
            }
        }
        _ => return -EINVAL as isize,
    }
    count as isize
}

unsafe fn cfg_services_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut u8) -> isize {
    let mut services = [0u8; ADF_CFG_MAX_VAL_LEN_IN_BYTES];
    let accel_dev = adf_devmgr_pci_to_accel_dev(to_pci_dev(dev));
    if accel_dev.is_null() { return -EINVAL as isize; }
    let ret = adf_cfg_get_param_value(accel_dev, ADF_GENERAL_SEC, ADF_SERVICES_ENABLED, services.as_mut_ptr());
    if ret != 0 { return ret as isize; }
    sysfs_emit(buf, b"%s\n\0".as_ptr(), services.as_ptr())
}

unsafe fn adf_sysfs_update_dev_config(accel_dev: *mut adf_accel_dev, services: *const u8) -> i32 {
    adf_cfg_add_key_value_param(accel_dev, ADF_GENERAL_SEC, ADF_SERVICES_ENABLED, services, ADF_STR)
}

unsafe fn cfg_services_store(dev: *mut device, _attr: *mut device_attribute, buf: *const u8, count: usize) -> isize {
    let mut services = [0u8; ADF_CFG_MAX_VAL_LEN_IN_BYTES];
    let accel_dev = adf_devmgr_pci_to_accel_dev(to_pci_dev(dev));
    if accel_dev.is_null() { return -EINVAL as isize; }
    let mut ret = adf_parse_service_string(accel_dev, buf, services.as_mut_ptr(), ADF_CFG_MAX_VAL_LEN_IN_BYTES);
    if ret != 0 { return ret as isize; }
    if adf_dev_started(accel_dev) {
        dev_info(dev, b"Device qat_dev%d must be down to reconfigure the service.\n\0".as_ptr(), (*accel_dev).accel_id);
        return -EINVAL as isize;
    }
    ret = adf_sysfs_update_dev_config(accel_dev, services.as_ptr());
    if ret < 0 { return ret as isize; }
    let hw_data = GET_HW_DATA(accel_dev);
    (*hw_data).accel_capabilities_mask = ((*hw_data).get_accel_cap)(accel_dev);
    if (*hw_data).accel_capabilities_mask == 0 { return -EINVAL as isize; }
    count as isize
}

unsafe fn pm_idle_enabled_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut u8) -> isize {
    let mut value = [0u8; ADF_CFG_MAX_VAL_LEN_IN_BYTES];
    let accel_dev = adf_devmgr_pci_to_accel_dev(to_pci_dev(dev));
    if accel_dev.is_null() { return -EINVAL as isize; }
    let ret = adf_cfg_get_param_value(accel_dev, ADF_GENERAL_SEC, ADF_PM_IDLE_SUPPORT, value.as_mut_ptr());
    if ret != 0 { return sysfs_emit(buf, b"1\n\0".as_ptr()); }
    sysfs_emit(buf, b"%s\n\0".as_ptr(), value.as_ptr())
}

unsafe fn pm_idle_enabled_store(dev: *mut device, _attr: *mut device_attribute, buf: *const u8, count: usize) -> isize {
    let mut enabled = false;
    let ret = kstrtobool(buf, &mut enabled);
    if ret != 0 { return ret as isize; }
    let value: c_ulong = enabled as c_ulong;
    let accel_dev = adf_devmgr_pci_to_accel_dev(to_pci_dev(dev));
    if accel_dev.is_null() { return -EINVAL as isize; }
    if adf_dev_started(accel_dev) { return -EINVAL as isize; }
    let ret = adf_cfg_add_key_value_param(accel_dev, ADF_GENERAL_SEC, ADF_PM_IDLE_SUPPORT, &value as *const _, ADF_DEC);
    if ret != 0 { return ret as isize; }
    count as isize
}

unsafe fn auto_reset_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut u8) -> isize {
    let accel_dev = adf_devmgr_pci_to_accel_dev(to_pci_dev(dev));
    if accel_dev.is_null() { return -EINVAL as isize; }
    sysfs_emit(buf, b"%s\n\0".as_ptr(), str_on_off((*accel_dev).autoreset_on_error))
}

unsafe fn auto_reset_store(dev: *mut device, _attr: *mut device_attribute, buf: *const u8, count: usize) -> isize {
    let mut enabled = false;
    let ret = kstrtobool(buf, &mut enabled);
    if ret != 0 { return ret as isize; }
    let accel_dev = adf_devmgr_pci_to_accel_dev(to_pci_dev(dev));
    if accel_dev.is_null() { return -EINVAL as isize; }
    (*accel_dev).autoreset_on_error = enabled;
    count as isize
}

unsafe fn rp2srv_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut u8) -> isize {
    let accel_dev = adf_devmgr_pci_to_accel_dev(to_pci_dev(dev));
    if accel_dev.is_null() { return -EINVAL as isize; }
    let hw_data = GET_HW_DATA(accel_dev);
    if (*accel_dev).sysfs.ring_num == UNSET_RING_NUM { return -EINVAL as isize; }
    down_read(&(*accel_dev).sysfs.lock);
    let svc = GET_SRV_TYPE(accel_dev, (*accel_dev).sysfs.ring_num as usize % (*hw_data).num_banks_per_vf);
    up_read(&(*accel_dev).sysfs.lock);
    match svc { COMP => sysfs_emit(buf, b"%s\n\0".as_ptr(), ADF_CFG_DC), SYM => sysfs_emit(buf, b"%s\n\0".as_ptr(), ADF_CFG_SYM), ASYM => sysfs_emit(buf, b"%s\n\0".as_ptr(), ADF_CFG_ASYM), DECOMP => sysfs_emit(buf, b"%s\n\0".as_ptr(), ADF_CFG_DECOMP), _ => -EINVAL as isize }
}

unsafe fn rp2srv_store(dev: *mut device, _attr: *mut device_attribute, buf: *const u8, count: usize) -> isize {
    let accel_dev = adf_devmgr_pci_to_accel_dev(to_pci_dev(dev));
    if accel_dev.is_null() { return -EINVAL as isize; }
    let mut ring = 0u32;
    let ret = kstrtouint(buf, 10, &mut ring);
    if ret != 0 { return ret as isize; }
    let num_rings = GET_MAX_BANKS(accel_dev);
    if ring >= num_rings { return -EINVAL as isize; }
    down_write(&mut (*accel_dev).sysfs.lock);
    (*accel_dev).sysfs.ring_num = ring as i32;
    up_write(&mut (*accel_dev).sysfs.lock);
    count as isize
}

unsafe fn num_rps_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut u8) -> isize {
    let accel_dev = adf_devmgr_pci_to_accel_dev(to_pci_dev(dev));
    if accel_dev.is_null() { return -EINVAL as isize; }
    sysfs_emit(buf, b"%u\n\0".as_ptr(), GET_MAX_BANKS(accel_dev))
}

unsafe fn adf_sysfs_init(accel_dev: *mut adf_accel_dev) -> i32 {
    let ret = devm_device_add_group(GET_DEV(accel_dev), &qat_group);
    if ret != 0 { dev_err(GET_DEV(accel_dev), b"Failed to create qat attribute group: %d\n\0".as_ptr(), ret); }
    (*accel_dev).sysfs.ring_num = UNSET_RING_NUM;
    ret
}

// DEVICE_ATTR_RW/RO, attribute arrays, and EXPORT_SYMBOL_GPL are supplied by
// the Linux driver integration and retain their original names here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
