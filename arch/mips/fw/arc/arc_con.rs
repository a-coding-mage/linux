// SPDX-License-Identifier: GPL-2.0
/*
 * Wrap-around code for a console using the
 * ARC io-routines.
 *
 * Copyright (c) 1998 Harald Koerfgen
 * Copyright (c) 2001 Ralf Baechle
 * Copyright (c) 2002 Thiemo Seufer
 */

// C dependencies supplied by the surrounding kernel sources:
// linux/tty.h, linux/major.h, linux/init.h, linux/console.h, linux/fs.h,
// asm/setup.h, and asm/sgialib.h.

use core::ffi::{c_char, c_int, c_uint};

#[repr(C)]
pub struct console {
    pub name: *const c_char,
    pub write: Option<unsafe extern "C" fn(*mut console, *const c_char, c_uint)>,
    pub setup: Option<unsafe extern "C" fn(*mut console, *mut c_char) -> c_int>,
    pub flags: c_uint,
    pub index: c_int,
}

unsafe extern "C" {
    static mut prom_flags: c_uint;
    fn prom_putchar(c: c_char);
    fn register_console(co: *mut console);
}

// PROM_FLAG_USE_AS_CONSOLE and CON_PRINTBUFFER are supplied by the kernel headers.
const PROM_FLAG_USE_AS_CONSOLE: c_uint = 1 << 0;
const CON_PRINTBUFFER: c_uint = 1 << 0;
const ENODEV: c_int = 19;

unsafe extern "C" fn prom_console_write(
    _co: *mut console,
    mut s: *const c_char,
    mut count: c_uint,
) {
    /* Do each character */
    while count != 0 {
        if *s == b'\n' as c_char {
            prom_putchar(b'\r' as c_char);
        }
        prom_putchar(*s);
        s = s.add(1);
        count -= 1;
    }
}

unsafe extern "C" fn prom_console_setup(
    _co: *mut console,
    _options: *mut c_char,
) -> c_int {
    if prom_flags & PROM_FLAG_USE_AS_CONSOLE != 0 {
        return 0;
    }
    -ENODEV
}

static mut arc_cons: console = console {
    name: b"arc\0".as_ptr() as *const c_char,
    write: Some(prom_console_write),
    setup: Some(prom_console_setup),
    flags: CON_PRINTBUFFER,
    index: -1,
};

/*
 *    Register console.
 */

unsafe extern "C" fn arc_console_init() -> c_int {
    register_console(&raw mut arc_cons);

    0
}

// console_initcall(arc_console_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
