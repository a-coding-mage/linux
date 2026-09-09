/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux compatibility and SPARC
// signal headers are intentionally left as external Rust dependencies.

#[cfg(CONFIG_COMPAT)]
#[repr(C)]
pub struct __new_sigaction32 {
    pub sa_handler: ::core::ffi::c_uint,
    pub sa_flags: ::core::ffi::c_uint,
    pub sa_restorer: ::core::ffi::c_uint, // not used by Linux/SPARC yet
    pub sa_mask: compat_sigset_t,
}

#[cfg(CONFIG_COMPAT)]
#[repr(C)]
pub struct __old_sigaction32 {
    pub sa_handler: ::core::ffi::c_uint,
    pub sa_mask: compat_old_sigset_t,
    pub sa_flags: ::core::ffi::c_uint,
    pub sa_restorer: ::core::ffi::c_uint, // not used by Linux/SPARC yet
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
