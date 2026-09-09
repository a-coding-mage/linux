/*
 * include/asm-xtensa/bootparam.h
 *
 * Definition of the Linux/Xtensa boot parameter structure
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2005  Tensilica Inc.
 *
 * (Concept borrowed from the 68K port)
 */

pub const BP_VERSION: u16 = 0x0001;

pub const BP_TAG_COMMAND_LINE: u16 = 0x1001; // command line (0-terminated string)
pub const BP_TAG_INITRD: u16 = 0x1002; // ramdisk addr and size (bp_meminfo)
pub const BP_TAG_MEMORY: u16 = 0x1003; // memory addr and size (bp_meminfo)
pub const BP_TAG_SERIAL_BAUDRATE: u16 = 0x1004; // baud rate of current console.
pub const BP_TAG_SERIAL_PORT: u16 = 0x1005; // serial device of current console
pub const BP_TAG_FDT: u16 = 0x1006; // flat device tree addr

pub const BP_TAG_FIRST: u16 = 0x7B0B; // first tag with a version number
pub const BP_TAG_LAST: u16 = 0x7E0B; // last tag

// All records are aligned to 4 bytes.
// C unsigned long is 32 bits on Xtensa.
#[repr(C)]
pub struct bp_tag {
    pub id: u16,      // tag id
    pub size: u16,    // size of this record excluding the structure
    pub data: [u32; 0], // data
}

#[repr(C)]
pub struct bp_meminfo {
    pub r#type: u32,
    pub start: u32,
    pub end: u32,
}

pub const MEMORY_TYPE_CONVENTIONAL: u32 = 0x1000;
pub const MEMORY_TYPE_NONE: u32 = 0x2000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
