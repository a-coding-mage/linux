/* SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause */
/* Copyright (c) 2014 Raspberry Pi (Trading) Ltd. All rights reserved. */

// Translated from the C header. The original include guard is not needed in Rust.

#[repr(C)]
pub struct vchiq_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vchiq_instance {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vchiq_debugfs_node {
    pub dentry: *mut dentry,
}

unsafe extern "C" {
    pub fn vchiq_debugfs_init(state: *mut vchiq_state);

    pub fn vchiq_debugfs_deinit();

    pub fn vchiq_debugfs_add_instance(instance: *mut vchiq_instance);

    pub fn vchiq_debugfs_remove_instance(instance: *mut vchiq_instance);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
