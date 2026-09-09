/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of asm/idtentry.h.  Kernel configuration conditions are
// retained as comments because their values are supplied by the build.

pub const IDT_ALIGN: usize = 8 * (1 + HAS_KERNEL_IBT as usize);

pub type IdtentryT = unsafe extern "C" fn(regs: *mut pt_regs);

macro_rules! declare_idtentry {
    ($vector:expr, $func:ident) => {
        unsafe extern "C" { pub fn asm_$func(); pub fn xen_asm_$func(); }
        pub unsafe extern "C" fn fred_$func(regs: *mut pt_regs);
        pub unsafe extern "C" fn $func(regs: *mut pt_regs);
    };
}

macro_rules! define_idtentry {
    ($func:ident, $body:block) => {
        #[inline(always)] unsafe fn __$func(regs: *mut pt_regs) $body
        pub unsafe extern "C" fn $func(regs: *mut pt_regs) {
            let state = irqentry_enter(regs);
            instrumentation_begin(); __$func(regs); instrumentation_end();
            irqentry_exit(regs, state);
        }
    };
}

macro_rules! declare_idtentry_errorcode {
    ($vector:expr, $func:ident) => {
        unsafe extern "C" { pub fn asm_$func(); pub fn xen_asm_$func(); }
        pub unsafe extern "C" fn $func(regs: *mut pt_regs, error_code: c_ulong);
    };
}

macro_rules! define_idtentry_errorcode {
    ($func:ident, $body:block) => {
        #[inline(always)] unsafe fn __$func(regs: *mut pt_regs, error_code: c_ulong) $body
        pub unsafe extern "C" fn $func(regs: *mut pt_regs, error_code: c_ulong) {
            let state = irqentry_enter(regs);
            instrumentation_begin(); __$func(regs, error_code); instrumentation_end();
            irqentry_exit(regs, state);
        }
    };
}

macro_rules! declare_idtentry_raw { ($vector:expr, $func:ident) => { declare_idtentry!($vector, $func); }; }
macro_rules! define_idtentry_raw { ($func:ident, $body:block) => { pub unsafe extern "C" fn $func(regs: *mut pt_regs) $body }; }
macro_rules! define_fredentry_raw { ($func:ident, $body:block) => { pub unsafe extern "C" fn fred_$func(regs: *mut pt_regs) $body }; }
macro_rules! declare_idtentry_raw_errorcode { ($vector:expr, $func:ident) => { declare_idtentry_errorcode!($vector, $func); }; }
macro_rules! define_idtentry_raw_errorcode { ($func:ident, $body:block) => { pub unsafe extern "C" fn $func(regs: *mut pt_regs, error_code: c_ulong) $body }; }
macro_rules! declare_idtentry_irq { ($vector:expr, $func:ident) => { declare_idtentry_errorcode!($vector, $func); }; }

macro_rules! define_idtentry_irq {
    ($func:ident, $body:block) => {
        unsafe fn __$func(regs: *mut pt_regs, vector: u32) $body
        pub unsafe extern "C" fn $func(regs: *mut pt_regs, error_code: c_ulong) {
            let state = irqentry_enter(regs); let vector = error_code as u8 as u32;
            kvm_set_cpu_l1tf_flush_l1d(); instrumentation_begin();
            run_irq_on_irqstack_cond(__$func, regs, vector); instrumentation_end();
            irqentry_exit(regs, state);
        }
    };
}

macro_rules! declare_idtentry_sysvec { ($vector:expr, $func:ident) => { declare_idtentry!($vector, $func); }; }
macro_rules! define_idtentry_sysvec {
    ($func:ident, $body:block) => {
        unsafe fn __$func(regs: *mut pt_regs) $body
        #[inline(always)] unsafe fn instr_$func(regs: *mut pt_regs) { run_sysvec_on_irqstack_cond(__$func, regs); }
        pub unsafe extern "C" fn $func(regs: *mut pt_regs) { let state=irqentry_enter(regs); kvm_set_cpu_l1tf_flush_l1d(); instrumentation_begin(); instr_$func(regs); instrumentation_end(); irqentry_exit(regs,state); }
        pub unsafe extern "C" fn fred_$func(regs: *mut pt_regs) { instr_$func(regs); }
    };
}
macro_rules! define_idtentry_sysvec_simple { ($func:ident, $body:block) => { define_idtentry_sysvec!($func, $body); }; }
macro_rules! declare_idtentry_xencb { ($vector:expr, $func:ident) => { declare_idtentry!($vector, $func); }; }

// CONFIG_X86_64 variants; the alternate 32-bit declarations are preserved below.
macro_rules! declare_idtentry_ist { ($vector:expr, $func:ident) => { declare_idtentry_raw!($vector,$func); unsafe extern "C" { pub fn noist_$func(regs:*mut pt_regs); } }; }
macro_rules! declare_idtentry_vc { ($vector:expr, $func:ident) => { declare_idtentry_raw_errorcode!($vector,$func); }; }
macro_rules! define_idtentry_ist { ($func:ident, $body:block) => { define_idtentry_raw!($func,$body); }; }
macro_rules! define_idtentry_noist { ($func:ident, $body:block) => { define_idtentry_raw!(noist_$func,$body); }; }
macro_rules! declare_idtentry_df { ($vector:expr, $func:ident) => { declare_idtentry_raw_errorcode!($vector,$func); }; }
macro_rules! define_idtentry_df { ($func:ident, $body:block) => { define_idtentry_raw_errorcode!($func,$body); }; }

macro_rules! declare_idtentry_nmi { ($vector:expr, $func:ident) => { declare_idtentry_raw!($vector,$func); }; }
macro_rules! define_idtentry_nmi { ($func:ident, $body:block) => { define_idtentry_raw!($func,$body); }; }
macro_rules! define_fredentry_nmi { ($func:ident, $body:block) => { define_fredentry_raw!($func,$body); }; }

unsafe extern "C" {
    pub fn idt_install_sysvec(n: c_uint, function: *const core::ffi::c_void);
    pub fn fred_install_sysvec(vector: c_uint, function: IdtentryT);
}

macro_rules! sysvec_install { ($vector:expr, $function:ident) => {{ if IS_ENABLED(CONFIG_X86_FRED) { fred_install_sysvec($vector, $function); } if !cpu_feature_enabled(X86_FEATURE_FRED) { idt_install_sysvec($vector, asm_$function as *const _); } }}; }

pub const X86_TRAP_OTHER: u32 = 0xffff;

// Declaration list (configuration guards from the C header remain build-time conditions).
declare_idtentry!(X86_TRAP_DE, exc_divide_error);
declare_idtentry!(X86_TRAP_OF, exc_overflow);
declare_idtentry!(X86_TRAP_BR, exc_bounds);
declare_idtentry!(X86_TRAP_NM, exc_device_not_available);
declare_idtentry!(X86_TRAP_OLD_MF, exc_coproc_segment_overrun);
declare_idtentry!(X86_TRAP_SPURIOUS, exc_spurious_interrupt_bug);
declare_idtentry!(X86_TRAP_MF, exc_coprocessor_error);
declare_idtentry!(X86_TRAP_XF, exc_simd_coprocessor_error);
declare_idtentry_errorcode!(X86_TRAP_TS, exc_invalid_tss);
declare_idtentry_errorcode!(X86_TRAP_NP, exc_segment_not_present);
declare_idtentry_errorcode!(X86_TRAP_SS, exc_stack_segment);
declare_idtentry_errorcode!(X86_TRAP_GP, exc_general_protection);
declare_idtentry_errorcode!(X86_TRAP_AC, exc_alignment_check);
declare_idtentry_raw!(X86_TRAP_UD, exc_invalid_op);
declare_idtentry_raw!(X86_TRAP_BP, exc_int3);
declare_idtentry_raw_errorcode!(X86_TRAP_PF, exc_page_fault);

// Additional declarations are enabled by CONFIG_IA32_EMULATION, CONFIG_X86_MCE,
// CONFIG_XEN_PV, CONFIG_X86_64, CONFIG_X86_CET, CONFIG_AMD_MEM_ENCRYPT,
// CONFIG_INTEL_TDX_GUEST, CONFIG_X86_LOCAL_APIC, CONFIG_SMP, CONFIG_KVM,
// CONFIG_HYPERV, CONFIG_ACRN_GUEST, CONFIG_XEN, CONFIG_KVM_GUEST and related
// vector-specific configuration symbols in the original header.

declare_idtentry_raw!(IA32_SYSCALL_VECTOR, int80_emulation);
declare_idtentry_ist!(X86_TRAP_MC, exc_machine_check);
declare_idtentry_raw!(X86_TRAP_MC, xenpv_exc_machine_check);
declare_idtentry!(X86_TRAP_NMI, exc_nmi_kvm_vmx);
declare_idtentry_nmi!(X86_TRAP_NMI, exc_nmi);
declare_idtentry_raw!(X86_TRAP_NMI, xenpv_exc_nmi);
declare_idtentry_ist!(X86_TRAP_DB, exc_debug);
declare_idtentry_raw!(X86_TRAP_DB, xenpv_exc_debug);
declare_idtentry_df!(X86_TRAP_DF, exc_double_fault);
declare_idtentry_raw_errorcode!(X86_TRAP_DF, xenpv_exc_double_fault);
declare_idtentry_errorcode!(X86_TRAP_CP, exc_control_protection);
declare_idtentry_vc!(X86_TRAP_VC, exc_vmm_communication);
declare_idtentry_xencb!(X86_TRAP_OTHER, exc_xen_hypervisor_callback);
declare_idtentry_raw!(X86_TRAP_OTHER, exc_xen_unknown_trap);
declare_idtentry!(X86_TRAP_VE, exc_virtualization_exception);
declare_idtentry_irq!(X86_TRAP_OTHER, common_interrupt);
declare_idtentry_irq!(X86_TRAP_OTHER, spurious_interrupt);
declare_idtentry_sysvec!(ERROR_APIC_VECTOR, sysvec_error_interrupt);
declare_idtentry_sysvec!(SPURIOUS_APIC_VECTOR, sysvec_spurious_apic_interrupt);
declare_idtentry_sysvec!(LOCAL_TIMER_VECTOR, sysvec_apic_timer_interrupt);
declare_idtentry_sysvec!(X86_PLATFORM_IPI_VECTOR, sysvec_x86_platform_ipi);
declare_idtentry!(RESCHEDULE_VECTOR, sysvec_reschedule_ipi);
declare_idtentry_sysvec!(REBOOT_VECTOR, sysvec_reboot);
declare_idtentry_sysvec!(CALL_FUNCTION_SINGLE_VECTOR, sysvec_call_function_single);
declare_idtentry_sysvec!(CALL_FUNCTION_VECTOR, sysvec_call_function);
declare_idtentry_sysvec!(THRESHOLD_APIC_VECTOR, sysvec_threshold);
declare_idtentry_sysvec!(DEFERRED_ERROR_VECTOR, sysvec_deferred_error);
declare_idtentry_sysvec!(THERMAL_APIC_VECTOR, sysvec_thermal);
declare_idtentry_sysvec!(IRQ_WORK_VECTOR, sysvec_irq_work);
declare_idtentry_sysvec!(POSTED_INTR_VECTOR, sysvec_kvm_posted_intr_ipi);
declare_idtentry_sysvec!(POSTED_INTR_WAKEUP_VECTOR, sysvec_kvm_posted_intr_wakeup_ipi);
declare_idtentry_sysvec!(POSTED_INTR_NESTED_VECTOR, sysvec_kvm_posted_intr_nested_ipi);
declare_idtentry_sysvec!(PERF_GUEST_MEDIATED_PMI_VECTOR, sysvec_perf_guest_mediated_pmi_handler);
declare_idtentry_sysvec!(POSTED_MSI_NOTIFICATION_VECTOR, sysvec_posted_msi_notification);
declare_idtentry_sysvec!(HYPERVISOR_CALLBACK_VECTOR, sysvec_hyperv_callback);
declare_idtentry_sysvec!(HYPERV_REENLIGHTENMENT_VECTOR, sysvec_hyperv_reenlightenment);
declare_idtentry_sysvec!(HYPERV_STIMER0_VECTOR, sysvec_hyperv_stimer0);
declare_idtentry_sysvec!(HYPERVISOR_CALLBACK_VECTOR, sysvec_acrn_hv_callback);
declare_idtentry_sysvec!(HYPERVISOR_CALLBACK_VECTOR, sysvec_xen_hvm_callback);
declare_idtentry_sysvec!(HYPERVISOR_CALLBACK_VECTOR, sysvec_kvm_asyncpf_interrupt);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
