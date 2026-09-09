/* SPDX-License-Identifier: GPL-2.0 */
// Translated from capabilities.h. C header dependencies are supplied externally.

pub const PT_MODE_SYSTEM: i32 = 0;
pub const PT_MODE_HOST_GUEST: i32 = 1;

extern "C" {
    pub static mut enable_vpid: bool;
    pub static mut flexpriority_enabled: bool;
    pub static mut enable_ept: bool;
    pub static mut enable_unrestricted_guest: bool;
    pub static mut enable_ept_ad_bits: bool;
    pub static mut enable_cet: bool;
    pub static mut enable_pml: bool;
    pub static mut enable_mbec: bool;
    pub static mut pt_mode: i32;
    pub static mut vmcs_config: vmcs_config;
    pub static mut vmx_capability: vmx_capability;
    pub static mut kvm_pmu_cap: kvm_pmu_cap;
    pub static mut enable_mediated_pmu: bool;
    pub fn lapic_in_kernel(vcpu: *mut kvm_vcpu) -> bool;
    pub fn boot_cpu_has(feature: u32) -> bool;
}

#[repr(C)]
pub struct nested_vmx_msrs {
    /* Only the true VMX capability MSRs are stored; non-true versions are
     * generated with the must-be-1 bits according to the SDM. */
    pub procbased_ctls_low: u32,
    pub procbased_ctls_high: u32,
    pub secondary_ctls_low: u32,
    pub secondary_ctls_high: u32,
    pub pinbased_ctls_low: u32,
    pub pinbased_ctls_high: u32,
    pub exit_ctls_low: u32,
    pub exit_ctls_high: u32,
    pub entry_ctls_low: u32,
    pub entry_ctls_high: u32,
    pub misc_low: u32,
    pub misc_high: u32,
    pub ept_caps: u32,
    pub vpid_caps: u32,
    pub basic: u64,
    pub cr0_fixed0: u64,
    pub cr0_fixed1: u64,
    pub cr4_fixed0: u64,
    pub cr4_fixed1: u64,
    pub vmcs_enum: u64,
    pub vmfunc_controls: u64,
}

#[repr(C)]
pub struct vmcs_config {
    pub basic: u64,
    pub pin_based_exec_ctrl: u32,
    pub cpu_based_exec_ctrl: u32,
    pub cpu_based_2nd_exec_ctrl: u32,
    pub cpu_based_3rd_exec_ctrl: u64,
    pub vmexit_ctrl: u32,
    pub vmentry_ctrl: u32,
    pub misc: u64,
    pub nested: nested_vmx_msrs,
}

#[repr(C)]
pub struct vmx_capability {
    pub ept: u32,
    pub vpid: u32,
}

#[repr(C)]
pub struct kvm_vcpu { _private: [u8; 0] }
#[repr(C)]
pub struct kvm_pmu_cap { pub pebs_ept: bool }

macro_rules! cap {
    ($name:ident, $expr:expr) => { #[inline] pub unsafe fn $name() -> bool { ($expr) != 0 } };
}

cap!(cpu_has_vmx_basic_inout, vmcs_config.basic & VMX_BASIC_INOUT);
cap!(cpu_has_vmx_basic_no_hw_errcode_cc, vmcs_config.basic & VMX_BASIC_NO_HW_ERROR_CODE_CC);
cap!(cpu_has_virtual_nmis, (vmcs_config.pin_based_exec_ctrl & PIN_BASED_VIRTUAL_NMIS) != 0 && (vmcs_config.cpu_based_exec_ctrl & CPU_BASED_NMI_WINDOW_EXITING) != 0);
cap!(cpu_has_vmx_preemption_timer, vmcs_config.pin_based_exec_ctrl & PIN_BASED_VMX_PREEMPTION_TIMER);
cap!(cpu_has_vmx_posted_intr, vmcs_config.pin_based_exec_ctrl & PIN_BASED_POSTED_INTR);
cap!(cpu_has_load_ia32_efer, vmcs_config.vmentry_ctrl & VM_ENTRY_LOAD_IA32_EFER);
cap!(cpu_has_load_perf_global_ctrl, vmcs_config.vmentry_ctrl & VM_ENTRY_LOAD_IA32_PERF_GLOBAL_CTRL);
cap!(cpu_has_load_cet_ctrl, vmcs_config.vmentry_ctrl & VM_ENTRY_LOAD_CET_STATE);
cap!(cpu_has_save_perf_global_ctrl, vmcs_config.vmexit_ctrl & VM_EXIT_SAVE_IA32_PERF_GLOBAL_CTRL);
cap!(cpu_has_vmx_mpx, vmcs_config.vmentry_ctrl & VM_ENTRY_LOAD_BNDCFGS);
cap!(cpu_has_vmx_tpr_shadow, vmcs_config.cpu_based_exec_ctrl & CPU_BASED_TPR_SHADOW);
cap!(cpu_has_vmx_msr_bitmap, vmcs_config.cpu_based_exec_ctrl & CPU_BASED_USE_MSR_BITMAPS);
cap!(cpu_has_secondary_exec_ctrls, vmcs_config.cpu_based_exec_ctrl & CPU_BASED_ACTIVATE_SECONDARY_CONTROLS);
cap!(cpu_has_tertiary_exec_ctrls, vmcs_config.cpu_based_exec_ctrl & CPU_BASED_ACTIVATE_TERTIARY_CONTROLS);
cap!(cpu_has_vmx_virtualize_apic_accesses, vmcs_config.cpu_based_2nd_exec_ctrl & SECONDARY_EXEC_VIRTUALIZE_APIC_ACCESSES);
cap!(cpu_has_vmx_ept, vmcs_config.cpu_based_2nd_exec_ctrl & SECONDARY_EXEC_ENABLE_EPT);
cap!(cpu_has_vmx_rdtscp, vmcs_config.cpu_based_2nd_exec_ctrl & SECONDARY_EXEC_ENABLE_RDTSCP);
cap!(cpu_has_vmx_virtualize_x2apic_mode, vmcs_config.cpu_based_2nd_exec_ctrl & SECONDARY_EXEC_VIRTUALIZE_X2APIC_MODE);
cap!(cpu_has_vmx_vpid, vmcs_config.cpu_based_2nd_exec_ctrl & SECONDARY_EXEC_ENABLE_VPID);
cap!(cpu_has_vmx_wbinvd_exit, vmcs_config.cpu_based_2nd_exec_ctrl & SECONDARY_EXEC_WBINVD_EXITING);
cap!(cpu_has_vmx_unrestricted_guest, vmcs_config.cpu_based_2nd_exec_ctrl & SECONDARY_EXEC_UNRESTRICTED_GUEST);
cap!(cpu_has_vmx_apic_register_virt, vmcs_config.cpu_based_2nd_exec_ctrl & SECONDARY_EXEC_APIC_REGISTER_VIRT);
cap!(cpu_has_vmx_virtual_intr_delivery, vmcs_config.cpu_based_2nd_exec_ctrl & SECONDARY_EXEC_VIRTUAL_INTR_DELIVERY);
cap!(cpu_has_vmx_ple, vmcs_config.cpu_based_2nd_exec_ctrl & SECONDARY_EXEC_PAUSE_LOOP_EXITING);
cap!(cpu_has_vmx_rdrand, vmcs_config.cpu_based_2nd_exec_ctrl & SECONDARY_EXEC_RDRAND_EXITING);
cap!(cpu_has_vmx_invpcid, vmcs_config.cpu_based_2nd_exec_ctrl & SECONDARY_EXEC_ENABLE_INVPCID);
cap!(cpu_has_vmx_vmfunc, vmcs_config.cpu_based_2nd_exec_ctrl & SECONDARY_EXEC_ENABLE_VMFUNC);
cap!(cpu_has_vmx_encls_vmexit, vmcs_config.cpu_based_2nd_exec_ctrl & SECONDARY_EXEC_ENCLS_EXITING);
cap!(cpu_has_vmx_rdseed, vmcs_config.cpu_based_2nd_exec_ctrl & SECONDARY_EXEC_RDSEED_EXITING);
cap!(cpu_has_vmx_pml, vmcs_config.cpu_based_2nd_exec_ctrl & SECONDARY_EXEC_ENABLE_PML);
cap!(cpu_has_vmx_xsaves, vmcs_config.cpu_based_2nd_exec_ctrl & SECONDARY_EXEC_ENABLE_XSAVES);
cap!(cpu_has_vmx_waitpkg, vmcs_config.cpu_based_2nd_exec_ctrl & SECONDARY_EXEC_ENABLE_USR_WAIT_PAUSE);
cap!(cpu_has_vmx_tsc_scaling, vmcs_config.cpu_based_2nd_exec_ctrl & SECONDARY_EXEC_TSC_SCALING);
cap!(cpu_has_vmx_bus_lock_detection, vmcs_config.cpu_based_2nd_exec_ctrl & SECONDARY_EXEC_BUS_LOCK_DETECTION);
cap!(cpu_has_vmx_ipiv, vmcs_config.cpu_based_3rd_exec_ctrl & TERTIARY_EXEC_IPI_VIRT);
cap!(cpu_has_vmx_ept_4levels, vmx_capability.ept & VMX_EPT_PAGE_WALK_4_BIT);
cap!(cpu_has_vmx_ept_5levels, vmx_capability.ept & VMX_EPT_PAGE_WALK_5_BIT);
cap!(cpu_has_vmx_ept_mt_wb, vmx_capability.ept & VMX_EPTP_WB_BIT);
cap!(cpu_has_vmx_ept_2m_page, vmx_capability.ept & VMX_EPT_2MB_PAGE_BIT);
cap!(cpu_has_vmx_ept_1g_page, vmx_capability.ept & VMX_EPT_1GB_PAGE_BIT);
cap!(cpu_has_vmx_ept_ad_bits, vmx_capability.ept & VMX_EPT_AD_BIT);
cap!(cpu_has_vmx_invept_context, vmx_capability.ept & VMX_EPT_EXTENT_CONTEXT_BIT);
cap!(cpu_has_vmx_invept_global, vmx_capability.ept & VMX_EPT_EXTENT_GLOBAL_BIT);
cap!(cpu_has_vmx_invvpid, vmx_capability.vpid & VMX_VPID_INVVPID_BIT);
cap!(cpu_has_vmx_invvpid_individual_addr, vmx_capability.vpid & VMX_VPID_EXTENT_INDIVIDUAL_ADDR_BIT);
cap!(cpu_has_vmx_invvpid_single, vmx_capability.vpid & VMX_VPID_EXTENT_SINGLE_CONTEXT_BIT);
cap!(cpu_has_vmx_invvpid_global, vmx_capability.vpid & VMX_VPID_EXTENT_GLOBAL_CONTEXT_BIT);

#[inline] pub unsafe fn cpu_need_tpr_shadow(vcpu: *mut kvm_vcpu) -> bool { cpu_has_vmx_tpr_shadow() && lapic_in_kernel(vcpu) }
#[inline] pub unsafe fn vmx_umip_emulated() -> bool { !boot_cpu_has(X86_FEATURE_UMIP) && (vmcs_config.cpu_based_2nd_exec_ctrl & SECONDARY_EXEC_DESC) != 0 }
#[inline] pub unsafe fn cpu_has_vmx_shadow_vmcs() -> bool { (vmcs_config.misc & VMX_MISC_VMWRITE_SHADOW_RO_FIELDS) != 0 && (vmcs_config.cpu_based_2nd_exec_ctrl & SECONDARY_EXEC_SHADOW_VMCS) != 0 }
#[inline] pub unsafe fn cpu_has_vmx_apicv() -> bool { cpu_has_vmx_apic_register_virt() && cpu_has_vmx_virtual_intr_delivery() && cpu_has_vmx_posted_intr() }
#[inline] pub unsafe fn cpu_has_vmx_flexpriority() -> bool { cpu_has_vmx_tpr_shadow() && cpu_has_vmx_virtualize_apic_accesses() }
#[inline] pub unsafe fn ept_caps_to_lpage_level(ept_caps: u32) -> i32 { if ept_caps & VMX_EPT_1GB_PAGE_BIT != 0 { PG_LEVEL_1G } else if ept_caps & VMX_EPT_2MB_PAGE_BIT != 0 { PG_LEVEL_2M } else { PG_LEVEL_4K } }
#[inline] pub unsafe fn cpu_has_vmx_intel_pt() -> bool { (vmcs_config.misc & VMX_MISC_INTEL_PT) != 0 && (vmcs_config.cpu_based_2nd_exec_ctrl & SECONDARY_EXEC_PT_USE_GPA) != 0 && (vmcs_config.vmentry_ctrl & VM_ENTRY_LOAD_IA32_RTIT_CTL) != 0 }
/* Processor Trace modes: system-wide, host-only, and host-guest. KVM supports
 * system-wide and host-guest. */
#[inline] pub unsafe fn vmx_pt_mode_is_system() -> bool { pt_mode == PT_MODE_SYSTEM }
#[inline] pub unsafe fn vmx_pt_mode_is_host_guest() -> bool { pt_mode == PT_MODE_HOST_GUEST }
#[inline] pub unsafe fn vmx_pebs_supported() -> bool { boot_cpu_has(X86_FEATURE_PEBS) && kvm_pmu_cap.pebs_ept && !enable_mediated_pmu }
cap!(cpu_has_notify_vmexit, vmcs_config.cpu_based_2nd_exec_ctrl & SECONDARY_EXEC_NOTIFY_VM_EXITING);
cap!(cpu_has_ept_mbec, vmcs_config.cpu_based_2nd_exec_ctrl & SECONDARY_EXEC_MODE_BASED_EPT_EXEC);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
