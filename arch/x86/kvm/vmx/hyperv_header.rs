/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding KVM/VMX translation unit:
// linux/kvm_host.h, vmcs12.h, and vmx.h.

pub const EVMPTR_INVALID: u64 = (-1i64) as u64;
pub const EVMPTR_MAP_PENDING: u64 = (-2i64) as u64;

#[repr(C)]
pub enum nested_evmptrld_status {
    EVMPTRLD_DISABLED,
    EVMPTRLD_SUCCEEDED,
    EVMPTRLD_VMFAIL,
    EVMPTRLD_ERROR,
}

// CONFIG_KVM_HYPERV conditional declarations and definitions.
#[cfg(CONFIG_KVM_HYPERV)]
pub unsafe fn evmptr_is_valid(evmptr: u64) -> bool {
    evmptr != EVMPTR_INVALID && evmptr != EVMPTR_MAP_PENDING
}

#[cfg(CONFIG_KVM_HYPERV)]
pub unsafe fn nested_vmx_is_evmptr12_valid(vmx: *mut vcpu_vmx) -> bool {
    evmptr_is_valid((*vmx).nested.hv_evmcs_vmptr)
}

#[cfg(CONFIG_KVM_HYPERV)]
pub unsafe fn evmptr_is_set(evmptr: u64) -> bool {
    evmptr != EVMPTR_INVALID
}

#[cfg(CONFIG_KVM_HYPERV)]
pub unsafe fn nested_vmx_is_evmptr12_set(vmx: *mut vcpu_vmx) -> bool {
    evmptr_is_set((*vmx).nested.hv_evmcs_vmptr)
}

#[cfg(CONFIG_KVM_HYPERV)]
pub unsafe fn nested_vmx_evmcs(vmx: *mut vcpu_vmx) -> *mut hv_enlightened_vmcs {
    (*vmx).nested.hv_evmcs
}

#[cfg(CONFIG_KVM_HYPERV)]
pub unsafe fn guest_cpu_cap_has_evmcs(vcpu: *mut kvm_vcpu) -> bool {
    /*
     * eVMCS is exposed to the guest if Hyper-V is enabled in CPUID and
     * eVMCS has been explicitly enabled by userspace.
     */
    (*vcpu).arch.hyperv_enabled
        && (*to_vmx(vcpu)).nested.enlightened_vmcs_enabled
}

extern "C" {
    pub fn nested_get_evmptr(vcpu: *mut kvm_vcpu) -> u64;
    pub fn nested_get_evmcs_version(vcpu: *mut kvm_vcpu) -> u16;
    pub fn nested_enable_evmcs(vcpu: *mut kvm_vcpu, vmcs_version: *mut u16) -> i32;
    pub fn nested_evmcs_filter_control_msr(
        vcpu: *mut kvm_vcpu,
        msr_index: u32,
        pdata: *mut u64,
    );
    pub fn nested_evmcs_check_controls(vmcs12: *mut vmcs12) -> i32;
    pub fn nested_evmcs_l2_tlb_flush_enabled(vcpu: *mut kvm_vcpu) -> bool;
    pub fn vmx_hv_inject_synthetic_vmexit_post_tlb_flush(vcpu: *mut kvm_vcpu);
}

#[cfg(not(CONFIG_KVM_HYPERV))]
pub unsafe fn evmptr_is_valid(_evmptr: u64) -> bool {
    false
}

#[cfg(not(CONFIG_KVM_HYPERV))]
pub unsafe fn nested_vmx_is_evmptr12_valid(_vmx: *mut vcpu_vmx) -> bool {
    false
}

#[cfg(not(CONFIG_KVM_HYPERV))]
pub unsafe fn evmptr_is_set(_evmptr: u64) -> bool {
    false
}

#[cfg(not(CONFIG_KVM_HYPERV))]
pub unsafe fn nested_vmx_is_evmptr12_set(_vmx: *mut vcpu_vmx) -> bool {
    false
}

#[cfg(not(CONFIG_KVM_HYPERV))]
pub unsafe fn nested_vmx_evmcs(_vmx: *mut vcpu_vmx) -> *mut hv_enlightened_vmcs {
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
