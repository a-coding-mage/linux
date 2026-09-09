// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  dock.rs - ACPI dock station driver
 *
 *  Copyright (C) 2006, 2014, Intel Corp.
 *  Author: Kristen Carlson Accardi <kristen.c.accardi@intel.com>
 *          Rafael J. Wysocki <rafael.j.wysocki@intel.com>
 */

// C dependencies supplied by the surrounding kernel translation.

static mut IMMEDIATE_UNDOCK: bool = true;

#[repr(C)]
struct DockStation {
    handle: acpi_handle,
    last_dock_time: c_ulong,
    flags: u32,
    dependent_devices: list_head,
    sibling: list_head,
    dock_device: *mut platform_device,
}

static mut DOCK_STATIONS: list_head = list_head { __list: core::ptr::null_mut() };
static mut DOCK_STATION_COUNT: c_int = 0;

#[repr(C)]
struct DockDependentDevice {
    list: list_head,
    adev: *mut acpi_device,
}

const DOCK_DOCKING: u32 = 0x00000001;
const DOCK_UNDOCKING: u32 = 0x00000002;
const DOCK_IS_DOCK: u32 = 0x00000010;
const DOCK_IS_ATA: u32 = 0x00000020;
const DOCK_IS_BAT: u32 = 0x00000040;
const DOCK_EVENT: c_int = 3;
const UNDOCK_EVENT: c_int = 2;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum DockCallbackType {
    DockCallHandler,
    DockCallFixup,
    DockCallUevent,
}

/* Dock dependent device functions. */
unsafe fn add_dock_dependent_device(ds: *mut DockStation, adev: *mut acpi_device) -> c_int {
    let dd = kzalloc::<DockDependentDevice>(core::mem::size_of::<DockDependentDevice>());
    if dd.is_null() { return -ENOMEM; }
    (*dd).adev = adev;
    INIT_LIST_HEAD(&mut (*dd).list);
    list_add_tail(&mut (*dd).list, &mut (*ds).dependent_devices);
    0
}

unsafe fn dock_hotplug_event(dd: *mut DockDependentDevice, event: u32, cb_type: DockCallbackType) {
    let adev = (*dd).adev;
    let mut fixup: acpi_hp_fixup = None;
    let mut uevent: acpi_hp_uevent = None;
    let mut notify: acpi_hp_notify = None;
    acpi_lock_hp_context();
    if !(*adev).hp.is_null() {
        if cb_type == DockCallbackType::DockCallFixup { fixup = (*(*adev).hp).fixup; }
        else if cb_type == DockCallbackType::DockCallUevent { uevent = (*(*adev).hp).uevent; }
        else { notify = (*(*adev).hp).notify; }
    }
    acpi_unlock_hp_context();
    if let Some(f) = fixup { f(adev); }
    else if let Some(f) = uevent { f(adev, event); }
    else if let Some(f) = notify { f(adev, event); }
}

unsafe fn find_dock_station(handle: acpi_handle) -> *mut DockStation {
    let mut ds: *mut DockStation = core::ptr::null_mut();
    list_for_each_entry!(ds, &mut DOCK_STATIONS, sibling, {
        if (*ds).handle == handle { return ds; }
    });
    core::ptr::null_mut()
}

unsafe fn find_dock_dependent_device(ds: *mut DockStation, adev: *mut acpi_device) -> *mut DockDependentDevice {
    let mut dd: *mut DockDependentDevice = core::ptr::null_mut();
    list_for_each_entry!(dd, &mut (*ds).dependent_devices, list, {
        if (*dd).adev == adev { return dd; }
    });
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn register_dock_dependent_device(adev: *mut acpi_device, dshandle: acpi_handle) {
    let ds = find_dock_station(dshandle);
    if !ds.is_null() && find_dock_dependent_device(ds, adev).is_null() { add_dock_dependent_device(ds, adev); }
}

#[no_mangle]
pub unsafe extern "C" fn is_dock_device(adev: *mut acpi_device) -> c_int {
    if DOCK_STATION_COUNT == 0 { return 0; }
    if acpi_dock_match((*adev).handle) { return 1; }
    let mut ds: *mut DockStation = core::ptr::null_mut();
    list_for_each_entry!(ds, &mut DOCK_STATIONS, sibling, {
        if !find_dock_dependent_device(ds, adev).is_null() { return 1; }
    });
    0
}

unsafe fn dock_present(ds: *mut DockStation) -> c_int {
    let mut sta = 0u64;
    if !ds.is_null() && ACPI_SUCCESS(acpi_evaluate_integer((*ds).handle, b"_STA\0".as_ptr() as _, core::ptr::null_mut(), &mut sta)) && sta != 0 { return 1; }
    0
}

unsafe fn hot_remove_dock_devices(ds: *mut DockStation) {
    let mut dd: *mut DockDependentDevice = core::ptr::null_mut();
    list_for_each_entry_reverse!(dd, &mut (*ds).dependent_devices, list, { dock_hotplug_event(dd, ACPI_NOTIFY_EJECT_REQUEST, DockCallbackType::DockCallHandler); });
    list_for_each_entry_reverse!(dd, &mut (*ds).dependent_devices, list, { acpi_bus_trim((*dd).adev); });
}

unsafe fn hotplug_dock_devices(ds: *mut DockStation, event: u32) {
    let mut dd: *mut DockDependentDevice = core::ptr::null_mut();
    list_for_each_entry!(dd, &mut (*ds).dependent_devices, list, { dock_hotplug_event(dd, event, DockCallbackType::DockCallFixup); });
    list_for_each_entry!(dd, &mut (*ds).dependent_devices, list, { dock_hotplug_event(dd, event, DockCallbackType::DockCallHandler); });
    list_for_each_entry!(dd, &mut (*ds).dependent_devices, list, {
        let adev = (*dd).adev;
        if !acpi_device_enumerated(adev) {
            let ret = acpi_bus_scan((*adev).handle);
            if ret != 0 { dev_dbg!(&(*adev).dev, "scan error {}\n", -ret); }
        }
    });
}

unsafe fn dock_event(ds: *mut DockStation, event: u32, num: c_int) {
    let dev = &mut (*(*ds).dock_device).dev;
    let mut event_string = [0i8; 13];
    let mut envp = [event_string.as_mut_ptr(), core::ptr::null_mut()];
    if num == UNDOCK_EVENT { sprintf!(event_string.as_mut_ptr(), "EVENT=undock"); } else { sprintf!(event_string.as_mut_ptr(), "EVENT=dock"); }
    if num == DOCK_EVENT { kobject_uevent_env(&mut dev.kobj, KOBJ_CHANGE, envp.as_mut_ptr()); }
    let mut dd: *mut DockDependentDevice = core::ptr::null_mut();
    list_for_each_entry!(dd, &mut (*ds).dependent_devices, list, { dock_hotplug_event(dd, event, DockCallbackType::DockCallUevent); });
    if num != DOCK_EVENT { kobject_uevent_env(&mut dev.kobj, KOBJ_CHANGE, envp.as_mut_ptr()); }
}

unsafe fn handle_dock(ds: *mut DockStation, dock_value: c_int) {
    acpi_handle_info!((*ds).handle, "{}\n", if dock_value != 0 { "docking" } else { "undocking" });
    let mut arg = acpi_object { type_: ACPI_TYPE_INTEGER, integer: acpi_integer { value: dock_value as u64 } };
    let args = acpi_object_list { count: 1, pointer: &mut arg };
    let mut value = 0u64;
    let status = acpi_evaluate_integer((*ds).handle, b"_DCK\0".as_ptr() as _, &args, &mut value);
    if ACPI_FAILURE(status) && status != AE_NOT_FOUND { acpi_handle_err!((*ds).handle, "Failed to execute _DCK (0x{:x})\n", status); }
}
unsafe fn dock(ds: *mut DockStation) { handle_dock(ds, 1); }
unsafe fn undock(ds: *mut DockStation) { handle_dock(ds, 0); }
unsafe fn begin_dock(ds: *mut DockStation) { (*ds).flags |= DOCK_DOCKING; }
unsafe fn complete_dock(ds: *mut DockStation) { (*ds).flags &= !DOCK_DOCKING; (*ds).last_dock_time = jiffies; }
unsafe fn begin_undock(ds: *mut DockStation) { (*ds).flags |= DOCK_UNDOCKING; }
unsafe fn complete_undock(ds: *mut DockStation) { (*ds).flags &= !DOCK_UNDOCKING; }
unsafe fn dock_in_progress(ds: *mut DockStation) -> c_int { if ((*ds).flags & DOCK_DOCKING) != 0 || time_before(jiffies, (*ds).last_dock_time.wrapping_add(HZ)) { 1 } else { 0 } }

unsafe fn handle_eject_request(ds: *mut DockStation, event: u32) -> c_int {
    if dock_in_progress(ds) != 0 { return -EBUSY; }
    dock_event(ds, event, UNDOCK_EVENT); hot_remove_dock_devices(ds); undock(ds);
    acpi_evaluate_lck((*ds).handle, 0); acpi_evaluate_ej0((*ds).handle);
    if dock_present(ds) != 0 { acpi_handle_err!((*ds).handle, "Unable to undock!\n"); return -EBUSY; }
    complete_undock(ds); 0
}

#[no_mangle]
pub unsafe extern "C" fn dock_notify(adev: *mut acpi_device, mut event: u32) -> c_int {
    let handle = (*adev).handle; let ds = find_dock_station(handle); let mut surprise_removal = false;
    if ds.is_null() { return -ENODEV; }
    if ((*ds).flags & DOCK_IS_DOCK) != 0 && event == ACPI_NOTIFY_DEVICE_CHECK { event = ACPI_NOTIFY_EJECT_REQUEST; }
    match event {
        ACPI_NOTIFY_BUS_CHECK | ACPI_NOTIFY_DEVICE_CHECK => {
            if dock_in_progress(ds) == 0 && !acpi_device_enumerated(adev) {
                begin_dock(ds); dock(ds);
                if dock_present(ds) == 0 { acpi_handle_err!(handle, "Unable to dock!\n"); complete_dock(ds); return 0; }
                hotplug_dock_devices(ds, event); complete_dock(ds); dock_event(ds, event, DOCK_EVENT);
                acpi_evaluate_lck((*ds).handle, 1); acpi_update_all_gpes(); return 0;
            }
            if dock_present(ds) != 0 || dock_in_progress(ds) != 0 { return 0; }
            surprise_removal = true; event = ACPI_NOTIFY_EJECT_REQUEST;
            /* fall through */
            begin_undock(ds);
            if (IMMEDIATE_UNDOCK && ((*ds).flags & DOCK_IS_ATA) == 0 || surprise_removal) { handle_eject_request(ds, event); } else { dock_event(ds, event, UNDOCK_EVENT); }
        }
        ACPI_NOTIFY_EJECT_REQUEST => { begin_undock(ds); if (IMMEDIATE_UNDOCK && ((*ds).flags & DOCK_IS_ATA) == 0 || surprise_removal) { handle_eject_request(ds, event); } else { dock_event(ds, event, UNDOCK_EVENT); } }
        _ => {}
    }
    0
}

/* Sysfs handlers and device registration. */
unsafe fn docked_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> isize { let ds = (*dev).platform_data as *mut DockStation; let adev = acpi_fetch_acpi_dev((*ds).handle); sysfs_emit!(buf, "{}\n", acpi_device_enumerated(adev)) }
unsafe fn flags_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> isize { let ds = (*dev).platform_data as *mut DockStation; sysfs_emit!(buf, "{}\n", (*ds).flags) }
unsafe fn undock_store(dev: *mut device, _attr: *mut device_attribute, _buf: *const c_char, count: usize) -> isize { if count == 0 { return -EINVAL as isize; } let ds = (*dev).platform_data as *mut DockStation; acpi_scan_lock_acquire(); begin_undock(ds); let ret = handle_eject_request(ds, ACPI_NOTIFY_EJECT_REQUEST); acpi_scan_lock_release(); if ret != 0 { ret as isize } else { count as isize } }
unsafe fn uid_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> isize { let ds = (*dev).platform_data as *mut DockStation; let mut value = 0u64; if ACPI_FAILURE(acpi_evaluate_integer((*ds).handle, b"_UID\0".as_ptr() as _, core::ptr::null_mut(), &mut value)) { return 0; } sysfs_emit!(buf, "{:x}\n", value) }
unsafe fn type_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> isize { let ds = (*dev).platform_data as *mut DockStation; let value = if (*ds).flags & DOCK_IS_DOCK != 0 { "dock_station" } else if (*ds).flags & DOCK_IS_ATA != 0 { "ata_bay" } else if (*ds).flags & DOCK_IS_BAT != 0 { "battery_bay" } else { "unknown" }; sysfs_emit!(buf, "{}\n", value) }

#[no_mangle]
pub unsafe extern "C" fn acpi_dock_add(adev: *mut acpi_device) {
    let mut ds = DockStation { handle: core::ptr::null_mut(), last_dock_time: 0, flags: 0, dependent_devices: core::mem::zeroed(), sibling: core::mem::zeroed(), dock_device: core::ptr::null_mut() };
    let mut info: platform_device_info = core::mem::zeroed(); info.name = b"dock\0".as_ptr() as _; info.id = DOCK_STATION_COUNT; info.fwnode = acpi_fwnode_handle(adev); info.data = &mut ds as *mut _ as _; info.size_data = core::mem::size_of::<DockStation>();
    let dd = platform_device_register_full(&info); if IS_ERR(dd) { return; }
    let station = (*dd).dev.platform_data as *mut DockStation; (*station).handle = (*adev).handle; (*station).dock_device = dd; (*station).last_dock_time = jiffies.wrapping_sub(HZ); INIT_LIST_HEAD(&mut (*station).sibling); INIT_LIST_HEAD(&mut (*station).dependent_devices); dev_set_uevent_suppress(&mut (*dd).dev, false);
    if acpi_dock_match((*station).handle) { (*station).flags |= DOCK_IS_DOCK; } if acpi_ata_match((*station).handle) { (*station).flags |= DOCK_IS_ATA; } if acpi_device_is_battery(adev) { (*station).flags |= DOCK_IS_BAT; }
    let ret = sysfs_create_group(&mut (*dd).dev.kobj, &dock_attribute_group); if ret != 0 { platform_device_unregister(dd); acpi_handle_err!((*station).handle, "{} encountered error {}\n", "acpi_dock_add", ret); return; }
    if add_dock_dependent_device(station, adev) != 0 { sysfs_remove_group(&mut (*dd).dev.kobj, &dock_attribute_group); platform_device_unregister(dd); return; }
    DOCK_STATION_COUNT += 1; list_add(&mut (*station).sibling, &mut DOCK_STATIONS); (*adev).flags.is_dock_station = true; dev_info!(&(*adev).dev, "ACPI dock station (docks/bays count: {})\n", DOCK_STATION_COUNT);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
