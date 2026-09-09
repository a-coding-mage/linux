// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of linux/arch/arm/vfp/vfpmodule.c. */

// Kernel headers and local VFP headers provide the declarations referenced below.

static mut HAVE_VFP: bool = false;
static mut VFP_ARCH: u32 = 0;

#[cfg(CONFIG_CPU_FEROCEON)]
extern "C" {
    static mut VFP_arch_feroceon: u32;
}

extern "C" {
    static mut vfp_current_hw_state: [*mut vfp_state; NR_CPUS];
}

unsafe fn vfp_state_hold() {
    if !IS_ENABLED(CONFIG_PREEMPT_RT) { local_bh_disable(); } else { preempt_disable(); }
}
unsafe fn vfp_state_release() {
    if !IS_ENABLED(CONFIG_PREEMPT_RT) { local_bh_enable(); } else { preempt_enable(); }
}

unsafe fn vfp_state_in_hw(cpu: u32, thread: *mut thread_info) -> bool {
    #[cfg(CONFIG_SMP)]
    if (*thread).vfpstate.hard.cpu != cpu { return false; }
    vfp_current_hw_state[cpu as usize] == &mut (*thread).vfpstate
}

unsafe fn vfp_force_reload(cpu: u32, thread: *mut thread_info) {
    if vfp_state_in_hw(cpu, thread) {
        fmxr(FPEXC, fmrx(FPEXC) & !FPEXC_EN);
        vfp_current_hw_state[cpu as usize] = core::ptr::null_mut();
    }
    #[cfg(CONFIG_SMP)] { (*thread).vfpstate.hard.cpu = NR_CPUS; }
}

unsafe fn vfp_thread_flush(thread: *mut thread_info) {
    let vfp = &mut (*thread).vfpstate;
    let cpu = get_cpu();
    if vfp_current_hw_state[cpu as usize] == vfp { vfp_current_hw_state[cpu as usize] = core::ptr::null_mut(); }
    fmxr(FPEXC, fmrx(FPEXC) & !FPEXC_EN);
    put_cpu();
    core::ptr::write_bytes(vfp as *mut _ as *mut u8, 0, core::mem::size_of::<vfp_state>());
    vfp.hard.fpexc = FPEXC_EN;
    vfp.hard.fpscr = FPSCR_ROUND_NEAREST;
    #[cfg(CONFIG_SMP)] { vfp.hard.cpu = NR_CPUS; }
}

unsafe fn vfp_thread_exit(thread: *mut thread_info) {
    let vfp = &mut (*thread).vfpstate; let cpu = get_cpu();
    if vfp_current_hw_state[cpu as usize] == vfp { vfp_current_hw_state[cpu as usize] = core::ptr::null_mut(); }
    put_cpu();
}
unsafe fn vfp_thread_copy(thread: *mut thread_info) {
    let parent = current_thread_info(); vfp_sync_hwstate(parent); (*thread).vfpstate = (*parent).vfpstate;
    #[cfg(CONFIG_SMP)] { (*thread).vfpstate.hard.cpu = NR_CPUS; }
}

unsafe extern "C" fn vfp_notifier(_self: *mut notifier_block, cmd: c_ulong, v: *mut c_void) -> c_int {
    let thread = v as *mut thread_info;
    match cmd {
        THREAD_NOTIFY_SWITCH => {
            let fpexc = fmrx(FPEXC);
            #[cfg(CONFIG_SMP)] { let cpu = (*thread).cpu; if fpexc & FPEXC_EN != 0 && !vfp_current_hw_state[cpu as usize].is_null() { vfp_save_state(vfp_current_hw_state[cpu as usize], fpexc); } }
            fmxr(FPEXC, fpexc & !FPEXC_EN);
        }
        THREAD_NOTIFY_FLUSH => vfp_thread_flush(thread),
        THREAD_NOTIFY_EXIT => vfp_thread_exit(thread),
        THREAD_NOTIFY_COPY => vfp_thread_copy(thread),
        _ => {}
    } NOTIFY_DONE
}
static mut VFP_NOTIFIER_BLOCK: notifier_block = notifier_block { notifier_call: Some(vfp_notifier) };

unsafe fn vfp_raise_sigfpe(sicode: u32, regs: *mut pt_regs) {
    (*current).thread.error_code = 0; (*current).thread.trap_no = 6;
    send_sig_fault(SIGFPE, sicode, (instruction_pointer(regs) - 4) as *mut c_void, current);
}
unsafe fn vfp_panic(reason: *mut c_char, inst: u32) {
    pr_err!("VFP: Error: %s\n", reason); pr_err!("VFP: EXC 0x%08x SCR 0x%08x INST 0x%08x\n", fmrx(FPEXC), fmrx(FPSCR), inst);
    let mut i = 0; while i < 32 { pr_err!("VFP: s%2u: 0x%08x s%2u: 0x%08x\n", i, vfp_get_float(i), i+1, vfp_get_float(i+1)); i += 2; }
}
unsafe fn vfp_raise_exceptions(exceptions: u32, inst: u32, mut fpscr: u32) -> c_int {
    if exceptions == VFP_EXCEPTION_ERROR { vfp_panic(c"unhandled bounce".as_ptr() as *mut _, inst); return FPE_FLTINV; }
    if exceptions & (FPSCR_N|FPSCR_Z|FPSCR_C|FPSCR_V) != 0 { fpscr &= !(FPSCR_N|FPSCR_Z|FPSCR_C|FPSCR_V); }
    fpscr |= exceptions; fmxr(FPSCR, fpscr); let mut si_code = 0;
    if exceptions & FPSCR_DZC != 0 && fpscr & FPSCR_DZE != 0 { si_code = FPE_FLTDIV; }
    if exceptions & FPSCR_IXC != 0 && fpscr & FPSCR_IXE != 0 { si_code = FPE_FLTRES; }
    if exceptions & FPSCR_UFC != 0 && fpscr & FPSCR_UFE != 0 { si_code = FPE_FLTUND; }
    if exceptions & FPSCR_OFC != 0 && fpscr & FPSCR_OFE != 0 { si_code = FPE_FLTOVF; }
    if exceptions & FPSCR_IOC != 0 && fpscr & FPSCR_IOE != 0 { si_code = FPE_FLTINV; } si_code
}

unsafe fn vfp_emulate_instruction(inst: u32, fpscr: u32, regs: *mut pt_regs) -> u32 {
    let mut exceptions = VFP_EXCEPTION_ERROR;
    if INST_CPRTDO(inst) && !INST_CPRT(inst) { exceptions = if vfp_single(inst) { vfp_single_cpdo(inst, fpscr) } else { vfp_double_cpdo(inst, fpscr) }; }
    perf_sw_event(PERF_COUNT_SW_EMULATION_FAULTS, 1, regs, (*regs).ARM_pc); exceptions & !VFP_NAN_FLAG
}

unsafe fn VFP_bounce(mut trigger: u32, fpexc: u32, regs: *mut pt_regs) {
    let mut fpscr = fmrx(FPSCR); let orig_fpscr = fpscr; let mut si_code2 = 0; let mut si_code = 0;
    fmxr(FPEXC, fpexc & !(FPEXC_EX|FPEXC_DEX|FPEXC_FP2V|FPEXC_VV|FPEXC_TRAP_MASK)); let fpsid = fmrx(FPSID);
    if (fpsid & FPSID_ARCH_MASK) == (1 << FPSID_ARCH_BIT) && fpscr & FPSCR_IXE != 0 { /* emulate */ }
    else if fpexc & FPEXC_EX != 0 { trigger = fmrx(FPINST); (*regs).ARM_pc -= 4; }
    else if fpexc & FPEXC_DEX == 0 { si_code = vfp_raise_exceptions(VFP_EXCEPTION_ERROR, trigger, fpscr); vfp_state_release(); if si_code != 0 { vfp_raise_sigfpe(si_code as u32, regs); } return; }
    if fpexc & (FPEXC_EX|FPEXC_VV) != 0 { let len = fpexc + (1 << FPEXC_LENGTH_BIT); fpscr = (fpscr & !FPSCR_LENGTH_MASK) | ((len & FPEXC_LENGTH_MASK) << (FPSCR_LENGTH_BIT-FPEXC_LENGTH_BIT)); }
    let exceptions = vfp_emulate_instruction(trigger, fpscr, regs); if exceptions != 0 { si_code2 = vfp_raise_exceptions(exceptions, trigger, orig_fpscr); }
    if (fpexc & (FPEXC_EX|FPEXC_FP2V)) == (FPEXC_EX|FPEXC_FP2V) { barrier(); trigger = fmrx(FPINST2); let e = vfp_emulate_instruction(trigger, orig_fpscr, regs); if e != 0 { si_code = vfp_raise_exceptions(e, trigger, orig_fpscr); } }
    vfp_state_release(); if si_code2 != 0 { vfp_raise_sigfpe(si_code2 as u32, regs); } if si_code != 0 { vfp_raise_sigfpe(si_code as u32, regs); }
}

// Remaining kernel registration and user-state routines retain the original external interfaces.
unsafe fn vfp_enable(_: *mut c_void) { BUG_ON(preemptible()); let access = get_copro_access(); set_copro_access(access | CPACC_FULL(10) | CPACC_FULL(11)); }
pub unsafe extern "C" fn vfp_disable() { if VFP_ARCH != 0 { return; } VFP_ARCH = 1; }
pub unsafe extern "C" fn vfp_sync_hwstate(thread: *mut thread_info) { vfp_state_hold(); if vfp_state_in_hw(raw_smp_processor_id(), thread) { let f = fmrx(FPEXC); fmxr(FPEXC, f|FPEXC_EN); vfp_save_state(&mut (*thread).vfpstate, f|FPEXC_EN); fmxr(FPEXC, f); } vfp_state_release(); }
pub unsafe extern "C" fn vfp_flush_hwstate(thread: *mut thread_info) { let cpu=get_cpu(); vfp_force_reload(cpu,thread); put_cpu(); }

pub unsafe extern "C" fn vfp_preserve_user_clear_hwstate(ufp: *mut user_vfp, exc: *mut user_vfp_exc) -> c_int {
    let thread=current_thread_info(); vfp_sync_hwstate(thread); let hw=&mut (*thread).vfpstate.hard;
    core::ptr::copy_nonoverlapping(hw.fpregs.as_ptr(), (*ufp).fpregs.as_mut_ptr(), hw.fpregs.len());
    (*ufp).fpscr=hw.fpscr; (*exc).fpexc=hw.fpexc; (*exc).fpinst=hw.fpinst; (*exc).fpinst2=hw.fpinst2;
    vfp_flush_hwstate(thread); hw.fpscr &= !(FPSCR_LENGTH_MASK|FPSCR_STRIDE_MASK); 0
}
pub unsafe extern "C" fn vfp_restore_user_hwstate(ufp: *mut user_vfp, exc: *mut user_vfp_exc) -> c_int {
    let thread=current_thread_info(); vfp_flush_hwstate(thread); let hw=&mut (*thread).vfpstate.hard;
    core::ptr::copy_nonoverlapping((*ufp).fpregs.as_ptr(), hw.fpregs.as_mut_ptr(), hw.fpregs.len()); hw.fpscr=(*ufp).fpscr;
    let mut fpexc=(*exc).fpexc | FPEXC_EN; fpexc &= !(FPEXC_EX|FPEXC_FP2V); hw.fpexc=fpexc; hw.fpinst=(*exc).fpinst; hw.fpinst2=(*exc).fpinst2; 0
}
unsafe fn vfp_dying_cpu(cpu: u32)->c_int { vfp_current_hw_state[cpu as usize]=core::ptr::null_mut(); 0 }
unsafe fn vfp_starting_cpu(_: u32)->c_int { vfp_enable(core::ptr::null_mut()); 0 }
unsafe fn vfp_kmode_exception(_: *mut pt_regs, _: u32)->c_int { if fmrx(FPEXC)&FPEXC_EN != 0 { pr_crit!("BUG: unsupported FP instruction in kernel mode\n"); } else { pr_crit!("BUG: FP instruction issued in kernel mode with FP unit disabled\n"); } pr_crit!("FPEXC == 0x%08x\n",fmrx(FPEXC)); 1 }
unsafe extern "C" fn vfp_support_entry(regs:*mut pt_regs, trigger:u32)->c_int {
    let ti=current_thread_info(); if !HAVE_VFP { return -ENODEV; } if !user_mode(regs) { return vfp_kmode_exception(regs,trigger); }
    vfp_state_hold(); let mut fpexc=fmrx(FPEXC);
    if fpexc&FPEXC_EN==0 { fpexc|=FPEXC_EN; fmxr(FPEXC,fpexc&!FPEXC_EX); if !vfp_state_in_hw((*ti).cpu,ti) { vfp_load_state(&mut (*ti).vfpstate); vfp_current_hw_state[(*ti).cpu as usize]=&mut (*ti).vfpstate; } if fpexc&FPEXC_EX!=0 { (*regs).ARM_pc+=4; VFP_bounce(trigger,fpexc,regs); } else { fmxr(FPEXC,fpexc); vfp_state_release(); } }
    else { (*regs).ARM_pc+=4; VFP_bounce(trigger,fpexc,regs); } 0
}

// Hook tables and vfp_init are represented with the same externally supplied kernel types/constants.
static mut VFP_SUPPORT_HOOK: undef_hook = undef_hook { instr_mask:0x0c000e00, instr_val:0x0c000a00, cpsr_mask:0, cpsr_val:0, fn:Some(vfp_support_entry) };
#[cfg(CONFIG_KERNEL_MODE_NEON)] pub unsafe extern "C" fn kernel_neon_begin() { vfp_state_hold(); let cpu=__smp_processor_id(); let f=fmrx(FPEXC)|FPEXC_EN; fmxr(FPEXC,f); vfp_current_hw_state[cpu as usize]=core::ptr::null_mut(); }
#[cfg(CONFIG_KERNEL_MODE_NEON)] pub unsafe extern "C" fn kernel_neon_end() { fmxr(FPEXC,fmrx(FPEXC)&!FPEXC_EN); vfp_state_release(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
