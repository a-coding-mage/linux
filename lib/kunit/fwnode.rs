// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) Qualcomm Technologies, Inc. and/or its subsidiaries
 */

// Dependencies supplied by the surrounding kernel tree are intentionally
// referenced here rather than reimplemented.

#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct property_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct software_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

extern "C" {
    fn fwnode_create_software_node(
        properties: *const property_entry,
        parent: *const fwnode_handle,
    ) -> *mut fwnode_handle;
    fn fwnode_remove_software_node(fwnode: *mut fwnode_handle);
    fn software_node_register(swnode: *const software_node) -> i32;
    fn software_node_unregister(swnode: *const software_node);
    fn software_node_fwnode(swnode: *const software_node) -> *mut fwnode_handle;
    fn software_node_register_node_group(nodes: *const *const software_node) -> i32;
    fn software_node_unregister_node_group(nodes: *const *const software_node);
    fn device_add_software_node(dev: *mut device, node: *const software_node) -> i32;
    fn device_remove_software_node(dev: *mut device);
    fn kunit_add_action_or_reset(
        test: *mut kunit,
        action: unsafe extern "C" fn(*mut core::ffi::c_void),
        context: *mut core::ffi::c_void,
    ) -> i32;
    fn is_err(ptr: *const core::ffi::c_void) -> bool;
    fn err_ptr(error: i32) -> *mut fwnode_handle;
    fn warn_on(condition: bool) -> bool;
}

unsafe extern "C" fn fwnode_remove_software_node_wrapper(context: *mut core::ffi::c_void) {
    fwnode_remove_software_node(context as *mut fwnode_handle);
}

/**
 * kunit_fwnode_create_software_node() - Create a kunit-managed software node
 * @test: Test context
 * @properties: Properties to use to create the new software node
 * @parent: Parent of this software node
 *
 * Create a test-managed software node and return its firmware node handle.
 * The software node is removed after the test case completes.
 *
 * Returns:
 * Firmware node handle of the newly created software node or IS_ERR() on
 * failure.
 */
#[no_mangle]
pub unsafe extern "C" fn kunit_fwnode_create_software_node(
    test: *mut kunit,
    properties: *const property_entry,
    parent: *const fwnode_handle,
) -> *mut fwnode_handle {
    let fwnode = fwnode_create_software_node(properties, parent);
    if is_err(fwnode as *const core::ffi::c_void) {
        return fwnode;
    }

    let ret = kunit_add_action_or_reset(
        test,
        fwnode_remove_software_node_wrapper,
        fwnode as *mut core::ffi::c_void,
    );
    if ret != 0 {
        return err_ptr(ret);
    }

    fwnode
}

unsafe extern "C" fn software_node_unregister_wrapper(context: *mut core::ffi::c_void) {
    software_node_unregister(context as *const software_node);
}

/**
 * kunit_software_node_register() - Register a kunit-managed software node
 * @test: Test context
 * @swnode: Software node to register
 *
 * Register a test-managed software node and return its firmware node handle.
 * The software node is unregistered after the test case completes.
 *
 * Returns:
 * Firmware node handle of the registered software node or IS_ERR() on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn kunit_software_node_register(
    test: *mut kunit,
    swnode: *const software_node,
) -> *mut fwnode_handle {
    let ret = software_node_register(swnode);
    if ret != 0 {
        return err_ptr(ret);
    }

    let fwnode = software_node_fwnode(swnode);
    if warn_on(fwnode.is_null()) {
        return err_ptr(-2); // -ENOENT
    }

    let ret = kunit_add_action_or_reset(
        test,
        software_node_unregister_wrapper,
        swnode as *mut core::ffi::c_void,
    );
    if ret != 0 {
        return err_ptr(ret);
    }

    fwnode
}

unsafe extern "C" fn software_node_unregister_node_group_wrapper(
    context: *mut core::ffi::c_void,
) {
    software_node_unregister_node_group(context as *const *const software_node);
}

/**
 * kunit_software_node_register_node_group() - Register a kunit-managed software node group
 * @test: Test context
 * @nodes: Software node group to register
 *
 * Register a test-managed software node group. The nodes are unregistered
 * after the test case completes.
 *
 * Returns:
 * 0 on success, negative error number on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn kunit_software_node_register_node_group(
    test: *mut kunit,
    nodes: *const *const software_node,
) -> i32 {
    let ret = software_node_register_node_group(nodes);
    if ret != 0 {
        return ret;
    }

    kunit_add_action_or_reset(
        test,
        software_node_unregister_node_group_wrapper,
        nodes as *mut core::ffi::c_void,
    )
}

unsafe extern "C" fn device_remove_software_node_wrapper(context: *mut core::ffi::c_void) {
    device_remove_software_node(context as *mut device);
}

/**
 * kunit_device_add_software_node() - Assign a kunit-managed software node to a device
 * @test: Test context
 * @dev: Device to assign the software node for
 * @node: The software node to assign
 *
 * Make @node the secondary firmware node of @dev. If @dev has no primary
 * firmware node, @node will become the primary node. The software node will
 * be automatically removed from @dev when the test case completes.
 *
 * Returns:
 * 0 on success, negative error number on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn kunit_device_add_software_node(
    test: *mut kunit,
    dev: *mut device,
    node: *const software_node,
) -> i32 {
    let ret = device_add_software_node(dev, node);
    if ret != 0 {
        return ret;
    }

    kunit_add_action_or_reset(
        test,
        device_remove_software_node_wrapper,
        dev as *mut core::ffi::c_void,
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
