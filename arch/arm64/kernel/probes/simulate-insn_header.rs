/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/arm64/kernel/probes/simulate-insn.h
 *
 * Copyright (C) 2013 Linaro Limited
 */

// C header guard: _ARM_KERNEL_KPROBES_SIMULATE_INSN_H

use core::ffi::c_long;

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn simulate_adr_adrp(opcode: u32, addr: c_long, regs: *mut pt_regs);
    pub fn simulate_b_bl(opcode: u32, addr: c_long, regs: *mut pt_regs);
    pub fn simulate_b_cond(opcode: u32, addr: c_long, regs: *mut pt_regs);
    pub fn simulate_br_blr(opcode: u32, addr: c_long, regs: *mut pt_regs);
    pub fn simulate_ret(opcode: u32, addr: c_long, regs: *mut pt_regs);
    pub fn simulate_cbz_cbnz(opcode: u32, addr: c_long, regs: *mut pt_regs);
    pub fn simulate_tbz_tbnz(opcode: u32, addr: c_long, regs: *mut pt_regs);
    pub fn simulate_ldr_literal(opcode: u32, addr: c_long, regs: *mut pt_regs);
    pub fn simulate_ldrsw_literal(opcode: u32, addr: c_long, regs: *mut pt_regs);
    pub fn simulate_nop(opcode: u32, addr: c_long, regs: *mut pt_regs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
