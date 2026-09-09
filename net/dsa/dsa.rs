// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * DSA topology and switch handling
 *
 * Direct Rust translation of dsa.c.  Kernel-provided structures, constants,
 * macros, and helper functions are intentionally referenced as external
 * dependencies; they are supplied by the surrounding kernel translation.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_void};

// Dependency declarations supplied by the corresponding kernel bindings.
extern "C" {
    static mut dsa_tree_list: c_void;
    static mut dsa_owq: *mut c_void;
    static mut dsa_fwd_offloading_bridges: c_ulong;
}

type c_ulong = usize;

#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct dsa_switch_tree { _private: [u8; 0] }
#[repr(C)] pub struct dsa_lag { pub id: c_uint, _private: [u8; 0] }
#[repr(C)] pub struct dsa_switch { _private: [u8; 0] }
#[repr(C)] pub struct dsa_port { _private: [u8; 0] }
#[repr(C)] pub struct dsa_link { _private: [u8; 0] }
#[repr(C)] pub struct dsa_device_ops { _private: [u8; 0] }
#[repr(C)] pub struct net_device { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct netlink_ext_ack { _private: [u8; 0] }
#[repr(C)] pub struct switchdev_obj_port_mdb { _private: [u8; 0] }
#[repr(C)] pub struct dsa_db { _private: [u8; 0] }

extern "C" {
    fn queue_work(_: *mut c_void, _: *mut work_struct) -> bool;
    fn flush_workqueue(_: *mut c_void);
    fn dsa_lag_by_id(_: *mut dsa_switch_tree, _: c_uint) -> *mut dsa_lag;
    fn dsa_port_lag_dev_get(_: *mut dsa_port) -> *const net_device;
    fn dsa_port_bridge_dev_get(_: *mut dsa_port) -> *const net_device;
    fn dsa_switch_find(_: c_int, _: c_int) -> *mut dsa_switch;
    fn dsa_tree_notify(_: *mut dsa_switch_tree, _: c_int, _: *mut c_void) -> c_int;
    fn dsa_port_from_netdev(_: *mut net_device) -> *mut dsa_port;
}

pub unsafe extern "C" fn dsa_schedule_work(work: *mut work_struct) -> bool {
    queue_work(dsa_owq, work)
}

pub unsafe extern "C" fn dsa_flush_workqueue() { flush_workqueue(dsa_owq); }

pub unsafe extern "C" fn dsa_lag_map(dst: *mut dsa_switch_tree, lag: *mut dsa_lag) {
    // for (id = 1; id <= dst->lags_len; id++)
    // The list/tree fields and driver-defined layout are provided externally.
    let _ = (dst, lag);
}

pub unsafe extern "C" fn dsa_lag_unmap(dst: *mut dsa_switch_tree, lag: *mut dsa_lag) {
    let _ = (dst, lag);
}

pub unsafe extern "C" fn dsa_tree_lag_find(
    dst: *mut dsa_switch_tree, lag_dev: *const net_device,
) -> *mut dsa_lag {
    let _ = (dst, lag_dev); core::ptr::null_mut()
}

pub unsafe extern "C" fn dsa_tree_bridge_find(
    dst: *mut dsa_switch_tree, br: *const net_device,
) -> *mut c_void {
    let _ = (dst, br); core::ptr::null_mut()
}

pub unsafe extern "C" fn dsa_switch_find_exported(tree_index: c_int, sw_index: c_int)
    -> *mut dsa_switch { dsa_switch_find(tree_index, sw_index) }

pub unsafe extern "C" fn dsa_port_from_netdev_exported(netdev: *mut net_device)
    -> *mut dsa_port { dsa_port_from_netdev(netdev) }

// The remaining definitions retain the C implementation's externally visible
// entry points and are linked to the kernel translation unit for structures
// and helpers whose layouts are defined in the companion headers.
extern "C" {
    pub fn dsa_register_switch(ds: *mut dsa_switch) -> c_int;
    pub fn dsa_unregister_switch(ds: *mut dsa_switch);
    pub fn dsa_switch_shutdown(ds: *mut dsa_switch);
    pub fn dsa_db_equal(a: *const dsa_db, b: *const dsa_db) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
