// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: psxface - Parser external interfaces
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// Dependencies supplied by the surrounding ACPICA translation unit.

// #define _COMPONENT ACPI_PARSER
// ACPI_MODULE_NAME("psxface")

/* Local Prototypes */
unsafe fn acpi_ps_update_parameter_list(info: *mut acpi_evaluate_info, action: u16);

pub unsafe fn acpi_debug_trace(
    name: *const core::ffi::c_char,
    debug_level: u32,
    debug_layer: u32,
    flags: u32,
) -> acpi_status {
    let mut status: acpi_status;

    status = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE);
    if ACPI_FAILURE(status) {
        return status;
    }

    acpi_gbl_trace_method_name = name;
    acpi_gbl_trace_flags = flags;
    acpi_gbl_trace_dbg_level = debug_level;
    acpi_gbl_trace_dbg_layer = debug_layer;
    status = AE_OK;

    let _ = acpi_ut_release_mutex(ACPI_MTX_NAMESPACE);
    status
}

pub unsafe fn acpi_ps_execute_method(info: *mut acpi_evaluate_info) -> acpi_status {
    let mut status: acpi_status;
    let mut op: *mut acpi_parse_object;
    let mut walk_state: *mut acpi_walk_state;

    macro_rules! goto_cleanup_method { ($label:ident) => {{
        acpi_ps_delete_parse_tree(op);
        acpi_ps_update_parameter_list(info, REF_DECREMENT);
        return status;
    }}; }

    acpi_tb_check_dsdt_header();

    if info.is_null() || (*info).node.is_null() {
        return AE_NULL_ENTRY;
    }

    status = acpi_ds_begin_method_execution((*info).node, (*info).obj_desc, core::ptr::null_mut());
    if ACPI_FAILURE(status) {
        return status;
    }

    /* The caller owns the parameters, so give each one an extra reference. */
    acpi_ps_update_parameter_list(info, REF_INCREMENT);

    op = acpi_ps_create_scope_op((*(*info).obj_desc).method.aml_start);
    if op.is_null() {
        status = AE_NO_MEMORY;
        goto_cleanup_method!(cleanup);
    }

    (*info).pass_number = ACPI_IMODE_EXECUTE;
    walk_state = acpi_ds_create_walk_state(
        (*(*info).obj_desc).method.owner_id,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        core::ptr::null_mut(),
    );
    if walk_state.is_null() {
        status = AE_NO_MEMORY;
        goto_cleanup_method!(cleanup);
    }

    status = acpi_ds_init_aml_walk(
        walk_state,
        op,
        (*info).node,
        (*(*info).obj_desc).method.aml_start,
        (*(*info).obj_desc).method.aml_length,
        info,
        (*info).pass_number,
    );
    if ACPI_FAILURE(status) {
        acpi_ds_delete_walk_state(walk_state);
        goto_cleanup_method!(cleanup);
    }

    (*walk_state).method_pathname = (*info).full_pathname;
    (*walk_state).method_is_nested = FALSE;

    if (*(*info).obj_desc).method.info_flags & ACPI_METHOD_MODULE_LEVEL != 0 {
        (*walk_state).parse_flags |= ACPI_PARSE_MODULE_LEVEL;
    }

    if (*(*info).obj_desc).method.info_flags & ACPI_METHOD_INTERNAL_ONLY != 0 {
        status = ((*(*info).obj_desc).method.dispatch.implementation)(walk_state);
        (*info).return_object = (*walk_state).return_desc;
        acpi_ds_scope_stack_clear(walk_state);
        acpi_ps_cleanup_scope(&mut (*walk_state).parser_state);
        acpi_ds_terminate_control_method((*walk_state).method_desc, walk_state);
        acpi_ds_delete_walk_state(walk_state);
        goto_cleanup_method!(cleanup);
    }

    if acpi_gbl_enable_interpreter_slack {
        (*walk_state).implicit_return_obj = acpi_ut_create_integer_object(0u64);
        if (*walk_state).implicit_return_obj.is_null() {
            status = AE_NO_MEMORY;
            acpi_ds_delete_walk_state(walk_state);
            goto_cleanup_method!(cleanup);
        }
    }

    status = acpi_ps_parse_aml(walk_state);

    cleanup: {
        acpi_ps_delete_parse_tree(op);
        acpi_ps_update_parameter_list(info, REF_DECREMENT);
        if ACPI_FAILURE(status) {
            return status;
        }
        if !(*info).return_object.is_null() {
            status = AE_CTRL_RETURN_VALUE;
        }
        status
    }
}

pub unsafe fn acpi_ps_execute_table(info: *mut acpi_evaluate_info) -> acpi_status {
    let mut status: acpi_status;
    let mut op: *mut acpi_parse_object = core::ptr::null_mut();
    let mut walk_state: *mut acpi_walk_state = core::ptr::null_mut();

    macro_rules! goto_cleanup_table { ($label:ident) => {{
        if !walk_state.is_null() { acpi_ds_delete_walk_state(walk_state); }
        if !op.is_null() { acpi_ps_delete_parse_tree(op); }
        return status;
    }}; }

    op = acpi_ps_create_scope_op((*(*info).obj_desc).method.aml_start);
    if op.is_null() {
        status = AE_NO_MEMORY;
        goto_cleanup_table!(cleanup);
    }

    walk_state = acpi_ds_create_walk_state(
        (*(*info).obj_desc).method.owner_id,
        core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    );
    if walk_state.is_null() {
        status = AE_NO_MEMORY;
        goto_cleanup_table!(cleanup);
    }

    status = acpi_ds_init_aml_walk(
        walk_state, op, (*info).node, (*(*info).obj_desc).method.aml_start,
        (*(*info).obj_desc).method.aml_length, info, (*info).pass_number,
    );
    if ACPI_FAILURE(status) {
        goto_cleanup_table!(cleanup);
    }

    (*walk_state).method_pathname = (*info).full_pathname;
    (*walk_state).method_is_nested = FALSE;
    if (*(*info).obj_desc).method.info_flags & ACPI_METHOD_MODULE_LEVEL != 0 {
        (*walk_state).parse_flags |= ACPI_PARSE_MODULE_LEVEL;
    }
    if !(*info).node.is_null() && (*info).node != acpi_gbl_root_node {
        status = acpi_ds_scope_stack_push((*info).node, ACPI_TYPE_METHOD, walk_state);
        if ACPI_FAILURE(status) {
            goto_cleanup_table!(cleanup);
        }
    }
    acpi_ex_enter_interpreter();
    status = acpi_ps_parse_aml(walk_state);
    acpi_ex_exit_interpreter();
    walk_state = core::ptr::null_mut();

    cleanup: {
        if !walk_state.is_null() { acpi_ds_delete_walk_state(walk_state); }
        if !op.is_null() { acpi_ps_delete_parse_tree(op); }
        status
    }
}

unsafe fn acpi_ps_update_parameter_list(info: *mut acpi_evaluate_info, action: u16) {
    if !(*info).parameters.is_null() {
        let mut i: u32 = 0;
        while !(*(*info).parameters.add(i as usize)).is_null() {
            let _ = acpi_ut_update_object_reference(*(*info).parameters.add(i as usize), action);
            i = i.wrapping_add(1);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
