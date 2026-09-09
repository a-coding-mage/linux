// SPDX-License-Identifier: GPL-2.0-only
/*
 * ACPI support for platform bus type.
 *
 * Copyright (C) 2012, Intel Corporation
 * Authors: Mika Westerberg <mika.westerberg@linux.intel.com>
 *          Mathias Nyman <mathias.nyman@linux.intel.com>
 *          Rafael J. Wysocki <rafael.j.wysocki@intel.com>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented in this file.

const ACPI_ALLOW_WO_RESOURCES: u64 = 1u64 << 0;

static FORBIDDEN_ID_LIST: [acpi_device_id; 7] = [
    acpi_device_id { id: "ACPI0009", driver_data: 0 }, // IOxAPIC
    acpi_device_id { id: "ACPI000A", driver_data: 0 }, // IOAPIC
    acpi_device_id { id: "PNP0000", driver_data: 0 },  // PIC
    acpi_device_id { id: "PNP0100", driver_data: 0 },  // Timer
    acpi_device_id { id: "PNP0200", driver_data: 0 },  // AT DMA Controller
    acpi_device_id { id: ACPI_SMBUS_MS_HID, driver_data: ACPI_ALLOW_WO_RESOURCES },
    acpi_device_id { id: "", driver_data: 0 },
];

unsafe fn acpi_platform_device_find_by_companion(
    adev: *mut acpi_device,
) -> *mut platform_device {
    let dev = bus_find_device_by_acpi_dev(&platform_bus_type, adev);
    if !dev.is_null() { to_platform_device(dev) } else { core::ptr::null_mut() }
}

unsafe extern "C" fn acpi_platform_device_remove_notify(
    nb: *mut notifier_block,
    value: c_ulong,
    arg: *mut c_void,
) -> c_int {
    let adev = arg as *mut acpi_device;
    let mut pdev: *mut platform_device;

    match value {
        ACPI_RECONFIG_DEVICE_ADD => { /* Nothing to do here */ }
        ACPI_RECONFIG_DEVICE_REMOVE => {
            if !acpi_device_enumerated(adev) { return NOTIFY_OK; }
            pdev = acpi_platform_device_find_by_companion(adev);
            if pdev.is_null() { return NOTIFY_OK; }
            platform_device_unregister(pdev);
            put_device(&mut (*pdev).dev);
        }
        _ => {}
    }
    NOTIFY_OK
}

static mut ACPI_PLATFORM_NOTIFIER: notifier_block = notifier_block {
    notifier_call: Some(acpi_platform_device_remove_notify),
};

unsafe fn acpi_platform_adjust_resources(
    adev: *mut acpi_device,
    new_res: *mut resource,
    resources: *mut resource,
    mut count: c_uint,
) -> c_uint {
    if (*new_res).flags & (IORESOURCE_IO | IORESOURCE_MEM) == 0 { return count; }
    let mut i = 0;
    while i < count {
        let res = resources.add(i as usize);
        if __resource_contains_unbound(res, new_res)
            || __resource_contains_unbound(new_res, res)
            || resource_type(new_res) != resource_type(res)
            || !resource_union(new_res, res, new_res) {
            i += 1;
            continue;
        }
        dev_info(&(*adev).dev, "%pR expanded due to overlap\n", new_res);
        core::ptr::copy(res.add(1), res, (count - i - 1) as usize);
        count -= 1;
    }
    count
}

unsafe fn acpi_platform_fill_resource(
    parent: *mut device,
    src: *const resource,
    dest: *mut resource,
) {
    *dest = *src;
    if !parent.is_null() && dev_is_pci(parent) {
        (*dest).parent = pci_find_resource(to_pci_dev(parent), dest);
    }
}

unsafe extern "C" fn acpi_platform_resource_count(
    ares: *mut acpi_resource,
    data: *mut c_void,
) -> acpi_status {
    *(data as *mut bool) = true;
    AE_CTRL_TERMINATE
}

pub unsafe fn acpi_create_platform_device(
    adev: *mut acpi_device,
    properties: *const property_entry,
) -> *mut platform_device {
    let p = acpi_dev_parent(adev);
    let parent = acpi_bus_get_primary_device(p);
    let mut pdev: *mut platform_device = core::ptr::null_mut();
    let mut pdevinfo: platform_device_info = core::mem::zeroed();
    let mut resources: *mut resource = core::ptr::null_mut();
    let mut count: c_int = 0;

    if (*adev).physical_node_count != 0 && !(*adev).pnp.type_.backlight { return core::ptr::null_mut(); }
    let m = acpi_match_acpi_device(FORBIDDEN_ID_LIST.as_ptr(), adev);
    if !m.is_null() {
        if (*m).driver_data & ACPI_ALLOW_WO_RESOURCES != 0 {
            let mut has_resources = false;
            acpi_walk_resources((*adev).handle, METHOD_NAME__CRS, Some(acpi_platform_resource_count), &mut has_resources as *mut _ as *mut c_void);
            if has_resources { return ERR_PTR(-EINVAL); }
        } else { return ERR_PTR(-EINVAL); }
    }

    if (*adev).device_type == ACPI_BUS_TYPE_DEVICE {
        let mut resource_list: list_head = core::mem::zeroed();
        count = acpi_dev_get_resources(adev, &mut resource_list, None, core::ptr::null_mut());
        if count < 0 { return ERR_PTR(-ENODATA); }
        if count > 0 {
            resources = kzalloc_objs(count as usize);
            if resources.is_null() { acpi_dev_free_resource_list(&mut resource_list); return ERR_PTR(-ENOMEM); }
            count = 0;
            let mut rentry = resource_list_first_entry(&resource_list);
            while !rentry.is_null() {
                count = acpi_platform_adjust_resources(adev, (*rentry).res, resources, count as c_uint) as c_int;
                acpi_platform_fill_resource(parent, (*rentry).res, resources.add(count as usize));
                count += 1;
                rentry = resource_list_next_entry(rentry);
            }
            acpi_dev_free_resource_list(&mut resource_list);
        }
    }

    pdevinfo.parent = parent;
    pdevinfo.name = dev_name(&(*adev).dev);
    pdevinfo.id = PLATFORM_DEVID_NONE;
    pdevinfo.res = resources;
    pdevinfo.num_res = count as c_uint;
    pdevinfo.fwnode = acpi_fwnode_handle(adev);
    pdevinfo.properties = properties;
    pdevinfo.dma_mask = if acpi_dma_supported(adev) { DMA_BIT_MASK(32) } else { 0 };

    pdev = platform_device_register_full(&pdevinfo);
    if IS_ERR(pdev) {
        dev_err(&(*adev).dev, "platform device creation failed: %ld\n", PTR_ERR(pdev));
    } else {
        set_dev_node(&mut (*pdev).dev, acpi_get_node((*adev).handle));
        dev_dbg(&(*adev).dev, "created platform device %s\n", dev_name(&(*pdev).dev));
    }
    kfree(resources);
    pdev
}

pub unsafe fn acpi_platform_init() {
    acpi_reconfig_notifier_register(&mut ACPI_PLATFORM_NOTIFIER);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
