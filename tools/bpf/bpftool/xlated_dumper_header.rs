/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/* Copyright (C) 2018 Netronome Systems, Inc. */

pub const SYM_MAX_NAME: usize = 256;
pub const MODULE_MAX_NAME: usize = 64;

#[repr(C)]
pub struct bpf_prog_linfo {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct btf {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct kernel_sym {
    pub address: libc::c_ulong,
    pub name: [libc::c_char; SYM_MAX_NAME],
    pub module: [libc::c_char; MODULE_MAX_NAME],
}

#[repr(C)]
pub struct dump_data {
    pub address_call_base: libc::c_ulong,
    pub sym_mapping: *mut kernel_sym,
    pub sym_count: __u32,
    pub jited_ksyms: *mut __u64,
    pub nr_jited_ksyms: __u32,
    pub btf: *mut btf,
    pub func_info: *mut libc::c_void,
    pub finfo_rec_size: __u32,
    pub prog_linfo: *const bpf_prog_linfo,
    pub scratch_buff: [libc::c_char; SYM_MAX_NAME + 8],
}

unsafe extern "C" {
    pub fn kernel_syms_load(dd: *mut dump_data);
    pub fn kernel_syms_destroy(dd: *mut dump_data);
    pub fn kernel_syms_search(dd: *mut dump_data, key: libc::c_ulong) -> *mut kernel_sym;
    pub fn dump_xlated_json(
        dd: *mut dump_data,
        buf: *mut libc::c_void,
        len: libc::c_uint,
        opcodes: bool,
        linum: bool,
    );
    pub fn dump_xlated_plain(
        dd: *mut dump_data,
        buf: *mut libc::c_void,
        len: libc::c_uint,
        opcodes: bool,
        linum: bool,
    );
    pub fn dump_xlated_for_graph(
        dd: *mut dump_data,
        buf: *mut libc::c_void,
        buf_end: *mut libc::c_void,
        start_index: libc::c_uint,
        opcodes: bool,
        linum: bool,
    );
}
