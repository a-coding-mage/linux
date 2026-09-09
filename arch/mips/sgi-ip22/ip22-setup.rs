// SPDX-License-Identifier: GPL-2.0
/*
 * ip22-setup.c: SGI specific setup, including init of the feature struct.
 *
 * Copyright (C) 1996 David S. Miller (davem@davemloft.net)
 * Copyright (C) 1997, 1998 Ralf Baechle (ralf@gnu.org)
 */

// Linux and MIPS declarations supplied by the corresponding dependencies.
use core::ffi::{c_char, c_void};

extern "C" {
    static mut board_be_init: unsafe extern "C" fn();
    static mut prom_flags: u32;

    fn ip22_be_init();
    fn sgihpc_init();
    fn sgimc_init();
    #[cfg(feature = "CONFIG_BOARD_SCACHE")]
    fn indy_sc_init();
    fn ioremap(offset: c_ulong, size: c_ulong) -> *mut c_void;
    fn set_io_port_base(base: c_ulong);
    fn ArcGetEnvironmentVariable(name: *const c_char) -> *mut c_char;
    fn strscpy(dest: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn add_preferred_console(name: *const c_char, index: u32, options: *const c_char);
}

type c_ulong = usize;

const PROM_FLAG_USE_AS_CONSOLE: u32 = 1 << 0;

#[no_mangle]
pub unsafe extern "C" fn plat_mem_setup() {
    let mut ctype: *mut c_char;
    let mut cserial: *mut c_char;

    board_be_init = ip22_be_init;

    /* Init the INDY HPC I/O controller.  Need to call this before
     * fucking with the memory controller because it needs to know the
     * boardID and whether this is a Guiness or a FullHouse machine.
     */
    sgihpc_init();

    /* Init INDY memory controller. */
    sgimc_init();

    // Build-time CONFIG_BOARD_SCACHE condition preserved from the source.
    #[cfg(feature = "CONFIG_BOARD_SCACHE")]
    {
        /* Now enable boardcaches, if any. */
        indy_sc_init();
    }

    /* Set EISA IO port base for Indigo2
     * ioremap cannot fail
     */
    set_io_port_base(ioremap(0x0008_0000, 0x1fff_ffff - 0x0008_0000) as c_ulong);

    /* ARCS console environment variable is set to "g?" for
     * graphics console, it is set to "d" for the first serial
     * line and "d2" for the second serial line.
     *
     * Need to check if the case is 'g' but no keyboard:
     * (ConsoleIn/Out = serial)
     */
    ctype = ArcGetEnvironmentVariable(b"console\0".as_ptr() as *const c_char);
    cserial = ArcGetEnvironmentVariable(b"ConsoleOut\0".as_ptr() as *const c_char);

    if ((!ctype.is_null() && *ctype == b'd' as c_char)
        || (!cserial.is_null() && *cserial == b's' as c_char)
    {
        static mut OPTIONS: [c_char; 8] = [0; 8];
        let baud = ArcGetEnvironmentVariable(b"dbaud\0".as_ptr() as *const c_char);
        if !baud.is_null() {
            strscpy(OPTIONS.as_mut_ptr(), baud, OPTIONS.len());
        }
        let index = if *ctype.add(1) == b'2' as c_char { 1 } else { 0 };
        add_preferred_console(
            b"ttyS\0".as_ptr() as *const c_char,
            index,
            if !baud.is_null() {
                OPTIONS.as_ptr()
            } else {
                core::ptr::null()
            },
        );
    } else if ctype.is_null() || *ctype != b'g' as c_char {
        /* Use ARC if we don't want serial ('d') or graphics ('g'). */
        prom_flags |= PROM_FLAG_USE_AS_CONSOLE;
        add_preferred_console(
            b"arc\0".as_ptr() as *const c_char,
            0,
            core::ptr::null(),
        );
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
