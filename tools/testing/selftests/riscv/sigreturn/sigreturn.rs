// SPDX-License-Identifier: GPL-2.0-only
//
// C dependencies removed from executable Rust:
// <signal.h>, <stdio.h>, <stdlib.h>, <ucontext.h>, <linux/ptrace.h>,
// and "kselftest_harness.h" provide the C ABI types, constants, and test
// harness macros used below.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::arch::asm;
use core::ffi::{c_char, c_int, c_uint, c_void};

const RISCV_V_MAGIC: c_uint = 0x53465457;
const DEFAULT_VALUE: c_int = 2;
const SIGNAL_HANDLER_OVERRIDE: c_int = 3;

// External C/header-provided items.
extern "C" {
    static mut stderr: *mut FILE;

    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn abort() -> !;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
}

// Header-provided constants from <signal.h> and <linux/ptrace.h>.
// These declarations preserve the dependency names used by the C source.
extern "C" {
    static SIGSEGV: c_int;
    static SA_SIGINFO: c_int;
    static REG_PC: usize;
}

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

type sighandler_t = unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void);

#[repr(C)]
struct sigaction {
    sa_sigaction: Option<sighandler_t>,
    sa_flags: c_int,
}

#[repr(C)]
struct siginfo_t {
    _private: [u8; 0],
}

#[repr(C)]
struct ucontext_t {
    uc_mcontext: mcontext_t,
}

#[repr(C)]
struct mcontext_t {
    __gregs: [usize; 32],
    __fpregs: __riscv_mc_fp_state,
}

#[repr(C)]
struct __riscv_mc_fp_state {
    _private: [u8; 0],
}

#[repr(C)]
struct __riscv_extra_ext_header {
    hdr: __riscv_ctx_hdr,
}

#[repr(C)]
struct __riscv_ctx_hdr {
    magic: c_uint,
}

#[repr(C)]
struct __riscv_v_ext_state {
    datap: *mut c_void,
}

unsafe extern "C" fn simple_handle(
    _sig_no: c_int,
    _info: *mut siginfo_t,
    vcontext: *mut c_void,
) {
    let context = vcontext as *mut ucontext_t;

    (*context).uc_mcontext.__gregs[REG_PC] =
        (*context).uc_mcontext.__gregs[REG_PC].wrapping_add(4);
}

unsafe extern "C" fn vector_override(
    _sig_no: c_int,
    _info: *mut siginfo_t,
    vcontext: *mut c_void,
) {
    let context = vcontext as *mut ucontext_t;

    // vector state
    let mut ext: *mut __riscv_extra_ext_header;
    let mut v_ext_state: *mut __riscv_v_ext_state;

    /* Find the vector context. */
    ext = &mut (*context).uc_mcontext.__fpregs as *mut __riscv_mc_fp_state
        as *mut c_void as *mut __riscv_extra_ext_header;
    if (*ext).hdr.magic != RISCV_V_MAGIC {
        fprintf(
            stderr,
            b"bad vector magic: %x\n\0".as_ptr() as *const c_char,
            (*ext).hdr.magic,
        );
        abort();
    }

    v_ext_state = (ext as *mut c_char).add(core::mem::size_of::<__riscv_extra_ext_header>())
        as *mut c_void as *mut __riscv_v_ext_state;

    *((*v_ext_state).datap as *mut c_int) = SIGNAL_HANDLER_OVERRIDE;

    (*context).uc_mcontext.__gregs[REG_PC] =
        (*context).uc_mcontext.__gregs[REG_PC].wrapping_add(4);
}

unsafe fn vector_sigreturn(data: c_int, handler: sighandler_t) -> c_int {
    let mut after_sigreturn: c_int;
    let sig_action = sigaction {
        sa_sigaction: Some(handler),
        sa_flags: SA_SIGINFO,
    };

    sigaction(SIGSEGV, &sig_action, core::ptr::null_mut());

    asm!(
        ".option push",
        ".option arch, +v",
        "vsetivli x0, 1, e32, m1, ta, ma",
        "vmv.s.x v0, {data_reg}",
        "# Generate SIGSEGV",
        "lw a0, 0(x0)",
        "vmv.x.s {after_reg}, v0",
        ".option pop",
        after_reg = lateout(reg) after_sigreturn,
        data_reg = in(reg) data,
    );

    after_sigreturn
}

// TEST(vector_restore)
unsafe fn vector_restore() {
    let result: c_int;

    result = vector_sigreturn(DEFAULT_VALUE, simple_handle);

    EXPECT_EQ(DEFAULT_VALUE, result);
}

// TEST(vector_restore_signal_handler_override)
unsafe fn vector_restore_signal_handler_override() {
    let result: c_int;

    result = vector_sigreturn(DEFAULT_VALUE, vector_override);

    EXPECT_EQ(SIGNAL_HANDLER_OVERRIDE, result);
}

// kselftest_harness.h provides EXPECT_EQ and TEST_HARNESS_MAIN.
extern "C" {
    fn EXPECT_EQ(expected: c_int, actual: c_int);
}

// TEST_HARNESS_MAIN
