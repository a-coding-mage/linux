// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * container.c  - ACPI Generic Container Driver
 *
 * Copyright (C) 2004 Anil S Keshavamurthy (anil.s.keshavamurthy@intel.com)
 * Copyright (C) 2004 Keiichiro Tokunaga (tokunaga.keiich@jp.fujitsu.com)
 * Copyright (C) 2004 Motoyuki Ito (motoyuki@soft.fujitsu.com)
 * Copyright (C) 2004 FUJITSU LIMITED
 * Copyright (C) 2004, 2013 Intel Corp.
 * Author: Rafael J. Wysocki <rafael.j.wysocki@intel.com>
 */

// Dependencies supplied by the surrounding ACPI/container implementation.

static CONTAINER_DEVICE_IDS: [acpi_device_id; 4] = [
    acpi_device_id { name: "ACPI0004", driver_data: 0 },
    acpi_device_id { name: "PNP0A05", driver_data: 0 },
    acpi_device_id { name: "PNP0A06", driver_data: 0 },
    acpi_device_id { name: "", driver_data: 0 },
];

#[cfg(CONFIG_ACPI_CONTAINER)]
unsafe fn check_offline(adev: *mut acpi_device, _not_used: *mut core::ffi::c_void) -> i32 {
    if acpi_scan_is_offline(adev, false) {
        return 0;
    }

    -EBUSY
}

#[cfg(CONFIG_ACPI_CONTAINER)]
unsafe fn acpi_container_offline(cdev: *mut container_dev) -> i32 {
    /* Check all of the dependent devices' physical companions. */
    acpi_dev_for_each_child(
        ACPI_COMPANION(&(*cdev).dev),
        Some(check_offline),
        core::ptr::null_mut(),
    )
}

#[cfg(CONFIG_ACPI_CONTAINER)]
unsafe fn acpi_container_release(dev: *mut device) {
    kfree(to_container_dev(dev));
}

#[cfg(CONFIG_ACPI_CONTAINER)]
unsafe fn container_device_attach(
    adev: *mut acpi_device,
    _not_used: *const acpi_device_id,
) -> i32 {
    let cdev: *mut container_dev;
    let dev: *mut device;
    let ret: i32;

    if (*adev).flags.is_dock_station {
        return 0;
    }

    cdev = kzalloc_obj::<container_dev>();
    if cdev.is_null() {
        return -ENOMEM;
    }

    (*cdev).offline = Some(acpi_container_offline);
    dev = &mut (*cdev).dev;
    (*dev).bus = &mut container_subsys;
    dev_set_name(dev, "%s", dev_name(&(*adev).dev));
    ACPI_COMPANION_SET(dev, adev);
    (*dev).release = Some(acpi_container_release);
    ret = device_register(dev);
    if ret != 0 {
        put_device(dev);
        return ret;
    }
    (*adev).driver_data = dev;
    1
}

#[cfg(CONFIG_ACPI_CONTAINER)]
unsafe fn container_device_detach(adev: *mut acpi_device) {
    let dev: *mut device = acpi_driver_data(adev);

    (*adev).driver_data = core::ptr::null_mut();
    if !dev.is_null() {
        device_unregister(dev);
    }
}

#[cfg(CONFIG_ACPI_CONTAINER)]
unsafe fn container_device_online(adev: *mut acpi_device) {
    let dev: *mut device = acpi_driver_data(adev);

    kobject_uevent(&mut (*dev).kobj, KOBJ_ONLINE);
}

#[cfg(CONFIG_ACPI_CONTAINER)]
static mut CONTAINER_HANDLER: acpi_scan_handler = acpi_scan_handler {
    ids: CONTAINER_DEVICE_IDS.as_ptr(),
    attach: Some(container_device_attach),
    detach: Some(container_device_detach),
    hotplug: acpi_scan_handler_hotplug {
        enabled: true,
        demand_offline: true,
        notify_online: Some(container_device_online),
    },
};

#[cfg(CONFIG_ACPI_CONTAINER)]
unsafe fn acpi_container_init() {
    acpi_scan_add_handler(&mut CONTAINER_HANDLER);
}

#[cfg(not(CONFIG_ACPI_CONTAINER))]
static mut CONTAINER_HANDLER: acpi_scan_handler = acpi_scan_handler {
    ids: CONTAINER_DEVICE_IDS.as_ptr(),
};

#[cfg(not(CONFIG_ACPI_CONTAINER))]
unsafe fn acpi_container_init() {
    acpi_scan_add_handler_with_hotplug(&mut CONTAINER_HANDLER, "container");
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
