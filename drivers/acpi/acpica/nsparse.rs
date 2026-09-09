// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: nsparse - namespace interface to AML parser
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 ******************************************************************************/

// Dependencies supplied by the ACPI implementation are intentionally external.

pub unsafe fn acpi_ns_execute_table(
    table_index: u32,
    start_node: *mut acpi_namespace_node,
) -> acpi_status {
    let mut status: acpi_status;
    let mut table: *mut acpi_table_header = core::ptr::null_mut();
    let mut owner_id: acpi_owner_id = 0;
    let mut info: *mut acpi_evaluate_info = core::ptr::null_mut();
    let mut aml_length: u32;
    let mut aml_start: *mut u8;
    let mut method_obj: *mut acpi_operand_object = core::ptr::null_mut();

    status = acpi_get_table_by_index(table_index, &mut table);
    if acpi_failure(status) {
        return status;
    }

    if (*table).length < core::mem::size_of::<acpi_table_header>() as u32 {
        return AE_BAD_HEADER;
    }

    aml_start = (table as *mut u8).add(core::mem::size_of::<acpi_table_header>());
    aml_length = (*table).length - core::mem::size_of::<acpi_table_header>() as u32;

    status = acpi_tb_get_owner_id(table_index, &mut owner_id);
    if acpi_failure(status) {
        return status;
    }

    method_obj = acpi_ut_create_internal_object(ACPI_TYPE_METHOD);
    if method_obj.is_null() {
        return AE_NO_MEMORY;
    }

    info = acpi_allocate_zeroed(core::mem::size_of::<acpi_evaluate_info>()) as *mut acpi_evaluate_info;
    if info.is_null() {
        status = AE_NO_MEMORY;
        acpi_ut_remove_reference(method_obj);
        return status;
    }

    (*method_obj).method.aml_start = aml_start;
    (*method_obj).method.aml_length = aml_length;
    (*method_obj).method.owner_id = owner_id;
    (*method_obj).method.info_flags |= ACPI_METHOD_MODULE_LEVEL;

    (*info).pass_number = ACPI_IMODE_EXECUTE;
    (*info).node = start_node;
    (*info).obj_desc = method_obj;
    (*info).node_flags = (*info).node.as_ref().unwrap().flags;
    (*info).full_pathname = acpi_ns_get_normalized_pathname((*info).node, TRUE);
    if (*info).full_pathname.is_null() {
        status = AE_NO_MEMORY;
    } else {
        status = acpi_ps_execute_table(info);
    }

    if !(*info).full_pathname.is_null() {
        acpi_free((*info).full_pathname as *mut core::ffi::c_void);
        (*info).full_pathname = core::ptr::null_mut();
    }
    acpi_free(info as *mut core::ffi::c_void);
    acpi_ut_remove_reference(method_obj);
    status
}

pub unsafe fn acpi_ns_one_complete_parse(
    pass_number: u32,
    table_index: u32,
    start_node: *mut acpi_namespace_node,
) -> acpi_status {
    let mut parse_root: *mut acpi_parse_object;
    let mut status: acpi_status;
    let mut aml_length: u32;
    let mut aml_start: *mut u8;
    let mut walk_state: *mut acpi_walk_state;
    let mut table: *mut acpi_table_header = core::ptr::null_mut();
    let mut owner_id: acpi_owner_id = 0;

    status = acpi_get_table_by_index(table_index, &mut table);
    if acpi_failure(status) { return status; }
    if (*table).length < core::mem::size_of::<acpi_table_header>() as u32 { return AE_BAD_HEADER; }
    aml_start = (table as *mut u8).add(core::mem::size_of::<acpi_table_header>());
    aml_length = (*table).length - core::mem::size_of::<acpi_table_header>() as u32;
    status = acpi_tb_get_owner_id(table_index, &mut owner_id);
    if acpi_failure(status) { return status; }

    parse_root = acpi_ps_create_scope_op(aml_start);
    if parse_root.is_null() { return AE_NO_MEMORY; }
    walk_state = acpi_ds_create_walk_state(owner_id, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    if walk_state.is_null() {
        acpi_ps_free_op(parse_root);
        return AE_NO_MEMORY;
    }
    status = acpi_ds_init_aml_walk(walk_state, parse_root, core::ptr::null_mut(), aml_start, aml_length, core::ptr::null_mut(), pass_number as u8);
    if acpi_failure(status) {
        acpi_ds_delete_walk_state(walk_state);
        acpi_ps_delete_parse_tree(parse_root);
        return status;
    }
    if acpi_compare_nameseg((*table).signature.as_ptr(), ACPI_SIG_OSDT) && pass_number == ACPI_IMODE_LOAD_PASS1 {
        (*walk_state).namespace_override = TRUE;
    }
    if !start_node.is_null() && start_node != acpi_gbl_root_node {
        status = acpi_ds_scope_stack_push(start_node, ACPI_TYPE_METHOD, walk_state);
        if acpi_failure(status) {
            acpi_ds_delete_walk_state(walk_state);
            acpi_ps_delete_parse_tree(parse_root);
            return status;
        }
    }
    acpi_ex_enter_interpreter();
    status = acpi_ps_parse_aml(walk_state);
    acpi_ex_exit_interpreter();
    acpi_ps_delete_parse_tree(parse_root);
    status
}

pub unsafe fn acpi_ns_parse_table(
    table_index: u32,
    start_node: *mut acpi_namespace_node,
) -> acpi_status {
    acpi_ns_execute_table(table_index, start_node)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
