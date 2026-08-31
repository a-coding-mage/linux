// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2019, Michael Ellerman, IBM Corp.
//
// Test that out-of-bounds reads/writes behave as expected.

// C dependencies: setjmp.h, stdbool.h, stdio.h, stdlib.h, string.h,
// sys/types.h, sys/wait.h, unistd.h, and "utils.h".

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

type bool_ = bool;

// Old distros (Ubuntu 16.04 at least) don't define this
const SEGV_BNDERR: c_int = 3;

// 64-bit kernel is always here
const PAGE_OFFSET: c_ulong = 0xcu64 << 60;

const SEGV_MAPERR: c_int = 1;
const SA_SIGINFO: c_int = 4;
const SIGSEGV: c_int = 11;
const _SC_PAGESIZE: c_int = 30;

#[repr(C)]
pub struct siginfo_t {
    pub si_signo: c_int,
    pub si_errno: c_int,
    pub si_code: c_int,
    pub si_addr: *mut c_void,
}

#[repr(C)]
pub struct sigaction {
    pub sa_sigaction: extern "C" fn(c_int, *mut siginfo_t, *mut c_void),
    pub sa_flags: c_ulong,
}

// Opaque storage for the platform jmp_buf. The exact layout is provided by C.
type jmp_buf = [c_long; 64];

static mut kernel_virt_end: c_ulong = 0;

static mut fault_code: c_int = 0;
static mut fault_addr: c_ulong = 0;
static mut setjmp_env: jmp_buf = [0; 64];

unsafe extern "C" {
    fn siglongjmp(env: *mut jmp_buf, val: c_int) -> !;
    fn sigsetjmp(env: *mut jmp_buf, savesigs: c_int) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn sysconf(name: c_int) -> c_long;
    fn printf(format: *const c_char, ...) -> c_int;

    fn using_hash_mmu(hash_mmu: *mut bool_) -> c_int;
    fn test_harness_set_timeout(timeout: c_int);
    fn test_harness(test: extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

macro_rules! FAIL_IF {
    ($cond:expr) => {
        if $cond {
            return 1;
        }
    };
}

extern "C" fn segv_handler(_n: c_int, info: *mut siginfo_t, _ctxt_v: *mut c_void) {
    unsafe {
        fault_code = (*info).si_code;
        fault_addr = (*info).si_addr as c_ulong;
        siglongjmp(&mut setjmp_env, 1);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bad_access(p: *mut c_char, write: bool_) -> c_int {
    let mut x: c_char = 0;

    fault_code = 0;
    fault_addr = 0;

    if sigsetjmp(&mut setjmp_env, 1) == 0 {
        if write {
            *p = 1;
        } else {
            x = *p;
        }

        printf(c"Bad - no SEGV! (%c)\n".as_ptr(), x as c_int);
        return 1;
    }

    // If we see MAPERR that means we took a page fault rather than an SLB
    // miss. We only expect to take page faults for addresses within the
    // valid kernel range.
    FAIL_IF!(
        fault_code == SEGV_MAPERR && (fault_addr < PAGE_OFFSET || fault_addr >= kernel_virt_end)
    );

    FAIL_IF!(fault_code != SEGV_MAPERR && fault_code != SEGV_BNDERR);

    0
}

extern "C" fn test() -> c_int {
    unsafe {
        let mut i: c_ulong;
        let mut j: c_ulong;
        let mut addr: c_ulong;
        let mut region_shift: c_ulong = 0;
        let mut page_shift: c_ulong;
        let page_size: c_ulong;
        let sig: sigaction;
        let mut hash_mmu: bool_ = false;

        sig = sigaction {
            sa_sigaction: segv_handler,
            sa_flags: SA_SIGINFO as c_ulong,
        };

        FAIL_IF!(sigaction(SIGSEGV, &sig, core::ptr::null_mut()) != 0);

        FAIL_IF!(using_hash_mmu(&mut hash_mmu));

        page_size = sysconf(_SC_PAGESIZE) as c_ulong;
        if page_size == (64 * 1024) {
            page_shift = 16;
        } else {
            page_shift = 12;
        }

        if page_size == (64 * 1024) || !hash_mmu {
            region_shift = 52;

            // We have 7 512T regions (4 kernel linear, vmalloc, io, vmemmap)
            kernel_virt_end = PAGE_OFFSET + (7 * (512u64 << 40));
        } else if page_size == (4 * 1024) && hash_mmu {
            region_shift = 46;

            // We have 7 64T regions (4 kernel linear, vmalloc, io, vmemmap)
            kernel_virt_end = PAGE_OFFSET + (7 * (64u64 << 40));
        } else {
            FAIL_IF!(true);
        }

        printf(
            c"Using %s MMU, PAGE_SIZE = %dKB start address 0x%016lx\n".as_ptr(),
            if hash_mmu {
                c"hash".as_ptr()
            } else {
                c"radix".as_ptr()
            },
            ((1 << page_shift) >> 10) as c_int,
            1u64 << region_shift,
        );

        // This generates access patterns like:
        //   0x0010000000000000
        //   0x0010000000010000
        //   0x0010000000020000
        //   ...
        //   0x0014000000000000
        //   0x0018000000000000
        //   0x0020000000000000
        //   0x0020000000010000
        //   0x0020000000020000
        //   ...
        //   0xf400000000000000
        //   0xf800000000000000

        i = 1;
        while i <= ((0xfu64 << 60) >> region_shift) {
            j = page_shift - 1;
            while j < 60 {
                let base: c_ulong;
                let delta: c_ulong;

                base = i << region_shift;
                delta = 1u64 << j;

                if delta >= base {
                    break;
                }

                addr = (base | delta) & !((1 << page_shift) - 1);

                FAIL_IF!(bad_access(addr as *mut c_char, false));
                FAIL_IF!(bad_access(addr as *mut c_char, true));

                j += 1;
            }
            i += 1;
        }

        0
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> c_int {
    unsafe {
        test_harness_set_timeout(300);
        test_harness(test, c"bad_accesses".as_ptr())
    }
}
