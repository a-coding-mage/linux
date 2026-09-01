/*
 * Copyright IBM Corp.
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of version 2.1 of the GNU Lesser General Public License
 * as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it would be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 *
 */

// C dependencies removed from executable Rust:
// assert.h, errno.h, fcntl.h, signal.h, stdarg.h, stdio.h, stdlib.h,
// string.h, sys/mman.h, sys/ptrace.h, sys/syscall.h, ucontext.h, unistd.h,
// and "utils.h".

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

extern "C" {
    static mut errno: c_int;

    static __NR_subpage_prot: c_long;

    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn syscall(number: c_long, ...) -> c_long;
    fn getpagesize() -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn posix_memalign(memptr: *mut *mut c_void, alignment: usize, size: usize) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;
    fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: off_t,
    ) -> *mut c_void;
    fn test_harness(test_function: extern "C" fn() -> c_int, name: *const c_char) -> c_int;

    static mut stderr: *mut FILE;
}

// External C types and constants supplied by the translated dependencies.
#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

type off_t = i64;

#[repr(C)]
pub struct pt_regs {
    pub dar: c_ulong,
    pub nip: c_ulong,
}

#[repr(C)]
pub struct mcontext_t {
    pub regs: *mut pt_regs,
}

#[repr(C)]
pub struct ucontext_t {
    pub uc_mcontext: mcontext_t,
}

#[repr(C)]
pub struct siginfo_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sigaction {
    pub sa_sigaction: extern "C" fn(c_int, *mut siginfo_t, *mut c_void),
    pub sa_flags: c_int,
}

const ENOENT: c_int = 2;
const ENOSYS: c_int = 38;
const SA_SIGINFO: c_int = 4;
const SIGSEGV: c_int = 11;
const O_RDWR: c_int = 2;
const SEEK_END: c_int = 2;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_SHARED: c_int = 0x01;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

// utils.h supplies SKIP_IF and FAIL_IF in the C source.
macro_rules! SKIP_IF {
    ($cond:expr) => {
        if $cond {
            return 0;
        }
    };
}

macro_rules! FAIL_IF {
    ($cond:expr) => {
        if $cond != 0 {
            return 1;
        }
    };
}

static mut file_name: *mut c_char = core::ptr::null_mut();

static mut in_test: c_int = 0;
static mut faulted: c_int = 0;
static mut dar: *mut c_void = core::ptr::null_mut();
static mut errors: c_int = 0;

extern "C" fn segv(_signum: c_int, _info: *mut siginfo_t, ctxt_v: *mut c_void) {
    unsafe {
        let ctxt = ctxt_v as *mut ucontext_t;
        let regs = (*ctxt).uc_mcontext.regs;

        if in_test == 0 {
            fprintf(stderr, cstr!("Segfault outside of test !\n"));
            exit(1);
        }

        faulted = 1;
        dar = (*regs).dar as *mut c_void;
        (*regs).nip = (*regs).nip.wrapping_add(4);
    }
}

#[inline]
unsafe fn do_read(addr: *const c_void) {
    let ret: c_int;

    asm!(
        "lwz {0},0({1}); twi 0,{0},0; isync;",
        out(reg) ret,
        in(reg) addr,
        options(nostack)
    );
    let _ = ret;
}

#[inline]
unsafe fn do_write(addr: *const c_void) {
    let val: c_int = 0x1234567;

    asm!(
        "stw {0},0({1}); sync;",
        in(reg) val,
        in(reg) addr,
        options(nostack)
    );
}

#[inline]
unsafe fn check_faulted(addr: *mut c_void, page: c_long, subpage: c_long, write: c_int) {
    let mut want_fault: c_int = (subpage == ((page + 3) % 16)) as c_int;

    if write != 0 {
        want_fault |= (subpage == ((page + 1) % 16)) as c_int;
    }

    if faulted != want_fault {
        printf(
            cstr!("Failed at %p (p=%ld,sp=%ld,w=%d), want=%s, got=%s !\n"),
            addr,
            page,
            subpage,
            write,
            if want_fault != 0 {
                cstr!("fault")
            } else {
                cstr!("pass")
            },
            if faulted != 0 {
                cstr!("fault")
            } else {
                cstr!("pass")
            },
        );
        errors += 1;
    }

    if faulted != 0 {
        if dar != addr {
            printf(cstr!("Fault expected at %p and happened at %p !\n"), addr, dar);
        }
        faulted = 0;
        asm!("sync", options(nostack));
    }
}

unsafe fn run_test(mut addr: *mut c_void, size: c_ulong) -> c_int {
    let map: *mut c_uint;
    let mut i: c_long;
    let mut j: c_long;
    let pages: c_long;
    let err: c_long;

    pages = (size / 0x10000) as c_long;
    map = malloc((pages * 4) as usize) as *mut c_uint;
    assert!(!map.is_null());

    /*
     * for each page, mark subpage i % 16 read only and subpage
     * (i + 3) % 16 inaccessible
     */
    i = 0;
    while i < pages {
        *map.offset(i as isize) = ((0x40000000u32 >> (((i + 1) * 2) % 32))
            | (0xc0000000u32 >> (((i + 3) * 2) % 32))) as c_uint;
        i += 1;
    }

    err = syscall(__NR_subpage_prot, addr, size, map);
    if err != 0 {
        perror(cstr!("subpage_perm"));
        return 1;
    }
    free(map as *mut c_void);

    in_test = 1;
    errors = 0;
    i = 0;
    while i < pages {
        j = 0;
        while j < 16 {
            do_read(addr as *const c_void);
            check_faulted(addr, i, j, 0);
            do_write(addr as *const c_void);
            check_faulted(addr, i, j, 1);

            j += 1;
            addr = (addr as *mut u8).add(0x1000) as *mut c_void;
        }
        i += 1;
    }

    in_test = 0;
    if errors != 0 {
        printf(cstr!("%d errors detected\n"), errors);
        return 1;
    }

    0
}

unsafe fn syscall_available() -> c_int {
    let rc: c_int;

    errno = 0;
    rc = syscall(__NR_subpage_prot, 0, 0, 0) as c_int;

    (rc == 0 || (errno != ENOENT && errno != ENOSYS)) as c_int
}

#[no_mangle]
pub extern "C" fn test_anon() -> c_int {
    unsafe {
        let mut align: c_ulong;
        let act = sigaction {
            sa_sigaction: segv,
            sa_flags: SA_SIGINFO,
        };
        let mut mallocblock: *mut c_void = core::ptr::null_mut();
        let mallocsize: c_ulong;

        SKIP_IF!(syscall_available() == 0);

        if getpagesize() != 0x10000 {
            fprintf(stderr, cstr!("Kernel page size must be 64K!\n"));
            return 1;
        }

        sigaction(SIGSEGV, &act, core::ptr::null_mut());

        mallocsize = 4 * 16 * 1024 * 1024;

        FAIL_IF!(posix_memalign(
            &mut mallocblock,
            (64 * 1024) as usize,
            mallocsize as usize
        ));

        align = mallocblock as c_ulong;
        if (align & 0xffff) != 0 {
            align = (align | 0xffff).wrapping_add(1);
        }

        mallocblock = align as *mut c_void;

        printf(
            cstr!("allocated malloc block of 0x%lx bytes at %p\n"),
            mallocsize,
            mallocblock,
        );

        printf(cstr!("testing malloc block...\n"));

        run_test(mallocblock, mallocsize)
    }
}

#[no_mangle]
pub extern "C" fn test_file() -> c_int {
    unsafe {
        let act = sigaction {
            sa_sigaction: segv,
            sa_flags: SA_SIGINFO,
        };
        let fileblock: *mut c_void;
        let mut filesize: off_t;
        let fd: c_int;

        SKIP_IF!(syscall_available() == 0);

        fd = open(file_name, O_RDWR);
        if fd == -1 {
            perror(cstr!("failed to open file"));
            return 1;
        }
        sigaction(SIGSEGV, &act, core::ptr::null_mut());

        filesize = lseek(fd, 0, SEEK_END);
        if (filesize & 0xffff) != 0 {
            filesize &= !(0xffffu64 as off_t);
        }

        fileblock = mmap(
            core::ptr::null_mut(),
            filesize as usize,
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            fd,
            0,
        );
        if fileblock == MAP_FAILED {
            perror(cstr!("failed to map file"));
            return 1;
        }
        printf(
            cstr!("allocated %s for 0x%llx bytes at %p\n"),
            file_name,
            filesize as i64,
            fileblock,
        );

        printf(cstr!("testing file map...\n"));

        run_test(fileblock, filesize as c_ulong)
    }
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let rc: c_int;

    rc = test_harness(test_anon, cstr!("subpage_prot_anon"));
    if rc != 0 {
        return rc;
    }

    if argc > 1 {
        file_name = *argv.add(1);
    } else {
        file_name = cstr!("tempfile") as *mut c_char;
    }

    test_harness(test_file, cstr!("subpage_prot_file"))
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
