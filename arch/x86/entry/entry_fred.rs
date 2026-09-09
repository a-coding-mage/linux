/* SPDX-License-Identifier: GPL-2.0 */
/*
 * The FRED specific kernel/user entry functions which are invoked from
 * assembly code and dispatch to the associated handlers.
 */

/* Dependencies supplied by the surrounding kernel translation unit. */

const FRED_SYSCALL: u32 = 1;
const FRED_SYSENTER: u32 = 2;

unsafe fn fred_bad_type(regs: *mut pt_regs, error_code: c_ulong) {
    let irq_state: irqentry_state_t = irqentry_nmi_enter(regs);

    instrumentation_begin();

    /* Panic on events from a high stack level */
    if (*regs).fred_cs.sl > 0 {
        pr_emerg!("PANIC: invalid or fatal FRED event; event type %u vector %u error 0x%lx aux 0x%lx at %04x:%016lx\n",
            (*regs).fred_ss.type_, (*regs).fred_ss.vector, error_code,
            fred_event_data(regs), (*regs).cs, (*regs).ip);
        die(c"invalid or fatal FRED event", regs, error_code);
        panic!("invalid or fatal FRED event");
    } else {
        let flags: c_ulong = oops_begin();
        let mut sig: c_int = SIGKILL;

        pr_alert!("BUG: invalid or fatal FRED event; event type %u vector %u error 0x%lx aux 0x%lx at %04x:%016lx\n",
            (*regs).fred_ss.type_, (*regs).fred_ss.vector, error_code,
            fred_event_data(regs), (*regs).cs, (*regs).ip);

        if __die(c"Invalid or fatal FRED event", regs, error_code) {
            sig = 0;
        }

        oops_end(flags, regs, sig);
    }

    instrumentation_end();
    irqentry_nmi_exit(regs, irq_state);
}

unsafe fn fred_intx(regs: *mut pt_regs) {
    match (*regs).fred_ss.vector {
        X86_TRAP_BP => return exc_int3(regs),
        X86_TRAP_OF => return exc_overflow(regs),
        #[cfg(CONFIG_IA32_EMULATION)]
        IA32_SYSCALL_VECTOR => {
            if ia32_enabled() {
                return fred_int80_emulation(regs);
            }
        }
        _ => return exc_general_protection(regs, 0),
    }
    exc_general_protection(regs, 0)
}

#[inline(always)]
unsafe fn fred_other(regs: *mut pt_regs) {
    /* The compiler can fold these conditions into a single test */
    if likely((*regs).fred_ss.vector == FRED_SYSCALL && (*regs).fred_ss.l) {
        (*regs).orig_ax = (*regs).ax;
        (*regs).ax = (-ENOSYS) as _;
        do_syscall_64(regs, (*regs).orig_ax);
        return;
    } else if ia32_enabled()
        && likely((*regs).fred_ss.vector == FRED_SYSENTER && !(*regs).fred_ss.l)
    {
        (*regs).orig_ax = (*regs).ax;
        (*regs).ax = (-ENOSYS) as _;
        do_fast_syscall_32(regs);
        return;
    } else {
        exc_invalid_op(regs);
        return;
    }
}

/* SYSVEC(_vector, _function) = [_vector - FIRST_SYSTEM_VECTOR] = fred_sysvec__function */
static mut sysvec_table: [idtentry_t; NR_SYSTEM_VECTORS as usize] = [
    fred_sysvec_error_interrupt,
    fred_sysvec_spurious_apic_interrupt,
    fred_sysvec_apic_timer_interrupt,
    fred_sysvec_x86_platform_ipi,
    fred_sysvec_reschedule_ipi,
    fred_sysvec_call_function_single,
    fred_sysvec_call_function,
    fred_sysvec_reboot,
    fred_sysvec_threshold,
    fred_sysvec_deferred_error,
    fred_sysvec_thermal,
    fred_sysvec_irq_work,
    fred_sysvec_perf_guest_mediated_pmi_handler,
    fred_sysvec_kvm_posted_intr_ipi,
    fred_sysvec_kvm_posted_intr_wakeup_ipi,
    fred_sysvec_kvm_posted_intr_nested_ipi,
    fred_sysvec_posted_msi_notification,
];

static mut fred_setup_done: bool = false;

unsafe fn fred_install_sysvec(sysvec: c_uint, handler: idtentry_t) {
    if WARN_ON_ONCE(sysvec < FIRST_SYSTEM_VECTOR) { return; }
    if WARN_ON_ONCE(fred_setup_done) { return; }
    if !WARN_ON_ONCE(sysvec_table[(sysvec - FIRST_SYSTEM_VECTOR) as usize]) {
        sysvec_table[(sysvec - FIRST_SYSTEM_VECTOR) as usize] = handler;
    }
}

unsafe fn fred_handle_spurious_interrupt(regs: *mut pt_regs) {
    spurious_interrupt(regs, (*regs).fred_ss.vector);
}

unsafe fn fred_complete_exception_setup() {
    let mut vector: c_uint = 0;
    while vector < FIRST_EXTERNAL_VECTOR {
        set_bit(vector, system_vectors);
        vector += 1;
    }
    vector = 0;
    while vector < NR_SYSTEM_VECTORS {
        if sysvec_table[vector as usize] {
            set_bit(vector + FIRST_SYSTEM_VECTOR, system_vectors);
        } else {
            sysvec_table[vector as usize] = fred_handle_spurious_interrupt;
        }
        vector += 1;
    }
    fred_setup_done = true;
}

unsafe fn fred_extint(regs: *mut pt_regs) {
    let vector = (*regs).fred_ss.vector;
    if WARN_ON_ONCE(vector < FIRST_EXTERNAL_VECTOR) { return; }
    if likely(vector >= FIRST_SYSTEM_VECTOR) {
        let state = irqentry_enter(regs);
        instrumentation_begin();
        sysvec_table[array_index_nospec(vector - FIRST_SYSTEM_VECTOR, NR_SYSTEM_VECTORS) as usize](regs);
        instrumentation_end();
        irqentry_exit(regs, state);
    } else {
        common_interrupt(regs, vector);
    }
}

unsafe fn fred_hwexc(regs: *mut pt_regs, error_code: c_ulong) {
    if likely((*regs).fred_ss.vector == X86_TRAP_PF) { return exc_page_fault(regs, error_code); }
    match (*regs).fred_ss.vector {
        X86_TRAP_DE => exc_divide_error(regs), X86_TRAP_DB => fred_exc_debug(regs),
        X86_TRAP_BR => exc_bounds(regs), X86_TRAP_UD => exc_invalid_op(regs),
        X86_TRAP_NM => exc_device_not_available(regs), X86_TRAP_DF => exc_double_fault(regs, error_code),
        X86_TRAP_TS => exc_invalid_tss(regs, error_code), X86_TRAP_NP => exc_segment_not_present(regs, error_code),
        X86_TRAP_SS => exc_stack_segment(regs, error_code), X86_TRAP_GP => exc_general_protection(regs, error_code),
        X86_TRAP_MF => exc_coprocessor_error(regs), X86_TRAP_AC => exc_alignment_check(regs, error_code),
        X86_TRAP_XF => exc_simd_coprocessor_error(regs), _ => fred_bad_type(regs, error_code),
    }
}

unsafe fn fred_swexc(regs: *mut pt_regs, error_code: c_ulong) {
    match (*regs).fred_ss.vector { X86_TRAP_BP => exc_int3(regs), X86_TRAP_OF => exc_overflow(regs), _ => fred_bad_type(regs, error_code) }
}

unsafe fn fred_entry_from_user(regs: *mut pt_regs) {
    let error_code = (*regs).orig_ax; (*regs).orig_ax = -1;
    match (*regs).fred_ss.type_ {
        EVENT_TYPE_EXTINT => fred_extint(regs), EVENT_TYPE_NMI if likely((*regs).fred_ss.vector == X86_TRAP_NMI) => fred_exc_nmi(regs),
        EVENT_TYPE_HWEXC => fred_hwexc(regs, error_code), EVENT_TYPE_SWINT => fred_intx(regs),
        EVENT_TYPE_PRIV_SWEXC if likely((*regs).fred_ss.vector == X86_TRAP_DB) => fred_exc_debug(regs),
        EVENT_TYPE_SWEXC => fred_swexc(regs, error_code), EVENT_TYPE_OTHER => fred_other(regs),
        _ => fred_bad_type(regs, error_code),
    }
}

unsafe fn fred_entry_from_kernel(regs: *mut pt_regs) {
    let error_code = (*regs).orig_ax; (*regs).orig_ax = -1;
    match (*regs).fred_ss.type_ {
        EVENT_TYPE_EXTINT => fred_extint(regs), EVENT_TYPE_NMI if likely((*regs).fred_ss.vector == X86_TRAP_NMI) => fred_exc_nmi(regs),
        EVENT_TYPE_HWEXC => fred_hwexc(regs, error_code),
        EVENT_TYPE_PRIV_SWEXC if likely((*regs).fred_ss.vector == X86_TRAP_DB) => fred_exc_debug(regs),
        EVENT_TYPE_SWEXC => fred_swexc(regs, error_code), _ => fred_bad_type(regs, error_code),
    }
}

#[cfg(IS_ENABLED(CONFIG_KVM_INTEL))]
unsafe fn __fred_entry_from_kvm(regs: *mut pt_regs) {
    match (*regs).fred_ss.type_ {
        EVENT_TYPE_EXTINT => fred_extint(regs), EVENT_TYPE_NMI => fred_exc_nmi(regs), _ => { WARN_ON_ONCE(1); }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
