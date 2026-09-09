/* SPDX-License-Identifier: LGPL-2.1-or-later */
/*
 *   Copyright (c) 2010 Wang Lei
 *   Author(s): Wang Lei (wang840925@gmail.com). All Rights Reserved.
 *
 *   Internal DNS Rsolver stuff
 */

// Linux kernel dependencies supplied by other translated files.

/*
 * Layout of key payload words.
 */
#[repr(C)]
pub enum DnsKey {
    dns_key_data,
    dns_key_error,
}

/*
 * dns_key.c
 */
extern "C" {
    pub static dns_resolver_cache: *const cred;
}

/* Opaque declaration supplied by the Linux kernel dependency. */
#[repr(C)]
pub struct cred {
    _private: [u8; 0],
}

/*
 * debug tracing
 */
extern "C" {
    pub static mut dns_resolver_debug: ::core::ffi::c_uint;
}

/*
 * The following macros preserve the C debug-tracing interface.  `unlikely`,
 * `printk`, `KERN_DEBUG`, and `current` are supplied by kernel dependencies.
 */
#[macro_export]
macro_rules! kdebug {
    ($fmt:literal $(, $args:expr)*) => {{
        if unsafe { unlikely(dns_resolver_debug != 0) } {
            unsafe {
                printk!(concat!(KERN_DEBUG, "[%-6.6s] ", $fmt, "\n"),
                    current.comm $(, $args)*);
            }
        }
    }};
}

#[macro_export]
macro_rules! kenter {
    ($fmt:literal $(, $args:expr)*) => {
        kdebug!(concat!("==> %s(", $fmt, ")"), __func__ $(, $args)*);
    };
}

#[macro_export]
macro_rules! kleave {
    ($fmt:literal $(, $args:expr)*) => {
        kdebug!(concat!("<== %s()", $fmt), __func__ $(, $args)*);
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
