// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: apfiles - File-related functions for acpidump utility
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

/* Dependencies from acpidump.h and the C runtime are declared externally. */

use core::ffi::{c_char, c_int, c_uint, c_void};

type ACPI_FILE = *mut c_void;
type acpi_size = usize;
type u32 = c_uint;

const ACPI_NAMESEG_SIZE: usize = 4;
const ACPI_UINT32_MAX: u32 = 0xFFFF_FFFF;

#[repr(C)]
pub struct acpi_table_header {
    pub signature: [c_char; 4],
    pub length: u32,
}

#[repr(C)]
pub struct stat {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut stderr: ACPI_FILE;
    static mut stdin: ACPI_FILE;
    static mut gbl_output_file: ACPI_FILE;
    static mut gbl_output_filename: *mut c_char;
    static mut gbl_verbose_mode: bool;

    static ACPI_RSDP_NAME: [c_char; 4];
    static FILE_SUFFIX_BINARY_TABLE: *const c_char;

    fn stat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
    fn fprintf(stream: ACPI_FILE, format: *const c_char, ...) -> c_int;
    fn fgetc(stream: ACPI_FILE) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> ACPI_FILE;
    fn fclose(stream: ACPI_FILE) -> c_int;
    fn fwrite(ptr: *const c_void, size: acpi_size, nmemb: acpi_size, stream: ACPI_FILE) -> acpi_size;
    fn fread(ptr: *mut c_void, size: acpi_size, nmemb: acpi_size, stream: ACPI_FILE) -> acpi_size;
    fn snprintf(s: *mut c_char, n: acpi_size, format: *const c_char, ...) -> c_int;
    fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn tolower(c: c_int) -> c_int;

    fn ap_get_table_length(table: *mut acpi_table_header) -> u32;
    fn ACPI_VALIDATE_RSDP_SIG(signature: *const c_char) -> bool;
    fn cm_get_file_size(file: ACPI_FILE) -> u32;
    fn ACPI_ALLOCATE_ZEROED(size: acpi_size) -> *mut c_void;
    fn ACPI_FREE(ptr: *mut c_void);
}

unsafe fn ACPI_COPY_NAMESEG(dest: *mut c_char, src: *const c_char) {
    let mut i: usize = 0;

    while i < ACPI_NAMESEG_SIZE {
        unsafe {
            *dest.add(i) = *src.add(i);
        }
        i += 1;
    }
}

/******************************************************************************
 *
 * FUNCTION:    ap_is_existing_file
 *
 * PARAMETERS:  pathname            - Output filename
 *
 * RETURN:      0 on success
 *
 * DESCRIPTION: Query for file overwrite if it already exists.
 *
 ******************************************************************************/

unsafe fn ap_is_existing_file(pathname: *mut c_char) -> c_int {
    /*
     * Original C condition:
     * #if !defined(_GNU_EFI) && !defined(_EDK2_EFI)
     */
    #[cfg(not(any(GNU_EFI, EDK2_EFI)))]
    unsafe {
        let mut stat_info: stat = core::mem::zeroed();
        let mut in_char: c_int;

        if stat(pathname, &mut stat_info) == 0 {
            fprintf(
                stderr,
                c"Target path already exists, overwrite? [y|n] ".as_ptr(),
            );

            in_char = fgetc(stdin);
            if in_char == '\n' as c_int {
                in_char = fgetc(stdin);
            }

            if in_char != 'y' as c_int && in_char != 'Y' as c_int {
                return -1;
            }
        }
    }

    0
}

/******************************************************************************
 *
 * FUNCTION:    ap_open_output_file
 *
 * PARAMETERS:  pathname            - Output filename
 *
 * RETURN:      Open file handle
 *
 * DESCRIPTION: Open a text output file for acpidump. Checks if file already
 *              exists.
 *
 ******************************************************************************/

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ap_open_output_file(pathname: *mut c_char) -> c_int {
    let file: ACPI_FILE;

    /* If file exists, prompt for overwrite */

    if unsafe { ap_is_existing_file(pathname) } != 0 {
        return -1;
    }

    /* Point stdout to the file */

    file = unsafe { fopen(pathname, c"w".as_ptr()) };
    if file.is_null() {
        unsafe {
            fprintf(
                stderr,
                c"Could not open output file: %s\n".as_ptr(),
                pathname,
            );
        }
        return -1;
    }

    /* Save the file and path */

    unsafe {
        gbl_output_file = file;
        gbl_output_filename = pathname;
    }
    0
}

/******************************************************************************
 *
 * FUNCTION:    ap_write_to_binary_file
 *
 * PARAMETERS:  table               - ACPI table to be written
 *              instance            - ACPI table instance no. to be written
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Write an ACPI table to a binary file. Builds the output
 *              filename from the table signature.
 *
 ******************************************************************************/

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ap_write_to_binary_file(
    table: *mut acpi_table_header,
    instance: u32,
) -> c_int {
    let mut filename: [c_char; ACPI_NAMESEG_SIZE + 16] = [0; ACPI_NAMESEG_SIZE + 16];
    let mut instance_str: [c_char; 16] = [0; 16];
    let file: ACPI_FILE;
    let actual: acpi_size;
    let table_length: u32;

    /* Obtain table length */

    table_length = unsafe { ap_get_table_length(table) };

    /* Construct lower-case filename from the table local signature */

    if unsafe { ACPI_VALIDATE_RSDP_SIG((*table).signature.as_ptr()) } {
        unsafe {
            ACPI_COPY_NAMESEG(filename.as_mut_ptr(), ACPI_RSDP_NAME.as_ptr());
        }
    } else {
        unsafe {
            ACPI_COPY_NAMESEG(filename.as_mut_ptr(), (*table).signature.as_ptr());
        }
    }

    filename[0] = unsafe { tolower(filename[0] as c_int) as c_char };
    filename[1] = unsafe { tolower(filename[1] as c_int) as c_char };
    filename[2] = unsafe { tolower(filename[2] as c_int) as c_char };
    filename[3] = unsafe { tolower(filename[3] as c_int) as c_char };
    filename[ACPI_NAMESEG_SIZE] = 0;

    /* Handle multiple SSDts - create different filenames for each */

    if instance > 0 {
        unsafe {
            snprintf(
                instance_str.as_mut_ptr(),
                core::mem::size_of_val(&instance_str),
                c"%u".as_ptr(),
                instance,
            );
            strcat(filename.as_mut_ptr(), instance_str.as_ptr());
        }
    }

    unsafe {
        strcat(filename.as_mut_ptr(), FILE_SUFFIX_BINARY_TABLE);
    }

    if unsafe { gbl_verbose_mode } {
        unsafe {
            fprintf(
                stderr,
                c"Writing [%4.4s] to binary file: %s 0x%X (%u) bytes\n".as_ptr(),
                (*table).signature.as_ptr(),
                filename.as_ptr(),
                (*table).length,
                (*table).length,
            );
        }
    }

    /* Open the file and dump the entire table in binary mode */

    file = unsafe { fopen(filename.as_ptr(), c"wb".as_ptr()) };
    if file.is_null() {
        unsafe {
            fprintf(
                stderr,
                c"Could not open output file: %s\n".as_ptr(),
                filename.as_ptr(),
            );
        }
        return -1;
    }

    actual = unsafe { fwrite(table as *const c_void, 1, table_length as acpi_size, file) };
    if actual != table_length as acpi_size {
        unsafe {
            fprintf(
                stderr,
                c"Error writing binary output file: %s\n".as_ptr(),
                filename.as_ptr(),
            );
            fclose(file);
        }
        return -1;
    }

    unsafe {
        fclose(file);
    }
    0
}

/******************************************************************************
 *
 * FUNCTION:    ap_get_table_from_file
 *
 * PARAMETERS:  pathname            - File containing the binary ACPI table
 *              out_file_size       - Where the file size is returned
 *
 * RETURN:      Buffer containing the ACPI table. NULL on error.
 *
 * DESCRIPTION: Open a file and read it entirely into a new buffer
 *
 ******************************************************************************/

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ap_get_table_from_file(
    pathname: *mut c_char,
    out_file_size: *mut u32,
) -> *mut acpi_table_header {
    let mut buffer: *mut acpi_table_header = core::ptr::null_mut();
    let file: ACPI_FILE;
    let file_size: u32;
    let actual: acpi_size;

    /* Must use binary mode */

    file = unsafe { fopen(pathname, c"rb".as_ptr()) };
    if file.is_null() {
        unsafe {
            fprintf(
                stderr,
                c"Could not open input file: %s\n".as_ptr(),
                pathname,
            );
        }
        return core::ptr::null_mut();
    }

    /* Need file size to allocate a buffer */

    file_size = unsafe { cm_get_file_size(file) };
    if file_size == ACPI_UINT32_MAX {
        unsafe {
            fprintf(
                stderr,
                c"Could not get input file size: %s\n".as_ptr(),
                pathname,
            );
        }
        unsafe {
            fclose(file);
        }
        return buffer;
    }

    /* Allocate a buffer for the entire file */

    buffer = unsafe { ACPI_ALLOCATE_ZEROED(file_size as acpi_size) as *mut acpi_table_header };
    if buffer.is_null() {
        unsafe {
            fprintf(
                stderr,
                c"Could not allocate file buffer of size: %u\n".as_ptr(),
                file_size,
            );
        }
        unsafe {
            fclose(file);
        }
        return buffer;
    }

    /* Read the entire file */

    actual = unsafe { fread(buffer as *mut c_void, 1, file_size as acpi_size, file) };
    if actual != file_size as acpi_size {
        unsafe {
            fprintf(
                stderr,
                c"Could not read input file: %s\n".as_ptr(),
                pathname,
            );
            ACPI_FREE(buffer as *mut c_void);
        }
        buffer = core::ptr::null_mut();
        unsafe {
            fclose(file);
        }
        return buffer;
    }

    unsafe {
        *out_file_size = file_size;
    }

    unsafe {
        fclose(file);
    }
    buffer
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
