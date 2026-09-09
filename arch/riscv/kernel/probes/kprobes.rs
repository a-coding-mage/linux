// SPDX-License-Identifier: GPL-2.0+

// C headers and kernel-provided symbols are supplied by the surrounding translation.

#[allow(non_upper_case_globals)]
static mut current_kprobe: *mut kprobe = core::ptr::null_mut();
static mut kprobe_ctlblk: kprobe_ctlblk = unsafe { core::mem::zeroed() };

unsafe fn arch_prepare_ss_slot(p: *mut kprobe) {
    let len: usize = GET_INSN_LENGTH((*p).opcode) as usize;
    let mut insn: u32 = __BUG_INSN_32;

    (*p).ainsn.api.restore = (*p).addr as usize + len;

    patch_text_nosync((*p).ainsn.api.insn, &(*p).opcode as *const _, len);
    patch_text_nosync(
        ((*p).ainsn.api.insn as *mut u8).add(len) as *mut _,
        &mut insn as *mut _,
        GET_INSN_LENGTH(insn) as usize,
    );
}

unsafe fn arch_prepare_simulate(p: *mut kprobe) {
    (*p).ainsn.api.restore = 0;
}

unsafe fn arch_simulate_insn(p: *mut kprobe, regs: *mut pt_regs) {
    let kcb: *mut kprobe_ctlblk = get_kprobe_ctlblk();

    if let Some(handler) = (*p).ainsn.api.handler {
        handler((*p).opcode as u32, (*p).addr as usize, regs);
    }

    post_kprobe_handler(p, kcb, regs);
}

unsafe fn arch_check_kprobe(addr: usize) -> bool {
    let mut offset: usize = 0;

    /* start iterating at the closest preceding symbol */
    if !kallsyms_lookup_size_offset(addr, core::ptr::null_mut(), &mut offset) {
        return false;
    }

    let mut tmp = addr - offset;
    while tmp <= addr {
        if tmp == addr {
            return true;
        }
        tmp += GET_INSN_LENGTH(*(tmp as *const u16)) as usize;
    }

    false
}

pub unsafe fn arch_prepare_kprobe(p: *mut kprobe) -> i32 {
    let mut insn = (*p).addr as *mut u16;

    if (insn as usize) & 0x1 != 0 {
        return -EILSEQ;
    }

    if !arch_check_kprobe((*p).addr as usize) {
        return -EILSEQ;
    }

    /* copy instruction */
    (*p).opcode = *insn as kprobe_opcode_t;
    insn = insn.add(1);
    if GET_INSN_LENGTH((*p).opcode) == 4 {
        (*p).opcode |= (*insn as kprobe_opcode_t) << 16;
    }

    /* decode instruction */
    match riscv_probe_decode_insn((*p).addr, &mut (*p).ainsn.api) {
        INSN_REJECTED => return -EINVAL, /* insn not supported */
        INSN_GOOD_NO_SLOT => {
            (*p).ainsn.api.insn = core::ptr::null_mut();
        } /* insn need simulation */,
        INSN_GOOD => {
            (*p).ainsn.api.insn = get_insn_slot();
            if (*p).ainsn.api.insn.is_null() {
                return -ENOMEM;
            }
        } /* instruction uses slot */,
        _ => {}
    }

    /* prepare the instruction */
    if !(*p).ainsn.api.insn.is_null() {
        arch_prepare_ss_slot(p);
    } else {
        arch_prepare_simulate(p);
    }

    0
}

/* install breakpoint in text */
pub unsafe fn arch_arm_kprobe(p: *mut kprobe) {
    let len = GET_INSN_LENGTH((*p).opcode) as usize;
    let mut insn: u32 = if len == 4 { __BUG_INSN_32 } else { __BUG_INSN_16 };
    patch_text((*p).addr, &mut insn as *mut _, len);
}

/* remove breakpoint from text */
pub unsafe fn arch_disarm_kprobe(p: *mut kprobe) {
    let len = GET_INSN_LENGTH((*p).opcode) as usize;
    patch_text((*p).addr, &(*p).opcode as *const _ as *mut _, len);
}

pub unsafe fn arch_remove_kprobe(_p: *mut kprobe) {}

unsafe fn save_previous_kprobe(kcb: *mut kprobe_ctlblk) {
    (*kcb).prev_kprobe.kp = kprobe_running();
    (*kcb).prev_kprobe.status = (*kcb).kprobe_status;
}

unsafe fn restore_previous_kprobe(kcb: *mut kprobe_ctlblk) {
    current_kprobe = (*kcb).prev_kprobe.kp;
    (*kcb).kprobe_status = (*kcb).prev_kprobe.status;
}

unsafe fn set_current_kprobe(p: *mut kprobe) {
    current_kprobe = p;
}

/*
 * Interrupts need to be disabled before single-step mode is set, and not
 * re-enabled until after single-step mode ends.
 * Without disabling interrupt on local CPU, there is a chance of
 * interrupt occurrence in the period of exception return and start of
 * out-of-line single-step, that result in wrongly single stepping
 * into the interrupt handler.
 */
unsafe fn kprobes_save_local_irqflag(kcb: *mut kprobe_ctlblk, regs: *mut pt_regs) {
    (*kcb).saved_status = (*regs).status;
    (*regs).status &= !SR_SPIE;
}

unsafe fn kprobes_restore_local_irqflag(kcb: *mut kprobe_ctlblk, regs: *mut pt_regs) {
    (*regs).status = (*kcb).saved_status;
}

unsafe fn setup_singlestep(p: *mut kprobe, regs: *mut pt_regs, kcb: *mut kprobe_ctlblk, reenter: i32) {
    let slot: usize;

    if reenter != 0 {
        save_previous_kprobe(kcb);
        set_current_kprobe(p);
        (*kcb).kprobe_status = KPROBE_REENTER;
    } else {
        (*kcb).kprobe_status = KPROBE_HIT_SS;
    }

    if !(*p).ainsn.api.insn.is_null() {
        /* prepare for single stepping */
        slot = (*p).ainsn.api.insn as usize;
        /* IRQs and single stepping do not mix well. */
        kprobes_save_local_irqflag(kcb, regs);
        instruction_pointer_set(regs, slot);
    } else {
        /* insn simulation */
        arch_simulate_insn(p, regs);
    }
}

unsafe fn reenter_kprobe(p: *mut kprobe, regs: *mut pt_regs, kcb: *mut kprobe_ctlblk) -> i32 {
    match (*kcb).kprobe_status {
        KPROBE_HIT_SSDONE | KPROBE_HIT_ACTIVE => {
            kprobes_inc_nmissed_count(p);
            setup_singlestep(p, regs, kcb, 1);
        }
        KPROBE_HIT_SS | KPROBE_REENTER => {
            pr_warn("Failed to recover from reentered kprobes.\n");
            dump_kprobe(p);
            BUG();
        }
        _ => {
            WARN_ON(1);
            return 0;
        }
    }
    1
}

unsafe fn post_kprobe_handler(cur: *mut kprobe, kcb: *mut kprobe_ctlblk, regs: *mut pt_regs) {
    /* return addr restore if non-branching insn */
    if (*cur).ainsn.api.restore != 0 {
        (*regs).epc = (*cur).ainsn.api.restore;
    }
    /* restore back original saved kprobe variables and continue */
    if (*kcb).kprobe_status == KPROBE_REENTER {
        restore_previous_kprobe(kcb);
        return;
    }
    /* call post handler */
    (*kcb).kprobe_status = KPROBE_HIT_SSDONE;
    if let Some(handler) = (*cur).post_handler {
        /* post_handler can hit breakpoint and single step again, so we enable D-flag for recursive exception. */
        handler(cur, regs, 0);
    }
    reset_current_kprobe();
}

pub unsafe fn kprobe_fault_handler(regs: *mut pt_regs, _trapnr: u32) -> i32 {
    let cur = kprobe_running();
    let kcb = get_kprobe_ctlblk();
    match (*kcb).kprobe_status {
        KPROBE_HIT_SS | KPROBE_REENTER => {
            (*regs).epc = (*cur).addr as usize;
            BUG_ON(!instruction_pointer(regs));
            if (*kcb).kprobe_status == KPROBE_REENTER { restore_previous_kprobe(kcb); }
            else { kprobes_restore_local_irqflag(kcb, regs); reset_current_kprobe(); }
        }
        KPROBE_HIT_ACTIVE | KPROBE_HIT_SSDONE => {
            if fixup_exception(regs) { return 1; }
        }
        _ => {}
    }
    0
}

pub unsafe fn kprobe_breakpoint_handler(regs: *mut pt_regs) -> bool {
    let addr = instruction_pointer(regs);
    let kcb = get_kprobe_ctlblk();
    let cur_kprobe = kprobe_running();
    let p = get_kprobe(addr as *mut kprobe_opcode_t);
    if !p.is_null() {
        if !cur_kprobe.is_null() {
            if reenter_kprobe(p, regs, kcb) != 0 { return true; }
        } else {
            set_current_kprobe(p);
            (*kcb).kprobe_status = KPROBE_HIT_ACTIVE;
            if (*p).pre_handler.is_none() || (*p).pre_handler.unwrap()(p, regs) == 0 { setup_singlestep(p, regs, kcb, 0); }
            else { reset_current_kprobe(); }
        }
        return true;
    }
    false
}

pub unsafe fn kprobe_single_step_handler(regs: *mut pt_regs) -> bool {
    let kcb = get_kprobe_ctlblk();
    let addr = instruction_pointer(regs);
    let cur = kprobe_running();
    if !cur.is_null() && ((*kcb).kprobe_status & (KPROBE_HIT_SS | KPROBE_REENTER)) != 0 &&
        ((&(*cur).ainsn.api.insn as *const _ as usize) + GET_INSN_LENGTH((*cur).opcode) as usize == addr) {
        kprobes_restore_local_irqflag(kcb, regs);
        post_kprobe_handler(cur, kcb, regs);
        return true;
    }
    false
}

/* Provide a blacklist of symbols identifying ranges which cannot be kprobed. */
pub unsafe fn arch_populate_kprobe_blacklist() -> i32 {
    kprobe_add_area_blacklist(__irqentry_text_start as usize, __irqentry_text_end as usize)
}

pub unsafe fn arch_trampoline_kprobe(_p: *mut kprobe) -> i32 { 0 }

pub unsafe fn arch_init_kprobes() -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
