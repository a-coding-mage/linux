// SPDX-License-Identifier: GPL-2.0-or-later

// Kernel headers supplied by the surrounding translation unit.

#[cfg(all(feature = "CONFIG_PPC_ADV_DEBUG_REGS", feature = "CONFIG_PPC32"))]
pub static mut global_dbcr0: [c_ulong; NR_CPUS] = [0; NR_CPUS];

#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
pub static mut interrupt_exit_not_reentrant: StaticKeyFalse = StaticKeyFalse::new();

#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
#[inline(always)]
unsafe fn exit_must_hard_disable() -> bool {
    static_branch_unlikely(&interrupt_exit_not_reentrant)
}

#[cfg(not(feature = "CONFIG_PPC_BOOK3S_64"))]
#[inline(always)]
unsafe fn exit_must_hard_disable() -> bool { true }

/* Local IRQs must be disabled. Returns false when the caller must retry. */
#[inline(always)]
unsafe fn prep_irq_for_enabled_exit(restartable: bool) -> bool {
    let must_hard_disable = exit_must_hard_disable() || !restartable;
    trace_hardirqs_on();

    if must_hard_disable { __hard_EE_RI_disable(); }

    #[cfg(feature = "CONFIG_PPC64")]
    {
        if unlikely(lazy_irq_pending_nocheck()) {
            if must_hard_disable {
                (*local_paca).irq_happened |= PACA_IRQ_HARD_DIS;
                __hard_RI_enable();
            }
            trace_hardirqs_off();
            return false;
        }
    }
    true
}

#[inline(never)]
pub unsafe fn syscall_exit_prepare(mut r3: c_ulong, regs: *mut pt_regs, scv: c_long) -> c_ulong {
    let mut ti_flags: c_ulong;
    let mut ret: c_ulong = 0;
    let is_not_scv = !cfg!(feature = "CONFIG_PPC_BOOK3S_64") || scv == 0;

    kuap_assert_locked();
    (*regs).result = r3;
    (*current).thread_info.exit_flags = 0;
    ti_flags = read_thread_flags();
    if unlikely(r3 >= (-(MAX_ERRNO as c_long)) as c_ulong) && is_not_scv {
        if likely((ti_flags & (_TIF_NOERROR | _TIF_RESTOREALL)) == 0) {
            r3 = (-(r3 as c_long)) as c_ulong;
            (*regs).ccr |= 0x10000000;
        }
    }
    if unlikely((ti_flags & _TIF_PERSYSCALL_MASK) != 0) {
        if (ti_flags & _TIF_RESTOREALL) != 0 { ret = _TIF_RESTOREALL; }
        else { (*regs).gpr[3] = r3; }
        clear_bits(_TIF_PERSYSCALL_MASK, &mut (*current_thread_info()).flags);
    } else { (*regs).gpr[3] = r3; }
    if unlikely((ti_flags & _TIF_SYSCALL_DOTRACE) != 0) { ret |= _TIF_RESTOREALL; }
    syscall_exit_to_user_mode(regs);

    loop {
        user_enter_irqoff();
        if prep_irq_for_enabled_exit(true) { break; }
        user_exit_irqoff(); local_irq_enable(); local_irq_disable();
    }
    kuap_user_restore(regs);
    ret |= (*current).thread_info.exit_flags;
    #[cfg(feature = "CONFIG_PPC64")]
    { (*regs).exit_result = ret; }
    ret
}

#[cfg(feature = "CONFIG_PPC64")]
#[inline(never)]
pub unsafe fn syscall_exit_restart(r3: c_ulong, regs: *mut pt_regs) -> c_ulong {
    __hard_irq_disable(); (*local_paca).irq_happened |= PACA_IRQ_HARD_DIS;
    #[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
    set_kuap(AMR_KUAP_BLOCKED);
    loop {
        user_enter_irqoff();
        if prep_irq_for_enabled_exit(true) { break; }
        user_exit_irqoff(); local_irq_enable(); local_irq_disable();
    }
    kuap_user_restore(regs);
    let ret = (*current_thread_info()).exit_flags & _TIF_RESTOREALL;
    (*current_thread_info()).exit_flags &= !_TIF_RESTOREALL;
    (*regs).exit_result |= ret;
    ret
}

#[inline(never)]
pub unsafe fn interrupt_exit_user_prepare(regs: *mut pt_regs) -> c_ulong {
    BUG_ON(regs_is_unrecoverable(regs)); BUG_ON(regs_irqs_disabled(regs));
    kuap_assert_locked();
    (*current_thread_info()).exit_flags = 0;
    local_irq_disable();
    loop {
        check_return_regs_valid(regs); user_enter_irqoff();
        if prep_irq_for_enabled_exit(true) { break; }
        user_exit_irqoff(); local_irq_enable(); local_irq_disable();
    }
    kuap_user_restore(regs);
    let ret = (*current_thread_info()).exit_flags & _TIF_RESTOREALL;
    #[cfg(feature = "CONFIG_PPC64")]
    { (*regs).exit_result = ret; }
    ret
}

pub unsafe extern "C" fn preempt_schedule_irq();

#[inline(never)]
pub unsafe fn interrupt_exit_kernel_prepare(regs: *mut pt_regs) -> c_ulong {
    let mut ret = 0;
    let kuap;
    let stack_store = (read_thread_flags() & _TIF_EMULATE_STACK_STORE) != 0;
    if regs_is_unrecoverable(regs) { unrecoverable_exception(regs); }
    if !cfg!(feature = "CONFIG_PPC_BOOK3E_64") && TRAP(regs) != INTERRUPT_PROGRAM && TRAP(regs) != INTERRUPT_PERFMON {
        CT_WARN_ON(ct_state() == CT_STATE_USER);
    }
    kuap = kuap_get_and_assert_locked();
    local_irq_disable();
    if !regs_irqs_disabled(regs) {
        WARN_ON_ONCE(((*regs).msr & MSR_EE) == 0);
        loop {
            check_return_regs_valid(regs);
            if !prep_irq_for_enabled_exit(!stack_store) {
                hard_irq_disable(); replay_soft_interrupts(); continue;
            }
            #[cfg(feature = "CONFIG_PPC64")]
            { (*local_paca).irq_happened &= !PACA_IRQ_HARD_DIS; }
            break;
        }
    } else {
        check_return_regs_valid(regs);
        if stack_store { __hard_EE_RI_disable(); }
    }
    if stack_store { clear_bits(_TIF_EMULATE_STACK_STORE, &mut (*current_thread_info()).flags); ret = 1; }
    #[cfg(feature = "CONFIG_PPC_TRANSACTIONAL_MEM")]
    { (*local_paca).tm_scratch = (*regs).msr; }
    kuap_kernel_restore(regs, kuap);
    ret
}

#[cfg(feature = "CONFIG_PPC64")]
#[inline(never)]
pub unsafe fn interrupt_exit_user_restart(regs: *mut pt_regs) -> c_ulong {
    __hard_irq_disable(); (*local_paca).irq_happened |= PACA_IRQ_HARD_DIS;
    #[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
    set_kuap(AMR_KUAP_BLOCKED);
    trace_hardirqs_off(); account_cpu_user_entry(); BUG_ON(!user_mode(regs));
    (*regs).exit_result |= interrupt_exit_user_prepare(regs);
    (*regs).exit_result
}

#[cfg(feature = "CONFIG_PPC64")]
#[inline(never)]
pub unsafe fn interrupt_exit_kernel_restart(regs: *mut pt_regs) -> c_ulong {
    __hard_irq_disable(); (*local_paca).irq_happened |= PACA_IRQ_HARD_DIS;
    #[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
    set_kuap(AMR_KUAP_BLOCKED);
    if (*regs).softe == IRQS_ENABLED { trace_hardirqs_off(); }
    BUG_ON(user_mode(regs)); interrupt_exit_kernel_prepare(regs)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
