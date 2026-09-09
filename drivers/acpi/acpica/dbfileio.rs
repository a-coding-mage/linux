// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*******************************************************************************
 *
 * Module Name: dbfileio - Debugger file I/O commands. These can't usually
 *              be used when running the debugger in Ring 0 (Kernel mode)
 *
 ******************************************************************************/

// C dependencies supplied by the surrounding ACPICA translation unit.

pub const _COMPONENT: u32 = ACPI_CA_DEBUGGER;

// ACPI_APPLICATION and ACPI_DEBUGGER are build-time C conditions. The code
// below is retained under equivalent feature conditions where applicable.

#[cfg(all(feature = "ACPI_APPLICATION", feature = "ACPI_DEBUGGER"))]
pub unsafe fn acpi_db_close_debug_file() {
    if !acpi_gbl_debug_file.is_null() {
        fclose(acpi_gbl_debug_file);
        acpi_gbl_debug_file = core::ptr::null_mut();
        acpi_gbl_db_output_to_file = FALSE;
        acpi_os_printf(
            b"Debug output file %s closed\n\0".as_ptr() as *const core::ffi::c_char,
            acpi_gbl_db_debug_filename,
        );
    }
}

#[cfg(all(feature = "ACPI_APPLICATION", feature = "ACPI_DEBUGGER"))]
pub unsafe fn acpi_db_open_debug_file(name: *mut core::ffi::c_char) {
    acpi_db_close_debug_file();
    acpi_gbl_debug_file = fopen(
        name,
        b"w+\0".as_ptr() as *const core::ffi::c_char,
    );
    if acpi_gbl_debug_file.is_null() {
        acpi_os_printf(
            b"Could not open debug file %s\n\0".as_ptr() as *const core::ffi::c_char,
            name,
        );
        return;
    }

    acpi_os_printf(
        b"Debug output file %s opened\n\0".as_ptr() as *const core::ffi::c_char,
        name,
    );
    acpi_ut_safe_strncpy(
        acpi_gbl_db_debug_filename,
        name,
        core::mem::size_of_val(&acpi_gbl_db_debug_filename),
    );
    acpi_gbl_db_output_to_file = TRUE;
}

pub unsafe fn acpi_db_load_tables(
    list_head: *mut acpi_new_table_desc,
) -> acpi_status {
    let mut status: acpi_status;
    let mut table_list_head: *mut acpi_new_table_desc;
    let mut table: *mut acpi_table_header;

    /* Load all ACPI tables in the list */

    table_list_head = list_head;
    while !table_list_head.is_null() {
        table = (*table_list_head).table;

        status = acpi_load_table(table, core::ptr::null_mut());
        if ACPI_FAILURE(status) {
            if status == AE_ALREADY_EXISTS {
                acpi_os_printf(
                    b"Table %4.4s is already installed\n\0".as_ptr()
                        as *const core::ffi::c_char,
                    (*table).signature.as_ptr(),
                );
            } else {
                acpi_os_printf(
                    b"Could not install table, %s\n\0".as_ptr()
                        as *const core::ffi::c_char,
                    acpi_format_exception(status),
                );
            }

            return status;
        }

        acpi_os_printf(
            b"Acpi table [%4.4s] successfully installed and loaded\n\0".as_ptr()
                as *const core::ffi::c_char,
            (*table).signature.as_ptr(),
        );

        table_list_head = (*table_list_head).next;
    }

    AE_OK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
