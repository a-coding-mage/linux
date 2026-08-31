// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2019, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>

use core::ffi::{c_char, c_int};

// C dependencies:
// #include <errno.h>
// #include "evswitch.h"
// #include "evlist.h"

pub const ENOENT: c_int = 2;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evswitch {
    pub on: *mut evsel,
    pub off: *mut evsel,
    pub on_name: *const c_char,
    pub off_name: *const c_char,
    pub discarding: bool,
    pub show_on_off_events: bool,
}

extern "C" {
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn evlist__find_evsel_by_str(evlist: *mut evlist, str_: *const c_char) -> *mut evsel;
}

#[no_mangle]
pub unsafe extern "C" fn evswitch__discard(
    evswitch: *mut evswitch,
    evsel: *mut evsel,
) -> bool {
    if (*evswitch).on != core::ptr::null_mut() && (*evswitch).discarding {
        if (*evswitch).on != evsel {
            return true;
        }

        (*evswitch).discarding = false;

        if !(*evswitch).show_on_off_events {
            return true;
        }

        return false;
    }

    if (*evswitch).off != core::ptr::null_mut() && !(*evswitch).discarding {
        if (*evswitch).off != evsel {
            return false;
        }

        (*evswitch).discarding = true;

        if !(*evswitch).show_on_off_events {
            return true;
        }
    }

    false
}

unsafe fn evswitch__fprintf_enoent(
    fp: *mut FILE,
    evtype: *const c_char,
    evname: *const c_char,
) -> c_int {
    let mut printed = fprintf(
        fp,
        b"ERROR: switch-%s event not found (%s)\n\0".as_ptr() as *const c_char,
        evtype,
        evname,
    );

    printed += fprintf(
        fp,
        b"HINT:  use 'perf evlist' to see the available event names\n\0".as_ptr() as *const c_char,
    );
    printed
}

#[no_mangle]
pub unsafe extern "C" fn evswitch__init(
    evswitch: *mut evswitch,
    evlist: *mut evlist,
    fp: *mut FILE,
) -> c_int {
    if !(*evswitch).on_name.is_null() {
        (*evswitch).on = evlist__find_evsel_by_str(evlist, (*evswitch).on_name);
        if (*evswitch).on.is_null() {
            evswitch__fprintf_enoent(
                fp,
                b"on\0".as_ptr() as *const c_char,
                (*evswitch).on_name,
            );
            return -ENOENT;
        }
        (*evswitch).discarding = true;
    }

    if !(*evswitch).off_name.is_null() {
        (*evswitch).off = evlist__find_evsel_by_str(evlist, (*evswitch).off_name);
        if (*evswitch).off.is_null() {
            evswitch__fprintf_enoent(
                fp,
                b"off\0".as_ptr() as *const c_char,
                (*evswitch).off_name,
            );
            return -ENOENT;
        }
    }

    0
}
