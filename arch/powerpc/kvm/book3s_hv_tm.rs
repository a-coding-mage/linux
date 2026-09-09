// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2017 Paul Mackerras, IBM Corp. <paulus@au1.ibm.com>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

unsafe fn emulate_tx_failure(vcpu: *mut kvm_vcpu, failure_cause: u64) {
    let mut texasr: u64;
    let mut tfiar: u64;
    let msr: u64 = (*vcpu).arch.shregs.msr;

    tfiar = (*vcpu).arch.regs.nip & !0x3u64;
    texasr = (failure_cause << 56) | TEXASR_ABORT | TEXASR_FS | TEXASR_EXACT;
    if MSR_TM_SUSPENDED((*vcpu).arch.shregs.msr) {
        texasr |= TEXASR_SUSP;
    }
    if msr & MSR_PR != 0 {
        texasr |= TEXASR_PR;
        tfiar |= 1;
    }
    (*vcpu).arch.tfiar = tfiar;
    /* Preserve ROT and TL fields of existing TEXASR */
    (*vcpu).arch.texasr = ((*vcpu).arch.texasr & 0x3ffffff) | texasr;
}

/*
 * This gets called on a softpatch interrupt on POWER9 DD2.2 processors.
 * We expect to find a TM-related instruction to be emulated.  The
 * instruction image is in vcpu->arch.emul_inst.  If the guest was in
 * TM suspended or transactional state, the checkpointed state has been
 * reclaimed and is in the vcpu struct.  The CPU is in virtual mode in
 * host context.
 */
unsafe fn kvmhv_p9_tm_emulation(vcpu: *mut kvm_vcpu) -> i32 {
    let instr: u32 = (*vcpu).arch.emul_inst;
    let mut msr: u64 = (*vcpu).arch.shregs.msr;
    let mut newmsr: u64;
    let mut bescr: u64;
    let mut ra: i32;
    let mut rs: i32;

    /* The softpatch interrupt points after the faulting instruction. */
    (*vcpu).arch.regs.nip = (*vcpu).arch.regs.nip.wrapping_sub(4);

    match instr & PO_XOP_OPCODE_MASK {
        PPC_INST_RFID => {
            newmsr = (*vcpu).arch.shregs.srr1;
            WARN_ON_ONCE(!(MSR_TM_SUSPENDED(msr) && MSR_TM_TRANSACTIONAL(newmsr) && (newmsr & MSR_TM != 0)));
            newmsr = sanitize_msr(newmsr);
            (*vcpu).arch.shregs.msr = newmsr;
            (*vcpu).arch.cfar = (*vcpu).arch.regs.nip;
            (*vcpu).arch.regs.nip = (*vcpu).arch.shregs.srr0;
            return RESUME_GUEST;
        }
        PPC_INST_RFEBB => {
            if msr & MSR_PR != 0 && (*vcpu).arch.vcore.pcr & PCR_ARCH_206 != 0 {
                kvmppc_core_queue_program(vcpu, SRR1_PROGILL);
                return RESUME_GUEST;
            }
            if (*vcpu).arch.hfscr & HFSCR_EBB == 0 {
                (*vcpu).arch.hfscr &= !HFSCR_INTR_CAUSE;
                (*vcpu).arch.hfscr |= (FSCR_EBB_LG as u64) << 56;
                (*vcpu).arch.trap = BOOK3S_INTERRUPT_H_FAC_UNAVAIL;
                return -1;
            }
            if msr & MSR_PR != 0 && (*vcpu).arch.fscr & FSCR_EBB == 0 {
                (*vcpu).arch.fscr &= !FSCR_INTR_CAUSE;
                (*vcpu).arch.fscr |= (FSCR_EBB_LG as u64) << 56;
                kvmppc_book3s_queue_irqprio(vcpu, BOOK3S_INTERRUPT_FAC_UNAVAIL);
                return RESUME_GUEST;
            }
            bescr = (*vcpu).arch.bescr;
            WARN_ON_ONCE(!(MSR_TM_SUSPENDED(msr) && ((bescr >> 30) & 3) == 2));
            bescr &= !BESCR_GE;
            if instr & (1 << 11) != 0 { bescr |= BESCR_GE; }
            (*vcpu).arch.bescr = bescr;
            msr = (msr & !MSR_TS_MASK) | MSR_TS_T;
            (*vcpu).arch.shregs.msr = msr;
            (*vcpu).arch.cfar = (*vcpu).arch.regs.nip;
            (*vcpu).arch.regs.nip = (*vcpu).arch.ebbrr;
            return RESUME_GUEST;
        }
        PPC_INST_MTMSRD => {
            rs = ((instr >> 21) & 0x1f) as i32;
            newmsr = kvmppc_get_gpr(vcpu, rs);
            WARN_ON_ONCE(!(MSR_TM_SUSPENDED(msr) && MSR_TM_TRANSACTIONAL(newmsr) && (newmsr & MSR_TM != 0)));
            newmsr = (newmsr & !MSR_LE) | (msr & MSR_LE);
            newmsr = sanitize_msr(newmsr);
            (*vcpu).arch.shregs.msr = newmsr;
            (*vcpu).arch.regs.nip = (*vcpu).arch.regs.nip.wrapping_add(4);
            return RESUME_GUEST;
        }
        (PPC_INST_TSR & PO_XOP_OPCODE_MASK) => {
            if msr & MSR_PR != 0 && (*vcpu).arch.vcore.pcr & PCR_ARCH_206 != 0 {
                kvmppc_core_queue_program(vcpu, SRR1_PROGILL); return RESUME_GUEST;
            }
            if (*vcpu).arch.hfscr & HFSCR_TM == 0 {
                (*vcpu).arch.hfscr &= !HFSCR_INTR_CAUSE; (*vcpu).arch.hfscr |= (FSCR_TM_LG as u64) << 56;
                (*vcpu).arch.trap = BOOK3S_INTERRUPT_H_FAC_UNAVAIL; return -1;
            }
            if msr & MSR_TM == 0 {
                (*vcpu).arch.fscr &= !FSCR_INTR_CAUSE; (*vcpu).arch.fscr |= (FSCR_TM_LG as u64) << 56;
                kvmppc_book3s_queue_irqprio(vcpu, BOOK3S_INTERRUPT_FAC_UNAVAIL); return RESUME_GUEST;
            }
            (*vcpu).arch.regs.ccr = ((*vcpu).arch.regs.ccr & 0x0fffffff) | (((msr & MSR_TS_MASK) >> MSR_TS_S_LG) << 29);
            if instr & (1 << 21) != 0 { if MSR_TM_SUSPENDED(msr) { msr = (msr & !MSR_TS_MASK) | MSR_TS_T; } }
            else if MSR_TM_TRANSACTIONAL(msr) { msr = (msr & !MSR_TS_MASK) | MSR_TS_S; }
            (*vcpu).arch.shregs.msr = msr; (*vcpu).arch.regs.nip = (*vcpu).arch.regs.nip.wrapping_add(4); return RESUME_GUEST;
        }
        (PPC_INST_TRECLAIM & PO_XOP_OPCODE_MASK) => {
            if (*vcpu).arch.hfscr & HFSCR_TM == 0 {
                (*vcpu).arch.hfscr &= !HFSCR_INTR_CAUSE; (*vcpu).arch.hfscr |= (FSCR_TM_LG as u64) << 56;
                (*vcpu).arch.trap = BOOK3S_INTERRUPT_H_FAC_UNAVAIL; return -1;
            }
            if msr & MSR_TM == 0 { (*vcpu).arch.fscr &= !FSCR_INTR_CAUSE; (*vcpu).arch.fscr |= (FSCR_TM_LG as u64) << 56; kvmppc_book3s_queue_irqprio(vcpu, BOOK3S_INTERRUPT_FAC_UNAVAIL); return RESUME_GUEST; }
            if !MSR_TM_ACTIVE(msr) { kvmppc_core_queue_program(vcpu, SRR1_PROGTM); return RESUME_GUEST; }
            if (*vcpu).arch.orig_texasr & TEXASR_FS == 0 { ra = ((instr >> 16) & 0x1f) as i32; if ra != 0 { ra = (kvmppc_get_gpr(vcpu, ra) & 0xff) as i32; } emulate_tx_failure(vcpu, ra as u64); }
            copy_from_checkpoint(vcpu);
            (*vcpu).arch.regs.ccr = ((*vcpu).arch.regs.ccr & 0x0fffffff) | (((msr & MSR_TS_MASK) >> MSR_TS_S_LG) << 29);
            (*vcpu).arch.shregs.msr &= !MSR_TS_MASK; (*vcpu).arch.regs.nip = (*vcpu).arch.regs.nip.wrapping_add(4); return RESUME_GUEST;
        }
        (PPC_INST_TRECHKPT & PO_XOP_OPCODE_MASK) => {
            if (*vcpu).arch.hfscr & HFSCR_TM == 0 { (*vcpu).arch.hfscr &= !HFSCR_INTR_CAUSE; (*vcpu).arch.hfscr |= (FSCR_TM_LG as u64) << 56; (*vcpu).arch.trap = BOOK3S_INTERRUPT_H_FAC_UNAVAIL; return -1; }
            if msr & MSR_TM == 0 { (*vcpu).arch.fscr &= !FSCR_INTR_CAUSE; (*vcpu).arch.fscr |= (FSCR_TM_LG as u64) << 56; kvmppc_book3s_queue_irqprio(vcpu, BOOK3S_INTERRUPT_FAC_UNAVAIL); return RESUME_GUEST; }
            if MSR_TM_ACTIVE(msr) || (*vcpu).arch.texasr & TEXASR_FS == 0 { kvmppc_core_queue_program(vcpu, SRR1_PROGTM); return RESUME_GUEST; }
            copy_to_checkpoint(vcpu);
            (*vcpu).arch.regs.ccr = ((*vcpu).arch.regs.ccr & 0x0fffffff) | (((msr & MSR_TS_MASK) >> MSR_TS_S_LG) << 29);
            (*vcpu).arch.shregs.msr = msr | MSR_TS_S; (*vcpu).arch.regs.nip = (*vcpu).arch.regs.nip.wrapping_add(4); return RESUME_GUEST;
        }
        _ => {}
    }
    kvmppc_core_queue_program(vcpu, SRR1_PROGILL);
    pr_warn_ratelimited!("Unrecognized TM-related instruction %#x for emulation", instr);
    RESUME_GUEST
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
