// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright IBM Corp. 2012
 *
 * Author(s):
 *   Jan Glauber <jang@linux.vnet.ibm.com>
 */

// Dependencies supplied by the surrounding kernel translation unit.

extern "C" {
    static mut firmware_kobj: *mut kobject;
    static mut zpci_unique_uid: bool;
    fn to_pci_dev(dev: *mut device) -> *mut pci_dev;
    fn to_zpci(dev: *mut pci_dev) -> *mut zpci_dev;
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> isize;
    fn zpci_use_mio(zdev: *mut zpci_dev) -> bool;
    fn zdev_enabled(zdev: *mut zpci_dev) -> bool;
    fn pci_stop_and_remove_bus_device(pdev: *mut pci_dev);
    fn zpci_disable_device(zdev: *mut zpci_dev) -> c_int;
    fn zpci_reenable_device(zdev: *mut zpci_dev) -> c_int;
    fn sysfs_break_active_protection(kobj: *mut kobject, attr: *mut attribute) -> *mut kernfs_node;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn device_remove_file(dev: *mut device, attr: *mut device_attribute);
    fn pci_lock_rescan_remove();
    fn pci_dev_is_added(pdev: *mut pci_dev) -> bool;
    fn pci_rescan_bus(bus: *mut pci_bus);
    fn pci_unlock_rescan_remove();
    fn sysfs_unbreak_active_protection(kn: *mut kernfs_node);
    fn memory_read_from_buffer(buf: *mut c_char, count: usize, off: *mut loff_t,
                               from: *const c_void, available: usize) -> isize;
    fn kobj_to_dev(kobj: *mut kobject) -> *mut device;
    fn sclp_pci_report(report: *mut zpci_report_error_header, fh: u32, fid: u32) -> c_int;
    fn container_of<T>(ptr: *mut c_void, member: usize) -> *mut T;
    fn sysfs_create_group(kobj: *mut kobject, group: *const attribute_group) -> c_int;
}

use core::ffi::{c_char, c_int, c_void};
type loff_t = i64;
type umode_t = u16;

#[repr(C)] pub struct device { pub kobj: kobject }
#[repr(C)] pub struct pci_dev { _private: [u8; 0] }
#[repr(C)] pub struct pci_bus { _private: [u8; 0] }
#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct kernfs_node { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct pci_slot { pub hotplug: *mut c_void }
#[repr(C)] pub struct zpci_report_error_header { _private: [u8; 0] }

#[repr(C)] pub struct zpci_dev {
    pub fid: u32, pub fh: u32, pub pchid: u16, pub pfgid: u8, pub vfn: u16,
    pub pft: u8, pub port: i32, pub fidparm: u8, pub uid: u32,
    pub pfip: [u8; 4], pub util_str: [u8; 0], pub state_lock: mutex,
    pub state: u32, pub zbus: *mut zpci_bus, pub hotplug_slot: pci_slot,
}
#[repr(C)] pub struct zpci_bus { pub bus: *mut pci_bus }
#[repr(C)] pub struct attribute { pub mode: umode_t }
#[repr(C)] pub struct device_attribute { pub attr: attribute }
#[repr(C)] pub struct kobj_attribute { pub attr: attribute }
#[repr(C)] pub struct bin_attribute { pub attr: attribute }
#[repr(C)] pub struct attribute_group {
    pub name: *const c_char, pub attrs: *mut *mut attribute,
    pub bin_attrs: *const *const bin_attribute,
    pub is_visible: Option<unsafe extern "C" fn(*mut kobject, *mut attribute, c_int) -> umode_t>,
}

const EINVAL: c_int = 22;
const S_IWUSR: u16 = 0o200;
const PAGE_SIZE: usize = 4096;
const ZPCI_FN_STATE_CONFIGURED: u32 = 1;
const CLP_UTIL_STR_LEN: usize = 0;

macro_rules! zpci_attr {
    ($name:ident, $fmt:literal, $member:ident) => {
        unsafe extern "C" fn $name##_show(dev: *mut device, _attr: *mut device_attribute,
                                          buf: *mut c_char) -> isize {
            let zdev = to_zpci(to_pci_dev(dev));
            sysfs_emit(buf, concat!($fmt, "\0").as_ptr() as *const c_char, (*zdev).$member)
        }
        static mut $name##_attr: device_attribute = device_attribute { attr: attribute { mode: 0 } };
    };
}

zpci_attr!(function_id, "0x%08x\n", fid);
zpci_attr!(function_handle, "0x%08x\n", fh);
zpci_attr!(pchid, "0x%04x\n", pchid);
zpci_attr!(pfgid, "0x%02x\n", pfgid);
zpci_attr!(vfn, "0x%04x\n", vfn);
zpci_attr!(pft, "0x%02x\n", pft);
zpci_attr!(port, "%d\n", port);
zpci_attr!(fidparm, "0x%02x\n", fidparm);
zpci_attr!(uid, "0x%x\n", uid);

unsafe extern "C" fn mio_enabled_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> isize {
    let zdev = to_zpci(to_pci_dev(dev));
    sysfs_emit(buf, if zpci_use_mio(zdev) { b"1\n\0" } else { b"0\n\0" }.as_ptr() as *const c_char)
}
static mut mio_enabled_attr: device_attribute = device_attribute { attr: attribute { mode: 0 } };

unsafe fn _do_recover(pdev: *mut pci_dev, zdev: *mut zpci_dev) -> c_int {
    pci_stop_and_remove_bus_device(pdev);
    if zdev_enabled(zdev) {
        let mut ret = zpci_disable_device(zdev);
        if ret == -EINVAL { ret = 0; }
        if ret != 0 { return ret; }
    }
    zpci_reenable_device(zdev)
}

unsafe extern "C" fn recover_store(dev: *mut device, attr: *mut device_attribute,
                                    _buf: *const c_char, count: usize) -> isize {
    let pdev = to_pci_dev(dev); let zdev = to_zpci(pdev);
    let kn = sysfs_break_active_protection(&mut (*dev).kobj, &mut (*attr).attr);
    mutex_lock(&mut (*zdev).state_lock);
    let mut ret = 0;
    if (*zdev).state == ZPCI_FN_STATE_CONFIGURED {
        device_remove_file(dev, attr);
        pci_lock_rescan_remove();
        if pci_dev_is_added(pdev) { ret = _do_recover(pdev, zdev); }
        pci_rescan_bus((*(*zdev).zbus).bus);
        pci_unlock_rescan_remove();
    }
    mutex_unlock(&mut (*zdev).state_lock);
    if !kn.is_null() { sysfs_unbreak_active_protection(kn); }
    if ret != 0 { ret as isize } else { count as isize }
}
static mut recover_attr: device_attribute = device_attribute { attr: attribute { mode: 0 } };

unsafe extern "C" fn util_string_read(_filp: *mut file, kobj: *mut kobject,
                                       _attr: *const bin_attribute, buf: *mut c_char,
                                       mut off: loff_t, count: usize) -> isize {
    let zdev = to_zpci(to_pci_dev(kobj_to_dev(kobj)));
    memory_read_from_buffer(buf, count, &mut off, (*zdev).util_str.as_ptr() as *const c_void, 0)
}
static mut util_string_attr: bin_attribute = bin_attribute { attr: attribute { mode: 0 } };

unsafe extern "C" fn report_error_write(_filp: *mut file, kobj: *mut kobject,
                                         _attr: *const bin_attribute, buf: *mut c_char,
                                         off: loff_t, count: usize) -> isize {
    if off != 0 || count < core::mem::size_of::<zpci_report_error_header>() { return -EINVAL as isize; }
    let zdev = to_zpci(to_pci_dev(kobj_to_dev(kobj)));
    let ret = sclp_pci_report(buf as *mut zpci_report_error_header, (*zdev).fh, (*zdev).fid);
    if ret != 0 { ret as isize } else { count as isize }
}
static mut report_error_attr: bin_attribute = bin_attribute { attr: attribute { mode: S_IWUSR } };

unsafe extern "C" fn uid_is_unique_show(_dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> isize {
    sysfs_emit(buf, b"%d\n\0".as_ptr() as *const c_char, if zpci_unique_uid { 1 } else { 0 })
}
static mut uid_is_unique_attr: device_attribute = device_attribute { attr: attribute { mode: 0 } };
unsafe extern "C" fn uid_checking_show(_kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *mut c_char) -> isize {
    sysfs_emit(buf, b"%d\n\0".as_ptr() as *const c_char, if zpci_unique_uid { 1 } else { 0 })
}
static mut uid_checking_attr: kobj_attribute = kobj_attribute { attr: attribute { mode: 0 } };

unsafe extern "C" fn index_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> isize {
    let zdev = to_zpci(to_pci_dev(dev)); let index = if zpci_unique_uid { (*zdev).uid } else { u32::MAX };
    sysfs_emit(buf, b"%u\n\0".as_ptr() as *const c_char, index)
}
static mut index_attr: device_attribute = device_attribute { attr: attribute { mode: 0 } };

unsafe extern "C" fn zpci_uid_slot_show(slot: *mut pci_slot, buf: *mut c_char) -> isize {
    let zdev = container_of::<zpci_dev>((*slot).hotplug, 0);
    sysfs_emit(buf, b"0x%x\n\0".as_ptr() as *const c_char, (*zdev).uid)
}
static mut zpci_slot_attr_uid: attribute = attribute { mode: 0o444 };
unsafe extern "C" fn zpci_index_is_visible(_kobj: *mut kobject, attr: *mut attribute, _n: c_int) -> umode_t {
    if zpci_unique_uid { (*attr).mode } else { 0 }
}

static mut zpci_ident_attrs: [*mut attribute; 2] = [unsafe { &mut index_attr.attr }, core::ptr::null_mut()];
#[no_mangle] pub static zpci_ident_attr_group: attribute_group = attribute_group {
    name: core::ptr::null(), attrs: unsafe { zpci_ident_attrs.as_mut_ptr() }, bin_attrs: core::ptr::null(), is_visible: Some(zpci_index_is_visible),
};
static mut zpci_bin_attrs: [*const bin_attribute; 3] = [unsafe { &util_string_attr }, unsafe { &report_error_attr }, core::ptr::null()];
static mut zpci_dev_attrs: [*mut attribute; 1] = [core::ptr::null_mut()];
#[no_mangle] pub static zpci_attr_group: attribute_group = attribute_group { name: core::ptr::null(), attrs: unsafe { zpci_dev_attrs.as_mut_ptr() }, bin_attrs: unsafe { zpci_bin_attrs.as_ptr() }, is_visible: None };
static mut pfip_attrs: [*mut attribute; 1] = [core::ptr::null_mut()];
#[no_mangle] pub static pfip_attr_group: attribute_group = attribute_group { name: b"pfip\0".as_ptr() as *const c_char, attrs: unsafe { pfip_attrs.as_mut_ptr() }, bin_attrs: core::ptr::null(), is_visible: None };
static mut zpci_slot_attrs: [*mut attribute; 2] = [unsafe { &mut zpci_slot_attr_uid }, core::ptr::null_mut()];
#[no_mangle] pub static zpci_slot_attr_group: attribute_group = attribute_group { name: core::ptr::null(), attrs: unsafe { zpci_slot_attrs.as_mut_ptr() }, bin_attrs: core::ptr::null(), is_visible: None };
static mut clp_fw_attrs: [*mut attribute; 2] = [unsafe { &mut uid_checking_attr.attr }, core::ptr::null_mut()];
static clp_fw_attr_group: attribute_group = attribute_group { name: b"clp\0".as_ptr() as *const c_char, attrs: unsafe { clp_fw_attrs.as_mut_ptr() }, bin_attrs: core::ptr::null(), is_visible: None };

pub unsafe extern "C" fn __zpci_fw_sysfs_init() -> c_int {
    sysfs_create_group(firmware_kobj, &clp_fw_attr_group)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
