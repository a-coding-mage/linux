// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: exstorob - AML object store support, store to object
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// Dependencies supplied by the ACPI implementation.

/*
 * FUNCTION:    acpi_ex_store_buffer_to_buffer
 *
 * PARAMETERS:  source_desc         - Source object to copy
 *              target_desc         - Destination object of the copy
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Copy a buffer object to another buffer object.
 */
pub unsafe fn acpi_ex_store_buffer_to_buffer(
    source_desc: *mut acpi_operand_object,
    target_desc: *mut acpi_operand_object,
) -> acpi_status {
    let length: u32;
    let buffer: *mut u8;

    // ACPI_FUNCTION_TRACE_PTR(ex_store_buffer_to_buffer, source_desc);

    /* If Source and Target are the same, just return */
    if source_desc == target_desc {
        return AE_OK;
    }

    /* We know that source_desc is a buffer by now */
    buffer = (*source_desc).buffer.pointer as *mut u8;
    length = (*source_desc).buffer.length;

    /*
     * If target is a buffer of length zero or is a static buffer,
     * allocate a new buffer of the proper length
     */
    if ((*target_desc).buffer.length == 0)
        || ((*target_desc).common.flags & AOPOBJ_STATIC_POINTER) != 0
    {
        (*target_desc).buffer.pointer = ACPI_ALLOCATE(length as acpi_size);
        if (*target_desc).buffer.pointer.is_null() {
            return AE_NO_MEMORY;
        }

        (*target_desc).buffer.length = length;
    }

    /* Copy source buffer to target buffer */
    if length <= (*target_desc).buffer.length {
        /* Clear existing buffer and copy in the new one */
        core::ptr::write_bytes(
            (*target_desc).buffer.pointer as *mut u8,
            0,
            (*target_desc).buffer.length as usize,
        );
        core::ptr::copy_nonoverlapping(
            buffer,
            (*target_desc).buffer.pointer as *mut u8,
            length as usize,
        );

        /* ACPI_OBSOLETE_BEHAVIOR is a build-time conditional in the C source. */
    } else {
        /* Truncate the source, copy only what will fit */
        core::ptr::copy_nonoverlapping(
            buffer,
            (*target_desc).buffer.pointer as *mut u8,
            (*target_desc).buffer.length as usize,
        );
        // ACPI_DEBUG_PRINT((ACPI_DB_INFO, "Truncating source buffer from %X to %X\n", length, target_desc->buffer.length));
    }

    /* Copy flags */
    (*target_desc).buffer.flags = (*source_desc).buffer.flags;
    (*target_desc).common.flags &= !AOPOBJ_STATIC_POINTER;
    AE_OK
}

/*
 * FUNCTION:    acpi_ex_store_string_to_string
 *
 * PARAMETERS:  source_desc         - Source object to copy
 *              target_desc         - Destination object of the copy
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Copy a String object to another String object
 */
pub unsafe fn acpi_ex_store_string_to_string(
    source_desc: *mut acpi_operand_object,
    target_desc: *mut acpi_operand_object,
) -> acpi_status {
    let length: u32;
    let buffer: *mut u8;

    // ACPI_FUNCTION_TRACE_PTR(ex_store_string_to_string, source_desc);

    /* If Source and Target are the same, just return */
    if source_desc == target_desc {
        return AE_OK;
    }

    /* We know that source_desc is a string by now */
    buffer = (*source_desc).string.pointer as *mut u8;
    length = (*source_desc).string.length;

    /* Replace existing string value if it will fit and is not static. */
    if (length < (*target_desc).string.length)
        && ((*target_desc).common.flags & AOPOBJ_STATIC_POINTER) == 0
    {
        /* String will fit in existing non-static buffer. */
        core::ptr::write_bytes(
            (*target_desc).string.pointer as *mut u8,
            0,
            (*target_desc).string.length as usize + 1,
        );
        core::ptr::copy_nonoverlapping(
            buffer,
            (*target_desc).string.pointer as *mut u8,
            length as usize,
        );
    } else {
        /* Free the current buffer, then allocate a new buffer. */
        if !(*target_desc).string.pointer.is_null()
            && ((*target_desc).common.flags & AOPOBJ_STATIC_POINTER) == 0
        {
            ACPI_FREE((*target_desc).string.pointer);
        }

        (*target_desc).string.pointer =
            ACPI_ALLOCATE_ZEROED(length as acpi_size + 1);
        if (*target_desc).string.pointer.is_null() {
            return AE_NO_MEMORY;
        }

        (*target_desc).common.flags &= !AOPOBJ_STATIC_POINTER;
        core::ptr::copy_nonoverlapping(
            buffer,
            (*target_desc).string.pointer as *mut u8,
            length as usize,
        );
    }

    /* Set the new target length */
    (*target_desc).string.length = length;
    AE_OK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
