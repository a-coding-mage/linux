/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * arch/powerpc/boot/ugecon.h
 *
 * USB Gecko early bootwrapper console.
 * Copyright (C) 2008-2009 The GameCube Linux Team
 * Copyright (C) 2008,2009 Albert Herranz
 */

use core::ffi::c_void;
use core::ffi::c_char;
use core::ffi::c_int;

extern "C" {
    pub fn ug_probe() -> *mut c_void;

    pub fn ug_putc(ch: c_char);
    pub fn ug_console_write(buf: *const c_char, len: c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
