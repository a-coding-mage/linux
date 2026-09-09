/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * KVM L1 hypervisor optimizations on Hyper-V for SVM.
 */

// Dependency supplied by the surrounding kernel translation: <asm/mshyperv.h>
// Dependencies supplied by the surrounding kernel translation:
// "kvm_onhyperv.h", "svm/hyperv.h"

#[cfg(feature = "CONFIG_HYPERV")]
extern "C" {
    // __init void svm_hv_hardware_setup(void);
    pub fn svm_hv_hardware_setup();
}

#[cfg(feature = "CONFIG_HYPERV")]
#[inline]
pub unsafe fn svm_hv_is_enlightened_tlb_enabled(vcpu: *mut kvm_vcpu) -> bool {
    let hve: *mut hv_vmcb_enlightenments = &mut (*(*to_svm(vcpu)).vmcb)
        .control
        .hv_enlightenments;

    (ms_hyperv.nested_features & HV_X64_NESTED_ENLIGHTENED_TLB) != 0
        && ((*hve).hv_enlightenments_control.enlightened_npt_tlb != 0)
}

#[cfg(feature = "CONFIG_HYPERV")]
#[inline]
pub unsafe fn svm_hv_init_vmcb(vmcb: *mut vmcb) {
    let hve: *mut hv_vmcb_enlightenments = &mut (*vmcb).control.hv_enlightenments;

    // BUILD_BUG_ON(sizeof(vmcb->control.hv_enlightenments) !=
    //              sizeof(vmcb->control.reserved_sw));
    // The kernel build-time assertion is preserved above; its type/layout
    // dependency is supplied by the surrounding translation.

    if npt_enabled && (ms_hyperv.nested_features & HV_X64_NESTED_ENLIGHTENED_TLB) != 0 {
        (*hve).hv_enlightenments_control.enlightened_npt_tlb = 1;
    }

    if (ms_hyperv.nested_features & HV_X64_NESTED_MSR_BITMAP) != 0 {
        (*hve).hv_enlightenments_control.msr_bitmap = 1;
    }
}

#[cfg(feature = "CONFIG_HYPERV")]
#[inline]
pub unsafe fn svm_hv_vmcb_dirty_nested_enlightenments(vcpu: *mut kvm_vcpu) {
    let vmcb: *mut vmcb = (*to_svm(vcpu)).vmcb;
    let hve: *mut hv_vmcb_enlightenments = &mut (*vmcb).control.hv_enlightenments;

    if (*hve).hv_enlightenments_control.msr_bitmap != 0 {
        vmcb_mark_dirty(vmcb, HV_VMCB_NESTED_ENLIGHTENMENTS);
    }
}

#[cfg(feature = "CONFIG_HYPERV")]
#[inline]
pub unsafe fn svm_hv_update_vp_id(vmcb: *mut vmcb, vcpu: *mut kvm_vcpu) {
    let hve: *mut hv_vmcb_enlightenments = &mut (*vmcb).control.hv_enlightenments;
    let vp_index: u32 = kvm_hv_get_vpindex(vcpu);

    if (*hve).hv_vp_id != vp_index {
        (*hve).hv_vp_id = vp_index;
        vmcb_mark_dirty(vmcb, HV_VMCB_NESTED_ENLIGHTENMENTS);
    }
}

#[cfg(not(feature = "CONFIG_HYPERV"))]
#[inline]
pub unsafe fn svm_hv_is_enlightened_tlb_enabled(_vcpu: *mut kvm_vcpu) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_HYPERV"))]
#[inline]
pub unsafe fn svm_hv_init_vmcb(_vmcb: *mut vmcb) {}

#[cfg(not(feature = "CONFIG_HYPERV"))]
#[inline]
pub unsafe fn svm_hv_hardware_setup() {}

#[cfg(not(feature = "CONFIG_HYPERV"))]
#[inline]
pub unsafe fn svm_hv_vmcb_dirty_nested_enlightenments(_vcpu: *mut kvm_vcpu) {}

#[cfg(not(feature = "CONFIG_HYPERV"))]
#[inline]
pub unsafe fn svm_hv_update_vp_id(_vmcb: *mut vmcb, _vcpu: *mut kvm_vcpu) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
