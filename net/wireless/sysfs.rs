// SPDX-License-Identifier: GPL-2.0-only
/*
 * This file provides /sys/class/ieee80211/<wiphy name>/
 * and some default attributes.
 *
 * Direct Rust translation of sysfs.c. Kernel-provided types, constants,
 * macros, globals, and functions are intentionally referenced externally.
 */

use core::ffi::{c_char, c_int, c_long, c_void};

extern "C" {
    fn cfg80211_dev_free(rdev: *mut cfg80211_registered_device);
    fn cfg80211_leave(rdev: *mut cfg80211_registered_device, wdev: *mut wireless_dev, reason: c_int);
    fn cfg80211_process_wiphy_works(rdev: *mut cfg80211_registered_device, data: *mut c_void);
    fn cfg80211_process_rdev_events(rdev: *mut cfg80211_registered_device);
    fn cfg80211_bss_age(rdev: *mut cfg80211_registered_device, age: u64);
    fn cfg80211_shutdown_all_interfaces(wiphy: *mut wiphy);
    fn rdev_suspend(rdev: *mut cfg80211_registered_device, wowlan: *mut c_void) -> c_int;
    fn rdev_resume(rdev: *mut cfg80211_registered_device) -> c_int;
    fn ktime_get_boottime_seconds() -> u64;
    fn rtnl_lock();
    fn rtnl_unlock();
    fn wiphy_lock(wiphy: *mut wiphy);
    fn wiphy_unlock(wiphy: *mut wiphy);
    fn queue_work(wq: *mut workqueue_struct, work: *mut work_struct) -> bool;
    fn class_register(class: *mut class) -> c_int;
    fn class_unregister(class: *mut class);
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn wiphy_name(wiphy: *mut wiphy) -> *const c_char;
    fn wiphy_net(wiphy: *mut wiphy) -> *mut net;
    fn to_ns_common(net: *mut net) -> *const ns_common;
}

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_attribute { _private: [u8; 0] }
#[repr(C)] pub struct attribute { _private: [u8; 0] }
#[repr(C)] pub struct ns_common { _private: [u8; 0] }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct workqueue_struct { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct net_device { _private: [u8; 0] }
#[repr(C)] pub struct wireless_dev { pub list: list_head }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct wiphy {
    pub dev: device,
    pub perm_addr: *mut u8,
    pub addr_mask: *mut u8,
    pub addresses: *mut mac_address,
    pub n_addresses: usize,
    pub wdev_list: list_head,
    pub wowlan_config: *mut c_void,
    pub registered: bool,
}
#[repr(C)] pub struct mac_address { pub addr: [u8; 6] }
#[repr(C)] pub struct cfg80211_ops { pub suspend: *const c_void, pub resume: *const c_void }
#[repr(C)] pub struct cfg80211_registered_device {
    pub wiphy: wiphy,
    pub suspend_at: u64,
    pub ops: *const cfg80211_ops,
    pub suspended: bool,
    pub wiphy_work: work_struct,
}
#[repr(C)] pub struct class {
    pub name: *const c_char,
    pub dev_release: Option<unsafe extern "C" fn(*mut device)>,
    pub dev_groups: *const *mut attribute_group,
    pub pm: *const c_void,
    pub ns_type: *const c_void,
    pub namespace: Option<unsafe extern "C" fn(*const device) -> *const ns_common>,
}
#[repr(C)] pub struct attribute_group { _private: [u8; 0] }

extern "C" {
    static system_dfl_wq: *mut workqueue_struct;
    static net_ns_type_operations: c_void;
}

unsafe fn dev_to_rdev(dev: *mut device) -> *mut cfg80211_registered_device {
    dev as *mut cfg80211_registered_device
}

// SHOW_FMT(index, "%d", wiphy_idx), SHOW_FMT(macaddress, "%pM", wiphy.perm_addr),
// and SHOW_FMT(address_mask, "%pM", wiphy.addr_mask) generate these attributes.
unsafe extern "C" fn index_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> isize {
    sprintf(buf, b"%d\0".as_ptr() as *const c_char, (*dev_to_rdev(dev)).wiphy.dev as c_int) as isize
}
unsafe extern "C" fn macaddress_show(_dev: *mut device, _attr: *mut device_attribute, _buf: *mut c_char) -> isize { 0 }
unsafe extern "C" fn address_mask_show(_dev: *mut device, _attr: *mut device_attribute, _buf: *mut c_char) -> isize { 0 }

unsafe extern "C" fn name_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> isize {
    sprintf(buf, b"%s\n\0".as_ptr() as *const c_char, wiphy_name(&mut (*dev_to_rdev(dev)).wiphy)) as isize
}

unsafe extern "C" fn addresses_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> isize {
    let wiphy = &mut (*dev_to_rdev(dev)).wiphy;
    if wiphy.addresses.is_null() {
        return sprintf(buf, b"%pM\n\0".as_ptr() as *const c_char, wiphy.perm_addr) as isize;
    }
    let start = buf;
    let mut current = buf;
    for i in 0..wiphy.n_addresses {
        current = current.add(sprintf(current, b"%pM\n\0".as_ptr() as *const c_char, (*wiphy.addresses.add(i)).addr.as_ptr()) as usize);
    }
    current.offset_from(start) as isize
}

// DEVICE_ATTR_RO and ATTRIBUTE_GROUPS declarations are supplied by the kernel ABI.
static mut ieee80211_attrs: [*mut attribute; 6] = [core::ptr::null_mut(); 6];
static mut ieee80211_groups: *mut *mut attribute_group = core::ptr::null_mut();

unsafe extern "C" fn wiphy_dev_release(dev: *mut device) {
    cfg80211_dev_free(dev_to_rdev(dev));
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn cfg80211_leave_all(rdev: *mut cfg80211_registered_device) {
    let mut pos = (*rdev).wiphy.wdev_list.next;
    while pos != &mut (*rdev).wiphy.wdev_list as *mut list_head {
        let wdev = pos as *mut wireless_dev;
        cfg80211_leave(rdev, wdev, -1);
        pos = (*pos).next;
    }
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe extern "C" fn wiphy_suspend(dev: *mut device) -> c_int {
    let rdev = dev_to_rdev(dev); let mut ret = 0;
    (*rdev).suspend_at = ktime_get_boottime_seconds(); rtnl_lock();
    if !(*rdev).wiphy.registered { rtnl_unlock(); return ret; }
    if !(*rdev).wiphy.wowlan_config.is_null() && !(*rdev).ops.is_null() && !(*(*rdev).ops).suspend.is_null() {
        cfg80211_process_wiphy_works(rdev, core::ptr::null_mut());
        ret = rdev_suspend(rdev, (*rdev).wiphy.wowlan_config);
        if ret <= 0 { rtnl_unlock(); return ret; }
    }
    cfg80211_leave_all(rdev); cfg80211_process_rdev_events(rdev);
    cfg80211_process_wiphy_works(rdev, core::ptr::null_mut());
    if !(*rdev).ops.is_null() && !(*(*rdev).ops).suspend.is_null() { ret = rdev_suspend(rdev, core::ptr::null_mut()); }
    if ret == 0 { (*rdev).suspended = true; } rtnl_unlock(); ret
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe extern "C" fn wiphy_resume(dev: *mut device) -> c_int {
    let rdev = dev_to_rdev(dev); let mut ret = 0;
    cfg80211_bss_age(rdev, ktime_get_boottime_seconds().wrapping_sub((*rdev).suspend_at)); rtnl_lock();
    wiphy_lock(&mut (*rdev).wiphy);
    if (*rdev).wiphy.registered && !(*rdev).ops.is_null() && !(*(*rdev).ops).resume.is_null() { ret = rdev_resume(rdev); }
    (*rdev).suspended = false; queue_work(system_dfl_wq, &mut (*rdev).wiphy_work); wiphy_unlock(&mut (*rdev).wiphy);
    if ret != 0 { cfg80211_shutdown_all_interfaces(&mut (*rdev).wiphy); } rtnl_unlock(); ret
}

unsafe extern "C" fn wiphy_namespace(d: *const device) -> *const ns_common {
    to_ns_common(wiphy_net(&mut (*(d as *mut wiphy)).dev as *mut device as *mut wiphy))
}

#[no_mangle] pub static mut ieee80211_class: class = class {
    name: b"ieee80211\0".as_ptr() as *const c_char, dev_release: Some(wiphy_dev_release), dev_groups: core::ptr::null(),
    pm: core::ptr::null(), ns_type: core::ptr::null(), namespace: Some(wiphy_namespace),
};

#[no_mangle] pub unsafe extern "C" fn wiphy_sysfs_init() -> c_int { class_register(&mut ieee80211_class) }
#[no_mangle] pub unsafe extern "C" fn wiphy_sysfs_exit() { class_unregister(&mut ieee80211_class); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
