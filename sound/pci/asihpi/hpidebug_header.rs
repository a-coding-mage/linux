/* SPDX-License-Identifier: GPL-2.0-only */
/*****************************************************************************

    AudioScience HPI driver
    Copyright (C) 1997-2011  AudioScience Inc. <support@audioscience.com>


Debug macros.

*****************************************************************************/

// C header dependency: "hpi_internal.h"

/* Define debugging levels.  */
pub const HPI_DEBUG_LEVEL_ERROR: ::std::os::raw::c_int = 0; /* always log errors */
pub const HPI_DEBUG_LEVEL_WARNING: ::std::os::raw::c_int = 1;
pub const HPI_DEBUG_LEVEL_NOTICE: ::std::os::raw::c_int = 2;
pub const HPI_DEBUG_LEVEL_INFO: ::std::os::raw::c_int = 3;
pub const HPI_DEBUG_LEVEL_DEBUG: ::std::os::raw::c_int = 4;
pub const HPI_DEBUG_LEVEL_VERBOSE: ::std::os::raw::c_int = 5; /* same printk level as DEBUG */

pub const HPI_DEBUG_LEVEL_DEFAULT: ::std::os::raw::c_int = HPI_DEBUG_LEVEL_NOTICE;

/*
 * an OS can define an extra flag string that is appended to
 * the start of each message, eg see linux kernel hpios.h
 */

/*
 * C conditional macro behavior:
 * #ifdef SOURCEFILE_NAME
 * #undef FILE_LINE
 * #define FILE_LINE  SOURCEFILE_NAME ":" __stringify(__LINE__) " "
 * #endif
 */

extern "C" {
    pub static mut hpi_debug_level: ::std::os::raw::c_int;

    pub fn hpi_debug_init();
    pub fn hpi_debug_level_set(level: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn hpi_debug_level_get() -> ::std::os::raw::c_int;

    pub fn hpi_debug_message(
        phm: *mut hpi_message,
        sz_fileline: *mut ::std::os::raw::c_char,
    );

    pub fn hpi_debug_data(pdata: *mut u16, len: u32);
}

#[macro_export]
macro_rules! HPI_DEBUG_ASSERT {
    ($expression:expr) => {{
        if !($expression) {
            unsafe {
                printk(
                    concat!(KERN_ERR, FILE_LINE, " ASSERT ", stringify!($expression))
                        .as_ptr() as *const ::std::os::raw::c_char,
                );
            }
        }
    }};
}

#[macro_export]
macro_rules! HPI_DEBUG_LOG {
    (ERROR, $($arg:tt)*) => {{
        if unsafe { hpi_debug_level } >= HPI_DEBUG_LEVEL_ERROR {
            unsafe { printk(concat!(HPI_DEBUG_FLAG_ERROR, FILE_LINE, " ", $($arg)*).as_ptr() as *const ::std::os::raw::c_char); }
        }
    }};
    (WARNING, $($arg:tt)*) => {{
        if unsafe { hpi_debug_level } >= HPI_DEBUG_LEVEL_WARNING {
            unsafe { printk(concat!(HPI_DEBUG_FLAG_WARNING, FILE_LINE, " ", $($arg)*).as_ptr() as *const ::std::os::raw::c_char); }
        }
    }};
    (NOTICE, $($arg:tt)*) => {{
        if unsafe { hpi_debug_level } >= HPI_DEBUG_LEVEL_NOTICE {
            unsafe { printk(concat!(HPI_DEBUG_FLAG_NOTICE, FILE_LINE, " ", $($arg)*).as_ptr() as *const ::std::os::raw::c_char); }
        }
    }};
    (INFO, $($arg:tt)*) => {{
        if unsafe { hpi_debug_level } >= HPI_DEBUG_LEVEL_INFO {
            unsafe { printk(concat!(HPI_DEBUG_FLAG_INFO, FILE_LINE, " ", $($arg)*).as_ptr() as *const ::std::os::raw::c_char); }
        }
    }};
    (DEBUG, $($arg:tt)*) => {{
        if unsafe { hpi_debug_level } >= HPI_DEBUG_LEVEL_DEBUG {
            unsafe { printk(concat!(HPI_DEBUG_FLAG_DEBUG, FILE_LINE, " ", $($arg)*).as_ptr() as *const ::std::os::raw::c_char); }
        }
    }};
    (VERBOSE, $($arg:tt)*) => {{
        if unsafe { hpi_debug_level } >= HPI_DEBUG_LEVEL_VERBOSE {
            unsafe { printk(concat!(HPI_DEBUG_FLAG_VERBOSE, FILE_LINE, " ", $($arg)*).as_ptr() as *const ::std::os::raw::c_char); }
        }
    }};
}

#[macro_export]
macro_rules! HPI_DEBUG_DATA {
    ($pdata:expr, $len:expr) => {{
        if unsafe { hpi_debug_level } >= HPI_DEBUG_LEVEL_VERBOSE {
            unsafe {
                hpi_debug_data($pdata, $len);
            }
        }
    }};
}

#[macro_export]
macro_rules! HPI_DEBUG_MESSAGE {
    (ERROR, $phm:expr) => {{
        if unsafe { hpi_debug_level } >= HPI_DEBUG_LEVEL_ERROR {
            unsafe {
                hpi_debug_message($phm, concat!(HPI_DEBUG_FLAG_ERROR, FILE_LINE, " ", stringify!(ERROR)).as_ptr() as *mut ::std::os::raw::c_char);
            }
        }
    }};
    (WARNING, $phm:expr) => {{
        if unsafe { hpi_debug_level } >= HPI_DEBUG_LEVEL_WARNING {
            unsafe {
                hpi_debug_message($phm, concat!(HPI_DEBUG_FLAG_WARNING, FILE_LINE, " ", stringify!(WARNING)).as_ptr() as *mut ::std::os::raw::c_char);
            }
        }
    }};
    (NOTICE, $phm:expr) => {{
        if unsafe { hpi_debug_level } >= HPI_DEBUG_LEVEL_NOTICE {
            unsafe {
                hpi_debug_message($phm, concat!(HPI_DEBUG_FLAG_NOTICE, FILE_LINE, " ", stringify!(NOTICE)).as_ptr() as *mut ::std::os::raw::c_char);
            }
        }
    }};
    (INFO, $phm:expr) => {{
        if unsafe { hpi_debug_level } >= HPI_DEBUG_LEVEL_INFO {
            unsafe {
                hpi_debug_message($phm, concat!(HPI_DEBUG_FLAG_INFO, FILE_LINE, " ", stringify!(INFO)).as_ptr() as *mut ::std::os::raw::c_char);
            }
        }
    }};
    (DEBUG, $phm:expr) => {{
        if unsafe { hpi_debug_level } >= HPI_DEBUG_LEVEL_DEBUG {
            unsafe {
                hpi_debug_message($phm, concat!(HPI_DEBUG_FLAG_DEBUG, FILE_LINE, " ", stringify!(DEBUG)).as_ptr() as *mut ::std::os::raw::c_char);
            }
        }
    }};
    (VERBOSE, $phm:expr) => {{
        if unsafe { hpi_debug_level } >= HPI_DEBUG_LEVEL_VERBOSE {
            unsafe {
                hpi_debug_message($phm, concat!(HPI_DEBUG_FLAG_VERBOSE, FILE_LINE, " ", stringify!(VERBOSE)).as_ptr() as *mut ::std::os::raw::c_char);
            }
        }
    }};
}

#[macro_export]
macro_rules! HPI_DEBUG_RESPONSE {
    ($phr:expr) => {{
        if ((unsafe { hpi_debug_level } >= HPI_DEBUG_LEVEL_DEBUG) && unsafe { (*$phr).error != 0 })
            || (unsafe { hpi_debug_level } >= HPI_DEBUG_LEVEL_VERBOSE)
        {
            unsafe {
                printk(
                    concat!(KERN_DEBUG, "HPI_RES%d,%d,%d\n").as_ptr()
                        as *const ::std::os::raw::c_char,
                    (*$phr).version,
                    (*$phr).error,
                    (*$phr).specific_error,
                );
            }
        }
    }};
}

#[macro_export]
macro_rules! compile_time_assert {
    ($cond:expr, $msg:ident) => {
        const _: [(); 1] = [(); ($cond) as usize];
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
