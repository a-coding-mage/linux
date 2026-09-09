// SPDX-License-Identifier: GPL-2.0-only
/*
 * Motorola/Emerson MVME5100 with PPCBug firmware.
 *
 * Author: Stephen Chivers <schivers@csc.com>
 *
 * Copyright 2013 CSC Australia Pty. Ltd.
 */

// Translated from the C dependencies: types.h, ops.h, and io.h.

// BSS_STACK(4096);
// The stack-storage macro is supplied by the surrounding platform support.

unsafe extern "C" {
    static mut _end: u8;
    static mut _dtb_start: u8;

    fn simple_alloc_init(start: *mut u8, size: u32, align: u32, boundary: u32);
    fn fdt_init(dtb_start: *mut u8);
    fn serial_console_init();
}

pub unsafe fn platform_init(_r3: usize, _r4: usize, _r5: usize) {
    let heapsize: u32;

    heapsize = 0x8000000u32.wrapping_sub((&raw mut _end as *mut u8) as u32); /* 128M */
    simple_alloc_init(&raw mut _end, heapsize, 32, 64);
    fdt_init(&raw mut _dtb_start);
    serial_console_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
