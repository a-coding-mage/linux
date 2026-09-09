/* SPDX-License-Identifier: GPL-2.0 */

/* C header dependencies: <linux/of.h>, <linux/platform_device.h>. */

extern "C" {
    pub static macio_bus_type: bus_type;
}

/* MacIO device driver is defined later. */
pub struct macio_chip;

pub const MACIO_DEV_COUNT_RESOURCES: usize = 8;
pub const MACIO_DEV_COUNT_IRQS: usize = 8;

/*
 * The macio_bus structure is used to describe a "virtual" bus within a
 * MacIO ASIC.  It is typically provided by a macio_pci_asic PCI device,
 * but could be provided differently as well (nubus machines using a fake
 * OF tree).  The pdev field can be NULL on non-PCI machines.
 */
#[repr(C)]
pub struct macio_bus {
    pub chip: *mut macio_chip, /* macio_chip (private use) */
    pub index: ::core::ffi::c_int, /* macio chip index in system */
    /* CONFIG_PCI: pub pdev: *mut pci_dev, */
}

/* The macio_dev structure is used to describe a device within an Apple MacIO ASIC. */
#[repr(C)]
pub struct macio_dev {
    pub bus: *mut macio_bus, /* macio bus this device is on */
    pub media_bay: *mut macio_dev, /* Device is part of a media bay */
    pub ofdev: platform_device,
    pub dma_parms: device_dma_parameters, /* ide needs that */
    pub n_resources: ::core::ffi::c_int,
    pub resource: [resource; MACIO_DEV_COUNT_RESOURCES],
    pub n_interrupts: ::core::ffi::c_int,
    pub interrupt: [resource; MACIO_DEV_COUNT_IRQS],
}

/* C: to_macio_device(d) = container_of(d, struct macio_dev, ofdev.dev) */
/* C: of_to_macio_device(d) = container_of(d, struct macio_dev, ofdev) */

extern "C" {
    pub fn macio_dev_get(dev: *mut macio_dev) -> *mut macio_dev;
    pub fn macio_dev_put(dev: *mut macio_dev);
}

pub unsafe fn macio_resource_count(dev: *mut macio_dev) -> ::core::ffi::c_int {
    (*dev).n_resources
}

pub unsafe fn macio_resource_start(
    dev: *mut macio_dev,
    resource_no: ::core::ffi::c_int,
) -> ::core::ffi::c_ulong {
    (*dev).resource[resource_no as usize].start
}

pub unsafe fn macio_resource_end(
    dev: *mut macio_dev,
    resource_no: ::core::ffi::c_int,
) -> ::core::ffi::c_ulong {
    (*dev).resource[resource_no as usize].end
}

pub unsafe fn macio_resource_len(
    dev: *mut macio_dev,
    resource_no: ::core::ffi::c_int,
) -> ::core::ffi::c_ulong {
    let res = &(*dev).resource[resource_no as usize];
    if res.start == 0 || res.end == 0 || res.end < res.start {
        return 0;
    }
    resource_size(res as *const resource)
}

extern "C" {
    pub fn macio_enable_devres(dev: *mut macio_dev) -> ::core::ffi::c_int;
    pub fn macio_request_resource(
        dev: *mut macio_dev,
        resource_no: ::core::ffi::c_int,
        name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    pub fn macio_release_resource(dev: *mut macio_dev, resource_no: ::core::ffi::c_int);
    pub fn macio_request_resources(
        dev: *mut macio_dev,
        name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    pub fn macio_release_resources(dev: *mut macio_dev);
}

pub unsafe fn macio_irq_count(dev: *mut macio_dev) -> ::core::ffi::c_int {
    (*dev).n_interrupts
}

pub unsafe fn macio_irq(
    dev: *mut macio_dev,
    irq_no: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    (*dev).interrupt[irq_no as usize].start as ::core::ffi::c_int
}

pub unsafe fn macio_set_drvdata(dev: *mut macio_dev, data: *mut ::core::ffi::c_void) {
    dev_set_drvdata(&mut (*dev).ofdev.dev, data);
}

pub unsafe fn macio_get_drvdata(dev: *mut macio_dev) -> *mut ::core::ffi::c_void {
    dev_get_drvdata(&(*dev).ofdev.dev)
}

pub unsafe fn macio_get_of_node(dev: *mut macio_dev) -> *mut device_node {
    (*dev).ofdev.dev.of_node
}

/* CONFIG_PCI: macio_get_pci_dev(mdev) returns mdev->bus->pdev. */

/* A driver for a mac-io chip based device. */
#[repr(C)]
pub struct macio_driver {
    pub probe: Option<unsafe extern "C" fn(*mut macio_dev, *const of_device_id) -> ::core::ffi::c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut macio_dev)>,
    pub suspend: Option<unsafe extern "C" fn(*mut macio_dev, pm_message_t) -> ::core::ffi::c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut macio_dev) -> ::core::ffi::c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut macio_dev) -> ::core::ffi::c_int>,
    /* CONFIG_PMAC_MEDIABAY: mediabay_event: Option<unsafe extern "C" fn(*mut macio_dev, c_int)>, */
    pub driver: device_driver,
}

extern "C" {
    pub fn macio_register_driver(driver: *mut macio_driver) -> ::core::ffi::c_int;
    pub fn macio_unregister_driver(driver: *mut macio_driver);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
