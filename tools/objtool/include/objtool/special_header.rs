/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2015 Josh Poimboeuf <jpoimboe@redhat.com>
 */

// C dependencies from:
// #include <stdbool.h>
// #include <objtool/check.h>
// #include <objtool/elf.h>

pub const C_JUMP_TABLE_SECTION: &str = ".data.rel.ro.c_jump_table";

#[repr(C)]
pub struct special_alt {
    pub list: list_head,

    pub group: bool,
    pub jump_or_nop: bool,
    pub key_addend: u8,

    pub orig_sec: *mut section,
    pub orig_off: core::ffi::c_ulong,

    pub new_sec: *mut section,
    pub new_off: core::ffi::c_ulong,

    pub orig_len: core::ffi::c_uint,
    pub new_len: core::ffi::c_uint,
    pub feature: core::ffi::c_uint, /* group only */
}

unsafe extern "C" {
    pub fn special_get_alts(elf: *mut elf, alts: *mut list_head) -> core::ffi::c_int;

    pub fn arch_handle_alternative(alt: *mut special_alt);

    /*
     * Should the reloc at @offset -- the "new" (replacement) field of a special
     * section group entry -- be ignored?  The meaning of a zero-length replacement
     * is arch specific, so the arch decides.
     */
    pub fn arch_alt_ignore_new_reloc(sec: *mut section, offset: core::ffi::c_ulong) -> bool;

    pub fn arch_support_alt_relocation(
        special_alt: *mut special_alt,
        insn: *mut instruction,
        reloc: *mut reloc,
    ) -> bool;

    pub fn arch_find_switch_table(
        file: *mut objtool_file,
        insn: *mut instruction,
        table_size: *mut core::ffi::c_ulong,
    ) -> *mut reloc;

    pub fn arch_cpu_feature_name(feature_number: core::ffi::c_int) -> *const core::ffi::c_char;
}
