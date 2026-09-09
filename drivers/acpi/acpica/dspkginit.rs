// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: dspkginit - Completion of deferred package initialization
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 ******************************************************************************/

// Dependencies supplied by the ACPICA translation unit.

/*******************************************************************************
 *
 * FUNCTION:    acpi_ds_build_internal_package_obj
 *
 ******************************************************************************/

unsafe fn acpi_ds_build_internal_package_obj(
    walk_state: *mut acpi_walk_state,
    op: *mut acpi_parse_object,
    element_count: u32,
    obj_desc_ptr: *mut *mut acpi_operand_object,
) -> acpi_status {
    let mut arg: *mut acpi_parse_object;
    let mut parent: *mut acpi_parse_object;
    let mut obj_desc: *mut acpi_operand_object = core::ptr::null_mut();
    let mut status: acpi_status = AE_OK;
    let mut module_level_code: u8 = FALSE;
    let mut reference_count: u16;
    let mut index: u32;
    let mut i: u32 = 0;

    // ACPI_FUNCTION_TRACE(ds_build_internal_package_obj);

    if ((*walk_state).parse_flags & ACPI_PARSE_MODULE_LEVEL) != 0 {
        module_level_code = TRUE;
    }

    parent = (*op).common.parent;
    while (*parent).common.aml_opcode == AML_PACKAGE_OP
        || (*parent).common.aml_opcode == AML_VARIABLE_PACKAGE_OP
    {
        parent = (*parent).common.parent;
    }

    obj_desc = *obj_desc_ptr;
    if obj_desc.is_null() {
        obj_desc = acpi_ut_create_internal_object(ACPI_TYPE_PACKAGE);
        *obj_desc_ptr = obj_desc;
        if obj_desc.is_null() {
            return AE_NO_MEMORY;
        }
        (*obj_desc).package.node = (*parent).common.node;
    }

    if ((*obj_desc).package.flags & AOPOBJ_DATA_VALID) != 0 {
        return AE_OK;
    }

    if (*obj_desc).package.elements.is_null() {
        (*obj_desc).package.elements = acpi_allocate_zeroed(
            ((element_count as usize) + 1) * core::mem::size_of::<*mut core::ffi::c_void>(),
        ) as *mut *mut acpi_operand_object;
        if (*obj_desc).package.elements.is_null() {
            acpi_ut_delete_object_desc(obj_desc);
            return AE_NO_MEMORY;
        }
        (*obj_desc).package.count = element_count;
    }

    arg = (*op).common.value.arg;
    arg = (*arg).common.next;

    if module_level_code != FALSE {
        (*obj_desc).package.aml_start = (*walk_state).aml;
        (*obj_desc).package.aml_length = 0;
        // ACPI_DEBUG_PRINT_RAW((ACPI_DB_PARSE, ...));
    }

    while !arg.is_null() && i < element_count {
        if (*arg).common.aml_opcode == AML_INT_RETURN_VALUE_OP {
            if (*arg).common.node.is_null() {
                // ACPI_EXCEPTION((AE_INFO, AE_SUPPORT, ...));
                acpi_ut_remove_reference((*walk_state).results.results.obj_desc[0]);
                return AE_SUPPORT;
            }

            if (*(*arg).common.node).type == ACPI_TYPE_METHOD {
                (*arg).common.aml_opcode = AML_INT_NAMEPATH_OP;
                status = acpi_ds_build_internal_object(
                    walk_state,
                    arg,
                    (*obj_desc).package.elements.add(i as usize),
                );
            } else {
                *(*obj_desc).package.elements.add(i as usize) =
                    (*arg).common.node as *mut acpi_operand_object;
            }
        } else {
            status = acpi_ds_build_internal_object(
                walk_state,
                arg,
                (*obj_desc).package.elements.add(i as usize),
            );
            if status == AE_NOT_FOUND {
                // ACPI_ERROR((AE_INFO, "%-48s", "****DS namepath not found"));
            }

            if module_level_code == FALSE {
                acpi_ds_init_package_element(
                    0,
                    *(*obj_desc).package.elements.add(i as usize),
                    core::ptr::null_mut(),
                    (*obj_desc).package.elements.add(i as usize) as *mut core::ffi::c_void,
                );
            }
        }

        if !(*obj_desc_ptr).is_null() {
            reference_count = (*(*obj_desc_ptr)).common.reference_count;
            if reference_count > 1 {
                index = 0;
                while index < reference_count as u32 - 1 {
                    acpi_ut_add_reference(*(*obj_desc).package.elements.add(i as usize));
                    index += 1;
                }
            }
        }

        arg = (*arg).common.next;
        i += 1;
    }

    if !arg.is_null() {
        while !arg.is_null() {
            if !(*arg).common.node.is_null() {
                acpi_ut_remove_reference((*arg).common.node as *mut acpi_operand_object);
                (*arg).common.node = core::ptr::null_mut();
            }
            i += 1;
            arg = (*arg).common.next;
        }
        // ACPI_INFO(("Actual Package length ..."));
    } else if i < element_count {
        // ACPI_DEBUG_PRINT_RAW((ACPI_DB_INFO, ...));
    }

    if module_level_code == FALSE {
        (*obj_desc).package.flags |= AOPOBJ_DATA_VALID;
    }

    (*op).common.node = obj_desc as *mut acpi_namespace_node;
    status
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ds_init_package_element
 *
 ******************************************************************************/

unsafe fn acpi_ds_init_package_element(
    _object_type: u8,
    source_object: *mut acpi_operand_object,
    state: *mut acpi_generic_state,
    context: *mut core::ffi::c_void,
) -> acpi_status {
    let element_ptr: *mut *mut acpi_operand_object;

    if source_object.is_null() {
        return AE_OK;
    }

    if !context.is_null() {
        element_ptr = context as *mut *mut acpi_operand_object;
    } else {
        element_ptr = (*state).pkg.this_target_obj;
    }

    if (*source_object).common.type_ == ACPI_TYPE_LOCAL_REFERENCE {
        acpi_ds_resolve_package_element(element_ptr);
    } else if (*source_object).common.type_ == ACPI_TYPE_PACKAGE {
        (*source_object).package.flags |= AOPOBJ_DATA_VALID;
    }

    AE_OK
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ds_resolve_package_element
 *
 ******************************************************************************/

unsafe fn acpi_ds_resolve_package_element(
    element_ptr: *mut *mut acpi_operand_object,
) {
    let mut status: acpi_status;
    let mut status2: acpi_status;
    let mut scope_info: acpi_generic_state = core::mem::zeroed();
    let element = *element_ptr;
    let mut resolved_node: *mut acpi_namespace_node;
    let original_node: *mut acpi_namespace_node;
    let mut external_path: *mut i8 = b"\0".as_ptr() as *mut i8;
    let object_type: acpi_object_type;

    if (*element).reference.resolved != FALSE {
        return;
    }

    scope_info.scope.node = (*element).reference.node;
    status = acpi_ns_lookup(
        &mut scope_info,
        (*element).reference.aml as *mut i8,
        ACPI_TYPE_ANY,
        ACPI_IMODE_EXECUTE,
        ACPI_NS_SEARCH_PARENT | ACPI_NS_DONT_OPEN_SCOPE,
        core::ptr::null_mut(),
        &mut resolved_node,
    );
    if ACPI_FAILURE(status) {
        if status == AE_NOT_FOUND && acpi_gbl_ignore_package_resolution_errors != FALSE {
            acpi_ut_remove_reference(element);
            *element_ptr = core::ptr::null_mut();
            return;
        }

        status2 = acpi_ns_externalize_name(
            ACPI_UINT32_MAX,
            (*element).reference.aml as *mut i8,
            core::ptr::null_mut(),
            &mut external_path,
        );
        // ACPI_EXCEPTION((AE_INFO, status, ...));
        if ACPI_SUCCESS(status2) {
            acpi_free(external_path as *mut core::ffi::c_void);
        }
        acpi_ut_remove_reference(element);
        *element_ptr = core::ptr::null_mut();
        return;
    } else if (*resolved_node).type_ == ACPI_TYPE_ANY {
        // ACPI_ERROR((AE_INFO, ...));
        *element_ptr = core::ptr::null_mut();
        return;
    }

    if (*resolved_node).type_ == ACPI_TYPE_LOCAL_ALIAS {
        resolved_node = (*resolved_node).object as *mut acpi_namespace_node;
    }

    (*element).reference.resolved = TRUE;
    (*element).reference.node = resolved_node;
    object_type = (*element).reference.node.as_ref().unwrap().type_;

    original_node = resolved_node;
    status = acpi_ex_resolve_node_to_value(&mut resolved_node, core::ptr::null_mut());
    if ACPI_FAILURE(status) {
        return;
    }

    match object_type {
        ACPI_TYPE_DEVICE | ACPI_TYPE_THERMAL | ACPI_TYPE_METHOD => {}
        ACPI_TYPE_MUTEX
        | ACPI_TYPE_POWER
        | ACPI_TYPE_PROCESSOR
        | ACPI_TYPE_EVENT
        | ACPI_TYPE_REGION => {
            acpi_ut_remove_reference((*original_node).object as *mut acpi_operand_object);
        }
        _ => {
            acpi_ut_remove_reference(element);
            *element_ptr = resolved_node as *mut acpi_operand_object;
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
