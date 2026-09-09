/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2002-2003  David McCullough <davidm@snapgear.com>
 * Copyright (C) 1998       Kenneth Albanowski <kjahds@kjahds.com>
 *                          The Silver Hammer Group, Ltd.
 *
 * This file provides the definitions and structures needed to
 * support uClinux flat-format executables.
 */

pub const FLAT_VERSION: u32 = 0x0000_0004;

/*
 * To make everything easier to port and manage cross platform
 * development, all fields are in network byte order.
 */

#[repr(C)]
pub struct flat_hdr {
    pub magic: [std::ffi::c_char; 4],
    pub rev: u32,          /* version (as above) */
    pub entry: u32,        /* Offset of first executable instruction
                              with text segment from beginning of file */
    pub data_start: u32,   /* Offset of data segment from beginning of
                              file */
    pub data_end: u32,     /* Offset of end of data segment from beginning
                              of file */
    pub bss_end: u32,      /* Offset of end of bss segment from beginning
                              of file */

    /* (It is assumed that data_end through bss_end forms the bss segment.) */

    pub stack_size: u32,   /* Size of stack, in bytes */
    pub reloc_start: u32,  /* Offset of relocation records from beginning of
                              file */
    pub reloc_count: u32,  /* Number of relocation records */
    pub flags: u32,
    pub build_date: u32,   /* When the program/library was built */
    pub filler: [u32; 5],  /* Reservered, set to zero */
}

pub const FLAT_FLAG_RAM: u32 = 0x0001;    /* load program entirely into RAM */
pub const FLAT_FLAG_GOTPIC: u32 = 0x0002; /* program is PIC with GOT */
pub const FLAT_FLAG_GZIP: u32 = 0x0004;   /* all but the header is compressed */
pub const FLAT_FLAG_GZDATA: u32 = 0x0008; /* only data/relocs are compressed (for XIP) */
pub const FLAT_FLAG_KTRACE: u32 = 0x0010; /* output useful kernel trace for debugging */

/*
 * While it would be nice to keep this header clean, users of older
 * tools still need this support in the kernel. So this section is
 * purely for compatibility with old tool chains.
 *
 * DO NOT make changes or enhancements to the old format please, just work
 *        with the format above, except to fix bugs with old format support.
 */

pub const OLD_FLAT_VERSION: u32 = 0x0000_0002;
pub const OLD_FLAT_RELOC_TYPE_TEXT: u32 = 0;
pub const OLD_FLAT_RELOC_TYPE_DATA: u32 = 1;
pub const OLD_FLAT_RELOC_TYPE_BSS: u32 = 2;

/* C bit-field order depends on the target's endian configuration. */
#[repr(C)]
pub struct flat_v2_reloc_bits {
    pub offset: i32,
    pub type_: u32,
}

#[repr(C)]
pub union flat_v2_reloc_t {
    pub value: u32,
    pub reloc: flat_v2_reloc_bits,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
