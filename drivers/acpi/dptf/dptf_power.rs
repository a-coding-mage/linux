// SPDX-License-Identifier: GPL-2.0-only
/*
 * dptf_power:  DPTF platform power driver
 * Copyright (c) 2016, Intel Corporation.
 */

use core::ffi::{c_char, c_int, c_long, c_ulonglong, c_uint, c_void};

// Linux kernel and ACPI declarations are supplied by the surrounding kernel bindings.

#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct device_attribute { _private: [u8; 0] }
#[repr(C)]
pub struct attribute { _private: [u8; 0] }
#[repr(C)]
pub struct attribute_group { pub attrs: *mut *mut attribute, pub name: *const c_char }
#[repr(C)]
pub struct platform_driver { pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>, pub remove: Option<unsafe extern "C" fn(*mut platform_device)> }
#[repr(C)]
pub struct acpi_device_id { pub id: [c_char; 16], pub driver_data: usize }
#[repr(C)]
pub struct acpi_device { pub handle: acpi_handle }
#[repr(C)]
pub struct platform_device { pub dev: device }
pub type acpi_handle = *mut c_void;
pub type acpi_status = c_int;
pub type ssize_t = isize;
pub type size_t = usize;
pub type u32 = c_uint;

// DEVICE_ATTR_RO/DEVICE_ATTR_WO declarations retain their exported names here.
extern "C" {
    static mut dev_attr_max_platform_power_mw: device_attribute;
    static mut dev_attr_platform_power_source: device_attribute;
    static mut dev_attr_adapter_rating_mw: device_attribute;
    static mut dev_attr_battery_steady_power_mw: device_attribute;
    static mut dev_attr_charger_type: device_attribute;
    static mut dev_attr_rest_of_platform_power_mw: device_attribute;
    static mut dev_attr_prochot_confirm: device_attribute;
    static mut dev_attr_max_steady_state_power_mw: device_attribute;
    static mut dev_attr_high_freq_impedance_mohm: device_attribute;
    static mut dev_attr_no_load_voltage_mv: device_attribute;
    static mut dev_attr_current_discharge_capbility_ma: device_attribute;
}

extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn acpi_evaluate_integer(handle: acpi_handle, pathname: *const c_char,
                             args: *mut c_void, data: *mut c_ulonglong) -> acpi_status;
    fn sprintf(buf: *mut c_char, format: *const c_char, ...) -> c_int;
    fn kstrtouint(buf: *const c_char, base: c_uint, res: *mut c_int) -> c_int;
    fn acpi_execute_simple_method(handle: acpi_handle, method: *const c_char, arg: c_int) -> acpi_status;
    fn sysfs_notify(kobj: *mut c_void, group: *const c_char, attr: *const c_char);
    fn dev_err(dev: *mut device, format: *const c_char, ...);
    fn acpi_install_notify_handler(handle: acpi_handle, handler_type: c_uint,
                                   handler: unsafe extern "C" fn(acpi_handle, u32, *mut c_void),
                                   data: *mut c_void) -> c_int;
    fn acpi_remove_notify_handler(handle: acpi_handle, handler_type: c_uint,
                                  handler: unsafe extern "C" fn(acpi_handle, u32, *mut c_void));
    fn acpi_companion(dev: *mut device) -> *mut acpi_device;
    fn sysfs_create_group(kobj: *mut c_void, group: *const attribute_group) -> c_int;
    fn sysfs_remove_group(kobj: *mut c_void, group: *const attribute_group);
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
}

const ENODEV: c_int = 19;
const EINVAL: c_int = 22;
const ACPI_DEVICE_NOTIFY: c_uint = 0x80;

unsafe extern "C" fn power_show(dev: *mut device, _attr: *mut device_attribute,
                                 buf: *mut c_char, object: *const c_char) -> ssize_t {
    let acpi_dev = dev_get_drvdata(dev) as *mut acpi_device;
    let mut val: c_ulonglong = 0;
    let status = acpi_evaluate_integer((*acpi_dev).handle, object, core::ptr::null_mut(), &mut val);
    if status == 0 { sprintf(buf, b"%d\0".as_ptr() as *const c_char, val as c_int) as ssize_t }
    else { -(EINVAL as ssize_t) }
}

unsafe extern "C" fn max_platform_power_mw_show(d: *mut device, a: *mut device_attribute, b: *mut c_char) -> ssize_t { power_show(d, a, b, b"PMAX\0".as_ptr() as *const c_char) }
unsafe extern "C" fn platform_power_source_show(d: *mut device, a: *mut device_attribute, b: *mut c_char) -> ssize_t { power_show(d, a, b, b"PSRC\0".as_ptr() as *const c_char) }
unsafe extern "C" fn adapter_rating_mw_show(d: *mut device, a: *mut device_attribute, b: *mut c_char) -> ssize_t { power_show(d, a, b, b"ARTG\0".as_ptr() as *const c_char) }
unsafe extern "C" fn battery_steady_power_mw_show(d: *mut device, a: *mut device_attribute, b: *mut c_char) -> ssize_t { power_show(d, a, b, b"PBSS\0".as_ptr() as *const c_char) }
unsafe extern "C" fn charger_type_show(d: *mut device, a: *mut device_attribute, b: *mut c_char) -> ssize_t { power_show(d, a, b, b"CTYP\0".as_ptr() as *const c_char) }
unsafe extern "C" fn rest_of_platform_power_mw_show(d: *mut device, a: *mut device_attribute, b: *mut c_char) -> ssize_t { power_show(d, a, b, b"PROP\0".as_ptr() as *const c_char) }
unsafe extern "C" fn max_steady_state_power_mw_show(d: *mut device, a: *mut device_attribute, b: *mut c_char) -> ssize_t { power_show(d, a, b, b"PBSS\0".as_ptr() as *const c_char) }
unsafe extern "C" fn high_freq_impedance_mohm_show(d: *mut device, a: *mut device_attribute, b: *mut c_char) -> ssize_t { power_show(d, a, b, b"RBHF\0".as_ptr() as *const c_char) }
unsafe extern "C" fn no_load_voltage_mv_show(d: *mut device, a: *mut device_attribute, b: *mut c_char) -> ssize_t { power_show(d, a, b, b"VBNL\0".as_ptr() as *const c_char) }
unsafe extern "C" fn current_discharge_capbility_ma_show(d: *mut device, a: *mut device_attribute, b: *mut c_char) -> ssize_t { power_show(d, a, b, b"CMPP\0".as_ptr() as *const c_char) }

unsafe extern "C" fn prochot_confirm_store(dev: *mut device, _attr: *mut device_attribute,
                                             buf: *const c_char, count: size_t) -> ssize_t {
    let acpi_dev = dev_get_drvdata(dev) as *mut acpi_device;
    let mut seq_no: c_int = 0;
    if kstrtouint(buf, 0, &mut seq_no) < 0 { return -(EINVAL as ssize_t); }
    if acpi_execute_simple_method((*acpi_dev).handle, b"PBOK\0".as_ptr() as *const c_char, seq_no) == 0 { count as ssize_t }
    else { -(EINVAL as ssize_t) }
}

const MAX_POWER_CHANGED: u32 = 0x80;
const POWER_STATE_CHANGED: u32 = 0x81;
const STEADY_STATE_POWER_CHANGED: u32 = 0x83;
const POWER_PROP_CHANGE_EVENT: u32 = 0x84;
const IMPEDANCE_CHANGED: u32 = 0x85;
const VOLTAGE_CURRENT_CHANGED: u32 = 0x86;

unsafe fn dptf_participant_type(handle: acpi_handle) -> c_long {
    let mut ptype: c_ulonglong = 0;
    if acpi_evaluate_integer(handle, b"PTYP\0".as_ptr() as *const c_char, core::ptr::null_mut(), &mut ptype) != 0 { -(ENODEV as c_long) } else { ptype as c_long }
}

unsafe extern "C" fn dptf_power_notify(handle: acpi_handle, event: u32, data: *mut c_void) {
    let pdev = data as *mut platform_device;
    let attr: *const c_char = match event {
        POWER_STATE_CHANGED => b"platform_power_source\0".as_ptr() as *const c_char,
        POWER_PROP_CHANGE_EVENT => b"rest_of_platform_power_mw\0".as_ptr() as *const c_char,
        MAX_POWER_CHANGED => b"max_platform_power_mw\0".as_ptr() as *const c_char,
        STEADY_STATE_POWER_CHANGED => b"max_steady_state_power_mw\0".as_ptr() as *const c_char,
        IMPEDANCE_CHANGED => b"high_freq_impedance_mohm\0".as_ptr() as *const c_char,
        VOLTAGE_CURRENT_CHANGED => b"no_load_voltage_mv\0".as_ptr() as *const c_char,
        _ => { dev_err(&mut (*pdev).dev, b"Unsupported event [0x%x]\n\0".as_ptr() as *const c_char, event); return; }
    };
    let group = if dptf_participant_type(handle) == 0x0C { b"dptf_battery\0" } else { b"dptf_power\0" };
    sysfs_notify(core::ptr::null_mut(), group.as_ptr() as *const c_char, attr);
}

unsafe extern "C" fn dptf_power_add(pdev: *mut platform_device) -> c_int {
    let acpi_dev = acpi_companion(&mut (*pdev).dev);
    if acpi_dev.is_null() { return -ENODEV; }
    let ptype = dptf_participant_type((*acpi_dev).handle);
    if ptype != 0x11 && ptype != 0x0c { return -ENODEV; }
    let result = acpi_install_notify_handler((*acpi_dev).handle, ACPI_DEVICE_NOTIFY, dptf_power_notify, pdev as *mut c_void);
    if result != 0 { return result; }
    platform_set_drvdata(pdev, acpi_dev as *mut c_void);
    result
}

unsafe extern "C" fn dptf_power_remove(pdev: *mut platform_device) {
    let acpi_dev = platform_get_drvdata(pdev) as *mut acpi_device;
    acpi_remove_notify_handler((*acpi_dev).handle, ACPI_DEVICE_NOTIFY, dptf_power_notify);
}

static mut INT3407_DEVICE_IDS: [acpi_device_id; 19] = [
    acpi_device_id { id: *b"INT3407\0\0\0\0\0\0\0\0\0", driver_data: 0 },
    acpi_device_id { id: *b"INT3532\0\0\0\0\0\0\0\0\0", driver_data: 0 },
    acpi_device_id { id: *b"INTC1047\0\0\0\0\0\0\0\0", driver_data: 0 },
    acpi_device_id { id: *b"INTC1050\0\0\0\0\0\0\0\0", driver_data: 0 },
    acpi_device_id { id: *b"INTC1060\0\0\0\0\0\0\0\0", driver_data: 0 },
    acpi_device_id { id: *b"INTC1061\0\0\0\0\0\0\0\0", driver_data: 0 },
    acpi_device_id { id: *b"INTC1065\0\0\0\0\0\0\0\0", driver_data: 0 },
    acpi_device_id { id: *b"INTC1066\0\0\0\0\0\0\0\0", driver_data: 0 },
    acpi_device_id { id: *b"INTC106C\0\0\0\0\0\0\0\0", driver_data: 0 },
    acpi_device_id { id: *b"INTC106D\0\0\0\0\0\0\0\0", driver_data: 0 },
    acpi_device_id { id: *b"INTC10A4\0\0\0\0\0\0\0\0", driver_data: 0 },
    acpi_device_id { id: *b"INTC10A5\0\0\0\0\0\0\0\0", driver_data: 0 },
    acpi_device_id { id: *b"INTC10D8\0\0\0\0\0\0\0\0", driver_data: 0 },
    acpi_device_id { id: *b"INTC10D9\0\0\0\0\0\0\0\0", driver_data: 0 },
    acpi_device_id { id: *b"INTC1100\0\0\0\0\0\0\0\0", driver_data: 0 },
    acpi_device_id { id: *b"INTC1101\0\0\0\0\0\0\0\0", driver_data: 0 },
    acpi_device_id { id: *b"INTC10F7\0\0\0\0\0\0\0\0", driver_data: 0 },
    acpi_device_id { id: *b"INTC10F8\0\0\0\0\0\0\0\0", driver_data: 0 },
    acpi_device_id { id: [0; 16], driver_data: 0 },
];

static mut DPTF_POWER_DRIVER: platform_driver = platform_driver { probe: Some(dptf_power_add), remove: Some(dptf_power_remove) };

// module_platform_driver(DPTF_POWER_DRIVER)
// MODULE_AUTHOR("Srinivas Pandruvada <srinivas.pandruvada@linux.intel.com>")
// MODULE_LICENSE("GPL v2")
// MODULE_DESCRIPTION("ACPI DPTF platform power driver")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
