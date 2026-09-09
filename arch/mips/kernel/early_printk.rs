/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2002, 2003, 06, 07 Ralf Baechle (ralf@linux-mips.org)
 * Copyright (C) 2007 MIPS Technologies, Inc.
 *   written by Ralf Baechle (ralf@linux-mips.org)
 */

// Dependencies supplied by the surrounding kernel translation.
use crate::linux::console::{console, CON_BOOT, CON_PRINTBUFFER};
use crate::linux::init::__init;
use crate::linux::printk::{early_console, register_console};
use crate::asm::setup::prom_putchar;

unsafe fn early_console_write(con: *mut console, s: *const core::ffi::c_char, mut n: u32) {
    while n != 0 {
        n = n.wrapping_sub(1);
        if *s == 0 {
            break;
        }
        if *s == b'\n' as core::ffi::c_char {
            prom_putchar(b'\r' as core::ffi::c_char);
        }
        prom_putchar(*s);
        s = s.add(1);
    }
}

static mut early_console_prom: console = console {
    name: b"early\0".as_ptr() as *const core::ffi::c_char,
    write: Some(early_console_write),
    flags: CON_PRINTBUFFER | CON_BOOT,
    index: -1,
};

#[__init]
pub unsafe fn setup_early_printk() {
    if !early_console.is_null() {
        return;
    }
    early_console = &raw mut early_console_prom;

    register_console(&raw mut early_console_prom);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
