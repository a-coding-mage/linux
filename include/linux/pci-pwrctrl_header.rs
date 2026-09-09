/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2024 Linaro Ltd.
 */

/* Dependencies supplied by the surrounding kernel translation unit. */
pub struct device;
pub struct device_link;
pub struct notifier_block;
pub struct work_struct;

/*
 * This is a simple framework for solving the issue of PCI devices that require
 * certain resources (regulators, GPIOs, clocks) to be enabled before the
 * device can actually be detected on the PCI bus.
 *
 * The idea is to reuse the platform bus to populate OF nodes describing the
 * PCI device and its resources, let these platform devices probe and enable
 * relevant resources and then trigger a rescan of the PCI bus allowing for the
 * same device (with a second associated struct device) to be registered with
 * the PCI subsystem.
 *
 * To preserve a correct hierarchy for PCI power management and device reset,
 * we create a device link between the power control platform device (parent)
 * and the supplied PCI device (child).
 */

/**
 * struct pci_pwrctrl - PCI device power control context.
 * @dev: Address of the power controlling device.
 * @power_on: Callback to power on the power controlling device.
 * @power_off: Callback to power off the power controlling device.
 *
 * An object of this type must be allocated by the PCI power control device and
 * passed to the pwrctrl subsystem to trigger a bus rescan and setup a device
 * link with the device once it's up.
 */
#[repr(C)]
pub struct pci_pwrctrl {
    pub dev: *mut device,
    pub power_on: Option<unsafe extern "C" fn(pwrctrl: *mut pci_pwrctrl) -> i32>,
    pub power_off: Option<unsafe extern "C" fn(pwrctrl: *mut pci_pwrctrl) -> i32>,

    /* private: internal use only */
    pub nb: notifier_block,
    pub link: *mut device_link,
    pub work: work_struct,
}

unsafe extern "C" {
    pub fn pci_pwrctrl_init(pwrctrl: *mut pci_pwrctrl, dev: *mut device);
    pub fn pci_pwrctrl_device_set_ready(pwrctrl: *mut pci_pwrctrl) -> i32;
    pub fn pci_pwrctrl_device_unset_ready(pwrctrl: *mut pci_pwrctrl);
    pub fn devm_pci_pwrctrl_device_set_ready(
        dev: *mut device,
        pwrctrl: *mut pci_pwrctrl,
    ) -> i32;
}

/* IS_ENABLED(CONFIG_PCI_PWRCTRL) selects these declarations at build time. */
#[cfg(feature = "CONFIG_PCI_PWRCTRL")]
unsafe extern "C" {
    pub fn pci_pwrctrl_create_devices(parent: *mut device) -> i32;
    pub fn pci_pwrctrl_destroy_devices(parent: *mut device);
    pub fn pci_pwrctrl_power_on_devices(parent: *mut device) -> i32;
    pub fn pci_pwrctrl_power_off_devices(parent: *mut device);
}

#[cfg(not(feature = "CONFIG_PCI_PWRCTRL"))]
#[inline]
pub unsafe fn pci_pwrctrl_create_devices(_parent: *mut device) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_PCI_PWRCTRL"))]
#[inline]
pub unsafe fn pci_pwrctrl_destroy_devices(_parent: *mut device) {}

#[cfg(not(feature = "CONFIG_PCI_PWRCTRL"))]
#[inline]
pub unsafe fn pci_pwrctrl_power_on_devices(_parent: *mut device) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_PCI_PWRCTRL"))]
#[inline]
pub unsafe fn pci_pwrctrl_power_off_devices(_parent: *mut device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
