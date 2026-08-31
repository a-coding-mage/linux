// SPDX-License-Identifier: LGPL-2.1

// C dependency intent:
// #define _GNU_SOURCE
// #include <assert.h>
// #include <sched.h>
// #include <signal.h>
// #include <stdio.h>
// #include <string.h>
// #include <sys/time.h>
// #include "rseq.h"

use core::ffi::{c_char, c_int};

unsafe extern "C" {
    fn __rseq_register_current_thread(do_rseq: bool, do_mm_cid: bool) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    if unsafe { __rseq_register_current_thread(true, false) } != 0 {
        return -1;
    }
    0
}
