// SPDX-License-Identifier: GPL-2.0+

// Kernel dependencies supplied by the surrounding Rust translation unit.

extern "C" {
    static mut current_kprobe: *mut kprobe;
    static mut kprobe_ctlblk: kprobe_ctlblk;
    fn num_online_cpus() -> c_int;
    fn cpu_relax();
    fn dcache_wb_range(start: c_uint, end: c_uint);
    fn icache_inv_range(start: c_uint, end: c_uint);
    fn stop_machine_cpuslocked(cb: unsafe extern "C" fn(*mut c_void) -> c_int,
                               data: *mut c_void, mask: *mut c_void) -> c_int;
    fn get_kprobe_ctlblk() -> *mut kprobe_ctlblk;
    fn is_insn32(opcode: u32) -> bool;
    fn csky_probe_decode_insn(addr: *mut kprobe_opcode_t, api: *mut c_void) -> c_int;
    fn get_insn_slot() -> *mut kprobe_opcode_t;
    fn free_insn_slot(slot: *mut kprobe_opcode_t, dirty: c_int);
    fn kprobe_running() -> *mut kprobe;
    fn reset_current_kprobe();
    fn kprobes_inc_nmissed_count(p: *mut kprobe);
    fn instruction_pointer_set(regs: *mut pt_regs, value: c_ulong);
    fn instruction_pointer(regs: *mut pt_regs) -> c_ulong;
    fn get_kprobe(addr: *mut kprobe_opcode_t) -> *mut kprobe;
    fn dump_kprobe(p: *mut kprobe);
    fn fixup_exception(regs: *mut pt_regs) -> c_int;
    fn kprobe_add_area_blacklist(start: c_ulong, end: c_ulong) -> c_int;
    fn kretprobe_trampoline_handler(regs: *mut pt_regs, data: *mut c_void) -> *mut c_void;
    static mut __kretprobe_trampoline: c_ulong;
    static mut __irqentry_text_start: c_ulong;
    static mut __irqentry_text_end: c_ulong;
}

#[repr(C)]
pub struct csky_insn_patch {
    pub addr: *mut kprobe_opcode_t,
    pub opcode: u32,
    pub cpu_count: atomic_t,
}

unsafe extern "C" fn patch_text_cb(priv_: *mut c_void) -> c_int {
    let param = priv_ as *mut csky_insn_patch;
    let addr = (*param).addr as c_uint;
    if atomic_inc_return(&mut (*param).cpu_count) == num_online_cpus() {
        *(addr as *mut u16) = cpu_to_le16((*param).opcode as u16);
        dcache_wb_range(addr, addr.wrapping_add(2));
        atomic_inc(&mut (*param).cpu_count);
    } else {
        while atomic_read(&(*param).cpu_count) <= num_online_cpus() { cpu_relax(); }
    }
    icache_inv_range(addr, addr.wrapping_add(2));
    0
}

unsafe fn patch_text(addr: *mut kprobe_opcode_t, opcode: u32) -> c_int {
    let mut param = csky_insn_patch { addr, opcode, cpu_count: ATOMIC_INIT(0) };
    stop_machine_cpuslocked(patch_text_cb, &mut param as *mut _ as *mut c_void, cpu_online_mask)
}

unsafe fn arch_prepare_ss_slot(p: *mut kprobe) {
    let offset: c_ulong = if is_insn32((*p).opcode) { 4 } else { 2 };
    (*p).ainsn.api.restore = (*p).addr as c_ulong + offset;
    patch_text((*p).ainsn.api.insn, (*p).opcode);
}

unsafe fn arch_prepare_simulate(p: *mut kprobe) { (*p).ainsn.api.restore = 0; }

unsafe fn arch_simulate_insn(p: *mut kprobe, regs: *mut pt_regs) {
    let kcb = get_kprobe_ctlblk();
    if let Some(handler) = (*p).ainsn.api.handler {
        handler((*p).opcode, (*p).addr as c_long, regs);
    }
    post_kprobe_handler(kcb, regs);
}

pub unsafe extern "C" fn arch_prepare_kprobe(p: *mut kprobe) -> c_int {
    let probe_addr = (*p).addr as c_ulong;
    if probe_addr & 1 != 0 { return -EILSEQ; }
    (*p).opcode = le32_to_cpu(*(*p).addr);
    match csky_probe_decode_insn((*p).addr, &mut (*p).ainsn.api as *mut _ as *mut c_void) {
        INSN_REJECTED => return -EINVAL,
        INSN_GOOD_NO_SLOT => (*p).ainsn.api.insn = core::ptr::null_mut(),
        INSN_GOOD => {
            (*p).ainsn.api.insn = get_insn_slot();
            if (*p).ainsn.api.insn.is_null() { return -ENOMEM; }
        }
        _ => {}
    }
    if !(*p).ainsn.api.insn.is_null() { arch_prepare_ss_slot(p); } else { arch_prepare_simulate(p); }
    0
}

pub unsafe extern "C" fn arch_arm_kprobe(p: *mut kprobe) { patch_text((*p).addr, USR_BKPT); }
pub unsafe extern "C" fn arch_disarm_kprobe(p: *mut kprobe) { patch_text((*p).addr, (*p).opcode); }
pub unsafe extern "C" fn arch_remove_kprobe(p: *mut kprobe) {
    if !(*p).ainsn.api.insn.is_null() { free_insn_slot((*p).ainsn.api.insn, 0); (*p).ainsn.api.insn = core::ptr::null_mut(); }
}

unsafe fn save_previous_kprobe(kcb: *mut kprobe_ctlblk) { (*kcb).prev_kprobe.kp = kprobe_running(); (*kcb).prev_kprobe.status = (*kcb).kprobe_status; }
unsafe fn restore_previous_kprobe(kcb: *mut kprobe_ctlblk) { current_kprobe = (*kcb).prev_kprobe.kp; (*kcb).kprobe_status = (*kcb).prev_kprobe.status; }
unsafe fn set_current_kprobe(p: *mut kprobe) { current_kprobe = p; }
unsafe fn kprobes_save_local_irqflag(kcb: *mut kprobe_ctlblk, regs: *mut pt_regs) { (*kcb).saved_sr = (*regs).sr; (*regs).sr &= !BIT(6); }
unsafe fn kprobes_restore_local_irqflag(kcb: *mut kprobe_ctlblk, regs: *mut pt_regs) { (*regs).sr = (*kcb).saved_sr; }
unsafe fn set_ss_context(kcb: *mut kprobe_ctlblk, addr: c_ulong, p: *mut kprobe) { (*kcb).ss_ctx.ss_pending = true; (*kcb).ss_ctx.match_addr = addr + if is_insn32((*p).opcode) { 4 } else { 2 }; }
unsafe fn clear_ss_context(kcb: *mut kprobe_ctlblk) { (*kcb).ss_ctx.ss_pending = false; (*kcb).ss_ctx.match_addr = 0; }

pub const TRACE_MODE_SI: u32 = BIT(14);
pub const TRACE_MODE_MASK: u32 = !(0x3 << 14);
pub const TRACE_MODE_RUN: u32 = 0;

unsafe fn setup_singlestep(p: *mut kprobe, regs: *mut pt_regs, kcb: *mut kprobe_ctlblk, reenter: c_int) {
    if reenter != 0 { save_previous_kprobe(kcb); set_current_kprobe(p); (*kcb).kprobe_status = KPROBE_REENTER; } else { (*kcb).kprobe_status = KPROBE_HIT_SS; }
    if !(*p).ainsn.api.insn.is_null() {
        let slot = (*p).ainsn.api.insn as c_ulong; set_ss_context(kcb, slot, p); kprobes_save_local_irqflag(kcb, regs); (*regs).sr = ((*regs).sr & TRACE_MODE_MASK) | TRACE_MODE_SI; instruction_pointer_set(regs, slot);
    } else { arch_simulate_insn(p, regs); }
}

unsafe fn reenter_kprobe(p: *mut kprobe, regs: *mut pt_regs, kcb: *mut kprobe_ctlblk) -> c_int {
    match (*kcb).kprobe_status { KPROBE_HIT_SSDONE | KPROBE_HIT_ACTIVE => { kprobes_inc_nmissed_count(p); setup_singlestep(p, regs, kcb, 1); }, KPROBE_HIT_SS | KPROBE_REENTER => { BUG(); }, _ => { WARN_ON(1); return 0; } } 1
}

unsafe fn post_kprobe_handler(kcb: *mut kprobe_ctlblk, regs: *mut pt_regs) {
    let cur = kprobe_running(); if cur.is_null() { return; }
    if (*cur).ainsn.api.restore != 0 { (*regs).pc = (*cur).ainsn.api.restore; }
    if (*kcb).kprobe_status == KPROBE_REENTER { restore_previous_kprobe(kcb); return; }
    (*kcb).kprobe_status = KPROBE_HIT_SSDONE;
    if let Some(handler) = (*cur).post_handler { handler(cur, regs, 0); }
    reset_current_kprobe();
}

pub unsafe extern "C" fn kprobe_fault_handler(regs: *mut pt_regs, _trapnr: c_uint) -> c_int {
    let cur = kprobe_running(); let kcb = get_kprobe_ctlblk();
    match (*kcb).kprobe_status { KPROBE_HIT_SS | KPROBE_REENTER => { (*regs).pc = (*cur).addr as c_ulong; BUG_ON(instruction_pointer(regs) == 0); if (*kcb).kprobe_status == KPROBE_REENTER { restore_previous_kprobe(kcb); } else { reset_current_kprobe(); } }, KPROBE_HIT_ACTIVE | KPROBE_HIT_SSDONE => if fixup_exception(regs) != 0 { return 1; }, _ => {} } 0
}

pub unsafe extern "C" fn kprobe_breakpoint_handler(regs: *mut pt_regs) -> c_int {
    let kcb = get_kprobe_ctlblk(); let cur = kprobe_running(); let p = get_kprobe(instruction_pointer(regs) as *mut kprobe_opcode_t);
    if !p.is_null() { if !cur.is_null() { if reenter_kprobe(p, regs, kcb) != 0 { return 1; } } else { set_current_kprobe(p); (*kcb).kprobe_status = KPROBE_HIT_ACTIVE; if (*p).pre_handler.is_none() || (*p).pre_handler.unwrap()(p, regs) == 0 { setup_singlestep(p, regs, kcb, 0); } else { reset_current_kprobe(); } } return 1; } 0
}

pub unsafe extern "C" fn kprobe_single_step_handler(regs: *mut pt_regs) -> c_int { let kcb = get_kprobe_ctlblk(); if (*kcb).ss_ctx.ss_pending && (*kcb).ss_ctx.match_addr == instruction_pointer(regs) { clear_ss_context(kcb); kprobes_restore_local_irqflag(kcb, regs); (*regs).sr = ((*regs).sr & TRACE_MODE_MASK) | TRACE_MODE_RUN; post_kprobe_handler(kcb, regs); return 1; } 0 }
pub unsafe extern "C" fn arch_populate_kprobe_blacklist() -> c_int { kprobe_add_area_blacklist(&__irqentry_text_start as *const _ as c_ulong, &__irqentry_text_end as *const _ as c_ulong) }
pub unsafe extern "C" fn trampoline_probe_handler(regs: *mut pt_regs) -> *mut c_void { kretprobe_trampoline_handler(regs, core::ptr::null_mut()) }
pub unsafe extern "C" fn arch_prepare_kretprobe(ri: *mut kretprobe_instance, regs: *mut pt_regs) { (*ri).ret_addr = (*regs).lr as *mut kprobe_opcode_t; (*ri).fp = core::ptr::null_mut(); (*regs).lr = &__kretprobe_trampoline as *const _ as c_ulong; }
pub unsafe extern "C" fn arch_trampoline_kprobe(_p: *mut kprobe) -> c_int { 0 }
pub unsafe extern "C" fn arch_init_kprobes() -> c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
