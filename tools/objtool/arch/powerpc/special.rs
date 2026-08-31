// SPDX-License-Identifier: GPL-2.0-or-later
// Translated from C implementation source: objtool/arch/powerpc/special.c

use core::ffi::{c_char, c_int, c_ulong};

#[repr(C)]
pub struct special_alt {
    _private: [u8; 0],
}

#[repr(C)]
pub struct instruction {
    _private: [u8; 0],
}

#[repr(C)]
pub struct reloc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct objtool_file {
    _private: [u8; 0],
}

extern "C" {
    fn exit(status: c_int) -> !;
}

#[no_mangle]
pub unsafe extern "C" fn arch_support_alt_relocation(
    special_alt: *mut special_alt,
    insn: *mut instruction,
    reloc: *mut reloc,
) -> bool {
    let _ = special_alt;
    let _ = insn;
    let _ = reloc;
    exit(-1);
}

#[no_mangle]
pub unsafe extern "C" fn arch_find_switch_table(
    file: *mut objtool_file,
    insn: *mut instruction,
    table_size: *mut c_ulong,
) -> *mut reloc {
    let _ = file;
    let _ = insn;
    let _ = table_size;
    exit(-1);
}

#[no_mangle]
pub unsafe extern "C" fn arch_cpu_feature_name(feature_number: c_int) -> *const c_char {
    let _ = feature_number;
    core::ptr::null()
}
