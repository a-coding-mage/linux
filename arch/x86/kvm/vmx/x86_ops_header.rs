/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: <linux/kvm_host.h>, "capabilities.h", and "x86.h".

extern "C" {
    pub fn vmx_hardware_setup() -> ::core::ffi::c_int;

    pub static mut vt_x86_ops: kvm_x86_ops;
    pub static mut vt_init_ops: kvm_x86_init_ops;

    pub fn vmx_hardware_unsetup();
    pub fn vmx_check_processor_compat() -> ::core::ffi::c_int;
    pub fn vmx_enable_virtualization_cpu() -> ::core::ffi::c_int;
    pub fn vmx_disable_virtualization_cpu();
    pub fn vmx_emergency_disable_virtualization_cpu();
    pub fn vmx_vm_init(kvm: *mut kvm) -> ::core::ffi::c_int;
    pub fn vmx_vm_destroy(kvm: *mut kvm);
    pub fn vmx_vcpu_precreate(kvm: *mut kvm) -> ::core::ffi::c_int;
    pub fn vmx_vcpu_create(vcpu: *mut kvm_vcpu) -> ::core::ffi::c_int;
    pub fn vmx_vcpu_run(vcpu: *mut kvm_vcpu, run_flags: u64) -> fastpath_t;
    pub fn vmx_vcpu_free(vcpu: *mut kvm_vcpu);
    pub fn vmx_vcpu_reset(vcpu: *mut kvm_vcpu, init_event: bool);
    pub fn vmx_vcpu_load(vcpu: *mut kvm_vcpu, cpu: ::core::ffi::c_int);
    pub fn vmx_vcpu_put(vcpu: *mut kvm_vcpu);
    pub fn vmx_handle_exit(vcpu: *mut kvm_vcpu, exit_fastpath: fastpath_t) -> ::core::ffi::c_int;
    pub fn vmx_handle_exit_irqoff(vcpu: *mut kvm_vcpu);
    pub fn vmx_skip_emulated_instruction(vcpu: *mut kvm_vcpu) -> ::core::ffi::c_int;
    pub fn vmx_update_emulated_instruction(vcpu: *mut kvm_vcpu);
    pub fn vmx_unhandleable_emulation_required(vcpu: *mut kvm_vcpu) -> bool;
    pub fn vmx_set_msr(vcpu: *mut kvm_vcpu, msr_info: *mut msr_data) -> ::core::ffi::c_int;

    // #ifdef CONFIG_KVM_SMM
    pub fn vmx_smi_allowed(vcpu: *mut kvm_vcpu, for_injection: bool) -> ::core::ffi::c_int;
    pub fn vmx_enter_smm(vcpu: *mut kvm_vcpu, smram: *mut kvm_smram) -> ::core::ffi::c_int;
    pub fn vmx_leave_smm(vcpu: *mut kvm_vcpu, smram: *const kvm_smram) -> ::core::ffi::c_int;
    pub fn vmx_enable_smi_window(vcpu: *mut kvm_vcpu);
    // #endif

    pub fn vmx_check_emulate_instruction(
        vcpu: *mut kvm_vcpu,
        emul_type: ::core::ffi::c_int,
        insn: *mut ::core::ffi::c_void,
        insn_len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn vmx_check_intercept(
        vcpu: *mut kvm_vcpu,
        info: *mut x86_instruction_info,
        stage: x86_intercept_stage,
        exception: *mut x86_exception,
    ) -> ::core::ffi::c_int;
    pub fn vmx_apic_init_signal_blocked(vcpu: *mut kvm_vcpu) -> bool;
    pub fn vmx_migrate_timers(vcpu: *mut kvm_vcpu);
    pub fn vmx_set_virtual_apic_mode(vcpu: *mut kvm_vcpu);
    pub fn vmx_hwapic_isr_update(vcpu: *mut kvm_vcpu, max_isr: ::core::ffi::c_int);
    pub fn vmx_sync_pir_to_irr(vcpu: *mut kvm_vcpu) -> ::core::ffi::c_int;
    pub fn vmx_deliver_interrupt(apic: *mut kvm_lapic, delivery_mode: ::core::ffi::c_int, trig_mode: ::core::ffi::c_int, vector: ::core::ffi::c_int);
    pub fn vmx_vcpu_after_set_cpuid(vcpu: *mut kvm_vcpu);
    pub fn vmx_has_emulated_msr(kvm: *mut kvm, index: u32) -> bool;
    pub fn vmx_recalc_intercepts(vcpu: *mut kvm_vcpu);
    pub fn vmx_prepare_switch_to_guest(vcpu: *mut kvm_vcpu);
    pub fn vmx_update_exception_bitmap(vcpu: *mut kvm_vcpu);
    pub fn vmx_get_feature_msr(msr: u32, data: *mut u64) -> ::core::ffi::c_int;
    pub fn vmx_get_msr(vcpu: *mut kvm_vcpu, msr_info: *mut msr_data) -> ::core::ffi::c_int;
    pub fn vmx_get_segment_base(vcpu: *mut kvm_vcpu, seg: ::core::ffi::c_int) -> u64;
    pub fn vmx_get_segment(vcpu: *mut kvm_vcpu, var: *mut kvm_segment, seg: ::core::ffi::c_int);
    pub fn vmx_set_segment(vcpu: *mut kvm_vcpu, var: *mut kvm_segment, seg: ::core::ffi::c_int);
    pub fn vmx_get_cpl(vcpu: *mut kvm_vcpu) -> ::core::ffi::c_int;
    pub fn vmx_get_cs_db_l_bits(vcpu: *mut kvm_vcpu, db: *mut ::core::ffi::c_int, l: *mut ::core::ffi::c_int);
    pub fn vmx_is_valid_cr0(vcpu: *mut kvm_vcpu, cr0: ::core::ffi::c_ulong) -> bool;
    pub fn vmx_set_cr0(vcpu: *mut kvm_vcpu, cr0: ::core::ffi::c_ulong);
    pub fn vmx_load_mmu_pgd(vcpu: *mut kvm_vcpu, root_hpa: hpa_t, root_level: ::core::ffi::c_int);
    pub fn vmx_set_cr4(vcpu: *mut kvm_vcpu, cr4: ::core::ffi::c_ulong);
    pub fn vmx_is_valid_cr4(vcpu: *mut kvm_vcpu, cr4: ::core::ffi::c_ulong) -> bool;
    pub fn vmx_set_efer(vcpu: *mut kvm_vcpu, efer: u64) -> ::core::ffi::c_int;
    pub fn vmx_get_idt(vcpu: *mut kvm_vcpu, dt: *mut desc_ptr);
    pub fn vmx_set_idt(vcpu: *mut kvm_vcpu, dt: *mut desc_ptr);
    pub fn vmx_get_gdt(vcpu: *mut kvm_vcpu, dt: *mut desc_ptr);
    pub fn vmx_set_gdt(vcpu: *mut kvm_vcpu, dt: *mut desc_ptr);
    pub fn vmx_set_dr7(vcpu: *mut kvm_vcpu, val: ::core::ffi::c_ulong);
    pub fn vmx_sync_dirty_debug_regs(vcpu: *mut kvm_vcpu);
    pub fn vmx_cache_reg(vcpu: *mut kvm_vcpu, reg: kvm_reg);
    pub fn vmx_get_rflags(vcpu: *mut kvm_vcpu) -> ::core::ffi::c_ulong;
    pub fn vmx_set_rflags(vcpu: *mut kvm_vcpu, rflags: ::core::ffi::c_ulong);
    pub fn vmx_get_if_flag(vcpu: *mut kvm_vcpu) -> bool;
    pub fn vmx_flush_tlb_all(vcpu: *mut kvm_vcpu);
    pub fn vmx_flush_tlb_current(vcpu: *mut kvm_vcpu);
    pub fn vmx_flush_tlb_gva(vcpu: *mut kvm_vcpu, addr: gva_t, full: *mut bool);
    pub fn vmx_flush_tlb_guest(vcpu: *mut kvm_vcpu);
    pub fn vmx_set_interrupt_shadow(vcpu: *mut kvm_vcpu, mask: ::core::ffi::c_int);
    pub fn vmx_get_interrupt_shadow(vcpu: *mut kvm_vcpu) -> u32;
    pub fn vmx_patch_hypercall(vcpu: *mut kvm_vcpu, hypercall: *mut u8);
    pub fn vmx_inject_irq(vcpu: *mut kvm_vcpu, reinjected: bool);
    pub fn vmx_inject_nmi(vcpu: *mut kvm_vcpu);
    pub fn vmx_inject_exception(vcpu: *mut kvm_vcpu);
    pub fn vmx_cancel_injection(vcpu: *mut kvm_vcpu);
    pub fn vmx_interrupt_allowed(vcpu: *mut kvm_vcpu, for_injection: bool) -> ::core::ffi::c_int;
    pub fn vmx_nmi_allowed(vcpu: *mut kvm_vcpu, for_injection: bool) -> ::core::ffi::c_int;
    pub fn vmx_get_nmi_mask(vcpu: *mut kvm_vcpu) -> bool;
    pub fn vmx_set_nmi_mask(vcpu: *mut kvm_vcpu, masked: bool);
    pub fn vmx_enable_nmi_window(vcpu: *mut kvm_vcpu);
    pub fn vmx_enable_irq_window(vcpu: *mut kvm_vcpu);
    pub fn vmx_update_cr8_intercept(vcpu: *mut kvm_vcpu, tpr: ::core::ffi::c_int, irr: ::core::ffi::c_int);
    pub fn vmx_set_apic_access_page_addr(vcpu: *mut kvm_vcpu);
    pub fn vmx_refresh_apicv_exec_ctrl(vcpu: *mut kvm_vcpu);
    pub fn vmx_load_eoi_exitmap(vcpu: *mut kvm_vcpu, eoi_exit_bitmap: *mut u64);
    pub fn vmx_set_tss_addr(kvm: *mut kvm, addr: u32) -> ::core::ffi::c_int;
    pub fn vmx_set_identity_map_addr(kvm: *mut kvm, ident_addr: u64) -> ::core::ffi::c_int;
    pub fn vmx_get_mt_mask(vcpu: *mut kvm_vcpu, gfn: gfn_t, is_mmio: bool) -> u8;

    pub fn vmx_get_exit_info(vcpu: *mut kvm_vcpu, reason: *mut u32, info1: *mut u64, info2: *mut u64, intr_info: *mut u32, error_code: *mut u32);
    pub fn vmx_get_entry_info(vcpu: *mut kvm_vcpu, intr_info: *mut u32, error_code: *mut u32);
    pub fn vmx_get_l2_tsc_offset(vcpu: *mut kvm_vcpu) -> u64;
    pub fn vmx_get_l2_tsc_multiplier(vcpu: *mut kvm_vcpu) -> u64;
    pub fn vmx_write_tsc_offset(vcpu: *mut kvm_vcpu);
    pub fn vmx_write_tsc_multiplier(vcpu: *mut kvm_vcpu);
    pub fn vmx_update_cpu_dirty_logging(vcpu: *mut kvm_vcpu);

    // #ifdef CONFIG_X86_64
    pub fn vmx_set_hv_timer(vcpu: *mut kvm_vcpu, guest_deadline_tsc: u64, expired: *mut bool) -> ::core::ffi::c_int;
    pub fn vmx_cancel_hv_timer(vcpu: *mut kvm_vcpu);
    // #endif
    pub fn vmx_setup_mce(vcpu: *mut kvm_vcpu);

    // #ifdef CONFIG_KVM_INTEL_TDX
    pub fn tdx_disable_virtualization_cpu();
    pub fn tdx_vm_init(kvm: *mut kvm) -> ::core::ffi::c_int;
    pub fn tdx_mmu_release_hkid(kvm: *mut kvm);
    pub fn tdx_vm_destroy(kvm: *mut kvm);
    pub fn tdx_vm_ioctl(kvm: *mut kvm, argp: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn tdx_vcpu_create(vcpu: *mut kvm_vcpu) -> ::core::ffi::c_int;
    pub fn tdx_vcpu_reset(vcpu: *mut kvm_vcpu, init_event: bool);
    pub fn tdx_vcpu_free(vcpu: *mut kvm_vcpu);
    pub fn tdx_vcpu_load(vcpu: *mut kvm_vcpu, cpu: ::core::ffi::c_int);
    pub fn tdx_vcpu_needs_initialization(vcpu: *mut kvm_vcpu) -> bool;
    pub fn tdx_vcpu_run(vcpu: *mut kvm_vcpu, run_flags: u64) -> fastpath_t;
    pub fn tdx_prepare_switch_to_guest(vcpu: *mut kvm_vcpu);
    pub fn tdx_vcpu_put(vcpu: *mut kvm_vcpu);
    pub fn tdx_handle_exit(vcpu: *mut kvm_vcpu, fastpath: exit_fastpath_completion) -> ::core::ffi::c_int;
    pub fn tdx_deliver_interrupt(apic: *mut kvm_lapic, delivery_mode: ::core::ffi::c_int, trig_mode: ::core::ffi::c_int, vector: ::core::ffi::c_int);
    pub fn tdx_inject_nmi(vcpu: *mut kvm_vcpu);
    pub fn tdx_get_exit_info(vcpu: *mut kvm_vcpu, reason: *mut u32, info1: *mut u64, info2: *mut u64, intr_info: *mut u32, error_code: *mut u32);
    pub fn tdx_has_emulated_msr(index: u32) -> bool;
    pub fn tdx_get_msr(vcpu: *mut kvm_vcpu, msr: *mut msr_data) -> ::core::ffi::c_int;
    pub fn tdx_set_msr(vcpu: *mut kvm_vcpu, msr: *mut msr_data) -> ::core::ffi::c_int;
    pub fn tdx_vcpu_ioctl(vcpu: *mut kvm_vcpu, argp: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn tdx_vcpu_unlocked_ioctl(vcpu: *mut kvm_vcpu, argp: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn tdx_flush_tlb_current(vcpu: *mut kvm_vcpu);
    pub fn tdx_flush_tlb_all(vcpu: *mut kvm_vcpu);
    pub fn tdx_load_mmu_pgd(vcpu: *mut kvm_vcpu, root_hpa: hpa_t, root_level: ::core::ffi::c_int);
    pub fn tdx_gmem_max_mapping_level(kvm: *mut kvm, pfn: kvm_pfn_t, is_private: bool) -> ::core::ffi::c_int;
    // #endif
}

// #define vmx_complete_emulated_msr kvm_complete_insn_gp
pub use kvm_complete_insn_gp as vmx_complete_emulated_msr;

#[inline]
pub unsafe fn vmx_tdp_has_smep(_kvm: *mut kvm) -> bool {
    enable_mbec
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
