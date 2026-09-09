// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * arch/powerpc/boot/gamecube.c
 *
 * Nintendo GameCube bootwrapper support
 * Copyright (C) 2004-2009 The GameCube Linux Team
 * Copyright (C) 2008,2009 Albert Herranz
 */

// Dependencies supplied by the surrounding bootwrapper sources:
// stddef.h, stdio.h, types.h, io.h, ops.h, and ugecon.h.

use core::ffi::c_void;

// BSS_STACK(8192);

unsafe extern "C" {
    static mut _end: u8;
    static _dtb_start: u8;

    fn simple_alloc_init(base: *mut c_void, size: u32, align: u32, min_alloc: u32);
    fn fdt_init(dtb: *const c_void);
    fn ug_probe() -> u32;
    fn ug_console_write(s: *const u8, n: usize);
}

#[repr(C)]
struct ConsoleOps {
    write: Option<unsafe extern "C" fn(*const u8, usize)>,
}

unsafe extern "C" {
    static mut console_ops: ConsoleOps;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn platform_init(
    _r3: usize,
    _r4: usize,
    _r5: usize,
) {
    let heapsize: u32 = 16 * 1024 * 1024 - (&raw mut _end as usize as u32);

    simple_alloc_init(&raw mut _end as *mut u8 as *mut c_void, heapsize, 32, 64);
    fdt_init(&raw const _dtb_start as *const u8 as *const c_void);

    if ug_probe() != 0 {
        (*(&raw mut console_ops)).write = Some(ug_console_write);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
