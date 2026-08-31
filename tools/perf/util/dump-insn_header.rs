/* SPDX-License-Identifier: GPL-2.0 */

// C header dependency: <linux/types.h>

pub const MAXINSN: i32 = 15;

#[repr(C)]
pub struct thread {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_insn {
    /* Initialized by callers: */
    pub thread: *mut thread,
    pub machine: *mut machine,
    pub cpumode: u8,
    pub is64bit: bool,
    pub cpu: ::std::os::raw::c_int,
    /* Temporary */
    pub out: [::std::os::raw::c_char; 256],
}

unsafe extern "C" {
    pub fn dump_insn(
        x: *mut perf_insn,
        ip: u64,
        inbuf: *mut u8,
        inlen: ::std::os::raw::c_int,
        lenp: *mut ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;

    pub fn arch_is_uncond_branch(
        buf: *const ::std::os::raw::c_uchar,
        len: usize,
        x86_64: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
