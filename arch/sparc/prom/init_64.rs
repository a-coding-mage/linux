// SPDX-License-Identifier: GPL-2.0
/*
 * init.c:  Initialize internal variables used by the PROM
 *          library functions.
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 * Copyright (C) 1996,1997 Jakub Jelinek (jj@sunsite.mff.cuni.cz)
 */

use core::ffi::{c_char, c_int, c_void};

// C dependencies: <linux/kernel.h>, <linux/init.h>, <linux/string.h>,
// <linux/ctype.h>, <asm/openprom.h>, and <asm/oplib.h>.

pub type phandle = u32;
pub type s32 = i32;

extern "C" {
    fn prom_cif_init(cif_handler: *mut c_void);
    fn prom_finddevice(path: *const c_char) -> phandle;
    fn prom_halt() -> !;
    fn prom_getint(node: phandle, property: *const c_char) -> c_int;
    fn prom_getstring(
        node: phandle,
        property: *const c_char,
        buf: *mut c_char,
        buflen: usize,
    );
    fn prom_printf(format: *const c_char, ...);
    fn printk(format: *const c_char, ...);

    static prom_chosen_path: *const c_char;
    static prom_root_compatible: *const c_char;
}

/* OBP version string. */
pub static mut prom_version: [c_char; 80] = [0; 80];

/* The root node of the prom device tree. */
pub static mut prom_stdout: c_int = 0;
pub static mut prom_chosen_node: phandle = 0;

/* You must call prom_init() before you attempt to use any of the
 * routines in the prom library.
 * It gets passed the pointer to the PROM vector.
 */
pub unsafe extern "C" fn prom_init(cif_handler: *mut c_void) {
    let mut node: phandle;

    prom_cif_init(cif_handler);

    prom_chosen_node = prom_finddevice(prom_chosen_path);
    if prom_chosen_node == 0 || (prom_chosen_node as s32) == -1 {
        prom_halt();
    }

    prom_stdout = prom_getint(prom_chosen_node, b"stdout\0".as_ptr() as *const c_char);

    node = prom_finddevice(b"/openprom\0".as_ptr() as *const c_char);
    if node == 0 || (node as s32) == -1 {
        prom_halt();
    }

    prom_getstring(
        node,
        b"version\0".as_ptr() as *const c_char,
        prom_version.as_mut_ptr(),
        core::mem::size_of_val(&prom_version),
    );

    prom_printf(b"\n\0".as_ptr() as *const c_char);
}

pub unsafe extern "C" fn prom_init_report() {
    printk(
        b"PROMLIB: Sun IEEE Boot Prom '%s'\n\0".as_ptr() as *const c_char,
        prom_version.as_ptr(),
    );
    printk(
        b"PROMLIB: Root node compatible: %s\n\0".as_ptr() as *const c_char,
        prom_root_compatible,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
