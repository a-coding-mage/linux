/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies: <stddef.h>, <stdio.h>, <linux/types.h>

use std::ffi::{c_int, c_uchar, c_void};

pub type size_t = usize;
pub type ssize_t = isize;
pub type u8 = c_uchar;
pub type uint8_t = u8;
pub type uint64_t = u64;
pub type FILE = c_void;

#[repr(C)]
pub struct addr_location {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_insn {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_sample {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

pub const PRINT_INSN_IMM_HEX: c_int = 1 << 0;

unsafe extern "C" {
    pub fn sample__fprintf_insn_asm(
        sample: *mut perf_sample,
        thread: *mut thread,
        machine: *mut machine,
        fp: *mut FILE,
        al: *mut addr_location,
    ) -> size_t;

    pub fn sample__fprintf_insn_raw(sample: *mut perf_sample, fp: *mut FILE) -> size_t;

    pub fn fprintf_insn_asm(
        machine: *mut machine,
        thread: *mut thread,
        cpumode: u8,
        is64bit: bool,
        code: *const uint8_t,
        code_size: size_t,
        ip: uint64_t,
        lenp: *mut c_int,
        print_opts: c_int,
        fp: *mut FILE,
    ) -> ssize_t;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
