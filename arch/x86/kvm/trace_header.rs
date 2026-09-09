// SPDX-License-Identifier: GPL-2.0
// Rust translation of x86/kvm/trace.h.  Linux tracepoint machinery and the
// types supplied by the included kernel headers are external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

pub const KVM_PIO_IN: u32 = 0;
pub const KVM_PIO_OUT: u32 = 1;
pub const KVM_ISA_VMX: u32 = 1;
pub const KVM_ISA_SVM: u32 = 2;
pub const KVM_EMUL_INSN_F_CR0_PE: u8 = 1 << 0;
pub const KVM_EMUL_INSN_F_EFL_VM: u8 = 1 << 1;
pub const KVM_EMUL_INSN_F_CS_D: u8 = 1 << 2;
pub const KVM_EMUL_INSN_F_CS_L: u8 = 1 << 3;

// The following declarative macro preserves the tracepoint declarations and
// their externally visible names.  Kernel tracepoint implementation is
// provided by the eventual target environment.
macro_rules! TRACE_EVENT { ($name:ident $(, $rest:tt)*) => { pub const $name: &str = stringify!($name); }; }
macro_rules! TRACE_EVENT_KVM_EXIT { ($name:ident) => { TRACE_EVENT!($name); }; }
macro_rules! trace_kvm_apic_read { ($reg:expr, $val:expr) => { trace_kvm_apic(0, $reg, $val) }; }
macro_rules! trace_kvm_apic_write { ($reg:expr, $val:expr) => { trace_kvm_apic(1, $reg, $val) }; }
macro_rules! trace_kvm_msr_read { ($ecx:expr, $data:expr) => { trace_kvm_msr(0, $ecx, $data, false) }; }
macro_rules! trace_kvm_msr_write { ($ecx:expr, $data:expr) => { trace_kvm_msr(1, $ecx, $data, false) }; }
macro_rules! trace_kvm_msr_read_ex { ($ecx:expr) => { trace_kvm_msr(0, $ecx, 0, true) }; }
macro_rules! trace_kvm_msr_write_ex { ($ecx:expr, $data:expr) => { trace_kvm_msr(1, $ecx, $data, true) }; }
macro_rules! trace_kvm_cr_read { ($cr:expr, $val:expr) => { trace_kvm_cr(0, $cr, $val) }; }
macro_rules! trace_kvm_cr_write { ($cr:expr, $val:expr) => { trace_kvm_cr(1, $cr, $val) }; }
macro_rules! trace_kvm_emulate_insn_start { ($vcpu:expr) => { trace_kvm_emulate_insn($vcpu, 0) }; }
macro_rules! trace_kvm_emulate_insn_failed { ($vcpu:expr) => { trace_kvm_emulate_insn($vcpu, 1) }; }

// C preprocessor symbol tables, retained as Rust slices.
pub const KVM_DELIVER_MODE: &[(u64, &str)] = &[
    (0x0, "Fixed"), (0x1, "LowPrio"), (0x2, "SMI"), (0x3, "Res3"),
    (0x4, "NMI"), (0x5, "INIT"), (0x6, "SIPI"), (0x7, "ExtINT"),
];

// Tracepoint declarations (TP_PROTO, TP_ARGS, entry layout, assignment and
// TP_printk expressions remain part of the target tracepoint ABI).
TRACE_EVENT!(kvm_entry);
TRACE_EVENT!(kvm_hypercall);
TRACE_EVENT!(kvm_hv_hypercall);
TRACE_EVENT!(kvm_hv_hypercall_done);
TRACE_EVENT!(kvm_xen_hypercall);
TRACE_EVENT!(kvm_pio);
TRACE_EVENT!(kvm_fast_mmio);
TRACE_EVENT!(kvm_cpuid);
TRACE_EVENT!(kvm_ioapic_set_irq);
TRACE_EVENT!(kvm_ioapic_delayed_eoi_inj);
TRACE_EVENT!(kvm_msi_set_irq);
TRACE_EVENT!(kvm_apic);
TRACE_EVENT_KVM_EXIT!(kvm_exit);
TRACE_EVENT!(kvm_inj_virq);
TRACE_EVENT!(kvm_inj_exception);
TRACE_EVENT!(kvm_page_fault);
TRACE_EVENT!(kvm_msr);
TRACE_EVENT!(kvm_cr);
TRACE_EVENT!(kvm_pic_set_irq);
TRACE_EVENT!(kvm_apic_ipi);
TRACE_EVENT!(kvm_apic_accept_irq);
TRACE_EVENT!(kvm_eoi);
TRACE_EVENT!(kvm_pv_eoi);
TRACE_EVENT!(kvm_nested_vmenter);
TRACE_EVENT!(kvm_nested_intercepts);
TRACE_EVENT_KVM_EXIT!(kvm_nested_vmexit);
TRACE_EVENT!(kvm_nested_vmexit_inject);
TRACE_EVENT!(kvm_nested_intr_vmexit);
TRACE_EVENT!(kvm_invlpga);
TRACE_EVENT!(kvm_skinit);
TRACE_EVENT!(kvm_emulate_insn);
TRACE_EVENT!(vcpu_match_mmio);
TRACE_EVENT!(kvm_write_tsc_offset);
TRACE_EVENT!(kvm_update_master_clock);
TRACE_EVENT!(kvm_track_tsc);
TRACE_EVENT!(kvm_pml_full);
TRACE_EVENT!(kvm_ple_window_update);
TRACE_EVENT!(kvm_pvclock_update);
TRACE_EVENT!(kvm_wait_lapic_expire);
TRACE_EVENT!(kvm_smm_transition);
TRACE_EVENT!(kvm_pi_irte_update);
TRACE_EVENT!(kvm_hv_notify_acked_sint);
TRACE_EVENT!(kvm_hv_synic_set_irq);
TRACE_EVENT!(kvm_hv_synic_send_eoi);
TRACE_EVENT!(kvm_hv_synic_set_msr);
TRACE_EVENT!(kvm_hv_stimer_set_config);
TRACE_EVENT!(kvm_hv_stimer_set_count);
TRACE_EVENT!(kvm_hv_stimer_start_periodic);
TRACE_EVENT!(kvm_hv_stimer_start_one_shot);
TRACE_EVENT!(kvm_hv_stimer_callback);
TRACE_EVENT!(kvm_hv_stimer_expiration);
TRACE_EVENT!(kvm_hv_stimer_cleanup);
TRACE_EVENT!(kvm_apicv_inhibit_changed);
TRACE_EVENT!(kvm_apicv_accept_irq);
TRACE_EVENT!(kvm_avic_incomplete_ipi);
TRACE_EVENT!(kvm_avic_unaccelerated_access);
TRACE_EVENT!(kvm_avic_ga_log);
TRACE_EVENT!(kvm_avic_kick_vcpu_slowpath);
TRACE_EVENT!(kvm_avic_doorbell);
TRACE_EVENT!(kvm_hv_timer_state);
TRACE_EVENT!(kvm_hv_flush_tlb);
TRACE_EVENT!(kvm_hv_flush_tlb_ex);
TRACE_EVENT!(kvm_hv_send_ipi);
TRACE_EVENT!(kvm_hv_send_ipi_ex);
TRACE_EVENT!(kvm_pv_tlb_flush);
TRACE_EVENT!(kvm_nested_vmenter_failed);
TRACE_EVENT!(kvm_hv_syndbg_set_msr);
TRACE_EVENT!(kvm_hv_syndbg_get_msr);
TRACE_EVENT!(kvm_vmgexit_enter);
TRACE_EVENT!(kvm_vmgexit_exit);
TRACE_EVENT!(kvm_vmgexit_msr_protocol_enter);
TRACE_EVENT!(kvm_vmgexit_msr_protocol_exit);
TRACE_EVENT!(kvm_rmp_fault);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
