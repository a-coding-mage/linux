/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding translation unit: regs.h, hyperv.h,
// vmcs12.h, and vmx.h.

#[repr(C)]
pub enum nvmx_vmentry_status {
    NVMX_VMENTRY_SUCCESS,
    NVMX_VMENTRY_VMFAIL,
    NVMX_VMENTRY_VMEXIT,
    NVMX_VMENTRY_KVM_INTERNAL_ERROR,
}

extern "C" {
    pub fn vmx_leave_nested(vcpu: *mut kvm_vcpu);
    pub fn nested_vmx_setup_ctls_msrs(vmcs_conf: *mut vmcs_config, ept_caps: u32);
    pub fn nested_vmx_hardware_unsetup();
    pub fn nested_vmx_hardware_setup(
        exit_handlers: *mut Option<unsafe extern "C" fn(*mut kvm_vcpu) -> i32>,
    ) -> i32;
    pub fn nested_vmx_set_vmcs_shadowing_bitmap();
    pub fn nested_vmx_check_restored_vmcs12(vcpu: *mut kvm_vcpu) -> i32;
    pub fn nested_vmx_free_vcpu(vcpu: *mut kvm_vcpu);
    pub fn nested_vmx_enter_non_root_mode(
        vcpu: *mut kvm_vcpu,
        from_vmentry: bool,
    ) -> nvmx_vmentry_status;
    pub fn nested_vmx_reflect_vmexit(vcpu: *mut kvm_vcpu) -> bool;
    pub fn __nested_vmx_vmexit(
        vcpu: *mut kvm_vcpu,
        vm_exit_reason: u32,
        exit_intr_info: u32,
        exit_qualification: usize,
        exit_insn_len: u32,
    );

    pub fn vmcs_read32(field: u32) -> u32;
    pub fn nested_sync_vmcs12_to_shadow(vcpu: *mut kvm_vcpu);
    pub fn vmx_set_vmx_msr(vcpu: *mut kvm_vcpu, msr_index: u32, data: u64) -> i32;
    pub fn vmx_get_vmx_msr(msrs: *mut nested_vmx_msrs, msr_index: u32, pdata: *mut u64) -> i32;
    pub fn get_vmx_mem_address(
        vcpu: *mut kvm_vcpu,
        exit_qualification: usize,
        vmx_instruction_info: u32,
        wr: bool,
        len: i32,
        ret: *mut gva_t,
    ) -> i32;
    pub fn nested_vmx_check_io_bitmaps(vcpu: *mut kvm_vcpu, port: u32, size: i32) -> bool;
    pub fn kvm_lockdep_assert_vcpu_is_locked_or_unreachable(vcpu: *mut kvm_vcpu);
    pub fn to_vmx(vcpu: *mut kvm_vcpu) -> *mut vcpu_vmx;
    pub fn nested_vmx_is_evmptr12_set(vmx: *mut vcpu_vmx) -> bool;
    pub fn vmx_misc_cr3_count(value: u64) -> u32;
    pub fn __kvm_is_valid_cr4(vcpu: *mut kvm_vcpu, val: usize) -> bool;
}

pub unsafe fn nested_vmx_vmexit(
    vcpu: *mut kvm_vcpu,
    vm_exit_reason: u32,
    exit_intr_info: u32,
    exit_qualification: usize,
) {
    let exit_insn_len: u32;
    if (*to_vmx(vcpu)).fail || vm_exit_reason == u32::MAX
        || (vm_exit_reason & VMX_EXIT_REASONS_FAILED_VMENTRY) != 0
    {
        exit_insn_len = 0;
    } else {
        exit_insn_len = vmcs_read32(VM_EXIT_INSTRUCTION_LEN);
    }
    __nested_vmx_vmexit(vcpu, vm_exit_reason, exit_intr_info, exit_qualification, exit_insn_len);
}

pub unsafe fn get_vmcs12(vcpu: *mut kvm_vcpu) -> *mut vmcs12 {
    kvm_lockdep_assert_vcpu_is_locked_or_unreachable(vcpu);
    (*to_vmx(vcpu)).nested.cached_vmcs12
}

pub unsafe fn get_shadow_vmcs12(vcpu: *mut kvm_vcpu) -> *mut vmcs12 {
    kvm_lockdep_assert_vcpu_is_locked_or_unreachable(vcpu);
    (*to_vmx(vcpu)).nested.cached_shadow_vmcs12
}

pub unsafe fn vmx_has_valid_vmcs12(vcpu: *mut kvm_vcpu) -> bool {
    let vmx = to_vmx(vcpu);
    (*vmx).nested.current_vmptr != u64::MAX || nested_vmx_is_evmptr12_set(vmx)
}

pub unsafe fn nested_get_vpid02(vcpu: *mut kvm_vcpu) -> u16 {
    let vmx = to_vmx(vcpu);
    if (*vmx).nested.vpid02 != 0 { (*vmx).nested.vpid02 } else { (*vmx).vpid }
}

pub unsafe fn nested_ept_get_eptp(vcpu: *mut kvm_vcpu) -> usize { (*get_vmcs12(vcpu)).ept_pointer }
pub unsafe fn nested_ept_ad_enabled(vcpu: *mut kvm_vcpu) -> bool {
    (nested_ept_get_eptp(vcpu) & VMX_EPTP_AD_ENABLE_BIT) != 0
}
pub unsafe fn nested_read_cr0(fields: *mut vmcs12) -> usize {
    ((*fields).guest_cr0 & !(*fields).cr0_guest_host_mask)
        | ((*fields).cr0_read_shadow & (*fields).cr0_guest_host_mask)
}
pub unsafe fn nested_read_cr4(fields: *mut vmcs12) -> usize {
    ((*fields).guest_cr4 & !(*fields).cr4_guest_host_mask)
        | ((*fields).cr4_read_shadow & (*fields).cr4_guest_host_mask)
}
pub unsafe fn nested_cpu_vmx_misc_cr3_count(vcpu: *mut kvm_vcpu) -> u32 {
    vmx_misc_cr3_count((*to_vmx(vcpu)).nested.msrs.misc_low)
}
pub unsafe fn nested_cpu_has_vmwrite_any_field(vcpu: *mut kvm_vcpu) -> bool {
    ((*to_vmx(vcpu)).nested.msrs.misc_low & VMX_MISC_VMWRITE_SHADOW_RO_FIELDS) != 0
}
pub unsafe fn nested_cpu_has_zero_length_injection(vcpu: *mut kvm_vcpu) -> bool {
    ((*to_vmx(vcpu)).nested.msrs.misc_low & VMX_MISC_ZERO_LEN_INS) != 0
}
pub unsafe fn nested_cpu_supports_monitor_trap_flag(vcpu: *mut kvm_vcpu) -> bool {
    ((*to_vmx(vcpu)).nested.msrs.procbased_ctls_high & CPU_BASED_MONITOR_TRAP_FLAG) != 0
}
pub unsafe fn nested_cpu_has_vmx_shadow_vmcs(vcpu: *mut kvm_vcpu) -> bool {
    ((*to_vmx(vcpu)).nested.msrs.secondary_ctls_high & SECONDARY_EXEC_SHADOW_VMCS) != 0
}
pub unsafe fn nested_cpu_has(vmcs12: *mut vmcs12, bit: u32) -> bool { ((*vmcs12).cpu_based_vm_exec_control & bit) != 0 }
pub unsafe fn nested_cpu_has2(vmcs12: *mut vmcs12, bit: u32) -> bool {
    ((*vmcs12).cpu_based_vm_exec_control & CPU_BASED_ACTIVATE_SECONDARY_CONTROLS) != 0
        && ((*vmcs12).secondary_vm_exec_control & bit) != 0
}
pub unsafe fn nested_cpu_has_preemption_timer(v: *mut vmcs12) -> bool { ((*v).pin_based_vm_exec_control & PIN_BASED_VMX_PREEMPTION_TIMER) != 0 }
pub unsafe fn nested_cpu_has_nmi_exiting(v: *mut vmcs12) -> bool { ((*v).pin_based_vm_exec_control & PIN_BASED_NMI_EXITING) != 0 }
pub unsafe fn nested_cpu_has_virtual_nmis(v: *mut vmcs12) -> bool { ((*v).pin_based_vm_exec_control & PIN_BASED_VIRTUAL_NMIS) != 0 }
pub unsafe fn nested_cpu_has_mtf(v: *mut vmcs12) -> i32 { nested_cpu_has(v, CPU_BASED_MONITOR_TRAP_FLAG) as i32 }
pub unsafe fn nested_cpu_has_ept(v: *mut vmcs12) -> i32 { nested_cpu_has2(v, SECONDARY_EXEC_ENABLE_EPT) as i32 }
pub unsafe fn nested_cpu_has_xsaves(v: *mut vmcs12) -> bool { nested_cpu_has2(v, SECONDARY_EXEC_ENABLE_XSAVES) }
pub unsafe fn nested_cpu_has_pml(v: *mut vmcs12) -> bool { nested_cpu_has2(v, SECONDARY_EXEC_ENABLE_PML) }
pub unsafe fn nested_cpu_has_virt_x2apic_mode(v: *mut vmcs12) -> bool { nested_cpu_has2(v, SECONDARY_EXEC_VIRTUALIZE_X2APIC_MODE) }
pub unsafe fn nested_cpu_has_vpid(v: *mut vmcs12) -> bool { nested_cpu_has2(v, SECONDARY_EXEC_ENABLE_VPID) }
pub unsafe fn nested_cpu_has_apic_reg_virt(v: *mut vmcs12) -> bool { nested_cpu_has2(v, SECONDARY_EXEC_APIC_REGISTER_VIRT) }
pub unsafe fn nested_cpu_has_vid(v: *mut vmcs12) -> bool { nested_cpu_has2(v, SECONDARY_EXEC_VIRTUAL_INTR_DELIVERY) }
pub unsafe fn nested_cpu_has_posted_intr(v: *mut vmcs12) -> bool { ((*v).pin_based_vm_exec_control & PIN_BASED_POSTED_INTR) != 0 }
pub unsafe fn nested_cpu_has_vmfunc(v: *mut vmcs12) -> bool { nested_cpu_has2(v, SECONDARY_EXEC_ENABLE_VMFUNC) }
pub unsafe fn nested_cpu_has_eptp_switching(v: *mut vmcs12) -> bool { nested_cpu_has_vmfunc(v) && ((*v).vm_function_control & VMX_VMFUNC_EPTP_SWITCHING) != 0 }
pub unsafe fn nested_cpu_has_shadow_vmcs(v: *mut vmcs12) -> bool { nested_cpu_has2(v, SECONDARY_EXEC_SHADOW_VMCS) }
pub unsafe fn nested_cpu_has_save_preemption_timer(v: *mut vmcs12) -> bool { ((*v).vm_exit_controls & VM_EXIT_SAVE_VMX_PREEMPTION_TIMER) != 0 }
pub unsafe fn nested_exit_on_nmi(vcpu: *mut kvm_vcpu) -> bool { nested_cpu_has_nmi_exiting(get_vmcs12(vcpu)) }
pub unsafe fn nested_exit_on_intr(vcpu: *mut kvm_vcpu) -> bool { ((*get_vmcs12(vcpu)).pin_based_vm_exec_control & PIN_BASED_EXT_INTR_MASK) != 0 }
pub unsafe fn nested_cpu_has_encls_exit(v: *mut vmcs12) -> bool { nested_cpu_has2(v, SECONDARY_EXEC_ENCLS_EXITING) }
pub fn fixed_bits_valid(val: u64, fixed0: u64, fixed1: u64) -> bool { ((val & fixed1) | fixed0) == val }

pub unsafe fn nested_guest_cr0_valid(vcpu: *mut kvm_vcpu, mut val: usize) -> bool {
    let fixed0 = (*to_vmx(vcpu)).nested.msrs.cr0_fixed0;
    let fixed1 = (*to_vmx(vcpu)).nested.msrs.cr0_fixed1;
    if ((*to_vmx(vcpu)).nested.msrs.secondary_ctls_high & SECONDARY_EXEC_UNRESTRICTED_GUEST) != 0
        && nested_cpu_has2(get_vmcs12(vcpu), SECONDARY_EXEC_UNRESTRICTED_GUEST) { return fixed_bits_valid(val as u64, fixed0 & !(X86_CR0_PE | X86_CR0_PG), fixed1); }
    fixed_bits_valid(val as u64, fixed0, fixed1)
}
pub unsafe fn nested_host_cr0_valid(vcpu: *mut kvm_vcpu, val: usize) -> bool { fixed_bits_valid(val as u64, (*to_vmx(vcpu)).nested.msrs.cr0_fixed0, (*to_vmx(vcpu)).nested.msrs.cr0_fixed1) }
pub unsafe fn nested_cr4_valid(vcpu: *mut kvm_vcpu, val: usize) -> bool { fixed_bits_valid(val as u64, (*to_vmx(vcpu)).nested.msrs.cr4_fixed0, (*to_vmx(vcpu)).nested.msrs.cr4_fixed1) && __kvm_is_valid_cr4(vcpu, val) }
pub unsafe fn nested_cpu_has_no_hw_errcode_cc(vcpu: *mut kvm_vcpu) -> bool { ((*to_vmx(vcpu)).nested.msrs.basic & VMX_BASIC_NO_HW_ERROR_CODE_CC) != 0 }

pub use nested_cr4_valid as nested_guest_cr4_valid;
pub use nested_cr4_valid as nested_host_cr4_valid;

extern "C" { pub static mut vmx_nested_ops: kvm_x86_nested_ops; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
