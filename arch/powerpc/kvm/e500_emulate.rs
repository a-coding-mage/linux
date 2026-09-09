// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2008-2011 Freescale Semiconductor, Inc. All rights reserved.
 *
 * Author: Yu Liu, <yu.liu@freescale.com>
 *
 * Description:
 * This file is derived from arch/powerpc/kvm/44x_emulate.c,
 * by Hollis Blanchard <hollisb@us.ibm.com>.
 */

// Dependencies supplied by the surrounding kernel translation.

const XOP_DCBTLS: u32 = 166;
const XOP_MSGSND: u32 = 206;
const XOP_MSGCLR: u32 = 238;
const XOP_MFTMR: u32 = 366;
const XOP_TLBIVAX: u32 = 786;
const XOP_TLBSX: u32 = 914;
const XOP_TLBRE: u32 = 946;
const XOP_TLBWE: u32 = 978;
const XOP_TLBILX: u32 = 18;
const XOP_EHPRIV: u32 = 270;

#[cfg(feature = "CONFIG_KVM_E500MC")]
unsafe fn dbell2prio(param: ulong) -> i32 {
    let msg = (param & PPC_DBELL_TYPE_MASK) as i32;
    let mut prio = -1;

    match msg {
        x if x == PPC_DBELL_TYPE(PPC_DBELL) as i32 => prio = BOOKE_IRQPRIO_DBELL,
        x if x == PPC_DBELL_TYPE(PPC_DBELL_CRIT) as i32 => prio = BOOKE_IRQPRIO_DBELL_CRIT,
        _ => {}
    }
    prio
}

#[cfg(feature = "CONFIG_KVM_E500MC")]
unsafe fn kvmppc_e500_emul_msgclr(vcpu: *mut kvm_vcpu, rb: i32) -> i32 {
    let param = (*vcpu).arch.regs.gpr[rb as usize];
    let prio = dbell2prio(param);
    if prio < 0 { return EMULATE_FAIL; }
    clear_bit(prio as usize, &mut (*vcpu).arch.pending_exceptions);
    EMULATE_DONE
}

#[cfg(feature = "CONFIG_KVM_E500MC")]
unsafe fn kvmppc_e500_emul_msgsnd(vcpu: *mut kvm_vcpu, rb: i32) -> i32 {
    let param = (*vcpu).arch.regs.gpr[rb as usize];
    let prio = dbell2prio(rb as ulong);
    let pir = param & PPC_DBELL_PIR_MASK;
    let mut i: ulong = 0;
    let mut cvcpu: *mut kvm_vcpu = core::ptr::null_mut();

    if prio < 0 { return EMULATE_FAIL; }
    kvm_for_each_vcpu(i, cvcpu, (*vcpu).kvm) {
        let cpir = (*cvcpu).arch.shared.pir;
        if (param & PPC_DBELL_MSG_BRDCAST) != 0 || cpir == pir {
            set_bit(prio as usize, &mut (*cvcpu).arch.pending_exceptions);
            kvm_vcpu_kick(cvcpu);
        }
    }
    EMULATE_DONE
}

unsafe fn kvmppc_e500_emul_ehpriv(vcpu: *mut kvm_vcpu, inst: u32, advance: *mut i32) -> i32 {
    let mut emulated = EMULATE_DONE;
    match get_oc(inst) {
        EHPRIV_OC_DEBUG => {
            (*vcpu).run.exit_reason = KVM_EXIT_DEBUG;
            (*vcpu).run.debug.arch.address = (*vcpu).arch.regs.nip;
            (*vcpu).run.debug.arch.status = 0;
            kvmppc_account_exit(vcpu, DEBUG_EXITS);
            emulated = EMULATE_EXIT_USER;
            *advance = 0;
        }
        _ => emulated = EMULATE_FAIL,
    }
    emulated
}

unsafe fn kvmppc_e500_emul_dcbtls(vcpu: *mut kvm_vcpu) -> i32 {
    let vcpu_e500 = to_e500(vcpu);
    // Always fail to lock the cache
    (*vcpu_e500).l1csr0 |= L1CSR0_CUL;
    EMULATE_DONE
}

unsafe fn kvmppc_e500_emul_mftmr(vcpu: *mut kvm_vcpu, inst: u32, rt: i32) -> i32 {
    // Expose one thread per vcpu
    if get_tmrn(inst) == TMRN_TMCFG0 {
        kvmppc_set_gpr(vcpu, rt, 1 | (1 << TMRN_TMCFG0_NATHRD_SHIFT));
        return EMULATE_DONE;
    }
    EMULATE_FAIL
}

pub unsafe fn kvmppc_core_emulate_op_e500(vcpu: *mut kvm_vcpu, inst: u32, advance: *mut i32) -> i32 {
    let mut emulated = EMULATE_DONE;
    let ra = get_ra(inst);
    let rb = get_rb(inst);
    let rt = get_rt(inst);
    let mut ea: gva_t;

    match get_op(inst) {
        31 => match get_xop(inst) {
            XOP_DCBTLS => emulated = kvmppc_e500_emul_dcbtls(vcpu),
            #[cfg(feature = "CONFIG_KVM_E500MC")]
            XOP_MSGSND => emulated = kvmppc_e500_emul_msgsnd(vcpu, rb),
            #[cfg(feature = "CONFIG_KVM_E500MC")]
            XOP_MSGCLR => emulated = kvmppc_e500_emul_msgclr(vcpu, rb),
            XOP_TLBRE => emulated = kvmppc_e500_emul_tlbre(vcpu),
            XOP_TLBWE => emulated = kvmppc_e500_emul_tlbwe(vcpu),
            XOP_TLBSX => { ea = kvmppc_get_ea_indexed(vcpu, ra, rb); emulated = kvmppc_e500_emul_tlbsx(vcpu, ea); }
            XOP_TLBILX => { let typ = rt & 0x3; ea = kvmppc_get_ea_indexed(vcpu, ra, rb); emulated = kvmppc_e500_emul_tlbilx(vcpu, typ, ea); }
            XOP_TLBIVAX => { ea = kvmppc_get_ea_indexed(vcpu, ra, rb); emulated = kvmppc_e500_emul_tlbivax(vcpu, ea); }
            XOP_MFTMR => emulated = kvmppc_e500_emul_mftmr(vcpu, inst, rt),
            XOP_EHPRIV => emulated = kvmppc_e500_emul_ehpriv(vcpu, inst, advance),
            _ => emulated = EMULATE_FAIL,
        },
        _ => emulated = EMULATE_FAIL,
    }
    if emulated == EMULATE_FAIL { emulated = kvmppc_booke_emulate_op(vcpu, inst, advance); }
    emulated
}

pub unsafe fn kvmppc_core_emulate_mtspr_e500(vcpu: *mut kvm_vcpu, sprn: i32, spr_val: ulong) -> i32 {
    let vcpu_e500 = to_e500(vcpu);
    let mut emulated = EMULATE_DONE;
    match sprn {
        #[cfg(not(feature = "CONFIG_KVM_BOOKE_HV"))]
        SPRN_PID => kvmppc_set_pid(vcpu, spr_val),
        #[cfg(not(feature = "CONFIG_KVM_BOOKE_HV"))]
        SPRN_PID1 => { if spr_val != 0 { return EMULATE_FAIL; } (*vcpu_e500).pid[1] = spr_val; }
        #[cfg(not(feature = "CONFIG_KVM_BOOKE_HV"))]
        SPRN_PID2 => { if spr_val != 0 { return EMULATE_FAIL; } (*vcpu_e500).pid[2] = spr_val; }
        #[cfg(not(feature = "CONFIG_KVM_BOOKE_HV"))]
        SPRN_MAS0 => (*vcpu).arch.shared.mas0 = spr_val,
        #[cfg(not(feature = "CONFIG_KVM_BOOKE_HV"))]
        SPRN_MAS1 => (*vcpu).arch.shared.mas1 = spr_val,
        #[cfg(not(feature = "CONFIG_KVM_BOOKE_HV"))]
        SPRN_MAS2 => (*vcpu).arch.shared.mas2 = spr_val,
        #[cfg(not(feature = "CONFIG_KVM_BOOKE_HV"))]
        SPRN_MAS3 => { (*vcpu).arch.shared.mas7_3 &= !(0xffff_ffffu64); (*vcpu).arch.shared.mas7_3 |= spr_val; }
        #[cfg(not(feature = "CONFIG_KVM_BOOKE_HV"))]
        SPRN_MAS4 => (*vcpu).arch.shared.mas4 = spr_val,
        #[cfg(not(feature = "CONFIG_KVM_BOOKE_HV"))]
        SPRN_MAS6 => (*vcpu).arch.shared.mas6 = spr_val,
        #[cfg(not(feature = "CONFIG_KVM_BOOKE_HV"))]
        SPRN_MAS7 => { (*vcpu).arch.shared.mas7_3 &= 0xffff_ffffu64; (*vcpu).arch.shared.mas7_3 |= (spr_val as u64) << 32; }
        SPRN_L1CSR0 => { (*vcpu_e500).l1csr0 = spr_val; (*vcpu_e500).l1csr0 &= !(L1CSR0_DCFI | L1CSR0_CLFC); }
        SPRN_L1CSR1 => { (*vcpu_e500).l1csr1 = spr_val; (*vcpu_e500).l1csr1 &= !(L1CSR1_ICFI | L1CSR1_ICLFR); }
        SPRN_HID0 => (*vcpu_e500).hid0 = spr_val,
        SPRN_HID1 => (*vcpu_e500).hid1 = spr_val,
        SPRN_MMUCSR0 => emulated = kvmppc_e500_emul_mt_mmucsr0(vcpu_e500, spr_val),
        // Guest relies on host power management configurations.
        // Treat the request as a general store.
        SPRN_PWRMGTCR0 => (*vcpu).arch.pwrmgtcr0 = spr_val,
        // The branch predictor has already been flushed; return to the guest.
        SPRN_BUCSR => {},
        #[cfg(feature = "CONFIG_SPE_POSSIBLE")]
        SPRN_IVOR32 => (*vcpu).arch.ivor[BOOKE_IRQPRIO_SPE_UNAVAIL as usize] = spr_val,
        #[cfg(feature = "CONFIG_SPE_POSSIBLE")]
        SPRN_IVOR33 => (*vcpu).arch.ivor[BOOKE_IRQPRIO_SPE_FP_DATA as usize] = spr_val,
        #[cfg(feature = "CONFIG_SPE_POSSIBLE")]
        SPRN_IVOR34 => (*vcpu).arch.ivor[BOOKE_IRQPRIO_SPE_FP_ROUND as usize] = spr_val,
        #[cfg(feature = "CONFIG_ALTIVEC")]
        SPRN_IVOR32 => (*vcpu).arch.ivor[BOOKE_IRQPRIO_ALTIVEC_UNAVAIL as usize] = spr_val,
        #[cfg(feature = "CONFIG_ALTIVEC")]
        SPRN_IVOR33 => (*vcpu).arch.ivor[BOOKE_IRQPRIO_ALTIVEC_ASSIST as usize] = spr_val,
        SPRN_IVOR35 => (*vcpu).arch.ivor[BOOKE_IRQPRIO_PERFORMANCE_MONITOR as usize] = spr_val,
        #[cfg(feature = "CONFIG_KVM_BOOKE_HV")]
        SPRN_IVOR36 => (*vcpu).arch.ivor[BOOKE_IRQPRIO_DBELL as usize] = spr_val,
        #[cfg(feature = "CONFIG_KVM_BOOKE_HV")]
        SPRN_IVOR37 => (*vcpu).arch.ivor[BOOKE_IRQPRIO_DBELL_CRIT as usize] = spr_val,
        _ => emulated = kvmppc_booke_emulate_mtspr(vcpu, sprn, spr_val),
    }
    emulated
}

pub unsafe fn kvmppc_core_emulate_mfspr_e500(vcpu: *mut kvm_vcpu, sprn: i32, spr_val: *mut ulong) -> i32 {
    let vcpu_e500 = to_e500(vcpu);
    let mut emulated = EMULATE_DONE;
    match sprn {
        #[cfg(not(feature = "CONFIG_KVM_BOOKE_HV"))]
        SPRN_PID => *spr_val = (*vcpu_e500).pid[0],
        #[cfg(not(feature = "CONFIG_KVM_BOOKE_HV"))]
        SPRN_PID1 => *spr_val = (*vcpu_e500).pid[1],
        #[cfg(not(feature = "CONFIG_KVM_BOOKE_HV"))]
        SPRN_PID2 => *spr_val = (*vcpu_e500).pid[2],
        #[cfg(not(feature = "CONFIG_KVM_BOOKE_HV"))]
        SPRN_MAS0 => *spr_val = (*vcpu).arch.shared.mas0,
        #[cfg(not(feature = "CONFIG_KVM_BOOKE_HV"))]
        SPRN_MAS1 => *spr_val = (*vcpu).arch.shared.mas1,
        #[cfg(not(feature = "CONFIG_KVM_BOOKE_HV"))]
        SPRN_MAS2 => *spr_val = (*vcpu).arch.shared.mas2,
        #[cfg(not(feature = "CONFIG_KVM_BOOKE_HV"))]
        SPRN_MAS3 => *spr_val = (*vcpu).arch.shared.mas7_3 as u32 as ulong,
        #[cfg(not(feature = "CONFIG_KVM_BOOKE_HV"))]
        SPRN_MAS4 => *spr_val = (*vcpu).arch.shared.mas4,
        #[cfg(not(feature = "CONFIG_KVM_BOOKE_HV"))]
        SPRN_MAS6 => *spr_val = (*vcpu).arch.shared.mas6,
        #[cfg(not(feature = "CONFIG_KVM_BOOKE_HV"))]
        SPRN_MAS7 => *spr_val = (*vcpu).arch.shared.mas7_3 >> 32,
        SPRN_DECAR => *spr_val = (*vcpu).arch.decar,
        SPRN_TLB0CFG => *spr_val = (*vcpu).arch.tlbcfg[0],
        SPRN_TLB1CFG => *spr_val = (*vcpu).arch.tlbcfg[1],
        SPRN_TLB0PS => { if !has_feature(vcpu, VCPU_FTR_MMU_V2) { return EMULATE_FAIL; } *spr_val = (*vcpu).arch.tlbps[0]; }
        SPRN_TLB1PS => { if !has_feature(vcpu, VCPU_FTR_MMU_V2) { return EMULATE_FAIL; } *spr_val = (*vcpu).arch.tlbps[1]; }
        SPRN_L1CSR0 => *spr_val = (*vcpu_e500).l1csr0,
        SPRN_L1CSR1 => *spr_val = (*vcpu_e500).l1csr1,
        SPRN_HID0 => *spr_val = (*vcpu_e500).hid0,
        SPRN_HID1 => *spr_val = (*vcpu_e500).hid1,
        SPRN_SVR => *spr_val = (*vcpu_e500).svr,
        SPRN_MMUCSR0 => *spr_val = 0,
        SPRN_MMUCFG => *spr_val = (*vcpu).arch.mmucfg,
        SPRN_EPTCFG => {
            if !has_feature(vcpu, VCPU_FTR_MMU_V2) { return EMULATE_FAIL; }
            // Legacy Linux guests access EPTCFG even when the E.PT category is disabled.
            *spr_val = (*vcpu).arch.eptcfg;
        }
        SPRN_PWRMGTCR0 => *spr_val = (*vcpu).arch.pwrmgtcr0,
        #[cfg(feature = "CONFIG_SPE_POSSIBLE")]
        SPRN_IVOR32 => *spr_val = (*vcpu).arch.ivor[BOOKE_IRQPRIO_SPE_UNAVAIL as usize],
        #[cfg(feature = "CONFIG_SPE_POSSIBLE")]
        SPRN_IVOR33 => *spr_val = (*vcpu).arch.ivor[BOOKE_IRQPRIO_SPE_FP_DATA as usize],
        #[cfg(feature = "CONFIG_SPE_POSSIBLE")]
        SPRN_IVOR34 => *spr_val = (*vcpu).arch.ivor[BOOKE_IRQPRIO_SPE_FP_ROUND as usize],
        #[cfg(feature = "CONFIG_ALTIVEC")]
        SPRN_IVOR32 => *spr_val = (*vcpu).arch.ivor[BOOKE_IRQPRIO_ALTIVEC_UNAVAIL as usize],
        #[cfg(feature = "CONFIG_ALTIVEC")]
        SPRN_IVOR33 => *spr_val = (*vcpu).arch.ivor[BOOKE_IRQPRIO_ALTIVEC_ASSIST as usize],
        SPRN_IVOR35 => *spr_val = (*vcpu).arch.ivor[BOOKE_IRQPRIO_PERFORMANCE_MONITOR as usize],
        #[cfg(feature = "CONFIG_KVM_BOOKE_HV")]
        SPRN_IVOR36 => *spr_val = (*vcpu).arch.ivor[BOOKE_IRQPRIO_DBELL as usize],
        #[cfg(feature = "CONFIG_KVM_BOOKE_HV")]
        SPRN_IVOR37 => *spr_val = (*vcpu).arch.ivor[BOOKE_IRQPRIO_DBELL_CRIT as usize],
        _ => emulated = kvmppc_booke_emulate_mfspr(vcpu, sprn, spr_val),
    }
    emulated
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
