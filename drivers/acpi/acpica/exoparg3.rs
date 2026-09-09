// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: exoparg3 - AML execution - opcodes with 3 arguments
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 ******************************************************************************/

// Dependencies supplied by the surrounding ACPICA translation unit.

/* Naming convention for AML interpreter execution routines. */

/*
 * The routines that begin execution of AML opcodes are named with a common
 * convention based upon the number of arguments, the number of target operands,
 * and whether or not a value is returned.
 */

/// Execute Triadic operator (3 operands).
pub unsafe fn acpi_ex_opcode_3a_0t_0r(
    walk_state: *mut acpi_walk_state,
) -> acpi_status {
    let operand = (*walk_state).operands.as_ptr();
    let mut fatal: acpi_signal_fatal_info = core::mem::zeroed();

    acpi_function_trace_str!(
        ex_opcode_3a_0t_0r,
        acpi_ps_get_opcode_name((*walk_state).opcode)
    );

    match (*walk_state).opcode {
        AML_FATAL_OP => {
            fatal.type_ = (*operand.add(0)).integer.value as u32;
            fatal.code = (*operand.add(1)).integer.value as u32;
            fatal.argument = (*operand.add(2)).integer.value as u32;

            acpi_bios_error!(
                AE_INFO,
                "Fatal ACPI BIOS error (Type 0x%X Code 0x%X Arg 0x%X)\n",
                fatal.type_,
                fatal.code,
                fatal.argument
            );

            acpi_os_signal(ACPI_SIGNAL_FATAL, &mut fatal as *mut _ as *mut core::ffi::c_void);

            // Build-time ACPI_CONTINUE_ON_FATAL selects the corresponding branch.
            #[cfg(not(feature = "ACPI_CONTINUE_ON_FATAL"))]
            {
                return AE_ERROR;
            }
            #[cfg(feature = "ACPI_CONTINUE_ON_FATAL")]
            {
                return AE_OK;
            }
        }
        AML_EXTERNAL_OP => {
            acpi_error!(AE_INFO, "Executed External Op");
            AE_OK
        }
        _ => {
            acpi_error!(AE_INFO, "Unknown AML opcode 0x%X", (*walk_state).opcode);
            AE_AML_BAD_OPCODE
        }
    }
}

/// Execute Triadic operator (3 operands).
pub unsafe fn acpi_ex_opcode_3a_1t_1r(
    walk_state: *mut acpi_walk_state,
) -> acpi_status {
    macro_rules! goto_cleanup {
        ($label:ident) => {{
            acpi_ut_remove_reference(return_desc);
            (*walk_state).result_obj = core::ptr::null_mut();
            return status;
        }};
    }

    let operand = (*walk_state).operands.as_ptr();
    let mut return_desc: *mut acpi_operand_object = core::ptr::null_mut();
    let mut buffer: *mut i8 = core::ptr::null_mut();
    let mut status: acpi_status = AE_OK;
    let mut index: u64;
    let mut length: acpi_size;

    acpi_function_trace_str!(
        ex_opcode_3a_1t_1r,
        acpi_ps_get_opcode_name((*walk_state).opcode)
    );

    match (*walk_state).opcode {
        AML_MID_OP => {
            return_desc = acpi_ut_create_internal_object((*operand).common.type_);
            if return_desc.is_null() {
                status = AE_NO_MEMORY;
                goto_cleanup!(cleanup);
            }

            index = (*operand.add(1)).integer.value;
            length = (*operand.add(2)).integer.value as acpi_size;

            if index >= (*operand).string.length as u64 {
                length = 0;
            } else if index.wrapping_add(length as u64) > (*operand).string.length as u64
                || index.wrapping_add(length as u64) < index
            {
                length = (*operand).string.length as acpi_size - index as acpi_size;
            }

            match (*operand).common.type_ {
                ACPI_TYPE_STRING => {
                    buffer = acpi_allocate_zeroed(length + 1) as *mut i8;
                    if buffer.is_null() {
                        status = AE_NO_MEMORY;
                        goto_cleanup!(cleanup);
                    }
                }
                ACPI_TYPE_BUFFER => {
                    if length > 0 {
                        buffer = acpi_allocate_zeroed(length) as *mut i8;
                        if buffer.is_null() {
                            status = AE_NO_MEMORY;
                            goto_cleanup!(cleanup);
                        }
                    }
                }
                _ => {
                    status = AE_AML_OPERAND_TYPE;
                    goto_cleanup!(cleanup);
                }
            }

            if !buffer.is_null() {
                core::ptr::copy_nonoverlapping(
                    (*operand).string.pointer.add(index as usize),
                    buffer,
                    length as usize,
                );
            }

            (*return_desc).string.pointer = buffer;
            (*return_desc).string.length = length as u32;
            (*return_desc).buffer.flags |= AOPOBJ_DATA_VALID;
        }
        _ => {
            acpi_error!(AE_INFO, "Unknown AML opcode 0x%X", (*walk_state).opcode);
            status = AE_AML_BAD_OPCODE;
            goto_cleanup!(cleanup);
        }
    }

    status = acpi_ex_store(return_desc, *operand.add(3), walk_state);

    'cleanup: {
        if ACPI_FAILURE(status) || !(*walk_state).result_obj.is_null() {
            acpi_ut_remove_reference(return_desc);
            (*walk_state).result_obj = core::ptr::null_mut();
        } else {
            (*walk_state).result_obj = return_desc;
        }
    }

    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
