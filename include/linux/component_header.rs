/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

use core::ffi::c_void;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

/**
 * struct component_ops - callbacks for component drivers
 *
 * Components are registered with component_add() and unregistered with
 * component_del().
 */
#[repr(C)]
pub struct component_ops {
    pub bind: Option<unsafe extern "C" fn(*mut device, *mut device, *mut c_void) -> i32>,
    pub unbind: Option<unsafe extern "C" fn(*mut device, *mut device, *mut c_void)>,
}

extern "C" {
    pub fn component_add(dev: *mut device, ops: *const component_ops) -> i32;
    pub fn component_add_typed(
        dev: *mut device,
        ops: *const component_ops,
        subcomponent: i32,
    ) -> i32;
    pub fn component_del(dev: *mut device, ops: *const component_ops);

    pub fn component_bind_all(parent: *mut device, data: *mut c_void) -> i32;
    pub fn component_unbind_all(parent: *mut device, data: *mut c_void);
}

#[repr(C)]
pub struct aggregate_device {
    _private: [u8; 0],
}

/**
 * struct component_master_ops - callback for the aggregate driver
 *
 * Aggregate drivers are registered with component_master_add_with_match() and
 * unregistered with component_master_del().
 */
#[repr(C)]
pub struct component_master_ops {
    pub bind: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub unbind: Option<unsafe extern "C" fn(*mut device)>,
}

/* A set helper functions for component compare/release */
extern "C" {
    pub fn component_compare_of(dev: *mut device, data: *mut c_void) -> i32;
    pub fn component_release_of(dev: *mut device, data: *mut c_void);
    pub fn component_compare_dev(dev: *mut device, data: *mut c_void) -> i32;
    pub fn component_compare_dev_name(dev: *mut device, data: *mut c_void) -> i32;

    pub fn component_master_del(dev: *mut device, ops: *const component_master_ops);
    pub fn component_master_is_bound(
        parent: *mut device,
        ops: *const component_master_ops,
    ) -> bool;
}

#[repr(C)]
pub struct component_match {
    _private: [u8; 0],
}

extern "C" {
    pub fn component_master_add_with_match(
        dev: *mut device,
        ops: *const component_master_ops,
        match_: *mut component_match,
    ) -> i32;
    pub fn component_match_add_release(
        parent: *mut device,
        matchptr: *mut *mut component_match,
        release: Option<unsafe extern "C" fn(*mut device, *mut c_void)>,
        compare: Option<unsafe extern "C" fn(*mut device, *mut c_void) -> i32>,
        compare_data: *mut c_void,
    );
    pub fn component_match_add_typed(
        parent: *mut device,
        matchptr: *mut *mut component_match,
        compare_typed: Option<unsafe extern "C" fn(*mut device, i32, *mut c_void) -> i32>,
        compare_data: *mut c_void,
    );
}

/**
 * component_match_add - add a component match entry
 * @parent: device with the aggregate driver
 * @matchptr: pointer to the list of component matches
 * @compare: compare function to match against all components
 * @compare_data: opaque pointer passed to the @compare function
 *
 * Adds a new component match to the list stored in @matchptr, which the @parent
 * aggregate driver needs to function. The list of component matches pointed to
 * by @matchptr must be initialized to NULL before adding the first match. This
 * only matches against components added with component_add().
 *
 * The allocated match list in @matchptr is automatically released using devm
 * actions.
 *
 * See also component_match_add_release() and component_match_add_typed().
 */
#[inline]
pub unsafe fn component_match_add(
    parent: *mut device,
    matchptr: *mut *mut component_match,
    compare: Option<unsafe extern "C" fn(*mut device, *mut c_void) -> i32>,
    compare_data: *mut c_void,
) {
    component_match_add_release(parent, matchptr, None, compare, compare_data);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
