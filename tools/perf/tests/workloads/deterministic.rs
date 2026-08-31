// SPDX-License-Identifier: GPL-2.0
// C dependencies: <linux/compiler.h>, "../tests.h"

#[no_mangle]
pub static mut dt_work: ::std::os::raw::c_int = 1234;

unsafe fn function1() {
    dt_work += 7;
    dt_work += 7;
    dt_work += 7;
}

unsafe fn function2() {
    dt_work += 7;
    dt_work += 7;
    dt_work += 7;
}

unsafe fn deterministic(
    _argc: ::std::os::raw::c_int,
    _argv: *const *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    dt_work += 7;
    dt_work += 7;
    dt_work += 7;

    function1();

    dt_work += 7;
    dt_work += 7;
    dt_work += 7;

    function2();

    0
}

// Original C registers this implementation with DEFINE_WORKLOAD(deterministic).
