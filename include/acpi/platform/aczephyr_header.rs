/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/******************************************************************************
 *
 * Module Name: aczephyr.h - OS specific defines, etc.
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// C header guard: __ACZEPHYR_H__

pub const ACPI_MACHINE_WIDTH: u32 = 64;

// C preprocessor configuration markers:
// #define ACPI_NO_ERROR_MESSAGES
// #undef ACPI_DEBUG_OUTPUT
// #define ACPI_USE_SYSTEM_CLIBRARY
// #undef ACPI_DBG_TRACK_ALLOCATIONS
// #define ACPI_SINGLE_THREADED
// #define ACPI_USE_NATIVE_RSDP_POINTER

// C dependencies supplied by the surrounding build:
// <zephyr/kernel.h>, <zephyr/device.h>, <stdio.h>, <stdlib.h>, <string.h>,
// <ctype.h>, <zephyr/fs/fs.h>, <zephyr/sys/printk.h>, <zephyr/sys/__assert.h>

/******************************************************************************
 *
 * FUNCTION:    acpi_enable_dbg_print
 *
 * PARAMETERS:  Enable,                 - Enable/Disable debug print
 *
 * RETURN:      None
 *
 * DESCRIPTION: Enable/disable debug print
 *
 *****************************************************************************/

unsafe extern "C" {
    pub fn acpi_enable_dbg_print(enable: bool);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
