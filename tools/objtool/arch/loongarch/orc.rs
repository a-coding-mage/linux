// SPDX-License-Identifier: GPL-2.0-or-later
// Translated from objtool/arch/loongarch/orc.c.
// Original C dependencies:
// <linux/objtool_types.h>, <asm/orc_types.h>,
// <objtool/check.h>, <objtool/orc.h>, <objtool/warn.h>

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

unsafe extern "C" {
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn printf(format: *const c_char, ...) -> c_int;

    fn elf_init_reloc_text_sym(
        elf: *mut elf,
        sec: *mut section,
        offset: usize,
        sym_idx: c_uint,
        insn_sec: *mut section,
        insn_off: c_ulong,
    ) -> bool;

    fn ERROR_INSN(insn: *mut instruction, format: *const c_char, ...);
}

pub unsafe extern "C" fn init_orc_entry(
    orc: *mut orc_entry,
    cfi: *mut cfi_state,
    insn: *mut instruction,
) -> c_int {
    unsafe {
        memset(
            orc as *mut c_void,
            0,
            size_of::<orc_entry>(),
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

        let fp: *mut cfi_reg = &mut (*cfi).regs[CFI_FP as usize];
        let ra: *mut cfi_reg = &mut (*cfi).regs[CFI_RA as usize];

        match (*cfi).type_ {
            UNWIND_HINT_TYPE_UNDEFINED => {
                (*orc).type_ = ORC_TYPE_UNDEFINED;
                return 0;
            }
            UNWIND_HINT_TYPE_END_OF_STACK => {
                (*orc).type_ = ORC_TYPE_END_OF_STACK;
                return 0;
            }
            UNWIND_HINT_TYPE_CALL => {
                (*orc).type_ = ORC_TYPE_CALL;
            }
            UNWIND_HINT_TYPE_REGS => {
                (*orc).type_ = ORC_TYPE_REGS;
            }
            UNWIND_HINT_TYPE_REGS_PARTIAL => {
                (*orc).type_ = ORC_TYPE_REGS_PARTIAL;
            }
            _ => {
                ERROR_INSN(
                    insn,
                    b"unknown unwind hint type %d\0".as_ptr() as *const c_char,
                    (*cfi).type_,
                );
                return -1;
            }
        }

        (*orc).signal = (*cfi).signal;

        match (*cfi).cfa.base {
            CFI_SP => {
                (*orc).sp_reg = ORC_REG_SP;
            }
            CFI_FP => {
                (*orc).sp_reg = ORC_REG_FP;
            }
            _ => {
                ERROR_INSN(
                    insn,
                    b"unknown CFA base reg %d\0".as_ptr() as *const c_char,
                    (*cfi).cfa.base,
                );
                return -1;
            }
        }

        match (*fp).base {
            CFI_UNDEFINED => {
                (*orc).fp_reg = ORC_REG_UNDEFINED;
                (*orc).fp_offset = 0;
            }
            CFI_CFA => {
                (*orc).fp_reg = ORC_REG_PREV_SP;
                (*orc).fp_offset = (*fp).offset;
            }
            CFI_FP => {
                (*orc).fp_reg = ORC_REG_FP;
            }
            _ => {
                ERROR_INSN(
                    insn,
                    b"unknown FP base reg %d\0".as_ptr() as *const c_char,
                    (*fp).base,
                );
                return -1;
            }
        }

        match (*ra).base {
            CFI_UNDEFINED => {
                (*orc).ra_reg = ORC_REG_UNDEFINED;
                (*orc).ra_offset = 0;
            }
            CFI_CFA => {
                (*orc).ra_reg = ORC_REG_PREV_SP;
                (*orc).ra_offset = (*ra).offset;
            }
            CFI_FP => {
                (*orc).ra_reg = ORC_REG_FP;
            }
            _ => {
                ERROR_INSN(
                    insn,
                    b"unknown RA base reg %d\0".as_ptr() as *const c_char,
                    (*ra).base,
                );
                return -1;
            }
        }

        (*orc).sp_offset = (*cfi).cfa.offset;

        0
    }
}

pub unsafe extern "C" fn write_orc_entry(
    elf: *mut elf,
    orc_sec: *mut section,
    ip_sec: *mut section,
    idx: c_uint,
    insn_sec: *mut section,
    insn_off: c_ulong,
    o: *mut orc_entry,
) -> c_int {
    unsafe {
        let orc: *mut orc_entry =
            ((*(*orc_sec).data).d_buf as *mut orc_entry).add(idx as usize);

        /* populate ORC data */
        memcpy(
            orc as *mut c_void,
            o as *const c_void,
            size_of::<orc_entry>(),
        );

        /* populate reloc for ip */
        if !elf_init_reloc_text_sym(
            elf,
            ip_sec,
            idx as usize * size_of::<c_int>(),
            idx,
            insn_sec,
            insn_off,
        ) {
            return -1;
        }

        0
    }
}

unsafe fn reg_name(reg: c_uint) -> *const c_char {
    match reg {
        ORC_REG_SP => b"sp\0".as_ptr() as *const c_char,
        ORC_REG_FP => b"fp\0".as_ptr() as *const c_char,
        ORC_REG_PREV_SP => b"prevsp\0".as_ptr() as *const c_char,
        _ => b"?\0".as_ptr() as *const c_char,
    }
}

unsafe fn orc_type_name(type_: c_uint) -> *const c_char {
    match type_ {
        UNWIND_HINT_TYPE_CALL => b"call\0".as_ptr() as *const c_char,
        UNWIND_HINT_TYPE_REGS => b"regs\0".as_ptr() as *const c_char,
        UNWIND_HINT_TYPE_REGS_PARTIAL => b"regs (partial)\0".as_ptr() as *const c_char,
        _ => b"?\0".as_ptr() as *const c_char,
    }
}

unsafe fn print_reg(reg: c_uint, offset: c_int) {
    unsafe {
        if reg == ORC_REG_UNDEFINED {
            printf(b" (und) \0".as_ptr() as *const c_char);
        } else {
            printf(
                b"%s + %3d\0".as_ptr() as *const c_char,
                reg_name(reg),
                offset,
            );
        }
    }
}

pub unsafe extern "C" fn orc_print_dump(
    dummy_elf: *mut elf,
    orc: *mut orc_entry,
    i: c_int,
) {
    unsafe {
        let _ = dummy_elf;
        printf(
            b"type:%s\0".as_ptr() as *const c_char,
            orc_type_name((*orc.add(i as usize)).type_),
        );

        printf(b" sp:\0".as_ptr() as *const c_char);
        print_reg(
            (*orc.add(i as usize)).sp_reg,
            (*orc.add(i as usize)).sp_offset,
        );

        printf(b" fp:\0".as_ptr() as *const c_char);
        print_reg(
            (*orc.add(i as usize)).fp_reg,
            (*orc.add(i as usize)).fp_offset,
        );

        printf(b" ra:\0".as_ptr() as *const c_char);
        print_reg(
            (*orc.add(i as usize)).ra_reg,
            (*orc.add(i as usize)).ra_offset,
        );

        printf(
            b" signal:%d\n\0".as_ptr() as *const c_char,
            (*orc.add(i as usize)).signal,
        );
    }
}
