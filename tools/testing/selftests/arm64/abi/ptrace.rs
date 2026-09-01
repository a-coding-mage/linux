// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2022 ARM Limited.
 */

use libc::{
    c_char, c_int, c_long, c_uint, c_ulong, c_void, pid_t, siginfo_t, size_t, EXIT_FAILURE,
    EXIT_SUCCESS,
};
use std::mem;
use std::ptr;

const EXPECTED_TESTS: c_uint = 11;

const MAX_TPIDRS: usize = 2;

const AT_HWCAP2: c_ulong = 26;
const HWCAP2_SME: c_ulong = 1 << 23;

const PTRACE_TRACEME: c_int = 0;
const PTRACE_CONT: c_int = 7;
const PTRACE_GETREGSET: c_int = 0x4204;
const PTRACE_SETREGSET: c_int = 0x4205;
const PTRACE_GETSIGINFO: c_int = 0x4202;

const NT_ARM_TLS: c_int = 0x401;
const NT_ARM_HW_BREAK: c_int = 0x402;
const NT_ARM_HW_WATCH: c_int = 0x403;

const SIGSTOP: c_int = 19;
const SIGKILL: c_int = 9;
const SI_TKILL: c_int = -6;

#[repr(C)]
struct user_hwdebug_reg {
    addr: u64,
    ctrl: u32,
    pad: u32,
}

#[repr(C)]
struct user_hwdebug_state {
    dbg_info: u32,
    pad: u32,
    dbg_regs: [user_hwdebug_reg; 16],
}

unsafe extern "C" {
    fn getauxval(type_: c_ulong) -> c_ulong;
    fn ptrace(request: c_int, pid: pid_t, ...) -> c_long;
    fn raise(sig: c_int) -> c_int;
    fn wait(wstatus: *mut c_int) -> pid_t;
    fn perror(s: *const c_char);
    fn strerror(errnum: c_int) -> *mut c_char;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn getpid() -> pid_t;
    fn fork() -> pid_t;
    fn srandom(seed: c_uint);

    fn ksft_print_header();
    fn ksft_set_plan(cnt: c_uint);
    fn ksft_print_cnts();
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_test_result(condition: bool, fmt: *const c_char, ...);
    fn ksft_test_result_fail(fmt: *const c_char, ...);
    fn ksft_test_result_skip(fmt: *const c_char, ...);
    fn ksft_exit_fail_perror(msg: *const c_char) -> !;
    fn ksft_exit_fail_msg(fmt: *const c_char, ...) -> !;
}

unsafe extern "C" {
    static mut errno: c_int;
}

fn wifexited(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn wifsignaled(status: c_int) -> bool {
    (((status & 0x7f) + 1) >> 1) > 0
}

fn wifstopped(status: c_int) -> bool {
    (status & 0xff) == 0x7f
}

fn wstopsig(status: c_int) -> c_int {
    (status >> 8) & 0xff
}

unsafe fn have_sme() -> bool {
    (getauxval(AT_HWCAP2) & HWCAP2_SME) != 0
}

unsafe fn test_tpidr(child: pid_t) {
    let mut read_val = [0_u64; MAX_TPIDRS];
    let mut write_val = [0_u64; MAX_TPIDRS];
    let mut read_iov = libc::iovec {
        iov_base: ptr::null_mut(),
        iov_len: 0,
    };
    let mut write_iov = libc::iovec {
        iov_base: ptr::null_mut(),
        iov_len: 0,
    };
    let mut test_tpidr2 = false;
    let mut ret: c_long;

    read_iov.iov_base = read_val.as_mut_ptr() as *mut c_void;
    write_iov.iov_base = write_val.as_mut_ptr() as *mut c_void;

    /* Should be able to read a single TPIDR... */
    read_iov.iov_len = mem::size_of::<u64>() as size_t;
    ret = ptrace(
        PTRACE_GETREGSET,
        child,
        NT_ARM_TLS,
        &mut read_iov as *mut libc::iovec,
    );
    ksft_test_result(ret == 0, c"read_tpidr_one\n".as_ptr());

    /* ...write a new value.. */
    write_iov.iov_len = mem::size_of::<u64>() as size_t;
    write_val[0] = read_val[0].wrapping_add(1);
    ret = ptrace(
        PTRACE_SETREGSET,
        child,
        NT_ARM_TLS,
        &mut write_iov as *mut libc::iovec,
    );
    ksft_test_result(ret == 0, c"write_tpidr_one\n".as_ptr());

    /* ...then read it back */
    ret = ptrace(
        PTRACE_GETREGSET,
        child,
        NT_ARM_TLS,
        &mut read_iov as *mut libc::iovec,
    );
    ksft_test_result(
        ret == 0 && write_val[0] == read_val[0],
        c"verify_tpidr_one\n".as_ptr(),
    );

    /* If we have TPIDR2 we should be able to read it */
    read_iov.iov_len = mem::size_of_val(&read_val) as size_t;
    ret = ptrace(
        PTRACE_GETREGSET,
        child,
        NT_ARM_TLS,
        &mut read_iov as *mut libc::iovec,
    );
    if ret == 0 {
        /* If we have SME there should be two TPIDRs */
        if read_iov.iov_len >= mem::size_of_val(&read_val) as size_t {
            test_tpidr2 = true;
        }

        if have_sme() && test_tpidr2 {
            ksft_test_result(test_tpidr2, c"count_tpidrs\n".as_ptr());
        } else {
            ksft_test_result(
                read_iov.iov_len % mem::size_of::<u64>() as size_t == 0,
                c"count_tpidrs\n".as_ptr(),
            );
        }
    } else {
        ksft_test_result_fail(c"count_tpidrs\n".as_ptr());
    }

    if test_tpidr2 {
        /* Try to write new values to all known TPIDRs... */
        write_iov.iov_len = mem::size_of_val(&write_val) as size_t;
        for i in 0..MAX_TPIDRS {
            write_val[i] = read_val[i].wrapping_add(1);
        }
        ret = ptrace(
            PTRACE_SETREGSET,
            child,
            NT_ARM_TLS,
            &mut write_iov as *mut libc::iovec,
        );

        ksft_test_result(
            ret == 0 && write_iov.iov_len == mem::size_of_val(&write_val) as size_t,
            c"tpidr2_write\n".as_ptr(),
        );

        /* ...then read them back */
        read_iov.iov_len = mem::size_of_val(&read_val) as size_t;
        ret = ptrace(
            PTRACE_GETREGSET,
            child,
            NT_ARM_TLS,
            &mut read_iov as *mut libc::iovec,
        );

        if have_sme() {
            /* Should read back the written value */
            ksft_test_result(
                ret == 0
                    && read_iov.iov_len >= mem::size_of_val(&read_val) as size_t
                    && libc::memcmp(
                        read_val.as_ptr() as *const c_void,
                        write_val.as_ptr() as *const c_void,
                        mem::size_of_val(&read_val),
                    ) == 0,
                c"tpidr2_read\n".as_ptr(),
            );
        } else {
            /* TPIDR2 should read as zero */
            ksft_test_result(
                ret == 0
                    && read_iov.iov_len >= mem::size_of_val(&read_val) as size_t
                    && read_val[0] == write_val[0]
                    && read_val[1] == 0,
                c"tpidr2_read\n".as_ptr(),
            );
        }

        /* Writing only TPIDR... */
        write_iov.iov_len = mem::size_of::<u64>() as size_t;
        libc::memcpy(
            write_val.as_mut_ptr() as *mut c_void,
            read_val.as_ptr() as *const c_void,
            mem::size_of_val(&read_val),
        );
        write_val[0] = write_val[0].wrapping_add(1);
        ret = ptrace(
            PTRACE_SETREGSET,
            child,
            NT_ARM_TLS,
            &mut write_iov as *mut libc::iovec,
        );

        if ret == 0 {
            /* ...should leave TPIDR2 untouched */
            read_iov.iov_len = mem::size_of_val(&read_val) as size_t;
            ret = ptrace(
                PTRACE_GETREGSET,
                child,
                NT_ARM_TLS,
                &mut read_iov as *mut libc::iovec,
            );

            ksft_test_result(
                ret == 0
                    && read_iov.iov_len >= mem::size_of_val(&read_val) as size_t
                    && libc::memcmp(
                        read_val.as_ptr() as *const c_void,
                        write_val.as_ptr() as *const c_void,
                        mem::size_of_val(&read_val),
                    ) == 0,
                c"write_tpidr_only\n".as_ptr(),
            );
        } else {
            ksft_test_result_fail(c"write_tpidr_only\n".as_ptr());
        }
    } else {
        ksft_test_result_skip(c"tpidr2_write\n".as_ptr());
        ksft_test_result_skip(c"tpidr2_read\n".as_ptr());
        ksft_test_result_skip(c"write_tpidr_only\n".as_ptr());
    }
}

unsafe fn test_hw_debug(child: pid_t, type_: c_int, type_name: *const c_char) {
    let mut state: user_hwdebug_state = mem::zeroed();
    let mut iov = libc::iovec {
        iov_base: ptr::null_mut(),
        iov_len: 0,
    };
    let slots: c_int;
    let arch: c_int;
    let ret: c_long;

    iov.iov_len = mem::size_of_val(&state) as size_t;
    iov.iov_base = &mut state as *mut user_hwdebug_state as *mut c_void;

    /* Should be able to read the values */
    ret = ptrace(
        PTRACE_GETREGSET,
        child,
        type_,
        &mut iov as *mut libc::iovec,
    );
    ksft_test_result(ret == 0, c"read_%s\n".as_ptr(), type_name);

    if ret == 0 {
        /* Low 8 bits is the number of slots, next 4 bits the arch */
        slots = (state.dbg_info & 0xff) as c_int;
        arch = ((state.dbg_info >> 8) & 0xf) as c_int;

        ksft_print_msg(
            c"%s version %d with %d slots\n".as_ptr(),
            type_name,
            arch,
            slots,
        );

        /* Zero is not currently architecturally valid */
        ksft_test_result(arch != 0, c"%s_arch_set\n".as_ptr(), type_name);
    } else {
        ksft_test_result_skip(c"%s_arch_set\n".as_ptr(), type_name);
    }
}

unsafe fn do_child() -> c_int {
    if ptrace(PTRACE_TRACEME, -1 as pid_t, ptr::null_mut::<c_void>(), ptr::null_mut::<c_void>())
        != 0
    {
        ksft_exit_fail_perror(c"PTRACE_TRACEME".as_ptr());
    }

    if raise(SIGSTOP) != 0 {
        ksft_exit_fail_perror(c"raise(SIGSTOP)".as_ptr());
    }

    EXIT_SUCCESS
}

unsafe fn do_parent(child: pid_t) -> c_int {
    let mut ret = EXIT_FAILURE;
    let mut pid: pid_t;
    let mut status: c_int = 0;
    let mut si: siginfo_t = mem::zeroed();

    /* Attach to the child */
    loop {
        let mut sig: c_int;

        pid = wait(&mut status as *mut c_int);
        if pid == -1 {
            perror(c"wait".as_ptr());
            goto_error(child);
            return ret;
        }

        /*
         * This should never happen but it's hard to flag in
         * the framework.
         */
        if pid != child {
            continue;
        }

        if wifexited(status) || wifsignaled(status) {
            ksft_exit_fail_msg(c"Child died unexpectedly\n".as_ptr());
        }

        if !wifstopped(status) {
            goto_error(child);
            return ret;
        }

        sig = wstopsig(status);

        if ptrace(
            PTRACE_GETSIGINFO,
            pid,
            ptr::null_mut::<c_void>(),
            &mut si as *mut siginfo_t,
        ) != 0
        {
            if errno == libc::ESRCH {
                return ret;
            }

            if errno == libc::EINVAL {
                sig = 0; /* bust group-stop */
            } else {
                ksft_test_result_fail(
                    c"PTRACE_GETSIGINFO: %s\n".as_ptr(),
                    strerror(errno),
                );
                goto_error(child);
                return ret;
            }
        } else if sig == SIGSTOP && si.si_code == SI_TKILL && si.si_pid() == pid {
            break;
        }

        if ptrace(
            PTRACE_CONT,
            pid,
            ptr::null_mut::<c_void>(),
            sig as *mut c_void,
        ) != 0
        {
            if errno == libc::ESRCH {
                return ret;
            }

            ksft_test_result_fail(c"PTRACE_CONT: %s\n".as_ptr(), strerror(errno));
            goto_error(child);
            return ret;
        }
    }

    ksft_print_msg(c"Parent is %d, child is %d\n".as_ptr(), getpid(), child);

    test_tpidr(child);
    test_hw_debug(child, NT_ARM_HW_WATCH, c"NT_ARM_HW_WATCH".as_ptr());
    test_hw_debug(child, NT_ARM_HW_BREAK, c"NT_ARM_HW_BREAK".as_ptr());

    ret = EXIT_SUCCESS;

    goto_error(child);

    ret
}

unsafe fn goto_error(child: pid_t) {
    kill(child, SIGKILL);
}

fn main() {
    let mut ret = EXIT_SUCCESS;
    let child: pid_t;

    unsafe {
        srandom(getpid() as c_uint);

        ksft_print_header();

        ksft_set_plan(EXPECTED_TESTS);

        child = fork();
        if child == 0 {
            std::process::exit(do_child());
        }

        if do_parent(child) != 0 {
            ret = EXIT_FAILURE;
        }

        ksft_print_cnts();
    }

    std::process::exit(ret);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
