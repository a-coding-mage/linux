// SPDX-License-Identifier: GPL-2.0-only
// arch/arm64/kernel/probes/kprobes.c

// Kernel dependencies supplied by other translation units are intentionally
// referenced here rather than reimplemented.

pub static mut CURRENT_KPROBE: *mut kprobe = core::ptr::null_mut();
pub static mut KPROBE_CTLBLK: kprobe_ctlblk = unsafe { core::mem::zeroed() };

unsafe extern "C" {
    fn post_kprobe_handler(p: *mut kprobe, kcb: *mut kprobe_ctlblk, regs: *mut pt_regs);
}

pub unsafe fn alloc_insn_page() -> *mut core::ffi::c_void {
    let addr = execmem_alloc(EXECMEM_KPROBES, PAGE_SIZE);
    if addr.is_null() { return core::ptr::null_mut(); }
    if set_memory_rox(addr as usize, 1) != 0 {
        execmem_free(addr);
        return core::ptr::null_mut();
    }
    addr
}

unsafe fn arch_prepare_ss_slot(p: *mut kprobe) {
    let addr = (*p).ainsn.xol_insn;
    aarch64_insn_patch_text_nosync(addr, le32_to_cpu((*p).opcode));
    aarch64_insn_patch_text_nosync(addr.add(1), BRK64_OPCODE_KPROBES_SS);
    (*p).ainsn.xol_restore = (*p).addr as usize + core::mem::size_of::<kprobe_opcode_t>();
}

unsafe fn arch_prepare_simulate(p: *mut kprobe) { (*p).ainsn.xol_restore = 0; }

unsafe fn arch_simulate_insn(p: *mut kprobe, regs: *mut pt_regs) {
    let kcb = get_kprobe_ctlblk();
    if let Some(handler) = (*p).ainsn.api.handler {
        handler(le32_to_cpu((*p).opcode), (*p).addr as isize, regs);
    }
    post_kprobe_handler(p, kcb, regs);
}

pub unsafe fn arch_prepare_kprobe(p: *mut kprobe) -> i32 {
    let probe_addr = (*p).addr as usize;
    if probe_addr & 3 != 0 { return -EINVAL; }
    (*p).opcode = *(*p).addr;
    if !search_exception_tables(probe_addr).is_null() { return -EINVAL; }
    match arm_kprobe_decode_insn((*p).addr, &mut (*p).ainsn) {
        INSN_REJECTED => return -EINVAL,
        INSN_GOOD_NO_SLOT => (*p).ainsn.xol_insn = core::ptr::null_mut(),
        INSN_GOOD => {
            (*p).ainsn.xol_insn = get_insn_slot();
            if (*p).ainsn.xol_insn.is_null() { return -ENOMEM; }
        }
        _ => {}
    }
    if !(*p).ainsn.xol_insn.is_null() { arch_prepare_ss_slot(p); }
    else { arch_prepare_simulate(p); }
    0
}

pub unsafe fn arch_arm_kprobe(p: *mut kprobe) {
    let mut addr = (*p).addr as *mut core::ffi::c_void;
    let mut insn = BRK64_OPCODE_KPROBES;
    aarch64_insn_patch_text(&mut addr, &mut insn, 1);
}

pub unsafe fn arch_disarm_kprobe(p: *mut kprobe) {
    let mut addr = (*p).addr as *mut core::ffi::c_void;
    let mut insn = le32_to_cpu((*p).opcode);
    aarch64_insn_patch_text(&mut addr, &mut insn, 1);
}

pub unsafe fn arch_remove_kprobe(p: *mut kprobe) {
    if !(*p).ainsn.xol_insn.is_null() {
        free_insn_slot((*p).ainsn.xol_insn, 0);
        (*p).ainsn.xol_insn = core::ptr::null_mut();
    }
}

unsafe fn save_previous_kprobe(kcb: *mut kprobe_ctlblk) {
    (*kcb).prev_kprobe.kp = kprobe_running();
    (*kcb).prev_kprobe.status = (*kcb).kprobe_status;
    (*kcb).prev_kprobe.saved_irqflag = (*kcb).saved_irqflag;
}
unsafe fn restore_previous_kprobe(kcb: *mut kprobe_ctlblk) {
    CURRENT_KPROBE = (*kcb).prev_kprobe.kp;
    (*kcb).kprobe_status = (*kcb).prev_kprobe.status;
    (*kcb).saved_irqflag = (*kcb).prev_kprobe.saved_irqflag;
}
unsafe fn set_current_kprobe(p: *mut kprobe) { CURRENT_KPROBE = p; }
unsafe fn kprobes_save_local_irqflag(kcb: *mut kprobe_ctlblk, regs: *mut pt_regs) {
    (*kcb).saved_irqflag = (*regs).pstate & DAIF_MASK;
    (*regs).pstate |= DAIF_MASK;
}
unsafe fn kprobes_restore_local_irqflag(kcb: *mut kprobe_ctlblk, regs: *mut pt_regs) {
    (*regs).pstate &= !DAIF_MASK;
    (*regs).pstate |= (*kcb).saved_irqflag;
}

unsafe fn setup_singlestep(p: *mut kprobe, regs: *mut pt_regs, kcb: *mut kprobe_ctlblk, reenter: i32) {
    let slot: usize;
    if reenter != 0 {
        save_previous_kprobe(kcb); set_current_kprobe(p); (*kcb).kprobe_status = KPROBE_REENTER;
    } else { (*kcb).kprobe_status = KPROBE_HIT_SS; }
    if !(*p).ainsn.xol_insn.is_null() {
        slot = (*p).ainsn.xol_insn as usize;
        kprobes_save_local_irqflag(kcb, regs); instruction_pointer_set(regs, slot);
    } else { arch_simulate_insn(p, regs); }
}

unsafe fn reenter_kprobe(p: *mut kprobe, regs: *mut pt_regs, kcb: *mut kprobe_ctlblk) -> i32 {
    match (*kcb).kprobe_status {
        KPROBE_HIT_SSDONE | KPROBE_HIT_ACTIVE | KPROBE_HIT_SS => { kprobes_inc_nmissed_count(p); setup_singlestep(p, regs, kcb, 1); }
        KPROBE_REENTER => { pr_warn!("Failed to recover from reentered kprobes.\n"); dump_kprobe(p); BUG!(); }
        _ => { WARN_ON!(true); return 0; }
    } 1
}

unsafe fn post_kprobe_handler_impl(cur: *mut kprobe, kcb: *mut kprobe_ctlblk, regs: *mut pt_regs) {
    if (*cur).ainsn.xol_restore != 0 { instruction_pointer_set(regs, (*cur).ainsn.xol_restore); }
    if (*kcb).kprobe_status == KPROBE_REENTER { restore_previous_kprobe(kcb); return; }
    (*kcb).kprobe_status = KPROBE_HIT_SSDONE;
    if let Some(handler) = (*cur).post_handler { handler(cur, regs, 0); }
    reset_current_kprobe();
}

pub unsafe fn kprobe_fault_handler(regs: *mut pt_regs, _fsr: u32) -> i32 {
    let cur = kprobe_running(); let kcb = get_kprobe_ctlblk();
    if !cur.is_null() && (*cur).ainsn.xol_insn.is_null() { return 0; }
    match (*kcb).kprobe_status {
        KPROBE_HIT_SS | KPROBE_REENTER => {
            if instruction_pointer(regs) != (*cur).ainsn.xol_insn as usize { return 0; }
            instruction_pointer_set(regs, (*cur).addr as usize); BUG_ON!(instruction_pointer(regs) == 0);
            if (*kcb).kprobe_status == KPROBE_REENTER { restore_previous_kprobe(kcb); }
            else { kprobes_restore_local_irqflag(kcb, regs); reset_current_kprobe(); }
        }
        _ => {}
    } 0
}

pub unsafe fn kprobe_brk_handler(regs: *mut pt_regs, _esr: usize) -> i32 {
    let kcb = get_kprobe_ctlblk(); let cur_kprobe = kprobe_running();
    let p = get_kprobe(instruction_pointer(regs) as *mut kprobe_opcode_t);
    if WARN_ON_ONCE!(p.is_null()) { return DBG_HOOK_ERROR; }
    if !cur_kprobe.is_null() { if reenter_kprobe(p, regs, kcb) == 0 { return DBG_HOOK_ERROR; } }
    else { set_current_kprobe(p); (*kcb).kprobe_status = KPROBE_HIT_ACTIVE;
        if (*p).pre_handler.is_none() || (*p).pre_handler.unwrap()(p, regs) == 0 { setup_singlestep(p, regs, kcb, 0); } else { reset_current_kprobe(); }
    } DBG_HOOK_HANDLED
}

pub unsafe fn kprobe_ss_brk_handler(regs: *mut pt_regs, _esr: usize) -> i32 {
    let kcb = get_kprobe_ctlblk(); let addr = instruction_pointer(regs); let cur = kprobe_running();
    if !cur.is_null() && ((*kcb).kprobe_status & (KPROBE_HIT_SS | KPROBE_REENTER)) != 0 && (&(*cur).ainsn.xol_insn.add(1) as *const _ as usize) == addr {
        kprobes_restore_local_irqflag(kcb, regs); post_kprobe_handler_impl(cur, kcb, regs); return DBG_HOOK_HANDLED;
    } DBG_HOOK_ERROR
}

pub unsafe fn kretprobe_brk_handler(regs: *mut pt_regs, _esr: usize) -> i32 {
    if (*regs).pc != __kretprobe_trampoline as usize { return DBG_HOOK_ERROR; }
    (*regs).pc = kretprobe_trampoline_handler(regs, (*regs).regs[29] as *mut core::ffi::c_void); DBG_HOOK_HANDLED
}

pub unsafe fn arch_populate_kprobe_blacklist() -> i32 {
    let mut ret = kprobe_add_area_blacklist(__entry_text_start as usize, __entry_text_end as usize); if ret != 0 { return ret; }
    ret = kprobe_add_area_blacklist(__irqentry_text_start as usize, __irqentry_text_end as usize); if ret != 0 { return ret; }
    ret = kprobe_add_area_blacklist(__hyp_text_start as usize, __hyp_text_end as usize); if ret != 0 || is_kernel_in_hyp_mode() { return ret; }
    kprobe_add_area_blacklist(__hyp_idmap_text_start as usize, __hyp_idmap_text_end as usize)
}

pub unsafe fn arch_prepare_kretprobe(ri: *mut kretprobe_instance, regs: *mut pt_regs) {
    (*ri).ret_addr = (*regs).regs[30] as *mut kprobe_opcode_t; (*ri).fp = (*regs).regs[29] as *mut core::ffi::c_void; (*regs).regs[30] = __kretprobe_trampoline as usize as isize;
}
pub unsafe fn arch_trampoline_kprobe(_p: *mut kprobe) -> i32 { 0 }
pub unsafe fn arch_init_kprobes() -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
