// SPDX-License-Identifier: GPL-2.0-only
/*
 * 32-bit test to check vDSO mremap.
 *
 * Copyright (c) 2016 Dmitry Safonov
 * Suggested-by: Andrew Lutomirski
 */
/*
 * Can be built statically:
 * gcc -Os -Wall -static -m32 test_mremap_vdso.c
 */
/* C dependencies: stdio.h, errno.h, unistd.h, string.h, stdbool.h,
 * sys/mman.h, sys/auxv.h, sys/syscall.h, sys/wait.h, kselftest.h.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

type size_t = usize;
type pid_t = c_int;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut stdout: *mut FILE;

    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn mremap(
        old_address: *mut c_void,
        old_size: size_t,
        new_size: size_t,
        flags: c_int,
        new_address: *mut c_void,
    ) -> *mut c_void;

    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fflush(stream: *mut FILE) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strlen(s: *const c_char) -> size_t;

    fn fork() -> pid_t;
    fn getauxval(type_: c_ulong) -> c_ulong;
    fn syscall(number: c_long, ...) -> c_long;
    fn waitpid(pid: pid_t, wstatus: *mut c_int, options: c_int) -> pid_t;

    fn __errno_location() -> *mut c_int;

    fn ksft_print_header();
    fn ksft_set_plan(plan: c_uint);
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_test_result_skip(fmt: *const c_char, ...);
    fn ksft_test_result_fail(fmt: *const c_char, ...);
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_finished();
    fn ksft_exit_fail_msg(fmt: *const c_char, ...) -> !;
}

type c_uint = u32;

const PAGE_SIZE: c_ulong = 4096;

const PROT_NONE: c_int = 0x0;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;
const MREMAP_MAYMOVE: c_int = 1;
const MREMAP_FIXED: c_int = 2;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const AT_SYSINFO_EHDR: c_ulong = 33;
const SYS_exit: c_long = 60;
const __NR_exit: c_long = 1;

const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

unsafe fn errno() -> c_int {
    unsafe { *__errno_location() }
}

unsafe fn try_to_remap(vdso_addr: *mut c_void, size: c_ulong) -> c_int {
    let dest_addr: *mut c_void;
    let new_addr: *mut c_void;

    /* Searching for memory location where to remap */
    dest_addr = unsafe {
        mmap(
            core::ptr::null_mut(),
            size as size_t,
            PROT_NONE,
            MAP_PRIVATE | MAP_ANONYMOUS,
            -1,
            0,
        )
    };
    if dest_addr == MAP_FAILED {
        unsafe {
            ksft_print_msg(
                b"WARN: mmap failed (%d): %m\n\0".as_ptr() as *const c_char,
                errno(),
            );
        }
        return 0;
    }

    unsafe {
        ksft_print_msg(
            b"Moving vDSO: [%p, %#lx] -> [%p, %#lx]\n\0".as_ptr() as *const c_char,
            vdso_addr,
            (vdso_addr as c_ulong).wrapping_add(size),
            dest_addr,
            (dest_addr as c_ulong).wrapping_add(size),
        );
        fflush(stdout);
    }

    new_addr = unsafe {
        mremap(
            vdso_addr,
            size as size_t,
            size as size_t,
            MREMAP_FIXED | MREMAP_MAYMOVE,
            dest_addr,
        )
    };
    if new_addr as c_ulong == (-1isize) as c_ulong {
        unsafe {
            munmap(dest_addr, size as size_t);
            if errno() == EINVAL {
                ksft_print_msg(
                    b"vDSO partial move failed, will try with bigger size\n\0".as_ptr()
                        as *const c_char,
                );
                return -1; /* Retry with larger */
            }
            ksft_print_msg(
                b"[FAIL]\tmremap failed (%d): %m\n\0".as_ptr() as *const c_char,
                errno(),
            );
        }
        return 1;
    }

    return 0;
}

const VDSO_NAME: &[u8] = b"[vdso]\0";
const VMFLAGS: &[u8] = b"VmFlags:\0";
const MSEAL_FLAGS: &[u8] = b"sl\0";
const MAX_LINE_LEN: usize = 512;

unsafe fn vdso_sealed(maps: *mut FILE) -> bool {
    let mut line: [c_char; MAX_LINE_LEN] = [0; MAX_LINE_LEN];
    let mut has_vdso: bool = false;

    while unsafe { !fgets(line.as_mut_ptr(), line.len() as c_int, maps).is_null() } {
        if unsafe { !strstr(line.as_ptr(), VDSO_NAME.as_ptr() as *const c_char).is_null() } {
            has_vdso = true;
        }

        if has_vdso
            && unsafe {
                strncmp(
                    line.as_ptr(),
                    VMFLAGS.as_ptr() as *const c_char,
                    strlen(VMFLAGS.as_ptr() as *const c_char),
                ) == 0
            }
        {
            if unsafe { !strstr(line.as_ptr(), MSEAL_FLAGS.as_ptr() as *const c_char).is_null() } {
                return true;
            }

            return false;
        }
    }

    return false;
}

fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

#[no_mangle]
pub unsafe extern "C" fn main(
    _argc: c_int,
    _argv: *mut *mut c_char,
    _envp: *mut *mut c_char,
) -> c_int {
    let child: pid_t;
    let maps: *mut FILE;

    unsafe {
        ksft_print_header();
        ksft_set_plan(1);
    }

    maps = unsafe {
        fopen(
            b"/proc/self/smaps\0".as_ptr() as *const c_char,
            b"r\0".as_ptr() as *const c_char,
        )
    };
    if maps.is_null() {
        unsafe {
            ksft_test_result_skip(
                b"Could not open /proc/self/smaps, errno=%d\n\0".as_ptr() as *const c_char,
                errno(),
            );
        }

        return 0;
    }

    if unsafe { vdso_sealed(maps) } {
        unsafe {
            ksft_test_result_skip(b"vdso is sealed\n\0".as_ptr() as *const c_char);
        }
        return 0;
    }

    unsafe {
        fclose(maps);
    }

    child = unsafe { fork() };
    if child == -1 {
        unsafe {
            ksft_exit_fail_msg(
                b"failed to fork (%d): %m\n\0".as_ptr() as *const c_char,
                errno(),
            );
        }
    }

    if child == 0 {
        let mut vdso_size: c_ulong = PAGE_SIZE;
        let auxval: c_ulong;
        let mut ret: c_int = -1;

        auxval = unsafe { getauxval(AT_SYSINFO_EHDR) };
        unsafe {
            ksft_print_msg(
                b"AT_SYSINFO_EHDR is %#lx\n\0".as_ptr() as *const c_char,
                auxval,
            );
        }
        if auxval == 0 || auxval == (-(ENOENT as c_long)) as c_ulong {
            unsafe {
                ksft_print_msg(b"WARN: getauxval failed\n\0".as_ptr() as *const c_char);
            }
            return 0;
        }

        /* Simpler than parsing ELF header */
        while ret < 0 {
            ret = unsafe { try_to_remap(auxval as *mut c_void, vdso_size) };
            vdso_size = vdso_size.wrapping_add(PAGE_SIZE);
        }

        #[cfg(target_arch = "x86")]
        unsafe {
            /* Glibc is likely to explode now - exit with raw syscall */
            asm!(
                "int 0x80",
                in("eax") __NR_exit,
                in("ebx") if ret != 0 { 1usize } else { 0usize },
                options(noreturn)
            );
        }

        #[cfg(not(target_arch = "x86"))]
        unsafe {
            syscall(SYS_exit, ret);
        }
    } else {
        let mut status: c_int = 0;

        if unsafe { waitpid(child, &mut status, 0) } != child || !WIFEXITED(status) {
            unsafe {
                ksft_test_result_fail(
                    b"mremap() of the vDSO does not work on this kernel!\n\0".as_ptr()
                        as *const c_char,
                );
            }
        } else if WEXITSTATUS(status) != 0 {
            unsafe {
                ksft_test_result_fail(
                    b"Child failed with %d\n\0".as_ptr() as *const c_char,
                    WEXITSTATUS(status),
                );
            }
        } else {
            unsafe {
                ksft_test_result_pass(b"%s\n\0".as_ptr() as *const c_char, b"main\0".as_ptr());
            }
        }
    }

    unsafe {
        ksft_finished();
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
