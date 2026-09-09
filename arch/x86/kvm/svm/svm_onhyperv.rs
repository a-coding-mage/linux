// SPDX-License-Identifier: GPL-2.0-only
/*
 * KVM L1 hypervisor optimizations on Hyper-V for SVM.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/kvm_host.h, asm/mshyperv.h, svm.h, svm_ops.h, hyperv.h,
// kvm_onhyperv.h, and svm_onhyperv.h.

unsafe fn svm_hv_enable_l2_tlb_flush(vcpu: *mut kvm_vcpu) -> i32 {
    let hve: *mut hv_vmcb_enlightenments;
    let partition_assist_page: hpa_t = hv_get_partition_assist_page(vcpu);

    if partition_assist_page == INVALID_PAGE {
        return -ENOMEM;
    }

    hve = &mut (*(*to_svm(vcpu)).vmcb).control.hv_enlightenments;

    (*hve).partition_assist_page = partition_assist_page;
    (*hve).hv_vm_id = vcpu as usize as _;
    if (*hve).hv_enlightenments_control.nested_flush_hypercall == 0 {
        (*hve).hv_enlightenments_control.nested_flush_hypercall = 1;
        vmcb_mark_dirty((*to_svm(vcpu)).vmcb, HV_VMCB_NESTED_ENLIGHTENMENTS);
    }

    0
}

// __init
unsafe fn svm_hv_hardware_setup() {
    if npt_enabled && (ms_hyperv.nested_features & HV_X64_NESTED_ENLIGHTENED_TLB) != 0 {
        pr_info!("{}: Hyper-V enlightened NPT TLB flush enabled\n", KBUILD_MODNAME);
        svm_x86_ops.flush_remote_tlbs = Some(hv_flush_remote_tlbs);
        svm_x86_ops.flush_remote_tlbs_range = Some(hv_flush_remote_tlbs_range);
    }

    if (ms_hyperv.nested_features & HV_X64_NESTED_DIRECT_FLUSH) != 0 {
        let mut cpu: i32;

        pr_info!("{}: Hyper-V Direct TLB Flush enabled\n", KBUILD_MODNAME);
        for_each_online_cpu!(cpu, {
            let vp_ap: *mut hv_vp_assist_page = hv_get_vp_assist_page(cpu);

            if vp_ap.is_null() {
                continue;
            }

            (*vp_ap).nested_control.features.directhypercall = 1;
        });
        svm_x86_ops.enable_l2_tlb_flush = Some(svm_hv_enable_l2_tlb_flush);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
