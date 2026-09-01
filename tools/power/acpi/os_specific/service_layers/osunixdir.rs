// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: osunixdir - Unix directory access interfaces
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// C dependencies: acpi/acpi.h, stdio.h, stdlib.h, string.h, dirent.h,
// fnmatch.h, ctype.h, sys/stat.h

use libc::{
    c_char, c_int, c_void, calloc, closedir, dirent, fprintf, free, fnmatch, opendir, readdir,
    stat, stat as stat_fn, strcat, strcpy, strlen, DIR, S_ISDIR, stderr,
};

extern "C" {
    static REQUEST_DIR_ONLY: c_char;
    static REQUEST_FILE_ONLY: c_char;
}

/*
 * Allocated structure returned from os_open_directory
 */
#[repr(C)]
pub struct external_find_info {
    pub dir_pathname: *mut c_char,
    pub dir_ptr: *mut DIR,
    pub temp_buffer: [c_char; 256],
    pub wildcard_spec: *mut c_char,
    pub requested_file_type: c_char,
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_os_open_directory
 *
 * PARAMETERS:  dir_pathname        - Full pathname to the directory
 *              wildcard_spec       - string of the form "*.c", etc.
 *
 * RETURN:      A directory "handle" to be used in subsequent search operations.
 *              NULL returned on failure.
 *
 * DESCRIPTION: Open a directory in preparation for a wildcard search
 *
 ******************************************************************************/

#[no_mangle]
pub unsafe extern "C" fn acpi_os_open_directory(
    dir_pathname: *mut c_char,
    wildcard_spec: *mut c_char,
    requested_file_type: c_char,
) -> *mut c_void {
    let mut external_info: *mut external_find_info;
    let mut dir: *mut DIR;

    /* Allocate the info struct that will be returned to the caller */

    external_info = calloc(1, core::mem::size_of::<external_find_info>()) as *mut external_find_info;
    if external_info.is_null() {
        return core::ptr::null_mut();
    }

    /* Get the directory stream */

    dir = opendir(dir_pathname);
    if dir.is_null() {
        fprintf(
            stderr,
            b"Cannot open directory - %s\n\0".as_ptr() as *const c_char,
            dir_pathname,
        );
        free(external_info as *mut c_void);
        return core::ptr::null_mut();
    }

    /* Save the info in the return structure */

    (*external_info).wildcard_spec = wildcard_spec;
    (*external_info).requested_file_type = requested_file_type;
    (*external_info).dir_pathname = dir_pathname;
    (*external_info).dir_ptr = dir;
    external_info as *mut c_void
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_os_get_next_filename
 *
 * PARAMETERS:  dir_handle          - Created via acpi_os_open_directory
 *
 * RETURN:      Next filename matched. NULL if no more matches.
 *
 * DESCRIPTION: Get the next file in the directory that matches the wildcard
 *              specification.
 *
 ******************************************************************************/

#[no_mangle]
pub unsafe extern "C" fn acpi_os_get_next_filename(dir_handle: *mut c_void) -> *mut c_char {
    let mut external_info: *mut external_find_info = dir_handle as *mut external_find_info;
    let mut dir_entry: *mut dirent;
    let mut temp_str: *mut c_char;
    let mut str_len: c_int;
    let mut temp_stat: stat = core::mem::zeroed();
    let mut err: c_int;

    loop {
        dir_entry = readdir((*external_info).dir_ptr);
        if dir_entry.is_null() {
            break;
        }

        if fnmatch((*external_info).wildcard_spec, (*dir_entry).d_name.as_ptr(), 0) == 0 {
            if (*dir_entry).d_name[0] as c_int == '.' as c_int {
                continue;
            }

            str_len = (strlen((*dir_entry).d_name.as_ptr())
                + strlen((*external_info).dir_pathname)
                + 2) as c_int;

            temp_str = calloc(str_len as usize, 1) as *mut c_char;
            if temp_str.is_null() {
                fprintf(
                    stderr,
                    b"Could not allocate buffer for temporary string\n\0".as_ptr() as *const c_char,
                );
                return core::ptr::null_mut();
            }

            strcpy(temp_str, (*external_info).dir_pathname);
            strcat(temp_str, b"/\0".as_ptr() as *const c_char);
            strcat(temp_str, (*dir_entry).d_name.as_ptr());

            err = stat_fn(temp_str, &mut temp_stat);
            if err == -1 {
                fprintf(
                    stderr,
                    b"Cannot stat file (should not happen) - %s\n\0".as_ptr() as *const c_char,
                    temp_str,
                );
                free(temp_str as *mut c_void);
                return core::ptr::null_mut();
            }

            free(temp_str as *mut c_void);

            if (S_ISDIR(temp_stat.st_mode)
                && (*external_info).requested_file_type == REQUEST_DIR_ONLY)
                || (!S_ISDIR(temp_stat.st_mode)
                    && (*external_info).requested_file_type == REQUEST_FILE_ONLY)
            {
                /* copy to a temp buffer because dir_entry struct is on the stack */

                strcpy(
                    (*external_info).temp_buffer.as_mut_ptr(),
                    (*dir_entry).d_name.as_ptr(),
                );
                return (*external_info).temp_buffer.as_mut_ptr();
            }
        }
    }

    core::ptr::null_mut()
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_os_close_directory
 *
 * PARAMETERS:  dir_handle          - Created via acpi_os_open_directory
 *
 * RETURN:      None.
 *
 * DESCRIPTION: Close the open directory and cleanup.
 *
 ******************************************************************************/

#[no_mangle]
pub unsafe extern "C" fn acpi_os_close_directory(dir_handle: *mut c_void) {
    let mut external_info: *mut external_find_info = dir_handle as *mut external_find_info;

    /* Close the directory and free allocations */

    closedir((*external_info).dir_ptr);
    free(dir_handle);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
