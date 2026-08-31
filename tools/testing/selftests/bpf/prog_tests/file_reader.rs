// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

// Translated from:
// <test_progs.h>
// <network_helpers.h>
// "file_reader.skel.h"
// "file_reader_fail.skel.h"
// <dlfcn.h>
// <sys/mman.h>

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

type SsizeT = isize;

#[repr(C)]
pub struct DlInfo {
    pub dli_fname: *const c_char,
    pub dli_fbase: *mut c_void,
    pub dli_sname: *const c_char,
    pub dli_saddr: *mut c_void,
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file_reader_bss {
    pub user_buf: [u8; 256000],
    pub pid: c_int,
    pub err: c_int,
    pub run_success: c_int,
}

#[repr(C)]
pub struct file_reader {
    pub obj: *mut bpf_object,
    pub bss: *mut file_reader_bss,
}

static mut USER_PTR: *const c_char = b"hello world\0".as_ptr() as *const c_char;
static mut FILE_CONTENTS: [u8; 256000] = [0; 256000];
static mut ADDR: *mut c_void = core::ptr::null_mut();

const O_RDONLY: c_int = 0;
const MADV_PAGEOUT: c_int = 21;
const _SC_PAGESIZE: c_int = 30;

unsafe extern "C" {
    fn dladdr(addr: *mut c_void, info: *mut DlInfo) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;
    fn sysconf(name: c_int) -> c_long;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> SsizeT;
    fn close(fd: c_int) -> c_int;
    fn getpid() -> c_int;
    fn madvise(addr: *mut c_void, length: usize, advice: c_int) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;

    fn ASSERT_OK_FD(fd: c_int, name: *const c_char) -> bool;
    fn ASSERT_GT(left: SsizeT, right: SsizeT, name: *const c_char) -> bool;
    fn ASSERT_EQ(left: c_long, right: c_long, name: *const c_char) -> bool;
    fn ASSERT_NEQ(left: *mut c_void, right: *mut c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *mut file_reader, name: *const c_char) -> bool;

    fn test__start_subtest(name: *const c_char) -> bool;
    fn RUN_TESTS_file_reader_fail();

    fn file_reader__open() -> *mut file_reader;
    fn file_reader__load(skel: *mut file_reader) -> c_int;
    fn file_reader__attach(skel: *mut file_reader) -> c_int;
    fn file_reader__destroy(skel: *mut file_reader);

    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn bpf_program__name(prog: *mut bpf_program) -> *const c_char;

    // Rust translation of bpf_object__for_each_program(prog, skel->obj).
    fn bpf_object__next_program(obj: *mut bpf_object, prog: *mut bpf_program) -> *mut bpf_program;
}

pub unsafe extern "C" fn get_executable_base_addr() -> *mut c_void {
    let mut info = DlInfo {
        dli_fname: core::ptr::null(),
        dli_fbase: core::ptr::null_mut(),
        dli_sname: core::ptr::null(),
        dli_saddr: core::ptr::null_mut(),
    };

    if unsafe {
        dladdr(
            get_executable_base_addr as *mut c_void,
            &mut info as *mut DlInfo,
        )
    } == 0
    {
        unsafe {
            fprintf(stderr, b"dladdr failed\n\0".as_ptr() as *const c_char);
        }
        return core::ptr::null_mut();
    }

    info.dli_fbase
}

unsafe fn initialize_file_contents() -> c_int {
    let fd: c_int;
    let page_sz: c_int = unsafe { sysconf(_SC_PAGESIZE) as c_int };
    let mut n: SsizeT = 0;
    let mut cur: SsizeT;

    fd = unsafe { open(b"/proc/self/exe\0".as_ptr() as *const c_char, O_RDONLY) };
    if !unsafe { ASSERT_OK_FD(fd, b"Open /proc/self/exe\n\0".as_ptr() as *const c_char) } {
        return 1;
    }

    loop {
        cur = unsafe {
            read(
                fd,
                FILE_CONTENTS.as_mut_ptr().offset(n) as *mut c_void,
                FILE_CONTENTS.len() - n as usize,
            )
        };
        if !unsafe { ASSERT_GT(cur, 0, b"read success\0".as_ptr() as *const c_char) } {
            break;
        }
        n += cur;
        if !(n < FILE_CONTENTS.len() as SsizeT) {
            break;
        }
    }

    unsafe {
        close(fd);
    }

    if !unsafe {
        ASSERT_EQ(
            n as c_long,
            FILE_CONTENTS.len() as c_long,
            b"Read /proc/self/exe\n\0".as_ptr() as *const c_char,
        )
    } {
        return 1;
    }

    unsafe {
        ADDR = get_executable_base_addr();
    }
    if !unsafe {
        ASSERT_NEQ(
            ADDR,
            core::ptr::null_mut(),
            b"get executable address\0".as_ptr() as *const c_char,
        )
    } {
        return 1;
    }

    /* page-align base file address */
    unsafe {
        ADDR = ((ADDR as c_ulong) & !((page_sz - 1) as c_ulong)) as *mut c_void;
    }

    0
}

unsafe fn run_test(prog_name: *const c_char) {
    let skel: *mut file_reader;
    let mut prog: *mut bpf_program;
    let mut err: c_int;
    let fd: c_int;

    err = unsafe { initialize_file_contents() };
    if !unsafe { ASSERT_OK(err, b"initialize file contents\0".as_ptr() as *const c_char) } {
        return;
    }

    skel = unsafe { file_reader__open() };
    if !unsafe { ASSERT_OK_PTR(skel, b"file_reader__open\0".as_ptr() as *const c_char) } {
        return;
    }

    // bpf_object__for_each_program(prog, skel->obj)
    prog = core::ptr::null_mut();
    loop {
        prog = unsafe { bpf_object__next_program((*skel).obj, prog) };
        if prog.is_null() {
            break;
        }
        unsafe {
            bpf_program__set_autoload(
                prog,
                strcmp(bpf_program__name(prog), prog_name) == 0,
            );
        }
    }

    unsafe {
        memcpy(
            (*(*skel).bss).user_buf.as_mut_ptr() as *mut c_void,
            FILE_CONTENTS.as_ptr() as *const c_void,
            FILE_CONTENTS.len(),
        );
        (*(*skel).bss).pid = getpid();
    }

    err = unsafe { file_reader__load(skel) };
    if !unsafe { ASSERT_OK(err, b"file_reader__load\0".as_ptr() as *const c_char) } {
        unsafe {
            file_reader__destroy(skel);
        }
        return;
    }

    /*
     * Page out range 0..512K, use 0..256K for positive tests and
     * 256K..512K for negative tests expecting page faults
     */
    if !unsafe {
        ASSERT_OK(
            madvise(ADDR, FILE_CONTENTS.len() * 2, MADV_PAGEOUT),
            b"madvise pageout\0".as_ptr() as *const c_char,
        )
    } {
        unsafe {
            file_reader__destroy(skel);
        }
        return;
    }

    err = unsafe { file_reader__attach(skel) };
    if !unsafe { ASSERT_OK(err, b"file_reader__attach\0".as_ptr() as *const c_char) } {
        unsafe {
            file_reader__destroy(skel);
        }
        return;
    }

    fd = unsafe { open(b"/proc/self/exe\0".as_ptr() as *const c_char, O_RDONLY) };
    if fd >= 0 {
        unsafe {
            close(fd);
        }
    }

    unsafe {
        ASSERT_EQ((*(*skel).bss).err as c_long, 0, b"err\0".as_ptr() as *const c_char);
        ASSERT_EQ(
            (*(*skel).bss).run_success as c_long,
            1,
            b"run_success\0".as_ptr() as *const c_char,
        );
    }

    unsafe {
        file_reader__destroy(skel);
    }
}

pub unsafe extern "C" fn test_file_reader() {
    if unsafe { test__start_subtest(b"on_open_expect_fault\0".as_ptr() as *const c_char) } {
        unsafe {
            run_test(b"on_open_expect_fault\0".as_ptr() as *const c_char);
        }
    }

    if unsafe { test__start_subtest(b"on_open_validate_file_read\0".as_ptr() as *const c_char) } {
        unsafe {
            run_test(b"on_open_validate_file_read\0".as_ptr() as *const c_char);
        }
    }

    if unsafe { test__start_subtest(b"negative\0".as_ptr() as *const c_char) } {
        unsafe {
            RUN_TESTS_file_reader_fail();
        }
    }
}
