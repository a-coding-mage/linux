// SPDX-License-Identifier: GPL-2.0-or-later
//
// Translated from objtool/arch/x86/orc.c.  The original C file depends on
// linux/objtool_types.h, asm/orc_types.h, objtool/check.h, objtool/orc.h, and
// objtool/warn.h for the exact definitions of these types and constants.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct orc_entry {
    pub sp_offset: c_int,
    pub bp_offset: c_int,
    pub sp_reg: c_uint,
    pub bp_reg: c_uint,
    pub type_: c_uint,
    pub signal: c_uint,
}

#[repr(C)]
pub struct cfi_reg {
    pub base: c_uint,
    pub offset: c_int,
}

#[repr(C)]
pub struct cfi_state {
    pub cfa: cfi_reg,
    pub regs: [cfi_reg; 16],
    pub type_: c_uint,
    pub signal: c_uint,
}

#[repr(C)]
pub struct elf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct elf_data {
    pub d_buf: *mut c_void,
}

#[repr(C)]
pub struct section {
    pub data: *mut elf_data,
}

#[repr(C)]
pub struct instruction {
    _private: [u8; 0],
}

extern "C" {
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn printf(fmt: *const c_char, ...) -> c_int;

    fn bswap_if_needed(elf: *mut elf, val: c_int) -> c_int;
    fn elf_init_reloc_text_sym(
        elf: *mut elf,
        sec: *mut section,
        offset: c_ulong,
        idx: c_uint,
        insn_sec: *mut section,
        insn_off: c_ulong,
    ) -> c_int;
    fn ERROR_INSN(insn: *mut instruction, fmt: *const c_char, ...) -> c_int;
}

// Constants are supplied by the translated equivalents of the original include
// files.  Numeric values are intentionally not reproduced here.
extern "C" {
    static CFI_BP: c_uint;
    static CFI_AX: c_uint;
    static CFI_DX: c_uint;
    static CFI_SP: c_uint;
    static CFI_DI: c_uint;
    static CFI_R10: c_uint;
    static CFI_R13: c_uint;
    static CFI_SP_INDIRECT: c_uint;
    static CFI_BP_INDIRECT: c_uint;
    static CFI_UNDEFINED: c_uint;
    static CFI_CFA: c_uint;

    static UNWIND_HINT_TYPE_UNDEFINED: c_uint;
    static UNWIND_HINT_TYPE_END_OF_STACK: c_uint;
    static UNWIND_HINT_TYPE_CALL: c_uint;
    static UNWIND_HINT_TYPE_REGS: c_uint;
    static UNWIND_HINT_TYPE_REGS_PARTIAL: c_uint;

    static ORC_TYPE_UNDEFINED: c_uint;
    static ORC_TYPE_END_OF_STACK: c_uint;
    static ORC_TYPE_CALL: c_uint;
    static ORC_TYPE_REGS: c_uint;
    static ORC_TYPE_REGS_PARTIAL: c_uint;

    static ORC_REG_UNDEFINED: c_uint;
    static ORC_REG_PREV_SP: c_uint;
    static ORC_REG_AX: c_uint;
    static ORC_REG_DX: c_uint;
    static ORC_REG_BP: c_uint;
    static ORC_REG_SP: c_uint;
    static ORC_REG_DI: c_uint;
    static ORC_REG_R10: c_uint;
    static ORC_REG_R13: c_uint;
    static ORC_REG_SP_INDIRECT: c_uint;
    static ORC_REG_BP_INDIRECT: c_uint;
}

#[no_mangle]
pub unsafe extern "C" fn init_orc_entry(
    orc: *mut orc_entry,
    cfi: *mut cfi_state,
    insn: *mut instruction,
) -> c_int {
    let bp: *mut cfi_reg = (*cfi).regs.as_mut_ptr().add(CFI_BP as usize);

    memset(
        orc as *mut c_void,
        0,
        core::mem::size_of::<orc_entry>(),
    );

    if cfi.is_null() {
        /*
         * This is usually either unreachable nops/traps (which don't
         * trigger unreachable instruction warnings), or
         * STACK_FRAME_NON_STANDARD functions.
         */
        (*orc).type_ = ORC_TYPE_UNDEFINED;
        return 0;
    }

    if (*cfi).type_ == UNWIND_HINT_TYPE_UNDEFINED {
        (*orc).type_ = ORC_TYPE_UNDEFINED;
        return 0;
    } else if (*cfi).type_ == UNWIND_HINT_TYPE_END_OF_STACK {
        (*orc).type_ = ORC_TYPE_END_OF_STACK;
        return 0;
    } else if (*cfi).type_ == UNWIND_HINT_TYPE_CALL {
        (*orc).type_ = ORC_TYPE_CALL;
    } else if (*cfi).type_ == UNWIND_HINT_TYPE_REGS {
        (*orc).type_ = ORC_TYPE_REGS;
    } else if (*cfi).type_ == UNWIND_HINT_TYPE_REGS_PARTIAL {
        (*orc).type_ = ORC_TYPE_REGS_PARTIAL;
    } else {
        ERROR_INSN(
            insn,
            b"unknown unwind hint type %d\0".as_ptr() as *const c_char,
            (*cfi).type_,
        );
        return -1;
    }

    (*orc).signal = (*cfi).signal;

    if (*cfi).cfa.base == CFI_AX {
        (*orc).sp_reg = ORC_REG_AX;
    } else if (*cfi).cfa.base == CFI_DX {
        (*orc).sp_reg = ORC_REG_DX;
    } else if (*cfi).cfa.base == CFI_SP {
        (*orc).sp_reg = ORC_REG_SP;
    } else if (*cfi).cfa.base == CFI_BP {
        (*orc).sp_reg = ORC_REG_BP;
    } else if (*cfi).cfa.base == CFI_DI {
        (*orc).sp_reg = ORC_REG_DI;
    } else if (*cfi).cfa.base == CFI_R10 {
        (*orc).sp_reg = ORC_REG_R10;
    } else if (*cfi).cfa.base == CFI_R13 {
        (*orc).sp_reg = ORC_REG_R13;
    } else if (*cfi).cfa.base == CFI_SP_INDIRECT {
        (*orc).sp_reg = ORC_REG_SP_INDIRECT;
    } else if (*cfi).cfa.base == CFI_BP_INDIRECT {
        (*orc).sp_reg = ORC_REG_BP_INDIRECT;
    } else {
        ERROR_INSN(
            insn,
            b"unknown CFA base reg %d\0".as_ptr() as *const c_char,
            (*cfi).cfa.base,
        );
        return -1;
    }

    if (*bp).base == CFI_UNDEFINED {
        (*orc).bp_reg = ORC_REG_UNDEFINED;
    } else if (*bp).base == CFI_CFA {
        (*orc).bp_reg = ORC_REG_PREV_SP;
    } else if (*bp).base == CFI_BP {
        (*orc).bp_reg = ORC_REG_BP;
    } else {
        ERROR_INSN(
            insn,
            b"unknown BP base reg %d\0".as_ptr() as *const c_char,
            (*bp).base,
        );
        return -1;
    }

    (*orc).sp_offset = (*cfi).cfa.offset;
    (*orc).bp_offset = (*bp).offset;

    0
}

#[no_mangle]
pub unsafe extern "C" fn write_orc_entry(
    elf: *mut elf,
    orc_sec: *mut section,
    ip_sec: *mut section,
    idx: c_uint,
    insn_sec: *mut section,
    insn_off: c_ulong,
    o: *mut orc_entry,
) -> c_int {
    let orc: *mut orc_entry;

    /* populate ORC data */
    orc = ((*(*orc_sec).data).d_buf as *mut orc_entry).add(idx as usize);
    memcpy(
        orc as *mut c_void,
        o as *const c_void,
        core::mem::size_of::<orc_entry>(),
    );
    (*orc).sp_offset = bswap_if_needed(elf, (*orc).sp_offset);
    (*orc).bp_offset = bswap_if_needed(elf, (*orc).bp_offset);

    /* populate reloc for ip */
    if elf_init_reloc_text_sym(
        elf,
        ip_sec,
        (idx as usize * core::mem::size_of::<c_int>()) as c_ulong,
        idx,
        insn_sec,
        insn_off,
    ) == 0
    {
        return -1;
    }

    0
}

unsafe fn reg_name(reg: c_uint) -> *const c_char {
    if reg == ORC_REG_PREV_SP {
        b"prevsp\0".as_ptr() as *const c_char
    } else if reg == ORC_REG_AX {
        b"ax\0".as_ptr() as *const c_char
    } else if reg == ORC_REG_DX {
        b"dx\0".as_ptr() as *const c_char
    } else if reg == ORC_REG_BP {
        b"bp\0".as_ptr() as *const c_char
    } else if reg == ORC_REG_SP {
        b"sp\0".as_ptr() as *const c_char
    } else if reg == ORC_REG_DI {
        b"di\0".as_ptr() as *const c_char
    } else if reg == ORC_REG_R10 {
        b"r10\0".as_ptr() as *const c_char
    } else if reg == ORC_REG_R13 {
        b"r13\0".as_ptr() as *const c_char
    } else if reg == ORC_REG_SP_INDIRECT {
        b"sp(ind)\0".as_ptr() as *const c_char
    } else if reg == ORC_REG_BP_INDIRECT {
        b"bp(ind)\0".as_ptr() as *const c_char
    } else {
        b"?\0".as_ptr() as *const c_char
    }
}

unsafe fn orc_type_name(type_: c_uint) -> *const c_char {
    if type_ == ORC_TYPE_UNDEFINED {
        b"(und)\0".as_ptr() as *const c_char
    } else if type_ == ORC_TYPE_END_OF_STACK {
        b"end\0".as_ptr() as *const c_char
    } else if type_ == ORC_TYPE_CALL {
        b"call\0".as_ptr() as *const c_char
    } else if type_ == ORC_TYPE_REGS {
        b"regs\0".as_ptr() as *const c_char
    } else if type_ == ORC_TYPE_REGS_PARTIAL {
        b"regs (partial)\0".as_ptr() as *const c_char
    } else {
        b"?\0".as_ptr() as *const c_char
    }
}

unsafe fn print_reg(reg: c_uint, offset: c_int) {
    if reg == ORC_REG_BP_INDIRECT {
        printf(b"(bp%+d)\0".as_ptr() as *const c_char, offset);
    } else if reg == ORC_REG_SP_INDIRECT {
        printf(b"(sp)%+d\0".as_ptr() as *const c_char, offset);
    } else if reg == ORC_REG_UNDEFINED {
        printf(b"(und)\0".as_ptr() as *const c_char);
    } else {
        printf(
            b"%s%+d\0".as_ptr() as *const c_char,
            reg_name(reg),
            offset,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn orc_print_dump(
    dummy_elf: *mut elf,
    orc: *mut orc_entry,
    i: c_int,
) {
    printf(
        b"type:%s\0".as_ptr() as *const c_char,
        orc_type_name((*orc.add(i as usize)).type_),
    );

    printf(b" sp:\0".as_ptr() as *const c_char);
    print_reg(
        (*orc.add(i as usize)).sp_reg,
        bswap_if_needed(dummy_elf, (*orc.add(i as usize)).sp_offset),
    );

    printf(b" bp:\0".as_ptr() as *const c_char);
    print_reg(
        (*orc.add(i as usize)).bp_reg,
        bswap_if_needed(dummy_elf, (*orc.add(i as usize)).bp_offset),
    );

    printf(
        b" signal:%d\n\0".as_ptr() as *const c_char,
        (*orc.add(i as usize)).signal,
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
