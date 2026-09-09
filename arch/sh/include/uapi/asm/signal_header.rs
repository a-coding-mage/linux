/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Corresponds to: #define SA_RESTORER 0x04000000
pub const SA_RESTORER: u32 = 0x04000000;

// Dependency corresponding to <asm-generic/signal.h>.

// This declaration is present only when __KERNEL__ is not defined in the C
// header. The build configuration is responsible for selecting that condition.
#[cfg(not(feature = "__KERNEL__"))]
#[repr(C)]
pub struct old_sigaction {
    pub sa_handler: __sighandler_t,
    pub sa_mask: old_sigset_t,
    pub sa_flags: ::core::ffi::c_ulong,
    pub sa_restorer: Option<unsafe extern "C" fn()>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
