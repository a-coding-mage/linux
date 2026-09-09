/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of kvm_asm.h. */

pub const ARM_EXIT_WITH_SERROR_BIT: u32 = 31;
pub const ARM_EXCEPTION_IRQ: u32 = 0;
pub const ARM_EXCEPTION_EL1_SERROR: u32 = 1;
pub const ARM_EXCEPTION_TRAP: u32 = 2;
pub const ARM_EXCEPTION_IL: u32 = 3;
pub const ARM_EXCEPTION_HYP_GONE: u32 = HVC_STUB_ERR;

#[inline]
pub const fn arm_exception_code(x: u32) -> u32 {
    x & !(1u32 << ARM_EXIT_WITH_SERROR_BIT)
}

#[inline]
pub const fn arm_exception_is_trap(x: u32) -> bool {
    arm_exception_code(x) == ARM_EXCEPTION_TRAP
}

#[inline]
pub const fn arm_serror_pending(x: u32) -> bool {
    (x & (1u32 << ARM_EXIT_WITH_SERROR_BIT)) != 0
}

pub const KVM_VECTOR_PREAMBLE: usize = 2 * AARCH64_INSN_SIZE;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum __kvm_host_smccc_func {
    __KVM_HOST_SMCCC_FUNC___kvm_hyp_init = 0,
    __KVM_HOST_SMCCC_FUNC___pkvm_init,
    __KVM_HOST_SMCCC_FUNC___pkvm_create_private_mapping,
    __KVM_HOST_SMCCC_FUNC___pkvm_cpu_set_vector,
    __KVM_HOST_SMCCC_FUNC___kvm_enable_ssbs,
    __KVM_HOST_SMCCC_FUNC___vgic_v3_init_lrs,
    __KVM_HOST_SMCCC_FUNC___vgic_v3_get_gic_config,
    __KVM_HOST_SMCCC_FUNC_MIN_PKVM,
    __KVM_HOST_SMCCC_FUNC___pkvm_prot_finalize,
    __KVM_HOST_SMCCC_FUNC___kvm_adjust_pc,
    __KVM_HOST_SMCCC_FUNC___kvm_vcpu_run,
    __KVM_HOST_SMCCC_FUNC___kvm_flush_vm_context,
    __KVM_HOST_SMCCC_FUNC___kvm_tlb_flush_vmid_ipa,
    __KVM_HOST_SMCCC_FUNC___kvm_tlb_flush_vmid_ipa_nsh,
    __KVM_HOST_SMCCC_FUNC___kvm_tlb_flush_vmid,
    __KVM_HOST_SMCCC_FUNC___kvm_tlb_flush_vmid_range,
    __KVM_HOST_SMCCC_FUNC___kvm_flush_cpu_context,
    __KVM_HOST_SMCCC_FUNC___kvm_timer_set_cntvoff,
    __KVM_HOST_SMCCC_FUNC___tracing_load,
    __KVM_HOST_SMCCC_FUNC___tracing_unload,
    __KVM_HOST_SMCCC_FUNC___tracing_enable,
    __KVM_HOST_SMCCC_FUNC___tracing_swap_reader,
    __KVM_HOST_SMCCC_FUNC___tracing_update_clock,
    __KVM_HOST_SMCCC_FUNC___tracing_reset,
    __KVM_HOST_SMCCC_FUNC___tracing_enable_event,
    __KVM_HOST_SMCCC_FUNC___tracing_write_event,
    __KVM_HOST_SMCCC_FUNC___vgic_v3_save_aprs,
    __KVM_HOST_SMCCC_FUNC___vgic_v3_restore_vmcr_aprs,
    __KVM_HOST_SMCCC_FUNC___vgic_v5_save_apr,
    __KVM_HOST_SMCCC_FUNC___vgic_v5_restore_vmcr_apr,
    __KVM_HOST_SMCCC_FUNC_PKVM_ONLY,
    __KVM_HOST_SMCCC_FUNC___pkvm_host_share_hyp,
    __KVM_HOST_SMCCC_FUNC___pkvm_host_unshare_hyp,
    __KVM_HOST_SMCCC_FUNC___pkvm_host_donate_guest,
    __KVM_HOST_SMCCC_FUNC___pkvm_host_share_guest,
    __KVM_HOST_SMCCC_FUNC___pkvm_host_unshare_guest,
    __KVM_HOST_SMCCC_FUNC___pkvm_host_relax_perms_guest,
    __KVM_HOST_SMCCC_FUNC___pkvm_host_wrprotect_guest,
    __KVM_HOST_SMCCC_FUNC___pkvm_host_test_clear_young_guest,
    __KVM_HOST_SMCCC_FUNC___pkvm_host_mkyoung_guest,
    __KVM_HOST_SMCCC_FUNC___pkvm_reserve_vm,
    __KVM_HOST_SMCCC_FUNC___pkvm_unreserve_vm,
    __KVM_HOST_SMCCC_FUNC___pkvm_init_vm,
    __KVM_HOST_SMCCC_FUNC___pkvm_init_vcpu,
    __KVM_HOST_SMCCC_FUNC___pkvm_vcpu_in_poison_fault,
    __KVM_HOST_SMCCC_FUNC___pkvm_force_reclaim_guest_page,
    __KVM_HOST_SMCCC_FUNC___pkvm_reclaim_dying_guest_page,
    __KVM_HOST_SMCCC_FUNC___pkvm_start_teardown_vm,
    __KVM_HOST_SMCCC_FUNC___pkvm_finalize_teardown_vm,
    __KVM_HOST_SMCCC_FUNC___pkvm_vcpu_load,
    __KVM_HOST_SMCCC_FUNC___pkvm_vcpu_put,
    __KVM_HOST_SMCCC_FUNC___pkvm_vcpu_sync_state,
    __KVM_HOST_SMCCC_FUNC___pkvm_tlb_flush_vmid,
    __KVM_HOST_SMCCC_FUNC_MAX,
}

#[repr(C)]
pub struct kvm_nvhe_init_params {
    pub mair_el2: ::core::ffi::c_ulong,
    pub tcr_el2: ::core::ffi::c_ulong,
    pub tpidr_el2: ::core::ffi::c_ulong,
    pub stack_hyp_va: ::core::ffi::c_ulong,
    pub stack_pa: ::core::ffi::c_ulong,
    pub pgd_pa: phys_addr_t,
    pub hcr_el2: ::core::ffi::c_ulong,
    pub vttbr: ::core::ffi::c_ulong,
    pub vtcr: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct kvm_nvhe_stacktrace_info {
    pub stack_base: ::core::ffi::c_ulong,
    pub overflow_stack_base: ::core::ffi::c_ulong,
    pub fp: ::core::ffi::c_ulong,
    pub pc: ::core::ffi::c_ulong,
}

pub struct kvm;
pub struct kvm_vcpu;
pub struct kvm_s2_mmu;
pub struct kvm_cpu_context;
pub struct alt_instr;
pub type __le32 = u32;

extern "C" {
    pub static mut kvm_nvhe_sym_kvm_arm_hyp_percpu_base: [::core::ffi::c_ulong; 0];
    pub fn __kvm_flush_vm_context();
    pub fn __kvm_flush_cpu_context(mmu: *mut kvm_s2_mmu);
    pub fn __kvm_tlb_flush_vmid_ipa(mmu: *mut kvm_s2_mmu, ipa: phys_addr_t, level: i32);
    pub fn __kvm_tlb_flush_vmid_ipa_nsh(mmu: *mut kvm_s2_mmu, ipa: phys_addr_t, level: i32);
    pub fn __kvm_tlb_flush_vmid_range(mmu: *mut kvm_s2_mmu, start: phys_addr_t, pages: ::core::ffi::c_ulong);
    pub fn __kvm_tlb_flush_vmid(mmu: *mut kvm_s2_mmu);
    pub fn __kvm_tlbi_s1e2(mmu: *mut kvm_s2_mmu, va: u64, sys_encoding: u64) -> i32;
    pub fn __kvm_timer_set_cntvoff(cntvoff: u64);
    pub fn __kvm_at_s1e01(vcpu: *mut kvm_vcpu, op: u32, vaddr: u64) -> i32;
    pub fn __kvm_at_s1e2(vcpu: *mut kvm_vcpu, op: u32, vaddr: u64) -> i32;
    pub fn __kvm_at_s12(vcpu: *mut kvm_vcpu, op: u32, vaddr: u64) -> i32;
    pub fn __kvm_vcpu_run(vcpu: *mut kvm_vcpu) -> i32;
    pub fn __kvm_adjust_pc(vcpu: *mut kvm_vcpu);
    pub fn __vgic_v3_get_gic_config() -> bool;
    pub fn __vgic_v3_init_lrs();
    pub fn hyp_panic() -> !;
    pub fn kvm_unexpected_el2_exception();
    pub fn hyp_panic_bad_stack() -> !;
    pub fn handle_trap(host_ctxt: *mut kvm_cpu_context);
    pub fn __kvm_host_psci_cpu_on_entry() -> !;
    pub fn __kvm_host_psci_cpu_resume_entry() -> !;
    pub fn __pkvm_init_finalise() -> !;
    pub fn kvm_nvhe_prepare_backtrace(fp: ::core::ffi::c_ulong, pc: ::core::ffi::c_ulong);
    pub fn kvm_patch_vector_branch(alt: *mut alt_instr, origptr: *mut __le32, updptr: *mut __le32, nr_inst: i32);
    pub fn kvm_get_kimage_voffset(alt: *mut alt_instr, origptr: *mut __le32, updptr: *mut __le32, nr_inst: i32);
    pub fn kvm_compute_final_ctr_el0(alt: *mut alt_instr, origptr: *mut __le32, updptr: *mut __le32, nr_inst: i32);
    pub fn nvhe_hyp_panic_handler(esr: u64, spsr: u64, elr_virt: u64, elr_phys: u64, par: u64, vcpu: usize, far: u64, hpfar: u64) -> !;
}

// The C header's assembler-only macros (get_host_ctxt, get_vcpu_ptr,
// get_loaded_vcpu, set_loaded_vcpu, _kvm_extable, save_callee_saved_regs,
// restore_callee_saved_regs, save_sp_el0, and restore_sp_el0) expand to
// AArch64 assembler and are retained here as dependency-facing assembly intent.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
