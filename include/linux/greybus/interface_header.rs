/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Greybus Interface Block code
 *
 * Copyright 2014 Google Inc.
 * Copyright 2014 Linaro Ltd.
 */

/* C dependencies: linux/types.h and linux/device.h. */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum gb_interface_type {
    GB_INTERFACE_TYPE_INVALID = 0,
    GB_INTERFACE_TYPE_UNKNOWN,
    GB_INTERFACE_TYPE_DUMMY,
    GB_INTERFACE_TYPE_UNIPRO,
    GB_INTERFACE_TYPE_GREYBUS,
}

pub const GB_INTERFACE_QUIRK_NO_CPORT_FEATURES: usize = 1usize << 0;
pub const GB_INTERFACE_QUIRK_NO_INIT_STATUS: usize = 1usize << 1;
pub const GB_INTERFACE_QUIRK_NO_GMP_IDS: usize = 1usize << 2;
pub const GB_INTERFACE_QUIRK_FORCED_DISABLE: usize = 1usize << 3;
pub const GB_INTERFACE_QUIRK_LEGACY_MODE_SWITCH: usize = 1usize << 4;
pub const GB_INTERFACE_QUIRK_NO_BUNDLE_ACTIVATE: usize = 1usize << 5;
pub const GB_INTERFACE_QUIRK_NO_PM: usize = 1usize << 6;

/* External kernel and Greybus types supplied by other translation units. */
pub struct device;
pub struct gb_control;
pub struct list_head;
pub struct gb_host_device;
pub struct gb_module;
pub struct mutex;
pub struct work_struct;
pub struct completion;

#[repr(C)]
pub struct gb_interface {
    pub dev: device,
    pub control: *mut gb_control,

    pub bundles: list_head,
    pub module_node: list_head,
    pub manifest_descs: list_head,
    pub interface_id: u8, /* Physical location within the Endo */
    pub device_id: u8,
    pub features: u8, /* Feature flags set in the manifest */

    pub type_: gb_interface_type,

    pub ddbl1_manufacturer_id: u32,
    pub ddbl1_product_id: u32,
    pub vendor_id: u32,
    pub product_id: u32,
    pub serial_number: u64,

    pub hd: *mut gb_host_device,
    pub module: *mut gb_module,

    pub quirks: usize,

    pub mutex: mutex,

    pub disconnected: bool,

    pub ejected: bool,
    pub removed: bool,
    pub active: bool,
    pub enabled: bool,
    pub mode_switch: bool,
    pub dme_read: bool,

    pub mode_switch_work: work_struct,
    pub mode_switch_completion: completion,
}

/* Equivalent of container_of(d, struct gb_interface, dev). */
pub unsafe fn to_gb_interface(d: *mut device) -> *mut gb_interface {
    (d as *mut u8).sub(core::mem::offset_of!(gb_interface, dev)) as *mut gb_interface
}

unsafe extern "C" {
    pub fn gb_interface_create(module: *mut gb_module, interface_id: u8) -> *mut gb_interface;
    pub fn gb_interface_activate(intf: *mut gb_interface) -> i32;
    pub fn gb_interface_deactivate(intf: *mut gb_interface);
    pub fn gb_interface_enable(intf: *mut gb_interface) -> i32;
    pub fn gb_interface_disable(intf: *mut gb_interface);
    pub fn gb_interface_add(intf: *mut gb_interface) -> i32;
    pub fn gb_interface_del(intf: *mut gb_interface);
    pub fn gb_interface_put(intf: *mut gb_interface);
    pub fn gb_interface_mailbox_event(intf: *mut gb_interface, result: u16, mailbox: u32);
    pub fn gb_interface_request_mode_switch(intf: *mut gb_interface) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
