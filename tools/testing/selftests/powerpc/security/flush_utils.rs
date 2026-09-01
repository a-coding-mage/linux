// SPDX-License-Identifier: GPL-2.0+

/*
 * Copyright 2018 IBM Corporation.
 */

// C dependency intent:
// #define __SANE_USERSPACE_TYPES__
// #include <sys/types.h>
// #include <stdint.h>
// #include <unistd.h>
// #include <signal.h>
// #include <stdlib.h>
// #include <string.h>
// #include <stdio.h>
// #include <sys/utsname.h>
// #include "reg.h"
// #include "utils.h"
// #include "flush_utils.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::arch::asm;
use core::ffi::{c_char, c_int, c_ulong, c_void};

type __u64 = u64;

const SA_SIGINFO: c_int = 4;
const SIGILL: c_int = 4;

#[repr(C)]
pub struct utsname {
    pub sysname: [c_char; 65],
    pub nodename: [c_char; 65],
    pub release: [c_char; 65],
    pub version: [c_char; 65],
    pub machine: [c_char; 65],
    pub domainname: [c_char; 65],
}

#[repr(C)]
pub struct siginfo_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ucontext_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sigset_t {
    pub __val: [c_ulong; 16],
}

#[repr(C)]
pub struct sigaction {
    pub sa_sigaction: Option<unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>,
    pub sa_mask: sigset_t,
    pub sa_flags: c_int,
    pub sa_restorer: Option<unsafe extern "C" fn()>,
}

unsafe extern "C" {
    static CACHELINE_SIZE: c_ulong;
    static SPRN_DSCR: c_ulong;

    fn getppid() -> c_int;
    fn uname(buf: *mut utsname) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn perror(s: *const c_char);
    fn printf(format: *const c_char, ...) -> c_int;
    fn abort() -> !;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn mtspr(spr: c_ulong, val: c_ulong);

    // External Rust form of the UCONTEXT_NIA(ctx) lvalue macro from utils.h.
    fn UCONTEXT_NIA(ctx: *mut ucontext_t) -> *mut c_ulong;
}

#[inline]
unsafe fn load(addr: *mut c_void) -> __u64 {
    let tmp: __u64;

    unsafe {
        asm!("ld {0},0({1})", out(reg) tmp, in(reg) addr, options(volatile));
    }

    tmp
}

#[no_mangle]
pub unsafe extern "C" fn syscall_loop(
    p: *mut c_char,
    iterations: c_ulong,
    zero_size: c_ulong,
) {
    let mut i: c_ulong = 0;
    while i < iterations {
        let mut j: c_ulong = 0;
        while j < zero_size {
            unsafe {
                load(p.add(j as usize) as *mut c_void);
            }
            j = j.wrapping_add(CACHELINE_SIZE);
        }
        unsafe {
            getppid();
        }
        i = i.wrapping_add(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn syscall_loop_uaccess(
    p: *mut c_char,
    iterations: c_ulong,
    zero_size: c_ulong,
) {
    let mut utsname: utsname = unsafe { core::mem::zeroed() };

    let mut i: c_ulong = 0;
    while i < iterations {
        let mut j: c_ulong = 0;
        while j < zero_size {
            unsafe {
                load(p.add(j as usize) as *mut c_void);
            }
            j = j.wrapping_add(CACHELINE_SIZE);
        }
        unsafe {
            uname(&mut utsname);
        }
        i = i.wrapping_add(1);
    }
}

unsafe extern "C" fn sigill_handler(signr: c_int, info: *mut siginfo_t, unused: *mut c_void) {
    static mut WARNED: c_int = 0;
    let ctx: *mut ucontext_t = unused as *mut ucontext_t;
    let pc: *mut c_ulong = unsafe { UCONTEXT_NIA(ctx) };

    let _ = signr;
    let _ = info;

    /* mtspr 3,RS to check for move to DSCR below */
    if unsafe { (*(*pc as *mut u32) & 0xfc1fffff) == 0x7c0303a6 } {
        unsafe {
            if WARNED == 0 {
                printf(
                    c"WARNING: Skipping over dscr setup. Consider running 'ppc64_cpu --dscr=1' manually.\n"
                        .as_ptr(),
                );
            }
            WARNED = WARNED.wrapping_add(1);
            *pc = (*pc).wrapping_add(4);
        }
    } else {
        unsafe {
            printf(c"SIGILL at %p\n".as_ptr(), pc);
            abort();
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn set_dscr(val: c_ulong) {
    static mut INIT: c_int = 0;
    let mut sa: sigaction = unsafe { core::mem::zeroed() };

    unsafe {
        if INIT == 0 {
            memset(
                &mut sa as *mut sigaction as *mut c_void,
                0,
                core::mem::size_of::<sigaction>(),
            );
            sa.sa_sigaction = Some(sigill_handler);
            sa.sa_flags = SA_SIGINFO;
            if sigaction(SIGILL, &sa, core::ptr::null_mut()) != 0 {
                perror(c"sigill_handler".as_ptr());
            }
            INIT = 1;
        }

        mtspr(SPRN_DSCR, val);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
