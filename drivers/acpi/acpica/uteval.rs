// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: uteval - Object evaluation
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// Dependencies supplied by the surrounding ACPICA translation unit.

pub unsafe fn acpi_ut_evaluate_object(
    prefix_node: *mut acpi_namespace_node,
    path: *const ::std::ffi::c_char,
    expected_return_btypes: u32,
    return_desc: *mut *mut acpi_operand_object,
) -> acpi_status {
    let mut info: *mut acpi_evaluate_info;
    let status: acpi_status;
    let mut return_btype: u32;

    // ACPI_FUNCTION_TRACE(ut_evaluate_object);

    /* Allocate the evaluation information block */
    info = ACPI_ALLOCATE_ZEROED(::std::mem::size_of::<acpi_evaluate_info>()) as *mut acpi_evaluate_info;
    if info.is_null() {
        return AE_NO_MEMORY;
    }

    (*info).prefix_node = prefix_node;
    (*info).relative_pathname = path;

    /* Evaluate the object/method */
    status = acpi_ns_evaluate(info);
    if (ACPI_FAILURE(status)) {
        if status == AE_NOT_FOUND {
            ACPI_DEBUG_PRINT!(ACPI_DB_EXEC, "[%4.4s.%s] was not found\n", acpi_ut_get_node_name(prefix_node), path);
        } else {
            ACPI_ERROR_METHOD!("Method execution failed", prefix_node, path, status);
        }
        ACPI_FREE(info);
        return status;
    }

    /* Did we get a return object? */
    if (*info).return_object.is_null() {
        if expected_return_btypes != 0 {
            ACPI_ERROR_METHOD!("No object was returned from", prefix_node, path, AE_NOT_EXIST);
            status = AE_NOT_EXIST;
        }
        ACPI_FREE(info);
        return status;
    }

    /* Map the return object type to the bitmapped type */
    return_btype = match (*(*info).return_object).common.type_ {
        ACPI_TYPE_INTEGER => ACPI_BTYPE_INTEGER,
        ACPI_TYPE_BUFFER => ACPI_BTYPE_BUFFER,
        ACPI_TYPE_STRING => ACPI_BTYPE_STRING,
        ACPI_TYPE_PACKAGE => ACPI_BTYPE_PACKAGE,
        _ => 0,
    };

    if acpi_gbl_enable_interpreter_slack && expected_return_btypes == 0 {
        /* We received a return object, but one was not expected. */
        acpi_ut_remove_reference((*info).return_object);
        ACPI_FREE(info);
        return status;
    }

    /* Is the return object one of the expected types? */
    if (expected_return_btypes & return_btype) == 0 {
        ACPI_ERROR_METHOD!("Return object type is incorrect", prefix_node, path, AE_TYPE);
        ACPI_ERROR!(AE_INFO, "Type returned from %s was incorrect: %s, expected Btypes: 0x%X", path, acpi_ut_get_object_type_name((*info).return_object), expected_return_btypes);
        acpi_ut_remove_reference((*info).return_object);
        status = AE_TYPE;
        ACPI_FREE(info);
        return status;
    }

    /* Object type is OK, return it */
    *return_desc = (*info).return_object;
    ACPI_FREE(info);
    status
}

pub unsafe fn acpi_ut_evaluate_numeric_object(
    object_name: *const ::std::ffi::c_char,
    device_node: *mut acpi_namespace_node,
    value: *mut u64,
) -> acpi_status {
    let mut obj_desc: *mut acpi_operand_object = ::std::ptr::null_mut();
    let status = acpi_ut_evaluate_object(device_node, object_name, ACPI_BTYPE_INTEGER, &mut obj_desc);
    if ACPI_FAILURE(status) { return status; }
    *value = (*obj_desc).integer.value;
    acpi_ut_remove_reference(obj_desc);
    status
}

pub unsafe fn acpi_ut_execute_STA(device_node: *mut acpi_namespace_node, flags: *mut u32) -> acpi_status {
    let mut obj_desc: *mut acpi_operand_object = ::std::ptr::null_mut();
    let mut status = acpi_ut_evaluate_object(device_node, METHOD_NAME__STA, ACPI_BTYPE_INTEGER, &mut obj_desc);
    if ACPI_FAILURE(status) {
        if AE_NOT_FOUND == status {
            ACPI_DEBUG_PRINT!(ACPI_DB_EXEC, "_STA on %4.4s was not found, assuming device is present\n", acpi_ut_get_node_name(device_node));
            *flags = ACPI_UINT32_MAX;
            status = AE_OK;
        }
        return status;
    }
    *flags = (*obj_desc).integer.value as u32;
    acpi_ut_remove_reference(obj_desc);
    status
}

pub unsafe fn acpi_ut_execute_power_methods(
    device_node: *mut acpi_namespace_node,
    method_names: *const *const ::std::ffi::c_char,
    method_count: u8,
    out_values: *mut u8,
) -> acpi_status {
    let mut final_status = AE_NOT_FOUND;
    for i in 0..method_count as usize {
        let mut obj_desc: *mut acpi_operand_object = ::std::ptr::null_mut();
        let status = acpi_ut_evaluate_object(device_node, *method_names.add(i), ACPI_BTYPE_INTEGER, &mut obj_desc);
        if ACPI_SUCCESS(status) {
            *out_values.add(i) = (*obj_desc).integer.value as u8;
            acpi_ut_remove_reference(obj_desc);
            final_status = AE_OK;
            continue;
        }
        *out_values.add(i) = ACPI_UINT8_MAX;
        if status == AE_NOT_FOUND { continue; }
        ACPI_DEBUG_PRINT!(ACPI_DB_EXEC, "Failed %s on Device %4.4s, %s\n", *method_names.add(i), acpi_ut_get_node_name(device_node), acpi_format_exception(status));
    }
    final_status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
