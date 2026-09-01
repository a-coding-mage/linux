// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2022, Michael Ellerman, IBM Corp.
//
// Test that the 4PB address space SLB handling doesn't corrupt userspace registers
// (r9-r13) due to a SLB fault while saving the PPR.
//
// The bug was introduced in f384796c4 ("powerpc/mm: Add support for handling > 512TB
// address in SLB miss") and fixed in 4c2de74cc869 ("powerpc/64: Interrupts save PPR on
// stack rather than thread_struct").
//
// To hit the bug requires the task struct and kernel stack to be in different segments.
// Usually that requires more than 1TB of RAM, or if that's not practical, boot the kernel
// with "disable_1tb_segments".
//
// The test works by creating mappings above 512TB, to trigger the large address space
// support. It creates 64 mappings, double the size of the SLB, to cause SLB faults on
// each access (assuming naive replacement). It then loops over those mappings touching
// each, and checks that r9-r13 aren't corrupted.
//
// It then forks another child and tries again, because a new child process will get a new
// kernel stack and thread struct allocated, which may be more optimally placed to trigger
// the bug. It would probably be better to leave the previous child processes hanging
// around, so that kernel stack & thread struct allocations are not reused, but that would
// amount to a 30 second fork bomb. The current design reliably triggers the bug on
// unpatched kernels.

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::ptr;

type pid_t = c_int;
type size_t = usize;
type sig_atomic_t = c_int;

const MAP_PRIVATE: c_int = 0x02;
const MAP_FIXED: c_int = 0x10;
const MAP_ANONYMOUS: c_int = 0x20;
const MAP_FIXED_NOREPLACE: c_int = 0x100000;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const SA_RESTART: c_int = 0x10000000;
const SIGALRM: c_int = 14;
const _SC_PAGESIZE: c_int = 30;

const BASE_ADDRESS: c_ulong = 1u64 << 50; // 1PB
const STRIDE: c_ulong = 2u64 << 40; // 2TB
const SLB_SIZE: c_int = 32;
const NR_MAPPINGS: c_int = SLB_SIZE * 2;

const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

static mut signaled: sig_atomic_t = 0;

#[repr(C)]
struct sigaction {
    sa_handler: extern "C" fn(c_int),
    sa_flags: c_ulong,
    sa_restorer: *mut c_void,
    sa_mask: c_ulong,
}

unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn sysconf(name: c_int) -> c_long;
    fn alarm(seconds: c_uint) -> c_uint;
    fn fork() -> pid_t;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn exit(status: c_int) -> !;
    fn _exit(status: c_int) -> !;

    fn using_hash_mmu(hash_mmu: *mut bool) -> c_int;
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

macro_rules! str_c {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! FAIL_IF {
    ($cond:expr) => {
        if $cond {
            return 1;
        }
    };
}

macro_rules! SKIP_IF {
    ($cond:expr) => {
        if $cond {
            return 4;
        }
    };
}

macro_rules! WIFEXITED {
    ($status:expr) => {
        (($status) & 0x7f) == 0
    };
}

macro_rules! WEXITSTATUS {
    ($status:expr) => {
        (($status) & 0xff00) >> 8
    };
}

macro_rules! WIFSIGNALED {
    ($status:expr) => {
        (((($status) & 0x7f) + 1) >> 1) > 0
    };
}

macro_rules! CHECK_REG {
    ($reg:ident, $orig:ident, $name:literal) => {
        if $reg != $orig {
            printf(
                str_c!(concat!($name, " corrupted! Expected 0x%lx != 0x%lx\n")),
                $orig,
                $reg,
            );
            _exit(1);
        }
    };
}

extern "C" fn signal_handler(_sig: c_int) {
    unsafe {
        signaled = 1;
    }
}

unsafe extern "C" fn touch_mappings() -> c_int {
    let mut r9_orig: c_ulong;
    let mut r10_orig: c_ulong;
    let mut r11_orig: c_ulong;
    let mut r12_orig: c_ulong;
    let mut r13_orig: c_ulong;
    let mut r9: c_ulong;
    let mut r10: c_ulong;
    let mut r11: c_ulong;
    let mut r12: c_ulong;
    let mut r13: c_ulong;
    let mut addr: c_ulong;
    let mut p: *mut c_ulong;
    let mut i: c_int;

    i = 0;
    while i < NR_MAPPINGS {
        addr = BASE_ADDRESS.wrapping_add((i as c_ulong).wrapping_mul(STRIDE));
        p = addr as *mut c_ulong;

        asm!(
            "mr   {0}, %r9",
            "mr   {1}, %r10",
            "mr   {2}, %r11",
            "mr   {3}, %r12",
            "mr   {4}, %r13",
            "std {10}, 0({11})",
            "mr   {5}, %r9",
            "mr   {6}, %r10",
            "mr   {7}, %r11",
            "mr   {8}, %r12",
            "mr   {9}, %r13",
            "mr   %r9,  {0}",
            "mr   %r10, {1}",
            "mr   %r11, {2}",
            "mr   %r12, {3}",
            "mr   %r13, {4}",
            out(reg) r9_orig,
            out(reg) r10_orig,
            out(reg) r11_orig,
            out(reg) r12_orig,
            out(reg) r13_orig,
            out(reg) r9,
            out(reg) r10,
            out(reg) r11,
            out(reg) r12,
            out(reg) r13,
            in(reg) i as c_ulong,
            in(reg) p,
            out("r9") _,
            out("r10") _,
            out("r11") _,
            out("r12") _,
            out("r13") _,
        );

        CHECK_REG!(r9, r9_orig, "r9");
        CHECK_REG!(r10, r10_orig, "r10");
        CHECK_REG!(r11, r11_orig, "r11");
        CHECK_REG!(r12, r12_orig, "r12");
        CHECK_REG!(r13, r13_orig, "r13");

        i += 1;
    }

    0
}

unsafe extern "C" fn test() -> c_int {
    let mut page_size: c_ulong;
    let mut addr: c_ulong;
    let mut p: *mut c_ulong;
    let mut action: sigaction = core::mem::zeroed();
    let mut hash_mmu: bool = false;
    let mut i: c_int;
    let mut status: c_int = 0;
    let mut pid: pid_t;

    // This tests a hash MMU specific bug.
    FAIL_IF!(using_hash_mmu(&mut hash_mmu) != 0);
    SKIP_IF!(!hash_mmu);
    // 4K kernels don't support 4PB address space
    SKIP_IF!(sysconf(_SC_PAGESIZE) < 65536);

    page_size = sysconf(_SC_PAGESIZE) as c_ulong;

    i = 0;
    while i < NR_MAPPINGS {
        addr = BASE_ADDRESS.wrapping_add((i as c_ulong).wrapping_mul(STRIDE));

        p = mmap(
            addr as *mut c_void,
            page_size as size_t,
            PROT_READ | PROT_WRITE,
            MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED_NOREPLACE,
            -1,
            0,
        ) as *mut c_ulong;
        if p as *mut c_void == MAP_FAILED {
            perror(str_c!("mmap"));
            printf(str_c!(
                "Error: couldn't mmap(), confirm kernel has 4PB support?\n"
            ));
            return 1;
        }

        i += 1;
    }

    action.sa_handler = signal_handler;
    action.sa_flags = SA_RESTART as c_ulong;
    FAIL_IF!(sigaction(SIGALRM, &action, ptr::null_mut()) < 0);

    // Seen to always crash in under ~10s on affected kernels.
    alarm(30);

    while signaled == 0 {
        // Fork new processes, to increase the chance that we hit the case where
        // the kernel stack and task struct are in different segments.
        pid = fork();
        if pid == 0 {
            exit(touch_mappings());
        }

        FAIL_IF!(waitpid(-1, &mut status, 0) == -1);
        FAIL_IF!(WIFSIGNALED!(status));
        FAIL_IF!(!WIFEXITED!(status));
        FAIL_IF!(WEXITSTATUS!(status) != 0);
    }

    0
}

fn main() {
    unsafe {
        core::process::exit(test_harness(
            test,
            str_c!("large_vm_gpr_corruption"),
        ));
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
