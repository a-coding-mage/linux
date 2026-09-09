// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: nsarguments - Validation of args for ACPI predefined methods
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// Dependencies supplied by the ACPI headers and other translation units.

/* Component: ACPI_NAMESPACE; ACPI_MODULE_NAME("nsarguments") */

/*******************************************************************************
 *
 * FUNCTION:    acpi_ns_check_argument_types
 *
 ******************************************************************************/
pub unsafe fn acpi_ns_check_argument_types(info: *mut acpi_evaluate_info) {
    let mut arg_type_list: u16;
    let arg_count: u8;
    let mut arg_type: u8;
    let mut user_arg_type: u8;
    let mut i: u32;

    if (*info).predefined.is_null()
        || ((*(*info).node).flags & ANOBJ_EVALUATED) != 0
    {
        return;
    }

    arg_type_list = (*(*info).predefined).info.argument_list;
    arg_count = METHOD_GET_ARG_COUNT(arg_type_list);

    i = 0;
    while (i < arg_count as u32 && i < (*info).param_count) {
        arg_type = METHOD_GET_NEXT_TYPE(arg_type_list);
        user_arg_type = (*(*info).parameters.add(i as usize)).common.type_;

        if user_arg_type != arg_type && arg_type != ACPI_TYPE_ANY {
            ACPI_WARN_PREDEFINED!(
                AE_INFO,
                (*info).full_pathname,
                ACPI_WARN_ALWAYS,
                "Argument #%u type mismatch - Found [%s], ACPI requires [%s]",
                i + 1,
                acpi_ut_get_type_name(user_arg_type),
                acpi_ut_get_type_name(arg_type)
            );

            (*(*info).node).flags |= ANOBJ_EVALUATED;
        }

        i += 1;
    }
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ns_check_acpi_compliance
 *
 ******************************************************************************/
pub unsafe fn acpi_ns_check_acpi_compliance(
    pathname: *mut i8,
    node: *mut acpi_namespace_node,
    predefined: *const acpi_predefined_info,
) {
    let mut aml_param_count: u32;
    let required_param_count: u32;

    if predefined.is_null() || ((*node).flags & ANOBJ_EVALUATED) != 0 {
        return;
    }

    required_param_count = METHOD_GET_ARG_COUNT((*predefined).info.argument_list);

    if (*node).type_ != ACPI_TYPE_METHOD {
        if required_param_count > 0 {
            ACPI_BIOS_ERROR_PREDEFINED!(
                AE_INFO, pathname, ACPI_WARN_ALWAYS,
                "Object (%s) must be a control method with %u arguments",
                acpi_ut_get_type_name((*node).type_), required_param_count
            );
        } else if required_param_count == 0 && (*predefined).info.expected_btypes == 0 {
            ACPI_BIOS_ERROR_PREDEFINED!(
                AE_INFO, pathname, ACPI_WARN_ALWAYS,
                "Object (%s) must be a control method with no arguments and no return value",
                acpi_ut_get_type_name((*node).type_)
            );
        }
        return;
    }

    aml_param_count = (*(*node).object).method.param_count;

    if aml_param_count < required_param_count {
        ACPI_BIOS_ERROR_PREDEFINED!(
            AE_INFO, pathname, ACPI_WARN_ALWAYS,
            "Insufficient arguments - ASL declared %u, ACPI requires %u",
            aml_param_count, required_param_count
        );
    } else if aml_param_count > required_param_count
        && ((*predefined).info.argument_list & ARG_COUNT_IS_MINIMUM) == 0
    {
        ACPI_BIOS_ERROR_PREDEFINED!(
            AE_INFO, pathname, ACPI_WARN_ALWAYS,
            "Excess arguments - ASL declared %u, ACPI requires %u",
            aml_param_count, required_param_count
        );
    }
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ns_check_argument_count
 *
 ******************************************************************************/
pub unsafe fn acpi_ns_check_argument_count(
    pathname: *mut i8,
    node: *mut acpi_namespace_node,
    user_param_count: u32,
    predefined: *const acpi_predefined_info,
) {
    let aml_param_count: u32;
    let required_param_count: u32;

    if ((*node).flags & ANOBJ_EVALUATED) != 0 {
        return;
    }

    if predefined.is_null() {
        if (*node).type_ != ACPI_TYPE_METHOD {
            if user_param_count != 0 {
                ACPI_INFO_PREDEFINED!(
                    AE_INFO, pathname, ACPI_WARN_ALWAYS,
                    "%u arguments were passed to a non-method ACPI object (%s)",
                    user_param_count, acpi_ut_get_type_name((*node).type_)
                );
            }
            return;
        }

        aml_param_count = (*(*node).object).method.param_count;
        if user_param_count < aml_param_count {
            ACPI_WARN_PREDEFINED!(
                AE_INFO, pathname, ACPI_WARN_ALWAYS,
                "Insufficient arguments - Caller passed %u, method requires %u",
                user_param_count, aml_param_count
            );
        } else if user_param_count > aml_param_count {
            ACPI_INFO_PREDEFINED!(
                AE_INFO, pathname, ACPI_WARN_ALWAYS,
                "Excess arguments - Caller passed %u, method requires %u",
                user_param_count, aml_param_count
            );
        }
        return;
    }

    required_param_count = METHOD_GET_ARG_COUNT((*predefined).info.argument_list);
    if user_param_count < required_param_count {
        ACPI_WARN_PREDEFINED!(
            AE_INFO, pathname, ACPI_WARN_ALWAYS,
            "Insufficient arguments - Caller passed %u, ACPI requires %u",
            user_param_count, required_param_count
        );
    } else if user_param_count > required_param_count
        && ((*predefined).info.argument_list & ARG_COUNT_IS_MINIMUM) == 0
    {
        ACPI_INFO_PREDEFINED!(
            AE_INFO, pathname, ACPI_WARN_ALWAYS,
            "Excess arguments - Caller passed %u, ACPI requires %u",
            user_param_count, required_param_count
        );
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
