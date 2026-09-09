// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2007 IBM Corporation
 *
 * Stephen Winiecki <stevewin@us.ibm.com>
 * Josh Boyer <jwboyer@linux.vnet.ibm.com>
 *
 * Based on earlier code:
 * Copyright (C) Paul Mackerras 1997.
 */

// C dependencies: stdarg.h, stddef.h, types.h, elf.h, string.h, stdio.h,
// page.h, ops.h, and io.h provide the surrounding boot environment.

// BSS_STACK(4096);

unsafe extern "C" {
    static mut _end: u8;
    static _dtb_start: u8;

    fn simple_alloc_init(base: *mut u8, heap_size: u32, align: u32, max_alloc: u32);
    fn fdt_init(dtb_start: *const u8);
    fn serial_console_init();
}

pub unsafe extern "C" fn platform_init(
    _r3: usize,
    _r4: usize,
    _r5: usize,
) {
    let heapsize: u32 = 0x8000000u32
        .wrapping_sub(core::ptr::addr_of_mut!(_end) as usize as u32); /* 128M */

    simple_alloc_init(core::ptr::addr_of_mut!(_end), heapsize, 32, 64);
    fdt_init(core::ptr::addr_of!(_dtb_start));
    serial_console_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
