/* SPDX-License-Identifier: GPL-2.0 */
// Translated from the Linux MIPS compat signal header.
//
// The original header includes Linux and MIPS declarations supplied by the
// surrounding kernel environment; those dependencies are intentionally left
// external here.

/// Equivalent of the C `static inline` conversion helper.
#[inline]
pub unsafe fn __copy_conv_sigset_to_user(
    d: *mut compat_sigset_t,
    s: *const sigset_t,
) -> i32 {
    // BUILD_BUG_ON(sizeof(*d) != sizeof(*s));
    const _: () = assert!(
        core::mem::size_of::<compat_sigset_t>() == core::mem::size_of::<sigset_t>()
    );

    // BUILD_BUG_ON(_NSIG_WORDS != 2);
    const _: () = assert!(_NSIG_WORDS == 2);

    put_compat_sigset(d, s, core::mem::size_of::<compat_sigset_t>())
}

/// Equivalent of the C `static inline` conversion helper.
#[inline]
pub unsafe fn __copy_conv_sigset_from_user(
    d: *mut sigset_t,
    s: *const compat_sigset_t,
) -> i32 {
    get_compat_sigset(d, s)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
