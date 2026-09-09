// SPDX-License-Identifier: GPL-2.0-only
/*
 * Generic Event Device for ACPI.
 *
 * Copyright (c) 2016, The Linux Foundation. All rights reserved.
 *
 * Generic Event Device allows platforms to handle interrupts in ACPI
 * ASL statements. It follows very similar to _EVT method approach
 * from GPIO events. All interrupts are listed in _CRS and the handler
 * is written in _EVT method.
 */

// Linux kernel headers supplying the following types, constants, and functions
// are intentionally external dependencies of this translation.

const MODULE_NAME: &[u8] = b"acpi-ged\0";

#[repr(C)]
struct AcpiGedDevice {
    dev: *mut device,
    event_list: list_head,
}

#[repr(C)]
struct AcpiGedEvent {
    node: list_head,
    dev: *mut device,
    gsi: u32,
    irq: u32,
    handle: acpi_handle,
}

unsafe extern "C" {
    fn acpi_execute_simple_method(handle: acpi_handle, pathname: *const core::ffi::c_void, arg: u32) -> acpi_status;
    fn acpi_dev_resource_interrupt(ares: *mut acpi_resource, index: u32, resource: *mut resource) -> bool;
    fn acpi_get_handle(handle: acpi_handle, name: *const i8, ret: *mut acpi_handle) -> acpi_status;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn request_threaded_irq(irq: u32, handler: *const core::ffi::c_void, thread_fn: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t, flags: u32, name: *const i8, data: *mut core::ffi::c_void) -> i32;
    fn free_irq(irq: u32, dev_id: *mut core::ffi::c_void);
    fn acpi_walk_resources(handle: acpi_handle, pathname: *const i8, user_function: unsafe extern "C" fn(*mut acpi_resource, *mut core::ffi::c_void) -> acpi_status, context: *mut core::ffi::c_void) -> acpi_status;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut core::ffi::c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut core::ffi::c_void;
    fn init_list_head(list: *mut list_head);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
}

unsafe extern "C" fn acpi_ged_irq_handler(_irq: i32, data: *mut core::ffi::c_void) -> irqreturn_t {
    let event = data as *mut AcpiGedEvent;
    let acpi_ret = acpi_execute_simple_method((*event).handle, core::ptr::null(), (*event).gsi);
    if acpi_failure(acpi_ret) {
        dev_err_once((*event).dev, b"IRQ method execution failed\n\0".as_ptr() as *const i8);
    }
    IRQ_HANDLED
}

unsafe extern "C" fn acpi_ged_request_interrupt(ares: *mut acpi_resource, context: *mut core::ffi::c_void) -> acpi_status {
    let geddev = context as *mut AcpiGedDevice;
    let dev = (*geddev).dev;
    let handle = acpi_handle_of(dev);
    let mut evt_handle: acpi_handle = core::ptr::null_mut();
    let mut r: resource = core::mem::zeroed();
    let mut ev_name = [0i8; 5];
    let (gsi, trigger) = if (*ares).type_ == ACPI_RESOURCE_TYPE_IRQ {
        ((*ares).data.irq.interrupts[0], (*ares).data.irq.triggering)
    } else {
        ((*ares).data.extended_irq.interrupts[0], (*ares).data.extended_irq.triggering)
    };

    if (*ares).type_ == ACPI_RESOURCE_TYPE_END_TAG { return AE_OK; }
    if !acpi_dev_resource_interrupt(ares, 0, &mut r) {
        dev_err(dev, b"unable to parse IRQ resource\n\0".as_ptr() as *const i8);
        return AE_ERROR;
    }
    let irq = r.start;
    match gsi {
        0..=255 => {
            ev_name[0] = b'_' as i8;
            ev_name[1] = if trigger == ACPI_EDGE_SENSITIVE { b'E' } else { b'L' } as i8;
            sprintf_hex2(&mut ev_name[2], gsi);
            if acpi_success(acpi_get_handle(handle, ev_name.as_ptr(), &mut evt_handle)) { }
            else if !acpi_success(acpi_get_handle(handle, b"_EVT\0".as_ptr() as *const i8, &mut evt_handle)) {
                dev_err(dev, b"cannot locate _EVT method\n\0".as_ptr() as *const i8);
                return AE_ERROR;
            }
        }
        _ => {
            if !acpi_success(acpi_get_handle(handle, b"_EVT\0".as_ptr() as *const i8, &mut evt_handle)) {
                dev_err(dev, b"cannot locate _EVT method\n\0".as_ptr() as *const i8);
                return AE_ERROR;
            }
        }
    }
    let event = devm_kzalloc(dev, core::mem::size_of::<AcpiGedEvent>(), GFP_KERNEL) as *mut AcpiGedEvent;
    if event.is_null() { return AE_ERROR; }
    (*event).gsi = gsi; (*event).dev = dev; (*event).irq = irq; (*event).handle = evt_handle;
    let mut irqflags = IRQF_ONESHOT;
    if r.flags & IORESOURCE_IRQ_SHAREABLE != 0 { irqflags |= IRQF_SHARED; }
    if request_threaded_irq(irq, core::ptr::null(), acpi_ged_irq_handler, irqflags, b"ACPI:Ged\0".as_ptr() as *const i8, event as *mut core::ffi::c_void) != 0 {
        dev_err(dev, b"failed to setup event handler for irq %u\n\0".as_ptr() as *const i8, irq);
        return AE_ERROR;
    }
    dev_dbg(dev, b"GED listening GSI %u @ IRQ %u\n\0".as_ptr() as *const i8, gsi, irq);
    list_add_tail(&mut (*event).node, &mut (*geddev).event_list);
    AE_OK
}

unsafe extern "C" fn ged_probe(pdev: *mut platform_device) -> i32 {
    let geddev = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<AcpiGedDevice>(), GFP_KERNEL) as *mut AcpiGedDevice;
    if geddev.is_null() { return -12; }
    (*geddev).dev = &mut (*pdev).dev;
    init_list_head(&mut (*geddev).event_list);
    if acpi_failure(acpi_walk_resources(acpi_handle_of((*geddev).dev), b"_CRS\0".as_ptr() as *const i8, acpi_ged_request_interrupt, geddev as *mut core::ffi::c_void)) {
        dev_err((*geddev).dev, b"unable to parse the _CRS record\n\0".as_ptr() as *const i8);
        return -22;
    }
    platform_set_drvdata(pdev, geddev as *mut core::ffi::c_void);
    0
}

unsafe extern "C" fn ged_shutdown(pdev: *mut platform_device) {
    let geddev = platform_get_drvdata(pdev) as *mut AcpiGedDevice;
    // Equivalent to list_for_each_entry_safe; list topology is supplied externally.
    let mut event = list_first_entry(&mut (*geddev).event_list) as *mut AcpiGedEvent;
    while !event.is_null() {
        let next = list_next_entry(event, &(*geddev).event_list);
        free_irq((*event).irq, event as *mut core::ffi::c_void);
        list_del(&mut (*event).node);
        dev_dbg((*geddev).dev, b"GED releasing GSI %u @ IRQ %u\n\0".as_ptr() as *const i8, (*event).gsi, (*event).irq);
        event = next;
    }
}

unsafe extern "C" fn ged_remove(pdev: *mut platform_device) { ged_shutdown(pdev); }

// The ACPI ID table and platform-driver registration retain the C driver's external ABI.
static GED_ACPI_IDS: &[&[u8]] = &[b"ACPI0013\0", b"\0"];

// Registration is performed by the platform-driver integration supplied by the kernel.
register_builtin_platform_driver!(ged_driver);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
