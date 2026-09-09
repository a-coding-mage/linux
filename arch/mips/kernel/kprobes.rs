// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Kernel Probes (KProbes)
 *  arch/mips/kernel/kprobes.c
 *
 *  Copyright 2006 Sony Corp.
 *  Copyright 2010 Cavium Networks
 *
 *  Some portions copied from the powerpc version.
 *
 *   Copyright (C) IBM Corporation, 2002, 2004
 */

// Dependency intent: linux/kprobes.h, linux/preempt.h, linux/uaccess.h,
// linux/kdebug.h, linux/slab.h, asm/ptrace.h, asm/branch.h, asm/break.h,
// and probes-common.h provide the kernel types, constants, and functions used below.

static BREAKPOINT_INSN: mips_instruction = mips_instruction {
    b_format: b_format {
        opcode: spec_op,
        code: BRK_KPROBE_BP,
        func: break_op,
    },
};

static BREAKPOINT2_INSN: mips_instruction = mips_instruction {
    b_format: b_format {
        opcode: spec_op,
        code: BRK_KPROBE_SSTEPBP,
        func: break_op,
    },
};

// DEFINE_PER_CPU(struct kprobe *, current_kprobe);
// DEFINE_PER_CPU(struct kprobe_ctlblk, kprobe_ctlblk);

unsafe fn insn_has_delayslot(insn: mips_instruction) -> i32 {
    __insn_has_delay_slot(insn)
}

unsafe fn insn_has_ll_or_sc(insn: mips_instruction) -> i32 {
    let mut ret: i32 = 0;

    match insn.i_format.opcode {
        ll_op | lld_op | sc_op | scd_op => ret = 1,
        _ => {}
    }
    ret
}

unsafe fn arch_prepare_kprobe(p: *mut kprobe) -> i32 {
    let insn = (*p).addr.read();
    let mut prev_insn: mips_instruction = core::mem::zeroed();
    let mut ret: i32 = 0;

    if insn_has_ll_or_sc(insn) != 0 {
        pr_notice!("Kprobes for ll and sc instructions are not supported\n");
        ret = -EINVAL;
        return ret;
    }

    if copy_from_kernel_nofault(
        &mut prev_insn as *mut _ as *mut core::ffi::c_void,
        (*p).addr.sub(1) as *const core::ffi::c_void,
        core::mem::size_of::<mips_instruction>(),
    ) == 0 && insn_has_delayslot(prev_insn) != 0 {
        pr_notice!("Kprobes for branch delayslot are not supported\n");
        ret = -EINVAL;
        return ret;
    }

    if __insn_is_compact_branch(insn) {
        pr_notice!("Kprobes for compact branches are not supported\n");
        ret = -EINVAL;
        return ret;
    }

    // insn: must be on special executable page on mips.
    (*p).ainsn.insn = get_insn_slot();
    if (*p).ainsn.insn.is_null() {
        ret = -ENOMEM;
        return ret;
    }

    /*
     * In the kprobe->ainsn.insn[] array we store the original
     * instruction at index zero and a break trap instruction at
     * index one.
     *
     * On MIPS arch if the instruction at probed address is a
     * branch instruction, we need to execute the instruction at
     * Branch Delayslot (BD) at the time of probe hit. As MIPS also
     * doesn't have single stepping support, the BD instruction can
     * not be executed in-line and it would be executed on SSOL slot
     * using a normal breakpoint instruction in the next slot.
     * So, read the instruction and save it for later execution.
     */
    if insn_has_delayslot(insn) != 0 {
        core::ptr::copy_nonoverlapping((*p).addr.add(1), (*p).ainsn.insn, 1);
    } else {
        core::ptr::copy_nonoverlapping((*p).addr, (*p).ainsn.insn, 1);
    }
    (*p).ainsn.insn.add(1).write(BREAKPOINT2_INSN);
    (*p).opcode = insn;
    ret
}

unsafe fn arch_arm_kprobe(p: *mut kprobe) {
    (*p).addr.write(BREAKPOINT_INSN);
    flush_insn_slot(p);
}

unsafe fn arch_disarm_kprobe(p: *mut kprobe) {
    (*p).addr.write((*p).opcode);
    flush_insn_slot(p);
}

unsafe fn arch_remove_kprobe(p: *mut kprobe) {
    if !(*p).ainsn.insn.is_null() {
        free_insn_slot((*p).ainsn.insn, 0);
        (*p).ainsn.insn = core::ptr::null_mut();
    }
}

unsafe fn save_previous_kprobe(kcb: *mut kprobe_ctlblk) {
    (*kcb).prev_kprobe.kp = kprobe_running();
    (*kcb).prev_kprobe.status = (*kcb).kprobe_status;
    (*kcb).prev_kprobe.old_SR = (*kcb).kprobe_old_SR;
    (*kcb).prev_kprobe.saved_SR = (*kcb).kprobe_saved_SR;
    (*kcb).prev_kprobe.saved_epc = (*kcb).kprobe_saved_epc;
}

unsafe fn restore_previous_kprobe(kcb: *mut kprobe_ctlblk) {
    __this_cpu_write!(current_kprobe, (*kcb).prev_kprobe.kp);
    (*kcb).kprobe_status = (*kcb).prev_kprobe.status;
    (*kcb).kprobe_old_SR = (*kcb).prev_kprobe.old_SR;
    (*kcb).kprobe_saved_SR = (*kcb).prev_kprobe.saved_SR;
    (*kcb).kprobe_saved_epc = (*kcb).prev_kprobe.saved_epc;
}

unsafe fn set_current_kprobe(p: *mut kprobe, regs: *mut pt_regs, kcb: *mut kprobe_ctlblk) {
    __this_cpu_write!(current_kprobe, p);
    (*kcb).kprobe_saved_SR = (*kcb).kprobe_old_SR = (*regs).cp0_status & ST0_IE;
    (*kcb).kprobe_saved_epc = (*regs).cp0_epc;
}

/**
 * evaluate_branch_instrucion -
 *
 * Evaluate the branch instruction at probed address during probe hit. The
 * result of evaluation would be the updated epc. The insturction in delayslot
 * would actually be single stepped using a normal breakpoint) on SSOL slot.
 *
 * The result is also saved in the kprobe control block for later use,
 * in case we need to execute the delayslot instruction. The latter will be
 * false for NOP instruction in dealyslot and the branch-likely instructions
 * when the branch is taken. And for those cases we set a flag as
 * SKIP_DELAYSLOT in the kprobe control block
 */
unsafe fn evaluate_branch_instruction(p: *mut kprobe, regs: *mut pt_regs, kcb: *mut kprobe_ctlblk) -> i32 {
    let insn = (*p).opcode;
    let epc = (*regs).cp0_epc;
    if epc & 3 != 0 {
        pr_notice!("Failed to emulate branch instruction because of unaligned epc - sending SIGBUS to %s.\n", current.comm);
        force_sig(SIGBUS);
        return -EFAULT;
    }

    if (*p).ainsn.insn.read().word == 0 {
        (*kcb).flags |= SKIP_DELAYSLOT;
    } else {
        (*kcb).flags &= !SKIP_DELAYSLOT;
    }
    let ret = __compute_return_epc_for_insn(regs, insn);
    if ret < 0 { return ret; }
    if ret == BRANCH_LIKELY_TAKEN { (*kcb).flags |= SKIP_DELAYSLOT; }
    (*kcb).target_epc = (*regs).cp0_epc;
    0
}

unsafe fn prepare_singlestep(p: *mut kprobe, regs: *mut pt_regs, kcb: *mut kprobe_ctlblk) {
    (*regs).cp0_status &= !ST0_IE;
    if (*p).opcode.word == BREAKPOINT_INSN.word || (*p).opcode.word == BREAKPOINT2_INSN.word {
        (*regs).cp0_epc = (*p).addr as usize;
    } else if insn_has_delayslot((*p).opcode) != 0 {
        if evaluate_branch_instruction(p, regs, kcb) < 0 { return; }
    }
    (*regs).cp0_epc = (*p).ainsn.insn as usize;
}

/*
 * Called after single-stepping.  p->addr is the address of the
 * instruction whose first byte has been replaced by the "break 0"
 * instruction.  To avoid the SMP problems that can occur when we
 * temporarily put back the original opcode to single-step, we
 * single-stepped a copy of the instruction.  The address of this
 * copy is p->ainsn.insn.
 *
 * This function prepares to return from the post-single-step
 * breakpoint trap. In case of branch instructions, the target
 * epc to be restored.
 */
unsafe fn resume_execution(p: *mut kprobe, regs: *mut pt_regs, kcb: *mut kprobe_ctlblk) {
    if insn_has_delayslot((*p).opcode) != 0 {
        (*regs).cp0_epc = (*kcb).target_epc;
    } else {
        (*regs).cp0_epc = (*kcb).kprobe_saved_epc.wrapping_add(4);
    }
}

unsafe fn kprobe_handler(regs: *mut pt_regs) -> i32 {
    let addr = (*regs).cp0_epc as *mut kprobe_opcode_t;
    preempt_disable();
    let kcb = get_kprobe_ctlblk();
    if kprobe_running() {
        if let Some(p) = get_kprobe(addr) {
            if (*kcb).kprobe_status == KPROBE_HIT_SS && (*p).ainsn.insn.read().word == BREAKPOINT_INSN.word {
                (*regs).cp0_status &= !ST0_IE;
                (*regs).cp0_status |= (*kcb).kprobe_saved_SR;
                preempt_enable_no_resched(); return 0;
            }
            save_previous_kprobe(kcb); set_current_kprobe(p, regs, kcb);
            kprobes_inc_nmissed_count(p); prepare_singlestep(p, regs, kcb);
            (*kcb).kprobe_status = KPROBE_REENTER;
            if (*kcb).flags & SKIP_DELAYSLOT != 0 { resume_execution(p, regs, kcb); restore_previous_kprobe(kcb); preempt_enable_no_resched(); }
            return 1;
        } else if (*addr).word != BREAKPOINT_INSN.word { preempt_enable_no_resched(); return 1; }
        preempt_enable_no_resched(); return 0;
    }
    let p = get_kprobe(addr);
    if p.is_null() { let ret = if (*addr).word != BREAKPOINT_INSN.word { 1 } else { 0 }; preempt_enable_no_resched(); return ret; }
    set_current_kprobe(p, regs, kcb); (*kcb).kprobe_status = KPROBE_HIT_ACTIVE;
    if !(*p).pre_handler.is_none() && (*p).pre_handler.unwrap()(p, regs) != 0 { reset_current_kprobe(); preempt_enable_no_resched(); return 1; }
    prepare_singlestep(p, regs, kcb);
    if (*kcb).flags & SKIP_DELAYSLOT != 0 { (*kcb).kprobe_status = KPROBE_HIT_SSDONE; if let Some(h) = (*p).post_handler { h(p, regs, 0); } resume_execution(p, regs, kcb); preempt_enable_no_resched(); } else { (*kcb).kprobe_status = KPROBE_HIT_SS; }
    1
}

unsafe fn post_kprobe_handler(regs: *mut pt_regs) -> i32 {
    let cur = kprobe_running(); let kcb = get_kprobe_ctlblk();
    if cur.is_null() { return 0; }
    if (*kcb).kprobe_status != KPROBE_REENTER { if let Some(h) = (*cur).post_handler { (*kcb).kprobe_status = KPROBE_HIT_SSDONE; h(cur, regs, 0); } }
    resume_execution(cur, regs, kcb); (*regs).cp0_status |= (*kcb).kprobe_saved_SR;
    if (*kcb).kprobe_status == KPROBE_REENTER { restore_previous_kprobe(kcb); } else { reset_current_kprobe(); }
    preempt_enable_no_resched(); 1
}

unsafe fn kprobe_fault_handler(regs: *mut pt_regs, _trapnr: i32) -> i32 {
    let cur = kprobe_running(); let kcb = get_kprobe_ctlblk();
    if (*kcb).kprobe_status & KPROBE_HIT_SS != 0 { resume_execution(cur, regs, kcb); (*regs).cp0_status |= (*kcb).kprobe_old_SR; reset_current_kprobe(); preempt_enable_no_resched(); }
    0
}

unsafe fn kprobe_exceptions_notify(_self_: *mut notifier_block, val: usize, data: *mut core::ffi::c_void) -> i32 {
    let args = data as *mut die_args; let mut ret = NOTIFY_DONE;
    match val { DIE_BREAK => { if kprobe_handler((*args).regs) != 0 { ret = NOTIFY_STOP; } }, DIE_SSTEPBP => { if post_kprobe_handler((*args).regs) != 0 { ret = NOTIFY_STOP; } }, DIE_PAGE_FAULT => { preempt_disable(); if !kprobe_running().is_null() && kprobe_fault_handler((*args).regs, (*args).trapnr) != 0 { ret = NOTIFY_STOP; } preempt_enable(); }, _ => {} }
    ret
}

#[allow(dead_code)]
unsafe fn kretprobe_trampoline_holder() {
    core::arch::asm!("nop", ".global __kretprobe_trampoline", "__kretprobe_trampoline:", "nop", options(nostack));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
