// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2023 Intel Corporation */

// Linux kernel dependencies and local headers are supplied by the surrounding translation unit.

use core::ffi::c_char;

// #define dev_fmt(fmt) "RateLimiting: " fmt
// #define GET_RL_STRUCT(accel_dev) ((accel_dev)->rate_limiting->user_input)

#[repr(C)]
#[derive(Copy, Clone)]
enum rl_ops {
    ADD,
    UPDATE,
    RM,
    RM_ALL,
    GET,
}

#[repr(C)]
#[derive(Copy, Clone)]
enum rl_params {
    RP_MASK,
    ID,
    CIR,
    PIR,
    SRV,
    CAP_REM_SRV,
}

extern "C" {
    static rl_services: [*const c_char; SVC_BASE_COUNT as usize];
    static rl_operations: [*const c_char; GET as usize + 1];
}

static RL_SERVICES: [Option<&'static [u8]>; 4] = [
    Some(b"asym\0"),
    Some(b"sym\0"),
    Some(b"dc\0"),
    Some(b"decomp\0"),
];

static RL_OPERATIONS: [Option<&'static [u8]>; 5] = [
    Some(b"add\0"),
    Some(b"update\0"),
    Some(b"rm\0"),
    Some(b"rm_all\0"),
    Some(b"get\0"),
];

unsafe fn set_param_u(dev: *mut device, param: rl_params, set: u64) -> i32 {
    let accel_dev = adf_devmgr_pci_to_accel_dev(to_pci_dev(dev));
    if accel_dev.is_null() { return -EINVAL; }
    let data = &mut (*(*accel_dev).rate_limiting).user_input;
    down_write(&mut data.lock);
    match param {
        rl_params::RP_MASK => data.input.rp_mask = set,
        rl_params::CIR => data.input.cir = set,
        rl_params::PIR => data.input.pir = set,
        rl_params::SRV => data.input.srv = set,
        rl_params::CAP_REM_SRV => data.cap_rem_srv = set,
        _ => { up_write(&mut data.lock); return -EINVAL; }
    }
    up_write(&mut data.lock);
    0
}

unsafe fn set_param_s(dev: *mut device, param: rl_params, set: i32) -> i32 {
    let accel_dev = adf_devmgr_pci_to_accel_dev(to_pci_dev(dev));
    if accel_dev.is_null() || !matches!(param, rl_params::ID) { return -EINVAL; }
    let data = &mut (*(*accel_dev).rate_limiting).user_input;
    down_write(&mut data.lock);
    data.input.sla_id = set;
    up_write(&mut data.lock);
    0
}

unsafe fn get_param_u(dev: *mut device, param: rl_params, get: *mut u64) -> i32 {
    let accel_dev = adf_devmgr_pci_to_accel_dev(to_pci_dev(dev));
    if accel_dev.is_null() { return -EINVAL; }
    let data = &mut (*(*accel_dev).rate_limiting).user_input;
    down_read(&mut data.lock);
    let ret = match param {
        rl_params::RP_MASK => { *get = data.input.rp_mask; 0 },
        rl_params::CIR => { *get = data.input.cir; 0 },
        rl_params::PIR => { *get = data.input.pir; 0 },
        rl_params::SRV => { *get = data.input.srv; 0 },
        _ => -EINVAL,
    };
    up_read(&mut data.lock);
    ret
}

unsafe fn get_param_s(dev: *mut device, param: rl_params) -> i32 {
    let accel_dev = adf_devmgr_pci_to_accel_dev(to_pci_dev(dev));
    if accel_dev.is_null() { return -EINVAL; }
    let data = &mut (*(*accel_dev).rate_limiting).user_input;
    down_read(&mut data.lock);
    let ret = if matches!(param, rl_params::ID) { data.input.sla_id } else { 0 };
    up_read(&mut data.lock);
    ret
}

unsafe fn rp_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> isize {
    let mut get = 0u64;
    let ret = get_param_u(dev, rl_params::RP_MASK, &mut get);
    if ret != 0 { return ret as isize; }
    sysfs_emit(buf, b"%#llx\0".as_ptr() as *const c_char, get)
}

unsafe fn rp_store(dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: usize) -> isize {
    let mut val = 0u64;
    let err = kstrtou64(buf, 16, &mut val);
    if err != 0 { return err as isize; }
    let err = set_param_u(dev, rl_params::RP_MASK, val);
    if err != 0 { return err as isize; }
    count as isize
}

// DEVICE_ATTR_RW(rp);

unsafe fn id_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> isize {
    sysfs_emit(buf, b"%d\0".as_ptr() as *const c_char, get_param_s(dev, rl_params::ID))
}

unsafe fn id_store(dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: usize) -> isize {
    let mut val = 0i32;
    let err = kstrtoint(buf, 10, &mut val);
    if err != 0 { return err as isize; }
    let err = set_param_s(dev, rl_params::ID, val);
    if err != 0 { return err as isize; }
    count as isize
}

// DEVICE_ATTR_RW(id);

unsafe fn cir_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> isize {
    let mut get = 0u64; let ret = get_param_u(dev, rl_params::CIR, &mut get);
    if ret != 0 { return ret as isize; } sysfs_emit(buf, b"%llu\0".as_ptr() as *const c_char, get)
}
unsafe fn cir_store(dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: usize) -> isize {
    let mut val = 0u32; let err = kstrtouint(buf, 10, &mut val);
    if err != 0 { return err as isize; } let err = set_param_u(dev, rl_params::CIR, val as u64);
    if err != 0 { return err as isize; } count as isize
}
// DEVICE_ATTR_RW(cir);

unsafe fn pir_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> isize {
    let mut get = 0u64; let ret = get_param_u(dev, rl_params::PIR, &mut get);
    if ret != 0 { return ret as isize; } sysfs_emit(buf, b"%llu\0".as_ptr() as *const c_char, get)
}
unsafe fn pir_store(dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: usize) -> isize {
    let mut val = 0u32; let err = kstrtouint(buf, 10, &mut val);
    if err != 0 { return err as isize; } let err = set_param_u(dev, rl_params::PIR, val as u64);
    if err != 0 { return err as isize; } count as isize
}
// DEVICE_ATTR_RW(pir);

unsafe fn srv_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> isize {
    let mut get = 0u64; let ret = get_param_u(dev, rl_params::SRV, &mut get);
    if ret != 0 { return ret as isize; } if get == SVC_BASE_COUNT as u64 { return -EINVAL as isize; }
    sysfs_emit(buf, b"%s\n\0".as_ptr() as *const c_char, rl_services[get as usize])
}
unsafe fn srv_store(dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: usize) -> isize {
    let accel_dev = adf_devmgr_pci_to_accel_dev(to_pci_dev(dev)); if accel_dev.is_null() { return -EINVAL as isize; }
    let ret = sysfs_match_string(rl_services.as_ptr(), buf); if ret < 0 { return ret as isize; }
    if !adf_is_service_enabled(accel_dev, ret as u32) { return -EINVAL as isize; }
    let err = set_param_u(dev, rl_params::SRV, ret as u64); if err != 0 { return err as isize; } count as isize
}
// DEVICE_ATTR_RW(srv);

unsafe fn cap_rem_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> isize {
    let accel_dev = adf_devmgr_pci_to_accel_dev(to_pci_dev(dev)); if accel_dev.is_null() { return -EINVAL as isize; }
    let data = &mut (*(*accel_dev).rate_limiting).user_input; down_read(&mut data.lock);
    let rem_cap = adf_rl_get_capability_remaining(accel_dev, data.cap_rem_srv, RL_SLA_EMPTY_ID); up_read(&mut data.lock);
    if rem_cap < 0 { return rem_cap as isize; } sysfs_emit(buf, b"%u\n\0".as_ptr() as *const c_char, rem_cap)
}
unsafe fn cap_rem_store(dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: usize) -> isize {
    let ret = sysfs_match_string(rl_services.as_ptr(), buf); if ret < 0 { return ret as isize; }
    let err = set_param_u(dev, rl_params::CAP_REM_SRV, ret as u64); if err != 0 { return err as isize; } count as isize
}
// DEVICE_ATTR_RW(cap_rem);

unsafe fn sla_op_store(dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: usize) -> isize {
    let accel_dev = adf_devmgr_pci_to_accel_dev(to_pci_dev(dev)); if accel_dev.is_null() { return -EINVAL as isize; }
    let data = &mut (*(*accel_dev).rate_limiting).user_input; let mut ret = sysfs_match_string(rl_operations.as_ptr(), buf);
    if ret < 0 { return ret as isize; } down_write(&mut data.lock);
    ret = match ret { x if x == ADD as i32 => { data.input.parent_id = RL_PARENT_DEFAULT_ID; data.input.r#type = RL_LEAF; data.input.sla_id = 0; adf_rl_add_sla(accel_dev, &mut data.input) }, x if x == UPDATE as i32 => adf_rl_update_sla(accel_dev, &mut data.input), x if x == RM as i32 => adf_rl_remove_sla(accel_dev, data.input.sla_id), x if x == RM_ALL as i32 => { adf_rl_remove_sla_all(accel_dev, false); 0 }, x if x == GET as i32 => adf_rl_get_sla(accel_dev, &mut data.input), _ => -EINVAL };
    up_write(&mut data.lock); if ret != 0 { return ret as isize; } count as isize
}
// DEVICE_ATTR_WO(sla_op);

// struct attribute *qat_rl_attrs[] and struct attribute_group qat_rl_group;
unsafe fn adf_sysfs_rl_add(accel_dev: *mut adf_accel_dev) -> i32 {
    let data = &mut (*(*accel_dev).rate_limiting).user_input;
    let ret = device_add_group(&mut GET_DEV(accel_dev), &mut qat_rl_group);
    if ret != 0 { dev_err(&mut GET_DEV(accel_dev), b"Failed to create qat_rl attribute group\n\0".as_ptr() as *const c_char); }
    data.cap_rem_srv = SVC_BASE_COUNT as u64; data.input.srv = SVC_BASE_COUNT as u64; data.sysfs_added = true; ret
}
unsafe fn adf_sysfs_rl_rm(accel_dev: *mut adf_accel_dev) {
    let data = &mut (*(*accel_dev).rate_limiting).user_input; if !data.sysfs_added { return; }
    device_remove_group(&mut GET_DEV(accel_dev), &mut qat_rl_group); data.sysfs_added = false;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
