// SPDX-License-Identifier: GPL-2.0
/*
 * Clang Control Flow Integrity (CFI) support.
 *
 * Copyright (C) 2023 Google LLC
 */

// Dependencies supplied by the kernel headers and architecture code.
use core::ffi::c_void;

extern "C" {
    fn get_kernel_nofault(dst: *mut u32, src: *const c_void) -> bool;
    fn riscv_insn_is_beq(insn: u32) -> bool;
    fn riscv_insn_is_jalr(insn: u32) -> bool;
    fn riscv_insn_is_c_jalr(insn: u32) -> bool;
    fn rv_extract_rs1_reg(insn: u32) -> usize;
    fn rvc_extract_c2_rs1_reg(insn: u32) -> usize;
    fn get_insn_length(insn: u32) -> usize;
    fn is_cfi_trap(epc: usize) -> bool;
    fn report_cfi_failure_noaddr(regs: *mut pt_regs, epc: usize) -> bug_trap_type;
    fn report_cfi_failure(
        regs: *mut pt_regs,
        epc: usize,
        target: *mut usize,
        type_: u32,
    ) -> bug_trap_type;
}

#[repr(C)]
pub struct pt_regs {
    pub epc: usize,
}

#[repr(C)]
pub enum bug_trap_type {}

const BUG_TRAP_TYPE_NONE: bug_trap_type = unsafe { core::mem::zeroed() };

/*
 * Returns the target address and the expected type when regs->epc points
 * to a compiler-generated CFI trap.
 */
unsafe fn decode_cfi_insn(regs: *mut pt_regs, target: *mut usize, type_: *mut u32) -> bool {
    let regs_ptr = regs as *mut usize;
    let mut rs1_num: usize;
    let mut insn: u32 = 0;

    *target = 0;
    *type_ = 0;

    /*
     * The compiler generates the following instruction sequence
     * for indirect call checks:
     *
     *   lw      t1, -4(<reg>)
     *   lui     t2, <hi20>
     *   addiw   t2, t2, <lo12>
     *   beq     t1, t2, .Ltmp1
     *   ebreak  ; <- regs->epc
     *   .Ltmp1:
     *   jalr    <reg>
     *
     * We can read the expected type and the target address from the
     * registers passed to the beq/jalr instructions.
     */
    if get_kernel_nofault(
        &mut insn,
        (regs as *mut u8).offset((*regs).epc as isize - 4) as *const c_void,
    ) {
        return false;
    }
    if !riscv_insn_is_beq(insn) {
        return false;
    }

    *type_ = *regs_ptr.add(rv_extract_rs1_reg(insn)) as u32;

    if get_kernel_nofault(
        &mut insn,
        (regs as *mut u8).add((*regs).epc) as *const c_void,
    ) || get_kernel_nofault(
        &mut insn,
        (regs as *mut u8).add((*regs).epc + get_insn_length(insn)) as *const c_void,
    ) {
        return false;
    }

    if riscv_insn_is_jalr(insn) {
        rs1_num = rv_extract_rs1_reg(insn);
    } else if riscv_insn_is_c_jalr(insn) {
        rs1_num = rvc_extract_c2_rs1_reg(insn);
    } else {
        return false;
    }

    *target = *regs_ptr.add(rs1_num);
    true
}

/*
 * Checks if the ebreak trap is because of a CFI failure, and handles the trap
 * if needed. Returns a bug_trap_type value similarly to report_bug.
 */
pub unsafe fn handle_cfi_failure(regs: *mut pt_regs) -> bug_trap_type {
    let mut target: usize = 0;
    let mut type_: u32 = 0;

    if !is_cfi_trap((*regs).epc) {
        return BUG_TRAP_TYPE_NONE;
    }

    if !decode_cfi_insn(regs, &mut target, &mut type_) {
        return report_cfi_failure_noaddr(regs, (*regs).epc);
    }

    report_cfi_failure(regs, (*regs).epc, &mut target, type_)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
