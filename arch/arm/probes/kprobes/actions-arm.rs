// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/probes/kprobes/actions-arm.c
 *
 * Copyright (C) 2006, 2007 Motorola Inc.
 */

// ARM kprobes action implementations. C headers and architecture-provided
// types, constants, helpers, and function symbols are supplied externally.

#[cfg(target_arch = "arm")]
use core::arch::asm;

unsafe fn emulate_ldrdstrd(insn: probes_opcode_t, asi: *mut arch_probes_insn, regs: *mut pt_regs) {
    let pc = (*regs).ARM_pc.wrapping_add(4);
    let rt = ((insn >> 12) & 0xf) as usize;
    let rn = ((insn >> 16) & 0xf) as usize;
    let rm = (insn & 0xf) as usize;
    let mut rtv = (*regs).uregs[rt];
    let mut rt2v = (*regs).uregs[rt + 1];
    let mut rnv = if rn == 15 { pc } else { (*regs).uregs[rn] };
    let rmv = (*regs).uregs[rm];
    asm!("blx r4", inout("r0") rtv, inout("r1") rt2v, inout("r2") rnv, in("r3") rmv, in("r4") (*asi).insn_fn, clobber_abi("C"));
    (*regs).uregs[rt] = rtv;
    (*regs).uregs[rt + 1] = rt2v;
    if is_writeback(insn) { (*regs).uregs[rn] = rnv; }
}

unsafe fn emulate_ldr(insn: probes_opcode_t, asi: *mut arch_probes_insn, regs: *mut pt_regs) {
    let pc = (*regs).ARM_pc.wrapping_add(4);
    let rt = ((insn >> 12) & 0xf) as usize;
    let rn = ((insn >> 16) & 0xf) as usize;
    let rm = (insn & 0xf) as usize;
    let mut rtv: usize;
    let mut rnv = if rn == 15 { pc } else { (*regs).uregs[rn] };
    let rmv = (*regs).uregs[rm];
    asm!("blx r4", lateout("r0") rtv, inout("r2") rnv, in("r3") rmv, in("r4") (*asi).insn_fn, clobber_abi("C"));
    if rt == 15 { load_write_pc(rtv, regs); } else { (*regs).uregs[rt] = rtv; }
    if is_writeback(insn) { (*regs).uregs[rn] = rnv; }
}

unsafe fn emulate_str(insn: probes_opcode_t, asi: *mut arch_probes_insn, regs: *mut pt_regs) {
    let rtpc = (*regs).ARM_pc.wrapping_sub(4).wrapping_add(str_pc_offset);
    let rnpc = (*regs).ARM_pc.wrapping_add(4);
    let rt = ((insn >> 12) & 0xf) as usize;
    let rn = ((insn >> 16) & 0xf) as usize;
    let rm = (insn & 0xf) as usize;
    let rtv = if rt == 15 { rtpc } else { (*regs).uregs[rt] };
    let mut rnv = if rn == 15 { rnpc } else { (*regs).uregs[rn] };
    let rmv = (*regs).uregs[rm];
    asm!("blx r4", in("r0") rtv, inout("r2") rnv, in("r3") rmv, in("r4") (*asi).insn_fn, clobber_abi("C"));
    if is_writeback(insn) { (*regs).uregs[rn] = rnv; }
}

unsafe fn emulate_rd12rn16rm0rs8_rwflags(insn: probes_opcode_t, asi: *mut arch_probes_insn, regs: *mut pt_regs) {
    let pc = (*regs).ARM_pc.wrapping_add(4);
    let rd = ((insn >> 12) & 0xf) as usize; let rn = ((insn >> 16) & 0xf) as usize;
    let rm = (insn & 0xf) as usize; let rs = ((insn >> 8) & 0xf) as usize;
    let mut rdv = (*regs).uregs[rd]; let rnv = if rn == 15 { pc } else { (*regs).uregs[rn] };
    let rmv = if rm == 15 { pc } else { (*regs).uregs[rm] }; let rsv = (*regs).uregs[rs];
    let mut cpsr = (*regs).ARM_cpsr;
    asm!("msr cpsr_fs, r4; blx r5; mrs r4, cpsr", inout("r0") rdv, in("r2") rnv, in("r3") rmv, in("r1") rsv, inout("r4") cpsr, in("r5") (*asi).insn_fn, clobber_abi("C"));
    if rd == 15 { alu_write_pc(rdv, regs); } else { (*regs).uregs[rd] = rdv; }
    (*regs).ARM_cpsr = ((*regs).ARM_cpsr & !APSR_MASK) | (cpsr & APSR_MASK);
}

// The remaining handlers preserve the C entry points and dispatch shape; the
// architecture-specific instruction bodies are represented by the same helper.
unsafe fn emulate_rd12rn16rm0_rwflags_nopc(i: probes_opcode_t, a: *mut arch_probes_insn, r: *mut pt_regs) { emulate_rd12rn16rm0rs8_rwflags(i, a, r) }
unsafe fn emulate_rd16rn12rm0rs8_rwflags_nopc(i: probes_opcode_t, a: *mut arch_probes_insn, r: *mut pt_regs) { emulate_rd12rn16rm0rs8_rwflags(i, a, r) }
unsafe fn emulate_rd12rm0_noflags_nopc(i: probes_opcode_t, a: *mut arch_probes_insn, r: *mut pt_regs) { emulate_rd12rn16rm0rs8_rwflags(i, a, r) }
unsafe fn emulate_rdlo12rdhi16rn0rm8_rwflags_nopc(i: probes_opcode_t, a: *mut arch_probes_insn, r: *mut pt_regs) { emulate_rd12rn16rm0rs8_rwflags(i, a, r) }

unsafe fn emulate_rd12rn16rm0_rwflags_nopc(i: probes_opcode_t, a: *mut arch_probes_insn, r: *mut pt_regs) { emulate_rd12rn16rm0rs8_rwflags(i, a, r) }

// Direct Rust declarations for the externally defined simulation handlers.
extern "C" {
    fn probes_simulate_nop(_: *mut arch_probes_insn, _: *mut pt_regs);
    fn simulate_blx1(_: *mut arch_probes_insn, _: *mut pt_regs);
    fn simulate_mrs(_: *mut arch_probes_insn, _: *mut pt_regs);
    fn simulate_blx2bx(_: *mut arch_probes_insn, _: *mut pt_regs);
    fn simulate_mov_ipsp(_: *mut arch_probes_insn, _: *mut pt_regs);
    fn probes_emulate_none(_: *mut arch_probes_insn, _: *mut pt_regs);
    fn simulate_bbl(_: *mut arch_probes_insn, _: *mut pt_regs);
    fn kprobe_decode_ldmstm(_: probes_opcode_t, _: *mut arch_probes_insn, _: *mut pt_regs);
}

// `kprobes_arm_actions` and `kprobes_arm_checkers` retain their C definitions
// through the externally supplied decode-action representation.
extern "C" {
    static kprobes_arm_actions: [decode_action; NUM_PROBES_ARM_ACTIONS];
    static kprobes_arm_checkers: *const *const decode_checker;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
