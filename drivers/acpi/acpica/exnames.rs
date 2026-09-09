// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*
 * Module Name: exnames - interpreter/scanner name load/execute
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 */

// Dependencies supplied by the ACPI implementation are intentionally left external.

// Local prototypes
unsafe fn acpi_ex_allocate_name_string(prefix_count: u32, num_name_segs: u32) -> *mut i8;
unsafe fn acpi_ex_name_segment(in_aml_address: *mut *mut u8, name_string: *mut i8) -> acpi_status;

unsafe fn acpi_ex_allocate_name_string(prefix_count: u32, num_name_segs: u32) -> *mut i8 {
    let mut temp_ptr: *mut i8;
    let name_string: *mut i8;
    let size_needed: u32;

    ACPI_FUNCTION_TRACE!(ex_allocate_name_string);

    /* Allow room for prefixes, all segments, a prefix opcode, and the terminator. */
    if prefix_count == ACPI_UINT32_MAX {
        size_needed = 1 + (ACPI_NAMESEG_SIZE * num_name_segs) + 2 + 1;
    } else {
        size_needed = prefix_count + (ACPI_NAMESEG_SIZE * num_name_segs) + 2 + 1;
    }

    name_string = ACPI_ALLOCATE(size_needed) as *mut i8;
    if name_string.is_null() {
        ACPI_ERROR!((AE_INFO, "Could not allocate size {}", size_needed));
        return_PTR!(core::ptr::null_mut());
    }

    temp_ptr = name_string;

    /* Set up Root or Parent prefixes if needed. */
    if prefix_count == ACPI_UINT32_MAX {
        *temp_ptr = AML_ROOT_PREFIX as i8;
        temp_ptr = temp_ptr.add(1);
    } else {
        let mut count = prefix_count;
        while count != 0 {
            *temp_ptr = AML_PARENT_PREFIX as i8;
            temp_ptr = temp_ptr.add(1);
            count -= 1;
        }
    }

    /* Set up Dual or Multi prefixes if needed. */
    if num_name_segs > 2 {
        *temp_ptr = AML_MULTI_NAME_PREFIX as i8;
        temp_ptr = temp_ptr.add(1);
        *temp_ptr = num_name_segs as i8;
        temp_ptr = temp_ptr.add(1);
    } else if num_name_segs == 2 {
        *temp_ptr = AML_DUAL_NAME_PREFIX as i8;
        temp_ptr = temp_ptr.add(1);
    }

    *temp_ptr = 0;
    return_PTR!(name_string)
}

unsafe fn acpi_ex_name_segment(in_aml_address: *mut *mut u8, name_string: *mut i8) -> acpi_status {
    let mut aml_address = *in_aml_address as *mut i8;
    let mut status = AE_OK;
    let mut index: u32;
    let mut char_buf = [0i8; 5];

    ACPI_FUNCTION_TRACE!(ex_name_segment);

    char_buf[0] = *aml_address;
    if ('0' as i8) <= char_buf[0] && char_buf[0] <= ('9' as i8) {
        ACPI_ERROR!((AE_INFO, "Invalid leading digit: {}", char_buf[0]));
        return_ACPI_STATUS!(AE_CTRL_PENDING);
    }

    index = 0;
    while index < ACPI_NAMESEG_SIZE && acpi_ut_valid_name_char(*aml_address as u8, 0) {
        char_buf[index as usize] = *aml_address;
        aml_address = aml_address.add(1);
        index += 1;
    }

    if index == 4 {
        char_buf[4] = 0;
        if !name_string.is_null() {
            ACPI_DEBUG_PRINT!((ACPI_DB_NAMES, "Appending NameSeg %s\n", char_buf.as_ptr()));
            acpi_os_strcat(name_string, char_buf.as_ptr());
        } else {
            ACPI_DEBUG_PRINT!((ACPI_DB_NAMES, "No Name string - %s\n", char_buf.as_ptr()));
        }
    } else if index == 0 {
        ACPI_DEBUG_PRINT!((ACPI_DB_INFO, "Leading character is not alpha: {:02X}h (not a name)\n", char_buf[0]));
        status = AE_CTRL_PENDING;
    } else {
        status = AE_AML_BAD_NAME;
        ACPI_ERROR!((AE_INFO, "Bad character 0x{:02x} in name, at {:?}", *aml_address, aml_address));
    }

    *in_aml_address = aml_address as *mut u8;
    return_ACPI_STATUS!(status)
}

pub unsafe fn acpi_ex_get_name_string(
    data_type: acpi_object_type,
    in_aml_address: *mut u8,
    out_name_string: *mut *mut i8,
    out_name_length: *mut u32,
) -> acpi_status {
    let mut status = AE_OK;
    let mut aml_address = in_aml_address;
    let mut name_string: *mut i8 = core::ptr::null_mut();
    let mut num_segments: u8;
    let mut prefix_count: u32 = 0;
    let mut has_prefix = FALSE;

    ACPI_FUNCTION_TRACE_PTR!(ex_get_name_string, aml_address);

    if data_type == ACPI_TYPE_LOCAL_REGION_FIELD || data_type == ACPI_TYPE_LOCAL_BANK_FIELD || data_type == ACPI_TYPE_LOCAL_INDEX_FIELD {
        name_string = acpi_ex_allocate_name_string(0, 1);
        if name_string.is_null() { status = AE_NO_MEMORY; }
        else { status = acpi_ex_name_segment(&mut aml_address, name_string); }
    } else {
        match *aml_address {
            AML_ROOT_PREFIX => { aml_address = aml_address.add(1); prefix_count = ACPI_UINT32_MAX; has_prefix = TRUE; }
            AML_PARENT_PREFIX => {
                while *aml_address == AML_PARENT_PREFIX { aml_address = aml_address.add(1); prefix_count += 1; }
                has_prefix = TRUE;
            }
            _ => {}
        }
        match *aml_address {
            AML_DUAL_NAME_PREFIX => {
                aml_address = aml_address.add(1); name_string = acpi_ex_allocate_name_string(prefix_count, 2);
                if name_string.is_null() { status = AE_NO_MEMORY; }
                else { has_prefix = TRUE; status = acpi_ex_name_segment(&mut aml_address, name_string); if ACPI_SUCCESS(status) { status = acpi_ex_name_segment(&mut aml_address, name_string); } }
            }
            AML_MULTI_NAME_PREFIX => {
                aml_address = aml_address.add(1); num_segments = *aml_address; name_string = acpi_ex_allocate_name_string(prefix_count, num_segments as u32);
                if name_string.is_null() { status = AE_NO_MEMORY; }
                else { aml_address = aml_address.add(1); has_prefix = TRUE; while num_segments != 0 { status = acpi_ex_name_segment(&mut aml_address, name_string); if status != AE_OK { break; } num_segments -= 1; } }
            }
            0 => { aml_address = aml_address.add(1); name_string = acpi_ex_allocate_name_string(prefix_count, 0); if name_string.is_null() { status = AE_NO_MEMORY; } }
            _ => { name_string = acpi_ex_allocate_name_string(prefix_count, 1); if name_string.is_null() { status = AE_NO_MEMORY; } else { status = acpi_ex_name_segment(&mut aml_address, name_string); } }
        }
    }

    if status == AE_CTRL_PENDING && has_prefix { status = AE_AML_BAD_NAME; }
    if ACPI_FAILURE(status) { if !name_string.is_null() { ACPI_FREE(name_string); } return_ACPI_STATUS!(status); }
    *out_name_string = name_string;
    *out_name_length = aml_address.offset_from(in_aml_address) as u32;
    return_ACPI_STATUS!(status)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
