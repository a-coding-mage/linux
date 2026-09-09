// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: nspredef - Validation of ACPI predefined methods and objects
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// #define ACPI_CREATE_PREDEFINED_TABLE
// Dependencies are supplied by the surrounding ACPI implementation.

// #define _COMPONENT ACPI_NAMESPACE
// ACPI_MODULE_NAME("nspredef")

/*******************************************************************************
 *
 * This module validates predefined ACPI objects that appear in the namespace,
 * at the time they are evaluated (via acpi_evaluate_object). The purpose of this
 * validation is to detect problems with BIOS-exposed predefined ACPI objects
 * before the results are returned to the ACPI-related drivers.
 *
 * There are several areas that are validated:
 *
 *  1) The number of input arguments as defined by the method/object in the
 *     ASL is validated against the ACPI specification.
 *  2) The type of the return object (if any) is validated against the ACPI
 *     specification.
 *  3) For returned package objects, the count of package elements is
 *     validated, as well as the type of each package element. Nested
 *     packages are supported.
 *
 * For any problems found, a warning message is issued.
 *
 ******************************************************************************/

/* Local prototypes */
unsafe fn acpi_ns_check_reference(
    info: *mut acpi_evaluate_info,
    return_object: *mut acpi_operand_object,
) -> acpi_status;

unsafe fn acpi_ns_get_bitmapped_type(
    return_object: *mut acpi_operand_object,
) -> u32;

pub unsafe fn acpi_ns_check_return_value(
    node: *mut acpi_namespace_node,
    info: *mut acpi_evaluate_info,
    user_param_count: u32,
    return_status: acpi_status,
    return_object_ptr: *mut *mut acpi_operand_object,
) -> acpi_status {
    let mut status: acpi_status;
    let predefined: *const acpi_predefined_info;

    ACPI_FUNCTION_TRACE!(ns_check_return_value);

    /* If not a predefined name, we cannot validate the return object */
    predefined = (*info).predefined;
    if predefined.is_null() {
        return_ACPI_STATUS!(AE_OK);
    }

    /* If the method failed or did not actually return an object, we cannot
     * validate the return object */
    if return_status != AE_OK && return_status != AE_CTRL_RETURN_VALUE {
        return_ACPI_STATUS!(AE_OK);
    }

    /*
     * Return value validation and possible repair.
     *
     * 1) Don't perform return value validation/repair if this feature
     * has been disabled via a global option.
     *
     * 2) We have a return value, but if one wasn't expected, just exit,
     * this is not a problem. For example, if the "Implicit Return"
     * feature is enabled, methods will always return a value.
     *
     * 3) If the return value can be of any type, then we cannot perform
     * any validation, just exit.
     */
    if acpi_gbl_disable_auto_repair
        || (*predefined).info.expected_btypes == 0
        || (*predefined).info.expected_btypes == ACPI_RTYPE_ALL
    {
        return_ACPI_STATUS!(AE_OK);
    }

    status = acpi_ns_check_object_type(
        info,
        return_object_ptr,
        (*predefined).info.expected_btypes,
        ACPI_NOT_PACKAGE_ELEMENT,
    );
    if ACPI_FAILURE!(status) {
        goto_exit!();
    }

    /* If there is no return value and it is optional, just return AE_OK (_WAK). */
    if (*return_object_ptr).is_null() {
        goto_exit!();
    }

    /* For returned Package objects, check the type of all sub-objects. */
    if (*(*return_object_ptr)).common.type_ == ACPI_TYPE_PACKAGE {
        (*info).parent_package = *return_object_ptr;
        status = acpi_ns_check_package(info, return_object_ptr);
        if ACPI_FAILURE!(status)
            && status != AE_AML_OPERAND_TYPE
            && status != AE_AML_OPERAND_VALUE
        {
            goto_exit!();
        }
    }

    status = acpi_ns_complex_repairs(info, node, status, return_object_ptr);

goto_exit:
    if ACPI_FAILURE!(status) || ((*info).return_flags & ACPI_OBJECT_REPAIRED) != 0 {
        (*node).flags |= ANOBJ_EVALUATED;
    }

    return_ACPI_STATUS!(status);
}

pub unsafe fn acpi_ns_check_object_type(
    info: *mut acpi_evaluate_info,
    return_object_ptr: *mut *mut acpi_operand_object,
    expected_btypes: u32,
    package_index: u32,
) -> acpi_status {
    let return_object = *return_object_ptr;
    let mut status = AE_OK;
    let mut type_buffer = [0i8; 96];

    if !return_object.is_null()
        && ACPI_GET_DESCRIPTOR_TYPE!(return_object) == ACPI_DESC_TYPE_NAMED
    {
        ACPI_WARN_PREDEFINED!(
            (AE_INFO, (*info).full_pathname, (*info).node_flags,
             "Invalid return type - Found a Namespace node [%4.4s] type %s",
             (*return_object).node.name.ascii,
             acpi_ut_get_type_name((*return_object).node.type_))
        );
        return AE_AML_OPERAND_TYPE;
    }

    (*info).return_btype = acpi_ns_get_bitmapped_type(return_object);
    if (*info).return_btype == ACPI_RTYPE_ANY {
        goto_type_error_exit!();
    }

    if ((*info).return_btype & expected_btypes) == ACPI_RTYPE_REFERENCE {
        status = acpi_ns_check_reference(info, return_object);
        return status;
    }

    status = acpi_ns_simple_repair(info, expected_btypes, package_index, return_object_ptr);
    if ACPI_SUCCESS!(status) {
        return AE_OK;
    }

goto_type_error_exit:
    acpi_ut_get_expected_return_types(type_buffer.as_mut_ptr(), expected_btypes);
    if return_object.is_null() {
        ACPI_WARN_PREDEFINED!((AE_INFO, (*info).full_pathname, (*info).node_flags,
            "Expected return object of type %s", type_buffer.as_ptr()));
    } else if package_index == ACPI_NOT_PACKAGE_ELEMENT {
        ACPI_WARN_PREDEFINED!((AE_INFO, (*info).full_pathname, (*info).node_flags,
            "Return type mismatch - found %s, expected %s",
            acpi_ut_get_object_type_name(return_object), type_buffer.as_ptr()));
    } else {
        ACPI_WARN_PREDEFINED!((AE_INFO, (*info).full_pathname, (*info).node_flags,
            "Return Package type mismatch at index %u - found %s, expected %s",
            package_index, acpi_ut_get_object_type_name(return_object), type_buffer.as_ptr()));
    }
    AE_AML_OPERAND_TYPE
}

unsafe fn acpi_ns_check_reference(
    info: *mut acpi_evaluate_info,
    return_object: *mut acpi_operand_object,
) -> acpi_status {
    if (*return_object).reference.class == ACPI_REFCLASS_NAME {
        return AE_OK;
    }
    ACPI_WARN_PREDEFINED!((AE_INFO, (*info).full_pathname, (*info).node_flags,
        "Return type mismatch - unexpected reference object type [%s] %2.2X",
        acpi_ut_get_reference_name(return_object), (*return_object).reference.class));
    AE_AML_OPERAND_TYPE
}

unsafe fn acpi_ns_get_bitmapped_type(return_object: *mut acpi_operand_object) -> u32 {
    if return_object.is_null() {
        return ACPI_RTYPE_NONE;
    }
    match (*return_object).common.type_ {
        ACPI_TYPE_INTEGER => ACPI_RTYPE_INTEGER,
        ACPI_TYPE_BUFFER => ACPI_RTYPE_BUFFER,
        ACPI_TYPE_STRING => ACPI_RTYPE_STRING,
        ACPI_TYPE_PACKAGE => ACPI_RTYPE_PACKAGE,
        ACPI_TYPE_LOCAL_REFERENCE => ACPI_RTYPE_REFERENCE,
        _ => ACPI_RTYPE_ANY,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
