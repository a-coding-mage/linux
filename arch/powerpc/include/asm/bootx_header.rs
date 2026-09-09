/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This file describes the structure passed from the BootX application
 * (for MacOS) when it is used to boot Linux.
 *
 * Written by Benjamin Herrenschmidt.
 */

/* Corresponds to <uapi/asm/bootx.h>. */

/* (*) The format of the colormap is 256 * 3 * 2 bytes. Each color index
 * is represented by 3 short words containing a 16 bits (unsigned) color
 * component. Later versions may contain the gamma table for direct-color
 * devices here.
 */
pub const BOOTX_COLORTABLE_SIZE: usize = 256usize * 3usize * 2usize;

/* BootX passes the device-tree using a format that comes from earlier
 * ppc32 kernels. This used to match what is in prom.h, but not anymore
 * so we now define it here
 */
#[repr(C)]
pub struct bootx_dt_prop {
    pub name: u32,
    pub length: i32,
    pub value: u32,
    pub next: u32,
}

#[repr(C)]
pub struct bootx_dt_node {
    pub unused0: u32,
    pub unused1: u32,
    pub phandle: u32, /* not really available */
    pub unused2: u32,
    pub unused3: u32,
    pub unused4: u32,
    pub unused5: u32,
    pub full_name: u32,
    pub properties: u32,
    pub parent: u32,
    pub child: u32,
    pub sibling: u32,
    pub next: u32,
    pub allnext: u32,
}

unsafe extern "C" {
    pub fn bootx_init(r4: usize, phys: usize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
