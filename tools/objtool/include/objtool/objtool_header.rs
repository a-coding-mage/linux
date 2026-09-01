// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2020 Matt Helsley <mhelsley@vmware.com>
 */

// C dependencies removed from executable Rust:
// <stdbool.h>, <linux/list.h>, <linux/hashtable.h>, <objtool/elf.h>
use crate::{elf, hlist_head, list_head, symbol};

// C macro:
// #define __weak __attribute__((weak))
// Rust equivalent at use sites is weak linkage, e.g. #[linkage = "weak"].

#[repr(C)]
pub struct pv_state {
    pub clean: bool,
    pub targets: list_head,
}

#[repr(C)]
pub struct objtool_file {
    pub elf: *mut elf,
    // DECLARE_HASHTABLE(insn_hash, 20);
    pub insn_hash: [hlist_head; 1usize << 20],
    pub retpoline_call_list: list_head,
    pub return_thunk_list: list_head,
    pub static_call_list: list_head,
    pub mcount_loc_list: list_head,
    pub endbr_list: list_head,
    pub call_list: list_head,
    pub ignore_unreachables: bool,
    pub hints: bool,
    pub rodata: bool,
    pub klp: bool,

    pub nr_endbr: ::std::os::raw::c_uint,
    pub nr_endbr_int: ::std::os::raw::c_uint,

    pub jl_short: ::std::os::raw::c_ulong,
    pub jl_long: ::std::os::raw::c_ulong,
    pub jl_nop_short: ::std::os::raw::c_ulong,
    pub jl_nop_long: ::std::os::raw::c_ulong,

    pub pv_ops: *mut pv_state,
}

unsafe extern "C" {
    pub fn top_level_dir(file: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;

    pub fn init_signal_handler() -> ::std::os::raw::c_int;

    pub fn objtool_open_read(_objname: *const ::std::os::raw::c_char) -> *mut objtool_file;

    pub fn objtool_pv_add(
        file: *mut objtool_file,
        idx: ::std::os::raw::c_int,
        func: *mut symbol,
    ) -> ::std::os::raw::c_int;

    pub fn check(file: *mut objtool_file) -> ::std::os::raw::c_int;
    pub fn orc_dump(objname: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn orc_create(file: *mut objtool_file) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
