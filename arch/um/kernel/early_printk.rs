// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2011 Richard Weinberger <richrd@nod.at>
 */

// C dependencies are supplied by the surrounding kernel translation.

#[repr(C)]
pub struct console {
    pub name: *const core::ffi::c_char,
    pub write: Option<unsafe extern "C" fn(*mut console, *const core::ffi::c_char, u32)>,
    pub flags: u32,
    pub index: i32,
}

unsafe extern "C" {
    fn um_early_printk(s: *const core::ffi::c_char, n: u32);
    static mut early_console: *mut console;
    fn register_console(con: *mut console);
}

const CON_BOOT: u32 = 1 << 5;

unsafe extern "C" fn early_console_write(
    _con: *mut console,
    s: *const core::ffi::c_char,
    n: u32,
) {
    unsafe {
        um_early_printk(s, n);
    }
}

static mut early_console_dev: console = console {
    name: b"earlycon\0".as_ptr() as *const core::ffi::c_char,
    write: Some(early_console_write),
    flags: CON_BOOT,
    index: -1,
};

unsafe extern "C" fn setup_early_printk(_buf: *mut core::ffi::c_char) -> i32 {
    unsafe {
        if early_console.is_null() {
            early_console = &raw mut early_console_dev;
            register_console(&raw mut early_console_dev);
        }
    }
    0
}

// C build-time registration: early_param("earlyprintk", setup_early_printk);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
