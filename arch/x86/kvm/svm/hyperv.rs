// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD SVM specific code for Hyper-V on KVM.
 *
 * Copyright 2022 Red Hat, Inc. and/or its affiliates.
 */

// Dependency intent: declarations and constants are supplied by hyperv.h and
// the surrounding KVM/SVM implementation.

#[repr(C)]
pub struct kvm_vcpu {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vmcb_control_area {
    pub exit_code: u64,
    pub exit_info_1: u64,
    pub exit_info_2: u64,
}

#[repr(C)]
pub struct vmcb {
    pub control: vmcb_control_area,
}

#[repr(C)]
pub struct vcpu_svm {
    pub vmcb: *mut vmcb,
}

extern "C" {
    fn to_svm(vcpu: *mut kvm_vcpu) -> *mut vcpu_svm;
    fn nested_svm_vmexit(svm: *mut vcpu_svm);
}

// BUILD_BUG_ON(HV_SVM_EXITCODE_ENL != SVM_EXIT_SW)
const _: () = assert!(HV_SVM_EXITCODE_ENL == SVM_EXIT_SW);

pub unsafe fn svm_hv_inject_synthetic_vmexit_post_tlb_flush(vcpu: *mut kvm_vcpu) {
    let svm: *mut vcpu_svm = to_svm(vcpu);

    /*
     * The exit code used by Hyper-V for software-defined exits is reserved
     * by AMD specifically for such use cases.
     */
    (*(*svm).vmcb).control.exit_code = HV_SVM_EXITCODE_ENL;
    (*(*svm).vmcb).control.exit_info_1 = HV_SVM_ENL_EXITCODE_TRAP_AFTER_FLUSH;
    (*(*svm).vmcb).control.exit_info_2 = 0;
    nested_svm_vmexit(svm);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
