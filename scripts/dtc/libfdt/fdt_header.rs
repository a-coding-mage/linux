/* SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause) */
/*
 * libfdt - Flat Device Tree manipulation
 * Copyright (C) 2006 David Gibson, IBM Corporation.
 * Copyright 2012 Kim Phillips, Freescale Semiconductor.
 *
 * C header guard: FDT_H
 * The fdt32_t and fdt64_t types are supplied by the surrounding libfdt
 * translation.
 */

#[repr(C)]
pub struct fdt_header {
    pub magic: fdt32_t,             /* magic word FDT_MAGIC */
    pub totalsize: fdt32_t,         /* total size of DT block */
    pub off_dt_struct: fdt32_t,     /* offset to structure */
    pub off_dt_strings: fdt32_t,    /* offset to strings */
    pub off_mem_rsvmap: fdt32_t,    /* offset to memory reserve map */
    pub version: fdt32_t,           /* format version */
    pub last_comp_version: fdt32_t, /* last compatible version */

    /* version 2 fields below */
    pub boot_cpuid_phys: fdt32_t,   /* Which physical CPU id we're booting on */
    /* version 3 fields below */
    pub size_dt_strings: fdt32_t,   /* size of the strings block */

    /* version 17 fields below */
    pub size_dt_struct: fdt32_t,     /* size of the structure block */
}

#[repr(C)]
pub struct fdt_reserve_entry {
    pub address: fdt64_t,
    pub size: fdt64_t,
}

#[repr(C)]
pub struct fdt_node_header {
    pub tag: fdt32_t,
    /* C flexible array member: char name[]; */
    pub name: [core::ffi::c_char; 0],
}

#[repr(C)]
pub struct fdt_property {
    pub tag: fdt32_t,
    pub len: fdt32_t,
    pub nameoff: fdt32_t,
    /* C flexible array member: char data[]; */
    pub data: [core::ffi::c_char; 0],
}

pub const FDT_MAGIC: u32 = 0xd00dfeed; /* 4: version, 4: total size */
pub const FDT_TAGSIZE: usize = core::mem::size_of::<fdt32_t>();

pub const FDT_BEGIN_NODE: u32 = 0x1; /* Start node: full name */
pub const FDT_END_NODE: u32 = 0x2;   /* End node */
pub const FDT_PROP: u32 = 0x3;       /* Property: name off, size, content */
pub const FDT_NOP: u32 = 0x4;        /* nop */
pub const FDT_END: u32 = 0x9;

pub const FDT_V1_SIZE: usize = 7 * core::mem::size_of::<fdt32_t>();
pub const FDT_V2_SIZE: usize = FDT_V1_SIZE + core::mem::size_of::<fdt32_t>();
pub const FDT_V3_SIZE: usize = FDT_V2_SIZE + core::mem::size_of::<fdt32_t>();
pub const FDT_V16_SIZE: usize = FDT_V3_SIZE;
pub const FDT_V17_SIZE: usize = FDT_V16_SIZE + core::mem::size_of::<fdt32_t>();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
