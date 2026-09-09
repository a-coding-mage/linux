// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: utbuffer - Buffer dump routines
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// Dependencies supplied by the surrounding ACPICA translation unit.

const _COMPONENT: u32 = ACPI_UTILITIES;

/*******************************************************************************
 *
 * FUNCTION:    acpi_ut_dump_buffer
 *
 * DESCRIPTION: Generic dump buffer in both hex and ascii.
 *
 ******************************************************************************/
pub unsafe fn acpi_ut_dump_buffer(
    buffer: *mut u8,
    count: u32,
    mut display: u32,
    base_offset: u32,
) {
    let mut i: u32 = 0;
    let mut j: u32;
    let mut temp32: u32;
    let mut buf_char: u8;
    let display_data_only = display & DB_DISPLAY_DATA_ONLY;

    display &= !DB_DISPLAY_DATA_ONLY;
    if buffer.is_null() {
        acpi_os_printf(c"Null Buffer Pointer in DumpBuffer!\n".as_ptr());
        return;
    }

    if (count < 4) || ((count & 0x01) != 0) {
        display = DB_BYTE_DISPLAY;
    }

    /* Nasty little dump buffer routine! */
    while i < count {
        if display_data_only == 0 {
            acpi_os_printf(c"%8.4X: ".as_ptr(), base_offset.wrapping_add(i));
        }

        for j in (0..16).step_by(display as usize) {
            if i.wrapping_add(j) >= count {
                acpi_os_printf(c"%*s".as_ptr(), (display * 2 + 1), c" ".as_ptr());
                continue;
            }

            match display {
                DB_WORD_DISPLAY => {
                    ACPI_MOVE_16_TO_32(&mut temp32, buffer.add((i + j) as usize));
                    acpi_os_printf(c"%04X ".as_ptr(), temp32);
                }
                DB_DWORD_DISPLAY => {
                    ACPI_MOVE_32_TO_32(&mut temp32, buffer.add((i + j) as usize));
                    acpi_os_printf(c"%08X ".as_ptr(), temp32);
                }
                DB_QWORD_DISPLAY => {
                    ACPI_MOVE_32_TO_32(&mut temp32, buffer.add((i + j) as usize));
                    acpi_os_printf(c"%08X".as_ptr(), temp32);
                    ACPI_MOVE_32_TO_32(&mut temp32, buffer.add((i + j + 4) as usize));
                    acpi_os_printf(c"%08X ".as_ptr(), temp32);
                }
                _ => {
                    acpi_os_printf(c"%02X ".as_ptr(), *buffer.add((i + j) as usize));
                }
            }
        }

        /* Print the ASCII equivalent characters but watch out for bad unprintable ones. */
        if display_data_only == 0 {
            acpi_os_printf(c" ".as_ptr());
            for j in 0..16 {
                if i.wrapping_add(j) >= count {
                    acpi_os_printf(c"\n".as_ptr());
                    return;
                }
                if j == 0 {
                    acpi_os_printf(c"// ".as_ptr());
                }
                buf_char = *buffer.add((i + j) as usize);
                if isprint(buf_char as i32) != 0 {
                    acpi_os_printf(c"%c".as_ptr(), buf_char as i32);
                } else {
                    acpi_os_printf(c".".as_ptr());
                }
            }
            acpi_os_printf(c"\n".as_ptr());
        }
        i = i.wrapping_add(16);
    }
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ut_debug_dump_buffer
 *
 ******************************************************************************/
pub unsafe fn acpi_ut_debug_dump_buffer(
    buffer: *mut u8,
    count: u32,
    display: u32,
    component_id: u32,
) {
    if (ACPI_LV_TABLES & acpi_dbg_level) == 0 || (component_id & acpi_dbg_layer) == 0 {
        return;
    }
    acpi_ut_dump_buffer(buffer, count, display, 0);
}

#[cfg(feature = "ACPI_APPLICATION")]
pub unsafe fn acpi_ut_dump_buffer_to_file(
    file: ACPI_FILE,
    buffer: *mut u8,
    count: u32,
    mut display: u32,
    base_offset: u32,
) {
    let mut i: u32 = 0;
    let mut temp32: u32;
    if buffer.is_null() {
        fprintf(file, c"Null Buffer Pointer in DumpBuffer!\n".as_ptr());
        return;
    }
    if (count < 4) || ((count & 0x01) != 0) {
        display = DB_BYTE_DISPLAY;
    }
    while i < count {
        fprintf(file, c"%8.4X: ".as_ptr(), base_offset.wrapping_add(i));
        for j in (0..16).step_by(display as usize) {
            if i + j >= count {
                fprintf(file, c"%*s".as_ptr(), display * 2 + 1, c" ".as_ptr());
                continue;
            }
            match display {
                DB_WORD_DISPLAY => { ACPI_MOVE_16_TO_32(&mut temp32, buffer.add((i+j) as usize)); fprintf(file, c"%04X ".as_ptr(), temp32); }
                DB_DWORD_DISPLAY => { ACPI_MOVE_32_TO_32(&mut temp32, buffer.add((i+j) as usize)); fprintf(file, c"%08X ".as_ptr(), temp32); }
                DB_QWORD_DISPLAY => { ACPI_MOVE_32_TO_32(&mut temp32, buffer.add((i+j) as usize)); fprintf(file, c"%08X".as_ptr(), temp32); ACPI_MOVE_32_TO_32(&mut temp32, buffer.add((i+j+4) as usize)); fprintf(file, c"%08X ".as_ptr(), temp32); }
                _ => fprintf(file, c"%02X ".as_ptr(), *buffer.add((i+j) as usize)),
            }
        }
        fprintf(file, c" ".as_ptr());
        for j in 0..16 {
            if i + j >= count { fprintf(file, c"\n".as_ptr()); return; }
            let ch = *buffer.add((i+j) as usize);
            if isprint(ch as i32) != 0 { fprintf(file, c"%c".as_ptr(), ch as i32); } else { fprintf(file, c".".as_ptr()); }
        }
        fprintf(file, c"\n".as_ptr());
        i = i.wrapping_add(16);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
