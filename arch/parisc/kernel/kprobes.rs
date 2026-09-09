// SPDX-License-Identifier: GPL-2.0
/*
 * arch/parisc/kernel/kprobes.c
 *
 * PA-RISC kprobes implementation
 *
 * Copyright (c) 2019 Sven Schnelle <svens@stackframe.org>
 * Copyright (c) 2022 Helge Deller <deller@gmx.de>
 */

// Dependencies supplied by the surrounding kernel translation.

pub static mut CURRENT_KPROBE: *mut Kprobe = core::ptr::null_mut();
pub static mut KPROBE_CTLBLK: KprobeCtlblk = KprobeCtlblk::zeroed();

pub unsafe fn arch_prepare_kprobe(p: *mut Kprobe) -> i32 {
    if (p as usize & 3usize) != 0 {
        return -22;
    }

    (*p).ainsn.insn = get_insn_slot();
    if (*p).ainsn.insn.is_null() {
        return -12;
    }

    /*
     * Set up new instructions. Second break instruction will
     * trigger call of parisc_kprobe_ss_handler().
     */
    (*p).opcode = *(*p).addr;
    (*p).ainsn.insn.add(0).write((*p).opcode);
    (*p).ainsn.insn.add(1).write(PARISC_KPROBES_BREAK_INSN2);

    flush_insn_slot(p);
    0
}

pub unsafe fn arch_remove_kprobe(p: *mut Kprobe) {
    if (*p).ainsn.insn.is_null() {
        return;
    }

    free_insn_slot((*p).ainsn.insn, 0);
    (*p).ainsn.insn = core::ptr::null_mut();
}

pub unsafe fn arch_arm_kprobe(p: *mut Kprobe) {
    patch_text((*p).addr, PARISC_KPROBES_BREAK_INSN);
}

pub unsafe fn arch_disarm_kprobe(p: *mut Kprobe) {
    patch_text((*p).addr, (*p).opcode);
}

unsafe fn save_previous_kprobe(kcb: *mut KprobeCtlblk) {
    (*kcb).prev_kprobe.kp = kprobe_running();
    (*kcb).prev_kprobe.status = (*kcb).kprobe_status;
}

unsafe fn restore_previous_kprobe(kcb: *mut KprobeCtlblk) {
    CURRENT_KPROBE = (*kcb).prev_kprobe.kp;
    (*kcb).kprobe_status = (*kcb).prev_kprobe.status;
}

#[inline]
unsafe fn set_current_kprobe(p: *mut Kprobe) {
    CURRENT_KPROBE = p;
}

unsafe fn setup_singlestep(p: *mut Kprobe, kcb: *mut KprobeCtlblk, regs: *mut PtRegs) {
    (*kcb).iaoq[0] = (*regs).iaoq[0];
    (*kcb).iaoq[1] = (*regs).iaoq[1];
    instruction_pointer_set(regs, (*p).ainsn.insn as usize);
}

pub unsafe fn parisc_kprobe_break_handler(regs: *mut PtRegs) -> i32 {
    preempt_disable();

    let kcb = get_kprobe_ctlblk();
    let p = get_kprobe((*regs).iaoq[0] as *mut usize);

    if p.is_null() {
        preempt_enable_no_resched();
        return 0;
    }

    if !kprobe_running().is_null() {
        /*
         * We have reentered the kprobe_handler, since another kprobe
         * was hit while within the handler, we save the original
         * kprobes and single step on the instruction of the new probe
         * without calling any user handlers to avoid recursive
         * kprobes.
         */
        save_previous_kprobe(kcb);
        set_current_kprobe(p);
        kprobes_inc_nmissed_count(p);
        setup_singlestep(p, kcb, regs);
        (*kcb).kprobe_status = KPROBE_REENTER;
        return 1;
    }

    set_current_kprobe(p);
    (*kcb).kprobe_status = KPROBE_HIT_ACTIVE;

    /* If we have no pre-handler or it returned 0, we continue with
     * normal processing. If we have a pre-handler and it returned
     * non-zero - which means user handler setup registers to exit
     * to another instruction, we must skip the single stepping.
     */
    if (*p).pre_handler.is_none() || ((*p).pre_handler.unwrap()(p, regs) == 0) {
        setup_singlestep(p, kcb, regs);
        (*kcb).kprobe_status = KPROBE_HIT_SS;
    } else {
        reset_current_kprobe();
        preempt_enable_no_resched();
    }
    1
}

pub unsafe fn parisc_kprobe_ss_handler(regs: *mut PtRegs) -> i32 {
    let kcb = get_kprobe_ctlblk();
    let p = kprobe_running();

    if p.is_null() {
        return 0;
    }

    if (*regs).iaoq[0] != (*p).ainsn.insn as usize + 4 {
        return 0;
    }

    /* restore back original saved kprobe variables and continue */
    if (*kcb).kprobe_status == KPROBE_REENTER {
        restore_previous_kprobe(kcb);
        return 1;
    }

    /* for absolute branch instructions we can copy iaoq_b. for relative
     * branch instructions we need to calculate the new address based on
     * the difference between iaoq_f and iaoq_b. We cannot use iaoq_b without
     * modifications because it's based on our ainsn.insn address.
     */
    if let Some(post_handler) = (*p).post_handler {
        post_handler(p, regs, 0);
    }

    match (*regs).iir >> 26 {
        0x38 | 0x39 | 0x3a | 0x3b => {
            /* for absolute branches, regs->iaoq[1] has already the right
             * address
             */
            (*regs).iaoq[0] = (*kcb).iaoq[1];
        }
        _ => {
            (*regs).iaoq[0] = (*kcb).iaoq[1];
            (*regs).iaoq[1] = (*regs).iaoq[0] + 4;
        }
    }
    (*kcb).kprobe_status = KPROBE_HIT_SSDONE;
    reset_current_kprobe();
    1
}

pub unsafe fn __kretprobe_trampoline() {
    core::arch::asm!("nop");
    core::arch::asm!("nop");
}

static mut TRAMPOLINE_P: Kprobe = Kprobe {
    pre_handler: Some(trampoline_probe_handler),
    ..Kprobe::zeroed()
};

unsafe fn trampoline_probe_handler(_p: *mut Kprobe, regs: *mut PtRegs) -> i32 {
    __kretprobe_trampoline_handler(regs, core::ptr::null_mut());
    1
}

pub unsafe fn arch_kretprobe_fixup_return(regs: *mut PtRegs, correct_ret_addr: *mut KprobeOpcode) {
    (*regs).gr[2] = correct_ret_addr as usize;
}

pub unsafe fn arch_prepare_kretprobe(ri: *mut KretprobeInstance, regs: *mut PtRegs) {
    (*ri).ret_addr = (*regs).gr[2] as *mut KprobeOpcode;
    (*ri).fp = core::ptr::null_mut();

    /* Replace the return addr with trampoline addr. */
    (*regs).gr[2] = TRAMPOLINE_P.addr as usize;
}

pub unsafe fn arch_trampoline_kprobe(p: *mut Kprobe) -> i32 {
    if (*p).addr == TRAMPOLINE_P.addr { 1 } else { 0 }
}

pub unsafe fn arch_init_kprobes() -> i32 {
    TRAMPOLINE_P.addr = dereference_function_descriptor(__kretprobe_trampoline);
    register_kprobe(&mut TRAMPOLINE_P)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
