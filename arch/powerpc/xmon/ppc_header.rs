/* ppc.h -- Header file for PowerPC opcode table
   Copyright (C) 1994-2016 Free Software Foundation, Inc.
   Written by Ian Lance Taylor, Cygnus Support

This file is part of GDB, GAS, and the GNU binutils.

GDB, GAS, and the GNU binutils are free software; you can redistribute
them and/or modify them under the terms of the GNU General Public
License as published by the Free Software Foundation; either version
1, or (at your option) any later version.

This file is distributed in the hope that it will be useful, but WITHOUT
ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public
License for more details.

You should have received a copy of the GNU General Public License
along with this file; see the file COPYING.  If not, write to the Free
Software Foundation, 51 Franklin Street - Fifth Floor, Boston, MA 02110-1301, USA.  */

pub type ppc_cpu_t = u64;

/* The opcode table is an array of struct powerpc_opcode.  */
#[repr(C)]
pub struct powerpc_opcode {
    pub name: *const i8,
    pub opcode: libc::c_ulong,
    pub mask: libc::c_ulong,
    pub flags: ppc_cpu_t,
    pub deprecated: ppc_cpu_t,
    pub operands: [u8; 8],
}

extern "C" {
    pub static powerpc_opcodes: [powerpc_opcode; 0];
    pub static powerpc_num_opcodes: libc::c_int;
    pub static vle_opcodes: [powerpc_opcode; 0];
    pub static vle_num_opcodes: libc::c_int;
}

pub const PPC_OPCODE_PPC: ppc_cpu_t = 1;
pub const PPC_OPCODE_POWER: ppc_cpu_t = 2;
pub const PPC_OPCODE_POWER2: ppc_cpu_t = 4;
pub const PPC_OPCODE_601: ppc_cpu_t = 8;
pub const PPC_OPCODE_COMMON: ppc_cpu_t = 0x10;
pub const PPC_OPCODE_ANY: ppc_cpu_t = 0x20;
pub const PPC_OPCODE_64: ppc_cpu_t = 0x40;
pub const PPC_OPCODE_64_BRIDGE: ppc_cpu_t = 0x80;
pub const PPC_OPCODE_ALTIVEC: ppc_cpu_t = 0x100;
pub const PPC_OPCODE_403: ppc_cpu_t = 0x200;
pub const PPC_OPCODE_BOOKE: ppc_cpu_t = 0x400;
pub const PPC_OPCODE_440: ppc_cpu_t = 0x800;
pub const PPC_OPCODE_POWER4: ppc_cpu_t = 0x1000;
pub const PPC_OPCODE_POWER7: ppc_cpu_t = 0x2000;
pub const PPC_OPCODE_SPE: ppc_cpu_t = 0x4000;
pub const PPC_OPCODE_ISEL: ppc_cpu_t = 0x8000;
pub const PPC_OPCODE_EFS: ppc_cpu_t = 0x10000;
pub const PPC_OPCODE_BRLOCK: ppc_cpu_t = 0x20000;
pub const PPC_OPCODE_PMR: ppc_cpu_t = 0x40000;
pub const PPC_OPCODE_CACHELCK: ppc_cpu_t = 0x80000;
pub const PPC_OPCODE_RFMCI: ppc_cpu_t = 0x100000;
pub const PPC_OPCODE_POWER5: ppc_cpu_t = 0x200000;
pub const PPC_OPCODE_E300: ppc_cpu_t = 0x400000;
pub const PPC_OPCODE_POWER6: ppc_cpu_t = 0x800000;
pub const PPC_OPCODE_CELL: ppc_cpu_t = 0x1000000;
pub const PPC_OPCODE_PPCPS: ppc_cpu_t = 0x2000000;
pub const PPC_OPCODE_E500MC: ppc_cpu_t = 0x4000000;
pub const PPC_OPCODE_405: ppc_cpu_t = 0x8000000;
pub const PPC_OPCODE_VSX: ppc_cpu_t = 0x10000000;
pub const PPC_OPCODE_A2: ppc_cpu_t = 0x20000000;
pub const PPC_OPCODE_476: ppc_cpu_t = 0x40000000;
pub const PPC_OPCODE_TITAN: ppc_cpu_t = 0x80000000;
pub const PPC_OPCODE_E500: ppc_cpu_t = 0x100000000;
pub const PPC_OPCODE_ALTIVEC2: ppc_cpu_t = 0x200000000;
pub const PPC_OPCODE_E6500: ppc_cpu_t = 0x400000000;
pub const PPC_OPCODE_TMR: ppc_cpu_t = 0x800000000;
pub const PPC_OPCODE_VLE: ppc_cpu_t = 0x1000000000;
pub const PPC_OPCODE_POWER8: ppc_cpu_t = 0x2000000000;
pub const PPC_OPCODE_HTM: ppc_cpu_t = PPC_OPCODE_POWER8;
pub const PPC_OPCODE_750: ppc_cpu_t = 0x4000000000;
pub const PPC_OPCODE_7450: ppc_cpu_t = 0x8000000000;
pub const PPC_OPCODE_860: ppc_cpu_t = 0x10000000000;
pub const PPC_OPCODE_POWER9: ppc_cpu_t = 0x20000000000;
pub const PPC_OPCODE_VSX3: ppc_cpu_t = 0x40000000000;
pub const PPC_OPCODE_E200Z4: ppc_cpu_t = 0x80000000000;

#[inline]
pub const fn PPC_OP(i: u64) -> u64 { (i >> 26) & 0x3f }
#[inline]
pub const fn PPC_OP_SE_VLE(m: u64) -> bool { m <= 0xffff }
#[inline]
pub const fn VLE_OP(i: u64, m: u64) -> u64 { (i >> if m <= 0xffff { 10 } else { 26 }) & 0x3f }
#[inline]
pub const fn VLE_OP_TO_SEG(i: u64) -> u64 { i >> 1 }

#[repr(C)]
pub struct powerpc_operand {
    pub bitm: libc::c_uint,
    pub shift: libc::c_int,
    pub insert: Option<unsafe extern "C" fn(libc::c_ulong, libc::c_long, ppc_cpu_t, *mut *const i8) -> libc::c_ulong>,
    pub extract: Option<unsafe extern "C" fn(libc::c_ulong, ppc_cpu_t, *mut libc::c_int) -> libc::c_long>,
    pub flags: libc::c_ulong,
}

extern "C" {
    pub static powerpc_operands: [powerpc_operand; 0];
    pub static num_powerpc_operands: libc::c_uint;
}

pub const PPC_OPSHIFT_INV: libc::c_uint = 0x80000000;
pub const PPC_OPERAND_SIGNED: libc::c_ulong = 0x1;
pub const PPC_OPERAND_SIGNOPT: libc::c_ulong = 0x2;
pub const PPC_OPERAND_FAKE: libc::c_ulong = 0x4;
pub const PPC_OPERAND_PARENS: libc::c_ulong = 0x8;
pub const PPC_OPERAND_CR_BIT: libc::c_ulong = 0x10;
pub const PPC_OPERAND_GPR: libc::c_ulong = 0x20;
pub const PPC_OPERAND_GPR_0: libc::c_ulong = 0x40;
pub const PPC_OPERAND_FPR: libc::c_ulong = 0x80;
pub const PPC_OPERAND_RELATIVE: libc::c_ulong = 0x100;
pub const PPC_OPERAND_ABSOLUTE: libc::c_ulong = 0x200;
pub const PPC_OPERAND_OPTIONAL: libc::c_ulong = 0x400;
pub const PPC_OPERAND_NEXT: libc::c_ulong = 0x800;
pub const PPC_OPERAND_NEGATIVE: libc::c_ulong = 0x1000;
pub const PPC_OPERAND_VR: libc::c_ulong = 0x2000;
pub const PPC_OPERAND_DS: libc::c_ulong = 0x4000;
pub const PPC_OPERAND_DQ: libc::c_ulong = 0x8000;
pub const PPC_OPERAND_PLUS1: libc::c_ulong = 0x10000;
pub const PPC_OPERAND_FSL: libc::c_ulong = 0x20000;
pub const PPC_OPERAND_FCR: libc::c_ulong = 0x40000;
pub const PPC_OPERAND_UDI: libc::c_ulong = 0x80000;
pub const PPC_OPERAND_VSR: libc::c_ulong = 0x100000;
pub const PPC_OPERAND_CR_REG: libc::c_ulong = 0x200000;
pub const PPC_OPERAND_OPTIONAL_VALUE: libc::c_ulong = 0x400000;
pub const PPC_OPERAND_OPTIONAL32: libc::c_ulong = 0x800000;

#[repr(C)]
pub struct powerpc_macro {
    pub name: *const i8,
    pub operands: libc::c_uint,
    pub flags: ppc_cpu_t,
    pub format: *const i8,
}

extern "C" {
    pub static powerpc_macros: [powerpc_macro; 0];
    pub static powerpc_num_macros: libc::c_int;
}

#[inline]
pub unsafe fn ppc_optional_operand_value(operand: *const powerpc_operand) -> libc::c_long {
    if ((*operand).flags & PPC_OPERAND_OPTIONAL_VALUE) != 0 {
        (*operand.add(1)).shift as libc::c_long
    } else {
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
