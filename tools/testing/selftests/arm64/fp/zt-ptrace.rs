// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2021 ARM Limited.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

/* C header dependencies from the original source:
 * errno.h, stdbool.h, stddef.h, stdio.h, stdlib.h, string.h, unistd.h,
 * sys/auxv.h, sys/prctl.h, sys/ptrace.h, sys/types.h, sys/uio.h,
 * sys/wait.h, asm/sigcontext.h, asm/ptrace.h, and "kselftest.h".
 */

type pid_t = c_int;
type size_t = usize;

/* <linux/elf.h> and <sys/auxv.h> don't like each other, so: */
const NT_ARM_ZA: c_int = 0x40c;
const NT_ARM_ZT: c_int = 0x40d;

const EXPECTED_TESTS: c_uint = 3;

const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;
const ENOMEM: c_int = 12;
const ESRCH: c_int = 3;
const EINVAL: c_int = 22;
const SIGSTOP: c_int = 19;
const SIGKILL: c_int = 9;
const SI_TKILL: c_int = -6;
const AT_HWCAP2: c_ulong = 26;
const HWCAP2_SME2: c_ulong = 1 << 37;
const PR_SME_GET_VL: c_int = 63;
const PTRACE_TRACEME: c_int = 0;
const PTRACE_CONT: c_int = 7;
const PTRACE_GETREGSET: c_int = 0x4204;
const PTRACE_SETREGSET: c_int = 0x4205;
const PTRACE_GETSIGINFO: c_int = 0x4202;
const ZT_SIG_REG_BYTES: usize = 512;
const ZA_PT_ZA_OFFSET: usize = 16;

static mut sme_vl: c_int = 0;

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
    _rest: [u8; 112],
}

#[repr(C)]
struct user_za_header {
    size: u32,
    max_size: u32,
    vl: u16,
    max_vl: u16,
    flags: u16,
    reserved: u16,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn fork() -> pid_t;
    fn getpid() -> pid_t;
    fn getauxval(type_: c_ulong) -> c_ulong;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn perror(s: *const c_char);
    fn prctl(option: c_int, ...) -> c_int;
    fn ptrace(request: c_int, ...) -> c_long;
    fn raise(sig: c_int) -> c_int;
    fn random() -> c_long;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn srandom(seed: c_uint);
    fn strerror(errnum: c_int) -> *mut c_char;
    fn wait(wstatus: *mut c_int) -> pid_t;

    fn ksft_exit_fail_msg(fmt: *const c_char, ...) -> !;
    fn ksft_exit_skip(fmt: *const c_char, ...) -> !;
    fn ksft_print_cnts();
    fn ksft_print_header();
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_set_plan(plan: c_uint);
    fn ksft_test_result(condition: bool, name: *const c_char, ...);
    fn ksft_test_result_fail(fmt: *const c_char, ...);
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

fn __sve_vq_from_vl(vl: c_int) -> c_int {
    vl / 16
}

fn ZA_PT_SIZE(vq: c_int) -> u32 {
    (ZA_PT_ZA_OFFSET + ZA_PT_ZA_SIZE(vq)) as u32
}

fn ZA_PT_ZA_SIZE(vq: c_int) -> usize {
    (vq as usize) * (vq as usize) * 16
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
    if ptrace(PTRACE_TRACEME, -1 as c_long, ptr::null_mut::<c_void>(), ptr::null_mut::<c_void>()) != 0 {
        ksft_exit_fail_msg(
            c"ptrace(PTRACE_TRACEME) failed: %s (%d)\n".as_ptr(),
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
    let mut sz: size_t = mem::size_of::<user_za_header>();
    let mut iov: iovec;

    loop {
        if *size < sz {
            p = realloc(*buf, sz);
            if p.is_null() {
                errno = ENOMEM;
                return ptr::null_mut();
            }

            *buf = p;
            *size = sz;
        }

        iov = iovec {
            iov_base: *buf,
            iov_len: sz,
        };
        if ptrace(PTRACE_GETREGSET, pid, NT_ARM_ZA, &mut iov as *mut iovec) != 0 {
            return ptr::null_mut();
        }

        za = *buf as *mut user_za_header;
        if (*za).size as size_t <= sz {
            break;
        }

        sz = (*za).size as size_t;
    }

    za
}

unsafe fn set_za(pid: pid_t, za: *const user_za_header) -> c_int {
    let mut iov: iovec;

    iov = iovec {
        iov_base: za as *mut c_void,
        iov_len: (*za).size as size_t,
    };
    ptrace(PTRACE_SETREGSET, pid, NT_ARM_ZA, &mut iov as *mut iovec) as c_int
}

unsafe fn get_zt(pid: pid_t, zt: *mut c_char) -> c_int {
    let mut iov: iovec;

    iov = iovec {
        iov_base: zt as *mut c_void,
        iov_len: ZT_SIG_REG_BYTES,
    };
    ptrace(PTRACE_GETREGSET, pid, NT_ARM_ZT, &mut iov as *mut iovec) as c_int
}

unsafe fn set_zt(pid: pid_t, zt: *const c_char) -> c_int {
    let mut iov: iovec;

    iov = iovec {
        iov_base: zt as *mut c_void,
        iov_len: ZT_SIG_REG_BYTES,
    };
    ptrace(PTRACE_SETREGSET, pid, NT_ARM_ZT, &mut iov as *mut iovec) as c_int
}

/* Reading with ZA disabled returns all zeros */
unsafe fn ptrace_za_disabled_read_zt(child: pid_t) {
    let mut za: user_za_header = mem::zeroed();
    let mut zt: [c_char; ZT_SIG_REG_BYTES] = [0; ZT_SIG_REG_BYTES];
    let mut ret: c_int;
    let mut i: c_int;
    let mut fail: bool = false;

    /* Disable PSTATE.ZA using the ZA interface */
    memset(
        &mut za as *mut user_za_header as *mut c_void,
        0,
        mem::size_of_val(&za),
    );
    za.vl = sme_vl as u16;
    za.size = mem::size_of_val(&za) as u32;

    ret = set_za(child, &za);
    if ret != 0 {
        ksft_print_msg(c"Failed to disable ZA\n".as_ptr());
        fail = true;
    }

    /* Read back ZT */
    ret = get_zt(child, zt.as_mut_ptr());
    if ret != 0 {
        ksft_print_msg(c"Failed to read ZT\n".as_ptr());
        fail = true;
    }

    i = 0;
    while (i as usize) < zt.len() {
        if zt[i as usize] != 0 {
            ksft_print_msg(c"zt[%d]: 0x%x != 0\n".as_ptr(), i, zt[i as usize] as c_int);
            fail = true;
        }
        i += 1;
    }

    ksft_test_result(!fail, c"ptrace_za_disabled_read_zt\n".as_ptr());
}

/* Writing then reading ZT should return the data written */
unsafe fn ptrace_set_get_zt(child: pid_t) {
    let mut zt_in: [c_char; ZT_SIG_REG_BYTES] = [0; ZT_SIG_REG_BYTES];
    let mut zt_out: [c_char; ZT_SIG_REG_BYTES] = [0; ZT_SIG_REG_BYTES];
    let mut ret: c_int;
    let mut i: c_int;
    let mut fail: bool = false;

    fill_buf(zt_in.as_mut_ptr(), mem::size_of_val(&zt_in));

    ret = set_zt(child, zt_in.as_ptr());
    if ret != 0 {
        ksft_print_msg(c"Failed to set ZT\n".as_ptr());
        fail = true;
    }

    ret = get_zt(child, zt_out.as_mut_ptr());
    if ret != 0 {
        ksft_print_msg(c"Failed to read ZT\n".as_ptr());
        fail = true;
    }

    i = 0;
    while (i as usize) < zt_in.len() {
        if zt_in[i as usize] != zt_out[i as usize] {
            ksft_print_msg(
                c"zt[%d]: 0x%x != 0x%x\n".as_ptr(),
                i,
                zt_in[i as usize] as c_int,
                zt_out[i as usize] as c_int,
            );
            fail = true;
        }
        i += 1;
    }

    ksft_test_result(!fail, c"ptrace_set_get_zt\n".as_ptr());
}

/* Writing ZT should set PSTATE.ZA */
unsafe fn ptrace_enable_za_via_zt(child: pid_t) {
    let mut za_in: user_za_header = mem::zeroed();
    let mut za_out: *mut user_za_header;
    let mut zt: [c_char; ZT_SIG_REG_BYTES] = [0; ZT_SIG_REG_BYTES];
    let mut za_data: *mut c_char;
    let mut za_out_size: size_t;
    let mut ret: c_int;
    let mut i: c_int;
    let mut vq: c_int;
    let mut fail: bool = false;

    /* Disable PSTATE.ZA using the ZA interface */
    memset(
        &mut za_in as *mut user_za_header as *mut c_void,
        0,
        mem::size_of_val(&za_in),
    );
    za_in.vl = sme_vl as u16;
    za_in.size = mem::size_of_val(&za_in) as u32;

    ret = set_za(child, &za_in);
    if ret != 0 {
        ksft_print_msg(c"Failed to disable ZA\n".as_ptr());
        fail = true;
    }

    /* Write ZT */
    fill_buf(zt.as_mut_ptr(), mem::size_of_val(&zt));
    ret = set_zt(child, zt.as_ptr());
    if ret != 0 {
        ksft_print_msg(c"Failed to set ZT\n".as_ptr());
        fail = true;
    }

    /* Read back ZA and check for register data */
    za_out = ptr::null_mut();
    za_out_size = 0;
    if !get_za(child, &mut za_out as *mut *mut user_za_header as *mut *mut c_void, &mut za_out_size).is_null() {
        /* Should have an unchanged VL */
        if (*za_out).vl as c_int != sme_vl {
            ksft_print_msg(
                c"VL changed from %d to %d\n".as_ptr(),
                sme_vl,
                (*za_out).vl as c_int,
            );
            fail = true;
        }
        vq = __sve_vq_from_vl((*za_out).vl as c_int);
        za_data = (za_out as *mut c_char).add(ZA_PT_ZA_OFFSET);

        /* Should have register data */
        if (*za_out).size < ZA_PT_SIZE(vq) {
            ksft_print_msg(
                c"ZA data less than expected: %u < %u\n".as_ptr(),
                (*za_out).size,
                ZA_PT_SIZE(vq) as c_uint,
            );
            fail = true;
            vq = 0;
        }

        /* That register data should be non-zero */
        i = 0;
        while (i as usize) < ZA_PT_ZA_SIZE(vq) {
            if *za_data.add(i as usize) != 0 {
                ksft_print_msg(
                    c"ZA byte %d is %x\n".as_ptr(),
                    i,
                    *za_data.add(i as usize) as c_int,
                );
                fail = true;
            }
            i += 1;
        }
    } else {
        ksft_print_msg(c"Failed to read ZA\n".as_ptr());
        fail = true;
    }

    ksft_test_result(!fail, c"ptrace_enable_za_via_zt\n".as_ptr());
}

unsafe fn do_parent(child: pid_t) -> c_int {
    let mut ret: c_int = EXIT_FAILURE;
    let mut pid: pid_t;
    let mut status: c_int = 0;
    let mut si: siginfo_t = mem::zeroed();

    /* Attach to the child */
    loop {
        let mut sig: c_int;

        pid = wait(&mut status);
        if pid == -1 {
            perror(c"wait".as_ptr());
            goto_error(child, ret);
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
            goto_error(child, ret);
            return ret;
        }

        sig = WSTOPSIG(status);

        if ptrace(
            PTRACE_GETSIGINFO,
            pid,
            ptr::null_mut::<c_void>(),
            &mut si as *mut siginfo_t,
        ) != 0
        {
            if errno == ESRCH {
                return ret;
            }

            if errno == EINVAL {
                sig = 0; /* bust group-stop */
            } else {
                ksft_test_result_fail(c"PTRACE_GETSIGINFO: %s\n".as_ptr(), strerror(errno));
                goto_error(child, ret);
                return ret;
            }
        } else if sig == SIGSTOP && si.si_code == SI_TKILL && si.si_pid == pid {
            break;
        }

        if ptrace(PTRACE_CONT, pid, ptr::null_mut::<c_void>(), sig) != 0 {
            if errno == ESRCH {
                return ret;
            }

            ksft_test_result_fail(c"PTRACE_CONT: %s\n".as_ptr(), strerror(errno));
            goto_error(child, ret);
            return ret;
        }
    }

    ksft_print_msg(c"Parent is %d, child is %d\n".as_ptr(), getpid(), child);

    ptrace_za_disabled_read_zt(child);
    ptrace_set_get_zt(child);
    ptrace_enable_za_via_zt(child);

    ret = EXIT_SUCCESS;

    goto_error(child, ret)
}

unsafe fn goto_error(child: pid_t, ret: c_int) -> c_int {
    kill(child, SIGKILL);

    ret
}

fn main() -> c_int {
    unsafe {
        let mut ret: c_int = EXIT_SUCCESS;
        let child: pid_t;

        srandom(getpid() as c_uint);

        ksft_print_header();

        if (getauxval(AT_HWCAP2) & HWCAP2_SME2) == 0 {
            ksft_set_plan(1);
            ksft_exit_skip(c"SME2 not available\n".as_ptr());
        }

        /* We need a valid SME VL to enable/disable ZA */
        sme_vl = prctl(PR_SME_GET_VL);
        if sme_vl == -1 {
            ksft_set_plan(1);
            ksft_exit_skip(
                c"Failed to read SME VL: %d (%s)\n".as_ptr(),
                errno,
                strerror(errno),
            );
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
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
