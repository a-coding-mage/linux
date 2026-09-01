// SPDX-License-Identifier: GPL-2.0-only
/*
 * POWER Data Stream Control Register (DSCR) fork test
 *
 * This testcase modifies the DSCR using mtspr, forks and then
 * verifies that the child process has the correct changed DSCR
 * value using mfspr.
 *
 * When using the privilege state SPR, the instructions such as
 * mfspr or mtspr are privileged and the kernel emulates them
 * for us. Instructions using problem state SPR can be executed
 * directly without any emulation if the HW supports them. Else
 * they also get emulated by the kernel.
 *
 * Copyright 2012, Anton Blanchard, IBM Corporation.
 * Copyright 2015, Anshuman Khandual, IBM Corporation.
 */
/* C dependency intent: #include "dscr.h" */

use core::ffi::{c_char, c_int, c_ulong};

type pid_t = c_int;

unsafe extern "C" {
    static mut stderr: *mut FILE;

    fn have_hwcap2(feature: c_ulong) -> c_int;
    fn getpid() -> pid_t;
    fn srand(seed: c_uint);
    fn set_dscr(val: c_ulong);
    fn set_dscr_usr(val: c_ulong);
    fn get_dscr() -> c_ulong;
    fn get_dscr_usr() -> c_ulong;
    fn fork() -> pid_t;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;

    static PPC_FEATURE2_DSCR: c_ulong;
    static COUNT: c_ulong;
    static DSCR_MAX: c_ulong;
}

use core::ffi::c_uint;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

/*
 * Macros supplied by dscr.h or system headers in the C source:
 * SKIP_IF, WIFEXITED, WEXITSTATUS.
 */

unsafe fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

unsafe fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe extern "C" fn dscr_inherit() -> c_int {
    let mut i: c_ulong;
    let mut dscr: c_ulong = 0;
    let mut pid: pid_t;

    if have_hwcap2(PPC_FEATURE2_DSCR) == 0 {
        return 0;
    }

    srand(getpid() as c_uint);
    set_dscr(dscr);

    i = 0;
    while i < COUNT {
        let cur_dscr: c_ulong;
        let cur_dscr_usr: c_ulong;

        dscr += 1;
        if dscr > DSCR_MAX {
            dscr = 0;
        }

        if i % 2 == 0 {
            set_dscr_usr(dscr);
        } else {
            set_dscr(dscr);
        }

        pid = fork();
        if pid == -1 {
            perror(c"fork() failed".as_ptr());
            exit(1);
        } else if pid != 0 {
            let mut status: c_int = 0;

            if waitpid(pid, &mut status, 0) == -1 {
                perror(c"waitpid() failed".as_ptr());
                exit(1);
            }

            if !WIFEXITED(status) {
                fprintf(stderr, c"Child didn't exit cleanly\n".as_ptr());
                exit(1);
            }

            if WEXITSTATUS(status) != 0 {
                fprintf(stderr, c"Child didn't exit cleanly\n".as_ptr());
                return 1;
            }
        } else {
            cur_dscr = get_dscr();
            if cur_dscr != dscr {
                fprintf(
                    stderr,
                    c"Kernel DSCR should be %ld but is %ld\n".as_ptr(),
                    dscr,
                    cur_dscr,
                );
                exit(1);
            }

            cur_dscr_usr = get_dscr_usr();
            if cur_dscr_usr != dscr {
                fprintf(
                    stderr,
                    c"User DSCR should be %ld but is %ld\n".as_ptr(),
                    dscr,
                    cur_dscr_usr,
                );
                exit(1);
            }
            exit(0);
        }

        i += 1;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    test_harness(dscr_inherit, c"dscr_inherit_test".as_ptr())
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
