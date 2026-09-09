// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

// Linux and architecture headers from the original translation unit provide
// the external types, constants, functions, and macros referenced below.

pub static mut show_unhandled_signals: ::core::ffi::c_int = 1;

// Defined in entry.S
extern "C" {
    fn csky_trap();
    fn csky_systemcall();
    fn csky_cmpxchg();
    fn csky_get_tls();
    fn csky_irq();
    fn csky_pagefault();
}

// Defined in head.S
extern "C" {
    fn _start_smp_secondary();
}

pub unsafe extern "C" fn pre_trap_init() {
    let mut i: ::core::ffi::c_int;

    mtcr("vbr", vec_base);

    i = 1;
    while i < 128 {
        VEC_INIT(i, csky_trap);
        i += 1;
    }
}

pub unsafe extern "C" fn trap_init() {
    VEC_INIT(VEC_AUTOVEC, csky_irq);

    /* setup trap0 trap2 trap3 */
    VEC_INIT(VEC_TRAP0, csky_systemcall);
    VEC_INIT(VEC_TRAP2, csky_cmpxchg);
    VEC_INIT(VEC_TRAP3, csky_get_tls);

    /* setup MMU TLB exception */
    VEC_INIT(VEC_TLBINVALIDL, csky_pagefault);
    VEC_INIT(VEC_TLBINVALIDS, csky_pagefault);
    VEC_INIT(VEC_TLBMODIFIED, csky_pagefault);

    // #ifdef CONFIG_CPU_HAS_FPU
    #[cfg(CONFIG_CPU_HAS_FPU)]
    init_fpu();
    // #endif

    // #ifdef CONFIG_SMP
    #[cfg(CONFIG_SMP)]
    {
        mtcr("cr<28, 0>", virt_to_phys(vec_base));
        VEC_INIT(VEC_RESET, virt_to_phys(_start_smp_secondary) as *mut ::core::ffi::c_void);
    }
    // #endif
}

static mut die_lock: DEFINE_SPINLOCK = DEFINE_SPINLOCK_INIT;

pub unsafe extern "C" fn die(regs: *mut pt_regs, str_: *const ::core::ffi::c_char) {
    static mut die_counter: ::core::ffi::c_int = 0;
    let ret: ::core::ffi::c_int;

    oops_enter();
    spin_lock_irq(&mut die_lock);
    console_verbose();
    bust_spinlocks(1);

    die_counter += 1;
    pr_emerg("%s [#%d]\n", str_, die_counter);
    print_modules();
    show_regs(regs);
    show_stack(current, (*regs).regs[4] as *mut ::core::ffi::c_ulong, KERN_INFO);

    ret = notify_die(DIE_OOPS, str_, regs, 0, trap_no(regs), SIGSEGV);

    bust_spinlocks(0);
    add_taint(TAINT_DIE, LOCKDEP_NOW_UNRELIABLE);
    spin_unlock_irq(&mut die_lock);
    oops_exit();

    if in_interrupt() != 0 {
        panic("Fatal exception in interrupt");
    }
    if panic_on_oops != 0 {
        panic("Fatal exception");
    }
    if ret != NOTIFY_STOP {
        make_task_dead(SIGSEGV);
    }
}

pub unsafe extern "C" fn do_trap(regs: *mut pt_regs, signo: ::core::ffi::c_int,
                                  code: ::core::ffi::c_int, addr: ::core::ffi::c_ulong) {
    let tsk = current;

    if show_unhandled_signals != 0 && unhandled_signal(tsk, signo) != 0
        && printk_ratelimit() != 0 {
        pr_info("%s[%d]: unhandled signal %d code 0x%x at 0x%08lx",
                (*tsk).comm.as_ptr(), task_pid_nr(tsk), signo, code, addr);
        print_vma_addr(" in ", instruction_pointer(regs));
        pr_cont("\n");
        show_regs(regs);
    }

    force_sig_fault(signo, code, addr as *mut ::core::ffi::c_void);
}

unsafe fn do_trap_error(regs: *mut pt_regs, signo: ::core::ffi::c_int,
                        code: ::core::ffi::c_int, addr: ::core::ffi::c_ulong,
                        str_: *const ::core::ffi::c_char) {
    (*current).thread.trap_no = trap_no(regs);

    if user_mode(regs) != 0 {
        do_trap(regs, signo, code, addr);
    } else if fixup_exception(regs) == 0 {
        die(regs, str_);
    }
}

pub unsafe extern "C" fn do_trap_unknown(regs: *mut pt_regs) {
    do_trap_error(regs, SIGILL, ILL_ILLTRP, (*regs).pc, c"Oops - unknown exception".as_ptr());
}
pub unsafe extern "C" fn do_trap_zdiv(regs: *mut pt_regs) {
    do_trap_error(regs, SIGFPE, FPE_INTDIV, (*regs).pc, c"Oops - error zero div exception".as_ptr());
}
pub unsafe extern "C" fn do_trap_buserr(regs: *mut pt_regs) {
    do_trap_error(regs, SIGSEGV, ILL_ILLADR, (*regs).pc, c"Oops - error bus error exception".as_ptr());
}

pub unsafe extern "C" fn do_trap_misaligned(regs: *mut pt_regs) {
    // #ifdef CONFIG_CPU_NEED_SOFTALIGN
    #[cfg(CONFIG_CPU_NEED_SOFTALIGN)]
    csky_alignment(regs);
    // #else
    #[cfg(not(CONFIG_CPU_NEED_SOFTALIGN))]
    {
        (*current).thread.trap_no = trap_no(regs);
        do_trap_error(regs, SIGBUS, BUS_ADRALN, (*regs).pc,
                      c"Oops - load/store address misaligned".as_ptr());
    }
}

pub unsafe extern "C" fn do_trap_bkpt(regs: *mut pt_regs) {
    // #ifdef CONFIG_KPROBES
    #[cfg(CONFIG_KPROBES)]
    if kprobe_single_step_handler(regs) != 0 { return; }
    // #endif
    // #ifdef CONFIG_UPROBES
    #[cfg(CONFIG_UPROBES)]
    if uprobe_single_step_handler(regs) != 0 { return; }
    // #endif
    if user_mode(regs) != 0 {
        send_sig(SIGTRAP, current, 0);
        return;
    }
    do_trap_error(regs, SIGILL, ILL_ILLTRP, (*regs).pc,
                  c"Oops - illegal trap exception".as_ptr());
}

pub unsafe extern "C" fn do_trap_illinsn(regs: *mut pt_regs) {
    (*current).thread.trap_no = trap_no(regs);
    // #ifdef CONFIG_KPROBES
    #[cfg(CONFIG_KPROBES)]
    if kprobe_breakpoint_handler(regs) != 0 { return; }
    // #endif
    // #ifdef CONFIG_UPROBES
    #[cfg(CONFIG_UPROBES)]
    if uprobe_breakpoint_handler(regs) != 0 { return; }
    // #endif
    // #ifndef CONFIG_CPU_NO_USER_BKPT
    #[cfg(not(CONFIG_CPU_NO_USER_BKPT))]
    if *(instruction_pointer(regs) as *const u16) != USR_BKPT {
        send_sig(SIGTRAP, current, 0);
        return;
    }
    // #endif
    do_trap_error(regs, SIGILL, ILL_ILLOPC, (*regs).pc,
                  c"Oops - illegal instruction exception".as_ptr());
}

pub unsafe extern "C" fn do_trap_fpe(regs: *mut pt_regs) {
    // #ifdef CONFIG_CPU_HAS_FPU
    #[cfg(CONFIG_CPU_HAS_FPU)]
    return fpu_fpe(regs);
    // #else
    #[cfg(not(CONFIG_CPU_HAS_FPU))]
    do_trap_error(regs, SIGILL, ILL_ILLOPC, (*regs).pc,
                  c"Oops - fpu instruction exception".as_ptr());
}

pub unsafe extern "C" fn do_trap_priv(regs: *mut pt_regs) {
    // #ifdef CONFIG_CPU_HAS_FPU
    #[cfg(CONFIG_CPU_HAS_FPU)]
    if user_mode(regs) != 0 && fpu_libc_helper(regs) != 0 { return; }
    // #endif
    do_trap_error(regs, SIGILL, ILL_PRVOPC, (*regs).pc,
                  c"Oops - illegal privileged exception".as_ptr());
}

pub unsafe extern "C" fn trap_c(regs: *mut pt_regs) {
    match trap_no(regs) {
        VEC_ZERODIV => do_trap_zdiv(regs),
        VEC_TRACE => do_trap_bkpt(regs),
        VEC_ILLEGAL => do_trap_illinsn(regs),
        VEC_TRAP1 | VEC_BREAKPOINT => do_trap_bkpt(regs),
        VEC_ACCESS => do_trap_buserr(regs),
        VEC_ALIGN => do_trap_misaligned(regs),
        VEC_FPE => do_trap_fpe(regs),
        VEC_PRIV => do_trap_priv(regs),
        _ => do_trap_unknown(regs),
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
