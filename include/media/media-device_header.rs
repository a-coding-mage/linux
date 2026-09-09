/* SPDX-License-Identifier: GPL-2.0-only */
/* Media device -- direct Rust translation of media-device.h. */

/* C header dependencies are supplied by other translation units. */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct media_devnode { _private: [u8; 0] }
#[repr(C)] pub struct media_entity { _private: [u8; 0] }
#[repr(C)] pub struct media_link { _private: [u8; 0] }
#[repr(C)] pub struct media_request { _private: [u8; 0] }
#[repr(C)] pub struct media_graph { _private: [u8; 0] }
#[repr(C)] pub struct media_pipeline { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { _private: [u8; 0] }
#[repr(C)] pub struct ida { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct pci_dev { _private: [u8; 0] }
#[repr(C)] pub struct usb_device { _private: [u8; 0] }
#[repr(C)] pub struct module { _private: [u8; 0] }

pub type u32 = core::ffi::c_uint;
pub type u64 = core::ffi::c_ulonglong;

#[repr(C)]
pub struct media_entity_notify {
    pub list: list_head,
    pub notify_data: *mut c_void,
    pub notify: Option<unsafe extern "C" fn(entity: *mut media_entity, notify_data: *mut c_void)>,
}

#[repr(C)]
pub struct media_device_ops {
    pub link_notify: Option<unsafe extern "C" fn(link: *mut media_link, flags: u32, notification: c_uint) -> c_int>,
    pub req_alloc: Option<unsafe extern "C" fn(mdev: *mut media_device) -> *mut media_request>,
    pub req_free: Option<unsafe extern "C" fn(req: *mut media_request)>,
    pub req_validate: Option<unsafe extern "C" fn(req: *mut media_request) -> c_int>,
    pub req_queue: Option<unsafe extern "C" fn(req: *mut media_request)>,
}

pub type c_uint = core::ffi::c_uint;

#[repr(C)]
pub struct media_device {
    pub dev: *mut device,
    pub devnode: *mut media_devnode,
    pub model: [c_char; 32],
    pub driver_name: [c_char; 32],
    pub serial: [c_char; 40],
    pub bus_info: [c_char; 32],
    pub hw_revision: u32,
    pub topology_version: u64,
    pub id: u32,
    pub entity_internal_idx: ida,
    pub entity_internal_idx_max: c_int,
    pub entities: list_head,
    pub interfaces: list_head,
    pub pads: list_head,
    pub links: list_head,
    pub entity_notify: list_head,
    pub graph_mutex: mutex,
    pub pm_count_walk: media_graph,
    pub source_priv: *mut c_void,
    pub enable_source: Option<unsafe extern "C" fn(entity: *mut media_entity, pipe: *mut media_pipeline) -> c_int>,
    pub disable_source: Option<unsafe extern "C" fn(entity: *mut media_entity)>,
    pub ops: *const media_device_ops,
    pub req_queue_mutex: mutex,
    pub num_requests: atomic_t,
    pub num_request_objects: atomic_t,
    pub media_dir: *mut dentry,
    pub request_id: atomic_t,
}

/* We don't need to include usb.h here. */

pub const MEDIA_DEV_NOTIFY_PRE_LINK_CH: c_int = 0;
pub const MEDIA_DEV_NOTIFY_POST_LINK_CH: c_int = 1;

/* CONFIG_MEDIA_CONTROLLER declarations. */
extern "C" {
    pub fn media_device_init(mdev: *mut media_device);
    pub fn media_device_cleanup(mdev: *mut media_device);
    pub fn __media_device_register(mdev: *mut media_device, owner: *mut module) -> c_int;
    pub fn media_device_unregister(mdev: *mut media_device);
    pub fn media_device_register_entity(mdev: *mut media_device, entity: *mut media_entity) -> c_int;
    pub fn media_device_unregister_entity(entity: *mut media_entity);
    pub fn media_device_register_entity_notify(mdev: *mut media_device, nptr: *mut media_entity_notify);
    pub fn media_device_unregister_entity_notify(mdev: *mut media_device, nptr: *mut media_entity_notify);
    pub fn media_device_pci_init(mdev: *mut media_device, pci_dev: *mut pci_dev, name: *const c_char);
    pub fn __media_device_usb_init(mdev: *mut media_device, udev: *mut usb_device, board_name: *const c_char, driver_name: *const c_char);
}

/* The C macro passes THIS_MODULE. */
#[macro_export]
macro_rules! media_device_register { ($mdev:expr) => { $crate::__media_device_register($mdev, THIS_MODULE) }; }
#[macro_export]
macro_rules! media_device_usb_init { ($mdev:expr, $udev:expr, $name:expr) => { $crate::__media_device_usb_init($mdev, $udev, $name, KBUILD_MODNAME) }; }

/* C list_for_each_entry iteration macros; the list implementation is external. */
#[macro_export] macro_rules! media_device_for_each_entity { ($entity:expr, $mdev:expr) => { list_for_each_entry!($entity, &mut (*$mdev).entities, graph_obj.list) }; }
#[macro_export] macro_rules! media_device_for_each_intf { ($intf:expr, $mdev:expr) => { list_for_each_entry!($intf, &mut (*$mdev).interfaces, graph_obj.list) }; }
#[macro_export] macro_rules! media_device_for_each_pad { ($pad:expr, $mdev:expr) => { list_for_each_entry!($pad, &mut (*$mdev).pads, graph_obj.list) }; }
#[macro_export] macro_rules! media_device_for_each_link { ($link:expr, $mdev:expr) => { list_for_each_entry!($link, &mut (*$mdev).links, graph_obj.list) }; }

/* The CONFIG_MEDIA_CONTROLLER disabled branch provides no-op inline stubs. */
#[cfg(not(feature = "CONFIG_MEDIA_CONTROLLER"))]
pub unsafe fn media_device_init_disabled(_: *mut media_device) {}
#[cfg(not(feature = "CONFIG_MEDIA_CONTROLLER"))]
pub unsafe fn media_device_register_disabled(_: *mut media_device) -> c_int { 0 }
#[cfg(not(feature = "CONFIG_MEDIA_CONTROLLER"))]
pub unsafe fn media_device_unregister_disabled(_: *mut media_device) {}
#[cfg(not(feature = "CONFIG_MEDIA_CONTROLLER"))]
pub unsafe fn media_device_cleanup_disabled(_: *mut media_device) {}
#[cfg(not(feature = "CONFIG_MEDIA_CONTROLLER"))]
pub unsafe fn media_device_register_entity_disabled(_: *mut media_device, _: *mut media_entity) -> c_int { 0 }
#[cfg(not(feature = "CONFIG_MEDIA_CONTROLLER"))]
pub unsafe fn media_device_unregister_entity_disabled(_: *mut media_entity) {}
#[cfg(not(feature = "CONFIG_MEDIA_CONTROLLER"))]
pub unsafe fn media_device_register_entity_notify_disabled(_: *mut media_device, _: *mut media_entity_notify) {}
#[cfg(not(feature = "CONFIG_MEDIA_CONTROLLER"))]
pub unsafe fn media_device_unregister_entity_notify_disabled(_: *mut media_device, _: *mut media_entity_notify) {}
#[cfg(not(feature = "CONFIG_MEDIA_CONTROLLER"))]
pub unsafe fn media_device_pci_init_disabled(_: *mut media_device, _: *mut pci_dev, _: *mut c_char) {}
#[cfg(not(feature = "CONFIG_MEDIA_CONTROLLER"))]
pub unsafe fn media_device_usb_init_disabled(_: *mut media_device, _: *mut usb_device, _: *mut c_char, _: *mut c_char) {}

extern "C" {
    fn strscpy(dst: *mut c_char, src: *const c_char, size: usize) -> isize;
    fn dev_is_platform(dev: *mut device) -> bool;
    fn dev_is_pci(dev: *mut device) -> bool;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn snprintf(dst: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
}

/* The C inline helper uses the kernel string and device helpers above. */
pub unsafe fn media_set_bus_info(bus_info: *mut c_char, bus_info_size: usize, dev: *mut device) {
    if dev.is_null() {
        strscpy(bus_info, b"no bus info\0".as_ptr() as *const c_char, bus_info_size);
    } else if dev_is_platform(dev) {
        snprintf(bus_info, bus_info_size, b"platform:%s\0".as_ptr() as *const c_char, dev_name(dev));
    } else if dev_is_pci(dev) {
        snprintf(bus_info, bus_info_size, b"PCI:%s\0".as_ptr() as *const c_char, dev_name(dev));
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
