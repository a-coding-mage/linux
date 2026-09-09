/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * DaVinci CPU type detection
 *
 * Author: Kevin Hilman, Deep Root Systems, LLC
 *
 * Defines the cpu_is_*() macros for runtime detection of DaVinci
 * device type.  In addition, if support for a given device is not
 * compiled in to the kernel, the macros return 0 so that
 * resulting code can be optimized out.
 *
 * 2009 (c) Deep Root Systems, LLC.
 */

// Dependency intent: declarations from "common.h" are supplied externally.

#[repr(C)]
pub struct davinci_id {
    pub variant: u8,          /* JTAG ID bits 31:28 */
    pub part_no: u16,         /* JTAG ID bits 27:12 */
    pub manufacturer: u16,    /* JTAG ID bits 11:1 */
    pub cpu_id: u32,
    pub name: *mut core::ffi::c_char,
}

/* Can use lower 16 bits of cpu id  for a variant when required */
pub const DAVINCI_CPU_ID_DA850: u32 = 0x0850_0000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
