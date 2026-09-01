#![allow(non_camel_case_types)]

use std::ffi::c_int;

type pid_t = c_int;
type uid_t = u32;

const CLONE_NEWUSER: c_int = 0x10000000;
const CLONE_NEWPID: c_int = 0x20000000;
const EXIT_SUCCESS: c_int = 0;
const ENOMEM: c_int = 12;

unsafe extern "C" {
    fn geteuid() -> uid_t;
    fn unshare(flags: c_int) -> c_int;
    fn fork() -> pid_t;
    fn exit(status: c_int) -> !;
    fn __errno_location() -> *mut c_int;

    /* From "../pidfd/pidfd.h". */
    fn wait_for_pid(pid: pid_t) -> c_int;
}

unsafe fn errno() -> c_int {
    unsafe { *__errno_location() }
}

/*
 * Regression test for:
 * 35f71bc0a09a ("fork: report pid reservation failure properly")
 * b26ebfe12f34 ("pid: Fix error return value in some cases")
 */
unsafe fn regression_enomem() {
    let mut pid: pid_t;

    if unsafe { geteuid() } != 0 {
        assert_eq!(0, unsafe { unshare(CLONE_NEWUSER) });
    }

    assert_eq!(0, unsafe { unshare(CLONE_NEWPID) });

    pid = unsafe { fork() };
    assert!(pid >= 0);

    if pid == 0 {
        unsafe { exit(EXIT_SUCCESS) };
    }

    assert_eq!(0, unsafe { wait_for_pid(pid) });

    pid = unsafe { fork() };
    assert!(pid < 0);
    assert_eq!(unsafe { errno() }, ENOMEM);
}

fn main() {
    unsafe { regression_enomem() };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
