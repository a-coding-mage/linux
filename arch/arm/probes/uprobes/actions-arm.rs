// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 Rabin Vincent <rabin at rab.in>
 */

// Kernel and local header dependencies are supplied by the surrounding build.

unsafe fn uprobes_substitute_pc(pinsn: *mut ::core::ffi::c_ulong, oregs: u32) -> i32 {
    let mut insn: probes_opcode_t = __mem_to_opcode_arm(*pinsn);
    let mut temp: probes_opcode_t;
    let mut mask: probes_opcode_t;
    let mut freereg: i32;
    let mut free: u32 = 0xffff;
    let mut regs: u32;

    regs = oregs;
    while regs != 0 {
        if (regs & 0xf) != REG_TYPE_NONE {
            free &= !(1u32 << (insn & 0xf));
        }
        regs >>= 4;
        insn >>= 4;
    }

    /* No PC, no problem */
    if (free & (1 << 15)) != 0 {
        return 15;
    }
    if free == 0 {
        return -1;
    }

    /*
     * fls instead of ffs ensures that for "ldrd r0, r1, [pc]" we would
     * pick LR instead of R1.
     */
    freereg = fls(free) - 1;
    free = freereg as u32;

    temp = __mem_to_opcode_arm(*pinsn);
    insn = temp;
    regs = oregs;
    mask = 0xf;

    while regs != 0 {
        if (regs & 0xf) != REG_TYPE_NONE && (temp & 0xf) == 15 {
            insn &= !mask;
            insn |= free & mask;
        }
        regs >>= 4;
        mask <<= 4;
        free <<= 4;
        temp >>= 4;
    }

    *pinsn = __opcode_to_mem_arm(insn);
    freereg
}

unsafe fn uprobe_set_pc(auprobe: *mut struct_arch_uprobe,
                        autask: *mut struct_arch_uprobe_task,
                        regs: *mut struct_pt_regs) {
    let pcreg = (*auprobe).pcreg;
    (*autask).backup = (*regs).uregs[pcreg as usize];
    (*regs).uregs[pcreg as usize] = (*regs).ARM_pc + 8;
}

unsafe fn uprobe_unset_pc(auprobe: *mut struct_arch_uprobe,
                          autask: *mut struct_arch_uprobe_task,
                          regs: *mut struct_pt_regs) {
    /* PC will be taken care of by common code */
    (*regs).uregs[(*auprobe).pcreg as usize] = (*autask).backup;
}

unsafe fn uprobe_aluwrite_pc(auprobe: *mut struct_arch_uprobe,
                             autask: *mut struct_arch_uprobe_task,
                             regs: *mut struct_pt_regs) {
    let pcreg = (*auprobe).pcreg;
    alu_write_pc((*regs).uregs[pcreg as usize], regs);
    (*regs).uregs[pcreg as usize] = (*autask).backup;
}

unsafe fn uprobe_write_pc(auprobe: *mut struct_arch_uprobe,
                          autask: *mut struct_arch_uprobe_task,
                          regs: *mut struct_pt_regs) {
    let pcreg = (*auprobe).pcreg;
    load_write_pc((*regs).uregs[pcreg as usize], regs);
    (*regs).uregs[pcreg as usize] = (*autask).backup;
}

unsafe fn decode_pc_ro(insn: probes_opcode_t, asi: *mut struct_arch_probes_insn,
                       d: *const struct_decode_header) -> probes_insn {
    let auprobe = container_of_arch_uprobe(asi);
    let decode = d as *const struct_decode_emulate;
    let regs = (*decode).header.type_regs.bits >> DECODE_TYPE_BITS;
    let reg = uprobes_substitute_pc(&mut (*auprobe).ixol[0], regs);
    if reg == 15 { return INSN_GOOD; }
    if reg == -1 { return INSN_REJECTED; }
    (*auprobe).pcreg = reg as u32;
    (*auprobe).prehandler = Some(uprobe_set_pc);
    (*auprobe).posthandler = Some(uprobe_unset_pc);
    INSN_GOOD
}

unsafe fn decode_wb_pc(insn: probes_opcode_t, asi: *mut struct_arch_probes_insn,
                       d: *const struct_decode_header, alu: bool) -> probes_insn {
    let auprobe = container_of_arch_uprobe(asi);
    let ret = decode_pc_ro(insn, asi, d);
    if ((insn >> 12) & 0xf) == 15 {
        (*auprobe).posthandler = if alu { Some(uprobe_aluwrite_pc) } else { Some(uprobe_write_pc) };
    }
    ret
}

unsafe fn decode_rd12rn16rm0rs8_rwflags(insn: probes_opcode_t, asi: *mut struct_arch_probes_insn,
                                        d: *const struct_decode_header) -> probes_insn {
    decode_wb_pc(insn, asi, d, true)
}

unsafe fn decode_ldr(insn: probes_opcode_t, asi: *mut struct_arch_probes_insn,
                     d: *const struct_decode_header) -> probes_insn {
    decode_wb_pc(insn, asi, d, false)
}

unsafe fn uprobe_decode_ldmstm(insn: probes_opcode_t, asi: *mut struct_arch_probes_insn,
                               _d: *const struct_decode_header) -> probes_insn {
    let auprobe = container_of_arch_uprobe(asi);
    let reglist = insn & 0xffff;
    let rn = (insn >> 16) & 0xf;
    let lbit = insn & (1 << 20);
    let used = reglist | (1 << rn);
    if rn == 15 { return INSN_REJECTED; }
    if (used & (1 << 15)) == 0 { return INSN_GOOD; }
    if (used & (1 << 14)) != 0 { return INSN_REJECTED; }
    let insn = insn ^ 0xc000;
    (*auprobe).pcreg = 14;
    (*auprobe).ixol[0] = __opcode_to_mem_arm(insn);
    (*auprobe).prehandler = Some(uprobe_set_pc);
    (*auprobe).posthandler = if lbit != 0 { Some(uprobe_write_pc) } else { Some(uprobe_unset_pc) };
    INSN_GOOD
}

// The indexed action table is provided using the surrounding kernel-compatible types.
// C designated initializers (the union fields are supplied by the surrounding bindings):
// PROBES_PRELOAD_IMM.handler = probes_simulate_nop
// PROBES_PRELOAD_REG.handler = probes_simulate_nop
// PROBES_BRANCH_IMM.handler = simulate_blx1
// PROBES_MRS.handler = simulate_mrs
// PROBES_BRANCH_REG.handler = simulate_blx2bx
// PROBES_CLZ.handler = probes_simulate_nop
// PROBES_SATURATING_ARITHMETIC.handler = probes_simulate_nop
// PROBES_MUL1.handler = probes_simulate_nop
// PROBES_MUL2.handler = probes_simulate_nop
// PROBES_SWP.handler = probes_simulate_nop
// PROBES_LDRSTRD.decoder = decode_pc_ro
// PROBES_LOAD_EXTRA.decoder = decode_pc_ro
// PROBES_LOAD.decoder = decode_ldr
// PROBES_STORE_EXTRA.decoder = decode_pc_ro
// PROBES_STORE.decoder = decode_pc_ro
// PROBES_MOV_IP_SP.handler = simulate_mov_ipsp
// PROBES_DATA_PROCESSING_REG.decoder = decode_rd12rn16rm0rs8_rwflags
// PROBES_DATA_PROCESSING_IMM.decoder = decode_rd12rn16rm0rs8_rwflags
// PROBES_MOV_HALFWORD.handler = probes_simulate_nop
// PROBES_SEV.handler = probes_simulate_nop
// PROBES_WFE.handler = probes_simulate_nop
// PROBES_SATURATE.handler = probes_simulate_nop
// PROBES_REV.handler = probes_simulate_nop
// PROBES_MMI.handler = probes_simulate_nop
// PROBES_PACK.handler = probes_simulate_nop
// PROBES_EXTEND.handler = probes_simulate_nop
// PROBES_EXTEND_ADD.handler = probes_simulate_nop
// PROBES_MUL_ADD_LONG.handler = probes_simulate_nop
// PROBES_MUL_ADD.handler = probes_simulate_nop
// PROBES_BITFIELD.handler = probes_simulate_nop
// PROBES_BRANCH.handler = simulate_bbl
// PROBES_LDMSTM.decoder = uprobe_decode_ldmstm
pub static uprobes_probes_actions: [union_decode_action; PROBES_LDMSTM + 1] = [
    union_decode_action::default(),
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
