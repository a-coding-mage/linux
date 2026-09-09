/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the Linux types translation: `atomic_t`.

// Corresponds to CONFIG_SECCOMP.
#[cfg(feature = "CONFIG_SECCOMP")]
pub struct seccomp_filter;

/**
 * struct seccomp - the state of a seccomp'ed process
 *
 * @mode:  indicates one of the valid values above for controlled
 *         system calls available to a process.
 * @filter_count: number of seccomp filters
 * @filter: must always point to a valid seccomp-filter or NULL as it is
 *          accessed without locking during system call entry.
 *
 *          @filter must only be accessed from the context of current as there
 *          is no read locking.
 */
#[cfg(feature = "CONFIG_SECCOMP")]
#[repr(C)]
pub struct seccomp {
    pub mode: ::core::ffi::c_int,
    pub filter_count: atomic_t,
    pub filter: *mut seccomp_filter,
}

// Corresponds to the CONFIG_SECCOMP-disabled branch.
#[cfg(not(feature = "CONFIG_SECCOMP"))]
#[repr(C)]
pub struct seccomp {}

#[cfg(not(feature = "CONFIG_SECCOMP"))]
#[repr(C)]
pub struct seccomp_filter {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
