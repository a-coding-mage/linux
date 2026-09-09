/* SPDX-License-Identifier: GPL-2.0 */
/*
 * KUnit resource management helpers for firmware nodes.
 *
 * Copyright (C) Qualcomm Technologies, Inc. and/or its subsidiaries
 */

// Forward declarations supplied by the corresponding dependencies.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct property_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct software_node {
    _private: [u8; 0],
}

extern "C" {
    pub fn kunit_fwnode_create_software_node(
        test: *mut kunit,
        properties: *const property_entry,
        parent: *const fwnode_handle,
    ) -> *mut fwnode_handle;

    pub fn kunit_software_node_register(
        test: *mut kunit,
        node: *const software_node,
    ) -> *mut fwnode_handle;

    pub fn kunit_software_node_register_node_group(
        test: *mut kunit,
        nodes: *const *const software_node,
    ) -> std::ffi::c_int;

    pub fn kunit_device_add_software_node(
        test: *mut kunit,
        dev: *mut device,
        node: *const software_node,
    ) -> std::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
