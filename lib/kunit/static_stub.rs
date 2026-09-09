// SPDX-License-Identifier: GPL-2.0
/*
 * KUnit function redirection (static stubbing) API.
 *
 * Copyright (C) 2022, Google LLC.
 * Author: David Gow <davidgow@google.com>
 */

// Dependencies supplied by kunit/test.h, kunit/static_stub.h, and hooks-impl.h

#[repr(C)]
pub struct kunit_static_stub_ctx {
    pub real_fn_addr: *mut core::ffi::c_void,
    pub replacement_addr: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct kunit_resource {
    pub data: *mut core::ffi::c_void,
    pub free: Option<unsafe extern "C" fn(*mut kunit_resource)>,
}

#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn kfree(ptr: *mut core::ffi::c_void);
    fn kunit_find_resource(
        test: *mut kunit,
        match_fn: unsafe extern "C" fn(
            *mut kunit,
            *mut kunit_resource,
            *mut core::ffi::c_void,
        ) -> bool,
        match_data: *mut core::ffi::c_void,
    ) -> *mut kunit_resource;
    fn kunit_put_resource(res: *mut kunit_resource);
    fn kunit_remove_resource(test: *mut kunit, res: *mut kunit_resource);
    fn kunit_alloc_resource(
        test: *mut kunit,
        init: *mut core::ffi::c_void,
        free: Option<unsafe extern "C" fn(*mut kunit_resource)>,
        gfp: u32,
        data: *mut core::ffi::c_void,
    ) -> *mut kunit_resource;
    fn kmalloc_obj<T>() -> *mut T;
}

const GFP_KERNEL: u32 = 0;

/* Context for a static stub. This is stored in the resource data. */
unsafe extern "C" fn __kunit_static_stub_resource_free(res: *mut kunit_resource) {
    kfree((*res).data);
}

/* Matching function for kunit_find_resource(). match_data is real_fn_addr. */
unsafe extern "C" fn __kunit_static_stub_resource_match(
    _test: *mut kunit,
    res: *mut kunit_resource,
    match_real_fn_addr: *mut core::ffi::c_void,
) -> bool {
    /* This pointer is only valid if res is a static stub resource. */
    let ctx = (*res).data as *mut kunit_static_stub_ctx;

    /* Make sure the resource is a static stub resource. */
    if (*res).free != Some(__kunit_static_stub_resource_free) {
        return false;
    }

    (*ctx).real_fn_addr == match_real_fn_addr
}

/* Hook to return the address of the replacement function. */
pub unsafe extern "C" fn __kunit_get_static_stub_address_impl(
    test: *mut kunit,
    real_fn_addr: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    let res = kunit_find_resource(
        test,
        __kunit_static_stub_resource_match,
        real_fn_addr,
    );

    if res.is_null() {
        return core::ptr::null_mut();
    }

    let ctx = (*res).data as *mut kunit_static_stub_ctx;
    let replacement_addr = (*ctx).replacement_addr;
    kunit_put_resource(res);
    replacement_addr
}

pub unsafe extern "C" fn kunit_deactivate_static_stub(
    test: *mut kunit,
    real_fn_addr: *mut core::ffi::c_void,
) {
    assert!(!real_fn_addr.is_null(), "Tried to deactivate a NULL stub.");

    /* Look up the existing stub for this function. */
    let res = kunit_find_resource(
        test,
        __kunit_static_stub_resource_match,
        real_fn_addr,
    );

    /* Error out if the stub doesn't exist. */
    assert!(!res.is_null(), "Tried to deactivate a nonexistent stub.");

    /* Free the stub. We 'put' twice, as we got a reference
     * from kunit_find_resource()
     */
    kunit_remove_resource(test, res);
    kunit_put_resource(res);
}

/* Helper function for kunit_activate_static_stub(). The macro does
 * typechecking, so use it instead.
 */
pub unsafe extern "C" fn __kunit_activate_static_stub(
    test: *mut kunit,
    real_fn_addr: *mut core::ffi::c_void,
    replacement_addr: *mut core::ffi::c_void,
) {
    assert!(
        !real_fn_addr.is_null(),
        "Tried to activate a stub for function NULL"
    );

    /* If the replacement address is NULL, deactivate the stub. */
    if replacement_addr.is_null() {
        kunit_deactivate_static_stub(test, real_fn_addr);
        return;
    }

    /* Look up any existing stubs for this function, and replace them. */
    let res = kunit_find_resource(
        test,
        __kunit_static_stub_resource_match,
        real_fn_addr,
    );
    if !res.is_null() {
        let ctx = (*res).data as *mut kunit_static_stub_ctx;
        (*ctx).replacement_addr = replacement_addr;

        /* We got an extra reference from find_resource(), so put it. */
        kunit_put_resource(res);
    } else {
        let ctx = kmalloc_obj::<kunit_static_stub_ctx>();
        assert!(!ctx.is_null());
        (*ctx).real_fn_addr = real_fn_addr;
        (*ctx).replacement_addr = replacement_addr;
        kunit_alloc_resource(
            test,
            core::ptr::null_mut(),
            Some(__kunit_static_stub_resource_free),
            GFP_KERNEL,
            ctx as *mut core::ffi::c_void,
        );
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
