// SPDX-License-Identifier: GPL-2.0-only
/*
 * unwind_vdso.c - tests unwind info for AT_SYSINFO in the vDSO
 * Copyright (c) 2014-2015 Andrew Lutomirski
 *
 * This tests __kernel_vsyscall's unwind info.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

/* C includes translated as external dependencies:
 * <features.h>, <stdio.h>, "helpers.h", <sys/time.h>, <stdlib.h>,
 * <syscall.h>, <unistd.h>, <string.h>, <inttypes.h>, <sys/mman.h>,
 * <signal.h>, <sys/ucontext.h>, <err.h>, <stddef.h>, <stdbool.h>,
 * <sys/ptrace.h>, <sys/user.h>, <link.h>, <sys/auxv.h>, <dlfcn.h>,
 * <unwind.h>
 */

/* Original C has:
 * #if defined(__GLIBC__) && __GLIBC__ == 2 && __GLIBC_MINOR__ < 16
 * compiling a skip-only main because getauxval() is unavailable.
 * Rust has no direct file-local equivalent for that libc preprocessor test.
 */

type sig_atomic_t = c_int;
type greg_t = c_long;
type _Unwind_Word = c_ulong;
type _Unwind_Reason_Code = c_int;

const _URC_NO_REASON: _Unwind_Reason_Code = 0;
const _URC_NORMAL_STOP: _Unwind_Reason_Code = 5;

const SYS_getpid: c_long = 20;
const AT_SYSINFO: c_ulong = 32;
const SIGTRAP: c_int = 5;
const REG_EIP: usize = 14;
const REG_ESP: usize = 7;
const REG_EFL: usize = 16;
const X86_EFLAGS_TF: greg_t = 0x00000100;

#[repr(C)]
struct siginfo_t {
    _private: [u8; 0],
}

#[repr(C)]
struct mcontext_t {
    gregs: [greg_t; 19],
}

#[repr(C)]
struct ucontext_t {
    uc_flags: c_ulong,
    uc_link: *mut ucontext_t,
    uc_stack: stack_t,
    uc_mcontext: mcontext_t,
}

#[repr(C)]
struct stack_t {
    ss_sp: *mut c_void,
    ss_flags: c_int,
    ss_size: usize,
}

#[repr(C)]
struct Dl_info {
    dli_fname: *const c_char,
    dli_fbase: *mut c_void,
    dli_sname: *const c_char,
    dli_saddr: *mut c_void,
}

#[repr(C)]
struct _Unwind_Context {
    _private: [u8; 0],
}

struct unwind_state {
    ip: c_ulong, /* trap source */
    depth: c_int, /* -1 until we hit the trap source */
}

static mut nerrs: sig_atomic_t = 0;
static mut sysinfo: c_ulong = 0;
static mut got_sysinfo: bool = false;
static mut return_address: c_ulong = 0;

unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn getpid() -> c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn getauxval(type_: c_ulong) -> c_ulong;
    fn dladdr(addr: *const c_void, info: *mut Dl_info) -> c_int;

    fn sethandler(sig: c_int, handler: extern "C" fn(c_int, *mut siginfo_t, *mut c_void), flags: c_int);
    fn get_eflags() -> c_ulong;
    fn set_eflags(eflags: c_ulong);

    fn _Unwind_GetIP(context: *mut _Unwind_Context) -> _Unwind_Word;
    fn _Unwind_GetGR(context: *mut _Unwind_Context, index: c_int) -> _Unwind_Word;
    fn _Unwind_Backtrace(
        trace: extern "C" fn(*mut _Unwind_Context, *mut c_void) -> _Unwind_Reason_Code,
        trace_argument: *mut c_void,
    ) -> _Unwind_Reason_Code;
}

extern "C" fn trace_fn(ctx: *mut _Unwind_Context, opaque: *mut c_void) -> _Unwind_Reason_Code {
    unsafe {
        let state = opaque as *mut unwind_state;
        let ip = _Unwind_GetIP(ctx) as c_ulong;

        if (*state).depth == -1 {
            if ip == (*state).ip {
                (*state).depth = 0;
            } else {
                return _URC_NO_REASON; /* Not there yet */
            }
        }
        printf(c"\t  0x%lx\n".as_ptr(), ip);

        if ip == return_address {
            /* Here we are. */
            let eax = _Unwind_GetGR(ctx, 0) as c_ulong;
            let ecx = _Unwind_GetGR(ctx, 1) as c_ulong;
            let edx = _Unwind_GetGR(ctx, 2) as c_ulong;
            let ebx = _Unwind_GetGR(ctx, 3) as c_ulong;
            let ebp = _Unwind_GetGR(ctx, 5) as c_ulong;
            let esi = _Unwind_GetGR(ctx, 6) as c_ulong;
            let edi = _Unwind_GetGR(ctx, 7) as c_ulong;
            let ok = (eax == SYS_getpid as c_ulong || eax == getpid() as c_ulong)
                && ebx == 1
                && ecx == 2
                && edx == 3
                && esi == 4
                && edi == 5
                && ebp == 6;

            if !ok {
                nerrs += 1;
            }
            printf(
                c"[%s]\t  NR = %ld, args = %ld, %ld, %ld, %ld, %ld, %ld\n".as_ptr(),
                if ok { c"OK".as_ptr() } else { c"FAIL".as_ptr() },
                eax,
                ebx,
                ecx,
                edx,
                esi,
                edi,
                ebp,
            );

            _URC_NORMAL_STOP
        } else {
            (*state).depth += 1;
            _URC_NO_REASON
        }
    }
}

extern "C" fn sigtrap(_sig: c_int, _info: *mut siginfo_t, ctx_void: *mut c_void) {
    unsafe {
        let ctx = ctx_void as *mut ucontext_t;
        let mut state: unwind_state;
        let ip = (*ctx).uc_mcontext.gregs[REG_EIP] as c_ulong;

        if !got_sysinfo && ip == sysinfo {
            got_sysinfo = true;

            /* Find the return address. */
            return_address =
                *((*ctx).uc_mcontext.gregs[REG_ESP] as c_ulong as *mut c_ulong);

            printf(
                c"\tIn vsyscall at 0x%lx, returning to 0x%lx\n".as_ptr(),
                ip,
                return_address,
            );
        }

        if !got_sysinfo {
            return; /* Not there yet */
        }

        if ip == return_address {
            (*ctx).uc_mcontext.gregs[REG_EFL] &= !X86_EFLAGS_TF;
            printf(c"\tVsyscall is done\n".as_ptr());
            return;
        }

        printf(c"\tSIGTRAP at 0x%lx\n".as_ptr(), ip);

        state = unwind_state { ip, depth: -1 };
        _Unwind_Backtrace(trace_fn, &mut state as *mut unwind_state as *mut c_void);
    }
}

fn main() {
    unsafe {
        sysinfo = getauxval(AT_SYSINFO);
        printf(c"\tAT_SYSINFO is 0x%lx\n".as_ptr(), sysinfo);

        let mut info: Dl_info = core::mem::zeroed();
        if dladdr(sysinfo as *mut c_void, &mut info) == 0 {
            printf(c"[WARN]\tdladdr failed on AT_SYSINFO\n".as_ptr());
        } else {
            printf(
                c"[OK]\tAT_SYSINFO maps to %s, loaded at 0x%p\n".as_ptr(),
                info.dli_fname,
                info.dli_fbase,
            );
        }

        sethandler(SIGTRAP, sigtrap, 0);

        syscall(SYS_getpid); /* Force symbol binding without TF set. */
        printf(c"[RUN]\tSet TF and check a fast syscall\n".as_ptr());
        set_eflags(get_eflags() | X86_EFLAGS_TF as c_ulong);
        syscall(SYS_getpid, 1, 2, 3, 4, 5, 6);
        if !got_sysinfo {
            set_eflags(get_eflags() & !(X86_EFLAGS_TF as c_ulong));

            /*
             * The most likely cause of this is that you're on Debian or
             * a Debian-based distro, you're missing libc6-i686, and you're
             * affected by libc/19006 (https://sourceware.org/PR19006).
             */
            printf(c"[WARN]\tsyscall(2) didn't enter AT_SYSINFO\n".as_ptr());
        }

        if get_eflags() & X86_EFLAGS_TF as c_ulong != 0 {
            printf(c"[FAIL]\tTF is still set\n".as_ptr());
            nerrs += 1;
        }

        if nerrs != 0 {
            printf(c"[FAIL]\tThere were errors\n".as_ptr());
            std::process::exit(1);
        } else {
            printf(c"[OK]\tAll is well\n".as_ptr());
            std::process::exit(0);
        }
    }
}
