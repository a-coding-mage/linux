// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright IBM Corp. 2008
 * Copyright 2011 Freescale Semiconductor, Inc.
 *
 * Authors: Hollis Blanchard <hollisb@us.ibm.com>
 */

// Dependencies supplied by the surrounding kernel translation unit.

const OP_19_XOP_RFI: u32 = 50;
const OP_19_XOP_RFCI: u32 = 51;
const OP_19_XOP_RFDI: u32 = 39;
const OP_31_XOP_MFMSR: u32 = 83;
const OP_31_XOP_WRTEE: u32 = 131;
const OP_31_XOP_MTMSR: u32 = 146;
const OP_31_XOP_WRTEEI: u32 = 163;

unsafe fn kvmppc_emul_rfi(vcpu: *mut kvm_vcpu) {
    (*vcpu).arch.regs.nip = (*(*vcpu).arch.shared).srr0;
    kvmppc_set_msr(vcpu, (*(*vcpu).arch.shared).srr1);
}

unsafe fn kvmppc_emul_rfdi(vcpu: *mut kvm_vcpu) {
    (*vcpu).arch.regs.nip = (*vcpu).arch.dsrr0;
    kvmppc_set_msr(vcpu, (*vcpu).arch.dsrr1);
}

unsafe fn kvmppc_emul_rfci(vcpu: *mut kvm_vcpu) {
    (*vcpu).arch.regs.nip = (*vcpu).arch.csrr0;
    kvmppc_set_msr(vcpu, (*vcpu).arch.csrr1);
}

pub unsafe fn kvmppc_booke_emulate_op(
    vcpu: *mut kvm_vcpu,
    inst: u32,
    advance: *mut i32,
) -> i32 {
    let mut emulated = EMULATE_DONE;
    let rs = get_rs(inst);
    let rt = get_rt(inst);

    match get_op(inst) {
        19 => match get_xop(inst) {
            OP_19_XOP_RFI => { kvmppc_emul_rfi(vcpu); kvmppc_set_exit_type(vcpu, EMULATED_RFI_EXITS); *advance = 0; }
            OP_19_XOP_RFCI => { kvmppc_emul_rfci(vcpu); kvmppc_set_exit_type(vcpu, EMULATED_RFCI_EXITS); *advance = 0; }
            OP_19_XOP_RFDI => { kvmppc_emul_rfdi(vcpu); kvmppc_set_exit_type(vcpu, EMULATED_RFDI_EXITS); *advance = 0; }
            _ => emulated = EMULATE_FAIL,
        },
        31 => match get_xop(inst) {
            OP_31_XOP_MFMSR => { kvmppc_set_gpr(vcpu, rt, (*(*vcpu).arch.shared).msr); kvmppc_set_exit_type(vcpu, EMULATED_MFMSR_EXITS); }
            OP_31_XOP_MTMSR => { kvmppc_set_exit_type(vcpu, EMULATED_MTMSR_EXITS); kvmppc_set_msr(vcpu, kvmppc_get_gpr(vcpu, rs)); }
            OP_31_XOP_WRTEE => { (*(*vcpu).arch.shared).msr = ((*(*vcpu).arch.shared).msr & !MSR_EE) | (kvmppc_get_gpr(vcpu, rs) & MSR_EE); kvmppc_set_exit_type(vcpu, EMULATED_WRTEE_EXITS); }
            OP_31_XOP_WRTEEI => { (*(*vcpu).arch.shared).msr = ((*(*vcpu).arch.shared).msr & !MSR_EE) | (inst as ulong & MSR_EE); kvmppc_set_exit_type(vcpu, EMULATED_WRTEE_EXITS); }
            _ => emulated = EMULATE_FAIL,
        },
        _ => emulated = EMULATE_FAIL,
    }
    emulated
}

/* NOTE: Some registers are backed by real registers in BOOKE_HV (GS-mode). */
pub unsafe fn kvmppc_booke_emulate_mtspr(vcpu: *mut kvm_vcpu, sprn: i32, mut spr_val: ulong) -> i32 {
    let mut emulated = EMULATE_DONE;
    let mut debug_inst = false;
    match sprn {
        SPRN_DEAR => (*(*vcpu).arch.shared).dar = spr_val,
        SPRN_ESR => (*(*vcpu).arch.shared).esr = spr_val,
        SPRN_CSRR0 => (*vcpu).arch.csrr0 = spr_val,
        SPRN_CSRR1 => (*vcpu).arch.csrr1 = spr_val,
        SPRN_DSRR0 => (*vcpu).arch.dsrr0 = spr_val,
        SPRN_DSRR1 => (*vcpu).arch.dsrr1 = spr_val,
        SPRN_IAC1 => { if !(*vcpu).guest_debug { debug_inst = true; (*vcpu).arch.dbg_reg.iac1 = spr_val; } }
        SPRN_IAC2 => { if !(*vcpu).guest_debug { debug_inst = true; (*vcpu).arch.dbg_reg.iac2 = spr_val; } }
        // CONFIG_PPC_ADV_DEBUG_IACS > 2
        SPRN_IAC3 => { if !(*vcpu).guest_debug { debug_inst = true; (*vcpu).arch.dbg_reg.iac3 = spr_val; } }
        SPRN_IAC4 => { if !(*vcpu).guest_debug { debug_inst = true; (*vcpu).arch.dbg_reg.iac4 = spr_val; } }
        SPRN_DAC1 => { if !(*vcpu).guest_debug { debug_inst = true; (*vcpu).arch.dbg_reg.dac1 = spr_val; } }
        SPRN_DAC2 => { if !(*vcpu).guest_debug { debug_inst = true; (*vcpu).arch.dbg_reg.dac2 = spr_val; } }
        SPRN_DBCR0 => { if !(*vcpu).guest_debug { debug_inst = true; spr_val &= DBCR0_IDM | DBCR0_IC | DBCR0_BT | DBCR0_TIE | DBCR0_IAC1 | DBCR0_IAC2 | DBCR0_IAC3 | DBCR0_IAC4 | DBCR0_DAC1R | DBCR0_DAC1W | DBCR0_DAC2R | DBCR0_DAC2W; (*vcpu).arch.dbg_reg.dbcr0 = spr_val; } }
        SPRN_DBCR1 => { if !(*vcpu).guest_debug { debug_inst = true; (*vcpu).arch.dbg_reg.dbcr1 = spr_val; } }
        SPRN_DBCR2 => { if !(*vcpu).guest_debug { debug_inst = true; (*vcpu).arch.dbg_reg.dbcr2 = spr_val; } }
        SPRN_DBSR => { if !(*vcpu).guest_debug { (*vcpu).arch.dbsr &= !spr_val; if !((*vcpu).arch.dbsr & !DBSR_IDE) { kvmppc_core_dequeue_debug(vcpu); } } }
        SPRN_TSR => kvmppc_clr_tsr_bits(vcpu, spr_val),
        SPRN_TCR => { if (*vcpu).arch.tcr & TCR_WRC_MASK != 0 { spr_val = (spr_val & !TCR_WRC_MASK) | ((*vcpu).arch.tcr & TCR_WRC_MASK); } kvmppc_set_tcr(vcpu, spr_val); }
        SPRN_DECAR => (*vcpu).arch.decar = spr_val,
        SPRN_SPRG4 => kvmppc_set_sprg4(vcpu, spr_val),
        SPRN_SPRG5 => kvmppc_set_sprg5(vcpu, spr_val),
        SPRN_SPRG6 => kvmppc_set_sprg6(vcpu, spr_val),
        SPRN_SPRG7 => kvmppc_set_sprg7(vcpu, spr_val),
        SPRN_IVPR => (*vcpu).arch.ivpr = spr_val,
        SPRN_IVOR0..=SPRN_IVOR15 => (*vcpu).arch.ivor[(sprn - SPRN_IVOR0) as usize] = spr_val,
        SPRN_MCSR => (*vcpu).arch.mcsr &= !spr_val,
        // CONFIG_64BIT: SPRN_EPCR => kvmppc_set_epcr(vcpu, spr_val),
        _ => emulated = EMULATE_FAIL,
    }
    if debug_inst { current.thread.debug = (*vcpu).arch.dbg_reg; switch_booke_debug_regs(&(*vcpu).arch.dbg_reg); }
    emulated
}

pub unsafe fn kvmppc_booke_emulate_mfspr(vcpu: *mut kvm_vcpu, sprn: i32, spr_val: *mut ulong) -> i32 {
    let mut emulated = EMULATE_DONE;
    match sprn {
        SPRN_IVPR => *spr_val = (*vcpu).arch.ivpr,
        SPRN_DEAR => *spr_val = (*(*vcpu).arch.shared).dar,
        SPRN_ESR => *spr_val = (*(*vcpu).arch.shared).esr,
        SPRN_EPR => *spr_val = (*vcpu).arch.epr,
        SPRN_CSRR0 => *spr_val = (*vcpu).arch.csrr0,
        SPRN_CSRR1 => *spr_val = (*vcpu).arch.csrr1,
        SPRN_DSRR0 => *spr_val = (*vcpu).arch.dsrr0,
        SPRN_DSRR1 => *spr_val = (*vcpu).arch.dsrr1,
        SPRN_IAC1 => *spr_val = (*vcpu).arch.dbg_reg.iac1,
        SPRN_IAC2 => *spr_val = (*vcpu).arch.dbg_reg.iac2,
        SPRN_IAC3 => *spr_val = (*vcpu).arch.dbg_reg.iac3,
        SPRN_IAC4 => *spr_val = (*vcpu).arch.dbg_reg.iac4,
        SPRN_DAC1 => *spr_val = (*vcpu).arch.dbg_reg.dac1,
        SPRN_DAC2 => *spr_val = (*vcpu).arch.dbg_reg.dac2,
        SPRN_DBCR0 => { *spr_val = (*vcpu).arch.dbg_reg.dbcr0; if (*vcpu).guest_debug { *spr_val |= DBCR0_EDM; } }
        SPRN_DBCR1 => *spr_val = (*vcpu).arch.dbg_reg.dbcr1,
        SPRN_DBCR2 => *spr_val = (*vcpu).arch.dbg_reg.dbcr2,
        SPRN_DBSR => *spr_val = (*vcpu).arch.dbsr,
        SPRN_TSR => *spr_val = (*vcpu).arch.tsr,
        SPRN_TCR => *spr_val = (*vcpu).arch.tcr,
        SPRN_IVOR0..=SPRN_IVOR15 => *spr_val = (*vcpu).arch.ivor[(sprn - SPRN_IVOR0) as usize],
        SPRN_MCSR => *spr_val = (*vcpu).arch.mcsr,
        // CONFIG_64BIT: SPRN_EPCR => *spr_val = (*vcpu).arch.epcr,
        _ => emulated = EMULATE_FAIL,
    }
    emulated
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
