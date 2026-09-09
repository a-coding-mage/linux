/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) International Business Machines Corp., 2000-2002
 *   Portions Copyright (C) Christoph Hellwig, 2001-2002
 */

/*
 * jfs_debug.h
 *
 * Global debug message, data structure/macro definitions under control of
 * CONFIG_JFS_DEBUG and CONFIG_JFS_STATISTICS.
 *
 * The C preprocessor configuration symbols are represented here as Cargo
 * feature names where applicable.
 */

#[cfg(all(
    feature = "CONFIG_PROC_FS",
    any(feature = "CONFIG_JFS_DEBUG", feature = "CONFIG_JFS_STATISTICS")
))]
pub const PROC_FS_JFS: () = ();

#[cfg(all(
    feature = "CONFIG_PROC_FS",
    any(feature = "CONFIG_JFS_DEBUG", feature = "CONFIG_JFS_STATISTICS")
))]
unsafe extern "C" {
    pub fn jfs_proc_init();
    pub fn jfs_proc_clean();
}

/* assert with traditional printf/panic */
#[macro_export]
macro_rules! assert {
    ($p:expr) => {{
        if !($p) {
            unsafe {
                printk(
                    KERN_CRIT,
                    concat!("BUG at ", file!(), ":", line!(), " assert(", stringify!($p), ")\n"),
                );
                BUG();
            }
        }
    }};
}

#[cfg(feature = "CONFIG_JFS_DEBUG")]
#[macro_export]
macro_rules! ASSERT {
    ($p:expr) => { $crate::assert!($p) };
}

#[cfg(not(feature = "CONFIG_JFS_DEBUG"))]
#[macro_export]
macro_rules! ASSERT {
    ($p:expr) => {{ }};
}

#[cfg(feature = "CONFIG_JFS_DEBUG")]
pub const JFS_LOGLEVEL_ERR: i32 = 1;
#[cfg(feature = "CONFIG_JFS_DEBUG")]
pub const JFS_LOGLEVEL_WARN: i32 = 2;
#[cfg(feature = "CONFIG_JFS_DEBUG")]
pub const JFS_LOGLEVEL_DEBUG: i32 = 3;
#[cfg(feature = "CONFIG_JFS_DEBUG")]
pub const JFS_LOGLEVEL_INFO: i32 = 4;

#[cfg(feature = "CONFIG_JFS_DEBUG")]
unsafe extern "C" {
    pub static mut jfsloglevel: i32;
    pub fn jfs_txanchor_proc_show(m: *mut seq_file, v: *mut core::ffi::c_void) -> i32;
}

#[cfg(feature = "CONFIG_JFS_DEBUG")]
#[macro_export]
macro_rules! jfs_info {
    ($fmt:expr $(, $arg:expr)*) => {{
        if unsafe { $crate::jfsloglevel } >= $crate::JFS_LOGLEVEL_INFO {
            unsafe { printk(KERN_INFO, concat!($fmt, "\n") $(, $arg)*) };
        }
    }};
}

#[cfg(feature = "CONFIG_JFS_DEBUG")]
#[macro_export]
macro_rules! jfs_debug {
    ($fmt:expr $(, $arg:expr)*) => {{
        if unsafe { $crate::jfsloglevel } >= $crate::JFS_LOGLEVEL_DEBUG {
            unsafe { printk(KERN_DEBUG, concat!($fmt, "\n") $(, $arg)*) };
        }
    }};
}

#[cfg(feature = "CONFIG_JFS_DEBUG")]
#[macro_export]
macro_rules! jfs_warn {
    ($fmt:expr $(, $arg:expr)*) => {{
        if unsafe { $crate::jfsloglevel } >= $crate::JFS_LOGLEVEL_WARN {
            unsafe { printk(KERN_WARNING, concat!($fmt, "\n") $(, $arg)*) };
        }
    }};
}

#[cfg(feature = "CONFIG_JFS_DEBUG")]
#[macro_export]
macro_rules! jfs_err {
    ($fmt:expr $(, $arg:expr)*) => {{
        if unsafe { $crate::jfsloglevel } >= $crate::JFS_LOGLEVEL_ERR {
            unsafe { printk(KERN_ERR, concat!($fmt, "\n") $(, $arg)*) };
        }
    }};
}

#[cfg(not(feature = "CONFIG_JFS_DEBUG"))]
#[macro_export]
macro_rules! jfs_info { ($fmt:expr $(, $arg:expr)*) => {{ }}; }
#[cfg(not(feature = "CONFIG_JFS_DEBUG"))]
#[macro_export]
macro_rules! jfs_debug { ($fmt:expr $(, $arg:expr)*) => {{ }}; }
#[cfg(not(feature = "CONFIG_JFS_DEBUG"))]
#[macro_export]
macro_rules! jfs_warn { ($fmt:expr $(, $arg:expr)*) => {{ }}; }
#[cfg(not(feature = "CONFIG_JFS_DEBUG"))]
#[macro_export]
macro_rules! jfs_err { ($fmt:expr $(, $arg:expr)*) => {{ }}; }

#[cfg(feature = "CONFIG_JFS_STATISTICS")]
unsafe extern "C" {
    pub fn jfs_lmstats_proc_show(m: *mut seq_file, v: *mut core::ffi::c_void) -> i32;
    pub fn jfs_txstats_proc_show(m: *mut seq_file, v: *mut core::ffi::c_void) -> i32;
    pub fn jfs_mpstat_proc_show(m: *mut seq_file, v: *mut core::ffi::c_void) -> i32;
    pub fn jfs_xtstat_proc_show(m: *mut seq_file, v: *mut core::ffi::c_void) -> i32;
}

#[cfg(feature = "CONFIG_JFS_STATISTICS")]
#[macro_export]
macro_rules! INCREMENT { ($x:expr) => { $x += 1 }; }
#[cfg(feature = "CONFIG_JFS_STATISTICS")]
#[macro_export]
macro_rules! DECREMENT { ($x:expr) => { $x -= 1 }; }
#[cfg(feature = "CONFIG_JFS_STATISTICS")]
#[macro_export]
macro_rules! HIGHWATERMARK {
    ($x:expr, $y:expr) => {{ if $x < $y { $x = $y; } }};
}

#[cfg(not(feature = "CONFIG_JFS_STATISTICS"))]
#[macro_export]
macro_rules! INCREMENT { ($x:expr) => {{ }}; }
#[cfg(not(feature = "CONFIG_JFS_STATISTICS"))]
#[macro_export]
macro_rules! DECREMENT { ($x:expr) => {{ }}; }
#[cfg(not(feature = "CONFIG_JFS_STATISTICS"))]
#[macro_export]
macro_rules! HIGHWATERMARK { ($x:expr, $y:expr) => {{ }}; }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
