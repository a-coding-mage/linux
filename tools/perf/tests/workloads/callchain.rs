// SPDX-License-Identifier: GPL-2.0
// Translated from perf/tests/workloads/callchain.c.
// C dependencies: <linux/compiler.h>, <sys/syscall.h>, <unistd.h>, "../tests.h".

use core::ffi::{c_char, c_int, c_long};

unsafe extern "C" {
    fn syscall(number: c_long, ...) -> c_long;
}

// Provided by the platform syscall definitions included by the original C file.
const SYS_gettid: c_long = libc::SYS_gettid as c_long;

/*
 * Mark as noinline to establish the call chain, and avoid the static
 * annotation to prevent LTO from renaming the functions.
 */
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn callchain_do_syscall() {
    unsafe {
        syscall(SYS_gettid);
    }
}

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn callchain_foo() {
    unsafe {
        callchain_do_syscall();
    }
}

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn callchain(
    _argc: c_int,
    _argv: *const *const c_char,
) -> c_int {
    unsafe {
        callchain_foo();
    }

    0
}

DEFINE_WORKLOAD!(callchain);
