// SPDX-License-Identifier: GPL-2.0-only

// C dependencies in the original file:
// limits.h, stdio.h, stdlib.h, string.h, unistd.h, linux/compiler.h
// "debug.h", "tests.h"
// When HAVE_JITDUMP is enabled, also libelf.h and "../util/genelf.h".

use core::ffi::{c_char, c_int, c_void};

const TEMPL: &[u8] = b"/tmp/perf-test-XXXXXX\0";

// PATH_MAX is provided by <limits.h> in the C source.
extern "C" {
    static PATH_MAX: usize;
}

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

extern "C" {
    fn mkstemp(template: *mut c_char) -> c_int;
    fn perror(s: *const c_char);
    fn close(fd: c_int) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;

    static TEST_FAIL: c_int;
    static TEST_SKIP: c_int;

    fn pr_info(fmt: *const c_char, ...);
}

// Available only when the original HAVE_JITDUMP build condition is enabled.
#[cfg(HAVE_JITDUMP)]
extern "C" {
    fn jit_write_elf(
        fd: c_int,
        load_addr: u64,
        sym: *const c_char,
        code: *const c_void,
        csize: usize,
        debug: *const c_void,
        nr_debug_entries: c_int,
        unwinding: *const c_void,
        unwinding_header_size: usize,
        unwinding_size: usize,
    ) -> c_int;
}

static mut test__jit_write_elf: unsafe extern "C" fn(*mut test_suite, c_int) -> c_int =
    test__jit_write_elf_impl;

unsafe extern "C" fn test__jit_write_elf_impl(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    #[cfg(HAVE_JITDUMP)]
    {
        static mut x86_code: [u8; 12] = [
            0xBB, 0x2A, 0x00, 0x00, 0x00, /* movl $42, %ebx */
            0xB8, 0x01, 0x00, 0x00, 0x00, /* movl $1, %eax */
            0xCD, 0x80, /* int $0x80 */
        ];
        let mut path = vec![0 as c_char; PATH_MAX];
        let fd: c_int;
        let ret: c_int;

        core::ptr::copy_nonoverlapping(
            TEMPL.as_ptr() as *const c_char,
            path.as_mut_ptr(),
            TEMPL.len(),
        );

        fd = mkstemp(path.as_mut_ptr());
        if fd < 0 {
            perror(b"mkstemp failed\0".as_ptr() as *const c_char);
            return TEST_FAIL;
        }

        pr_info(
            b"Writing jit code to: %s\n\0".as_ptr() as *const c_char,
            path.as_ptr(),
        );

        ret = jit_write_elf(
            fd,
            0,
            b"main\0".as_ptr() as *const c_char,
            x86_code.as_ptr() as *const c_void,
            x86_code.len(),
            core::ptr::null(),
            0,
            core::ptr::null(),
            0,
            0,
        );
        close(fd);

        unlink(path.as_ptr());

        return if ret != 0 { TEST_FAIL } else { 0 };
    }

    #[cfg(not(HAVE_JITDUMP))]
    {
        return TEST_SKIP;
    }
}

// DEFINE_SUITE("Test jit_write_elf", jit_write_elf);
extern "C" {
    static mut jit_write_elf: test_suite;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
