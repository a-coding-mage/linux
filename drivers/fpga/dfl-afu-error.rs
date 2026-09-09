// SPDX-License-Identifier: GPL-2.0
/*
 * Driver for FPGA Accelerated Function Unit (AFU) Error Reporting
 *
 * Copyright 2019 Intel Corporation, Inc.
 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

const PORT_ERROR_MASK: usize = 0x8;
const PORT_ERROR: usize = 0x10;
const PORT_FIRST_ERROR: usize = 0x18;
const PORT_MALFORMED_REQ0: usize = 0x20;
const PORT_MALFORMED_REQ1: usize = 0x28;
const ERROR_MASK: u64 = u64::MAX;

// Supplied by the surrounding DFL driver and kernel headers.
type Bool = bool;
type U64 = u64;
type SizeT = usize;
type SSizeT = isize;
type UmodeT = u16;

#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct platform_device { pub dev: device }
#[repr(C)]
pub struct dfl_feature_dev_data { pub lock: c_void, _private: [u8; 0] }
#[repr(C)]
pub struct dfl_feature { _private: [u8; 0] }
#[repr(C)]
pub struct device_attribute { _private: [u8; 0] }
#[repr(C)]
pub struct attribute { pub mode: UmodeT }
#[repr(C)]
pub struct attribute_group { pub name: *const c_char, pub attrs: *mut *mut attribute, pub is_visible: Option<unsafe extern "C" fn(*mut kobject, *mut attribute, c_int) -> UmodeT> }
#[repr(C)]
pub struct kobject { _private: [u8; 0] }
#[repr(C)]
pub struct dfl_feature_id { pub id: c_int }
#[repr(C)]
pub struct dfl_feature_ops { pub init: Option<unsafe extern "C" fn(*mut platform_device, *mut dfl_feature) -> c_int>, pub uinit: Option<unsafe extern "C" fn(*mut platform_device, *mut dfl_feature)>, pub ioctl: Option<unsafe extern "C" fn(*mut platform_device, *mut dfl_feature, c_uint, c_ulong) -> c_long> }
type c_uint = u32;

extern "C" {
    static PORT_FEATURE_ID_ERROR: c_int;
    static PORT_FEATURE_ID_HEADER: c_int;
    static PORT_HDR_STS: usize;
    static PORT_STS_PWR_STATE: u64;
    static PORT_STS_PWR_STATE_AP6: u64;
    static DFL_FPGA_PORT_ERR_GET_IRQ_NUM: c_uint;
    static DFL_FPGA_PORT_ERR_SET_IRQ: c_uint;
    fn dfl_get_feature_ioaddr_by_id(fdata: *mut dfl_feature_dev_data, id: c_int) -> *mut c_void;
    fn to_dfl_feature_dev_data(dev: *mut device) -> *mut dfl_feature_dev_data;
    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);
    fn readq(addr: *const c_void) -> u64;
    fn writeq(value: u64, addr: *mut c_void);
    fn __afu_port_disable(fdata: *mut dfl_feature_dev_data) -> c_int;
    fn __afu_port_enable(fdata: *mut dfl_feature_dev_data) -> c_int;
    fn dfl_get_feature_by_id(fdata: *mut dfl_feature_dev_data, id: c_int) -> *mut c_void;
    fn dfl_feature_ioctl_get_num_irqs(pdev: *mut platform_device, feature: *mut dfl_feature, arg: c_ulong) -> c_long;
    fn dfl_feature_ioctl_set_irq(pdev: *mut platform_device, feature: *mut dfl_feature, arg: c_ulong) -> c_long;
}

unsafe fn __afu_port_err_mask(fdata: *mut dfl_feature_dev_data, mask: Bool) {
    let base = dfl_get_feature_ioaddr_by_id(fdata, PORT_FEATURE_ID_ERROR);
    writeq(if mask { ERROR_MASK } else { 0 }, (base as *mut u8).add(PORT_ERROR_MASK) as *mut c_void);
}

unsafe fn afu_port_err_mask(dev: *mut device, mask: Bool) {
    let fdata = to_dfl_feature_dev_data(dev);
    mutex_lock(&mut (*fdata).lock);
    __afu_port_err_mask(fdata, mask);
    mutex_unlock(&mut (*fdata).lock);
}

unsafe fn afu_port_err_clear(dev: *mut device, err: U64) -> c_int {
    let fdata = to_dfl_feature_dev_data(dev);
    let base_err = dfl_get_feature_ioaddr_by_id(fdata, PORT_FEATURE_ID_ERROR);
    let base_hdr = dfl_get_feature_ioaddr_by_id(fdata, PORT_FEATURE_ID_HEADER);
    let mut enable_ret: c_int = 0;
    let mut ret: c_int = -16; // -EBUSY
    mutex_lock(&mut (*fdata).lock);
    let v = readq((base_hdr as *mut u8).add(PORT_HDR_STS) as *const c_void);
    if (v & PORT_STS_PWR_STATE) == PORT_STS_PWR_STATE_AP6 { goto_done(&mut ret, &mut enable_ret); mutex_unlock(&mut (*fdata).lock); return if enable_ret != 0 { enable_ret } else { ret }; }
    ret = __afu_port_disable(fdata);
    if ret != 0 { mutex_unlock(&mut (*fdata).lock); return if enable_ret != 0 { enable_ret } else { ret }; }
    __afu_port_err_mask(fdata, true);
    let v = readq((base_err as *mut u8).add(PORT_ERROR) as *const c_void);
    if v == err {
        writeq(v, (base_err as *mut u8).add(PORT_ERROR) as *mut c_void);
        let first = readq((base_err as *mut u8).add(PORT_FIRST_ERROR) as *const c_void);
        writeq(first, (base_err as *mut u8).add(PORT_FIRST_ERROR) as *mut c_void);
    } else { ret = -22; }
    __afu_port_err_mask(fdata, false);
    enable_ret = __afu_port_enable(fdata);
    mutex_unlock(&mut (*fdata).lock);
    if enable_ret != 0 { enable_ret } else { ret }
}

unsafe fn goto_done(_ret: &mut c_int, _enable_ret: &mut c_int) {}

#[no_mangle]
pub unsafe extern "C" fn errors_show(_dev: *mut device, _attr: *mut device_attribute, _buf: *mut c_char) -> SSizeT { 0 }
#[no_mangle]
pub unsafe extern "C" fn errors_store(_dev: *mut device, _attr: *mut device_attribute, _buff: *const c_char, count: SizeT) -> SSizeT { count as SSizeT }
#[no_mangle]
pub unsafe extern "C" fn first_error_show(_dev: *mut device, _attr: *mut device_attribute, _buf: *mut c_char) -> SSizeT { 0 }
#[no_mangle]
pub unsafe extern "C" fn first_malformed_req_show(_dev: *mut device, _attr: *mut device_attribute, _buf: *mut c_char) -> SSizeT { 0 }

pub static mut port_err_group: attribute_group = attribute_group { name: b"errors\0".as_ptr() as *const c_char, attrs: core::ptr::null_mut(), is_visible: None };

unsafe extern "C" fn port_err_init(pdev: *mut platform_device, _feature: *mut dfl_feature) -> c_int { afu_port_err_mask(&mut (*pdev).dev, false); 0 }
unsafe extern "C" fn port_err_uinit(pdev: *mut platform_device, _feature: *mut dfl_feature) { afu_port_err_mask(&mut (*pdev).dev, true); }
unsafe extern "C" fn port_err_ioctl(pdev: *mut platform_device, feature: *mut dfl_feature, cmd: c_uint, arg: c_ulong) -> c_long {
    if cmd == DFL_FPGA_PORT_ERR_GET_IRQ_NUM { dfl_feature_ioctl_get_num_irqs(pdev, feature, arg) } else if cmd == DFL_FPGA_PORT_ERR_SET_IRQ { dfl_feature_ioctl_set_irq(pdev, feature, arg) } else { -19 }
}

pub static port_err_id_table: [dfl_feature_id; 2] = [dfl_feature_id { id: 0 }, dfl_feature_id { id: 0 }];
pub static port_err_ops: dfl_feature_ops = dfl_feature_ops { init: Some(port_err_init), uinit: Some(port_err_uinit), ioctl: Some(port_err_ioctl) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
