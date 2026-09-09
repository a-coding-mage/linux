/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2014 IBM Corp.
 */

// Dependencies supplied by the corresponding kernel headers.

pub const PCI_SLOT_ID_PREFIX: u64 = 1u64 << 63;

#[inline]
pub const fn PCI_SLOT_ID(phb_id: u64, bdfn: u64) -> u64 {
    PCI_SLOT_ID_PREFIX | (bdfn << 16) | phb_id
}

#[inline]
pub const fn PCI_PHB_SLOT_ID(phb_id: u64) -> u64 {
    phb_id
}

extern "C" {
    pub fn pnv_pci_get_slot_id(np: *mut device_node, id: *mut u64) -> ::core::ffi::c_int;
    pub fn pnv_pci_get_device_tree(
        phandle: u32,
        buf: *mut ::core::ffi::c_void,
        len: u64,
    ) -> ::core::ffi::c_int;
    pub fn pnv_pci_get_presence_state(id: u64, state: *mut u8) -> ::core::ffi::c_int;
    pub fn pnv_pci_get_power_state(id: u64, state: *mut u8) -> ::core::ffi::c_int;
    pub fn pnv_pci_set_power_state(
        id: u64,
        state: u8,
        msg: *mut opal_msg,
    ) -> ::core::ffi::c_int;

    pub fn pnv_opal_pci_msi_eoi(d: *mut irq_data) -> i64;
    pub fn is_pnv_opal_msi(chip: *mut irq_chip) -> bool;
}

pub const PNV_PHP_FLAG_BROKEN_PDC: u32 = 0x1;
pub const PNV_PHP_STATE_INITIALIZED: i32 = 0;
pub const PNV_PHP_STATE_REGISTERED: i32 = 1;
pub const PNV_PHP_STATE_POPULATED: i32 = 2;
pub const PNV_PHP_STATE_OFFLINE: i32 = 3;

#[repr(C)]
pub struct pnv_php_slot {
    pub slot: hotplug_slot,
    pub id: u64,
    pub name: *mut ::core::ffi::c_char,
    pub slot_no: ::core::ffi::c_int,
    pub flags: u32,
    pub kref: kref,
    pub state: ::core::ffi::c_int,
    pub irq: ::core::ffi::c_int,
    pub wq: *mut workqueue_struct,
    pub dn: *mut device_node,
    pub pdev: *mut pci_dev,
    pub bus: *mut pci_bus,
    pub power_state_check: bool,
    pub attention_state: u8,
    pub fdt: *mut ::core::ffi::c_void,
    pub dt: *mut ::core::ffi::c_void,
    pub ocs: of_changeset,
    pub parent: *mut pnv_php_slot,
    pub children: list_head,
    pub link: list_head,
}

extern "C" {
    pub fn pnv_php_find_slot(dn: *mut device_node) -> *mut pnv_php_slot;
    pub fn pnv_php_set_slot_power_state(
        slot: *mut hotplug_slot,
        state: u8,
    ) -> ::core::ffi::c_int;
}

// Opaque declarations for types supplied by included headers.
pub enum device_node {}
pub enum opal_msg {}
pub enum irq_data {}
pub enum irq_chip {}
pub enum hotplug_slot {}
pub enum kref {}
pub enum workqueue_struct {}
pub enum pci_dev {}
pub enum pci_bus {}
pub enum of_changeset {}
pub enum list_head {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
