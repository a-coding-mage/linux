// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

// Dependencies supplied by the Linux kernel and architecture-specific headers.

const MIN_STACK_SIZE: unsafe fn(usize) -> usize = |addr| unsafe {
    core::cmp::min(MAX_STACK_SIZE as usize,
        current_thread_info() as usize + THREAD_SIZE as usize - addr)
};

static mut CURRENT_KPROBE: *mut kprobe = core::ptr::null_mut();
static mut KPROBE_CTLBLK: kprobe_ctlblk = unsafe { core::mem::zeroed() };

pub unsafe fn arch_prepare_kprobe(p: *mut kprobe) -> i32 {
    if (p as usize) & 0x01 != 0 { return -EINVAL; }
    (*p).ainsn.is_short = is_short_instr((*p).addr as usize);
    (*p).opcode = *(*p).addr;
    0
}

pub unsafe fn arch_arm_kprobe(p: *mut kprobe) {
    *(*p).addr = UNIMP_S_INSTRUCTION;
    flush_icache_range((*p).addr as usize, (*p).addr as usize + core::mem::size_of::<kprobe_opcode_t>());
}

pub unsafe fn arch_disarm_kprobe(p: *mut kprobe) {
    *(*p).addr = (*p).opcode;
    flush_icache_range((*p).addr as usize, (*p).addr as usize + core::mem::size_of::<kprobe_opcode_t>());
}

pub unsafe fn arch_remove_kprobe(p: *mut kprobe) {
    arch_disarm_kprobe(p);
    if !(*p).ainsn.t1_addr.is_null() {
        *(*p).ainsn.t1_addr = (*p).ainsn.t1_opcode;
        flush_icache_range((*p).ainsn.t1_addr as usize, (*p).ainsn.t1_addr as usize + core::mem::size_of::<kprobe_opcode_t>());
        (*p).ainsn.t1_addr = core::ptr::null_mut();
    }
    if !(*p).ainsn.t2_addr.is_null() {
        *(*p).ainsn.t2_addr = (*p).ainsn.t2_opcode;
        flush_icache_range((*p).ainsn.t2_addr as usize, (*p).ainsn.t2_addr as usize + core::mem::size_of::<kprobe_opcode_t>());
        (*p).ainsn.t2_addr = core::ptr::null_mut();
    }
}

unsafe fn save_previous_kprobe(kcb: *mut kprobe_ctlblk) {
    (*kcb).prev_kprobe.kp = kprobe_running();
    (*kcb).prev_kprobe.status = (*kcb).kprobe_status;
}

unsafe fn restore_previous_kprobe(kcb: *mut kprobe_ctlblk) {
    CURRENT_KPROBE = (*kcb).prev_kprobe.kp;
    (*kcb).kprobe_status = (*kcb).prev_kprobe.status;
}

#[inline]
unsafe fn set_current_kprobe(p: *mut kprobe) { CURRENT_KPROBE = p; }

unsafe fn resume_execution(p: *mut kprobe, _addr: usize, _regs: *mut pt_regs) {
    if !(*p).ainsn.t1_addr.is_null() {
        *(*p).ainsn.t1_addr = (*p).ainsn.t1_opcode;
        flush_icache_range((*p).ainsn.t1_addr as usize, (*p).ainsn.t1_addr as usize + core::mem::size_of::<kprobe_opcode_t>());
        (*p).ainsn.t1_addr = core::ptr::null_mut();
    }
    if !(*p).ainsn.t2_addr.is_null() {
        *(*p).ainsn.t2_addr = (*p).ainsn.t2_opcode;
        flush_icache_range((*p).ainsn.t2_addr as usize, (*p).ainsn.t2_addr as usize + core::mem::size_of::<kprobe_opcode_t>());
        (*p).ainsn.t2_addr = core::ptr::null_mut();
    }
}

unsafe fn setup_singlestep(p: *mut kprobe, regs: *mut pt_regs) {
    let mut next_pc: usize;
    let mut tgt_if_br: usize = 0;
    let is_branch: i32;
    let bta = (*regs).bta;
    *(*p).addr = (*p).opcode;
    flush_icache_range((*p).addr as usize, (*p).addr as usize + core::mem::size_of::<kprobe_opcode_t>());
    if (*regs).status32 & 0x40 != 0 {
        next_pc = bta & !0x01;
        if !(*p).ainsn.is_short {
            if bta & 0x01 != 0 { (*regs).blink += 2; }
            else { next_pc += 2; (*regs).bta += 2; }
        }
        is_branch = 0;
    } else {
        is_branch = disasm_next_pc((*p).addr as usize, regs,
            (*current).thread.callee_reg as *mut callee_regs, &mut next_pc, &mut tgt_if_br);
    }
    (*p).ainsn.t1_addr = next_pc as *mut kprobe_opcode_t;
    (*p).ainsn.t1_opcode = *(*p).ainsn.t1_addr;
    *(*p).ainsn.t1_addr = TRAP_S_2_INSTRUCTION;
    flush_icache_range((*p).ainsn.t1_addr as usize, (*p).ainsn.t1_addr as usize + core::mem::size_of::<kprobe_opcode_t>());
    if is_branch != 0 {
        (*p).ainsn.t2_addr = tgt_if_br as *mut kprobe_opcode_t;
        (*p).ainsn.t2_opcode = *(*p).ainsn.t2_addr;
        *(*p).ainsn.t2_addr = TRAP_S_2_INSTRUCTION;
        flush_icache_range((*p).ainsn.t2_addr as usize, (*p).ainsn.t2_addr as usize + core::mem::size_of::<kprobe_opcode_t>());
    }
}

unsafe fn arc_kprobe_handler(addr: usize, regs: *mut pt_regs) -> i32 {
    preempt_disable();
    let kcb = get_kprobe_ctlblk();
    let p = get_kprobe(addr as *mut usize);
    if !p.is_null() {
        if !kprobe_running().is_null() {
            save_previous_kprobe(kcb); set_current_kprobe(p); kprobes_inc_nmissed_count(p);
            setup_singlestep(p, regs); (*kcb).kprobe_status = KPROBE_REENTER; return 1;
        }
        set_current_kprobe(p); (*kcb).kprobe_status = KPROBE_HIT_ACTIVE;
        if (*p).pre_handler.is_none() || (*p).pre_handler.unwrap()(p, regs) == 0 {
            setup_singlestep(p, regs); (*kcb).kprobe_status = KPROBE_HIT_SS;
        } else { reset_current_kprobe(); preempt_enable_no_resched(); }
        return 1;
    }
    preempt_enable_no_resched(); 0
}

unsafe fn arc_post_kprobe_handler(addr: usize, regs: *mut pt_regs) -> i32 {
    let cur = kprobe_running(); let kcb = get_kprobe_ctlblk();
    if cur.is_null() { return 0; }
    resume_execution(cur, addr, regs); arch_arm_kprobe(cur); (*regs).ret = addr;
    if (*kcb).kprobe_status != KPROBE_REENTER && (*cur).post_handler.is_some() {
        (*kcb).kprobe_status = KPROBE_HIT_SSDONE; (*cur).post_handler.unwrap()(cur, regs, 0);
    }
    if (*kcb).kprobe_status == KPROBE_REENTER { restore_previous_kprobe(kcb); }
    else { reset_current_kprobe(); }
    preempt_enable_no_resched(); 1
}

pub unsafe fn kprobe_fault_handler(regs: *mut pt_regs, _trapnr: usize) -> i32 {
    let cur = kprobe_running(); let kcb = get_kprobe_ctlblk();
    match (*kcb).kprobe_status {
        KPROBE_HIT_SS | KPROBE_REENTER => { resume_execution(cur, (*cur).addr as usize, regs); if (*kcb).kprobe_status == KPROBE_REENTER { restore_previous_kprobe(kcb); } else { reset_current_kprobe(); } preempt_enable_no_resched(); }
        KPROBE_HIT_ACTIVE | KPROBE_HIT_SSDONE => { if fixup_exception(regs) != 0 { return 1; } }
        _ => {}
    } 0
}

pub unsafe fn kprobe_exceptions_notify(_self_: *mut notifier_block, val: usize, data: *mut core::ffi::c_void) -> i32 {
    let args = data as *mut die_args; let addr = (*args).err;
    match val { DIE_IERR => if arc_kprobe_handler(addr, (*args).regs) != 0 { return NOTIFY_STOP; }, DIE_TRAP => if arc_post_kprobe_handler(addr, (*args).regs) != 0 { return NOTIFY_STOP; }, _ => {} }
    NOTIFY_DONE
}

#[used]
unsafe fn kretprobe_trampoline_holder() { /* .global __kretprobe_trampoline; __kretprobe_trampoline: nop */ }

pub unsafe fn arch_prepare_kretprobe(ri: *mut kretprobe_instance, regs: *mut pt_regs) {
    (*ri).ret_addr = (*regs).blink as *mut kprobe_opcode_t; (*ri).fp = core::ptr::null_mut();
    (*regs).blink = &__kretprobe_trampoline as *const _ as usize;
}

unsafe fn trampoline_probe_handler(_p: *mut kprobe, regs: *mut pt_regs) -> i32 {
    (*regs).ret = __kretprobe_trampoline_handler(regs, core::ptr::null_mut()); 1
}

static mut trampoline_p: kprobe = kprobe { addr: &__kretprobe_trampoline as *const _ as *mut kprobe_opcode_t, pre_handler: Some(trampoline_probe_handler) };

pub unsafe fn arch_init_kprobes() -> i32 { register_kprobe(&mut trampoline_p) }

pub unsafe fn arch_trampoline_kprobe(p: *mut kprobe) -> i32 {
    if (*p).addr == &__kretprobe_trampoline as *const _ as *mut kprobe_opcode_t { 1 } else { 0 }
}

pub unsafe fn trap_is_kprobe(address: usize, regs: *mut pt_regs) {
    notify_die(DIE_TRAP, b"kprobe_trap\0".as_ptr() as *const i8, regs, address, 0, SIGTRAP);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
