// SPDX-License-Identifier: GPL-2.0-only
//
// Translated from C. Header dependencies in the original:
// <stdio.h>, <stdlib.h>, <asm/ptrace.h>, <linux/elf.h>, <sys/ptrace.h>,
// <sys/uio.h>, <sys/wait.h>, "../../kselftest.h", "v_helpers.h"

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of;

type pid_t = c_int;
type size_t = usize;

const EXIT_FAILURE: c_int = 1;
const EXIT_SUCCESS: c_int = 0;
const SIGTRAP: c_int = 5;

const PTRACE_TRACEME: __ptrace_request = 0;
const PTRACE_CONT: __ptrace_request = 7;
const PTRACE_GETREGSET: __ptrace_request = 0x4204;
const PTRACE_SETREGSET: __ptrace_request = 0x4205;

const NT_PRSTATUS: c_long = 1;
const NT_RISCV_VECTOR: c_long = 0x900;

type __ptrace_request = c_uint;

#[repr(C)]
struct iovec {
    iov_base: *mut c_void,
    iov_len: size_t,
}

#[repr(C)]
struct __riscv_v_regset_state {
    vstart: usize,
    vl: usize,
    vtype: usize,
    vcsr: usize,
    vlenb: usize,
    datap: *mut c_void,
}

#[repr(C)]
struct user_regs_struct {
    pc: usize,
}

unsafe extern "C" {
    fn ptrace(request: __ptrace_request, pid: pid_t, addr: *mut c_void, data: *mut c_void) -> c_long;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn malloc(size: size_t) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn rand() -> c_int;
    fn srandom(seed: c_uint);
    fn getpid() -> pid_t;
    fn fork() -> pid_t;

    fn ksft_perror(msg: *const c_char);
    fn ksft_test_result(condition: bool, msg: *const c_char, ...);
    fn ksft_print_msg(msg: *const c_char, ...);
    fn ksft_set_plan(plan: c_uint);
    fn ksft_exit_skip(msg: *const c_char, ...) -> !;
    fn ksft_exit_fail_msg(msg: *const c_char, ...) -> !;
    fn ksft_finished() -> c_int;

    fn is_vector_supported() -> bool;
    fn is_xtheadvector_supported() -> bool;
}

static mut parent_set_val: c_int = 0;
static mut child_set_val: c_int = 0;

fn wifexited(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn wexitstatus(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

fn wifstopped(status: c_int) -> bool {
    (status & 0xff) == 0x7f
}

fn wstopsig(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn do_ptrace(
    op: __ptrace_request,
    pid: pid_t,
    type_: c_long,
    size: size_t,
    data: *mut c_void,
) -> c_long {
    let mut v_iovec = iovec {
        iov_len: size,
        iov_base: data,
    };

    unsafe {
        ptrace(
            op,
            pid,
            type_ as *mut c_void,
            &mut v_iovec as *mut iovec as *mut c_void,
        )
    }
}

unsafe fn do_child() -> c_int {
    let out: c_int;

    if unsafe {
        ptrace(
            PTRACE_TRACEME,
            -1,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        )
    } != 0
    {
        unsafe {
            ksft_perror(c"PTRACE_TRACEME failed\n".as_ptr());
        }
        return EXIT_FAILURE;
    }

    unsafe {
        asm!(
            ".option push",
            ".option arch, +v",
            ".option norvc",
            "vsetivli x0, 1, e32, m1, ta, ma",
            "vmv.s.x v31, {in_reg}",
            "ebreak",
            "vmv.x.s {out_reg}, v31",
            ".option pop",
            out_reg = lateout(reg) out,
            in_reg = in(reg) child_set_val,
            options(nostack)
        );
    }

    if unsafe { out != parent_set_val } {
        return EXIT_FAILURE;
    }

    EXIT_SUCCESS
}

unsafe fn do_parent(child: pid_t) {
    let mut status: c_int = 0;
    let mut data: *mut c_void = core::ptr::null_mut();

    /* Attach to the child */
    while unsafe { waitpid(child, &mut status, 0) } != 0 {
        if wifexited(status) {
            unsafe {
                ksft_test_result(wexitstatus(status) == 0, c"SETREGSET vector\n".as_ptr());
            }
            break;
        } else if wifstopped(status) && (wstopsig(status) == SIGTRAP) {
            let mut size: size_t;
            let mut v31: *mut c_void;
            let mut v_regset_hdr: *mut __riscv_v_regset_state;
            let gpreg: *mut user_regs_struct;

            size = size_of::<__riscv_v_regset_state>();
            data = unsafe { malloc(size) };
            if data.is_null() {
                break;
            }
            v_regset_hdr = data as *mut __riscv_v_regset_state;

            if unsafe { do_ptrace(PTRACE_GETREGSET, child, NT_RISCV_VECTOR, size, data) } != 0 {
                break;
            }

            unsafe {
                ksft_print_msg(c"vlenb %ld\n".as_ptr(), (*v_regset_hdr).vlenb as c_long);
            }
            data = unsafe { realloc(data, size + (*v_regset_hdr).vlenb * 32) };
            if data.is_null() {
                break;
            }
            v_regset_hdr = data as *mut __riscv_v_regset_state;
            v31 = unsafe { (data as *mut u8).add(size + (*v_regset_hdr).vlenb * 31) as *mut c_void };
            size += unsafe { (*v_regset_hdr).vlenb * 32 };

            if unsafe { do_ptrace(PTRACE_GETREGSET, child, NT_RISCV_VECTOR, size, data) } != 0 {
                break;
            }

            unsafe {
                ksft_test_result(
                    *(v31 as *mut c_int) == child_set_val,
                    c"GETREGSET vector\n".as_ptr(),
                );
            }

            unsafe {
                *(v31 as *mut c_int) = parent_set_val;
            }
            if unsafe { do_ptrace(PTRACE_SETREGSET, child, NT_RISCV_VECTOR, size, data) } != 0 {
                break;
            }

            /* move the pc forward */
            size = size_of::<user_regs_struct>();
            data = unsafe { realloc(data, size) };
            gpreg = data as *mut user_regs_struct;

            if unsafe { do_ptrace(PTRACE_GETREGSET, child, NT_PRSTATUS, size, data) } != 0 {
                break;
            }

            unsafe {
                (*gpreg).pc += 4;
            }
            if unsafe { do_ptrace(PTRACE_SETREGSET, child, NT_PRSTATUS, size, data) } != 0 {
                break;
            }
        }

        unsafe {
            ptrace(
                PTRACE_CONT,
                child,
                core::ptr::null_mut(),
                core::ptr::null_mut(),
            );
        }
    }

    unsafe {
        free(data);
    }
}

fn main() -> c_int {
    let child: pid_t;

    unsafe {
        ksft_set_plan(2);
        if !is_vector_supported() && !is_xtheadvector_supported() {
            ksft_exit_skip(c"Vector not supported\n".as_ptr());
        }

        srandom(getpid() as c_uint);
        parent_set_val = rand();
        child_set_val = rand();

        child = fork();
        if child < 0 {
            ksft_exit_fail_msg(c"Fork failed %d\n".as_ptr(), child);
        }

        if child == 0 {
            return do_child();
        }

        do_parent(child);

        ksft_finished()
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
