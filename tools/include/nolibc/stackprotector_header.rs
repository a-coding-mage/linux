/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * Stack protector support for NOLIBC
 * Copyright (C) 2023 Thomas Weissschuh <linux@weissschuh.net>
 */

/* Original C dependencies:
 * #include "compiler.h"
 * When runtime support is enabled and _NOLIBC_STACKPROTECTOR is defined:
 * #include "sys.h"
 * #include "stdlib.h"
 */

#[cfg(all(not(NOLIBC_NO_RUNTIME), _NOLIBC_STACKPROTECTOR))]
extern "C" {
    fn __nolibc_syscall0(n: core::ffi::c_long) -> core::ffi::c_long;
    fn __nolibc_syscall2(
        n: core::ffi::c_long,
        a1: core::ffi::c_long,
        a2: core::ffi::c_long,
    ) -> core::ffi::c_long;
    fn __nolibc_syscall3(
        n: core::ffi::c_long,
        a1: core::ffi::c_long,
        a2: core::ffi::c_long,
        a3: core::ffi::c_long,
    ) -> core::ffi::c_long;

    static __NR_write: core::ffi::c_long;
    static __NR_getpid: core::ffi::c_long;
    static __NR_kill: core::ffi::c_long;
    static __NR_getrandom: core::ffi::c_long;
    static STDERR_FILENO: core::ffi::c_int;
    static SIGABRT: core::ffi::c_int;
    static GRND_INSECURE: core::ffi::c_uint;
    static GRND_NONBLOCK: core::ffi::c_uint;
}

/* The functions in this header are using raw syscall macros to avoid
 * triggering stack protector errors themselves
 */

#[cfg(all(not(NOLIBC_NO_RUNTIME), _NOLIBC_STACKPROTECTOR))]
#[link_section = ".data.nolibc_stack_chk"]
#[used]
#[no_mangle]
pub static mut __stack_chk_guard: usize = 0;

#[cfg(all(not(NOLIBC_NO_RUNTIME), _NOLIBC_STACKPROTECTOR))]
#[link_section = ".text.nolibc_stack_chk"]
#[used]
#[no_mangle]
pub unsafe extern "C" fn __stack_chk_fail() -> ! {
    let pid: core::ffi::c_long;

    unsafe {
        __nolibc_syscall3(
            __NR_write,
            STDERR_FILENO as core::ffi::c_long,
            b"!!Stack smashing detected!!\n".as_ptr() as core::ffi::c_long,
            28,
        );
        pid = __nolibc_syscall0(__NR_getpid);
        __nolibc_syscall2(__NR_kill, pid, SIGABRT as core::ffi::c_long);
    }

    loop {}
}

#[cfg(all(not(NOLIBC_NO_RUNTIME), _NOLIBC_STACKPROTECTOR))]
#[link_section = ".text.nolibc_stack_chk"]
#[no_mangle]
pub unsafe extern "C" fn __stack_chk_fail_local() -> ! {
    unsafe {
        __stack_chk_fail();
    }
}

#[cfg(all(not(NOLIBC_NO_RUNTIME), _NOLIBC_STACKPROTECTOR))]
pub unsafe fn __stack_chk_init() {
    unsafe {
        __nolibc_syscall3(
            __NR_getrandom,
            core::ptr::addr_of_mut!(__stack_chk_guard) as core::ffi::c_long,
            core::mem::size_of_val(&__stack_chk_guard) as core::ffi::c_long,
            (GRND_INSECURE | GRND_NONBLOCK) as core::ffi::c_long,
        );
    }

    /* a bit more randomness in case getrandom() fails, ensure the guard is never 0 */
    unsafe {
        let guard_addr = core::ptr::addr_of!(__stack_chk_guard) as usize;
        if __stack_chk_guard != guard_addr {
            __stack_chk_guard ^= guard_addr;
        }
    }
}

#[cfg(all(not(NOLIBC_NO_RUNTIME), not(_NOLIBC_STACKPROTECTOR)))]
pub fn __stack_chk_init() {}
