// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2021 ARM Limited.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

/* Original C dependencies:
 * errno.h, stdbool.h, stddef.h, stdio.h, stdlib.h, string.h, unistd.h,
 * sys/auxv.h, sys/prctl.h, sys/ptrace.h, sys/types.h, sys/uio.h,
 * sys/wait.h, asm/sigcontext.h, asm/ptrace.h, and "kselftest.h".
 */

/* <linux/elf.h> and <sys/auxv.h> don't like each other, so: */
const NT_ARM_ZA: c_int = 0x40c;

/*
 * The architecture defines the maximum VQ as 16 but for extensibility
 * the kernel specifies the SVE_VQ_MAX as 512 resulting in us running
 * a *lot* more tests than are useful if we use it.  Until the
 * architecture is extended let's limit our coverage to what is
 * currently allowed, plus one extra to ensure we cover constraining
 * the VL as expected.
 */
const TEST_VQ_MAX: c_uint = 17;

const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;
const ENOMEM: c_int = 12;
const ESRCH: c_int = 3;
const EINVAL: c_int = 22;
const SIGSTOP: c_int = 19;
const SIGKILL: c_int = 9;
const SI_TKILL: c_int = -6;
const AT_HWCAP2: c_ulong = 26;
const HWCAP2_SME: c_ulong = 1 << 23;
const PTRACE_TRACEME: c_int = 0;
const PTRACE_CONT: c_int = 7;
const PTRACE_GETREGSET: c_int = 0x4204;
const PTRACE_SETREGSET: c_int = 0x4205;
const PTRACE_GETSIGINFO: c_int = 0x4202;
const PR_SME_SET_VL: c_int = 63;

type size_t = usize;
type pid_t = c_int;
type c_ulong = u64;

const SVE_VQ_MIN: c_uint = 1;
const ZA_PT_ZA_OFFSET: size_t = core::mem::size_of::<user_za_header>();
const EXPECTED_TESTS: c_uint = (((TEST_VQ_MAX - SVE_VQ_MIN) + 1) * 3);

#[repr(C)]
struct iovec {
    iov_base: *mut c_void,
    iov_len: size_t,
}

#[repr(C)]
struct siginfo_t {
    si_signo: c_int,
    si_errno: c_int,
    si_code: c_int,
    si_pid: pid_t,
    _rest: [u8; 128 - 4 * core::mem::size_of::<c_int>()],
}

#[repr(C)]
struct user_za_header {
    size: u32,
    max_size: u32,
    vl: u16,
    max_vl: u16,
    flags: u16,
    __reserved: [u16; 3],
}

unsafe extern "C" {
    static mut errno: c_int;

    fn random() -> c_long;
    fn srandom(seed: c_uint);
    fn getpid() -> pid_t;
    fn fork() -> pid_t;
    fn raise(sig: c_int) -> c_int;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn wait(status: *mut c_int) -> pid_t;
    fn getauxval(type_: c_ulong) -> c_ulong;
    fn prctl(option: c_int, ...) -> c_int;
    fn ptrace(request: c_int, ...) -> c_long;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn perror(s: *const c_char);
    fn malloc(size: size_t) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;

    fn ksft_print_header();
    fn ksft_set_plan(plan: c_uint);
    fn ksft_print_cnts();
    fn ksft_exit_skip(fmt: *const c_char, ...) -> !;
    fn ksft_exit_fail_msg(fmt: *const c_char, ...) -> !;
    fn ksft_test_result(condition: bool, fmt: *const c_char, ...);
    fn ksft_test_result_fail(fmt: *const c_char, ...);
    fn ksft_test_result_skip(fmt: *const c_char, ...);
    fn ksft_print_msg(fmt: *const c_char, ...);
}

fn sve_vq_from_vl(vl: c_uint) -> c_uint {
    vl / 16
}

fn sve_vl_from_vq(vq: c_uint) -> c_uint {
    vq * 16
}

fn ZA_PT_ZA_SIZE(vq: c_uint) -> size_t {
    let vl = sve_vl_from_vq(vq) as size_t;
    vl * vl
}

fn ZA_PT_SIZE(vq: c_uint) -> size_t {
    ZA_PT_ZA_OFFSET + ZA_PT_ZA_SIZE(vq)
}

fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn WIFSIGNALED(status: c_int) -> bool {
    (((status & 0x7f) + 1) >> 1) > 0
}

fn WIFSTOPPED(status: c_int) -> bool {
    (status & 0xff) == 0x7f
}

fn WSTOPSIG(status: c_int) -> c_int {
    (status >> 8) & 0xff
}

unsafe fn fill_buf(buf: *mut c_char, size: size_t) {
    let mut i: c_int;

    i = 0;
    while (i as size_t) < size {
        *buf.add(i as usize) = random() as c_char;
        i += 1;
    }
}

unsafe fn do_child() -> c_int {
    if ptrace(PTRACE_TRACEME, -1 as c_long, core::ptr::null_mut::<c_void>(), core::ptr::null_mut::<c_void>()) != 0 {
        ksft_exit_fail_msg(
            c"ptrace(PTRACE_TRACEME) failed: %s (%d)".as_ptr(),
            strerror(errno),
            errno,
        );
    }

    if raise(SIGSTOP) != 0 {
        ksft_exit_fail_msg(
            c"raise(SIGSTOP) failed: %s (%d)\n".as_ptr(),
            strerror(errno),
            errno,
        );
    }

    EXIT_SUCCESS
}

unsafe fn get_za(pid: pid_t, buf: *mut *mut c_void, size: *mut size_t) -> *mut user_za_header {
    let mut za: *mut user_za_header;
    let mut p: *mut c_void;
    let mut sz: size_t = core::mem::size_of::<user_za_header>();
    let mut iov: iovec = core::mem::zeroed();

    loop {
        if *size < sz {
            p = realloc(*buf, sz);
            if p.is_null() {
                errno = ENOMEM;
                return core::ptr::null_mut();
            }

            *buf = p;
            *size = sz;
        }

        iov.iov_base = *buf;
        iov.iov_len = sz;
        if ptrace(PTRACE_GETREGSET, pid, NT_ARM_ZA, &mut iov as *mut iovec) != 0 {
            return core::ptr::null_mut();
        }

        za = *buf as *mut user_za_header;
        if (*za).size as size_t <= sz {
            break;
        }

        sz = (*za).size as size_t;
    }

    return za;
}

unsafe fn set_za(pid: pid_t, za: *const user_za_header) -> c_int {
    let mut iov: iovec = core::mem::zeroed();

    iov.iov_base = za as *mut c_void;
    iov.iov_len = (*za).size as size_t;
    ptrace(PTRACE_SETREGSET, pid, NT_ARM_ZA, &mut iov as *mut iovec) as c_int
}

/* Validate attempting to set the specfied VL via ptrace */
unsafe fn ptrace_set_get_vl(child: pid_t, vl: c_uint, supported: *mut bool) {
    let mut za: user_za_header = core::mem::zeroed();
    let mut new_za: *mut user_za_header = core::ptr::null_mut();
    let mut new_za_size: size_t = 0;
    let ret: c_int;
    let prctl_vl: c_int;

    *supported = false;

    /* Check if the VL is supported in this process */
    prctl_vl = prctl(PR_SME_SET_VL, vl);
    if prctl_vl == -1 {
        ksft_exit_fail_msg(
            c"prctl(PR_SME_SET_VL) failed: %s (%d)\n".as_ptr(),
            strerror(errno),
            errno,
        );
    }

    /* If the VL is not supported then a supported VL will be returned */
    *supported = prctl_vl == vl as c_int;

    /* Set the VL by doing a set with no register payload */
    memset(&mut za as *mut user_za_header as *mut c_void, 0, core::mem::size_of::<user_za_header>());
    za.size = core::mem::size_of::<user_za_header>() as u32;
    za.vl = vl as u16;
    ret = set_za(child, &za as *const user_za_header);
    if ret != 0 {
        ksft_test_result_fail(c"Failed to set VL %u\n".as_ptr(), vl);
        return;
    }

    /*
     * Read back the new register state and verify that we have the
     * same VL that we got from prctl() on ourselves.
     */
    if get_za(child, &mut new_za as *mut *mut user_za_header as *mut *mut c_void, &mut new_za_size).is_null() {
        ksft_test_result_fail(c"Failed to read VL %u\n".as_ptr(), vl);
        return;
    }

    ksft_test_result((*new_za).vl as c_int == prctl_vl, c"Set VL %u\n".as_ptr(), vl);

    free(new_za as *mut c_void);
}

/* Validate attempting to set no ZA data and read it back */
unsafe fn ptrace_set_no_data(child: pid_t, vl: c_uint) {
    let mut read_buf: *mut c_void = core::ptr::null_mut();
    let mut write_za: user_za_header = core::mem::zeroed();
    let read_za: *mut user_za_header;
    let mut read_za_size: size_t = 0;
    let ret: c_int;

    /* Set up some data and write it out */
    memset(&mut write_za as *mut user_za_header as *mut c_void, 0, core::mem::size_of::<user_za_header>());
    write_za.size = ZA_PT_ZA_OFFSET as u32;
    write_za.vl = vl as u16;

    ret = set_za(child, &write_za as *const user_za_header);
    if ret != 0 {
        ksft_test_result_fail(c"Failed to set VL %u no data\n".as_ptr(), vl);
        return;
    }

    /* Read the data back */
    if get_za(child, &mut read_buf as *mut *mut c_void, &mut read_za_size).is_null() {
        ksft_test_result_fail(c"Failed to read VL %u no data\n".as_ptr(), vl);
        return;
    }
    read_za = read_buf as *mut user_za_header;

    /* We might read more data if there's extensions we don't know */
    if (*read_za).size < write_za.size {
        ksft_test_result_fail(
            c"VL %u wrote %d bytes, only read %d\n".as_ptr(),
            vl,
            write_za.size,
            (*read_za).size,
        );
        free(read_buf);
        return;
    }

    ksft_test_result((*read_za).size == write_za.size, c"Disabled ZA for VL %u\n".as_ptr(), vl);

    free(read_buf);
}

/* Validate attempting to set data and read it back */
unsafe fn ptrace_set_get_data(child: pid_t, vl: c_uint) {
    let write_buf: *mut c_void;
    let mut read_buf: *mut c_void = core::ptr::null_mut();
    let write_za: *mut user_za_header;
    let read_za: *mut user_za_header;
    let mut read_za_size: size_t = 0;
    let vq: c_uint = sve_vq_from_vl(vl);
    let ret: c_int;
    let data_size: size_t;

    data_size = ZA_PT_SIZE(vq);
    write_buf = malloc(data_size);
    if write_buf.is_null() {
        ksft_test_result_fail(
            c"Error allocating %ld byte buffer for VL %u\n".as_ptr(),
            data_size as c_long,
            vl,
        );
        return;
    }
    write_za = write_buf as *mut user_za_header;

    /* Set up some data and write it out */
    memset(write_za as *mut c_void, 0, data_size);
    (*write_za).size = data_size as u32;
    (*write_za).vl = vl as u16;

    fill_buf((write_buf as *mut c_char).add(ZA_PT_ZA_OFFSET), ZA_PT_ZA_SIZE(vq));

    ret = set_za(child, write_za);
    if ret != 0 {
        ksft_test_result_fail(c"Failed to set VL %u data\n".as_ptr(), vl);
        free(write_buf);
        return;
    }

    /* Read the data back */
    if get_za(child, &mut read_buf as *mut *mut c_void, &mut read_za_size).is_null() {
        ksft_test_result_fail(c"Failed to read VL %u data\n".as_ptr(), vl);
        free(write_buf);
        return;
    }
    read_za = read_buf as *mut user_za_header;

    /* We might read more data if there's extensions we don't know */
    if (*read_za).size < (*write_za).size {
        ksft_test_result_fail(
            c"VL %u wrote %d bytes, only read %d\n".as_ptr(),
            vl,
            (*write_za).size,
            (*read_za).size,
        );
        free(read_buf);
        free(write_buf);
        return;
    }

    ksft_test_result(
        memcmp(
            (write_buf as *mut c_char).add(ZA_PT_ZA_OFFSET) as *const c_void,
            (read_buf as *mut c_char).add(ZA_PT_ZA_OFFSET) as *const c_void,
            ZA_PT_ZA_SIZE(vq),
        ) == 0,
        c"Data match for VL %u\n".as_ptr(),
        vl,
    );

    free(read_buf);
    free(write_buf);
}

unsafe fn do_parent(child: pid_t) -> c_int {
    let mut ret: c_int = EXIT_FAILURE;
    let mut pid: pid_t;
    let mut status: c_int = 0;
    let mut si: siginfo_t = core::mem::zeroed();
    let mut vq: c_uint;
    let mut vl: c_uint;
    let mut vl_supported: bool = false;

    /* Attach to the child */
    loop {
        let mut sig: c_int;

        pid = wait(&mut status as *mut c_int);
        if pid == -1 {
            perror(c"wait".as_ptr());
            kill(child, SIGKILL);
            return ret;
        }

        /*
         * This should never happen but it's hard to flag in
         * the framework.
         */
        if pid != child {
            continue;
        }

        if WIFEXITED(status) || WIFSIGNALED(status) {
            ksft_exit_fail_msg(c"Child died unexpectedly\n".as_ptr());
        }

        if !WIFSTOPPED(status) {
            kill(child, SIGKILL);
            return ret;
        }

        sig = WSTOPSIG(status);

        if ptrace(PTRACE_GETSIGINFO, pid, core::ptr::null_mut::<c_void>(), &mut si as *mut siginfo_t) != 0 {
            if errno == ESRCH {
                return ret;
            }

            if errno == EINVAL {
                sig = 0; /* bust group-stop */
            } else {
                ksft_test_result_fail(c"PTRACE_GETSIGINFO: %s\n".as_ptr(), strerror(errno));
                kill(child, SIGKILL);
                return ret;
            }
        } else if sig == SIGSTOP && si.si_code == SI_TKILL && si.si_pid == pid {
            break;
        }

        if ptrace(PTRACE_CONT, pid, core::ptr::null_mut::<c_void>(), sig) != 0 {
            if errno == ESRCH {
                return ret;
            }

            ksft_test_result_fail(c"PTRACE_CONT: %s\n".as_ptr(), strerror(errno));
            kill(child, SIGKILL);
            return ret;
        }
    }

    ksft_print_msg(c"Parent is %d, child is %d\n".as_ptr(), getpid(), child);

    /* Step through every possible VQ */
    vq = SVE_VQ_MIN;
    while vq <= TEST_VQ_MAX {
        vl = sve_vl_from_vq(vq);

        /* First, try to set this vector length */
        ptrace_set_get_vl(child, vl, &mut vl_supported as *mut bool);

        /* If the VL is supported validate data set/get */
        if vl_supported {
            ptrace_set_no_data(child, vl);
            ptrace_set_get_data(child, vl);
        } else {
            ksft_test_result_skip(c"Disabled ZA for VL %u\n".as_ptr(), vl);
            ksft_test_result_skip(c"Get and set data for VL %u\n".as_ptr(), vl);
        }

        vq += 1;
    }

    ret = EXIT_SUCCESS;

    kill(child, SIGKILL);

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> c_int {
    let mut ret: c_int = EXIT_SUCCESS;
    let child: pid_t;

    srandom(getpid() as c_uint);

    ksft_print_header();

    if (getauxval(AT_HWCAP2) & HWCAP2_SME) == 0 {
        ksft_set_plan(1);
        ksft_exit_skip(c"SME not available\n".as_ptr());
    }

    ksft_set_plan(EXPECTED_TESTS);

    child = fork();
    if child == 0 {
        return do_child();
    }

    if do_parent(child) != 0 {
        ret = EXIT_FAILURE;
    }

    ksft_print_cnts();

    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
