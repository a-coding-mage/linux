// SPDX-License-Identifier: GPL-2.0

// C source used `_GNU_SOURCE` for inline xstate helpers from "xstate.h".

#![allow(non_upper_case_globals)]

use std::os::raw::c_int;

unsafe extern "C" {
    fn test_xstate(xfeature: c_int);

    static XFEATURE_YMM: c_int;
    static XFEATURE_OPMASK: c_int;
    static XFEATURE_ZMM_Hi256: c_int;
    static XFEATURE_Hi16_ZMM: c_int;
}

fn main() {
    unsafe {
        test_xstate(XFEATURE_YMM);
        test_xstate(XFEATURE_OPMASK);
        test_xstate(XFEATURE_ZMM_Hi256);
        test_xstate(XFEATURE_Hi16_ZMM);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
