// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * DECstation PROM-based early console support.
 *
 * Copyright (C) 2004, 2007, 2026  Maciej W. Rozycki
 */

// Linux kernel dependencies supplied by other translation units.
use crate::asm::dec::prom::prom_printf;
use crate::linux::console::{register_console, console, CON_BOOT, CON_PRINTBUFFER};

extern "C" {
    fn memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize)
        -> *mut core::ffi::c_void;
    fn bug_on(condition: bool);
}

unsafe extern "C" fn prom_console_write(
    _con: *mut console,
    mut s: *const core::ffi::c_char,
    mut c: u32,
) {
    static mut BUF: [u8; 81] = [0; 81];
    let mut chunk: u32 = (core::mem::size_of::<[u8; 81]>() - 1) as u32;

    bug_on((BUF.as_ptr() as isize) != (BUF.as_ptr() as isize as i32) as isize);

    while c > 0 {
        if chunk > c {
            chunk = c;
        }
        memcpy(
            BUF.as_mut_ptr() as *mut core::ffi::c_void,
            s as *const core::ffi::c_void,
            chunk as usize,
        );
        BUF[chunk as usize] = b'\0';
        prom_printf(b"%s\0".as_ptr() as *const core::ffi::c_char, BUF.as_ptr());
        s = s.add(chunk as usize);
        c -= chunk;
    }
}

static mut PROMCONS: console = console {
    name: b"prom\0".as_ptr() as *const core::ffi::c_char,
    write: Some(prom_console_write),
    flags: CON_BOOT | CON_PRINTBUFFER,
    index: -1,
};

pub unsafe extern "C" fn register_prom_console() {
    register_console(&mut PROMCONS);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
