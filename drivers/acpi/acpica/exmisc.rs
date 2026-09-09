// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: exmisc - ACPI AML (p-code) execution - specific opcodes
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// Dependencies supplied by the ACPI implementation are intentionally external.

pub unsafe fn acpi_ex_get_object_reference(
    obj_desc: *mut AcpiOperandObject,
    return_desc: *mut *mut AcpiOperandObject,
    walk_state: *mut AcpiWalkState,
) -> AcpiStatus {
    let reference_obj: *mut AcpiOperandObject;
    let referenced_obj: *mut AcpiOperandObject;

    acpi_function_trace_ptr(ex_get_object_reference, obj_desc);
    *return_desc = core::ptr::null_mut();

    match acpi_get_descriptor_type(obj_desc) {
        ACPI_DESC_TYPE_OPERAND => {
            if (*obj_desc).common.type_ != ACPI_TYPE_LOCAL_REFERENCE {
                return AE_AML_OPERAND_TYPE;
            }

            /* Must be a reference to a Local or Arg */
            match (*obj_desc).reference.class {
                ACPI_REFCLASS_LOCAL | ACPI_REFCLASS_ARG | ACPI_REFCLASS_DEBUG => {
                    /* The referenced object is the pseudo-node for the local/arg */
                    referenced_obj = (*obj_desc).reference.object;
                }
                _ => {
                    acpi_error(AE_INFO, "Invalid Reference Class 0x%2.2X", (*obj_desc).reference.class);
                    return AE_AML_OPERAND_TYPE;
                }
            }
        }
        ACPI_DESC_TYPE_NAMED => {
            /* A named reference that has already been resolved to a Node */
            referenced_obj = obj_desc;
        }
        _ => {
            acpi_error(AE_INFO, "Invalid descriptor type 0x%X", acpi_get_descriptor_type(obj_desc));
            return AE_TYPE;
        }
    }

    /* Create a new reference object */
    reference_obj = acpi_ut_create_internal_object(ACPI_TYPE_LOCAL_REFERENCE);
    if reference_obj.is_null() {
        return AE_NO_MEMORY;
    }

    (*reference_obj).reference.class = ACPI_REFCLASS_REFOF;
    (*reference_obj).reference.object = referenced_obj;
    *return_desc = reference_obj;

    acpi_debug_print(
        ACPI_DB_EXEC,
        "Object %p Type [%s], returning Reference %p\n",
        obj_desc,
        acpi_ut_get_object_type_name(obj_desc),
        *return_desc,
    );

    AE_OK
}

pub unsafe fn acpi_ex_do_math_op(opcode: u16, integer0: u64, integer1: u64) -> u64 {
    acpi_function_entry();

    match opcode {
        AML_ADD_OP => integer0.wrapping_add(integer1),
        AML_BIT_AND_OP => integer0 & integer1,
        AML_BIT_NAND_OP => !(integer0 & integer1),
        AML_BIT_OR_OP => integer0 | integer1,
        AML_BIT_NOR_OP => !(integer0 | integer1),
        AML_BIT_XOR_OP => integer0 ^ integer1,
        AML_MULTIPLY_OP => integer0.wrapping_mul(integer1),
        AML_SHIFT_LEFT_OP => {
            if integer1 >= acpi_gbl_integer_bit_width { 0 } else { integer0 << integer1 }
        }
        AML_SHIFT_RIGHT_OP => {
            if integer1 >= acpi_gbl_integer_bit_width { 0 } else { integer0 >> integer1 }
        }
        AML_SUBTRACT_OP => integer0.wrapping_sub(integer1),
        _ => 0,
    }
}

pub unsafe fn acpi_ex_do_logical_numeric_op(
    opcode: u16,
    integer0: u64,
    integer1: u64,
    logical_result: *mut u8,
) -> AcpiStatus {
    let mut status = AE_OK;
    let mut local_result: u8 = FALSE;
    acpi_function_trace(ex_do_logical_numeric_op);

    match opcode {
        AML_LOGICAL_AND_OP => { if integer0 != 0 && integer1 != 0 { local_result = TRUE; } }
        AML_LOGICAL_OR_OP => { if integer0 != 0 || integer1 != 0 { local_result = TRUE; } }
        _ => {
            acpi_error(AE_INFO, "Invalid numeric logical opcode: %X", opcode);
            status = AE_AML_INTERNAL;
        }
    }
    *logical_result = local_result;
    status
}

pub unsafe fn acpi_ex_do_logical_op(
    opcode: u16,
    operand0: *mut AcpiOperandObject,
    operand1: *mut AcpiOperandObject,
    logical_result: *mut u8,
) -> AcpiStatus {
    let mut local_operand1 = operand1;
    let mut integer0: u64;
    let mut integer1: u64;
    let mut length0: u32;
    let mut length1: u32;
    let mut status = AE_OK;
    let mut local_result: u8 = FALSE;
    let mut compare: i32;

    acpi_function_trace(ex_do_logical_op);

    match (*operand0).common.type_ {
        ACPI_TYPE_INTEGER => { status = acpi_ex_convert_to_integer(operand1, &mut local_operand1, ACPI_IMPLICIT_CONVERSION); }
        ACPI_TYPE_STRING => { status = acpi_ex_convert_to_string(operand1, &mut local_operand1, ACPI_IMPLICIT_CONVERT_HEX); }
        ACPI_TYPE_BUFFER => { status = acpi_ex_convert_to_buffer(operand1, &mut local_operand1); }
        _ => { acpi_error(AE_INFO, "Invalid object type for logical operator: %X", (*operand0).common.type_); status = AE_AML_INTERNAL; }
    }
    if acpi_failure(status) { *logical_result = local_result; return status; }

    if (*operand0).common.type_ == ACPI_TYPE_INTEGER {
        integer0 = (*operand0).integer.value;
        integer1 = (*local_operand1).integer.value;
        match opcode {
            AML_LOGICAL_EQUAL_OP => { if integer0 == integer1 { local_result = TRUE; } }
            AML_LOGICAL_GREATER_OP => { if integer0 > integer1 { local_result = TRUE; } }
            AML_LOGICAL_LESS_OP => { if integer0 < integer1 { local_result = TRUE; } }
            _ => { acpi_error(AE_INFO, "Invalid comparison opcode: %X", opcode); status = AE_AML_INTERNAL; }
        }
    } else {
        length0 = (*operand0).buffer.length;
        length1 = (*local_operand1).buffer.length;
        let length = if length0 > length1 { length1 } else { length0 };
        compare = acpi_memcmp((*operand0).buffer.pointer, (*local_operand1).buffer.pointer, length);
        match opcode {
            AML_LOGICAL_EQUAL_OP => { if length0 == length1 && compare == 0 { local_result = TRUE; } }
            AML_LOGICAL_GREATER_OP => { if compare > 0 { local_result = TRUE; } else if compare == 0 && length0 > length1 { local_result = TRUE; } }
            AML_LOGICAL_LESS_OP => { if compare < 0 { local_result = TRUE; } else if compare == 0 && length0 < length1 { local_result = TRUE; } }
            _ => { acpi_error(AE_INFO, "Invalid comparison opcode: %X", opcode); status = AE_AML_INTERNAL; }
        }
    }

    if local_operand1 != operand1 { acpi_ut_remove_reference(local_operand1); }
    *logical_result = local_result;
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
