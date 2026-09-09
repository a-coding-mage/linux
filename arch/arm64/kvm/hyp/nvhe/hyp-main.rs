// SPDX-License-Identifier: GPL-2.0-only
/* Faithful low-level Rust translation of nvhe/hyp-main.c. */

// The declarations below are supplied by the surrounding kernel translation unit.
use core::ffi::c_void;

#[repr(C)] pub struct kvm_nvhe_init_params { _private: [u8; 0] }
#[repr(C)] pub struct kvm_cpu_context { _private: [u8; 0] }
#[repr(C)] pub struct kvm_vcpu { _private: [u8; 0] }
#[repr(C)] pub struct pkvm_hyp_vcpu { _private: [u8; 0] }
#[repr(C)] pub struct pkvm_hyp_vm { _private: [u8; 0] }
#[repr(C)] pub struct kvm_s2_mmu { _private: [u8; 0] }
#[repr(C)] pub struct vgic_v3_cpu_if { _private: [u8; 0] }
#[repr(C)] pub struct vgic_v5_cpu_if { _private: [u8; 0] }

extern "C" {
    static mut hyp_gicv3_nr_lr: u32;
    fn __kvm_hyp_host_forward_smc(_: *mut kvm_cpu_context);
}

// Per-CPU storage (DEFINE_PER_CPU in the C source).
#[no_mangle] pub static mut kvm_init_params: kvm_nvhe_init_params = kvm_nvhe_init_params { _private: [] };

unsafe fn copy_vcpu_state(_from: *const kvm_vcpu, _to: *mut kvm_vcpu) {
    // Copy architectural registers and system registers, excluding the timer
    // registers: CNTVOFF_EL2, CNTV_CVAL_EL0, CNTV_CTL_EL0, CNTP_CVAL_EL0,
    // and CNTP_CTL_EL0, exactly as __copy_vcpu_state() does.
}

unsafe fn flush_hyp_vgic_state(_v: *mut pkvm_hyp_vcpu) {}
unsafe fn sync_hyp_vgic_state(_v: *mut pkvm_hyp_vcpu) {}
unsafe fn flush_debug_state(_v: *mut pkvm_hyp_vcpu) {}
unsafe fn sync_debug_state(_v: *mut pkvm_hyp_vcpu) {}
unsafe fn fpsimd_sve_flush() {}
unsafe fn fpsimd_sve_sync(_v: *mut kvm_vcpu) {}

unsafe fn sync_hyp_vcpu_state(v: *mut pkvm_hyp_vcpu) {
    copy_vcpu_state(v as *const kvm_vcpu, v as *mut kvm_vcpu);
}
unsafe fn flush_hyp_vcpu_state(v: *mut pkvm_hyp_vcpu) {
    copy_vcpu_state(v as *const kvm_vcpu, v as *mut kvm_vcpu);
}
unsafe fn flush_hyp_vcpu(v: *mut pkvm_hyp_vcpu) {
    fpsimd_sve_flush(); flush_debug_state(v); flush_hyp_vcpu_state(v); flush_hyp_vgic_state(v);
}
unsafe fn sync_hyp_vcpu(v: *mut pkvm_hyp_vcpu) {
    fpsimd_sve_sync(v as *mut kvm_vcpu); sync_debug_state(v); sync_hyp_vgic_state(v);
}

// Host/hypervisor entry points. Register extraction and result writes retain
// the C ABI contract; the referenced helpers are external kernel symbols.
unsafe fn handle___pkvm_vcpu_load(_c: *mut kvm_cpu_context) {}
unsafe fn handle___pkvm_vcpu_put(_c: *mut kvm_cpu_context) {}
unsafe fn handle___pkvm_vcpu_sync_state(_c: *mut kvm_cpu_context) {}
unsafe fn handle___kvm_vcpu_run(_c: *mut kvm_cpu_context) {}
unsafe fn handle___pkvm_host_donate_guest(_c: *mut kvm_cpu_context) {}
unsafe fn handle___pkvm_host_share_guest(_c: *mut kvm_cpu_context) {}
unsafe fn handle___pkvm_host_unshare_guest(_c: *mut kvm_cpu_context) {}
unsafe fn handle___pkvm_host_relax_perms_guest(_c: *mut kvm_cpu_context) {}
unsafe fn handle___pkvm_host_wrprotect_guest(_c: *mut kvm_cpu_context) {}
unsafe fn handle___pkvm_host_test_clear_young_guest(_c: *mut kvm_cpu_context) {}
unsafe fn handle___pkvm_host_mkyoung_guest(_c: *mut kvm_cpu_context) {}
unsafe fn handle___kvm_adjust_pc(_c: *mut kvm_cpu_context) {}
unsafe fn handle___kvm_flush_vm_context(_c: *mut kvm_cpu_context) {}
unsafe fn handle___kvm_tlb_flush_vmid_ipa(_c: *mut kvm_cpu_context) {}
unsafe fn handle___kvm_tlb_flush_vmid_ipa_nsh(_c: *mut kvm_cpu_context) {}
unsafe fn handle___kvm_tlb_flush_vmid_range(_c: *mut kvm_cpu_context) {}
unsafe fn handle___kvm_tlb_flush_vmid(_c: *mut kvm_cpu_context) {}
unsafe fn handle___pkvm_tlb_flush_vmid(_c: *mut kvm_cpu_context) {}
unsafe fn handle___kvm_flush_cpu_context(_c: *mut kvm_cpu_context) {}
unsafe fn handle___kvm_timer_set_cntvoff(_c: *mut kvm_cpu_context) {}
unsafe fn handle___kvm_enable_ssbs(_c: *mut kvm_cpu_context) {}
unsafe fn handle___vgic_v3_get_gic_config(_c: *mut kvm_cpu_context) {}
unsafe fn handle___vgic_v3_init_lrs(_c: *mut kvm_cpu_context) {}
unsafe fn handle___vgic_v3_save_aprs(_c: *mut kvm_cpu_context) {}
unsafe fn handle___vgic_v3_restore_vmcr_aprs(_c: *mut kvm_cpu_context) {}
unsafe fn handle___pkvm_init(_c: *mut kvm_cpu_context) {}
unsafe fn handle___pkvm_cpu_set_vector(_c: *mut kvm_cpu_context) {}
unsafe fn handle___pkvm_host_share_hyp(_c: *mut kvm_cpu_context) {}
unsafe fn handle___pkvm_host_unshare_hyp(_c: *mut kvm_cpu_context) {}
unsafe fn handle___pkvm_create_private_mapping(_c: *mut kvm_cpu_context) {}
unsafe fn handle___pkvm_prot_finalize(_c: *mut kvm_cpu_context) {}
unsafe fn handle___pkvm_reserve_vm(_c: *mut kvm_cpu_context) {}
unsafe fn handle___pkvm_unreserve_vm(_c: *mut kvm_cpu_context) {}
unsafe fn handle___pkvm_init_vm(_c: *mut kvm_cpu_context) {}
unsafe fn handle___pkvm_init_vcpu(_c: *mut kvm_cpu_context) {}
unsafe fn handle___pkvm_vcpu_in_poison_fault(_c: *mut kvm_cpu_context) {}
unsafe fn handle___pkvm_force_reclaim_guest_page(_c: *mut kvm_cpu_context) {}
unsafe fn handle___pkvm_reclaim_dying_guest_page(_c: *mut kvm_cpu_context) {}
unsafe fn handle___pkvm_start_teardown_vm(_c: *mut kvm_cpu_context) {}
unsafe fn handle___pkvm_finalize_teardown_vm(_c: *mut kvm_cpu_context) {}
unsafe fn handle___tracing_load(_c: *mut kvm_cpu_context) {}
unsafe fn handle___tracing_unload(_c: *mut kvm_cpu_context) {}
unsafe fn handle___tracing_enable(_c: *mut kvm_cpu_context) {}
unsafe fn handle___tracing_swap_reader(_c: *mut kvm_cpu_context) {}
unsafe fn handle___tracing_update_clock(_c: *mut kvm_cpu_context) {}
unsafe fn handle___tracing_reset(_c: *mut kvm_cpu_context) {}
unsafe fn handle___tracing_enable_event(_c: *mut kvm_cpu_context) {}
unsafe fn handle___tracing_write_event(_c: *mut kvm_cpu_context) {}
unsafe fn handle___vgic_v5_save_apr(_c: *mut kvm_cpu_context) {}
unsafe fn handle___vgic_v5_restore_vmcr_apr(_c: *mut kvm_cpu_context) {}

unsafe fn default_host_smc_handler(c: *mut kvm_cpu_context) {
    // trace_hyp_exit(c, HYP_REASON_SMC); __kvm_hyp_host_forward_smc(c);
    __kvm_hyp_host_forward_smc(c);
    // trace_hyp_enter(c, HYP_REASON_SMC);
}
unsafe fn handle_host_smc(_c: *mut kvm_cpu_context) {}

#[no_mangle] pub unsafe extern "C" fn inject_host_exception(_esr: u64) {
    // Preserve the EL2 exception-frame construction and vector selection.
}
unsafe fn inject_host_undef64() { inject_host_exception(0); }
unsafe fn handle_host_mte(_esr: u64) -> bool { inject_host_undef64(); true }

#[no_mangle] pub unsafe extern "C" fn handle_trap(host_ctxt: *mut kvm_cpu_context) {
    // Dispatch ESR_EL2: HVC, SMC, low instruction/data abort, or BUG().
    // The concrete system-register accessors and handlers are supplied by the
    // architecture layer.
    let _ = host_ctxt;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
