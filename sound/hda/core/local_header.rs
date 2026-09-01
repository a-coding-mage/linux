/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Local helper macros and functions for HD-audio core drivers
 */

use core::ffi::{c_int, c_uint};

#[repr(C)]
pub struct attribute_group {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hdac_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hdac_bus {
    _private: [u8; 0],
}

// External dependency: hda_nid_t is defined by the HD-audio core headers.
pub type hda_nid_t = crate::hda_nid_t;

unsafe extern "C" {
    pub static hdac_dev_attr_groups: [*const attribute_group; 0];

    pub fn hda_widget_sysfs_init(codec: *mut hdac_device) -> c_int;
    pub fn hda_widget_sysfs_reinit(
        codec: *mut hdac_device,
        start_nid: hda_nid_t,
        num_nodes: c_int,
    ) -> c_int;
    pub fn hda_widget_sysfs_exit(codec: *mut hdac_device);

    pub fn snd_hdac_bus_add_device(bus: *mut hdac_bus, codec: *mut hdac_device) -> c_int;
    pub fn snd_hdac_bus_remove_device(bus: *mut hdac_bus, codec: *mut hdac_device);
    pub fn snd_hdac_bus_queue_event(bus: *mut hdac_bus, res: u32, res_ex: u32);
    pub fn snd_hdac_bus_exec_verb(
        bus: *mut hdac_bus,
        addr: c_uint,
        cmd: c_uint,
        res: *mut c_uint,
    ) -> c_int;

    pub fn snd_hdac_exec_verb(
        codec: *mut hdac_device,
        cmd: c_uint,
        flags: c_uint,
        res: *mut c_uint,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
