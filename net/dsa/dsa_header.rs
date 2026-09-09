/* SPDX-License-Identifier: GPL-2.0-or-later */

// Declarations corresponding to the included Linux kernel types and
// forward-declared structures.
#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dsa_db {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dsa_device_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dsa_lag {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dsa_switch_tree {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dsa_bridge {
    _private: [u8; 0],
}

extern "C" {
    pub static mut dsa_tree_list: list_head;

    pub fn dsa_db_equal(a: *const dsa_db, b: *const dsa_db) -> bool;
    pub fn dsa_schedule_work(work: *mut work_struct) -> bool;
    pub fn dsa_lag_map(dst: *mut dsa_switch_tree, lag: *mut dsa_lag);
    pub fn dsa_lag_unmap(dst: *mut dsa_switch_tree, lag: *mut dsa_lag);
    pub fn dsa_tree_lag_find(
        dst: *mut dsa_switch_tree,
        lag_dev: *const net_device,
    ) -> *mut dsa_lag;
    pub fn dsa_tree_find_first_conduit(dst: *mut dsa_switch_tree) -> *mut net_device;
    pub fn dsa_tree_change_tag_proto(
        dst: *mut dsa_switch_tree,
        tag_ops: *const dsa_device_ops,
        old_tag_ops: *const dsa_device_ops,
    ) -> i32;
    pub fn dsa_tree_conduit_admin_state_change(
        dst: *mut dsa_switch_tree,
        conduit: *mut net_device,
        up: bool,
    );
    pub fn dsa_tree_conduit_oper_state_change(
        dst: *mut dsa_switch_tree,
        conduit: *mut net_device,
        up: bool,
    );
    pub fn dsa_bridge_num_get(bridge_dev: *const net_device, max: i32) -> u32;
    pub fn dsa_bridge_num_put(bridge_dev: *const net_device, bridge_num: u32);
    pub fn dsa_tree_bridge_find(
        dst: *mut dsa_switch_tree,
        br: *const net_device,
    ) -> *mut dsa_bridge;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
