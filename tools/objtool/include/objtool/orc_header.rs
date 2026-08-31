// SPDX-License-Identifier: GPL-2.0-or-later
// Translated from objtool/include/objtool/orc.h.
// C header guard _OBJTOOL_ORC_H omitted in Rust.
// Depends on declarations from <objtool/check.h>.

unsafe extern "C" {
    pub fn init_orc_entry(
        orc: *mut orc_entry,
        cfi: *mut cfi_state,
        insn: *mut instruction,
    ) -> ::core::ffi::c_int;

    pub fn orc_print_dump(
        dummy_elf: *mut elf,
        orc: *mut orc_entry,
        i: ::core::ffi::c_int,
    );

    pub fn write_orc_entry(
        elf: *mut elf,
        orc_sec: *mut section,
        ip_sec: *mut section,
        idx: ::core::ffi::c_uint,
        insn_sec: *mut section,
        insn_off: ::core::ffi::c_ulong,
        o: *mut orc_entry,
    ) -> ::core::ffi::c_int;
}
