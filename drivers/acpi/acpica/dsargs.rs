// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: dsargs - Support for execution of dynamic arguments for static
 *                       objects (regions, fields, buffer fields, etc.)
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// Dependencies supplied by the ACPICA translation unit.

/* Local prototypes */
unsafe fn acpi_ds_execute_arguments(
    node: *mut acpi_namespace_node,
    scope_node: *mut acpi_namespace_node,
    aml_length: u32,
    aml_start: *mut u8,
) -> acpi_status;

/*******************************************************************************
 *
 * FUNCTION:    acpi_ds_execute_arguments
 *
 * PARAMETERS:  node                - Object NS node
 *              scope_node          - Parent NS node
 *              aml_length          - Length of executable AML
 *              aml_start           - Pointer to the AML
 *
 * RETURN:      Status.
 *
 * DESCRIPTION: Late (deferred) execution of region or field arguments
 *
 ******************************************************************************/

unsafe fn acpi_ds_execute_arguments(
    node: *mut acpi_namespace_node,
    scope_node: *mut acpi_namespace_node,
    aml_length: u32,
    aml_start: *mut u8,
) -> acpi_status {
    let mut status: acpi_status;
    let mut op: *mut acpi_parse_object;
    let mut walk_state: *mut acpi_walk_state;

    op = acpi_ps_alloc_op(AML_INT_EVAL_SUBTREE_OP, aml_start);
    if op.is_null() {
        return AE_NO_MEMORY;
    }

    (*op).common.node = scope_node;

    walk_state = acpi_ds_create_walk_state(0, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    if walk_state.is_null() {
        status = AE_NO_MEMORY;
        acpi_ps_delete_parse_tree(op);
        return status;
    }

    status = acpi_ds_init_aml_walk(
        walk_state, op, core::ptr::null_mut(), aml_start, aml_length,
        core::ptr::null_mut(), ACPI_IMODE_LOAD_PASS1,
    );
    if ACPI_FAILURE(status) {
        acpi_ds_delete_walk_state(walk_state);
        acpi_ps_delete_parse_tree(op);
        return status;
    }

    (*walk_state).parse_flags = ACPI_PARSE_DEFERRED_OP;
    (*walk_state).deferred_node = node;

    status = acpi_ps_parse_aml(walk_state);
    if ACPI_FAILURE(status) {
        acpi_ps_delete_parse_tree(op);
        return status;
    }

    (*op).common.node = node;
    acpi_ps_delete_parse_tree(op);

    op = acpi_ps_alloc_op(AML_INT_EVAL_SUBTREE_OP, aml_start);
    if op.is_null() {
        return AE_NO_MEMORY;
    }

    (*op).common.node = scope_node;

    walk_state = acpi_ds_create_walk_state(0, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    if walk_state.is_null() {
        status = AE_NO_MEMORY;
        acpi_ps_delete_parse_tree(op);
        return status;
    }

    status = acpi_ds_init_aml_walk(
        walk_state, op, core::ptr::null_mut(), aml_start, aml_length,
        core::ptr::null_mut(), ACPI_IMODE_EXECUTE,
    );
    if ACPI_FAILURE(status) {
        acpi_ds_delete_walk_state(walk_state);
        acpi_ps_delete_parse_tree(op);
        return status;
    }

    (*walk_state).deferred_node = node;
    status = acpi_ps_parse_aml(walk_state);
    acpi_ps_delete_parse_tree(op);
    status
}

pub unsafe fn acpi_ds_get_buffer_field_arguments(
    obj_desc: *mut acpi_operand_object,
) -> acpi_status {
    if (*obj_desc).common.flags & AOPOBJ_DATA_VALID != 0 {
        return AE_OK;
    }

    let extra_desc = acpi_ns_get_secondary_object(obj_desc);
    let node = (*obj_desc).buffer_field.node;
    acpi_ds_execute_arguments(
        node, (*node).parent, (*extra_desc).extra.aml_length,
        (*extra_desc).extra.aml_start,
    )
}

pub unsafe fn acpi_ds_get_bank_field_arguments(
    obj_desc: *mut acpi_operand_object,
) -> acpi_status {
    if (*obj_desc).common.flags & AOPOBJ_DATA_VALID != 0 {
        return AE_OK;
    }

    let extra_desc = acpi_ns_get_secondary_object(obj_desc);
    let node = (*obj_desc).bank_field.node;
    let status = acpi_ds_execute_arguments(
        node, (*node).parent, (*extra_desc).extra.aml_length,
        (*extra_desc).extra.aml_start,
    );
    if ACPI_FAILURE(status) {
        return status;
    }

    acpi_ut_add_address_range(
        (*obj_desc).region.space_id, (*obj_desc).region.address,
        (*obj_desc).region.length, node,
    )
}

pub unsafe fn acpi_ds_get_buffer_arguments(
    obj_desc: *mut acpi_operand_object,
) -> acpi_status {
    if (*obj_desc).common.flags & AOPOBJ_DATA_VALID != 0 {
        return AE_OK;
    }

    let node = (*obj_desc).buffer.node;
    if node.is_null() {
        return AE_AML_INTERNAL;
    }

    acpi_ds_execute_arguments(
        node, node, (*obj_desc).buffer.aml_length,
        (*obj_desc).buffer.aml_start,
    )
}

pub unsafe fn acpi_ds_get_package_arguments(
    obj_desc: *mut acpi_operand_object,
) -> acpi_status {
    if (*obj_desc).common.flags & AOPOBJ_DATA_VALID != 0 {
        return AE_OK;
    }

    let node = (*obj_desc).package.node;
    if node.is_null() {
        return AE_AML_INTERNAL;
    }

    acpi_ds_execute_arguments(
        node, node, (*obj_desc).package.aml_length,
        (*obj_desc).package.aml_start,
    )
}

pub unsafe fn acpi_ds_get_region_arguments(
    obj_desc: *mut acpi_operand_object,
) -> acpi_status {
    if (*obj_desc).region.flags & AOPOBJ_DATA_VALID != 0 {
        return AE_OK;
    }

    let extra_desc = acpi_ns_get_secondary_object(obj_desc);
    if extra_desc.is_null() {
        return AE_NOT_EXIST;
    }

    let node = (*obj_desc).region.node;
    let status = acpi_ds_execute_arguments(
        node, (*extra_desc).extra.scope_node, (*extra_desc).extra.aml_length,
        (*extra_desc).extra.aml_start,
    );
    if ACPI_FAILURE(status) {
        return status;
    }

    acpi_ut_add_address_range(
        (*obj_desc).region.space_id, (*obj_desc).region.address,
        (*obj_desc).region.length, node,
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
