// SPDX-License-Identifier: GPL-2.0-only
/*
 * Link physical devices with ACPI devices support
 *
 * Copyright (c) 2005 David Shaohua Li <shaohua.li@intel.com>
 * Copyright (c) 2005 Intel Corp.
 */

// C headers and the build-time kernel environment are supplied by other files.

static mut BUS_TYPE_LIST: list_head = LIST_HEAD_INIT();
static mut BUS_TYPE_SEM: rw_semaphore = DECLARE_RWSEM_INIT();

const PHYSICAL_NODE_STRING: &str = "physical_node";
const PHYSICAL_NODE_NAME_SIZE: usize = PHYSICAL_NODE_STRING.len() + 10;

pub unsafe fn register_acpi_bus_type(type_: *mut acpi_bus_type) -> c_int {
    if acpi_disabled { return -ENODEV; }
    if !type_.is_null() && (*type_).match_.is_some() && (*type_).find_companion.is_some() {
        down_write(&mut BUS_TYPE_SEM);
        list_add_tail(&mut (*type_).list, &mut BUS_TYPE_LIST);
        up_write(&mut BUS_TYPE_SEM);
        pr_info!("bus type {} registered\n", (*type_).name);
        return 0;
    }
    -ENODEV
}

pub unsafe fn unregister_acpi_bus_type(type_: *mut acpi_bus_type) -> c_int {
    if acpi_disabled { return 0; }
    if !type_.is_null() {
        down_write(&mut BUS_TYPE_SEM);
        list_del_init(&mut (*type_).list);
        up_write(&mut BUS_TYPE_SEM);
        pr_info!("bus type {} unregistered\n", (*type_).name);
        return 0;
    }
    -ENODEV
}

unsafe fn acpi_get_bus_type(dev: *mut device) -> *mut acpi_bus_type {
    let mut ret: *mut acpi_bus_type = core::ptr::null_mut();
    down_read(&mut BUS_TYPE_SEM);
    list_for_each_entry!(tmp, &BUS_TYPE_LIST, list, {
        if ((*tmp).match_)(dev) { ret = tmp; break; }
    });
    up_read(&mut BUS_TYPE_SEM);
    ret
}

const FIND_CHILD_MIN_SCORE: c_int = 1;
const FIND_CHILD_MID_SCORE: c_int = 2;
const FIND_CHILD_MAX_SCORE: c_int = 3;

unsafe fn match_any(_adev: *mut acpi_device, _not_used: *mut c_void) -> c_int { 1 }

unsafe fn acpi_dev_has_children(adev: *mut acpi_device) -> bool {
    acpi_dev_for_each_child(adev, Some(match_any), core::ptr::null_mut()) > 0
}

unsafe fn find_child_checks(adev: *mut acpi_device, check_children: bool) -> c_int {
    let mut sta: u64 = 0;
    if check_children && !acpi_dev_has_children(adev) { return -ENODEV; }
    let status = acpi_evaluate_integer((*adev).handle, "_STA", core::ptr::null(), &mut sta);
    if status == AE_NOT_FOUND {
        if (*adev).pnp.type_.backlight { return FIND_CHILD_MID_SCORE; }
        return FIND_CHILD_MIN_SCORE;
    }
    if ACPI_FAILURE(status) || (sta & ACPI_STA_DEVICE_ENABLED) == 0 { return -ENODEV; }
    if (*adev).pnp.type_.platform_id { return FIND_CHILD_MIN_SCORE; }
    FIND_CHILD_MAX_SCORE
}

#[repr(C)]
struct find_child_walk_data {
    adev: *mut acpi_device,
    address: u64,
    score: c_int,
    check_sta: bool,
    check_children: bool,
}

unsafe fn check_one_child(adev: *mut acpi_device, data: *mut c_void) -> c_int {
    let wd = &mut *(data as *mut find_child_walk_data);
    if !(*adev).pnp.type_.bus_address || acpi_device_adr(adev) != wd.address { return 0; }
    if wd.adev.is_null() {
        wd.adev = adev;
        return if wd.check_sta || wd.check_children { 0 } else { 1 };
    }
    if wd.score == 0 {
        let score = find_child_checks(wd.adev, wd.check_children);
        if score == FIND_CHILD_MAX_SCORE { return 1; }
        wd.score = score;
    }
    let score = find_child_checks(adev, wd.check_children);
    if score > wd.score {
        wd.adev = adev;
        if score == FIND_CHILD_MAX_SCORE { return 1; }
        wd.score = score;
    }
    0
}

unsafe fn acpi_find_child(parent: *mut acpi_device, address: u64, check_children: bool, check_sta: bool) -> *mut acpi_device {
    let mut wd = find_child_walk_data { address, check_children, check_sta, adev: core::ptr::null_mut(), score: 0 };
    if !parent.is_null() { acpi_dev_for_each_child(parent, Some(check_one_child), &mut wd as *mut _ as *mut c_void); }
    wd.adev
}

pub unsafe fn acpi_find_child_device(parent: *mut acpi_device, address: u64, check_children: bool) -> *mut acpi_device { acpi_find_child(parent, address, check_children, true) }
pub unsafe fn acpi_find_child_by_adr(adev: *mut acpi_device, adr: acpi_bus_address) -> *mut acpi_device { acpi_find_child(adev, adr, false, false) }

unsafe fn acpi_physnode_link_name(buf: *mut c_char, node_id: c_uint) {
    if node_id > 0 { snprintf(buf, PHYSICAL_NODE_NAME_SIZE, concat!(PHYSICAL_NODE_STRING, "%u"), node_id); }
    else { strcpy(buf, PHYSICAL_NODE_STRING); }
}

pub unsafe fn acpi_bind_one(dev: *mut device, mut acpi_dev: *mut acpi_device) -> c_int {
    let mut physical_node: *mut acpi_device_physical_node;
    let mut physical_node_name = [0 as c_char; PHYSICAL_NODE_NAME_SIZE];
    let mut physnode_list: *mut list_head;
    let mut node_id: c_uint;
    let mut retval = -EINVAL;
    if has_acpi_companion(dev) { if !acpi_dev.is_null() { dev_warn!(dev, "ACPI companion already set\n"); return -EINVAL; } else { acpi_dev = ACPI_COMPANION(dev); } }
    if acpi_dev.is_null() { return -EINVAL; }
    acpi_dev_get(acpi_dev); get_device(dev);
    physical_node = kzalloc_obj!(*physical_node);
    if physical_node.is_null() { retval = -ENOMEM; goto err; }
    mutex_lock(&mut (*acpi_dev).physical_node_lock);
    physnode_list = &mut (*acpi_dev).physical_node_list;
    node_id = 0;
    list_for_each_entry!(pn, &(*acpi_dev).physical_node_list, node, {
        if (*pn).dev == dev { mutex_unlock(&mut (*acpi_dev).physical_node_lock); dev_warn!(dev, "Already associated with ACPI node\n"); kfree(physical_node); if ACPI_COMPANION(dev) != acpi_dev { goto err; } put_device(dev); acpi_dev_put(acpi_dev); return 0; }
        if (*pn).node_id == node_id { physnode_list = &mut (*pn).node; node_id += 1; }
    });
    (*physical_node).node_id = node_id; (*physical_node).dev = dev; list_add(&mut (*physical_node).node, physnode_list); (*acpi_dev).physical_node_count += 1;
    if !has_acpi_companion(dev) { ACPI_COMPANION_SET(dev, acpi_dev); }
    acpi_physnode_link_name(physical_node_name.as_mut_ptr(), node_id);
    retval = sysfs_create_link(&(*acpi_dev).dev.kobj, &(*dev).kobj, physical_node_name.as_ptr());
    if retval { dev_err!(&(*acpi_dev).dev, "Failed to create link %s (%d)\n", physical_node_name.as_ptr(), retval); }
    retval = sysfs_create_link(&(*dev).kobj, &(*acpi_dev).dev.kobj, "firmware_node");
    if retval { dev_err!(dev, "Failed to create link firmware_node (%d)\n", retval); }
    mutex_unlock(&mut (*acpi_dev).physical_node_lock);
    if (*acpi_dev).wakeup.flags.valid { device_set_wakeup_capable(dev, true); }
    return 0;
err:
    ACPI_COMPANION_SET(dev, core::ptr::null_mut()); put_device(dev); acpi_dev_put(acpi_dev); retval
}

pub unsafe fn acpi_unbind_one(dev: *mut device) -> c_int {
    let acpi_dev = ACPI_COMPANION(dev); if acpi_dev.is_null() { return 0; }
    mutex_lock(&mut (*acpi_dev).physical_node_lock);
    list_for_each_entry!(entry, &(*acpi_dev).physical_node_list, node, {
        if (*entry).dev == dev { let mut name = [0 as c_char; PHYSICAL_NODE_NAME_SIZE]; list_del(&mut (*entry).node); (*acpi_dev).physical_node_count -= 1; acpi_physnode_link_name(name.as_mut_ptr(), (*entry).node_id); sysfs_remove_link(&(*acpi_dev).dev.kobj, name.as_ptr()); sysfs_remove_link(&(*dev).kobj, "firmware_node"); ACPI_COMPANION_SET(dev, core::ptr::null_mut()); put_device(dev); acpi_dev_put(acpi_dev); kfree(entry); break; }
    });
    mutex_unlock(&mut (*acpi_dev).physical_node_lock); 0
}

pub unsafe fn acpi_device_notify(dev: *mut device) {
    let mut adev: *mut acpi_device; let mut ret = acpi_bind_one(dev, core::ptr::null_mut());
    if ret != 0 { let type_ = acpi_get_bus_type(dev); if type_.is_null() { goto err; } adev = ((*type_).find_companion)(dev); if adev.is_null() { dev_dbg!(dev, "ACPI companion not found\n"); goto err; } ret = acpi_bind_one(dev, adev); if ret != 0 { goto err; } if let Some(setup) = (*type_).setup { setup(dev); goto done; } }
    else { adev = ACPI_COMPANION(dev); if dev_is_pci(dev) { pci_acpi_setup(dev, adev); goto done; } else if dev_is_platform(dev) { acpi_configure_pmsi_domain(dev); } }
    if !(*adev).handler.is_null() && (*(*adev).handler).bind.is_some() { ((*(*adev).handler).bind)(dev); }
done: acpi_handle_debug!(ACPI_HANDLE(dev), "Bound to device %s\n", dev_name(dev)); return;
err: dev_dbg!(dev, "No ACPI support\n");
}

pub unsafe fn acpi_device_notify_remove(dev: *mut device) {
    let adev = ACPI_COMPANION(dev); if adev.is_null() { return; }
    if dev_is_pci(dev) { pci_acpi_cleanup(dev, adev); } else if !(*adev).handler.is_null() && (*(*adev).handler).unbind.is_some() { ((*(*adev).handler).unbind)(dev); }
    acpi_unbind_one(dev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
