// SPDX-License-Identifier: GPL-2.0
/*
 * printf.c:  Internal prom library printf facility.
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.com)
 */

/* This routine is internal to the prom library, no one else should know
 * about or use it!  It's simple and smelly anyway....
 */

use core::ffi::{c_char, c_int, c_void};

// Supplied by the Linux kernel and OpenPROM dependencies.
unsafe extern "C" {
    pub fn vsnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, args: *mut c_void) -> c_int;
    pub fn prom_putchar(ch: c_char);
    #[cfg(feature = "CONFIG_KGDB")]
    pub fn pr_info(fmt: *const c_char, ...);
    #[cfg(feature = "CONFIG_KGDB")]
    pub fn putpacket(buf: *mut c_char, error: c_int);
}

#[cfg(feature = "CONFIG_KGDB")]
unsafe extern "C" {
    pub static mut kgdb_initialized: c_int;
}

static mut ppbuf: [c_char; 1024] = [0; 1024];

pub unsafe extern "C" fn prom_printf(fmt: *mut c_char, mut args: ...) {
    let mut ch: c_char;
    let mut bptr: *mut c_char;

    // va_start(args, fmt);

    #[cfg(feature = "CONFIG_KGDB")]
    {
        ppbuf[0] = b'O' as c_char;
        vsnprintf(
            ppbuf.as_mut_ptr().add(1),
            core::mem::size_of_val(&ppbuf) - 1,
            fmt,
            &mut args as *mut _ as *mut c_void,
        );
    }

    #[cfg(not(feature = "CONFIG_KGDB"))]
    {
        vsnprintf(
            ppbuf.as_mut_ptr(),
            core::mem::size_of_val(&ppbuf),
            fmt,
            &mut args as *mut _ as *mut c_void,
        );
    }

    bptr = ppbuf.as_mut_ptr();

    #[cfg(feature = "CONFIG_KGDB")]
    {
        if kgdb_initialized != 0 {
            pr_info(b"kgdb_initialized = %d\n\0".as_ptr() as *const c_char, kgdb_initialized);
            putpacket(bptr, 1);
        }
    }

    #[cfg(not(feature = "CONFIG_KGDB"))]
    while {
        ch = *bptr;
        bptr = bptr.add(1);
        ch != 0
    } {
        if ch == b'\n' as c_char {
            prom_putchar(b'\r' as c_char);
        }

        prom_putchar(ch);
    }

    // va_end(args);
    return;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
