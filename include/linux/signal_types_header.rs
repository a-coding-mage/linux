/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Basic signal handling related data type definitions.
 *
 * The C header's included Linux type definitions and UAPI signal definitions
 * are supplied by other translated files.
 */

#[repr(C)]
pub struct kernel_siginfo {
    /* __SIGINFO; */
}

pub type kernel_siginfo_t = kernel_siginfo;

pub struct ucounts;

/*
 * Real Time signals may be queued.
 */

#[repr(C)]
pub struct sigqueue {
    pub list: list_head,
    pub flags: ::core::ffi::c_int,
    pub info: kernel_siginfo_t,
    pub ucounts: *mut ucounts,
}

/* flags values. */
pub const SIGQUEUE_PREALLOC: ::core::ffi::c_int = 1;

#[repr(C)]
pub struct sigpending {
    pub list: list_head,
    pub signal: sigset_t,
}

#[repr(C)]
pub struct sigaction {
    /* Preserve __ARCH_HAS_IRIX_SIGACTION field ordering. */
    #[cfg(not(feature = "__ARCH_HAS_IRIX_SIGACTION"))]
    pub sa_handler: __sighandler_t,
    #[cfg(not(feature = "__ARCH_HAS_IRIX_SIGACTION"))]
    pub sa_flags: ::core::ffi::c_ulong,
    #[cfg(feature = "__ARCH_HAS_IRIX_SIGACTION")]
    pub sa_flags: ::core::ffi::c_uint,
    #[cfg(feature = "__ARCH_HAS_IRIX_SIGACTION")]
    pub sa_handler: __sighandler_t,
    #[cfg(feature = "__ARCH_HAS_SA_RESTORER")]
    pub sa_restorer: __sigrestore_t,
    pub sa_mask: sigset_t, /* mask last for extensibility */
}

#[repr(C)]
pub struct k_sigaction {
    pub sa: sigaction,
    #[cfg(feature = "__ARCH_HAS_KA_RESTORER")]
    pub ka_restorer: __sigrestore_t,
}

#[cfg(feature = "CONFIG_OLD_SIGACTION")]
#[repr(C)]
pub struct old_sigaction {
    pub sa_handler: __sighandler_t,
    pub sa_mask: old_sigset_t,
    pub sa_flags: ::core::ffi::c_ulong,
    pub sa_restorer: __sigrestore_t,
}

#[repr(C)]
pub struct ksignal {
    pub ka: k_sigaction,
    pub info: kernel_siginfo_t,
    pub sig: ::core::ffi::c_int,
}

/* Used to kill the race between sigaction and forced signals. */
pub const SA_IMMUTABLE: ::core::ffi::c_ulong = 0x00800000;

/* __ARCH_UAPI_SA_FLAGS is supplied by the architecture/UAPI configuration. */
#[cfg(feature = "__ARCH_UAPI_SA_FLAGS")]
pub const __ARCH_UAPI_SA_FLAGS: _ = __ARCH_UAPI_SA_FLAGS;
#[cfg(not(feature = "__ARCH_UAPI_SA_FLAGS"))]
pub const __ARCH_UAPI_SA_FLAGS: ::core::ffi::c_ulong = 0;

pub const UAPI_SA_FLAGS: _ =
    SA_NOCLDSTOP | SA_NOCLDWAIT | SA_SIGINFO | SA_ONSTACK | SA_RESTART |
    SA_NODEFER | SA_RESETHAND | SA_EXPOSE_TAGBITS | __ARCH_UAPI_SA_FLAGS;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
