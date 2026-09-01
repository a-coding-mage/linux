// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2019, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>

use core::ffi::{c_char, c_int};

// Dependencies from the original C header:
// - bool from <stdbool.h>
// - FILE from <stdio.h>

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

unsafe extern "C" {
    pub fn evswitch__init(
        evswitch: *mut evswitch,
        evlist: *mut evlist,
        fp: *mut libc::FILE,
    ) -> c_int;

    pub fn evswitch__discard(evswitch: *mut evswitch, evsel: *mut evsel) -> bool;
}

macro_rules! OPTS_EVSWITCH {
    ($evswitch:expr) => {
        OPT_STRING!(
            0,
            "switch-on",
            core::ptr::addr_of_mut!((*$evswitch).on_name),
            "event",
            "Consider events after the occurrence of this event"
        ),
        OPT_STRING!(
            0,
            "switch-off",
            core::ptr::addr_of_mut!((*$evswitch).off_name),
            "event",
            "Stop considering events after the occurrence of this event"
        ),
        OPT_BOOLEAN!(
            0,
            "show-on-off-events",
            core::ptr::addr_of_mut!((*$evswitch).show_on_off_events),
            "Show the on/off switch events, used with --switch-on and --switch-off"
        )
    };
}

pub(crate) use OPTS_EVSWITCH;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
