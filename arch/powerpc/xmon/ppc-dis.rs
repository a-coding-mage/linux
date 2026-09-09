// SPDX-License-Identifier: GPL-2.0-or-later
/* ppc-dis.c -- Disassemble PowerPC instructions
   Copyright (C) 1994-2016 Free Software Foundation, Inc.
   Written by Ian Lance Taylor, Cygnus Support

This file is part of GDB, GAS, and the GNU binutils.
 */

// Dependencies supplied by the surrounding PowerPC disassembler implementation.

use core::ffi::c_void;

pub type ppc_cpu_t = u32;

#[repr(C)]
pub struct powerpc_operand {
    pub extract: Option<unsafe extern "C" fn(u64, ppc_cpu_t, *mut i32) -> i64>,
    pub shift: i32,
    pub bitm: u64,
    pub flags: u32,
}

#[repr(C)]
pub struct powerpc_opcode {
    pub mask: u64,
    pub opcode: u64,
    pub flags: u32,
    pub deprecated: u32,
    pub operands: *const u8,
    pub name: *const core::ffi::c_char,
}

extern "C" {
    static powerpc_operands: powerpc_operand;
    static powerpc_opcodes: powerpc_opcode;
    static powerpc_num_opcodes: usize;

    fn ppc_optional_operand_value(operand: *const powerpc_operand) -> i64;
    fn cpu_has_feature(feature: u32) -> bool;
    fn print_address(address: u64);
    fn printf(format: *const core::ffi::c_char, ...);
}

// Constants and feature predicates are provided by the corresponding headers.
const PPC_OPERAND_SIGNED: u32 = 1 << 0;
const PPC_OPERAND_NEXT: u32 = 1 << 1;
const PPC_OPERAND_OPTIONAL: u32 = 1 << 2;
const PPC_OPERAND_FAKE: u32 = 1 << 3;
const PPC_OPERAND_GPR: u32 = 1 << 4;
const PPC_OPERAND_GPR_0: u32 = 1 << 5;
const PPC_OPERAND_FPR: u32 = 1 << 6;
const PPC_OPERAND_VR: u32 = 1 << 7;
const PPC_OPERAND_VSR: u32 = 1 << 8;
const PPC_OPERAND_RELATIVE: u32 = 1 << 9;
const PPC_OPERAND_ABSOLUTE: u32 = 1 << 10;
const PPC_OPERAND_FSL: u32 = 1 << 11;
const PPC_OPERAND_FCR: u32 = 1 << 12;
const PPC_OPERAND_UDI: u32 = 1 << 13;
const PPC_OPERAND_CR_REG: u32 = 1 << 14;
const PPC_OPERAND_CR_BIT: u32 = 1 << 15;
const PPC_OPERAND_PARENS: u32 = 1 << 16;

const PPC_OPCODE_PPC: u32 = 1 << 0;
const PPC_OPCODE_COMMON: u32 = 1 << 1;
const PPC_OPCODE_64: u32 = 1 << 2;
const PPC_OPCODE_POWER4: u32 = 1 << 3;
const PPC_OPCODE_CELL: u32 = 1 << 4;
const PPC_OPCODE_POWER5: u32 = 1 << 5;
const PPC_OPCODE_POWER6: u32 = 1 << 6;
const PPC_OPCODE_POWER7: u32 = 1 << 7;
const PPC_OPCODE_POWER8: u32 = 1 << 8;
const PPC_OPCODE_POWER9: u32 = 1 << 9;
const PPC_OPCODE_HTM: u32 = 1 << 10;
const PPC_OPCODE_ALTIVEC: u32 = 1 << 11;
const PPC_OPCODE_ALTIVEC2: u32 = 1 << 12;
const PPC_OPCODE_VSX: u32 = 1 << 13;
const PPC_OPCODE_VSX3: u32 = 1 << 14;
const PPC_OPCODE_ANY: u32 = 1 << 15;
const PPC_OPCODE_VLE: u32 = 1 << 16;
const CPU_FTR_TM: u32 = 0;
const CPU_FTR_ALTIVEC: u32 = 0;
const CPU_FTR_VSX: u32 = 0;

unsafe fn operand_value_powerpc(operand: *const powerpc_operand, insn: u64, dialect: ppc_cpu_t) -> i64 {
    let mut invalid: i32 = 0;
    let value: i64;
    if let Some(extract) = (*operand).extract {
        value = extract(insn, dialect, &mut invalid);
    } else {
        let raw = if (*operand).shift >= 0 {
            (insn >> (*operand).shift) & (*operand).bitm
        } else {
            (insn << (-(*operand).shift)) & (*operand).bitm
        };
        if ((*operand).flags & PPC_OPERAND_SIGNED) != 0 {
            let mut top = (*operand).bitm;
            top |= (top & top.wrapping_neg()).wrapping_sub(1);
            top &= !(top >> 1);
            value = ((raw ^ top).wrapping_sub(top)) as i64;
        } else {
            value = raw as i64;
        }
    }
    value
}

unsafe fn skip_optional_operands(mut opindex: *const u8, insn: u64, dialect: ppc_cpu_t) -> i32 {
    while *opindex != 0 {
        let operand = &powerpc_operands.add(*opindex as usize);
        if (operand.flags & PPC_OPERAND_NEXT) != 0
            || ((operand.flags & PPC_OPERAND_OPTIONAL) != 0
                && operand_value_powerpc(operand, insn, dialect) != ppc_optional_operand_value(operand))
        {
            return 0;
        }
        opindex = opindex.add(1);
    }
    1
}

unsafe fn lookup_powerpc(insn: u64, dialect: ppc_cpu_t) -> *const powerpc_opcode {
    let mut opcode = &powerpc_opcodes as *const powerpc_opcode;
    let opcode_end = opcode.add(powerpc_num_opcodes);
    while opcode < opcode_end {
        let current = &*opcode;
        if (insn & current.mask) == current.opcode
            && (dialect == u32::MAX
                || ((current.flags & dialect) != 0 && (current.deprecated & dialect) == 0))
        {
            let mut invalid = 0;
            let mut opindex = current.operands;
            while *opindex != 0 {
                let operand = &powerpc_operands.add(*opindex as usize);
                if let Some(extract) = operand.extract { extract(insn, dialect, &mut invalid); }
                opindex = opindex.add(1);
            }
            if invalid == 0 { return opcode; }
        }
        opcode = opcode.add(1);
    }
    core::ptr::null()
}

pub unsafe fn print_insn_powerpc(mut insn: u64, mut memaddr: u64) -> i32 {
    let mut dialect = PPC_OPCODE_PPC | PPC_OPCODE_COMMON;
    let mut opcode: *const powerpc_opcode = core::ptr::null();
    let insn_is_short = false;
    if cpu_has_feature(CPU_FTR_TM) { dialect |= PPC_OPCODE_HTM; }
    if cpu_has_feature(CPU_FTR_ALTIVEC) { dialect |= PPC_OPCODE_ALTIVEC | PPC_OPCODE_ALTIVEC2; }
    if cpu_has_feature(CPU_FTR_VSX) { dialect |= PPC_OPCODE_VSX | PPC_OPCODE_VSX3; }
    if opcode.is_null() { opcode = lookup_powerpc(insn, dialect); }
    if opcode.is_null() && (dialect & PPC_OPCODE_ANY) != 0 { opcode = lookup_powerpc(insn, u32::MAX); }
    if !opcode.is_null() {
        let op = &*opcode;
        let mut opindex = op.operands;
        let mut need_comma = false;
        let mut need_paren = false;
        let mut skip_optional = -1;
        if *opindex != 0 { printf(b"%-7s \0".as_ptr() as _, op.name); } else { printf(b"%s\0".as_ptr() as _, op.name); }
        if insn_is_short { insn >>= 16; }
        while *opindex != 0 {
            let operand = &powerpc_operands.add(*opindex as usize);
            opindex = opindex.add(1);
            if (operand.flags & PPC_OPERAND_FAKE) != 0 { continue; }
            if (operand.flags & PPC_OPERAND_OPTIONAL) != 0 {
                if skip_optional < 0 { skip_optional = skip_optional_operands(opindex.sub(1), insn, dialect); }
                if skip_optional != 0 { continue; }
            }
            let value = operand_value_powerpc(operand, insn, dialect);
            if need_comma { printf(b",\0".as_ptr() as _); need_comma = false; }
            if (operand.flags & (PPC_OPERAND_GPR | PPC_OPERAND_GPR_0)) != 0 { printf(b"r%ld\0".as_ptr() as _, value); }
            else if (operand.flags & PPC_OPERAND_FPR) != 0 { printf(b"f%ld\0".as_ptr() as _, value); }
            else if (operand.flags & PPC_OPERAND_VR) != 0 { printf(b"v%ld\0".as_ptr() as _, value); }
            else if (operand.flags & PPC_OPERAND_VSR) != 0 { printf(b"vs%ld\0".as_ptr() as _, value); }
            else if (operand.flags & PPC_OPERAND_RELATIVE) != 0 { print_address(memaddr.wrapping_add(value as u64)); }
            else if (operand.flags & PPC_OPERAND_ABSOLUTE) != 0 { print_address((value as u64) & 0xffff_ffff); }
            else { printf(b"%d\0".as_ptr() as _, value as i32); }
            if need_paren { printf(b")\0".as_ptr() as _); need_paren = false; }
            if (operand.flags & PPC_OPERAND_PARENS) == 0 { need_comma = true; } else { printf(b"(\0".as_ptr() as _); need_paren = true; }
        }
        if insn_is_short { memaddr += 2; 2 } else { 4 }
    } else { printf(b".long 0x%lx\0".as_ptr() as _, insn); 4 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
