// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: dbhistry - debugger HISTORY command
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 ******************************************************************************/

// C dependencies: acpi/acpi.h, accommon.h, and acdebug.h.

use core::ffi::{c_char, c_int, c_ulong, c_void};

const HI_NO_HISTORY: u32 = 0;
const HI_RECORD_HISTORY: u32 = 1;
const HISTORY_SIZE: usize = 40;

#[repr(C)]
struct HISTORY_INFO {
    command: *mut c_char,
    cmd_num: u32,
}

extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strtoul(s: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn acpi_os_free(memory: *mut c_void);
    fn acpi_os_allocate(size: usize) -> *mut c_char;
    fn acpi_os_printf(format: *const c_char, ...);
    static mut acpi_gbl_next_cmd_num: u32;
}

static mut acpi_gbl_history_buffer: [HISTORY_INFO; HISTORY_SIZE] =
    [HISTORY_INFO { command: core::ptr::null_mut(), cmd_num: 0 }; HISTORY_SIZE];
static mut acpi_gbl_lo_history: u16 = 0;
static mut acpi_gbl_num_history: u16 = 0;
static mut acpi_gbl_next_history_index: u16 = 0;

/*******************************************************************************
 *
 * FUNCTION:    acpi_db_add_to_history
 *
 * PARAMETERS:  command_line    - Command to add
 *
 * RETURN:      None
 *
 * DESCRIPTION: Add a command line to the history buffer.
 *
 ******************************************************************************/

pub unsafe extern "C" fn acpi_db_add_to_history(command_line: *mut c_char) {
    let cmd_len: u16;
    let mut buffer_len: u16;

    /* Put command into the next available slot */

    cmd_len = strlen(command_line) as u16;
    if cmd_len == 0 {
        return;
    }

    let index = acpi_gbl_next_history_index as usize;
    if !acpi_gbl_history_buffer[index].command.is_null() {
        buffer_len = strlen(acpi_gbl_history_buffer[index].command) as u16;

        if cmd_len > buffer_len {
            acpi_os_free(acpi_gbl_history_buffer[index].command as *mut c_void);
            acpi_gbl_history_buffer[index].command = acpi_os_allocate(cmd_len as usize + 1);
        }
    } else {
        acpi_gbl_history_buffer[index].command = acpi_os_allocate(cmd_len as usize + 1);
    }

    strcpy(acpi_gbl_history_buffer[index].command, command_line);

    acpi_gbl_history_buffer[index].cmd_num = acpi_gbl_next_cmd_num;

    /* Adjust indexes */

    if acpi_gbl_num_history == HISTORY_SIZE as u16
        && acpi_gbl_next_history_index == acpi_gbl_lo_history
    {
        acpi_gbl_lo_history += 1;
        if acpi_gbl_lo_history >= HISTORY_SIZE as u16 {
            acpi_gbl_lo_history = 0;
        }
    }

    acpi_gbl_next_history_index += 1;
    if acpi_gbl_next_history_index >= HISTORY_SIZE as u16 {
        acpi_gbl_next_history_index = 0;
    }

    acpi_gbl_next_cmd_num += 1;
    if acpi_gbl_num_history < HISTORY_SIZE as u16 {
        acpi_gbl_num_history += 1;
    }
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_db_display_history
 *
 * PARAMETERS:  None
 *
 * RETURN:      None
 *
 * DESCRIPTION: Display the contents of the history buffer
 *
 ******************************************************************************/

pub unsafe extern "C" fn acpi_db_display_history() {
    let mut history_index = acpi_gbl_lo_history;

    /* Dump entire history buffer */

    for _i in 0..acpi_gbl_num_history {
        let entry = &acpi_gbl_history_buffer[history_index as usize];
        if !entry.command.is_null() {
            acpi_os_printf(b"%3u %s\0".as_ptr() as *const c_char, entry.cmd_num, entry.command);
        }

        history_index += 1;
        if history_index >= HISTORY_SIZE as u16 {
            history_index = 0;
        }
    }
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_db_get_from_history
 *
 * PARAMETERS:  command_num_arg         - String containing the number of the
 *                                        command to be retrieved
 *
 * RETURN:      Pointer to the retrieved command. Null on error.
 *
 * DESCRIPTION: Get a command from the history buffer
 *
 ******************************************************************************/

pub unsafe extern "C" fn acpi_db_get_from_history(command_num_arg: *mut c_char) -> *mut c_char {
    let cmd_num: u32;

    if command_num_arg.is_null() {
        cmd_num = acpi_gbl_next_cmd_num - 1;
    } else {
        cmd_num = strtoul(command_num_arg, core::ptr::null_mut(), 0) as u32;
    }

    acpi_db_get_history_by_index(cmd_num)
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_db_get_history_by_index
 *
 * PARAMETERS:  cmd_num             - Index of the desired history entry.
 *                                    Values are 0...(acpi_gbl_next_cmd_num - 1)
 *
 * RETURN:      Pointer to the retrieved command. Null on error.
 *
 * DESCRIPTION: Get a command from the history buffer
 *
 ******************************************************************************/

pub unsafe extern "C" fn acpi_db_get_history_by_index(cmd_num: u32) -> *mut c_char {
    let mut history_index = acpi_gbl_lo_history;

    /* Search history buffer */

    for _i in 0..acpi_gbl_num_history {
        let entry = &acpi_gbl_history_buffer[history_index as usize];
        if entry.cmd_num == cmd_num {
            /* Found the command, return it */
            return entry.command;
        }

        /* History buffer is circular */

        history_index += 1;
        if history_index >= HISTORY_SIZE as u16 {
            history_index = 0;
        }
    }

    acpi_os_printf(b"Invalid history number: %u\n\0".as_ptr() as *const c_char, history_index);
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
