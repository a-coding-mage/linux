// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Sysfs entries for PCI Error Recovery for PAPR-compliant platform.
 * Copyright IBM Corporation 2007
 * Copyright Linas Vepstas <linas@austin.ibm.com> 2007
 */

use core::ffi::{c_char, c_int, c_void};

pub type ssize_t = isize;
pub type size_t = usize;

#[repr(C)] pub struct device { pub kobj: kobject }
#[repr(C)] pub struct kobject { pub sd: *mut c_void }
#[repr(C)] pub struct device_attribute;
#[repr(C)] pub struct pci_dn { pub last_allow_rc: c_int }
#[repr(C)] pub struct eeh_pe { pub state: u32 }
#[repr(C)] pub struct eeh_dev { pub mode: u32, pub pe_config_addr: u32, pub pe: *mut eeh_pe }
#[repr(C)] pub struct pci_dev {
    pub dev: device,
    pub is_physfn: bool,
    pub physfn: *mut pci_dev,
}
#[repr(C)] pub struct eeh_ops_type {
    pub get_state: Option<unsafe extern "C" fn(*mut eeh_pe, *mut c_void) -> c_int>,
    pub notify_resume: Option<unsafe extern "C" fn(*mut eeh_dev) -> c_int>,
}

extern "C" {
    static mut eeh_ops: *mut eeh_ops_type;
    fn to_pci_dev(dev: *mut device) -> *mut pci_dev;
    fn pci_dev_to_eeh_dev(dev: *mut pci_dev) -> *mut eeh_dev;
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> ssize_t;
    fn eeh_unfreeze_pe(pe: *mut eeh_pe) -> c_int;
    fn eeh_pe_state_clear(pe: *mut eeh_pe, state: u32, val: bool);
    fn pci_get_pdn(dev: *mut pci_dev) -> *mut pci_dn;
    fn pci_device_to_OF_node(dev: *mut pci_dev) -> *mut c_void;
    fn of_property_read_bool(np: *mut c_void, name: *const c_char) -> bool;
    fn device_create_file(dev: *mut device, attr: *mut device_attribute) -> c_int;
    fn device_remove_file(dev: *mut device, attr: *mut device_attribute);
    fn eeh_enabled() -> bool;
    fn pr_warn(fmt: *const c_char, ...);
    fn WARN_ON(cond: bool);
}

const ENODEV: ssize_t = -19;
const EIO: ssize_t = -5;
const EEH_PE_ISOLATED: u32 = 1 << 0;
const EEH_DEV_SYSFS: u32 = 1 << 0;

static mut dev_attr_eeh_mode: device_attribute = device_attribute;
static mut dev_attr_eeh_pe_config_addr: device_attribute = device_attribute;
static mut dev_attr_eeh_pe_state: device_attribute = device_attribute;

unsafe fn eeh_show_eeh_mode(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let pdev = to_pci_dev(dev);
    let edev = pci_dev_to_eeh_dev(pdev);
    if edev.is_null() { return 0; }
    sysfs_emit(buf, b"0x%x\n\0".as_ptr() as *const c_char, (*edev).mode)
}

unsafe fn eeh_show_eeh_pe_config_addr(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let pdev = to_pci_dev(dev);
    let edev = pci_dev_to_eeh_dev(pdev);
    if edev.is_null() { return 0; }
    sysfs_emit(buf, b"0x%x\n\0".as_ptr() as *const c_char, (*edev).pe_config_addr)
}

unsafe extern "C" fn eeh_pe_state_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let pdev = to_pci_dev(dev);
    let edev = pci_dev_to_eeh_dev(pdev);
    if edev.is_null() || (*edev).pe.is_null() { return ENODEV; }
    let state = (*eeh_ops).get_state.unwrap()((*edev).pe, core::ptr::null_mut());
    sysfs_emit(buf, b"0x%08x 0x%08x\n\0".as_ptr() as *const c_char, state, (*(*edev).pe).state)
}

unsafe extern "C" fn eeh_pe_state_store(dev: *mut device, _attr: *mut device_attribute, _buf: *const c_char, count: size_t) -> ssize_t {
    let pdev = to_pci_dev(dev);
    let edev = pci_dev_to_eeh_dev(pdev);
    if edev.is_null() || (*edev).pe.is_null() { return ENODEV; }
    if (*(*edev).pe).state & EEH_PE_ISOLATED == 0 { return count as ssize_t; }
    if eeh_unfreeze_pe((*edev).pe) != 0 { return EIO; }
    eeh_pe_state_clear((*edev).pe, EEH_PE_ISOLATED, true);
    count as ssize_t
}

unsafe fn eeh_notify_resume_add(pdev: *mut pci_dev) -> c_int {
    let np = pci_device_to_OF_node(if (*pdev).is_physfn { pdev } else { (*pdev).physfn });
    if of_property_read_bool(np, b"ibm,is-open-sriov-pf\0".as_ptr() as *const c_char) {
        device_create_file(&mut (*pdev).dev, &mut dev_attr_eeh_notify_resume)
    } else { 0 }
}
unsafe extern "C" fn eeh_notify_resume_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let pdev = to_pci_dev(dev);
    let edev = pci_dev_to_eeh_dev(pdev);
    let pdn = pci_get_pdn(pdev);
    if edev.is_null() || (*edev).pe.is_null() { return ENODEV; }
    sysfs_emit(buf, b"%d\n\0".as_ptr() as *const c_char, (*pdn).last_allow_rc)
}
unsafe extern "C" fn eeh_notify_resume_store(dev: *mut device, _attr: *mut device_attribute, _buf: *const c_char, count: size_t) -> ssize_t {
    let pdev = to_pci_dev(dev);
    let edev = pci_dev_to_eeh_dev(pdev);
    if edev.is_null() || (*edev).pe.is_null() || (*eeh_ops).notify_resume.is_none() { return ENODEV; }
    if (*eeh_ops).notify_resume.unwrap()(edev) != 0 { return EIO; }
    count as ssize_t
}
unsafe fn eeh_notify_resume_remove(pdev: *mut pci_dev) {
    let np = pci_device_to_OF_node(if (*pdev).is_physfn { pdev } else { (*pdev).physfn });
    if of_property_read_bool(np, b"ibm,is-open-sriov-pf\0".as_ptr() as *const c_char) { device_remove_file(&mut (*pdev).dev, &mut dev_attr_eeh_notify_resume); }
}
static mut dev_attr_eeh_notify_resume: device_attribute = device_attribute;

pub unsafe fn eeh_sysfs_add_device(pdev: *mut pci_dev) {
    let edev = pci_dev_to_eeh_dev(pdev);
    if !eeh_enabled() || (!edev.is_null() && (*edev).mode & EEH_DEV_SYSFS != 0) { return; }
    let mut rc = 0;
    rc += device_create_file(&mut (*pdev).dev, &mut dev_attr_eeh_mode);
    rc += device_create_file(&mut (*pdev).dev, &mut dev_attr_eeh_pe_config_addr);
    rc += device_create_file(&mut (*pdev).dev, &mut dev_attr_eeh_pe_state);
    rc += eeh_notify_resume_add(pdev);
    if rc != 0 { pr_warn(b"EEH: Unable to create sysfs entries\n\0".as_ptr() as *const c_char); }
    else if !edev.is_null() { (*edev).mode |= EEH_DEV_SYSFS; }
}

pub unsafe fn eeh_sysfs_remove_device(pdev: *mut pci_dev) {
    let edev = pci_dev_to_eeh_dev(pdev);
    if edev.is_null() { WARN_ON(eeh_enabled()); return; }
    (*edev).mode &= !EEH_DEV_SYSFS;
    if (*pdev).dev.kobj.sd.is_null() { return; }
    device_remove_file(&mut (*pdev).dev, &mut dev_attr_eeh_mode);
    device_remove_file(&mut (*pdev).dev, &mut dev_attr_eeh_pe_config_addr);
    device_remove_file(&mut (*pdev).dev, &mut dev_attr_eeh_pe_state);
    eeh_notify_resume_remove(pdev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
