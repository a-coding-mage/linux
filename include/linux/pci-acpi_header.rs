/* SPDX-License-Identifier: GPL-2.0 */
/*
 * File		pci-acpi.h
 *
 * Copyright (C) 2004 Intel
 * Copyright (C) Tom Long Nguyen (tom.l.nguyen@intel.com)
 */

/* C dependency: <linux/acpi.h> */

#[cfg(CONFIG_ACPI)]
extern "C" {
    pub fn pci_acpi_add_root_pm_notifier(
        dev: *mut acpi_device,
        pci_root: *mut acpi_pci_root,
    ) -> acpi_status;

    pub fn acpi_remove_pm_notifier(dev: *mut acpi_device) -> acpi_status;

    pub fn pci_acpi_add_pm_notifier(
        dev: *mut acpi_device,
        pci_dev: *mut pci_dev,
    ) -> acpi_status;

    pub fn acpi_pci_root_get_mcfg_addr(handle: acpi_handle) -> phys_addr_t;

    pub fn pci_mcfg_lookup(
        root: *mut acpi_pci_root,
        cfgres: *mut resource,
        ecam_ops: *mut *const pci_ecam_ops,
    ) -> core::ffi::c_int;

    pub fn pci_is_root_bus(pbus: *mut pci_bus) -> bool;
    pub fn acpi_handle_from_device(dev: *mut device) -> acpi_handle;
}

#[cfg(CONFIG_ACPI)]
#[inline]
pub unsafe fn pci_acpi_remove_bus_pm_notifier(dev: *mut acpi_device) -> acpi_status {
    unsafe { acpi_remove_pm_notifier(dev) }
}

#[cfg(CONFIG_ACPI)]
#[inline]
pub unsafe fn pci_acpi_remove_pm_notifier(dev: *mut acpi_device) -> acpi_status {
    unsafe { acpi_remove_pm_notifier(dev) }
}

#[cfg(CONFIG_ACPI)]
#[inline]
pub unsafe fn acpi_find_root_bridge_handle(pdev: *mut pci_dev) -> acpi_handle {
    let mut pbus = unsafe { (*pdev).bus };

    /* Find a PCI root bus */
    while !unsafe { pci_is_root_bus(pbus) } {
        pbus = unsafe { (*pbus).parent };
    }

    unsafe { acpi_handle_from_device((*pbus).bridge) }
}

#[cfg(CONFIG_ACPI)]
#[inline]
pub unsafe fn acpi_pci_get_bridge_handle(pbus: *mut pci_bus) -> acpi_handle {
    let dev: *mut device;

    if unsafe { pci_is_root_bus(pbus) } {
        dev = unsafe { (*pbus).bridge };
    } else {
        /* If pbus is a virtual bus, there is no bridge to it */
        if unsafe { (*pbus).self_ }.is_null() {
            return core::ptr::null_mut();
        }

        dev = unsafe { &mut (*(*pbus).self_).dev };
    }

    unsafe { acpi_handle_from_device(dev) }
}

/* C forward declarations: struct acpi_pci_root; struct acpi_pci_root_ops; */

#[repr(C)]
pub struct acpi_pci_root_info {
    pub root: *mut acpi_pci_root,
    pub bridge: *mut acpi_device,
    pub ops: *mut acpi_pci_root_ops,
    pub resources: list_head,
    pub name: [core::ffi::c_char; 16],
}

#[repr(C)]
pub struct acpi_pci_root_ops {
    pub pci_ops: *mut pci_ops,
    pub init_info: Option<unsafe extern "C" fn(*mut acpi_pci_root_info) -> core::ffi::c_int>,
    pub release_info: Option<unsafe extern "C" fn(*mut acpi_pci_root_info)>,
    pub prepare_resources:
        Option<unsafe extern "C" fn(*mut acpi_pci_root_info) -> core::ffi::c_int>,
}

#[cfg(CONFIG_ACPI)]
extern "C" {
    pub fn acpi_pci_probe_root_resources(info: *mut acpi_pci_root_info) -> core::ffi::c_int;
    pub fn acpi_pci_root_create(
        root: *mut acpi_pci_root,
        ops: *mut acpi_pci_root_ops,
        info: *mut acpi_pci_root_info,
        sd: *mut core::ffi::c_void,
    ) -> *mut pci_bus;

    pub fn acpi_pci_add_bus(bus: *mut pci_bus);
    pub fn acpi_pci_remove_bus(bus: *mut pci_bus);
}

#[cfg(CONFIG_ACPI)]
#[cfg(CONFIG_PCI)]
extern "C" {
    pub fn pci_acpi_setup(dev: *mut device, adev: *mut acpi_device);
    pub fn pci_acpi_cleanup(dev: *mut device, adev: *mut acpi_device);
}

#[cfg(CONFIG_ACPI)]
#[cfg(not(CONFIG_PCI))]
#[inline]
pub unsafe fn pci_acpi_setup(_dev: *mut device, _adev: *mut acpi_device) {}

#[cfg(CONFIG_ACPI)]
#[cfg(not(CONFIG_PCI))]
#[inline]
pub unsafe fn pci_acpi_cleanup(_dev: *mut device, _adev: *mut acpi_device) {}

#[cfg(CONFIG_ACPI)]
#[cfg(CONFIG_ACPI_PCI_SLOT)]
extern "C" {
    pub fn acpi_pci_slot_init();
    pub fn acpi_pci_slot_enumerate(bus: *mut pci_bus);
    pub fn acpi_pci_slot_remove(bus: *mut pci_bus);
}

#[cfg(CONFIG_ACPI)]
#[cfg(not(CONFIG_ACPI_PCI_SLOT))]
#[inline]
pub unsafe fn acpi_pci_slot_init() {}
#[cfg(CONFIG_ACPI)]
#[cfg(not(CONFIG_ACPI_PCI_SLOT))]
#[inline]
pub unsafe fn acpi_pci_slot_enumerate(_bus: *mut pci_bus) {}
#[cfg(CONFIG_ACPI)]
#[cfg(not(CONFIG_ACPI_PCI_SLOT))]
#[inline]
pub unsafe fn acpi_pci_slot_remove(_bus: *mut pci_bus) {}

#[cfg(CONFIG_ACPI)]
#[cfg(CONFIG_HOTPLUG_PCI_ACPI)]
extern "C" {
    pub fn acpiphp_init();
    pub fn acpiphp_enumerate_slots(bus: *mut pci_bus);
    pub fn acpiphp_remove_slots(bus: *mut pci_bus);
    pub fn acpiphp_check_host_bridge(adev: *mut acpi_device);
}

#[cfg(CONFIG_ACPI)]
#[cfg(not(CONFIG_HOTPLUG_PCI_ACPI))]
#[inline]
pub unsafe fn acpiphp_init() {}
#[cfg(CONFIG_ACPI)]
#[cfg(not(CONFIG_HOTPLUG_PCI_ACPI))]
#[inline]
pub unsafe fn acpiphp_enumerate_slots(_bus: *mut pci_bus) {}
#[cfg(CONFIG_ACPI)]
#[cfg(not(CONFIG_HOTPLUG_PCI_ACPI))]
#[inline]
pub unsafe fn acpiphp_remove_slots(_bus: *mut pci_bus) {}
#[cfg(CONFIG_ACPI)]
#[cfg(not(CONFIG_HOTPLUG_PCI_ACPI))]
#[inline]
pub unsafe fn acpiphp_check_host_bridge(_adev: *mut acpi_device) {}

pub static pci_acpi_dsm_guid: guid_t = unsafe { core::mem::zeroed() };

/* _DSM Definitions for PCI */
pub const DSM_PCI_PRESERVE_BOOT_CONFIG: u32 = 0x05;
pub const DSM_PCI_DEVICE_NAME: u32 = 0x07;
pub const DSM_PCI_POWER_ON_RESET_DELAY: u32 = 0x08;
pub const DSM_PCI_DEVICE_READINESS_DURATIONS: u32 = 0x09;

#[cfg(CONFIG_ACPI)]
#[cfg(CONFIG_PCIE_EDR)]
extern "C" {
    pub fn pci_acpi_add_edr_notifier(pdev: *mut pci_dev);
    pub fn pci_acpi_remove_edr_notifier(pdev: *mut pci_dev);
}

#[cfg(CONFIG_ACPI)]
#[cfg(not(CONFIG_PCIE_EDR))]
#[inline]
pub unsafe fn pci_acpi_add_edr_notifier(_pdev: *mut pci_dev) {}
#[cfg(CONFIG_ACPI)]
#[cfg(not(CONFIG_PCIE_EDR))]
#[inline]
pub unsafe fn pci_acpi_remove_edr_notifier(_pdev: *mut pci_dev) {}

#[cfg(CONFIG_ACPI)]
extern "C" {
    pub fn pci_acpi_set_companion_lookup_hook(
        func: Option<unsafe extern "C" fn(*mut pci_dev) -> *mut acpi_device>,
    ) -> core::ffi::c_int;
    pub fn pci_acpi_clear_companion_lookup_hook();
}

#[cfg(not(CONFIG_ACPI))]
#[inline]
pub unsafe fn acpi_pci_add_bus(_bus: *mut pci_bus) {}
#[cfg(not(CONFIG_ACPI))]
#[inline]
pub unsafe fn acpi_pci_remove_bus(_bus: *mut pci_bus) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
