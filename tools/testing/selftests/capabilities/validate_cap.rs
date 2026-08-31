// SPDX-License-Identifier: GPL-2.0
//
// Translated from testing/selftests/capabilities/validate_cap.c.
// C dependencies: cap-ng.h, linux/capability.h, sys/prctl.h, sys/auxv.h,
// and kselftest.h.

use std::os::raw::{c_char, c_int, c_ulong};

const CAPNG_EFFECTIVE: c_int = 0x0000_0001;
const CAPNG_PERMITTED: c_int = 0x0000_0002;
const CAPNG_INHERITABLE: c_int = 0x0000_0004;

const CAP_NET_BIND_SERVICE: c_int = 10;

const PR_CAP_AMBIENT: c_int = 47;
const PR_CAP_AMBIENT_IS_SET: c_int = 1;

#[cfg(target_env = "gnu")]
const AT_SECURE: c_ulong = 23;

unsafe extern "C" {
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;

    fn getauxval(type_: c_ulong) -> c_ulong;

    fn capng_get_caps_process() -> c_int;
    fn capng_have_capability(which: c_int, capability: c_int) -> c_int;

    fn prctl(option: c_int, ...) -> c_int;

    fn ksft_exit_fail_msg(msg: *const c_char, ...);
    fn ksft_print_msg(msg: *const c_char, ...);
}

unsafe fn bool_arg(argv: *mut *mut c_char, i: c_int) -> bool {
    if strcmp(*argv.offset(i as isize), b"0\0".as_ptr() as *const c_char) == 0 {
        false
    } else if strcmp(*argv.offset(i as isize), b"1\0".as_ptr() as *const c_char) == 0 {
        true
    } else {
        ksft_exit_fail_msg(b"wrong argv[%d]\n\0".as_ptr() as *const c_char, i);
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut atsec: *const c_char = b"\0".as_ptr() as *const c_char;
    let mut ret: c_int;

    /*
     * Be careful just in case a setgid or setcapped copy of this
     * helper gets out.
     */

    if argc != 5 {
        ksft_exit_fail_msg(b"wrong argc\n\0".as_ptr() as *const c_char);
    }

    /*
     * C condition:
     * __GLIBC__ > 2 || (__GLIBC__ == 2 && __GLIBC_MINOR__ >= 19)
     */
    #[cfg(target_env = "gnu")]
    {
        if getauxval(AT_SECURE) != 0 {
            atsec = b" (AT_SECURE is set)\0".as_ptr() as *const c_char;
        } else {
            atsec = b" (AT_SECURE is not set)\0".as_ptr() as *const c_char;
        }
    }

    ret = capng_get_caps_process();
    if ret == -1 {
        ksft_print_msg(b"capng_get_caps_process failed\n\0".as_ptr() as *const c_char);
        return 1;
    }

    if capng_have_capability(CAPNG_EFFECTIVE, CAP_NET_BIND_SERVICE) != bool_arg(argv, 1) as c_int {
        ksft_print_msg(b"Wrong effective state%s\n\0".as_ptr() as *const c_char, atsec);
        return 1;
    }

    if capng_have_capability(CAPNG_PERMITTED, CAP_NET_BIND_SERVICE) != bool_arg(argv, 2) as c_int {
        ksft_print_msg(b"Wrong permitted state%s\n\0".as_ptr() as *const c_char, atsec);
        return 1;
    }

    if capng_have_capability(CAPNG_INHERITABLE, CAP_NET_BIND_SERVICE) != bool_arg(argv, 3) as c_int {
        ksft_print_msg(
            b"Wrong inheritable state%s\n\0".as_ptr() as *const c_char,
            atsec,
        );
        return 1;
    }

    if prctl(
        PR_CAP_AMBIENT,
        PR_CAP_AMBIENT_IS_SET,
        CAP_NET_BIND_SERVICE,
        0,
        0,
        0,
    ) != bool_arg(argv, 4) as c_int
    {
        ksft_print_msg(b"Wrong ambient state%s\n\0".as_ptr() as *const c_char, atsec);
        return 1;
    }

    ksft_print_msg(
        b"%s: Capabilities after execve were correct\n\0".as_ptr() as *const c_char,
        b"validate_cap:\0".as_ptr() as *const c_char,
    );
    0
}
