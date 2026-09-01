// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: cmfsize - Common get file size function
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// C dependencies:
// #include <acpi/acpi.h>
// #include "accommon.h"
// #include "acapps.h"

use core::ffi::{c_char, c_int, c_long, c_void};

pub type ACPI_FILE = *mut c_void;
pub type acpi_status = u32;

pub const _COMPONENT: u32 = ACPI_TOOLS;
// ACPI_MODULE_NAME("cmfsize")

const SEEK_SET: c_int = 0;
const SEEK_END: c_int = 2;
const ACPI_UINT32_MAX: u32 = u32::MAX;

extern "C" {
    static mut stderr: *mut c_void;

    fn ftell(stream: ACPI_FILE) -> c_long;
    fn fseek(stream: ACPI_FILE, offset: c_long, whence: c_int) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
}

extern "C" {
    static ACPI_TOOLS: u32;
}

#[inline]
fn ACPI_FAILURE(status: acpi_status) -> bool {
    (status & 0x80000000) != 0
}

/*******************************************************************************
 *
 * FUNCTION:    cm_get_file_size
 *
 * PARAMETERS:  file                    - Open file descriptor
 *
 * RETURN:      File Size. On error, -1 (ACPI_UINT32_MAX)
 *
 * DESCRIPTION: Get the size of a file. Uses seek-to-EOF. File must be open.
 *              Does not disturb the current file pointer.
 *
 ******************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn cm_get_file_size(file: ACPI_FILE) -> u32 {
    let file_size: c_long;
    let current_offset: c_long;
    let mut status: acpi_status;

    /* Save the current file pointer, seek to EOF to obtain file size */

    current_offset = ftell(file);
    if current_offset < 0 {
        fprintf(stderr, b"Could not get file offset\n\0".as_ptr() as *const c_char);
        return ACPI_UINT32_MAX;
    }

    status = fseek(file, 0, SEEK_END) as acpi_status;
    if ACPI_FAILURE(status) {
        fprintf(stderr, b"Could not set file offset\n\0".as_ptr() as *const c_char);
        return ACPI_UINT32_MAX;
    }

    file_size = ftell(file);
    if file_size < 0 {
        fprintf(stderr, b"Could not get file offset\n\0".as_ptr() as *const c_char);
        return ACPI_UINT32_MAX;
    }

    /* Restore original file pointer */

    status = fseek(file, current_offset, SEEK_SET) as acpi_status;
    if ACPI_FAILURE(status) {
        fprintf(stderr, b"Could not set file offset\n\0".as_ptr() as *const c_char);
        return ACPI_UINT32_MAX;
    }

    file_size as u32
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
