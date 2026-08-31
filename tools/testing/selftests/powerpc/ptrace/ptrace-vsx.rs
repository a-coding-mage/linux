// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Ptrace test for VMX/VSX registers
 *
 * Copyright (C) 2015 Anshuman Khandual, IBM Corporation.
 */

// C dependencies: "ptrace.h", "ptrace-vsx.h"

use core::arch::asm;
use core::ffi::{c_char, c_int, c_void};

type pid_t = c_int;

unsafe extern "C" {
    fn shmat(shmid: c_int, shmaddr: *const c_void, shmflg: c_int) -> *mut c_void;
    fn shmdt(shmaddr: *const c_void) -> c_int;
    fn shmget(key: c_int, size: usize, shmflg: c_int) -> c_int;
    fn shmctl(shmid: c_int, cmd: c_int, buf: *mut c_void) -> c_int;
    fn exit(status: c_int) -> !;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn rand() -> c_int;
    fn fork() -> pid_t;
    fn perror(s: *const c_char);
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn wait(wstatus: *mut c_int) -> pid_t;
    fn printf(format: *const c_char, ...) -> c_int;

    fn loadvsx(ptr: *mut c_ulong, offset: c_int);
    fn storevsx(ptr: *mut c_ulong, offset: c_int);
    fn compare_vsx_vmx(vsx: *mut c_ulong, vmx: *mut c_ulong) -> c_int;
    fn start_trace(child: pid_t) -> c_int;
    fn show_vsx(child: pid_t, vsx: *mut c_ulong) -> c_int;
    fn validate_vsx(vsx: *mut c_ulong, expected: *mut c_ulong) -> c_int;
    fn show_vmx(child: pid_t, vmx: *mut [[c_ulong; 2]; (VMX_MAX + 2) as usize]) -> c_int;
    fn validate_vmx(vmx: *mut [[c_ulong; 2]; (VMX_MAX + 2) as usize], expected: *mut c_ulong) -> c_int;
    fn load_vsx_vmx(
        fp: *mut c_ulong,
        vsx: *mut c_ulong,
        vmx: *mut [[c_ulong; 2]; (VMX_MAX + 2) as usize],
    );
    fn write_vsx(child: pid_t, vsx: *mut c_ulong) -> c_int;
    fn write_vmx(child: pid_t, vmx: *mut [[c_ulong; 2]; (VMX_MAX + 2) as usize]) -> c_int;
    fn stop_trace(child: pid_t) -> c_int;
    fn have_hwcap(feature: c_ulong) -> c_int;
    fn WIFEXITED(status: c_int) -> c_int;
    fn WEXITSTATUS(status: c_int) -> c_int;
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

type c_ulong = u64;

/* Tracer and Tracee Shared Data */
static mut shm_id: c_int = 0;
static mut cptr: *mut c_int = core::ptr::null_mut();
static mut pptr: *mut c_int = core::ptr::null_mut();

static mut fp_load: [c_ulong; VEC_MAX as usize] = [0; VEC_MAX as usize];
static mut fp_load_new: [c_ulong; VEC_MAX as usize] = [0; VEC_MAX as usize];
static mut fp_store: [c_ulong; VEC_MAX as usize] = [0; VEC_MAX as usize];

unsafe extern "C" fn vsx() {
    let ret: c_int;

    cptr = shmat(shm_id, core::ptr::null(), 0) as *mut c_int;
    loadvsx(core::ptr::addr_of_mut!(fp_load) as *mut c_ulong, 0);
    *cptr.add(1) = 1;

    while *cptr.add(0) == 0 {
        asm!("", options(nostack, preserves_flags));
    }
    shmdt(cptr as *mut c_void);

    storevsx(core::ptr::addr_of_mut!(fp_store) as *mut c_ulong, 0);
    ret = compare_vsx_vmx(
        core::ptr::addr_of_mut!(fp_store) as *mut c_ulong,
        core::ptr::addr_of_mut!(fp_load_new) as *mut c_ulong,
    );
    if ret != 0 {
        exit(1);
    }
    exit(0);
}

unsafe extern "C" fn trace_vsx(child: pid_t) -> c_int {
    let mut vsx: [c_ulong; VSX_MAX as usize] = [0; VSX_MAX as usize];
    let mut vmx: [[c_ulong; 2]; (VMX_MAX + 2) as usize] = [[0; 2]; (VMX_MAX + 2) as usize];

    if start_trace(child) != 0 {
        return TEST_FAIL;
    }
    if show_vsx(child, vsx.as_mut_ptr()) != 0 {
        return TEST_FAIL;
    }
    if validate_vsx(vsx.as_mut_ptr(), core::ptr::addr_of_mut!(fp_load) as *mut c_ulong) != 0 {
        return TEST_FAIL;
    }
    if show_vmx(child, vmx.as_mut_ptr()) != 0 {
        return TEST_FAIL;
    }
    if validate_vmx(vmx.as_mut_ptr(), core::ptr::addr_of_mut!(fp_load) as *mut c_ulong) != 0 {
        return TEST_FAIL;
    }

    memset(
        vsx.as_mut_ptr() as *mut c_void,
        0,
        core::mem::size_of_val(&vsx),
    );
    memset(
        vmx.as_mut_ptr() as *mut c_void,
        0,
        core::mem::size_of_val(&vmx),
    );
    load_vsx_vmx(
        core::ptr::addr_of_mut!(fp_load_new) as *mut c_ulong,
        vsx.as_mut_ptr(),
        vmx.as_mut_ptr(),
    );

    if write_vsx(child, vsx.as_mut_ptr()) != 0 {
        return TEST_FAIL;
    }
    if write_vmx(child, vmx.as_mut_ptr()) != 0 {
        return TEST_FAIL;
    }
    if stop_trace(child) != 0 {
        return TEST_FAIL;
    }

    TEST_PASS
}

unsafe extern "C" fn ptrace_vsx() -> c_int {
    let pid: pid_t;
    let mut ret: c_int;
    let mut status: c_int = 0;
    let mut i: c_int;

    // Original C used SKIP_IF_MSG(!have_hwcap(PPC_FEATURE_HAS_VSX), "Don't have VSX").
    if have_hwcap(PPC_FEATURE_HAS_VSX) == 0 {
        println!("Don't have VSX");
        return TEST_SKIP;
    }

    shm_id = shmget(IPC_PRIVATE, core::mem::size_of::<c_int>() * 2, 0o777 | IPC_CREAT);

    i = 0;
    while i < VEC_MAX {
        fp_load[i as usize] = (i + rand()) as c_ulong;
        i += 1;
    }

    i = 0;
    while i < VEC_MAX {
        fp_load_new[i as usize] = (i + 2 * rand()) as c_ulong;
        i += 1;
    }

    pid = fork();
    if pid < 0 {
        perror(c"fork() failed".as_ptr());
        return TEST_FAIL;
    }

    if pid == 0 {
        vsx();
    }

    if pid != 0 {
        pptr = shmat(shm_id, core::ptr::null(), 0) as *mut c_int;
        while *pptr.add(1) == 0 {
            asm!("", options(nostack, preserves_flags));
        }

        ret = trace_vsx(pid);
        if ret != 0 {
            kill(pid, SIGTERM);
            shmdt(pptr as *mut c_void);
            shmctl(shm_id, IPC_RMID, core::ptr::null_mut());
            return TEST_FAIL;
        }

        *pptr.add(0) = 1;
        shmdt(pptr as *mut c_void);

        ret = wait(&mut status);
        shmctl(shm_id, IPC_RMID, core::ptr::null_mut());
        if ret != pid {
            printf(c"Child's exit status not captured\n".as_ptr());
            return TEST_FAIL;
        }

        return if WIFEXITED(status) != 0 && WEXITSTATUS(status) != 0 {
            TEST_FAIL
        } else {
            TEST_PASS
        };
    }
    TEST_PASS
}

unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let _ = argc;
    let _ = argv;
    test_harness(ptrace_vsx, c"ptrace_vsx".as_ptr())
}
