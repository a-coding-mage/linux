/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Disassemble s390 instructions.
 *
 * Copyright IBM Corp. 2007
 * Author(s): Martin Schwidefsky (schwidefsky@de.ibm.com),
 */

// Dependency intent: declarations from <asm/dis-defs.h> are supplied by the
// surrounding translation unit.

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct s390_insn {
    _private: [u8; 0],
}

#[inline]
pub fn insn_length(code: u8) -> i32 {
    ((((code as i32).wrapping_add(64)) >> 7).wrapping_add(1)) << 1
}

extern "C" {
    pub fn show_code(regs: *mut pt_regs);
    pub fn print_fn_code(code: *mut u8, len: c_ulong);
    pub fn find_insn(code: *mut u8) -> *mut s390_insn;
}

#[allow(non_camel_case_types)]
pub type c_ulong = usize;

#[inline]
pub unsafe fn is_known_insn(code: *mut u8) -> i32 {
    if find_insn(code).is_null() { 0 } else { 1 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
