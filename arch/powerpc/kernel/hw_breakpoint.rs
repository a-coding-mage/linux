// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * HW_breakpoint: a unified kernel/user-space hardware breakpoint facility,
 * using the CPU's debug registers. Derived from arch/x86/kernel/hw_breakpoint.c
 */

// Kernel and architecture dependencies supplied by the surrounding tree.

static mut BP_PER_REG: [*mut perf_event; HBP_NUM_MAX] = [core::ptr::null_mut(); HBP_NUM_MAX];

pub fn hw_breakpoint_slots(typ: i32) -> i32 {
    if typ == TYPE_DATA { nr_wp_slots() } else { 0 }
}

pub unsafe fn arch_install_hw_breakpoint(bp: *mut perf_event) -> i32 {
    let info = counter_arch_bp(bp);
    let mut i = 0;
    while i < nr_wp_slots() {
        let slot = &mut BP_PER_REG[i as usize];
        if (*slot).is_null() { *slot = bp; break; }
        i += 1;
    }
    if i == nr_wp_slots() {
        WARN_ONCE(true, "Can't find any breakpoint slot");
        return -EBUSY;
    }
    if !(*info).perf_single_step { __set_breakpoint(i, info); }
    0
}

pub unsafe fn arch_uninstall_hw_breakpoint(bp: *mut perf_event) {
    let null_brk: arch_hw_breakpoint = core::mem::zeroed();
    let mut i = 0;
    while i < nr_wp_slots() {
        let slot = &mut BP_PER_REG[i as usize];
        if *slot == bp { *slot = core::ptr::null_mut(); break; }
        i += 1;
    }
    if i == nr_wp_slots() { WARN_ONCE(true, "Can't find any breakpoint slot"); return; }
    __set_breakpoint(i, &null_brk);
}

unsafe fn is_ptrace_bp(bp: *mut perf_event) -> bool { (*bp).overflow_handler == ptrace_triggered }

pub unsafe fn arch_check_bp_in_kernelspace(hw: *mut arch_hw_breakpoint) -> i32 { is_kernel_addr((*hw).address) }

pub unsafe fn arch_bp_generic_fields(typ: i32, gen_bp_type: *mut i32) -> i32 {
    *gen_bp_type = 0;
    if typ & HW_BRK_TYPE_READ != 0 { *gen_bp_type |= HW_BREAKPOINT_R; }
    if typ & HW_BRK_TYPE_WRITE != 0 { *gen_bp_type |= HW_BREAKPOINT_W; }
    if *gen_bp_type == 0 { return -EINVAL; }
    0
}

unsafe fn hw_breakpoint_validate_len(hw: *mut arch_hw_breakpoint) -> i32 {
    let mut max_len: u16 = DABR_MAX_LEN;
    let start_addr = ALIGN_DOWN((*hw).address, HW_BREAKPOINT_SIZE);
    let end_addr = ALIGN((*hw).address + (*hw).len, HW_BREAKPOINT_SIZE);
    let hw_len = end_addr - start_addr;
    if dawr_enabled() {
        max_len = DAWR_MAX_LEN;
        if !cpu_has_feature(CPU_FTR_ARCH_31) && ALIGN_DOWN(start_addr, SZ_512) != ALIGN_DOWN(end_addr - 1, SZ_512) { return -EINVAL; }
    } else if IS_ENABLED(CONFIG_PPC_8xx) { max_len = U16_MAX; }
    if hw_len > max_len as _ { return -EINVAL; }
    (*hw).hw_len = hw_len;
    0
}

pub unsafe fn hw_breakpoint_arch_parse(bp: *mut perf_event, attr: *const perf_event_attr, hw: *mut arch_hw_breakpoint) -> i32 {
    let ret = -EINVAL;
    if bp.is_null() || (*attr).bp_len == 0 { return ret; }
    (*hw).typ = HW_BRK_TYPE_TRANSLATE;
    if (*attr).bp_type & HW_BREAKPOINT_R != 0 { (*hw).typ |= HW_BRK_TYPE_READ; }
    if (*attr).bp_type & HW_BREAKPOINT_W != 0 { (*hw).typ |= HW_BRK_TYPE_WRITE; }
    if (*hw).typ == HW_BRK_TYPE_TRANSLATE { return ret; }
    if !(*attr).exclude_user { (*hw).typ |= HW_BRK_TYPE_USER; }
    if !(*attr).exclude_kernel { (*hw).typ |= HW_BRK_TYPE_KERNEL; }
    if !(*attr).exclude_hv { (*hw).typ |= HW_BRK_TYPE_HYP; }
    (*hw).address = (*attr).bp_addr; (*hw).len = (*attr).bp_len;
    if !ppc_breakpoint_available() { return -ENODEV; }
    hw_breakpoint_validate_len(hw)
}

pub unsafe fn thread_change_pc(_tsk: *mut task_struct, regs: *mut pt_regs) {
    preempt_disable();
    let mut reset = false;
    for i in 0..nr_wp_slots() { let bp = BP_PER_REG[i as usize]; if !bp.is_null() && (*counter_arch_bp(bp)).perf_single_step { reset = true; break; } }
    if reset { regs_set_return_msr(regs, (*regs).msr & !MSR_SE); for i in 0..nr_wp_slots() { let bp = BP_PER_REG[i as usize]; if !bp.is_null() { let info = counter_arch_bp(bp); __set_breakpoint(i, info); info.perf_single_step = false; } } }
    preempt_enable();
}

unsafe fn is_larx_stcx_instr(typ: i32) -> bool { typ == LARX || typ == STCX }
unsafe fn is_octword_vsx_instr(typ: i32, size: i32) -> bool { (typ == LOAD_VSX || typ == STORE_VSX) && size == 32 }

unsafe fn handler_error(bp: *mut perf_event) { WARN(true, "Unable to handle hardware breakpoint. Breakpoint will be disabled.", counter_arch_bp(bp).address); perf_event_disable_inatomic(bp); }
unsafe fn larx_stcx_err(bp: *mut perf_event) { printk_ratelimited("Breakpoint hit on instruction that can't be emulated. Breakpoint will be disabled.\n", counter_arch_bp(bp).address); perf_event_disable_inatomic(bp); }

unsafe fn stepping_handler(regs: *mut pt_regs, bp: *mut *mut perf_event, hit: *mut i32, instr: ppc_inst_t) -> bool {
    if user_mode(regs) { for i in 0..nr_wp_slots() { if *hit.add(i as usize) != 0 { (*counter_arch_bp(*bp.add(i as usize))).perf_single_step = true; *bp.add(i as usize) = core::ptr::null_mut(); } } regs_set_return_msr(regs, (*regs).msr | MSR_SE); return false; }
    if emulate_step(regs, instr) == 0 { for i in 0..nr_wp_slots() { if *hit.add(i as usize) != 0 { handler_error(*bp.add(i as usize)); *bp.add(i as usize) = core::ptr::null_mut(); } } return false; }
    true
}

unsafe fn handle_p10dd1_spurious_exception(bp: *mut *mut perf_event, hit: *mut i32, ea: u64) {
    let mut i = 0; while i < nr_wp_slots() { if !(*bp.add(i as usize)).is_null() { let info = counter_arch_bp(*bp.add(i as usize)); let end = ALIGN(info.address + info.len, HW_BREAKPOINT_SIZE); if end - 1 < ea && ((end - 1) >> 10) == (ea >> 10) && (ea & 0x800) != ((ea + 64) & 0x800) { break; } } i += 1; }
    if i == nr_wp_slots() { return; }
    for j in 0..nr_wp_slots() { if !(*bp.add(j as usize)).is_null() { *hit.add(j as usize) = 1; (*counter_arch_bp(*bp.add(j as usize))).typ |= HW_BRK_TYPE_EXTRANEOUS_IRQ; } }
}

pub unsafe fn hw_breakpoint_handler(args: *mut die_args) -> i32 {
    let mut bp = [core::ptr::null_mut(); HBP_NUM_MAX]; let mut hit = [0i32; HBP_NUM_MAX];
    let regs = (*args).regs; let mut rc = NOTIFY_STOP; let mut nr_hit = 0; let mut ptrace_bp = false;
    let mut instr = ppc_inst(0); let mut typ = 0; let mut size = 0; let mut ea = 0; let mut err = false;
    hw_breakpoint_disable(); rcu_read_lock();
    if !IS_ENABLED(CONFIG_PPC_8xx) { wp_get_instr_detail(regs, &mut instr, &mut typ, &mut size, &mut ea); }
    for i in 0..nr_wp_slots() { bp[i as usize] = BP_PER_REG[i as usize]; if bp[i as usize].is_null() { continue; } let info = counter_arch_bp(bp[i as usize]); info.typ &= !HW_BRK_TYPE_EXTRANEOUS_IRQ; if wp_check_constraints(regs, instr, ea, typ, size, info) { if !IS_ENABLED(CONFIG_PPC_8xx) && ppc_inst_equal(instr, ppc_inst(0)) { handler_error(bp[i as usize]); bp[i as usize] = core::ptr::null_mut(); err = true; continue; } if is_ptrace_bp(bp[i as usize]) { ptrace_bp = true; } hit[i as usize] = 1; nr_hit += 1; } }
    if err { for i in 0..nr_wp_slots() { if !bp[i as usize].is_null() { __set_breakpoint(i, counter_arch_bp(bp[i as usize])); } } rcu_read_unlock(); return rc; }
    if nr_hit == 0 { if !IS_ENABLED(CONFIG_PPC_8xx) && mfspr(SPRN_PVR) == 0x800100 && is_octword_vsx_instr(typ, size) { handle_p10dd1_spurious_exception(bp.as_mut_ptr(), hit.as_mut_ptr(), ea); } else { rc = NOTIFY_DONE; rcu_read_unlock(); return rc; } }
    if ptrace_bp { for i in 0..nr_wp_slots() { if hit[i as usize] != 0 && is_ptrace_bp(bp[i as usize]) { perf_bp_event(bp[i as usize], regs); bp[i as usize] = core::ptr::null_mut(); } } rc = NOTIFY_DONE; } else if !IS_ENABLED(CONFIG_PPC_8xx) { if is_larx_stcx_instr(typ) { for i in 0..nr_wp_slots() { if hit[i as usize] != 0 { larx_stcx_err(bp[i as usize]); bp[i as usize] = core::ptr::null_mut(); } } } else if stepping_handler(regs, bp.as_mut_ptr(), hit.as_mut_ptr(), instr) { for i in 0..nr_wp_slots() { if hit[i as usize] != 0 && ((*counter_arch_bp(bp[i as usize])).typ & HW_BRK_TYPE_EXTRANEOUS_IRQ) == 0 { perf_bp_event(bp[i as usize], regs); } } } } else { for i in 0..nr_wp_slots() { if hit[i as usize] != 0 { perf_bp_event(bp[i as usize], regs); } } }
    for i in 0..nr_wp_slots() { if !bp[i as usize].is_null() { __set_breakpoint(i, counter_arch_bp(bp[i as usize])); } } rcu_read_unlock(); rc
}

unsafe fn single_step_dabr_instruction(args: *mut die_args) -> i32 { let mut found = false; for i in 0..nr_wp_slots() { let bp = BP_PER_REG[i as usize]; if !bp.is_null() { let info = counter_arch_bp(bp); if info.perf_single_step { found = true; if info.typ & HW_BRK_TYPE_EXTRANEOUS_IRQ == 0 { perf_bp_event(bp, (*args).regs); } info.perf_single_step = false; __set_breakpoint(i, info); } } } if !found || test_thread_flag(TIF_SINGLESTEP) { NOTIFY_DONE } else { NOTIFY_STOP } }

pub unsafe fn hw_breakpoint_exceptions_notify(_unused: *mut notifier_block, val: u64, data: *mut core::ffi::c_void) -> i32 { match val { DIE_DABR_MATCH => hw_breakpoint_handler(data as *mut die_args), DIE_SSTEP => single_step_dabr_instruction(data as *mut die_args), _ => NOTIFY_DONE } }
pub unsafe fn flush_ptrace_hw_breakpoint(tsk: *mut task_struct) { for i in 0..nr_wp_slots() { unregister_hw_breakpoint((*tsk).thread.ptrace_bps[i as usize]); (*tsk).thread.ptrace_bps[i as usize] = core::ptr::null_mut(); } }
pub unsafe fn hw_breakpoint_pmu_read(_bp: *mut perf_event) { /* TODO */ }
pub unsafe fn ptrace_triggered(bp: *mut perf_event, _data: *mut perf_sample_data, _regs: *mut pt_regs) { let mut attr = (*bp).attr; attr.disabled = true; modify_user_hw_breakpoint(bp, &attr); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
