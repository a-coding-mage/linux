/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies: stdbool.h, stdint.h, stdio.h, stdlib.h,
// linux/compiler.h, and linux/types.h.
// The C preprocessor condition HAVE_LIBCAPSTONE_SUPPORT is preserved as a Rust
// cfg name for the build system to define.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct annotate_args {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

pub type FILE = c_void;
pub type ssize_t = isize;
pub type size_t = usize;

#[cfg(HAVE_LIBCAPSTONE_SUPPORT)]
unsafe extern "C" {
    pub fn capstone__fprintf_insn_asm(
        machine: *mut machine,
        thread: *mut thread,
        cpumode: u8,
        is64bit: bool,
        code: *const u8,
        code_size: size_t,
        ip: u64,
        lenp: *mut c_int,
        print_opts: c_int,
        fp: *mut FILE,
    ) -> ssize_t;

    pub fn symbol__disassemble_capstone(
        filename: *const c_char,
        sym: *mut symbol,
        args: *mut annotate_args,
    ) -> c_int;

    pub fn symbol__disassemble_capstone_powerpc(
        filename: *const c_char,
        sym: *mut symbol,
        args: *mut annotate_args,
    ) -> c_int;
}

#[cfg(not(HAVE_LIBCAPSTONE_SUPPORT))]
#[inline]
pub unsafe fn capstone__fprintf_insn_asm(
    _machine: *mut machine,
    _thread: *mut thread,
    _cpumode: u8,
    _is64bit: bool,
    _code: *const u8,
    _code_size: size_t,
    _ip: u64,
    _lenp: *mut c_int,
    _print_opts: c_int,
    _fp: *mut FILE,
) -> ssize_t {
    -1
}

#[cfg(not(HAVE_LIBCAPSTONE_SUPPORT))]
#[inline]
pub unsafe fn symbol__disassemble_capstone(
    _filename: *const c_char,
    _sym: *mut symbol,
    _args: *mut annotate_args,
) -> c_int {
    -1
}

#[cfg(not(HAVE_LIBCAPSTONE_SUPPORT))]
#[inline]
pub unsafe fn symbol__disassemble_capstone_powerpc(
    _filename: *const c_char,
    _sym: *mut symbol,
    _args: *mut annotate_args,
) -> c_int {
    -1
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
