// SPDX-License-Identifier: GPL-2.0-only
/*
 * sysret_ss_attrs.c - test that syscalls return valid hidden SS attributes
 * Copyright (c) 2015 Andrew Lutomirski
 *
 * On AMD CPUs, SYSRET can return with a valid SS descriptor with with
 * the hidden attributes set to an unusable state.  Make sure the kernel
 * doesn't let this happen.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::{c_char, c_int, c_ulong, c_void};
use std::ptr;

type size_t = usize;
type useconds_t = u32;
type pthread_t = c_ulong;

const CPU_SETSIZE: usize = 1024;
const __NCPUBITS: usize = 8 * std::mem::size_of::<c_ulong>();

const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;

#[cfg(target_arch = "x86_64")]
const MAP_32BIT: c_int = 0x40;

#[repr(C)]
struct cpu_set_t {
    __bits: [c_ulong; CPU_SETSIZE / __NCPUBITS],
}

unsafe extern "C" {
    fn sched_setaffinity(pid: c_int, cpusetsize: size_t, mask: *const cpu_set_t) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn usleep(usec: useconds_t) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn err(eval: c_int, fmt: *const c_char, ...) -> !;
}

#[cfg(target_arch = "x86_64")]
unsafe extern "C" {
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: isize,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn call32_from_64(stack: *mut c_void, function: unsafe extern "C" fn()) -> c_ulong;
    fn test_ss();
}

#[cfg(target_arch = "x86_64")]
std::arch::global_asm!(
    ".pushsection .text",
    ".code32",
    "test_ss:",
    "pushl $0",
    "popl %eax",
    "ret",
    ".code64",
);

fn CPU_ZERO(set: &mut cpu_set_t) {
    for slot in &mut set.__bits {
        *slot = 0;
    }
}

fn CPU_SET(cpu: usize, set: &mut cpu_set_t) {
    set.__bits[cpu / __NCPUBITS] |= 1u64.wrapping_shl((cpu % __NCPUBITS) as u32) as c_ulong;
}

unsafe extern "C" fn threadproc(_ctx: *mut c_void) -> *mut c_void {
    /*
     * Do our best to cause sleeps on this CPU to exit the kernel and
     * re-enter with SS = 0.
     */
    loop {}

    #[allow(unreachable_code)]
    ptr::null_mut()
}

fn main() {
    unsafe {
        /*
         * Start a busy-looping thread on the same CPU we're on.
         * For simplicity, just stick everything to CPU 0.  This will
         * fail in some containers, but that's probably okay.
         */
        let mut cpuset = cpu_set_t {
            __bits: [0; CPU_SETSIZE / __NCPUBITS],
        };
        CPU_ZERO(&mut cpuset);
        CPU_SET(0, &mut cpuset);
        if sched_setaffinity(0, std::mem::size_of_val(&cpuset), &cpuset) != 0 {
            printf(c"[WARN]\tsched_setaffinity failed\n".as_ptr());
        }

        let mut thread: pthread_t = 0;
        if pthread_create(&mut thread, ptr::null(), threadproc, ptr::null_mut()) != 0 {
            err(1, c"pthread_create".as_ptr());
        }

        #[cfg(target_arch = "x86_64")]
        let stack32 = {
            let stack32 = mmap(
                ptr::null_mut(),
                4096,
                PROT_READ | PROT_WRITE,
                MAP_32BIT | MAP_ANONYMOUS | MAP_PRIVATE,
                -1,
                0,
            ) as *mut u8;
            if stack32 == (-1isize) as *mut u8 {
                err(1, c"mmap".as_ptr());
            }
            stack32
        };

        printf(c"[RUN]\tSyscalls followed by SS validation\n".as_ptr());

        for _i in 0..1000 {
            /*
             * Go to sleep and return using sysret (if we're 64-bit
             * or we're 32-bit on AMD on a 64-bit kernel).  On AMD CPUs,
             * SYSRET doesn't fix up the cached SS descriptor, so the
             * kernel needs some kind of workaround to make sure that we
             * end the system call with a valid stack segment.  This
             * can be a confusing failure because the SS *selector*
             * is the same regardless.
             */
            usleep(2);

            #[cfg(target_arch = "x86_64")]
            {
                /*
                 * On 32-bit, just doing a syscall through glibc is enough
                 * to cause a crash if our cached SS descriptor is invalid.
                 * On 64-bit, it's not, so try extra hard.
                 */
                call32_from_64(stack32.add(4088) as *mut c_void, test_ss);
            }
        }

        printf(c"[OK]\tWe survived\n".as_ptr());

        #[cfg(target_arch = "x86_64")]
        {
            munmap(stack32 as *mut c_void, 4096);
        }
    }
}
