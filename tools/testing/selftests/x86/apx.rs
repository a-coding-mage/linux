// SPDX-License-Identifier: GPL-2.0

// C source defined _GNU_SOURCE and included "xstate.h".
// test_xstate and XFEATURE_APX are expected to be supplied by the translated
// xstate dependency.

extern "C" {
    fn test_xstate(feature: i32);
}

extern "C" {
    static XFEATURE_APX: i32;
}

fn main() {
    unsafe {
        test_xstate(XFEATURE_APX);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
