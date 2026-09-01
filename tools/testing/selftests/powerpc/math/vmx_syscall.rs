// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2015, Cyril Bur, IBM Corp.
 *
 * This test attempts to see if the VMX registers change across a syscall (fork).
 */

type c_int = i32;
type c_char = i8;
type pid_t = i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vector_int {
    pub lanes: [c_int; 4],
}

pub static mut varray: [vector_int; 12] = [
    vector_int { lanes: [1, 2, 3, 4] },
    vector_int { lanes: [5, 6, 7, 8] },
    vector_int { lanes: [9, 10, 11, 12] },
    vector_int { lanes: [13, 14, 15, 16] },
    vector_int { lanes: [17, 18, 19, 20] },
    vector_int { lanes: [21, 22, 23, 24] },
    vector_int { lanes: [25, 26, 27, 28] },
    vector_int { lanes: [29, 30, 31, 32] },
    vector_int { lanes: [33, 34, 35, 36] },
    vector_int { lanes: [37, 38, 39, 40] },
    vector_int { lanes: [41, 42, 43, 44] },
    vector_int { lanes: [45, 46, 47, 48] },
];

extern "C" {
    fn test_vmx(varray: *mut vector_int, pid: *mut pid_t) -> c_int;
    fn fork() -> pid_t;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn exit(status: c_int) -> !;

    fn have_hwcap2(feature: u64) -> c_int;
    fn SKIP_IF(condition: c_int);
    fn FAIL_IF(condition: c_int);
    fn test_harness(
        test_function: Option<unsafe extern "C" fn() -> c_int>,
        name: *const c_char,
    ) -> c_int;

    static PPC_FEATURE2_ARCH_2_07: u64;
}

#[no_mangle]
pub unsafe extern "C" fn vmx_syscall() -> c_int {
    let mut fork_pid: pid_t = 0;
    let mut i: c_int;
    let mut ret: c_int;
    let mut child_ret: c_int = 0;

    i = 0;
    while i < 1000 {
        /* test_vmx will fork() */
        ret = test_vmx(varray.as_mut_ptr(), &mut fork_pid);
        if fork_pid == -1 {
            return -1;
        }
        if fork_pid == 0 {
            exit(ret);
        }
        waitpid(fork_pid, &mut child_ret, 0);
        if ret != 0 || child_ret != 0 {
            return 1;
        }

        i += 1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn test_vmx_syscall() -> c_int {
    /*
     * Setup an environment with much context switching
     */
    let mut pid2: pid_t;
    let pid: pid_t;
    let mut ret: c_int;
    let mut child_ret: c_int = 0;

    // vcmpequd used in vmx_asm.S is v2.07
    SKIP_IF((have_hwcap2(PPC_FEATURE2_ARCH_2_07) == 0) as c_int);

    pid = fork();
    FAIL_IF((pid == -1) as c_int);

    pid2 = fork();
    ret = vmx_syscall();
    /* Can't FAIL_IF(pid2 == -1); because we've already forked */
    if pid2 == -1 {
        /*
         * Couldn't fork, ensure child_ret is set and is a fail
         */
        child_ret = 1;
        ret = child_ret;
    } else {
        if pid2 != 0 {
            waitpid(pid2, &mut child_ret, 0);
        } else {
            exit(ret);
        }
    }

    ret |= child_ret;

    if pid != 0 {
        waitpid(pid, &mut child_ret, 0);
    } else {
        exit(ret);
    }

    FAIL_IF((ret != 0 || child_ret != 0) as c_int);
    0
}

#[no_mangle]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    test_harness(Some(test_vmx_syscall), b"vmx_syscall\0".as_ptr() as *const c_char)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
