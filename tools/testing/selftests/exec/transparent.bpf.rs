// SPDX-License-Identifier: GPL-2.0
/*
 * binfmt_misc_ops handler for the transparent-mode case: match a synthetic
 * riscv ELF header and run the asserting interpreter transparently - the
 * argument vector untouched, the binary in AT_EXECFD and mm->exe_file
 * labeled with the binary.
 */
// C dependencies: "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [c_char; 4] = [
    b'G' as c_char,
    b'P' as c_char,
    b'L' as c_char,
    0,
];

pub const EI_CLASS: usize = 4;
pub const ELFCLASS64: c_int = 2;
pub const EM_RISCV: u16 = 243;

extern "C" {
    pub type linux_binprm;
    pub type bpf_binprm_flags;
    pub type binfmt_misc_ops;

    #[link_name = "BPF_BINPRM_TRANSPARENT"]
    pub static BPF_BINPRM_TRANSPARENT: bpf_binprm_flags;

    #[link_name = "bpf_binprm_set_interp"]
    pub fn bpf_binprm_set_interp(
        bprm: *mut linux_binprm,
        path: *const c_char,
        path__sz: usize,
    ) -> c_int;

    #[link_name = "bpf_binprm_set_flags"]
    pub fn bpf_binprm_set_flags(
        bprm: *mut linux_binprm,
        flags: bpf_binprm_flags,
    ) -> c_int;
}

// From vmlinux.h; used here only for the file-local field access performed by C.
#[repr(C)]
pub struct linux_binprm_layout {
    pub buf: [c_char; 256],
}

#[link_section = "struct_ops.s/match"]
#[no_mangle]
pub unsafe extern "C" fn transparent_match(bprm: *mut linux_binprm) -> bool {
    let bprm = bprm as *mut linux_binprm_layout;
    let machine: u16;

    if (*bprm).buf[0] as u8 != 0x7f
        || (*bprm).buf[1] as u8 != b'E'
        || (*bprm).buf[2] as u8 != b'L'
        || (*bprm).buf[3] as u8 != b'F'
        || (*bprm).buf[EI_CLASS] as c_int != ELFCLASS64
    {
        return false;
    }

    /* e_machine is a 16-bit little-endian field at offset 18. */
    machine = ((*bprm).buf[18] as u8) as u16 | (((*bprm).buf[19] as u8) as u16) << 8;
    machine == EM_RISCV
}

#[link_section = "struct_ops.s/load"]
#[no_mangle]
pub unsafe extern "C" fn transparent_load(bprm: *mut linux_binprm) -> c_int {
    let interp: [c_char; 32] = [
        b'/' as c_char,
        b't' as c_char,
        b'm' as c_char,
        b'p' as c_char,
        b'/' as c_char,
        b'b' as c_char,
        b'i' as c_char,
        b'n' as c_char,
        b'f' as c_char,
        b'm' as c_char,
        b't' as c_char,
        b'_' as c_char,
        b't' as c_char,
        b'r' as c_char,
        b'a' as c_char,
        b'n' as c_char,
        b's' as c_char,
        b'p' as c_char,
        b'a' as c_char,
        b'r' as c_char,
        b'e' as c_char,
        b'n' as c_char,
        b't' as c_char,
        b'_' as c_char,
        b'i' as c_char,
        b'n' as c_char,
        b't' as c_char,
        b'e' as c_char,
        b'r' as c_char,
        b'p' as c_char,
        0,
    ];
    let err: c_int;

    err = bpf_binprm_set_flags(bprm, BPF_BINPRM_TRANSPARENT);
    if err != 0 {
        return err;
    }

    /* @path__sz includes the terminating NUL; 0 commits the selection. */
    bpf_binprm_set_interp(bprm, interp.as_ptr(), core::mem::size_of_val(&interp))
}

// SEC(".struct_ops.link")
// struct binfmt_misc_ops transparent = {
//     .match = (void *)transparent_match,
//     .load = (void *)transparent_load,
//     .name = "transparent",
// };
#[repr(C)]
pub struct binfmt_misc_ops_layout {
    pub match_: *mut c_void,
    pub load: *mut c_void,
    pub name: *const c_char,
}

#[link_section = ".struct_ops.link"]
#[no_mangle]
pub static mut transparent: binfmt_misc_ops_layout = binfmt_misc_ops_layout {
    match_: transparent_match as *mut c_void,
    load: transparent_load as *mut c_void,
    name: b"transparent\0".as_ptr() as *const c_char,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
