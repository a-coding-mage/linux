/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding translation unit:
// linux/kvm_host.h, capabilities.h, and vmx_ops.h.

#[cfg(feature = "CONFIG_X86_SGX_KVM")]
extern "C" {
    pub static mut enable_sgx: bool;

    pub fn handle_encls(vcpu: *mut kvm_vcpu) -> ::core::ffi::c_int;

    pub fn setup_default_sgx_lepubkeyhash();
    pub fn vcpu_setup_sgx_lepubkeyhash(vcpu: *mut kvm_vcpu);

    pub fn vmx_write_encls_bitmap(vcpu: *mut kvm_vcpu, vmcs12: *mut vmcs12);
}

#[cfg(not(feature = "CONFIG_X86_SGX_KVM"))]
pub const enable_sgx: bool = false;

#[cfg(not(feature = "CONFIG_X86_SGX_KVM"))]
#[inline]
pub unsafe fn setup_default_sgx_lepubkeyhash() {}

#[cfg(not(feature = "CONFIG_X86_SGX_KVM"))]
#[inline]
pub unsafe fn vcpu_setup_sgx_lepubkeyhash(_vcpu: *mut kvm_vcpu) {}

#[cfg(not(feature = "CONFIG_X86_SGX_KVM"))]
#[inline]
pub unsafe fn vmx_write_encls_bitmap(vcpu: *mut kvm_vcpu, vmcs12: *mut vmcs12) {
    // Nothing to do if hardware doesn't support SGX
    if cpu_has_vmx_encls_vmexit() {
        vmcs_write64(ENCLS_EXITING_BITMAP, u64::MAX);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
