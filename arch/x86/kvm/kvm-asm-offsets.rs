// SPDX-License-Identifier: GPL-2.0
/*
 * Generate definitions needed by assembly language modules.
 * This code generates raw asm output which is post-processed to extract
 * and format the required data.
 */

// COMPILE_OFFSETS
// The C implementation includes linux/kbuild.h, vmx/vmx.h, and svm/svm.h.

#[allow(dead_code)]
unsafe fn common() {
    // IS_ENABLED(CONFIG_KVM_AMD)
    #[cfg(CONFIG_KVM_AMD)]
    {
        // BLANK();
        // OFFSET(SVM_vcpu_arch_regs, vcpu_svm, vcpu.arch.regs);
        // OFFSET(SVM_current_vmcb, vcpu_svm, current_vmcb);
        // OFFSET(SVM_spec_ctrl, vcpu_svm, spec_ctrl);
        // OFFSET(SVM_vmcb01, vcpu_svm, vmcb01);
        // OFFSET(KVM_VMCB_pa, kvm_vmcb_info, pa);
        // OFFSET(SD_save_area_pa, svm_cpu_data, save_area_pa);
    }

    // IS_ENABLED(CONFIG_KVM_INTEL)
    #[cfg(CONFIG_KVM_INTEL)]
    {
        // BLANK();
        // OFFSET(VMX_vcpu_arch_regs, vcpu_vmx, vcpu.arch.regs);
        // OFFSET(VMX_spec_ctrl, vcpu_vmx, spec_ctrl);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
