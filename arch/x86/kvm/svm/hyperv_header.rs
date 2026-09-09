/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Common Hyper-V on KVM and KVM on Hyper-V definitions (SVM).
 */

// Dependencies supplied by the surrounding translation unit:
// asm/mshyperv.h, ../hyperv.h, and svm.h.

#[cfg(feature = "CONFIG_KVM_HYPERV")]
#[inline]
pub unsafe fn nested_svm_hv_update_vm_vp_ids(vcpu: *mut kvm_vcpu) {
    let svm: *mut vcpu_svm = to_svm(vcpu);
    let hve: *mut hv_vmcb_enlightenments =
        &mut (*svm).nested.ctl.hv_enlightenments;
    let hv_vcpu: *mut kvm_vcpu_hv = to_hv_vcpu(vcpu);

    if hv_vcpu.is_null() {
        return;
    }

    (*hv_vcpu).nested.pa_page_gpa = (*hve).partition_assist_page;
    (*hv_vcpu).nested.vm_id = (*hve).hv_vm_id;
    (*hv_vcpu).nested.vp_id = (*hve).hv_vp_id;
}

#[cfg(feature = "CONFIG_KVM_HYPERV")]
#[inline]
pub unsafe fn nested_svm_l2_tlb_flush_enabled(vcpu: *mut kvm_vcpu) -> bool {
    let svm: *mut vcpu_svm = to_svm(vcpu);
    let hve: *mut hv_vmcb_enlightenments =
        &mut (*svm).nested.ctl.hv_enlightenments;
    let hv_vcpu: *mut kvm_vcpu_hv = to_hv_vcpu(vcpu);

    if hv_vcpu.is_null() {
        return false;
    }

    if !(*hve).hv_enlightenments_control.nested_flush_hypercall {
        return false;
    }

    (*hv_vcpu).vp_assist_page.nested_control.features.directhypercall
}

#[cfg(feature = "CONFIG_KVM_HYPERV")]
#[inline]
pub unsafe fn nested_svm_is_l2_tlb_flush_hcall(vcpu: *mut kvm_vcpu) -> bool {
    guest_hv_cpuid_has_l2_tlb_flush(vcpu)
        && nested_svm_l2_tlb_flush_enabled(vcpu)
        && kvm_hv_is_tlb_flush_hcall(vcpu)
}

#[cfg(feature = "CONFIG_KVM_HYPERV")]
unsafe extern "C" {
    pub fn svm_hv_inject_synthetic_vmexit_post_tlb_flush(vcpu: *mut kvm_vcpu);
}

// CONFIG_KVM_HYPERV disabled: retain the no-op and false inline definitions.
#[cfg(not(feature = "CONFIG_KVM_HYPERV"))]
#[inline]
pub unsafe fn nested_svm_hv_update_vm_vp_ids(_vcpu: *mut kvm_vcpu) {}

#[cfg(not(feature = "CONFIG_KVM_HYPERV"))]
#[inline]
pub unsafe fn nested_svm_is_l2_tlb_flush_hcall(_vcpu: *mut kvm_vcpu) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_KVM_HYPERV"))]
#[inline]
pub unsafe fn svm_hv_inject_synthetic_vmexit_post_tlb_flush(_vcpu: *mut kvm_vcpu) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
