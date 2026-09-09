// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright IBM Corp. 2007
 * Copyright 2011 Freescale Semiconductor, Inc.
 *
 * Authors: Hollis Blanchard <hollisb@us.ibm.com>
 */

// Dependencies supplied by the surrounding kernel/KVM translation unit.

pub unsafe fn kvmppc_emulate_dec(vcpu: *mut kvm_vcpu) {
    let mut dec_nsec: c_ulong;
    let mut dec_time: c_ulonglong;

    pr_debug!("mtDEC: {:lx}\n", (*vcpu).arch.dec);
    hrtimer_try_to_cancel(&mut (*vcpu).arch.dec_timer);

    // mtdec lowers the interrupt line when positive.
    #[cfg(CONFIG_PPC_BOOK3S)]
    kvmppc_core_dequeue_dec(vcpu);

    // On BOOKE, DEC = 0 is as good as decrementer not enabled.
    #[cfg(CONFIG_BOOKE)]
    if (*vcpu).arch.dec == 0 {
        return;
    }

    // The decrementer ticks at the same rate as the timebase, so that's how
    // we convert the guest DEC value to the number of host ticks.
    dec_time = (*vcpu).arch.dec;
    // Guest timebase ticks at the same frequency as host timebase.
    // So use the host timebase calculations for decrementer emulation.
    dec_time = tb_to_ns(dec_time);
    dec_nsec = do_div(&mut dec_time, NSEC_PER_SEC);
    hrtimer_start(
        &mut (*vcpu).arch.dec_timer,
        ktime_set(dec_time, dec_nsec),
        HRTIMER_MODE_REL,
    );
    (*vcpu).arch.dec_jiffies = get_tb();
}

pub unsafe fn kvmppc_get_dec(vcpu: *mut kvm_vcpu, tb: u64) -> u32 {
    let jd: u64 = tb.wrapping_sub((*vcpu).arch.dec_jiffies);

    #[cfg(CONFIG_BOOKE)]
    if (*vcpu).arch.dec < jd {
        return 0;
    }

    (*vcpu).arch.dec.wrapping_sub(jd)
}

unsafe fn kvmppc_emulate_mtspr(vcpu: *mut kvm_vcpu, sprn: c_int, rs: c_int) -> c_int {
    let mut emulated: c_int = EMULATE_DONE;
    let spr_val: ulong = kvmppc_get_gpr(vcpu, rs);

    match sprn {
        SPRN_SRR0 => kvmppc_set_srr0(vcpu, spr_val),
        SPRN_SRR1 => kvmppc_set_srr1(vcpu, spr_val),
        // We need to context-switch the timebase for watchdog and FIT.
        SPRN_TBWL | SPRN_TBWU => {}
        SPRN_DEC => {
            (*vcpu).arch.dec = spr_val as u32;
            kvmppc_emulate_dec(vcpu);
        }
        SPRN_SPRG0 => kvmppc_set_sprg0(vcpu, spr_val),
        SPRN_SPRG1 => kvmppc_set_sprg1(vcpu, spr_val),
        SPRN_SPRG2 => kvmppc_set_sprg2(vcpu, spr_val),
        SPRN_SPRG3 => kvmppc_set_sprg3(vcpu, spr_val),
        // PIR can legally be written, but we ignore it.
        SPRN_PIR => {}
        _ => {
            emulated = (*(*vcpu).kvm).arch.kvm_ops.emulate_mtspr(vcpu, sprn, spr_val);
            if emulated == EMULATE_FAIL {
                printk!(KERN_INFO, "mtspr: unknown spr 0x{:x}\n", sprn);
            }
        }
    }

    kvmppc_set_exit_type(vcpu, EMULATED_MTSPR_EXITS);
    emulated
}

unsafe fn kvmppc_emulate_mfspr(vcpu: *mut kvm_vcpu, sprn: c_int, rt: c_int) -> c_int {
    let mut emulated: c_int = EMULATE_DONE;
    let mut spr_val: ulong = 0;

    match sprn {
        SPRN_SRR0 => spr_val = kvmppc_get_srr0(vcpu),
        SPRN_SRR1 => spr_val = kvmppc_get_srr1(vcpu),
        SPRN_PVR => spr_val = (*vcpu).arch.pvr,
        SPRN_PIR => spr_val = (*vcpu).vcpu_id,
        // mftb and TBRL/TBWL are user-accessible, so the guest can always
        // access the real TB anyways. In fact, these traps may never occur.
        SPRN_TBWL => spr_val = get_tb() >> 32,
        SPRN_TBWU => spr_val = get_tb(),
        SPRN_SPRG0 => spr_val = kvmppc_get_sprg0(vcpu),
        SPRN_SPRG1 => spr_val = kvmppc_get_sprg1(vcpu),
        SPRN_SPRG2 => spr_val = kvmppc_get_sprg2(vcpu),
        SPRN_SPRG3 => spr_val = kvmppc_get_sprg3(vcpu),
        // SPRG4-7 are user-readable, so we don't get a trap.
        SPRN_DEC => spr_val = kvmppc_get_dec(vcpu, get_tb()),
        _ => {
            emulated = (*(*vcpu).kvm).arch.kvm_ops.emulate_mfspr(vcpu, sprn, &mut spr_val);
            if unlikely(emulated == EMULATE_FAIL) {
                printk!(KERN_INFO, "mfspr: unknown spr 0x{:x}\n", sprn);
            }
        }
    }

    if emulated == EMULATE_DONE {
        kvmppc_set_gpr(vcpu, rt, spr_val);
    }
    kvmppc_set_exit_type(vcpu, EMULATED_MFSPR_EXITS);
    emulated
}

// XXX Should probably auto-generate instruction decoding for a particular
// core from opcode tables in the future.
pub unsafe fn kvmppc_emulate_instruction(vcpu: *mut kvm_vcpu) -> c_int {
    let mut pinst: ppc_inst_t = core::mem::zeroed();
    let mut inst: u32;
    let mut rs: c_int;
    let mut rt: c_int;
    let mut sprn: c_int;
    let mut emulated: c_int;
    let mut advance: c_int = 1;

    kvmppc_set_exit_type(vcpu, EMULATED_INST_EXITS);
    emulated = kvmppc_get_last_inst(vcpu, INST_GENERIC, &mut pinst);
    inst = ppc_inst_val(pinst);
    if emulated != EMULATE_DONE {
        return emulated;
    }

    pr_debug!("Emulating opcode {} / {}\n", get_op(inst), get_xop(inst));
    rs = get_rs(inst);
    rt = get_rt(inst);
    sprn = get_sprn(inst);

    match get_op(inst) {
        OP_TRAP => {
            #[cfg(CONFIG_PPC_BOOK3S)]
            kvmppc_core_queue_program(vcpu, SRR1_PROGTRAP);
            #[cfg(not(CONFIG_PPC_BOOK3S))]
            kvmppc_core_queue_program(vcpu, (*vcpu).arch.shared.esr | ESR_PTR);
            advance = 0;
        }
        31 => match get_xop(inst) {
            OP_31_XOP_TRAP => {
                #[cfg(CONFIG_PPC_BOOK3S)]
                kvmppc_core_queue_program(vcpu, SRR1_PROGTRAP);
                #[cfg(not(CONFIG_PPC_BOOK3S))]
                kvmppc_core_queue_program(vcpu, (*vcpu).arch.shared.esr | ESR_PTR);
                advance = 0;
            }
            OP_31_XOP_MFSPR => {
                emulated = kvmppc_emulate_mfspr(vcpu, sprn, rt);
                if emulated == EMULATE_AGAIN { emulated = EMULATE_DONE; advance = 0; }
            }
            OP_31_XOP_MTSPR => {
                emulated = kvmppc_emulate_mtspr(vcpu, sprn, rs);
                if emulated == EMULATE_AGAIN { emulated = EMULATE_DONE; advance = 0; }
            }
            OP_31_XOP_TLBSYNC => {}
            _ => emulated = EMULATE_FAIL,
        },
        0 => {
            // Instruction with primary opcode 0. Based on PowerISA these are illegal instructions.
            if inst == KVMPPC_INST_SW_BREAKPOINT {
                (*(*vcpu).run).exit_reason = KVM_EXIT_DEBUG;
                (*(*vcpu).run).debug.arch.status = 0;
                (*(*vcpu).run).debug.arch.address = kvmppc_get_pc(vcpu);
                emulated = EMULATE_EXIT_USER;
                advance = 0;
            } else { emulated = EMULATE_FAIL; }
        }
        _ => emulated = EMULATE_FAIL,
    }

    if emulated == EMULATE_FAIL {
        emulated = (*(*vcpu).kvm).arch.kvm_ops.emulate_op(vcpu, inst, &mut advance);
        if emulated == EMULATE_AGAIN {
            advance = 0;
        } else if emulated == EMULATE_FAIL {
            advance = 0;
            printk!(KERN_ERR, "Couldn't emulate instruction 0x{:08x} (op {} xop {})\n", inst, get_op(inst), get_xop(inst));
        }
    }

    trace_kvm_ppc_instr(inst, kvmppc_get_pc(vcpu), emulated);
    if advance != 0 {
        kvmppc_set_pc(vcpu, kvmppc_get_pc(vcpu).wrapping_add(4));
    }
    emulated
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
