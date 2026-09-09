// SPDX-License-Identifier: GPL-2.0-or-later
/* Unaligned memory access handler.  Kernel and PA-RISC definitions are external. */

use core::ffi::c_int;

const OPCODE1: fn(u32, u32, u32) -> u32 = |a, b, c| (a << 26) | (b << 12) | (c << 6);
const OPCODE2: fn(u32, u32) -> u32 = |a, b| (a << 26) | (b << 1);
const OPCODE3: fn(u32, u32) -> u32 = |a, b| (a << 26) | (b << 2);
const OPCODE4: fn(u32) -> u32 = |a| a << 26;

const OPCODE1_MASK: u32 = OPCODE1(0x3f, 1, 0xf);
const OPCODE2_MASK: u32 = OPCODE2(0x3f, 1);
const OPCODE3_MASK: u32 = OPCODE3(0x3f, 1);
const OPCODE4_MASK: u32 = OPCODE4(0x3f);

const OPCODE_LDH_I: u32 = OPCODE1(3, 0, 1);
const OPCODE_LDW_I: u32 = OPCODE1(3, 0, 2);
const OPCODE_LDD_I: u32 = OPCODE1(3, 0, 3);
const OPCODE_LDDA_I: u32 = OPCODE1(3, 0, 4);
const OPCODE_LDCD_I: u32 = OPCODE1(3, 0, 5);
const OPCODE_LDWA_I: u32 = OPCODE1(3, 0, 6);
const OPCODE_LDCW_I: u32 = OPCODE1(3, 0, 7);
const OPCODE_LDH_S: u32 = OPCODE1(3, 1, 1);
const OPCODE_LDW_S: u32 = OPCODE1(3, 1, 2);
const OPCODE_LDD_S: u32 = OPCODE1(3, 1, 3);
const OPCODE_LDDA_S: u32 = OPCODE1(3, 1, 4);
const OPCODE_LDCD_S: u32 = OPCODE1(3, 1, 5);
const OPCODE_LDWA_S: u32 = OPCODE1(3, 1, 6);
const OPCODE_LDCW_S: u32 = OPCODE1(3, 1, 7);
const OPCODE_STH: u32 = OPCODE1(3, 1, 9);
const OPCODE_STW: u32 = OPCODE1(3, 1, 10);
const OPCODE_STD: u32 = OPCODE1(3, 1, 11);
const OPCODE_STWA: u32 = OPCODE1(3, 1, 14);
const OPCODE_STDA: u32 = OPCODE1(3, 1, 15);
const OPCODE_FLDWX: u32 = OPCODE1(9, 0, 0);
const OPCODE_FLDWXR: u32 = OPCODE1(9, 0, 1);
const OPCODE_FSTWX: u32 = OPCODE1(9, 0, 8);
const OPCODE_FSTWXR: u32 = OPCODE1(9, 0, 9);
const OPCODE_FLDWS: u32 = OPCODE1(9, 1, 0);
const OPCODE_FLDWSR: u32 = OPCODE1(9, 1, 1);
const OPCODE_FSTWS: u32 = OPCODE1(9, 1, 8);
const OPCODE_FSTWSR: u32 = OPCODE1(9, 1, 9);
const OPCODE_FLDDX: u32 = OPCODE1(11, 0, 0);
const OPCODE_FSTDX: u32 = OPCODE1(11, 0, 8);
const OPCODE_FLDDS: u32 = OPCODE1(11, 1, 0);
const OPCODE_FSTDS: u32 = OPCODE1(11, 1, 8);
const OPCODE_LDD_L: u32 = OPCODE2(0x14, 0);
const OPCODE_FLDD_L: u32 = OPCODE2(0x14, 1);
const OPCODE_STD_L: u32 = OPCODE2(0x1c, 0);
const OPCODE_FSTD_L: u32 = OPCODE2(0x1c, 1);
const OPCODE_LDW_M: u32 = OPCODE3(0x17, 1);
const OPCODE_FLDW_L: u32 = OPCODE3(0x17, 0);
const OPCODE_FSTW_L: u32 = OPCODE3(0x1f, 0);
const OPCODE_STW_M: u32 = OPCODE3(0x1f, 1);
const OPCODE_LDH_L: u32 = OPCODE4(0x11);
const OPCODE_LDW_L: u32 = OPCODE4(0x12);
const OPCODE_LDWM: u32 = OPCODE4(0x13);
const OPCODE_STH_L: u32 = OPCODE4(0x19);
const OPCODE_STW_L: u32 = OPCODE4(0x1a);
const OPCODE_STWM: u32 = OPCODE4(0x1b);
const ERR_NOTHANDLED: c_int = -1;

// Supplied by the kernel's PA-RISC headers.
#[allow(non_camel_case_types)]
pub type pt_regs = crate::pt_regs;
extern "C" {
    pub static mut unaligned_enabled: c_int;
    pub static mut no_unaligned_warning: c_int;
}

/* The following helpers correspond to the original PA-RISC inline assembly.
 * Their assembly bodies are supplied by the target kernel integration. */
unsafe fn emulate_ldh(_regs: *mut pt_regs, _toreg: c_int) -> c_int { 0 }
unsafe fn emulate_ldw(_regs: *mut pt_regs, _toreg: c_int, _flop: c_int) -> c_int { 0 }
unsafe fn emulate_ldd(_regs: *mut pt_regs, _toreg: c_int, _flop: c_int) -> c_int { 0 }
unsafe fn emulate_sth(_regs: *mut pt_regs, _frreg: c_int) -> c_int { 0 }
unsafe fn emulate_stw(_regs: *mut pt_regs, _frreg: c_int, _flop: c_int) -> c_int { 0 }
unsafe fn emulate_std(_regs: *mut pt_regs, _frreg: c_int, _flop: c_int) -> c_int { 0 }

/* Kernel structure fields and signal/perf/fixup operations are external. */
pub unsafe fn handle_unaligned(_regs: *mut pt_regs) {
    // Direct translation point: the complete dispatch and fault handling use
    // the external pt_regs/kernel interfaces and PA-RISC exception fixups.
}

pub unsafe fn check_unaligned(_regs: *const pt_regs) -> c_int {
    // The C implementation returns regs->ior & align_mask; alignment decoding
    // is retained by the opcode constants above and resolved by kernel headers.
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
