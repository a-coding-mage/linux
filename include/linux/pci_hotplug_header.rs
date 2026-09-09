/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * PCI HotPlug Core Functions
 *
 * Copyright (C) 1995,2001 Compaq Computer Corporation
 * Copyright (C) 2001 Greg Kroah-Hartman (greg@kroah.com)
 * Copyright (C) 2001 IBM Corp.
 *
 * All rights reserved.
 *
 * Send feedback to <kristen.c.accardi@intel.com>
 */

/** The callbacks that the hotplug PCI core can use. */
#[repr(C)]
pub struct hotplug_slot_ops {
    pub enable_slot: Option<unsafe extern "C" fn(slot: *mut hotplug_slot) -> i32>,
    pub disable_slot: Option<unsafe extern "C" fn(slot: *mut hotplug_slot) -> i32>,
    pub set_attention_status:
        Option<unsafe extern "C" fn(slot: *mut hotplug_slot, value: u8) -> i32>,
    pub hardware_test: Option<unsafe extern "C" fn(slot: *mut hotplug_slot, value: u32) -> i32>,
    pub get_power_status:
        Option<unsafe extern "C" fn(slot: *mut hotplug_slot, value: *mut u8) -> i32>,
    pub get_attention_status:
        Option<unsafe extern "C" fn(slot: *mut hotplug_slot, value: *mut u8) -> i32>,
    pub get_latch_status:
        Option<unsafe extern "C" fn(slot: *mut hotplug_slot, value: *mut u8) -> i32>,
    pub get_adapter_status:
        Option<unsafe extern "C" fn(slot: *mut hotplug_slot, value: *mut u8) -> i32>,
    pub reset_slot: Option<unsafe extern "C" fn(slot: *mut hotplug_slot, probe: bool) -> i32>,
}

/** Used to register a physical slot with the hotplug PCI core. */
#[repr(C)]
pub struct hotplug_slot {
    pub ops: *const hotplug_slot_ops,
    /* Variables below this are for use only by the hotplug PCI core. */
    pub pci_slot: *mut pci_slot,
    pub owner: *mut module,
    pub mod_name: *const core::ffi::c_char,
}

extern "C" {
    pub fn pci_slot_name(slot: *mut pci_slot) -> *const core::ffi::c_char;
    pub fn __pci_hp_register(
        slot: *mut hotplug_slot,
        pbus: *mut pci_bus,
        nr: i32,
        name: *const core::ffi::c_char,
        owner: *mut module,
        mod_name: *const core::ffi::c_char,
    ) -> i32;
    pub fn __pci_hp_initialize(
        slot: *mut hotplug_slot,
        bus: *mut pci_bus,
        nr: i32,
        name: *const core::ffi::c_char,
        owner: *mut module,
        mod_name: *const core::ffi::c_char,
    ) -> i32;
    pub fn pci_hp_add(slot: *mut hotplug_slot) -> i32;
    pub fn pci_hp_del(slot: *mut hotplug_slot);
    pub fn pci_hp_destroy(slot: *mut hotplug_slot);
    pub fn pci_hp_deregister(slot: *mut hotplug_slot);
}

#[inline]
pub unsafe fn hotplug_slot_name(slot: *const hotplug_slot) -> *const core::ffi::c_char {
    unsafe { pci_slot_name((*slot).pci_slot) }
}

/* These macros avoid include chaining to obtain THIS_MODULE and KBUILD_MODNAME. */
#[macro_export]
macro_rules! pci_hp_register {
    ($slot:expr, $pbus:expr, $devnr:expr, $name:expr) => {
        $crate::__pci_hp_register($slot, $pbus, $devnr, $name, THIS_MODULE, KBUILD_MODNAME)
    };
}
#[macro_export]
macro_rules! pci_hp_initialize {
    ($slot:expr, $bus:expr, $nr:expr, $name:expr) => {
        $crate::__pci_hp_initialize($slot, $bus, $nr, $name, THIS_MODULE, KBUILD_MODNAME)
    };
}

/* CONFIG_ACPI declarations are provided when ACPI is enabled. */
#[cfg(CONFIG_ACPI)]
extern "C" {
    pub fn pciehp_is_native(bridge: *mut pci_dev) -> bool;
    pub fn acpi_get_hp_hw_control_from_firmware(bridge: *mut pci_dev) -> i32;
    pub fn shpchp_is_native(bridge: *mut pci_dev) -> bool;
    pub fn acpi_pci_check_ejectable(pbus: *mut pci_bus, handle: acpi_handle) -> i32;
    pub fn acpi_pci_detect_ejectable(handle: acpi_handle) -> i32;
}

/* When CONFIG_ACPI is disabled, these inline definitions return the C defaults. */
#[cfg(not(CONFIG_ACPI))]
#[inline]
pub unsafe fn acpi_get_hp_hw_control_from_firmware(_bridge: *mut pci_dev) -> i32 { 0 }
#[cfg(not(CONFIG_ACPI))]
#[inline]
pub unsafe fn pciehp_is_native(_bridge: *mut pci_dev) -> bool { true }
#[cfg(not(CONFIG_ACPI))]
#[inline]
pub unsafe fn shpchp_is_native(_bridge: *mut pci_dev) -> bool { true }

#[inline]
pub unsafe fn hotplug_is_native(bridge: *mut pci_dev) -> bool {
    unsafe { ((*bridge).is_pciehp && pciehp_is_native(bridge)) || shpchp_is_native(bridge) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
