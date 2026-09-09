// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// External dependencies are supplied by the surrounding translation unit.

const INVALID: i32 = -1;

unsafe fn dml21_allocate_memory(dml_ctx: *mut *mut dml2_context) -> bool {
    *dml_ctx = vzalloc(core::mem::size_of::<dml2_context>()) as *mut dml2_context;
    if (*dml_ctx).is_null() {
        return false;
    }

    (*dml_ctx).as_mut().unwrap().v21.dml_init.dml2_instance =
        vzalloc(core::mem::size_of::<dml2_instance>()) as *mut dml2_instance;
    if (*dml_ctx).as_ref().unwrap().v21.dml_init.dml2_instance.is_null() {
        return false;
    }

    (*dml_ctx).as_mut().unwrap().v21.mode_support.dml2_instance =
        (*dml_ctx).as_ref().unwrap().v21.dml_init.dml2_instance;
    (*dml_ctx).as_mut().unwrap().v21.mode_programming.dml2_instance =
        (*dml_ctx).as_ref().unwrap().v21.dml_init.dml2_instance;

    (*dml_ctx).as_mut().unwrap().v21.mode_support.display_config =
        &mut (*dml_ctx).as_mut().unwrap().v21.display_config;
    (*dml_ctx).as_mut().unwrap().v21.mode_programming.display_config =
        (*dml_ctx).as_ref().unwrap().v21.mode_support.display_config;

    (*dml_ctx).as_mut().unwrap().v21.mode_programming.programming =
        vzalloc(core::mem::size_of::<dml2_display_cfg_programming>())
            as *mut dml2_display_cfg_programming;

    if (*dml_ctx)
        .as_ref()
        .unwrap()
        .v21
        .mode_programming
        .programming
        .is_null()
    {
        return false;
    }

    true
}

pub unsafe fn dml21_create(
    in_dc: *const dc,
    dml_ctx: *mut *mut dml2_context,
    config: *const dml2_configuration_options,
) -> bool {
    // Allocate memory for initializing DML21 instance
    if !dml21_allocate_memory(dml_ctx) {
        return false;
    }

    dml21_init(in_dc, *dml_ctx, config);

    true
}

pub unsafe fn dml21_destroy(dml2: *mut dml2_context) {
    vfree((*dml2).v21.dml_init.dml2_instance as *mut core::ffi::c_void);
    vfree((*dml2).v21.mode_programming.programming as *mut core::ffi::c_void);
}

pub unsafe fn dml21_copy(
    dst_dml_ctx: *mut dml2_context,
    src_dml_ctx: *mut dml2_context,
) {
    // Preserve references to internals
    let dst_dml2_instance = (*dst_dml_ctx).v21.dml_init.dml2_instance;
    let dst_dml2_programming = (*dst_dml_ctx).v21.mode_programming.programming;

    // Copy context
    memcpy(
        dst_dml_ctx as *mut core::ffi::c_void,
        src_dml_ctx as *const core::ffi::c_void,
        core::mem::size_of::<dml2_context>(),
    );

    // Copy Internals
    memcpy(
        dst_dml2_instance as *mut core::ffi::c_void,
        (*src_dml_ctx).v21.dml_init.dml2_instance as *const core::ffi::c_void,
        core::mem::size_of::<dml2_instance>(),
    );
    memcpy(
        dst_dml2_programming as *mut core::ffi::c_void,
        (*src_dml_ctx).v21.mode_programming.programming as *const core::ffi::c_void,
        core::mem::size_of::<dml2_display_cfg_programming>(),
    );

    // Restore references to internals
    (*dst_dml_ctx).v21.dml_init.dml2_instance = dst_dml2_instance;

    (*dst_dml_ctx).v21.mode_support.dml2_instance = dst_dml2_instance;
    (*dst_dml_ctx).v21.mode_programming.dml2_instance = dst_dml2_instance;

    (*dst_dml_ctx).v21.mode_support.display_config = &mut (*dst_dml_ctx).v21.display_config;
    (*dst_dml_ctx).v21.mode_programming.display_config =
        (*dst_dml_ctx).v21.mode_support.display_config;

    (*dst_dml_ctx).v21.mode_programming.programming = dst_dml2_programming;

    // need to initialize copied instance for internal references to be correct
    dml2_initialize_instance(&mut (*dst_dml_ctx).v21.dml_init);
}

pub unsafe fn dml21_create_copy(
    dst_dml_ctx: *mut *mut dml2_context,
    src_dml_ctx: *mut dml2_context,
) -> bool {
    // Allocate memory for initializing DML21 instance
    if !dml21_allocate_memory(dst_dml_ctx) {
        return false;
    }

    dml21_copy(*dst_dml_ctx, src_dml_ctx);

    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
