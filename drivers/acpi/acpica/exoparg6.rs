// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: exoparg6 - AML execution - opcodes with 6 arguments
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 ******************************************************************************/

// Dependencies supplied by the ACPI implementation are intentionally external.

/*
 * Naming convention for AML interpreter execution routines.
 *
 * The routines that begin execution of AML opcodes are named with a common
 * convention based upon the number of arguments, the number of target operands,
 * and whether or not a value is returned.
 */

/* Local prototypes */
unsafe fn acpi_ex_do_match(
    match_op: u32,
    package_obj: *mut acpi_operand_object,
    match_obj: *mut acpi_operand_object,
) -> u8;

/*******************************************************************************
 *
 * FUNCTION:    acpi_ex_do_match
 *
 * PARAMETERS:  match_op        - The AML match operand
 *              package_obj     - Object from the target package
 *              match_obj       - Object to be matched
 *
 * RETURN:      TRUE if the match is successful, FALSE otherwise
 *
 * DESCRIPTION: Implements the low-level match for the ASL Match operator.
 *              Package elements will be implicitly converted to the type of
 *              the match object (Integer/Buffer/String).
 *
 ******************************************************************************/

unsafe fn acpi_ex_do_match(
    match_op: u32,
    package_obj: *mut acpi_operand_object,
    match_obj: *mut acpi_operand_object,
) -> u8 {
    let mut logical_result: u8 = TRUE;
    let mut status: acpi_status;

    /*
     * Note: Since the package_obj/match_obj ordering is opposite to that of
     * the standard logical operators, we have to reverse them when we call
     * do_logical_op in order to make the implicit conversion rules work
     * correctly. However, this means we have to flip the entire equation
     * also. A bit ugly perhaps, but overall, better than fussing the
     * parameters around at runtime, over and over again.
     *
     * Below, P[i] refers to the package element, M refers to the Match object.
     */
    match match_op {
        MATCH_MTR => { /* Always true */ }
        MATCH_MEQ => {
            /* True if equal: (P[i] == M); change to: (M == P[i]) */
            status = acpi_ex_do_logical_op(
                AML_LOGICAL_EQUAL_OP, match_obj, package_obj, &mut logical_result,
            );
            if ACPI_FAILURE(status) { return FALSE; }
        }
        MATCH_MLE => {
            /* True if less than or equal: (P[i] <= M); change to (M >= P[i]) */
            status = acpi_ex_do_logical_op(
                AML_LOGICAL_LESS_OP, match_obj, package_obj, &mut logical_result,
            );
            if ACPI_FAILURE(status) { return FALSE; }
            logical_result = (!logical_result) as u8;
        }
        MATCH_MLT => {
            /* True if less than: (P[i] < M); change to (M > P[i]) */
            status = acpi_ex_do_logical_op(
                AML_LOGICAL_GREATER_OP, match_obj, package_obj, &mut logical_result,
            );
            if ACPI_FAILURE(status) { return FALSE; }
        }
        MATCH_MGE => {
            /* True if greater than or equal: (P[i] >= M); change to (M <= P[i]) */
            status = acpi_ex_do_logical_op(
                AML_LOGICAL_GREATER_OP, match_obj, package_obj, &mut logical_result,
            );
            if ACPI_FAILURE(status) { return FALSE; }
            logical_result = (!logical_result) as u8;
        }
        MATCH_MGT => {
            /* True if greater than: (P[i] > M); change to (M < P[i]) */
            status = acpi_ex_do_logical_op(
                AML_LOGICAL_LESS_OP, match_obj, package_obj, &mut logical_result,
            );
            if ACPI_FAILURE(status) { return FALSE; }
        }
        _ => return FALSE, /* Undefined */
    }

    logical_result
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ex_opcode_6A_0T_1R
 *
 * PARAMETERS:  walk_state          - Current walk state
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Execute opcode with 6 arguments, no target, and a return value
 *
 ******************************************************************************/

pub unsafe fn acpi_ex_opcode_6A_0T_1R(walk_state: *mut acpi_walk_state) -> acpi_status {
    let operand: *mut *mut acpi_operand_object = (*walk_state).operands.as_mut_ptr();
    let mut return_desc: *mut acpi_operand_object = core::ptr::null_mut();
    let mut status: acpi_status = AE_OK;
    let mut index: u64;
    let mut this_element: *mut acpi_operand_object;

    ACPI_FUNCTION_TRACE_STR(
        ex_opcode_6A_0T_1R,
        acpi_ps_get_opcode_name((*walk_state).opcode),
    );

    match (*walk_state).opcode {
        AML_MATCH_OP => {
            /* Match (search_pkg, match_op1, match_obj1, match_op2, match_obj2, start_index) */
            if (*(*operand.add(1))).integer.value > MAX_MATCH_OPERATOR
                || (*(*operand.add(3))).integer.value > MAX_MATCH_OPERATOR
            {
                ACPI_ERROR((AE_INFO, "Match operator out of range"));
                status = AE_AML_OPERAND_VALUE;
                acpi_ut_remove_reference(return_desc);
                return status;
            }

            index = (*(*operand.add(5))).integer.value;
            if index >= (*(*operand)).package.count {
                ACPI_ERROR((AE_INFO, "Index beyond package end"));
                status = AE_AML_PACKAGE_LIMIT;
                acpi_ut_remove_reference(return_desc);
                return status;
            }

            return_desc = acpi_ut_create_integer_object(ACPI_UINT64_MAX);
            if return_desc.is_null() {
                status = AE_NO_MEMORY;
                goto_cleanup!();
            }

            while index < (*(*operand)).package.count {
                this_element = *(*operand).package.elements.add(index as usize);
                if this_element.is_null() {
                    index += 1;
                    continue;
                }
                if acpi_ex_do_match((*(*operand.add(1))).integer.value as u32, this_element, *operand.add(2)) == 0 {
                    index += 1;
                    continue;
                }
                if acpi_ex_do_match((*(*operand.add(3))).integer.value as u32, this_element, *operand.add(4)) == 0 {
                    index += 1;
                    continue;
                }
                (*return_desc).integer.value = index;
                break;
            }
        }
        AML_LOAD_TABLE_OP => {
            status = acpi_ex_load_table_op(walk_state, &mut return_desc);
        }
        _ => {
            ACPI_ERROR((AE_INFO, "Unknown AML opcode"));
            status = AE_AML_BAD_OPCODE;
            acpi_ut_remove_reference(return_desc);
            return status;
        }
    }

    if ACPI_FAILURE(status) {
        acpi_ut_remove_reference(return_desc);
    } else {
        (*walk_state).result_obj = return_desc;
    }
    return_ACPI_STATUS(status)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
