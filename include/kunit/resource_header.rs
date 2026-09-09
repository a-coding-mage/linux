/* SPDX-License-Identifier: GPL-2.0 */
/*
 * KUnit resource API for test managed resources (allocations, etc.).
 *
 * Copyright (C) 2022, Google LLC.
 * Author: Daniel Latypov <dlatypov@google.com>
 */

// #include <kunit/test.h>
// #include <linux/kref.h>
// #include <linux/list.h>
// #include <linux/slab.h>
// #include <linux/spinlock.h>

pub struct kunit_resource;

pub type kunit_resource_init_t = unsafe extern "C" fn(*mut kunit_resource, *mut core::ffi::c_void) -> i32;
pub type kunit_resource_free_t = unsafe extern "C" fn(*mut kunit_resource);

/**
 * struct kunit_resource - represents a *test managed resource*
 * @data: for the user to store arbitrary data.
 * @name: optional name
 * @free: a user supplied function to free the resource.
 */
#[repr(C)]
pub struct kunit_resource {
    pub data: *mut core::ffi::c_void,
    pub name: *const core::ffi::c_char,
    pub free: Option<kunit_resource_free_t>,
    // private: internal use only.
    pub refcount: kref,
    pub node: list_head,
    pub should_kfree: bool,
}

pub unsafe fn kunit_get_resource(res: *mut kunit_resource) {
    unsafe { kref_get(&mut (*res).refcount) };
}

/* Called when refcount reaches zero via kunit_put_resource(); */
pub unsafe extern "C" fn kunit_release_resource(kref: *mut kref) {
    let res = unsafe { container_of!(kref, kunit_resource, refcount) };

    if let Some(free) = unsafe { (*res).free } {
        unsafe { free(res) };
    }
    if unsafe { (*res).should_kfree } {
        unsafe { kfree(res as *mut core::ffi::c_void) };
    }
}

pub unsafe fn kunit_put_resource(res: *mut kunit_resource) {
    unsafe { kref_put(&mut (*res).refcount, Some(kunit_release_resource)) };
}

pub unsafe extern "C" fn __kunit_add_resource(
    test: *mut kunit,
    init: Option<kunit_resource_init_t>,
    free: Option<kunit_resource_free_t>,
    res: *mut kunit_resource,
    data: *mut core::ffi::c_void,
) -> i32;

pub unsafe fn kunit_add_resource(
    test: *mut kunit,
    init: Option<kunit_resource_init_t>,
    free: Option<kunit_resource_free_t>,
    res: *mut kunit_resource,
    data: *mut core::ffi::c_void,
) -> i32 {
    unsafe { (*res).should_kfree = false; }
    unsafe { __kunit_add_resource(test, init, free, res, data) }
}

pub unsafe fn kunit_find_named_resource(test: *mut kunit, name: *const core::ffi::c_char) -> *mut kunit_resource;

pub unsafe fn kunit_add_named_resource(
    test: *mut kunit,
    init: Option<kunit_resource_init_t>,
    free: Option<kunit_resource_free_t>,
    res: *mut kunit_resource,
    name: *const core::ffi::c_char,
    data: *mut core::ffi::c_void,
) -> i32 {
    if name.is_null() { return -EINVAL; }
    let existing = unsafe { kunit_find_named_resource(test, name) };
    if !existing.is_null() {
        unsafe { kunit_put_resource(existing) };
        return -EEXIST;
    }
    unsafe { (*res).name = name; (*res).should_kfree = false; }
    unsafe { __kunit_add_resource(test, init, free, res, data) }
}

pub unsafe fn kunit_alloc_and_get_resource(
    test: *mut kunit, init: Option<kunit_resource_init_t>, free: Option<kunit_resource_free_t>,
    internal_gfp: gfp_t, context: *mut core::ffi::c_void,
) -> *mut kunit_resource {
    let res = unsafe { kzalloc_obj::<kunit_resource>(internal_gfp) };
    if res.is_null() { return core::ptr::null_mut(); }
    unsafe { (*res).should_kfree = true; }
    if unsafe { __kunit_add_resource(test, init, free, res, context) } == 0 {
        unsafe { kunit_get_resource(res) };
        return res;
    }
    core::ptr::null_mut()
}

pub unsafe fn kunit_alloc_resource(
    test: *mut kunit, init: Option<kunit_resource_init_t>, free: Option<kunit_resource_free_t>,
    internal_gfp: gfp_t, context: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    let res = unsafe { kzalloc_obj::<kunit_resource>(internal_gfp) };
    if res.is_null() { return core::ptr::null_mut(); }
    unsafe { (*res).should_kfree = true; }
    if unsafe { __kunit_add_resource(test, init, free, res, context) } == 0 {
        return unsafe { (*res).data };
    }
    core::ptr::null_mut()
}

pub type kunit_resource_match_t = unsafe extern "C" fn(*mut kunit, *mut kunit_resource, *mut core::ffi::c_void) -> bool;

pub unsafe fn kunit_resource_name_match(_test: *mut kunit, res: *mut kunit_resource, match_name: *mut core::ffi::c_void) -> bool {
    unsafe { !(*res).name.is_null() && strcmp((*res).name, match_name as *const core::ffi::c_char) == 0 }
}

pub unsafe fn kunit_find_resource(test: *mut kunit, match_fn: kunit_resource_match_t, match_data: *mut core::ffi::c_void) -> *mut kunit_resource;
pub unsafe fn kunit_destroy_resource(test: *mut kunit, match_fn: kunit_resource_match_t, match_data: *mut core::ffi::c_void) -> i32;
pub unsafe fn kunit_destroy_named_resource(test: *mut kunit, name: *const core::ffi::c_char) -> i32 { unsafe { kunit_destroy_resource(test, kunit_resource_name_match, name as *mut _) } }
pub unsafe fn kunit_remove_resource(test: *mut kunit, res: *mut kunit_resource);

pub type kunit_action_t = unsafe extern "C" fn(*mut core::ffi::c_void);

// KUNIT_DEFINE_ACTION_WRAPPER defines a CFI-safe wrapper around a pointer-sized callback.
#[macro_export]
macro_rules! KUNIT_DEFINE_ACTION_WRAPPER {
    ($wrapper:ident, $orig:path, $arg_type:ty) => {
        unsafe extern "C" fn $wrapper(in_: *mut core::ffi::c_void) {
            let arg = in_ as $arg_type;
            unsafe { $orig(arg); }
        }
    };
}

pub unsafe fn kunit_add_action(test: *mut kunit, action: kunit_action_t, ctx: *mut core::ffi::c_void) -> i32;
pub unsafe fn kunit_add_action_or_reset(test: *mut kunit, action: kunit_action_t, ctx: *mut core::ffi::c_void) -> i32;
pub unsafe fn kunit_remove_action(test: *mut kunit, action: kunit_action_t, ctx: *mut core::ffi::c_void);
pub unsafe fn kunit_release_action(test: *mut kunit, action: kunit_action_t, ctx: *mut core::ffi::c_void);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
