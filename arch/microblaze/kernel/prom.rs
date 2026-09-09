// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Procedures for creating, accessing and interpreting the device tree.
 *
 * Paul Mackerras\tAugust 1996.
 * Copyright (C) 1996-2005 Paul Mackerras.
 *
 *  Adapted for 64bit PowerPC by Dave Engebretsen and Peter Bergner.
 *    {engebret|bergner}@us.ibm.com
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/kernel.h, linux/string.h, linux/memblock.h, linux/of_fdt.h

use core::ffi::{c_char, c_int, c_ulong, c_void};

extern "C" {
    static mut boot_command_line: *mut c_char;
    static cmd_line: *const c_char;

    fn pr_debug(format: *const c_char, ...);
    fn early_init_dt_scan(params: *mut c_void, phys: usize);
    fn strlen(s: *const c_char) -> usize;
    fn strscpy(dst: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn memblock_allow_resize();
    fn memblock_phys_mem_size() -> c_ulong;

    // __pa is a kernel macro; this declaration represents its external
    // address translation dependency at the Rust source level.
    fn __pa(addr: *const c_void) -> usize;
}

const COMMAND_LINE_SIZE: usize = 2048;

pub unsafe fn early_init_devtree(params: *mut c_void) {
    pr_debug(b" -> early_init_devtree(%p)\0".as_ptr() as *const c_char, params);

    early_init_dt_scan(params, __pa(params));
    if strlen(boot_command_line) == 0 {
        strscpy(boot_command_line, cmd_line, COMMAND_LINE_SIZE);
    }

    memblock_allow_resize();

    pr_debug(
        b"Phys. mem: %lx\n\0".as_ptr() as *const c_char,
        memblock_phys_mem_size(),
    );

    pr_debug(b" <- early_init_devtree()\n\0".as_ptr() as *const c_char);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
