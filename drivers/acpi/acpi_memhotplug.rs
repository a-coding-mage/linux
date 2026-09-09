// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2004, 2013 Intel Corporation
 * Author: Naveen B S <naveen.b.s@intel.com>
 * Author: Rafael J. Wysocki <rafael.j.wysocki@intel.com>
 *
 * All rights reserved.
 *
 * ACPI based HotPlug driver that supports Memory Hotplug
 * This driver fields notifications from firmware for memory add
 * and remove operations and alerts the VM of the affected memory
 * ranges.
 */

// Dependencies supplied by the Linux kernel bindings.

const ACPI_MEMORY_DEVICE_HID: &str = "PNP0C80";

static MEMORY_DEVICE_IDS: [acpi_device_id; 2] = [
    acpi_device_id { hid: ACPI_MEMORY_DEVICE_HID, driver_data: 0 },
    acpi_device_id { hid: "", driver_data: 0 },
];

#[cfg(CONFIG_ACPI_HOTPLUG_MEMORY)]
static mut MEMORY_DEVICE_HANDLER: acpi_scan_handler = acpi_scan_handler {
    ids: &MEMORY_DEVICE_IDS,
    attach: Some(acpi_memory_device_add),
    detach: Some(acpi_memory_device_remove),
    hotplug: acpi_scan_hotplug { enabled: true },
};

#[cfg(CONFIG_ACPI_HOTPLUG_MEMORY)]
#[repr(C)]
struct acpi_memory_info {
    list: list_head,
    start_addr: u64,
    length: u64,
    caching: u16,
    write_protect: u16,
    enabled: bool,
}

#[cfg(CONFIG_ACPI_HOTPLUG_MEMORY)]
#[repr(C)]
struct acpi_memory_device {
    device: *mut acpi_device,
    res_list: list_head,
    mgid: c_int,
}

#[cfg(CONFIG_ACPI_HOTPLUG_MEMORY)]
unsafe fn acpi_memory_get_resource(resource: *mut acpi_resource, context: *mut c_void) -> acpi_status {
    let mem_device = context as *mut acpi_memory_device;
    let mut address64 = acpi_resource_address64::default();
    let status = acpi_resource_to_address64(resource, &mut address64);
    if ACPI_FAILURE(status) || address64.resource_type != ACPI_MEMORY_RANGE {
        return AE_OK;
    }

    list_for_each_entry!(info, (*mem_device).res_list, list, {
        if (*info).caching == address64.info.mem.caching
            && (*info).write_protect == address64.info.mem.write_protect
            && (*info).start_addr.wrapping_add((*info).length) == address64.address.minimum
        {
            (*info).length = (*info).length.wrapping_add(address64.address.address_length);
            return AE_OK;
        }
    });

    let new = kzalloc_obj::<acpi_memory_info>();
    if new.is_null() { return AE_ERROR; }
    INIT_LIST_HEAD!(&mut (*new).list);
    (*new).caching = address64.info.mem.caching;
    (*new).write_protect = address64.info.mem.write_protect;
    (*new).start_addr = address64.address.minimum;
    (*new).length = address64.address.address_length;
    list_add_tail!(&mut (*new).list, &mut (*mem_device).res_list);
    AE_OK
}

#[cfg(CONFIG_ACPI_HOTPLUG_MEMORY)]
unsafe fn acpi_memory_free_device_resources(mem_device: *mut acpi_memory_device) {
    list_for_each_entry_safe!(info, n, (*mem_device).res_list, list, { kfree(info); });
    INIT_LIST_HEAD!(&mut (*mem_device).res_list);
}

#[cfg(CONFIG_ACPI_HOTPLUG_MEMORY)]
unsafe fn acpi_memory_get_device_resources(mem_device: *mut acpi_memory_device) -> c_int {
    if !list_empty!(&(*mem_device).res_list) { return 0; }
    let status = acpi_walk_resources((*(*mem_device).device).handle, METHOD_NAME__CRS,
        acpi_memory_get_resource, mem_device as *mut c_void);
    if ACPI_FAILURE(status) {
        acpi_memory_free_device_resources(mem_device);
        return -EINVAL;
    }
    0
}

#[cfg(CONFIG_ACPI_HOTPLUG_MEMORY)]
unsafe fn acpi_memory_check_device(mem_device: *mut acpi_memory_device) -> c_int {
    let mut current_status: u64 = 0;
    if ACPI_FAILURE(acpi_evaluate_integer((*(*mem_device).device).handle, METHOD_NAME__STA,
                                          core::ptr::null_mut(), &mut current_status)) { return -ENODEV; }
    if (current_status & ACPI_STA_DEVICE_PRESENT) == 0
        || (current_status & ACPI_STA_DEVICE_ENABLED) == 0
        || (current_status & ACPI_STA_DEVICE_FUNCTIONING) == 0 { return -ENODEV; }
    0
}

#[cfg(CONFIG_ACPI_HOTPLUG_MEMORY)]
unsafe fn acpi_bind_memblk(mem: *mut memory_block, arg: *mut c_void) -> c_int { acpi_bind_one(&mut (*mem).dev, arg) }

#[cfg(CONFIG_ACPI_HOTPLUG_MEMORY)]
unsafe fn acpi_bind_memory_blocks(info: *mut acpi_memory_info, adev: *mut acpi_device) -> c_int {
    walk_memory_blocks((*info).start_addr, (*info).length, adev as *mut c_void, acpi_bind_memblk)
}

#[cfg(CONFIG_ACPI_HOTPLUG_MEMORY)]
unsafe fn acpi_unbind_memblk(mem: *mut memory_block, _arg: *mut c_void) -> c_int { acpi_unbind_one(&mut (*mem).dev); 0 }

#[cfg(CONFIG_ACPI_HOTPLUG_MEMORY)]
unsafe fn acpi_unbind_memory_blocks(info: *mut acpi_memory_info) {
    walk_memory_blocks((*info).start_addr, (*info).length, core::ptr::null_mut(), acpi_unbind_memblk);
}

#[cfg(CONFIG_ACPI_HOTPLUG_MEMORY)]
unsafe fn acpi_memory_enable_device(mem_device: *mut acpi_memory_device) -> c_int {
    let handle = (*(*mem_device).device).handle;
    let mut mhp_flags = MHP_NID_IS_MGID;
    let mut result;
    let mut num_enabled = 0;
    let mut total_length = 0u64;
    let mut node = acpi_get_node(handle);
    list_for_each_entry!(info, (*mem_device).res_list, list, {
        if (*info).length == 0 { continue; }
        if node < 0 { node = memory_add_physaddr_to_nid((*info).start_addr); }
        total_length = total_length.wrapping_add((*info).length);
    });
    if total_length == 0 { dev_err!(&(*(*mem_device).device).dev, "device is empty\n"); return -EINVAL; }
    let mgid = memory_group_register_static(node, PFN_UP(total_length));
    if mgid < 0 { return mgid; }
    (*mem_device).mgid = mgid;
    list_for_each_entry!(info, (*mem_device).res_list, list, {
        if (*info).length == 0 { continue; }
        mhp_flags |= MHP_MEMMAP_ON_MEMORY;
        result = __add_memory(mgid, (*info).start_addr, (*info).length, mhp_flags);
        if result != 0 && result != -EEXIST { continue; }
        result = acpi_bind_memory_blocks(info, (*mem_device).device);
        if result != 0 { acpi_unbind_memory_blocks(info); return -ENODEV; }
        (*info).enabled = true;
        num_enabled += 1;
    });
    if num_enabled == 0 { dev_err!(&(*(*mem_device).device).dev, "add_memory failed\n"); return -EINVAL; }
    0
}

#[cfg(CONFIG_ACPI_HOTPLUG_MEMORY)]
unsafe fn acpi_memory_remove_memory(mem_device: *mut acpi_memory_device) {
    list_for_each_entry_safe!(info, n, (*mem_device).res_list, list, {
        if !(*info).enabled { continue; }
        acpi_unbind_memory_blocks(info);
        __remove_memory((*info).start_addr, (*info).length);
        list_del!(&mut (*info).list);
        kfree(info);
    });
}

#[cfg(CONFIG_ACPI_HOTPLUG_MEMORY)]
unsafe fn acpi_memory_device_free(mem_device: *mut acpi_memory_device) {
    if mem_device.is_null() { return; }
    if (*mem_device).mgid >= 0 { memory_group_unregister((*mem_device).mgid); }
    acpi_memory_free_device_resources(mem_device);
    (*(*mem_device).device).driver_data = core::ptr::null_mut();
    kfree(mem_device);
}

#[cfg(CONFIG_ACPI_HOTPLUG_MEMORY)]
unsafe fn acpi_memory_device_add(device: *mut acpi_device, _not_used: *const acpi_device_id) -> c_int {
    if device.is_null() { return -EINVAL; }
    let mem_device = kzalloc_obj::<acpi_memory_device>();
    if mem_device.is_null() { return -ENOMEM; }
    INIT_LIST_HEAD!(&mut (*mem_device).res_list);
    (*mem_device).device = device;
    (*mem_device).mgid = -1;
    (*device).driver_data = mem_device as *mut c_void;
    let mut result = acpi_memory_get_device_resources(mem_device);
    if result != 0 { (*device).driver_data = core::ptr::null_mut(); kfree(mem_device); return result; }
    result = acpi_memory_check_device(mem_device);
    if result != 0 { acpi_memory_device_free(mem_device); return 0; }
    result = acpi_memory_enable_device(mem_device);
    if result != 0 { dev_err!(&(*device).dev, "acpi_memory_enable_device() error\n"); acpi_memory_device_free(mem_device); return result; }
    dev_dbg!(&(*device).dev, "Memory device configured by ACPI\n");
    1
}

#[cfg(CONFIG_ACPI_HOTPLUG_MEMORY)]
unsafe fn acpi_memory_device_remove(device: *mut acpi_device) {
    if device.is_null() || acpi_driver_data(device).is_null() { return; }
    let mem_device = acpi_driver_data(device) as *mut acpi_memory_device;
    acpi_memory_remove_memory(mem_device);
    acpi_memory_device_free(mem_device);
}

#[cfg(CONFIG_ACPI_HOTPLUG_MEMORY)]
static mut ACPI_NO_MEMHOTPLUG: bool = false;

#[cfg(CONFIG_ACPI_HOTPLUG_MEMORY)]
unsafe fn acpi_memory_hotplug_init() {
    if ACPI_NO_MEMHOTPLUG {
        MEMORY_DEVICE_HANDLER.attach = None;
        acpi_scan_add_handler(&mut MEMORY_DEVICE_HANDLER);
        return;
    }
    acpi_scan_add_handler_with_hotplug(&mut MEMORY_DEVICE_HANDLER, "memory");
}

#[cfg(CONFIG_ACPI_HOTPLUG_MEMORY)]
unsafe fn disable_acpi_memory_hotplug(_str: *mut c_char) -> c_int { ACPI_NO_MEMHOTPLUG = true; 1 }

#[cfg(not(CONFIG_ACPI_HOTPLUG_MEMORY))]
static mut MEMORY_DEVICE_HANDLER: acpi_scan_handler = acpi_scan_handler { ids: &MEMORY_DEVICE_IDS };

#[cfg(not(CONFIG_ACPI_HOTPLUG_MEMORY))]
unsafe fn acpi_memory_hotplug_init() { acpi_scan_add_handler(&mut MEMORY_DEVICE_HANDLER); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
