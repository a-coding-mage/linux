/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (c) 2014 Finn Thain
 */

// Dependencies supplied by the kernel and architecture-specific sources.
// The initcall and early-param registration macros are represented by their
// corresponding external integration points in the surrounding translation.

use core::ffi::{c_char, c_int};

extern "C" {
    static mut early_console: *mut console;

    fn debug_cons_nputs(c: *mut console, s: *const c_char, n: u32);
    fn mvme147_scc_write(c: *mut console, s: *const c_char, n: u32);
    fn mvme16x_cons_write(c: *mut console, s: *const c_char, n: u32);

    fn register_console(c: *mut console);
    fn unregister_console(c: *mut console) -> c_int;
}

#[repr(C)]
pub struct console {
    pub name: *const c_char,
    pub write: Option<unsafe extern "C" fn(*mut console, *const c_char, u32)>,
    pub flags: u32,
    pub index: c_int,
}

// CON_PRINTBUFFER | CON_BOOT; values are supplied by the kernel headers.
const CONSOLE_FLAGS: u32 = CON_PRINTBUFFER | CON_BOOT;
extern "C" {
    static CON_PRINTBUFFER: u32;
    static CON_BOOT: u32;
}

// MACH_IS_MVME147 and MACH_IS_MVME16x are architecture configuration tests.
extern "C" {
    static MACH_IS_MVME147: bool;
    static MACH_IS_MVME16x: bool;
}

static mut early_console_instance: console = console {
    name: b"debug\0".as_ptr() as *const c_char,
    write: None,
    flags: CONSOLE_FLAGS,
    index: -1,
};

unsafe extern "C" fn setup_early_printk(buf: *mut c_char) -> c_int {
    if !early_console.is_null() || !buf.is_null() {
        return 0;
    }

    if MACH_IS_MVME147 {
        early_console_instance.write = Some(mvme147_scc_write);
    } else if MACH_IS_MVME16x {
        early_console_instance.write = Some(mvme16x_cons_write);
    } else {
        early_console_instance.write = Some(debug_cons_nputs);
    }
    early_console = &mut early_console_instance;
    register_console(early_console);

    0
}

unsafe extern "C" fn unregister_early_console() -> c_int {
    /*
     * debug_cons_nputs() defined in arch/m68k/kernel/head.S cannot be
     * called after init sections are discarded (for platforms that use it).
     */
    if !early_console.is_null()
        && early_console_instance.write.map(|f| f as usize)
            == Some(debug_cons_nputs as usize)
    {
        return unregister_console(early_console);
    }

    0
}

// early_param("earlyprintk", setup_early_printk);
// late_initcall(unregister_early_console);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
