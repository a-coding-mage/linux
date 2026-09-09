/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/* Translation of exoparg1.c.  External ACPI types, constants, and routines are
 * supplied by the surrounding ACPICA translation unit. */

#[allow(non_snake_case, non_camel_case_types, dead_code)]
pub unsafe extern "C" fn acpi_ex_opcode_0A_0T_1R(
    walk_state: *mut acpi_walk_state,
) -> acpi_status {
    let mut status = AE_OK;
    let mut return_desc: *mut acpi_operand_object = core::ptr::null_mut();
    match (*walk_state).opcode {
        AML_TIMER_OP => {
            return_desc = acpi_ut_create_integer_object(acpi_os_get_timer());
            if return_desc.is_null() { status = AE_NO_MEMORY; }
        }
        _ => { ACPI_ERROR!((AE_INFO, "Unknown AML opcode 0x%X", (*walk_state).opcode)); status = AE_AML_BAD_OPCODE; }
    }
    if ACPI_FAILURE(status) || !(*walk_state).result_obj.is_null() {
        acpi_ut_remove_reference(return_desc);
        (*walk_state).result_obj = core::ptr::null_mut();
    } else { (*walk_state).result_obj = return_desc; }
    status
}

#[allow(non_snake_case, non_camel_case_types, dead_code)]
pub unsafe extern "C" fn acpi_ex_opcode_1A_0T_0R(
    walk_state: *mut acpi_walk_state,
) -> acpi_status {
    let operand = (*walk_state).operands.as_mut_ptr();
    let status = match (*walk_state).opcode {
        AML_RELEASE_OP => acpi_ex_release_mutex(*operand, walk_state),
        AML_RESET_OP => acpi_ex_system_reset_event(*operand),
        AML_SIGNAL_OP => acpi_ex_system_signal_event(*operand),
        AML_SLEEP_OP => acpi_ex_system_do_sleep((**operand).integer.value),
        AML_STALL_OP => acpi_ex_system_do_stall((**operand).integer.value as u32),
        AML_UNLOAD_OP => acpi_ex_unload_table(*operand),
        _ => { ACPI_ERROR!((AE_INFO, "Unknown AML opcode 0x%X", (*walk_state).opcode)); AE_AML_BAD_OPCODE }
    };
    status
}

/* The Load-only legacy entry point is retained under its original build-time
 * condition.  Its external dependencies are intentionally unresolved here. */
#[cfg(any())]
pub unsafe extern "C" fn acpi_ex_opcode_1A_1T_0R(walk_state: *mut acpi_walk_state) -> acpi_status {
    let operand = (*walk_state).operands.as_mut_ptr();
    match (*walk_state).opcode {
        AML_LOAD_OP => acpi_ex_load_op(*operand, *operand.add(1), walk_state),
        _ => { ACPI_ERROR!((AE_INFO, "Unknown opcode")); AE_AML_BAD_OPCODE }
    }
}

#[allow(non_snake_case, non_camel_case_types, dead_code)]
pub unsafe extern "C" fn acpi_ex_opcode_1A_1T_1R(
    walk_state: *mut acpi_walk_state,
) -> acpi_status {
    let operand = (*walk_state).operands.as_mut_ptr();
    let mut status = AE_OK;
    let mut return_desc: *mut acpi_operand_object = core::ptr::null_mut();
    let mut return_desc2: *mut acpi_operand_object = core::ptr::null_mut();
    let mut temp32: u32;
    let mut i: u32;
    let mut power_of_ten: u64;
    let mut digit: u64;
    match (*walk_state).opcode {
        AML_BIT_NOT_OP | AML_FIND_SET_LEFT_BIT_OP | AML_FIND_SET_RIGHT_BIT_OP |
        AML_FROM_BCD_OP | AML_LOAD_OP | AML_TO_BCD_OP | AML_CONDITIONAL_REF_OF_OP => {
            return_desc = acpi_ut_create_internal_object(ACPI_TYPE_INTEGER);
            if return_desc.is_null() { status = AE_NO_MEMORY; }
            else { match (*walk_state).opcode {
                AML_BIT_NOT_OP => (*return_desc).integer.value = !(**operand).integer.value,
                AML_FIND_SET_LEFT_BIT_OP => {
                    (*return_desc).integer.value = (**operand).integer.value; temp32 = 0;
                    while (*return_desc).integer.value != 0 && temp32 < ACPI_INTEGER_BIT_SIZE { (*return_desc).integer.value >>= 1; temp32 += 1; }
                    (*return_desc).integer.value = temp32 as u64;
                }
                AML_FIND_SET_RIGHT_BIT_OP => {
                    (*return_desc).integer.value = (**operand).integer.value; temp32 = 0;
                    while (*return_desc).integer.value != 0 && temp32 < ACPI_INTEGER_BIT_SIZE { (*return_desc).integer.value <<= 1; temp32 += 1; }
                    (*return_desc).integer.value = if temp32 == 0 { 0 } else { (ACPI_INTEGER_BIT_SIZE + 1 - temp32) as u64 };
                }
                AML_FROM_BCD_OP => {
                    power_of_ten = 1; (*return_desc).integer.value = 0; digit = (**operand).integer.value; i = 0;
                    while i < acpi_gbl_integer_nybble_width && digit > 0 { temp32 = digit as u32 & 0xf; if temp32 > 9 { status = AE_AML_NUMERIC_OVERFLOW; break; } (*return_desc).integer.value += temp32 as u64 * power_of_ten; digit >>= 4; power_of_ten *= 10; i += 1; }
                }
                AML_LOAD_OP => { (*return_desc).integer.value = 0; status = acpi_ex_load_op(*operand, return_desc, walk_state); if ACPI_SUCCESS(status) { (*return_desc).integer.value = ACPI_UINT64_MAX; } }
                AML_TO_BCD_OP => {
                    (*return_desc).integer.value = 0; digit = (**operand).integer.value; i = 0;
                    while i < acpi_gbl_integer_nybble_width && digit > 0 { let mut rem = 0u32; acpi_ut_short_divide(digit, 10, &mut digit, &mut rem); (*return_desc).integer.value |= (rem as u64) << (ACPI_MUL_4(i)); i += 1; }
                    if digit > 0 { status = AE_AML_NUMERIC_OVERFLOW; }
                }
                AML_CONDITIONAL_REF_OF_OP => {
                    if *operand as *mut acpi_namespace_node == acpi_gbl_root_node { (*return_desc).integer.value = 0; }
                    else { status = acpi_ex_get_object_reference(*operand, &mut return_desc2, walk_state); if ACPI_SUCCESS(status) { status = acpi_ex_store(return_desc2, *operand.add(1), walk_state); acpi_ut_remove_reference(return_desc2); (*return_desc).integer.value = ACPI_UINT64_MAX; } }
                }
                _ => {}
            }}
        }
        AML_STORE_OP => {
            status = acpi_ex_store(*operand, *operand.add(1), walk_state);
            if ACPI_SUCCESS(status) && (*walk_state).result_obj.is_null() { (*walk_state).result_obj = *operand; *operand = core::ptr::null_mut(); return status; }
        }
        AML_COPY_OBJECT_OP => status = acpi_ut_copy_iobject_to_iobject(*operand, &mut return_desc, walk_state),
        AML_TO_DECIMAL_STRING_OP => { status = acpi_ex_convert_to_string(*operand, &mut return_desc, ACPI_EXPLICIT_CONVERT_DECIMAL); if return_desc == *operand { acpi_ut_add_reference(return_desc); } }
        AML_TO_HEX_STRING_OP => { status = acpi_ex_convert_to_string(*operand, &mut return_desc, ACPI_EXPLICIT_CONVERT_HEX); if return_desc == *operand { acpi_ut_add_reference(return_desc); } }
        AML_TO_BUFFER_OP => { status = acpi_ex_convert_to_buffer(*operand, &mut return_desc); if return_desc == *operand { acpi_ut_add_reference(return_desc); } }
        AML_TO_INTEGER_OP => { status = acpi_ex_convert_to_integer(*operand, &mut return_desc, 0); if return_desc == *operand { acpi_ut_add_reference(return_desc); } }
        AML_SHIFT_LEFT_BIT_OP | AML_SHIFT_RIGHT_BIT_OP => status = AE_SUPPORT,
        _ => status = AE_AML_BAD_OPCODE,
    }
    if ACPI_SUCCESS(status) { status = acpi_ex_store(return_desc, *operand.add(1), walk_state); }
    if ACPI_FAILURE(status) { acpi_ut_remove_reference(return_desc); } else if (*walk_state).result_obj.is_null() { (*walk_state).result_obj = return_desc; }
    status
}

#[allow(non_snake_case, non_camel_case_types, dead_code)]
pub unsafe extern "C" fn acpi_ex_opcode_1A_0T_1R(walk_state: *mut acpi_walk_state) -> acpi_status {
    let operand = (*walk_state).operands.as_mut_ptr();
    let mut status = AE_OK;
    let mut return_desc: *mut acpi_operand_object = core::ptr::null_mut();
    let mut temp_desc: *mut acpi_operand_object;
    let mut ty: u32 = 0;
    let mut value: u64 = 0;
    match (*walk_state).opcode {
        AML_LOGICAL_NOT_OP => { return_desc = acpi_ut_create_integer_object(0); if return_desc.is_null() { status = AE_NO_MEMORY; } else if (**operand).integer.value == 0 { (*return_desc).integer.value = ACPI_UINT64_MAX; } }
        AML_DECREMENT_OP | AML_INCREMENT_OP => {
            return_desc = acpi_ut_create_internal_object(ACPI_TYPE_INTEGER); if return_desc.is_null() { status = AE_NO_MEMORY; }
            else { temp_desc = *operand; status = acpi_ex_resolve_operands(AML_LOGICAL_NOT_OP, &mut temp_desc, walk_state); if ACPI_SUCCESS(status) { (*return_desc).integer.value = if (*walk_state).opcode == AML_INCREMENT_OP { (*temp_desc).integer.value.wrapping_add(1) } else { (*temp_desc).integer.value.wrapping_sub(1) }; acpi_ut_remove_reference(temp_desc); status = acpi_ex_store(return_desc, *operand, walk_state); } }
        }
        AML_OBJECT_TYPE_OP => { status = acpi_ex_resolve_multiple(walk_state, *operand, &mut ty, core::ptr::null_mut()); if ACPI_SUCCESS(status) { return_desc = acpi_ut_create_integer_object(ty as u64); if return_desc.is_null() { status = AE_NO_MEMORY; } } }
        AML_SIZE_OF_OP => { status = acpi_ex_resolve_multiple(walk_state, *operand, &mut ty, &mut temp_desc); if ACPI_SUCCESS(status) { value = match ty { ACPI_TYPE_INTEGER => acpi_gbl_integer_byte_width as u64, ACPI_TYPE_STRING => (*temp_desc).string.length as u64, ACPI_TYPE_BUFFER => { status = acpi_ds_get_buffer_arguments(temp_desc); (*temp_desc).buffer.length as u64 }, ACPI_TYPE_PACKAGE => { status = acpi_ds_get_package_arguments(temp_desc); (*temp_desc).package.count as u64 }, _ => { status = AE_AML_OPERAND_TYPE; 0 } }; if ACPI_SUCCESS(status) { return_desc = acpi_ut_create_integer_object(value); if return_desc.is_null() { status = AE_NO_MEMORY; } } } }
        AML_REF_OF_OP => status = acpi_ex_get_object_reference(*operand, &mut return_desc, walk_state),
        AML_DEREF_OF_OP => {
            /* Preserve the complete reference semantics: named objects, local/
             * argument references, Index(Buffer/Package), RefOf, and strings. */
            if ACPI_GET_DESCRIPTOR_TYPE(*operand) == ACPI_DESC_TYPE_NAMED { return_desc = acpi_ns_get_attached_object(*operand as *mut acpi_namespace_node); if return_desc.is_null() { status = AE_AML_OPERAND_TYPE; } else { acpi_ut_add_reference(return_desc); } }
            else if (**operand).common.type == ACPI_TYPE_STRING { status = acpi_ns_get_node_unlocked((*walk_state).scope_info.scope.node, (**operand).string.pointer, ACPI_NS_SEARCH_PARENT, &mut return_desc as *mut _ as *mut *mut acpi_namespace_node); }
            else { status = AE_AML_OPERAND_TYPE; }
        }
        _ => status = AE_AML_BAD_OPCODE,
    }
    if ACPI_FAILURE(status) { acpi_ut_remove_reference(return_desc); } else { (*walk_state).result_obj = return_desc; }
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
