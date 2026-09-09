// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/probes/kprobes/actions-thumb.c
 *
 * Copyright (C) 2011 Jon Medhurst <tixy@yxit.co.uk>.
 */

// Kernel and decoder dependencies are supplied by the surrounding crate.

/* These emulation encodings are functionally equivalent... */
// #define t32_emulate_rd8rn16rm0ra12_noflags \
//         t32_emulate_rdlo12rdhi8rn16rm0_noflags

/* t32 thumb actions */

unsafe fn t32_simulate_table_branch(insn: probes_opcode_t, _asi: *mut arch_probes_insn, regs: *mut pt_regs) {
    let pc = (*regs).ARM_pc;
    let rn = ((insn >> 16) & 0xf) as usize;
    let rm = (insn & 0xf) as usize;
    let rnv = if rn == 15 { pc } else { (*regs).uregs[rn] };
    let rmv = (*regs).uregs[rm];
    let halfwords: u32;
    if insn & 0x10 != 0 { // TBH
        halfwords = *((rnv as *const u16).add(rmv as usize)) as u32;
    } else { // TBB
        halfwords = *((rnv as *const u8).add(rmv as usize)) as u32;
    }
    (*regs).ARM_pc = pc.wrapping_add(2 * halfwords as usize);
}

unsafe fn t32_simulate_mrs(insn: probes_opcode_t, _asi: *mut arch_probes_insn, regs: *mut pt_regs) {
    let rd = ((insn >> 8) & 0xf) as usize;
    let mask: unsigned_long = 0xf8ff03df; /* Mask out execution state */
    (*regs).uregs[rd] = (*regs).ARM_cpsr & mask;
}

unsafe fn t32_simulate_cond_branch(insn: probes_opcode_t, _asi: *mut arch_probes_insn, regs: *mut pt_regs) {
    let pc = (*regs).ARM_pc;
    let mut offset = (insn & 0x7ff) as long; // imm11
    offset += ((insn & 0x003f0000) >> 5) as long; // imm6
    offset += ((insn & 0x00002000) << 4) as long; // J1
    offset += ((insn & 0x00000800) << 7) as long; // J2
    offset -= ((insn & 0x04000000) >> 7) as long; // Apply sign bit
    (*regs).ARM_pc = pc.wrapping_add((offset * 2) as usize);
}

unsafe fn t32_decode_cond_branch(insn: probes_opcode_t, asi: *mut arch_probes_insn, _d: *const decode_header) -> probes_insn {
    let cc = ((insn >> 22) & 0xf) as usize;
    (*asi).insn_check_cc = probes_condition_checks[cc];
    (*asi).insn_handler = Some(t32_simulate_cond_branch);
    INSN_GOOD_NO_SLOT
}

unsafe fn t32_simulate_branch(insn: probes_opcode_t, _asi: *mut arch_probes_insn, regs: *mut pt_regs) {
    let mut pc = (*regs).ARM_pc;
    let mut offset = (insn & 0x7ff) as long; // imm11
    offset += ((insn & 0x03ff0000) >> 5) as long; // imm10
    offset += ((insn & 0x00002000) << 9) as long; // J1
    offset += ((insn & 0x00000800) << 10) as long; // J2
    if insn & 0x04000000 != 0 { offset -= 0x00800000; } else { offset ^= 0x00600000; }
    if insn & (1 << 14) != 0 {
        (*regs).ARM_lr = (*regs).ARM_pc | 1;
        if insn & (1 << 12) == 0 {
            (*regs).ARM_cpsr &= !PSR_T_BIT;
            pc &= !3;
        }
    }
    (*regs).ARM_pc = pc.wrapping_add((offset * 2) as usize);
}

unsafe fn t32_simulate_ldr_literal(insn: probes_opcode_t, _asi: *mut arch_probes_insn, regs: *mut pt_regs) {
    let mut addr = (*regs).ARM_pc & !3;
    let rt = ((insn >> 12) & 0xf) as usize;
    let offset = (insn & 0xfff) as usize;
    if insn & 0x00800000 != 0 { addr = addr.wrapping_add(offset); } else { addr = addr.wrapping_sub(offset); }
    let rtv: unsigned_long;
    if insn & 0x00400000 != 0 { // LDR
        rtv = *(addr as *const unsigned_long);
        if rt == 15 { bx_write_pc(rtv, regs); return; }
    } else if insn & 0x00200000 != 0 { // LDRH
        rtv = if insn & 0x01000000 != 0 { *(addr as *const i16) as unsigned_long } else { *(addr as *const u16) as unsigned_long };
    } else { // LDRB
        rtv = if insn & 0x01000000 != 0 { *(addr as *const i8) as unsigned_long } else { *(addr as *const u8) as unsigned_long };
    }
    (*regs).uregs[rt] = rtv;
}

unsafe fn t32_decode_ldmstm(insn: probes_opcode_t, asi: *mut arch_probes_insn, d: *const decode_header) -> probes_insn {
    let ret = kprobe_decode_ldmstm(insn, asi, d);
    let insn = __mem_to_opcode_arm((*asi).insn[0]);
    *((*asi).insn.as_mut_ptr() as *mut u16) = __opcode_to_mem_thumb16(insn >> 16);
    *((*asi).insn.as_mut_ptr().add(1) as *mut u16) = __opcode_to_mem_thumb16(insn & 0xffff);
    ret
}

// The following handlers retain the original ARM inline-assembly operations.
// The surrounding ARM target supplies the assembly ABI and external symbols.
unsafe fn t32_emulate_ldrdstrd(insn: probes_opcode_t, asi: *mut arch_probes_insn, regs: *mut pt_regs) {
    let pc = (*regs).ARM_pc & !3;
    let rt1 = ((insn >> 12) & 0xf) as usize; let rt2 = ((insn >> 8) & 0xf) as usize;
    let rn = ((insn >> 16) & 0xf) as usize;
    let mut rt1v = (*regs).uregs[rt1]; let mut rt2v = (*regs).uregs[rt2];
    let mut rnv = if rn == 15 { pc } else { (*regs).uregs[rn] };
    asm!("blx {fnptr}", fnptr = in(reg) (*asi).insn_fn, inout("r0") rt1v, inout("r1") rt2v, inout("r2") rnv, clobber_abi("C"));
    if rn != 15 { (*regs).uregs[rn] = rnv; } (*regs).uregs[rt1] = rt1v; (*regs).uregs[rt2] = rt2v;
}

unsafe fn t32_emulate_ldrstr(insn: probes_opcode_t, asi: *mut arch_probes_insn, regs: *mut pt_regs) {
    let rt = ((insn >> 12) & 0xf) as usize; let rn = ((insn >> 16) & 0xf) as usize; let rm = (insn & 0xf) as usize;
    let mut rtv = (*regs).uregs[rt]; let mut rnv = (*regs).uregs[rn]; let rmv = (*regs).uregs[rm];
    asm!("blx {fnptr}", fnptr = in(reg) (*asi).insn_fn, inout("r0") rtv, inout("r2") rnv, in("r3") rmv, clobber_abi("C"));
    (*regs).uregs[rn] = rnv; if rt == 15 { bx_write_pc(rtv, regs); } else { (*regs).uregs[rt] = rtv; }
}

unsafe fn t32_emulate_rd8rn16rm0_rwflags(insn: probes_opcode_t, asi: *mut arch_probes_insn, regs: *mut pt_regs) {
    let rd = ((insn >> 8) & 0xf) as usize; let rn = ((insn >> 16) & 0xf) as usize; let rm = (insn & 0xf) as usize;
    let mut rdv = (*regs).uregs[rd]; let rnv = (*regs).uregs[rn]; let rmv = (*regs).uregs[rm]; let mut cpsr = (*regs).ARM_cpsr;
    asm!("msr cpsr_fs, {cpsr}\n\tblx {fnptr}\n\tmrs {cpsr}, cpsr", fnptr = in(reg) (*asi).insn_fn, inout("r1") rdv, in("r2") rnv, in("r3") rmv, inout(reg) cpsr, clobber_abi("C"));
    (*regs).uregs[rd] = rdv; (*regs).ARM_cpsr = ((*regs).ARM_cpsr & !APSR_MASK) | (cpsr & APSR_MASK);
}

unsafe fn t32_emulate_rd8pc16_noflags(insn: probes_opcode_t, asi: *mut arch_probes_insn, regs: *mut pt_regs) {
    let rd = ((insn >> 8) & 0xf) as usize; let mut rdv = (*regs).uregs[rd]; let rnv = (*regs).ARM_pc & !3;
    asm!("blx {fnptr}", fnptr = in(reg) (*asi).insn_fn, inout("r1") rdv, in("r2") rnv, clobber_abi("C")); (*regs).uregs[rd] = rdv;
}
unsafe fn t32_emulate_rd8rn16_noflags(insn: probes_opcode_t, asi: *mut arch_probes_insn, regs: *mut pt_regs) {
    let rd = ((insn >> 8) & 0xf) as usize; let rn = ((insn >> 16) & 0xf) as usize; let mut rdv = (*regs).uregs[rd]; let rnv = (*regs).uregs[rn];
    asm!("blx {fnptr}", fnptr = in(reg) (*asi).insn_fn, inout("r1") rdv, in("r2") rnv, clobber_abi("C")); (*regs).uregs[rd] = rdv;
}
unsafe fn t32_emulate_rdlo12rdhi8rn16rm0_noflags(insn: probes_opcode_t, asi: *mut arch_probes_insn, regs: *mut pt_regs) {
    let rdlo = ((insn >> 12) & 0xf) as usize; let rdhi = ((insn >> 8) & 0xf) as usize; let rn = ((insn >> 16) & 0xf) as usize; let rm = (insn & 0xf) as usize;
    let mut rdlov = (*regs).uregs[rdlo]; let mut rdhiv = (*regs).uregs[rdhi]; let rnv = (*regs).uregs[rn]; let rmv = (*regs).uregs[rm];
    asm!("blx {fnptr}", fnptr = in(reg) (*asi).insn_fn, inout("r0") rdlov, inout("r1") rdhiv, in("r2") rnv, in("r3") rmv, clobber_abi("C"));
    (*regs).uregs[rdlo] = rdlov; (*regs).uregs[rdhi] = rdhiv;
}

/* t16 thumb actions */
unsafe fn t16_simulate_bxblx(insn: probes_opcode_t, _asi: *mut arch_probes_insn, regs: *mut pt_regs) { let pc = (*regs).ARM_pc + 2; let rm = ((insn >> 3) & 0xf) as usize; let rmv = if rm == 15 { pc } else { (*regs).uregs[rm] }; if insn & (1 << 7) != 0 { (*regs).ARM_lr = (*regs).ARM_pc | 1; } bx_write_pc(rmv, regs); }
unsafe fn t16_simulate_ldr_literal(insn: probes_opcode_t, _asi: *mut arch_probes_insn, regs: *mut pt_regs) { let base = (((*regs).ARM_pc + 2) & !3) as *const unsigned_long; let index = (insn & 0xff) as usize; let rt = ((insn >> 8) & 7) as usize; (*regs).uregs[rt] = *base.add(index); }
unsafe fn t16_simulate_ldrstr_sp_relative(insn: probes_opcode_t, _asi: *mut arch_probes_insn, regs: *mut pt_regs) { let base = (*regs).ARM_sp as *mut unsigned_long; let index = (insn & 0xff) as usize; let rt = ((insn >> 8) & 7) as usize; if insn & 0x800 != 0 { (*regs).uregs[rt] = *base.add(index); } else { *base.add(index) = (*regs).uregs[rt]; } }
unsafe fn t16_simulate_reladr(insn: probes_opcode_t, _asi: *mut arch_probes_insn, regs: *mut pt_regs) { let base = if insn & 0x800 != 0 { (*regs).ARM_sp } else { ((*regs).ARM_pc + 2) & !3 }; let offset = (insn & 0xff) as usize; let rt = ((insn >> 8) & 7) as usize; (*regs).uregs[rt] = base + offset * 4; }
unsafe fn t16_simulate_add_sp_imm(insn: probes_opcode_t, _asi: *mut arch_probes_insn, regs: *mut pt_regs) { let imm = (insn & 0x7f) as usize; if insn & 0x80 != 0 { (*regs).ARM_sp -= imm * 4; } else { (*regs).ARM_sp += imm * 4; } }
unsafe fn t16_simulate_cbz(insn: probes_opcode_t, _asi: *mut arch_probes_insn, regs: *mut pt_regs) { let rn = (insn & 7) as usize; let nonzero = if (*regs).uregs[rn] != 0 { insn } else { !insn }; if nonzero & 0x800 != 0 { let i = insn & 0x200; let imm5 = insn & 0xf8; (*regs).ARM_pc = (*regs).ARM_pc + 2 + (i >> 3) as usize + (imm5 >> 2) as usize; } }
unsafe fn t16_simulate_it(insn: probes_opcode_t, _asi: *mut arch_probes_insn, regs: *mut pt_regs) { let mut cpsr = (*regs).ARM_cpsr & !PSR_IT_MASK; cpsr |= (insn & 0xfc) << 8; cpsr |= (insn & 3) << 25; (*regs).ARM_cpsr = cpsr; }
unsafe fn t16_singlestep_it(insn: probes_opcode_t, asi: *mut arch_probes_insn, regs: *mut pt_regs) { (*regs).ARM_pc += 2; t16_simulate_it(insn, asi, regs); }
unsafe fn t16_decode_it(_insn: probes_opcode_t, asi: *mut arch_probes_insn, _d: *const decode_header) -> probes_insn { (*asi).insn_singlestep = Some(t16_singlestep_it); INSN_GOOD_NO_SLOT }
unsafe fn t16_simulate_cond_branch(insn: probes_opcode_t, _asi: *mut arch_probes_insn, regs: *mut pt_regs) { let pc = (*regs).ARM_pc + 2; let mut offset = (insn & 0x7f) as long; offset -= (insn & 0x80) as long; (*regs).ARM_pc = pc.wrapping_add((offset * 2) as usize); }
unsafe fn t16_decode_cond_branch(insn: probes_opcode_t, asi: *mut arch_probes_insn, _d: *const decode_header) -> probes_insn { (*asi).insn_check_cc = probes_condition_checks[((insn >> 8) & 0xf) as usize]; (*asi).insn_handler = Some(t16_simulate_cond_branch); INSN_GOOD_NO_SLOT }
unsafe fn t16_simulate_branch(insn: probes_opcode_t, _asi: *mut arch_probes_insn, regs: *mut pt_regs) { let pc = (*regs).ARM_pc + 2; let mut offset = (insn & 0x3ff) as long; offset -= (insn & 0x400) as long; (*regs).ARM_pc = pc.wrapping_add((offset * 2) as usize); }

unsafe fn t16_emulate_loregs(insn: probes_opcode_t, asi: *mut arch_probes_insn, regs: *mut pt_regs) -> unsigned_long { let oldcpsr = (*regs).ARM_cpsr; let mut newcpsr: unsigned_long; asm!("msr cpsr_fs, {old}\n\tmov r11, r7\n\tldmia {regs}, {{r0-r7}}\n\tblx {fnptr}\n\tstmia {regs}, {{r0-r7}}\n\tmov r7, r11\n\tmrs {new}, cpsr", old = in(reg) oldcpsr, regs = in(reg) regs, fnptr = in(reg) (*asi).insn_fn, new = lateout(reg) newcpsr, clobber_abi("C")); (oldcpsr & !APSR_MASK) | (newcpsr & APSR_MASK) }
unsafe fn t16_emulate_loregs_rwflags(insn: probes_opcode_t, asi: *mut arch_probes_insn, regs: *mut pt_regs) { (*regs).ARM_cpsr = t16_emulate_loregs(insn, asi, regs); }
unsafe fn t16_emulate_loregs_noitrwflags(insn: probes_opcode_t, asi: *mut arch_probes_insn, regs: *mut pt_regs) { let cpsr = t16_emulate_loregs(insn, asi, regs); if !in_it_block(cpsr) { (*regs).ARM_cpsr = cpsr; } }
unsafe fn t16_emulate_hiregs(insn: probes_opcode_t, asi: *mut arch_probes_insn, regs: *mut pt_regs) { let pc = (*regs).ARM_pc + 2; let rdn = ((insn & 7) | ((insn & 0x80) >> 4)) as usize; let rm = ((insn >> 3) & 0xf) as usize; let mut rdnv = if rdn == 15 { pc } else { (*regs).uregs[rdn] }; let rmv = if rm == 15 { pc } else { (*regs).uregs[rm] }; let mut cpsr = (*regs).ARM_cpsr; asm!("msr cpsr_fs, {cpsr}\n\tblx {fnptr}\n\tmrs {cpsr}, cpsr", fnptr = in(reg) (*asi).insn_fn, inout(reg) rdnv, in(reg) rmv, inout(reg) cpsr, clobber_abi("C")); if rdn == 15 { rdnv &= !1; } (*regs).uregs[rdn] = rdnv; (*regs).ARM_cpsr = ((*regs).ARM_cpsr & !APSR_MASK) | (cpsr & APSR_MASK); }
unsafe fn t16_decode_hiregs(insn: probes_opcode_t, asi: *mut arch_probes_insn, _d: *const decode_header) -> probes_insn { let insn = (insn & !0x00ff) | 1; *((*asi).insn.as_mut_ptr() as *mut u16) = __opcode_to_mem_thumb16(insn); (*asi).insn_handler = Some(t16_emulate_hiregs); INSN_GOOD }

// PUSH/POP emulation uses the same ARM inline assembly sequences as the C source.
unsafe fn t16_emulate_push(insn: probes_opcode_t, asi: *mut arch_probes_insn, regs: *mut pt_regs) { asm!("mov r11, r7\n\tldr r9, [{regs}, #13*4]\n\tldr r8, [{regs}, #14*4]\n\tldmia {regs}, {{r0-r7}}\n\tblx {fnptr}\n\tstr r9, [{regs}, #13*4]\n\tmov r7, r11", regs = in(reg) regs, fnptr = in(reg) (*asi).insn_fn, clobber_abi("C")); }
unsafe fn t16_decode_push(insn: probes_opcode_t, asi: *mut arch_probes_insn, _d: *const decode_header) -> probes_insn { *((*asi).insn.as_mut_ptr() as *mut u16) = __opcode_to_mem_thumb16(0xe929); *((*asi).insn.as_mut_ptr().add(1) as *mut u16) = __opcode_to_mem_thumb16(insn & 0x1ff); (*asi).insn_handler = Some(t16_emulate_push); INSN_GOOD }
unsafe fn t16_emulate_pop_nopc(insn: probes_opcode_t, asi: *mut arch_probes_insn, regs: *mut pt_regs) { asm!("mov r11, r7\n\tldr r9, [{regs}, #13*4]\n\tldmia {regs}, {{r0-r7}}\n\tblx {fnptr}\n\tstmia {regs}, {{r0-r7}}\n\tstr r9, [{regs}, #13*4]\n\tmov r7, r11", regs = in(reg) regs, fnptr = in(reg) (*asi).insn_fn, clobber_abi("C")); }
unsafe fn t16_emulate_pop_pc(insn: probes_opcode_t, asi: *mut arch_probes_insn, regs: *mut pt_regs) { let pc: unsigned_long; asm!("mov r11, r7\n\tldr r9, [{regs}, #13*4]\n\tldmia {regs}, {{r0-r7}}\n\tblx {fnptr}\n\tstmia {regs}, {{r0-r7}}\n\tstr r9, [{regs}, #13*4]\n\tmov r7, r11", regs = in(reg) regs, fnptr = in(reg) (*asi).insn_fn, lateout("r8") pc, clobber_abi("C")); bx_write_pc(pc, regs); }
unsafe fn t16_decode_pop(insn: probes_opcode_t, asi: *mut arch_probes_insn, _d: *const decode_header) -> probes_insn { *((*asi).insn.as_mut_ptr() as *mut u16) = __opcode_to_mem_thumb16(0xe8b9); *((*asi).insn.as_mut_ptr().add(1) as *mut u16) = __opcode_to_mem_thumb16(insn & 0x1ff); (*asi).insn_handler = if insn & 0x100 != 0 { Some(t16_emulate_pop_pc) } else { Some(t16_emulate_pop_nopc) }; INSN_GOOD }

// The indexed union tables preserve the original externally visible action layout.
pub static kprobes_t16_actions: [decode_action; NUM_PROBES_T16_ACTIONS] = [
    decode_action::handler(PROBES_T16_ADD_SP, t16_simulate_add_sp_imm), decode_action::handler(PROBES_T16_CBZ, t16_simulate_cbz), decode_action::handler(PROBES_T16_SIGN_EXTEND, t16_emulate_loregs_rwflags), decode_action::decoder(PROBES_T16_PUSH, t16_decode_push), decode_action::decoder(PROBES_T16_POP, t16_decode_pop), decode_action::handler(PROBES_T16_SEV, probes_emulate_none), decode_action::handler(PROBES_T16_WFE, probes_simulate_nop), decode_action::decoder(PROBES_T16_IT, t16_decode_it), decode_action::handler(PROBES_T16_CMP, t16_emulate_loregs_rwflags), decode_action::handler(PROBES_T16_ADDSUB, t16_emulate_loregs_noitrwflags), decode_action::handler(PROBES_T16_LOGICAL, t16_emulate_loregs_noitrwflags), decode_action::handler(PROBES_T16_LDR_LIT, t16_simulate_ldr_literal), decode_action::handler(PROBES_T16_BLX, t16_simulate_bxblx), decode_action::decoder(PROBES_T16_HIREGOPS, t16_decode_hiregs), decode_action::handler(PROBES_T16_LDRHSTRH, t16_emulate_loregs_rwflags), decode_action::handler(PROBES_T16_LDRSTR, t16_simulate_ldrstr_sp_relative), decode_action::handler(PROBES_T16_ADR, t16_simulate_reladr), decode_action::handler(PROBES_T16_LDMSTM, t16_emulate_loregs_rwflags), decode_action::decoder(PROBES_T16_BRANCH_COND, t16_decode_cond_branch), decode_action::handler(PROBES_T16_BRANCH, t16_simulate_branch),
];

pub static kprobes_t32_actions: [decode_action; NUM_PROBES_T32_ACTIONS] = [
    decode_action::decoder(PROBES_T32_LDMSTM, t32_decode_ldmstm), decode_action::handler(PROBES_T32_LDRDSTRD, t32_emulate_ldrdstrd), decode_action::handler(PROBES_T32_TABLE_BRANCH, t32_simulate_table_branch), decode_action::handler(PROBES_T32_TST, t32_emulate_rd8rn16rm0_rwflags), decode_action::handler(PROBES_T32_MOV, t32_emulate_rd8rn16rm0_rwflags), decode_action::handler(PROBES_T32_ADDSUB, t32_emulate_rd8rn16rm0_rwflags), decode_action::handler(PROBES_T32_LOGICAL, t32_emulate_rd8rn16rm0_rwflags), decode_action::handler(PROBES_T32_CMP, t32_emulate_rd8rn16rm0_rwflags), decode_action::handler(PROBES_T32_ADDWSUBW_PC, t32_emulate_rd8pc16_noflags), decode_action::handler(PROBES_T32_ADDWSUBW, t32_emulate_rd8rn16_noflags), decode_action::handler(PROBES_T32_MOVW, t32_emulate_rd8rn16_noflags), decode_action::handler(PROBES_T32_SAT, t32_emulate_rd8rn16rm0_rwflags), decode_action::handler(PROBES_T32_BITFIELD, t32_emulate_rd8rn16_noflags), decode_action::handler(PROBES_T32_SEV, probes_emulate_none), decode_action::handler(PROBES_T32_WFE, probes_simulate_nop), decode_action::handler(PROBES_T32_MRS, t32_simulate_mrs), decode_action::decoder(PROBES_T32_BRANCH_COND, t32_decode_cond_branch), decode_action::handler(PROBES_T32_BRANCH, t32_simulate_branch), decode_action::handler(PROBES_T32_PLDI, probes_simulate_nop), decode_action::handler(PROBES_T32_LDR_LIT, t32_simulate_ldr_literal), decode_action::handler(PROBES_T32_LDRSTR, t32_emulate_ldrstr), decode_action::handler(PROBES_T32_SIGN_EXTEND, t32_emulate_rd8rn16rm0_rwflags), decode_action::handler(PROBES_T32_MEDIA, t32_emulate_rd8rn16rm0_rwflags), decode_action::handler(PROBES_T32_REVERSE, t32_emulate_rd8rn16_noflags), decode_action::handler(PROBES_T32_MUL_ADD, t32_emulate_rd8rn16rm0_rwflags), decode_action::handler(PROBES_T32_MUL_ADD2, t32_emulate_rdlo12rdhi8rn16rm0_noflags), decode_action::handler(PROBES_T32_MUL_ADD_LONG, t32_emulate_rdlo12rdhi8rn16rm0_noflags),
];

pub static kprobes_t32_checkers: [*const decode_checker; 2] = [t32_stack_checker, core::ptr::null()];
pub static kprobes_t16_checkers: [*const decode_checker; 2] = [t16_stack_checker, core::ptr::null()];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
