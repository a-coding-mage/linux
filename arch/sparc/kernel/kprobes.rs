// SPDX-License-Identifier: GPL-2.0
/* arch/sparc64/kernel/kprobes.c */

// Kernel and architecture dependencies are supplied by the surrounding tree.

DEFINE_PER_CPU!(struct kprobe *, current_kprobe, core::ptr::null_mut());
DEFINE_PER_CPU!(struct kprobe_ctlblk, kprobe_ctlblk);

static mut kretprobe_blacklist: [kretprobe_blackpoint; 1] =
    [kretprobe_blackpoint { bitfield: [0; 2] }];

unsafe fn arch_prepare_kprobe(p: *mut kprobe) -> i32 {
    if ((*p).addr as usize & 0x3usize) != 0 {
        return -EILSEQ;
    }

    (*p).ainsn.insn[0] = *(*p).addr;
    flushi(&mut (*p).ainsn.insn[0]);

    (*p).ainsn.insn[1] = BREAKPOINT_INSTRUCTION_2;
    flushi(&mut (*p).ainsn.insn[1]);

    (*p).opcode = *(*p).addr;
    0
}

unsafe fn arch_arm_kprobe(p: *mut kprobe) {
    *(*p).addr = BREAKPOINT_INSTRUCTION;
    flushi((*p).addr);
}

unsafe fn arch_disarm_kprobe(p: *mut kprobe) {
    *(*p).addr = (*p).opcode;
    flushi((*p).addr);
}

unsafe fn save_previous_kprobe(kcb: *mut kprobe_ctlblk) {
    (*kcb).prev_kprobe.kp = kprobe_running();
    (*kcb).prev_kprobe.status = (*kcb).kprobe_status;
    (*kcb).prev_kprobe.orig_tnpc = (*kcb).kprobe_orig_tnpc;
    (*kcb).prev_kprobe.orig_tstate_pil = (*kcb).kprobe_orig_tstate_pil;
}

unsafe fn restore_previous_kprobe(kcb: *mut kprobe_ctlblk) {
    __this_cpu_write!(current_kprobe, (*kcb).prev_kprobe.kp);
    (*kcb).kprobe_status = (*kcb).prev_kprobe.status;
    (*kcb).kprobe_orig_tnpc = (*kcb).prev_kprobe.orig_tnpc;
    (*kcb).kprobe_orig_tstate_pil = (*kcb).prev_kprobe.orig_tstate_pil;
}

unsafe fn set_current_kprobe(p: *mut kprobe, regs: *mut pt_regs, kcb: *mut kprobe_ctlblk) {
    __this_cpu_write!(current_kprobe, p);
    (*kcb).kprobe_orig_tnpc = (*regs).tnpc;
    (*kcb).kprobe_orig_tstate_pil = (*regs).tstate & TSTATE_PIL;
}

unsafe fn prepare_singlestep(p: *mut kprobe, regs: *mut pt_regs, kcb: *mut kprobe_ctlblk) {
    (*regs).tstate |= TSTATE_PIL;
    if (*p).opcode == BREAKPOINT_INSTRUCTION {
        (*regs).tpc = (*p).addr as usize as u64;
        (*regs).tnpc = (*kcb).kprobe_orig_tnpc;
    } else {
        (*regs).tpc = (&mut (*p).ainsn.insn[0]) as *mut _ as usize as u64;
        (*regs).tnpc = (&mut (*p).ainsn.insn[1]) as *mut _ as usize as u64;
    }
}

unsafe fn kprobe_handler(regs: *mut pt_regs) -> i32 {
    let mut p: *mut kprobe;
    let addr = (*regs).tpc as *mut core::ffi::c_void;
    let mut ret = 0;
    let kcb: *mut kprobe_ctlblk;

    preempt_disable();
    kcb = get_kprobe_ctlblk();

    if !kprobe_running().is_null() {
        p = get_kprobe(addr);
        if !p.is_null() {
            if (*kcb).kprobe_status == KPROBE_HIT_SS {
                (*regs).tstate = ((*regs).tstate & !TSTATE_PIL) | (*kcb).kprobe_orig_tstate_pil;
                goto_no_kprobe!();
            }
            save_previous_kprobe(kcb);
            set_current_kprobe(p, regs, kcb);
            kprobes_inc_nmissed_count(p);
            (*kcb).kprobe_status = KPROBE_REENTER;
            prepare_singlestep(p, regs, kcb);
            return 1;
        } else if *(addr as *mut u32) != BREAKPOINT_INSTRUCTION {
            ret = 1;
        }
        goto_no_kprobe!();
    }

    p = get_kprobe(addr);
    if p.is_null() {
        if *(addr as *mut u32) != BREAKPOINT_INSTRUCTION { ret = 1; }
        goto_no_kprobe!();
    }

    set_current_kprobe(p, regs, kcb);
    (*kcb).kprobe_status = KPROBE_HIT_ACTIVE;
    if !(*p).pre_handler.is_none() && (*p).pre_handler.unwrap()(p, regs) != 0 {
        reset_current_kprobe();
        preempt_enable_no_resched();
        return 1;
    }
    prepare_singlestep(p, regs, kcb);
    (*kcb).kprobe_status = KPROBE_HIT_SS;
    return 1;

    goto_no_kprobe!();
    preempt_enable_no_resched();
    ret
}

unsafe fn relbranch_fixup(insn: u32, p: *mut kprobe, regs: *mut pt_regs) -> u64 {
    let real_pc = (*p).addr as usize as u64;
    if (*regs).tnpc == (*regs).tpc + 0x4 { return real_pc + 0x8; }
    if (insn & 0xc0000000) == 0x40000000 || (insn & 0xc1c00000) == 0x00400000 || (insn & 0xc1c00000) == 0x00800000 {
        let ainsn_addr = (&(*p).ainsn.insn[0]) as *const _ as usize as u64;
        return real_pc + ((*regs).tnpc - ainsn_addr);
    }
    (*regs).tnpc
}

unsafe fn retpc_fixup(regs: *mut pt_regs, insn: u32, real_pc: u64) {
    let mut slot: *mut u64 = core::ptr::null_mut();
    if (insn & 0xc0000000) == 0x40000000 { slot = &mut (*regs).u_regs[UREG_I7]; }
    if (insn & 0xc1f80000) == 0x81c00000 {
        let mut rd = ((insn >> 25) & 0x1f) as usize;
        if rd <= 15 { slot = &mut (*regs).u_regs[rd]; }
        else { flushw_all(); rd -= 16; slot = ((*regs).u_regs[UREG_FP] + STACK_BIAS) as *mut u64; slot = slot.add(rd); }
    }
    if !slot.is_null() { *slot = real_pc; }
}

unsafe fn resume_execution(p: *mut kprobe, regs: *mut pt_regs, kcb: *mut kprobe_ctlblk) {
    let insn = (*p).ainsn.insn[0];
    (*regs).tnpc = relbranch_fixup(insn, p, regs);
    (*regs).tpc = (*kcb).kprobe_orig_tnpc;
    retpc_fixup(regs, insn, (*p).addr as usize as u64);
    (*regs).tstate = ((*regs).tstate & !TSTATE_PIL) | (*kcb).kprobe_orig_tstate_pil;
}

unsafe fn post_kprobe_handler(regs: *mut pt_regs) -> i32 {
    let cur = kprobe_running();
    let kcb = get_kprobe_ctlblk();
    if cur.is_null() { return 0; }
    if (*kcb).kprobe_status != KPROBE_REENTER { if let Some(handler) = (*cur).post_handler { (*kcb).kprobe_status = KPROBE_HIT_SSDONE; handler(cur, regs, 0); } }
    resume_execution(cur, regs, kcb);
    if (*kcb).kprobe_status == KPROBE_REENTER { restore_previous_kprobe(kcb); } else { reset_current_kprobe(); }
    preempt_enable_no_resched();
    1
}

unsafe fn kprobe_fault_handler(regs: *mut pt_regs, _trapnr: i32) -> i32 {
    let cur = kprobe_running();
    let kcb = get_kprobe_ctlblk();
    match (*kcb).kprobe_status {
        KPROBE_HIT_SS | KPROBE_REENTER => {
            (*regs).tpc = (*cur).addr as usize as u64; (*regs).tnpc = (*kcb).kprobe_orig_tnpc;
            (*regs).tstate = ((*regs).tstate & !TSTATE_PIL) | (*kcb).kprobe_orig_tstate_pil;
            if (*kcb).kprobe_status == KPROBE_REENTER { restore_previous_kprobe(kcb); } else { reset_current_kprobe(); }
            preempt_enable_no_resched();
        }
        KPROBE_HIT_ACTIVE | KPROBE_HIT_SSDONE => {
            let entry = search_exception_tables((*regs).tpc);
            if !entry.is_null() { (*regs).tpc = (*entry).fixup; (*regs).tnpc = (*regs).tpc + 4; return 1; }
        }
        _ => {}
    }
    0
}

unsafe fn kprobe_exceptions_notify(_self: *mut notifier_block, val: usize, data: *mut core::ffi::c_void) -> i32 {
    let args = data as *mut die_args;
    let mut ret = NOTIFY_DONE;
    if !(*args).regs.is_null() && user_mode((*args).regs) { return ret; }
    match val { DIE_DEBUG => if kprobe_handler((*args).regs) != 0 { ret = NOTIFY_STOP; }, DIE_DEBUG_2 => if post_kprobe_handler((*args).regs) != 0 { ret = NOTIFY_STOP; }, _ => {} }
    ret
}

unsafe fn kprobe_trap(trap_level: usize, regs: *mut pt_regs) {
    let prev_state = exception_enter();
    BUG_ON!(trap_level != 0x170 && trap_level != 0x171);
    if user_mode(regs) { local_irq_enable(); bad_trap(regs, trap_level); exception_exit(prev_state); return; }
    if notify_die(if trap_level == 0x170 { DIE_DEBUG } else { DIE_DEBUG_2 }, if trap_level == 0x170 { "debug" } else { "debug_2" }, regs, 0, trap_level, SIGTRAP) != NOTIFY_STOP { bad_trap(regs, trap_level); }
    exception_exit(prev_state);
}

unsafe fn arch_prepare_kretprobe(ri: *mut kretprobe_instance, regs: *mut pt_regs) {
    (*ri).ret_addr = ( (*regs).u_regs[UREG_RETPC] + 8) as *mut kprobe_opcode_t;
    (*ri).fp = core::ptr::null_mut();
    (*regs).u_regs[UREG_RETPC] = __kretprobe_trampoline as usize as u64 - 8;
}

unsafe fn trampoline_probe_handler(_p: *mut kprobe, regs: *mut pt_regs) -> i32 {
    let orig_ret_address = __kretprobe_trampoline_handler(regs, core::ptr::null_mut());
    (*regs).tpc = orig_ret_address; (*regs).tnpc = orig_ret_address + 4; 1
}

#[used]
unsafe fn kretprobe_trampoline_holder() { core::arch::asm!(".global __kretprobe_trampoline\n__kretprobe_trampoline:\n\tnop\n\tnop\n"); }

static mut trampoline_p: kprobe = kprobe { addr: __kretprobe_trampoline as *mut kprobe_opcode_t, pre_handler: Some(trampoline_probe_handler) };

unsafe fn arch_init_kprobes() -> i32 { register_kprobe(&mut trampoline_p) }

unsafe fn arch_trampoline_kprobe(p: *mut kprobe) -> i32 {
    if (*p).addr == __kretprobe_trampoline as *mut kprobe_opcode_t { 1 } else { 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
