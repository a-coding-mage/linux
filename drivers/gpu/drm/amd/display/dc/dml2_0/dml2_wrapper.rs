// SPDX-License-Identifier: MIT
/*
 * Copyright 2025 Advanced Micro Devices, Inc.
 *
 * Authors: AMD
 */

// Dependencies supplied by the corresponding DML2 headers and implementation.

#[cfg(not(DC_RUN_WITH_PREEMPTION_ENABLED))]
macro_rules! dc_run_with_preemption_enabled {
    ($code:expr) => { $code };
}

extern "C" {
    fn vzalloc(size: usize) -> *mut core::ffi::c_void;
    fn vfree(ptr: *mut core::ffi::c_void);

    fn dml2_apply_debug_options(in_dc: *const dc, dml2: *mut dml2_context);
    fn dml21_validate(
        in_dc: *const dc,
        context: *mut dc_state,
        dml2: *mut dml2_context,
        validate_mode: dc_validate_mode,
    ) -> bool;
    fn dml2_validate_only(context: *mut dc_state, validate_mode: dc_validate_mode) -> bool;
    fn dml2_validate_and_build_resource(
        in_dc: *const dc,
        context: *mut dc_state,
        validate_mode: dc_validate_mode,
    ) -> bool;
    fn dml21_reinit(
        in_dc: *const dc,
        dml2: *mut dml2_context,
        config: *const dml2_configuration_options,
    );
    fn dml21_create(
        in_dc: *const dc,
        dml2: *mut *mut dml2_context,
        config: *const dml2_configuration_options,
    ) -> bool;
    fn dml21_destroy(dml2: *mut dml2_context);
    fn initialize_dml2_ip_params(
        dml2: *mut dml2_context,
        in_dc: *const dc,
        ip: *mut dml2_ip_params,
    );
    fn initialize_dml2_soc_bbox(
        dml2: *mut dml2_context,
        in_dc: *const dc,
        soc: *mut dml2_soc_bounding_box,
    );
    fn initialize_dml2_soc_states(
        dml2: *mut dml2_context,
        in_dc: *const dc,
        soc: *mut dml2_soc_bounding_box,
        states: *mut dml2_soc_states,
    );
}

pub unsafe fn dml2_allocate_memory() -> *mut dml2_context {
    let mut dml2: *mut dml2_context = core::ptr::null_mut();

    #[cfg(DC_RUN_WITH_PREEMPTION_ENABLED)]
    {
        dml2 = vzalloc(core::mem::size_of::<dml2_context>()) as *mut dml2_context;
    }
    #[cfg(not(DC_RUN_WITH_PREEMPTION_ENABLED))]
    {
        dc_run_with_preemption_enabled! {
            dml2 = vzalloc(core::mem::size_of::<dml2_context>()) as *mut dml2_context
        };
    }
    dml2
}

pub unsafe fn dml2_validate(
    in_dc: *const dc,
    context: *mut dc_state,
    dml2: *mut dml2_context,
    validate_mode: dc_validate_mode,
) -> bool {
    let mut out = false;

    if dml2.is_null() {
        return false;
    }
    dml2_apply_debug_options(in_dc, dml2);

    /* DML2.1 validation path */
    if (*dml2).architecture == dml2_architecture_21 {
        out = dml21_validate(in_dc, context, dml2, validate_mode);
        return out;
    }

    /* Use dml_validate_only for DC_VALIDATE_MODE_ONLY and DC_VALIDATE_MODE_AND_STATE_INDEX path */
    if validate_mode != DC_VALIDATE_MODE_AND_PROGRAMMING {
        out = dml2_validate_only(context, validate_mode);
    } else {
        out = dml2_validate_and_build_resource(in_dc, context, validate_mode);
    }

    out
}

unsafe fn dml2_init(
    in_dc: *const dc,
    config: *const dml2_configuration_options,
    dml2: *mut *mut dml2_context,
) {
    if (*in_dc).debug.using_dml21 && (*in_dc).ctx.dce_version >= DCN_VERSION_4_01 {
        dml21_reinit(in_dc, *dml2, config);
        return;
    }

    // Store config options
    (*dml2).config = core::ptr::read(config);

    match (*in_dc).ctx.dce_version {
        DCN_VERSION_3_5 => (*dml2).v20.dml_core_ctx.project = dml_project_dcn35,
        DCN_VERSION_3_51 => (*dml2).v20.dml_core_ctx.project = dml_project_dcn351,
        DCN_VERSION_3_6 => (*dml2).v20.dml_core_ctx.project = dml_project_dcn36,
        DCN_VERSION_3_2 => (*dml2).v20.dml_core_ctx.project = dml_project_dcn32,
        DCN_VERSION_3_21 => (*dml2).v20.dml_core_ctx.project = dml_project_dcn321,
        DCN_VERSION_4_01 => (*dml2).v20.dml_core_ctx.project = dml_project_dcn401,
        _ => (*dml2).v20.dml_core_ctx.project = dml_project_default,
    }

    initialize_dml2_ip_params(dml2, in_dc, &mut (*dml2).v20.dml_core_ctx.ip);
    initialize_dml2_soc_bbox(dml2, in_dc, &mut (*dml2).v20.dml_core_ctx.soc);
    initialize_dml2_soc_states(
        dml2,
        in_dc,
        &mut (*dml2).v20.dml_core_ctx.soc,
        &mut (*dml2).v20.dml_core_ctx.states,
    );
}

pub unsafe fn dml2_create(
    in_dc: *const dc,
    config: *const dml2_configuration_options,
    dml2: *mut *mut dml2_context,
) -> bool {
    // TODO : Temporarily add DCN_VERSION_3_2 for N-1 validation. Remove DCN_VERSION_3_2 after N-1 validation phase is complete.
    if (*in_dc).debug.using_dml21 && (*in_dc).ctx.dce_version >= DCN_VERSION_4_01 {
        return dml21_create(in_dc, dml2, config);
    }

    // Allocate Mode Lib Ctx
    *dml2 = dml2_allocate_memory();

    if (*dml2).is_null() {
        return false;
    }

    dml2_init(in_dc, config, dml2);
    true
}

pub unsafe fn dml2_destroy(dml2: *mut dml2_context) {
    if dml2.is_null() {
        return;
    }

    if (*dml2).architecture == dml2_architecture_21 {
        dml21_destroy(dml2);
    }

    #[cfg(DC_RUN_WITH_PREEMPTION_ENABLED)]
    {
        vfree(dml2 as *mut core::ffi::c_void);
    }
    #[cfg(not(DC_RUN_WITH_PREEMPTION_ENABLED))]
    {
        dc_run_with_preemption_enabled! {
            vfree(dml2 as *mut core::ffi::c_void)
        };
    }
}

pub unsafe fn dml2_reinit(
    in_dc: *const dc,
    config: *const dml2_configuration_options,
    dml2: *mut *mut dml2_context,
) {
    if (*in_dc).debug.using_dml21 && (*in_dc).ctx.dce_version >= DCN_VERSION_4_01 {
        dml21_reinit(in_dc, *dml2, config);
        return;
    }

    dml2_init(in_dc, config, dml2);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
