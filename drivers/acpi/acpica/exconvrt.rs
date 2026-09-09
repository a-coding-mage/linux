// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: exconvrt - Object conversion routines
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 ******************************************************************************/

// C dependencies supplied by the ACPI implementation.

pub unsafe fn acpi_ex_convert_to_integer(
    obj_desc: *mut acpi_operand_object,
    result_desc: *mut *mut acpi_operand_object,
    implicit_conversion: u32,
) -> acpi_status {
    let mut return_desc: *mut acpi_operand_object;
    let mut pointer: *mut u8;
    let mut result: u64;
    let mut i: u32;
    let mut count: u32;

    match (*obj_desc).common.r#type {
        ACPI_TYPE_INTEGER => {
            *result_desc = obj_desc;
            return AE_OK;
        }
        ACPI_TYPE_BUFFER | ACPI_TYPE_STRING => {
            pointer = (*obj_desc).buffer.pointer;
            count = (*obj_desc).buffer.length;
        }
        _ => return AE_TYPE,
    }

    result = 0;
    match (*obj_desc).common.r#type {
        ACPI_TYPE_STRING => {
            if implicit_conversion != 0 {
                result = acpi_ut_implicit_strtoul64(pointer as *const i8);
            } else {
                result = acpi_ut_explicit_strtoul64(pointer as *const i8);
            }
        }
        ACPI_TYPE_BUFFER => {
            if count == 0 {
                return AE_AML_BUFFER_LIMIT;
            }
            if count > acpi_gbl_integer_byte_width {
                count = acpi_gbl_integer_byte_width;
            }
            i = 0;
            while i < count {
                result |= ((*pointer.add(i as usize) as u64) << (i * 8));
                i += 1;
            }
        }
        _ => {}
    }

    return_desc = acpi_ut_create_integer_object(result);
    if return_desc.is_null() {
        return AE_NO_MEMORY;
    }
    let _ = acpi_ex_truncate_for32bit_table(return_desc);
    *result_desc = return_desc;
    AE_OK
}

pub unsafe fn acpi_ex_convert_to_buffer(
    obj_desc: *mut acpi_operand_object,
    result_desc: *mut *mut acpi_operand_object,
) -> acpi_status {
    let return_desc: *mut acpi_operand_object;
    let new_buf: *mut u8;

    match (*obj_desc).common.r#type {
        ACPI_TYPE_BUFFER => {
            *result_desc = obj_desc;
            return AE_OK;
        }
        ACPI_TYPE_INTEGER => {
            return_desc = acpi_ut_create_buffer_object(acpi_gbl_integer_byte_width as acpi_size);
            if return_desc.is_null() { return AE_NO_MEMORY; }
            new_buf = return_desc.as_mut().unwrap().buffer.pointer;
            core::ptr::copy_nonoverlapping(
                &(*obj_desc).integer.value as *const u64 as *const u8,
                new_buf,
                acpi_gbl_integer_byte_width as usize,
            );
        }
        ACPI_TYPE_STRING => {
            return_desc = acpi_ut_create_buffer_object(((*obj_desc).string.length + 1) as acpi_size);
            if return_desc.is_null() { return AE_NO_MEMORY; }
            new_buf = return_desc.as_mut().unwrap().buffer.pointer;
            core::ptr::copy_nonoverlapping(
                (*obj_desc).string.pointer,
                new_buf,
                (*obj_desc).string.length as usize,
            );
        }
        _ => return AE_TYPE,
    }
    (*return_desc).common.flags |= AOPOBJ_DATA_VALID;
    *result_desc = return_desc;
    AE_OK
}

unsafe fn acpi_ex_convert_to_ascii(
    integer: u64, base: u16, string: *mut u8, data_width: u8, leading_zeros: u8,
) -> u32 {
    let mut digit: u64;
    let mut i: u32;
    let mut j: u32;
    let mut k: u32 = 0;
    let mut hex_length: u32;
    let decimal_length: u32;
    let mut remainder: u32 = 0;
    let mut supress_zeros: u8 = if leading_zeros == 0 { TRUE } else { FALSE };
    let mut hex_char: u8;

    match base {
        10 => {
            decimal_length = match data_width { 1 => ACPI_MAX8_DECIMAL_DIGITS, 4 => ACPI_MAX32_DECIMAL_DIGITS, _ => ACPI_MAX64_DECIMAL_DIGITS };
            i = decimal_length;
            while i > 0 {
                digit = integer;
                j = 0;
                while j < i { acpi_ut_short_divide(digit, 10, &mut digit, &mut remainder); j += 1; }
                if remainder != 0 { supress_zeros = FALSE; }
                if supress_zeros == FALSE { *string.add(k as usize) = (ACPI_ASCII_ZERO + remainder) as u8; k += 1; }
                i -= 1;
            }
        }
        16 => {
            hex_length = (data_width as u32) * 2;
            i = 0; j = hex_length - 1;
            while i < hex_length {
                hex_char = acpi_ut_hex_to_ascii_char(integer, ACPI_MUL_4(j)) as u8;
                if !(hex_char == ACPI_ASCII_ZERO && supress_zeros != FALSE) { supress_zeros = FALSE; *string.add(k as usize) = hex_char; k += 1; }
                i += 1; j -= 1;
            }
        }
        _ => return 0,
    }
    if k == 0 { *string = ACPI_ASCII_ZERO as u8; k = 1; }
    *string.add(k as usize) = 0;
    k
}

pub unsafe fn acpi_ex_convert_to_string(
    obj_desc: *mut acpi_operand_object,
    result_desc: *mut *mut acpi_operand_object,
    r#type: u32,
) -> acpi_status {
    let return_desc: *mut acpi_operand_object;
    let mut new_buf: *mut u8;
    let mut i: u32;
    let mut string_length: u32 = 0;
    let mut base: u16 = 16;
    let mut separator: u8 = b',';
    let leading_zeros: u8;

    match (*obj_desc).common.r#type {
        ACPI_TYPE_STRING => { *result_desc = obj_desc; return AE_OK; }
        ACPI_TYPE_INTEGER => {
            match r#type {
                ACPI_EXPLICIT_CONVERT_DECIMAL => { string_length = ACPI_MAX_DECIMAL_DIGITS; leading_zeros = FALSE; base = 10; }
                ACPI_EXPLICIT_CONVERT_HEX => { string_length = ACPI_MUL_2(acpi_gbl_integer_byte_width) + 2; leading_zeros = FALSE; }
                _ => { string_length = ACPI_MUL_2(acpi_gbl_integer_byte_width); leading_zeros = TRUE; }
            }
            return_desc = acpi_ut_create_string_object(string_length as acpi_size);
            if return_desc.is_null() { return AE_NO_MEMORY; }
            new_buf = (*return_desc).buffer.pointer;
            if r#type == ACPI_EXPLICIT_CONVERT_HEX { *new_buf = b'0'; *new_buf.add(1) = b'x'; new_buf = new_buf.add(2); }
            string_length = acpi_ex_convert_to_ascii((*obj_desc).integer.value, base, new_buf, acpi_gbl_integer_byte_width as u8, leading_zeros);
            (*return_desc).string.length = string_length;
            if r#type == ACPI_EXPLICIT_CONVERT_HEX { (*return_desc).string.length += 2; }
            *new_buf.add(string_length as usize) = 0;
        }
        ACPI_TYPE_BUFFER => {
            leading_zeros = match r#type { ACPI_EXPLICIT_CONVERT_DECIMAL => { base = 10; FALSE }, ACPI_IMPLICIT_CONVERT_HEX => { separator = b' '; TRUE }, ACPI_EXPLICIT_CONVERT_HEX => TRUE, _ => return AE_BAD_PARAMETER };
            if r#type == ACPI_EXPLICIT_CONVERT_DECIMAL { for i in 0..(*obj_desc).buffer.length { string_length += if *(*obj_desc).buffer.pointer.add(i as usize) >= 100 { 4 } else if *(*obj_desc).buffer.pointer.add(i as usize) >= 10 { 3 } else { 2 }; } } else { string_length = (*obj_desc).buffer.length * 5; }
            if string_length != 0 { string_length -= 1; }
            return_desc = acpi_ut_create_string_object(string_length as acpi_size);
            if return_desc.is_null() { return AE_NO_MEMORY; }
            new_buf = (*return_desc).buffer.pointer;
            for i in 0..(*obj_desc).buffer.length { if base == 16 { *new_buf = b'0'; *new_buf.add(1) = b'x'; new_buf = new_buf.add(2); } new_buf = new_buf.add(acpi_ex_convert_to_ascii(*(*obj_desc).buffer.pointer.add(i as usize) as u64, base, new_buf, 1, leading_zeros) as usize); *new_buf = separator; new_buf = new_buf.add(1); }
            if (*obj_desc).buffer.length != 0 { new_buf = new_buf.sub(1); } *new_buf = 0;
        }
        _ => return AE_TYPE,
    }
    *result_desc = return_desc;
    AE_OK
}

pub unsafe fn acpi_ex_convert_to_target_type(
    destination_type: acpi_object_type, source_desc: *mut acpi_operand_object,
    result_desc: *mut *mut acpi_operand_object, walk_state: *mut acpi_walk_state,
) -> acpi_status {
    let mut status = AE_OK;
    *result_desc = source_desc;
    match GET_CURRENT_ARG_TYPE((*walk_state).op_info.runtime_args) {
        ARGI_SIMPLE_TARGET | ARGI_FIXED_TARGET | ARGI_INTEGER_REF => {
            if destination_type != ACPI_TYPE_LOCAL_REGION_FIELD && destination_type != (*source_desc).common.r#type { status = AE_TYPE; }
        }
        ARGI_TARGETREF | ARGI_STORE_TARGET => match destination_type {
            ACPI_TYPE_INTEGER | ACPI_TYPE_BUFFER_FIELD | ACPI_TYPE_LOCAL_BANK_FIELD | ACPI_TYPE_LOCAL_INDEX_FIELD => status = acpi_ex_convert_to_integer(source_desc, result_desc, ACPI_IMPLICIT_CONVERSION),
            ACPI_TYPE_STRING => status = acpi_ex_convert_to_string(source_desc, result_desc, ACPI_IMPLICIT_CONVERT_HEX),
            ACPI_TYPE_BUFFER => status = acpi_ex_convert_to_buffer(source_desc, result_desc),
            _ => status = AE_AML_INTERNAL,
        },
        ARGI_REFERENCE => {},
        _ => status = AE_AML_INTERNAL,
    }
    if status == AE_TYPE { status = AE_OK; }
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
