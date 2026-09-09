// SPDX-License-Identifier: GPL-2.0-only
/* Based on arch/arm/kernel/traps.c */

// Linux headers and symbols referenced by this translation are supplied by
// the surrounding kernel translation unit.

#[inline] unsafe fn __check_eq(pstate: usize) -> bool { pstate & PSR_Z_BIT != 0 }
#[inline] unsafe fn __check_ne(pstate: usize) -> bool { pstate & PSR_Z_BIT == 0 }
#[inline] unsafe fn __check_cs(pstate: usize) -> bool { pstate & PSR_C_BIT != 0 }
#[inline] unsafe fn __check_cc(pstate: usize) -> bool { pstate & PSR_C_BIT == 0 }
#[inline] unsafe fn __check_mi(pstate: usize) -> bool { pstate & PSR_N_BIT != 0 }
#[inline] unsafe fn __check_pl(pstate: usize) -> bool { pstate & PSR_N_BIT == 0 }
#[inline] unsafe fn __check_vs(pstate: usize) -> bool { pstate & PSR_V_BIT != 0 }
#[inline] unsafe fn __check_vc(pstate: usize) -> bool { pstate & PSR_V_BIT == 0 }
#[inline] unsafe fn __check_hi(mut pstate: usize) -> bool { pstate &= !(pstate >> 1); pstate & PSR_C_BIT != 0 }
#[inline] unsafe fn __check_ls(mut pstate: usize) -> bool { pstate &= !(pstate >> 1); pstate & PSR_C_BIT == 0 }
#[inline] unsafe fn __check_ge(mut pstate: usize) -> bool { pstate ^= pstate << 3; pstate & PSR_N_BIT == 0 }
#[inline] unsafe fn __check_lt(mut pstate: usize) -> bool { pstate ^= pstate << 3; pstate & PSR_N_BIT != 0 }
#[inline] unsafe fn __check_gt(pstate: usize) -> bool { let mut t = pstate ^ (pstate << 3); t |= pstate << 1; t & PSR_N_BIT == 0 }
#[inline] unsafe fn __check_le(pstate: usize) -> bool { let mut t = pstate ^ (pstate << 3); t |= pstate << 1; t & PSR_N_BIT != 0 }
#[inline] unsafe fn __check_al(_: usize) -> bool { true }

/* The ARMv8 “nv” condition behaves identically to “al”. */
pub static aarch32_opcode_cond_checks: [unsafe fn(usize) -> bool; 16] = [
    __check_eq, __check_ne, __check_cs, __check_cc, __check_mi, __check_pl,
    __check_vs, __check_vc, __check_hi, __check_ls, __check_ge, __check_lt,
    __check_gt, __check_le, __check_al, __check_al,
];

pub static mut show_unhandled_signals: i32 = 0;

pub unsafe fn dump_kernel_instr(kaddr: usize) {
    if !is_ttbr1_addr(kaddr) { return; }
    let mut s = [0u8; 52]; let mut p = 0usize;
    for i in -4isize..1 {
        let mut val = 0u32;
        let bad = aarch64_insn_read((kaddr as *mut u32).offset(i), &mut val);
        let text = if bad == 0 { if i == 0 { format!("({val:08x}) ") } else { format!("{val:08x} ") } }
                   else if i == 0 { "(????????) ".into() } else { "???????? ".into() };
        for b in text.as_bytes() { if p < s.len() - 1 { s[p] = *b; p += 1; } }
    }
    printk(KERN_EMERG, "Code: %s\n", s.as_ptr());
}

unsafe fn __die(str_: *const i8, err: isize, regs: *mut pt_regs) -> i32 {
    static mut DIE_COUNTER: i32 = 0;
    let addr = instruction_pointer(regs);
    DIE_COUNTER += 1;
    pr_emerg("Internal error: %s: %016lx [#%d]  SMP\n", str_, err, DIE_COUNTER);
    let ret = notify_die(DIE_OOPS, str_, regs, err, 0, SIGSEGV);
    if ret == NOTIFY_STOP { return ret; }
    print_modules(); show_regs(regs);
    if user_mode(regs) { return ret; }
    dump_kernel_instr(addr); ret
}

pub unsafe fn die(str_: *const i8, regs: *mut pt_regs, err: isize) {
    let mut flags = 0usize; raw_spin_lock_irqsave(&die_lock, &mut flags); oops_enter();
    console_verbose(); bust_spinlocks(1); let ret = __die(str_, err, regs);
    if !regs.is_null() && kexec_should_crash(current) { crash_kexec(regs); }
    bust_spinlocks(0); add_taint(TAINT_DIE, LOCKDEP_NOW_UNRELIABLE); oops_exit();
    if in_interrupt() { panic!("%s: Fatal exception in interrupt", str_); }
    if panic_on_oops { panic!("%s: Fatal exception", str_); }
    raw_spin_unlock_irqrestore(&die_lock, flags);
    if ret != NOTIFY_STOP { make_task_dead(SIGSEGV); }
}

unsafe fn arm64_show_signal(signo: i32, str_: *const i8) {
    let tsk = current; let esr = (*tsk).thread.fault_code; let regs = task_pt_regs(tsk);
    if show_unhandled_signals == 0 || !unhandled_signal(tsk, signo) || !__ratelimit(&mut rs) { return; }
    pr_info("%s[%d]: unhandled exception: ", (*tsk).comm.as_ptr(), task_pid_nr(tsk));
    if esr != 0 { pr_cont!("%s, ESR 0x%016lx, ", esr_get_class_string(esr), esr); }
    pr_cont!("%s", str_); print_vma_addr(KERN_CONT, " in ", (*regs).pc); pr_cont!("\n"); __show_regs(regs);
}

pub unsafe fn arm64_force_sig_fault(signo: i32, code: i32, far: usize, str_: *const i8) {
    arm64_show_signal(signo, str_); if signo == SIGKILL { force_sig(SIGKILL); } else { force_sig_fault(signo, code, far as *mut core::ffi::c_void); }
}
pub unsafe fn arm64_force_sig_fault_pkey(far: usize, str_: *const i8, pkey: i32) { arm64_show_signal(SIGSEGV, str_); force_sig_pkuerr(far as *mut _, pkey); }
pub unsafe fn arm64_force_sig_mceerr(code: i32, far: usize, lsb: i16, str_: *const i8) { arm64_show_signal(SIGBUS, str_); force_sig_mceerr(code, far as *mut _, lsb); }
pub unsafe fn arm64_force_sig_ptrace_errno_trap(errno: i32, far: usize, str_: *const i8) { arm64_show_signal(SIGTRAP, str_); force_sig_ptrace_errno_trap(errno, far as *mut _); }

pub unsafe fn arm64_notify_die(str_: *const i8, regs: *mut pt_regs, signo: i32, sicode: i32, far: usize, err: usize) {
    if user_mode(regs) { WARN_ON(regs != current_pt_regs()); (*current).thread.fault_address = 0; (*current).thread.fault_code = err; arm64_force_sig_fault(signo, sicode, far, str_); } else { die(str_, regs, err as isize); }
}

#[cfg(feature = "CONFIG_COMPAT")]
unsafe fn compat_get_it_state(regs: *mut pt_regs) -> u32 { let p = (*regs).pstate; ((p & PSTATE_IT_1_0_MASK) >> PSTATE_IT_1_0_SHIFT) | (((p & PSTATE_IT_7_2_MASK) >> PSTATE_IT_7_2_SHIFT) << 2) }
#[cfg(feature = "CONFIG_COMPAT")]
unsafe fn compat_set_it_state(regs: *mut pt_regs, it: u32) { let x = ((it << PSTATE_IT_1_0_SHIFT) & PSTATE_IT_1_0_MASK) | (((it >> 2) << PSTATE_IT_7_2_SHIFT) & PSTATE_IT_7_2_MASK); (*regs).pstate = ((*regs).pstate & !PSR_AA32_IT_MASK) | x as usize; }
#[cfg(feature = "CONFIG_COMPAT")]
unsafe fn advance_itstate(regs: *mut pt_regs) { if (*regs).pstate & PSR_AA32_T_BIT == 0 || (*regs).pstate & PSR_AA32_IT_MASK == 0 { return; } let mut it = compat_get_it_state(regs); if it & 7 == 0 { it = 0; } else { it = (it & 0xe0) | ((it << 1) & 0x1f); } compat_set_it_state(regs, it); }
#[cfg(not(feature = "CONFIG_COMPAT"))]
unsafe fn advance_itstate(_: *mut pt_regs) {}

pub unsafe fn arm64_skip_faulting_instruction(regs: *mut pt_regs, size: usize) { (*regs).pc = (*regs).pc.wrapping_add(size); if user_mode(regs) { user_fastforward_single_step(current); } if compat_user_mode(regs) { advance_itstate(regs); } else { (*regs).pstate &= !PSR_BTYPE_MASK; } }

unsafe fn user_insn_read(regs: *mut pt_regs, insnp: *mut u32) -> i32 {
    let pc = instruction_pointer(regs); let mut instr: u32;
    if compat_thumb_mode(regs) { let mut x = 0u16; if get_user(&mut x, pc as *mut u16) != 0 { return -EFAULT; } instr = u16::from_le(x) as u32; if aarch32_insn_is_wide(instr) { if get_user(&mut x, (pc + 2) as *mut u16) != 0 { return -EFAULT; } instr = (instr << 16) | u16::from_le(x) as u32; } }
    else { let mut x = 0u32; if get_user(&mut x, pc as *mut u32) != 0 { return -EFAULT; } instr = u32::from_le(x); }
    *insnp = instr; 0
}

pub unsafe fn force_signal_inject(mut signal: i32, code: i32, address: usize, err: usize) { let regs = current_pt_regs(); if WARN_ON(!user_mode(regs)) { return; } let desc = match signal { SIGILL => c"undefined instruction".as_ptr(), SIGSEGV => c"illegal memory access".as_ptr(), _ => c"unknown or unrecoverable error".as_ptr() }; if WARN_ON(signal != SIGKILL && siginfo_layout(signal, code) != SIL_FAULT) { signal = SIGKILL; } arm64_notify_die(desc, regs, signal, code, address, err); }

pub unsafe fn arm64_notify_segfault(addr: usize) { let code; mmap_read_lock((*current).mm); code = if find_vma((*current).mm, untagged_addr(addr)).is_null() { SEGV_MAPERR } else { SEGV_ACCERR }; mmap_read_unlock((*current).mm); force_signal_inject(SIGSEGV, code, addr, 0); }

pub unsafe fn do_el0_undef(regs: *mut pt_regs, _: usize) { let mut insn = 0; if try_handle_aarch32_break(regs) != 0 { return; } if user_insn_read(regs, &mut insn) != 0 || (try_emulate_mrs(regs, insn) == 0 && try_emulate_armv8_deprecated(regs, insn) == 0) { force_signal_inject(SIGILL, ILL_ILLOPC, (*regs).pc, 0); } }
pub unsafe fn do_el1_undef(regs: *mut pt_regs, esr: usize) { let mut insn=0; if aarch64_insn_read((*regs).pc as *mut _, &mut insn) != 0 || try_emulate_el1_ssbs(regs, insn) == 0 { die(c"Oops - Undefined instruction".as_ptr(), regs, esr as isize); } }
pub unsafe fn do_el0_bti(regs: *mut pt_regs) { force_signal_inject(SIGILL, ILL_ILLOPC, (*regs).pc, 0); }
pub unsafe fn do_el1_bti(regs: *mut pt_regs, esr: usize) { if efi_runtime_fixup_exception(regs, c"BTI violation".as_ptr()) != 0 { (*regs).pstate &= !PSR_BTYPE_MASK; } else { die(c"Oops - BTI".as_ptr(), regs, esr as isize); } }
pub unsafe fn do_el0_gcs(regs: *mut pt_regs, _: usize) { force_signal_inject(SIGSEGV, SEGV_CPERR, (*regs).pc, 0); }
pub unsafe fn do_el1_gcs(regs: *mut pt_regs, esr: usize) { die(c"Oops - GCS".as_ptr(), regs, esr as isize); }
pub unsafe fn do_el0_fpac(regs: *mut pt_regs, esr: usize) { force_signal_inject(SIGILL, ILL_ILLOPN, (*regs).pc, esr); }
pub unsafe fn do_el1_fpac(regs: *mut pt_regs, esr: usize) { die(c"Oops - FPAC".as_ptr(), regs, esr as isize); }
pub unsafe fn do_el0_mops(regs: *mut pt_regs, esr: usize) { arm64_mops_reset_regs(&mut (*regs).user_regs, esr); user_fastforward_single_step(current); }
pub unsafe fn do_el1_mops(regs: *mut pt_regs, esr: usize) { arm64_mops_reset_regs(&mut (*regs).user_regs, esr); kernel_fastforward_single_step(regs); }

#[repr(C)] pub struct sys64_hook { pub esr_mask: usize, pub esr_val: usize, pub handler: Option<unsafe fn(usize, *mut pt_regs)> }
unsafe fn wfi_handler(_: usize, regs: *mut pt_regs) { arm64_skip_faulting_instruction(regs, AARCH64_INSN_SIZE); }
unsafe fn cntvct_read_handler(esr: usize, regs: *mut pt_regs) { if test_thread_flag(TIF_TSC_SIGSEGV) { force_sig(SIGSEGV); } else { pt_regs_write_reg(regs, ESR_ELx_SYS64_ISS_RT(esr), arch_timer_read_counter()); arm64_skip_faulting_instruction(regs, AARCH64_INSN_SIZE); } }
unsafe fn cntfrq_read_handler(esr: usize, regs: *mut pt_regs) { if test_thread_flag(TIF_TSC_SIGSEGV) { force_sig(SIGSEGV); } else { pt_regs_write_reg(regs, ESR_ELx_SYS64_ISS_RT(esr), arch_timer_get_rate()); arm64_skip_faulting_instruction(regs, AARCH64_INSN_SIZE); } }
unsafe fn mrs_handler(esr: usize, regs: *mut pt_regs) { if do_emulate_mrs(regs, esr_sys64_to_sysreg(esr), ESR_ELx_SYS64_ISS_RT(esr)) != 0 { force_signal_inject(SIGILL, ILL_ILLOPC, (*regs).pc, 0); } }
static sys64_hooks: [sys64_hook; 3] = [sys64_hook { esr_mask: ESR_ELx_SYS64_ISS_SYS_OP_MASK, esr_val: ESR_ELx_SYS64_ISS_SYS_CNTVCT, handler: Some(cntvct_read_handler) }, sys64_hook { esr_mask: ESR_ELx_SYS64_ISS_SYS_MRS_OP_MASK, esr_val: ESR_ELx_SYS64_ISS_SYS_MRS_OP_VAL, handler: Some(mrs_handler) }, sys64_hook { esr_mask: 0, esr_val: 0, handler: None }];
pub unsafe fn do_el0_sys(esr: usize, regs: *mut pt_regs) { for h in sys64_hooks.iter() { if h.handler.is_some() && h.esr_mask & esr == h.esr_val { (h.handler.unwrap())(esr, regs); return; } } do_el0_undef(regs, esr); }

static esr_class_str: [&str; 64] = ["UNRECOGNIZED EC"; 64];
pub unsafe fn esr_get_class_string(esr: usize) -> *const i8 { esr_class_str[ESR_ELx_EC(esr) as usize].as_ptr() as *const i8 }
pub unsafe fn bad_el0_sync(regs: *mut pt_regs, _: i32, esr: usize) { let pc=instruction_pointer(regs); (*current).thread.fault_address=0; (*current).thread.fault_code=esr; arm64_force_sig_fault(SIGILL, ILL_ILLOPC, pc, c"Bad EL0 synchronous exception".as_ptr()); }
pub unsafe fn do_serror(regs: *mut pt_regs, esr: usize) { if !arm64_is_ras_serror(esr) || arm64_is_fatal_ras_serror(regs, esr) { arm64_serror_panic(regs, esr); } }
#[cfg(feature = "CONFIG_GENERIC_BUG")] pub unsafe fn is_valid_bugaddr(_: usize) -> i32 { 1 }
pub unsafe fn bug_brk_handler(regs: *mut pt_regs, esr: usize) -> i32 { match report_bug((*regs).pc, regs) { BUG_TRAP_TYPE_BUG => die(c"Oops - BUG".as_ptr(), regs, esr as isize), BUG_TRAP_TYPE_WARN => (), _ => return DBG_HOOK_ERROR }; arm64_skip_faulting_instruction(regs, AARCH64_INSN_SIZE); DBG_HOOK_HANDLED }
pub unsafe fn reserved_fault_brk_handler(regs: *mut pt_regs, _: usize) -> i32 { pr_err("%s generated an invalid instruction at %pS!\n", c"Kernel text patching".as_ptr(), instruction_pointer(regs)); DBG_HOOK_ERROR }

pub unsafe fn panic_bad_stack(regs: *mut pt_regs, esr: usize, far: usize) -> ! {
    let task = (*current).stack as usize; let irq = this_cpu_read(irq_stack_ptr) as usize; let ovf = this_cpu_ptr(overflow_stack) as usize;
    console_verbose(); pr_emerg("Insufficient stack space to handle exception!");
    pr_emerg("ESR: 0x%016lx -- %s\n", esr, esr_get_class_string(esr)); pr_emerg("FAR: 0x%016lx\n", far);
    pr_emerg("Task stack:     [0x%016lx..0x%016lx]\n", task, task + THREAD_SIZE);
    pr_emerg("IRQ stack:      [0x%016lx..0x%016lx]\n", irq, irq + IRQ_STACK_SIZE);
    pr_emerg("Overflow stack: [0x%016lx..0x%016lx]\n", ovf, ovf + OVERFLOW_STACK_SIZE);
    __show_regs(regs); nmi_panic(core::ptr::null_mut(), c"kernel stack overflow".as_ptr()); cpu_park_loop()
}
pub unsafe fn arm64_serror_panic(regs: *mut pt_regs, esr: usize) -> ! { add_taint(TAINT_MACHINE_CHECK, LOCKDEP_STILL_OK); console_verbose(); pr_crit!("SError Interrupt on CPU%d, code 0x%016lx -- %s\n", smp_processor_id(), esr, esr_get_class_string(esr)); if !regs.is_null() { __show_regs(regs); } nmi_panic(regs, c"Asynchronous SError Interrupt".as_ptr()); cpu_park_loop() }
pub unsafe fn arm64_is_fatal_ras_serror(regs: *mut pt_regs, esr: usize) -> bool { match arm64_ras_serror_get_severity(esr) { ESR_ELx_AET_CE | ESR_ELx_AET_UEO => false, ESR_ELx_AET_UEU | ESR_ELx_AET_UER => true, _ => { arm64_serror_panic(regs, esr); } } }

#[cfg(feature = "CONFIG_CFI")]
pub unsafe fn cfi_brk_handler(regs: *mut pt_regs, esr: usize) -> i32 {
    let mut target = pt_regs_read_reg(regs, FIELD_GET(CFI_BRK_IMM_TARGET, esr)); let ty = pt_regs_read_reg(regs, FIELD_GET(CFI_BRK_IMM_TYPE, esr)) as u32;
    match report_cfi_failure(regs, (*regs).pc, &mut target, ty) { BUG_TRAP_TYPE_BUG => die(c"Oops - CFI".as_ptr(), regs, esr as isize), BUG_TRAP_TYPE_WARN => (), _ => return DBG_HOOK_ERROR }
    arm64_skip_faulting_instruction(regs, AARCH64_INSN_SIZE); DBG_HOOK_HANDLED
}

#[cfg(feature = "CONFIG_KASAN_SW_TAGS")]
pub unsafe fn kasan_brk_handler(regs: *mut pt_regs, esr: usize) -> i32 {
    let recover = esr & 0x20 != 0; let write = esr & 0x10 != 0; let size = 1usize << (esr & 0xf); let addr = (*regs).regs[0] as *mut _;
    kasan_report(addr, size, write, (*regs).pc); if !recover { die(c"Oops - KASAN".as_ptr(), regs, esr as isize); }
    arm64_skip_faulting_instruction(regs, AARCH64_INSN_SIZE); DBG_HOOK_HANDLED
}
#[cfg(feature = "CONFIG_UBSAN_TRAP")]
pub unsafe fn ubsan_brk_handler(regs: *mut pt_regs, esr: usize) -> i32 { die(report_ubsan_failure(esr & UBSAN_BRK_MASK), regs, esr as isize); DBG_HOOK_HANDLED }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
