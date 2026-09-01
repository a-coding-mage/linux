/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Support for libpfm4 event encoding.
 *
 * Copyright 2020 Google LLC.
 */

/* Dependencies from the original header:
 * - "print-events.h" provides struct print_callbacks.
 * - <subcmd/parse-options.h> provides struct option.
 */

#[repr(C)]
pub struct option {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct print_callbacks {
    _unused: [u8; 0],
}

/* Original condition: #ifdef HAVE_LIBPFM */
#[cfg(HAVE_LIBPFM)]
unsafe extern "C" {
    pub fn parse_libpfm_events_option(
        opt: *const option,
        str: *const ::std::os::raw::c_char,
        unset: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn print_libpfm_events(
        print_cb: *const print_callbacks,
        print_state: *mut ::std::os::raw::c_void,
    );
}

/* Original #else fallback when HAVE_LIBPFM is not defined. */
#[cfg(not(HAVE_LIBPFM))]
#[inline]
pub unsafe extern "C" fn parse_libpfm_events_option(
    _opt: *const option,
    _str: *const ::std::os::raw::c_char,
    _unset: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    0
}

#[cfg(not(HAVE_LIBPFM))]
#[inline]
pub unsafe extern "C" fn print_libpfm_events(
    _print_cb: *const print_callbacks,
    _print_state: *mut ::std::os::raw::c_void,
) {
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
