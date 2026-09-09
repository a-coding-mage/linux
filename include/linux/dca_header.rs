/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright(c) 2007 - 2009 Intel Corporation. All rights reserved.
 */

// Dependency supplied by the Linux PCI headers: linux/pci.h

/* DCA Provider API */

/* DCA Notifier Interface */
extern "C" {
    pub fn dca_register_notify(nb: *mut notifier_block);
    pub fn dca_unregister_notify(nb: *mut notifier_block);
}

pub const DCA_PROVIDER_ADD: i32 = 0x0001;
pub const DCA_PROVIDER_REMOVE: i32 = 0x0002;

#[repr(C)]
pub struct dca_provider {
    pub node: list_head,
    pub ops: *const dca_ops,
    pub cd: *mut device,
    pub id: i32,
}

#[repr(C)]
pub struct dca_domain {
    pub node: list_head,
    pub dca_providers: list_head,
    pub pci_rc: *mut pci_bus,
}

#[repr(C)]
pub struct dca_ops {
    pub add_requester:
        Option<unsafe extern "C" fn(*mut dca_provider, *mut device) -> i32>,
    pub remove_requester:
        Option<unsafe extern "C" fn(*mut dca_provider, *mut device) -> i32>,
    pub get_tag:
        Option<unsafe extern "C" fn(*mut dca_provider, *mut device, i32) -> u8>,
    pub dev_managed:
        Option<unsafe extern "C" fn(*mut dca_provider, *mut device) -> i32>,
}

extern "C" {
    pub fn alloc_dca_provider(ops: *const dca_ops, priv_size: i32) -> *mut dca_provider;
    pub fn free_dca_provider(dca: *mut dca_provider);
    pub fn register_dca_provider(dca: *mut dca_provider, dev: *mut device) -> i32;
    pub fn unregister_dca_provider(dca: *mut dca_provider, dev: *mut device);
}

pub unsafe fn dca_priv(dca: *mut dca_provider) -> *mut core::ffi::c_void {
    (dca as *mut u8).add(core::mem::size_of::<dca_provider>()) as *mut core::ffi::c_void
}

/* Requester API */
// #define DCA_GET_TAG_TWO_ARGS
pub const DCA_GET_TAG_TWO_ARGS: () = ();
extern "C" {
    pub fn dca_add_requester(dev: *mut device) -> i32;
    pub fn dca_remove_requester(dev: *mut device) -> i32;
    pub fn dca_get_tag(cpu: i32) -> u8;
    pub fn dca3_get_tag(dev: *mut device, cpu: i32) -> u8;
}

/* internal stuff */
// The C __init and __exit attributes are build-system/linker annotations.
extern "C" {
    pub fn dca_sysfs_init() -> i32;
    pub fn dca_sysfs_exit();
    pub fn dca_sysfs_add_provider(dca: *mut dca_provider, dev: *mut device) -> i32;
    pub fn dca_sysfs_remove_provider(dca: *mut dca_provider);
    pub fn dca_sysfs_add_req(dca: *mut dca_provider, dev: *mut device, slot: i32) -> i32;
    pub fn dca_sysfs_remove_req(dca: *mut dca_provider, slot: i32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
