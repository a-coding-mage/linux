// SPDX-License-Identifier: GPL-2.0
// Dependency intent: Linux kernel headers and local VMX/KVM headers provide
// the types, constants, and helper functions referenced below.

const CC: bool = KVM_NESTED_VMENTER_CONSISTENCY_CHECK;

pub unsafe fn nested_get_evmptr(vcpu: *mut kvm_vcpu) -> u64 {
    let hv_vcpu: *mut kvm_vcpu_hv = to_hv_vcpu(vcpu);

    if unlikely(kvm_hv_get_assist_page(vcpu) != 0) {
        return EVMPTR_INVALID;
    }

    if unlikely(!(*hv_vcpu).vp_assist_page.enlighten_vmentry) {
        return EVMPTR_INVALID;
    }

    (*hv_vcpu).vp_assist_page.current_nested_vmcs
}

pub unsafe fn nested_get_evmcs_version(vcpu: *mut kvm_vcpu) -> u16 {
    // vmcs_version represents the range of supported Enlightened VMCS
    // versions: lower 8 bits is the minimal version, higher 8 bits is the
    // maximum supported version. KVM supports versions from 1 to
    // KVM_EVMCS_VERSION.
    //
    // Do not check whether Hyper-V is fully enabled in guest CPUID; this
    // helper is used to get the vCPU's supported CPUID.
    if kvm_cpu_cap_get(X86_FEATURE_VMX) != 0
        && (vcpu.is_null() || (*to_vmx(vcpu)).nested.enlightened_vmcs_enabled)
    {
        ((KVM_EVMCS_VERSION << 8) | 1) as u16
    } else {
        0
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum evmcs_revision {
    EVMCSv1_LEGACY,
    NR_EVMCS_REVISIONS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum evmcs_ctrl_type {
    EVMCS_EXIT_CTRLS,
    EVMCS_ENTRY_CTRLS,
    EVMCS_EXEC_CTRL,
    EVMCS_2NDEXEC,
    EVMCS_3RDEXEC,
    EVMCS_PINCTRL,
    EVMCS_VMFUNC,
    NR_EVMCS_CTRLS,
}

static evmcs_supported_ctrls: [[u32; NR_EVMCS_REVISIONS as usize]; NR_EVMCS_CTRLS as usize] = [
    [EVMCS1_SUPPORTED_VMEXIT_CTRL],
    [EVMCS1_SUPPORTED_VMENTRY_CTRL],
    [EVMCS1_SUPPORTED_EXEC_CTRL],
    [EVMCS1_SUPPORTED_2NDEXEC & !SECONDARY_EXEC_TSC_SCALING],
    [EVMCS1_SUPPORTED_3RDEXEC],
    [EVMCS1_SUPPORTED_PINCTRL],
    [EVMCS1_SUPPORTED_VMFUNC],
];

unsafe fn evmcs_get_supported_ctls(ctrl_type: evmcs_ctrl_type) -> u32 {
    evmcs_supported_ctrls[ctrl_type as usize][evmcs_revision::EVMCSv1_LEGACY as usize]
}

unsafe fn evmcs_has_perf_global_ctrl(vcpu: *mut kvm_vcpu) -> bool {
    let hv_vcpu: *mut kvm_vcpu_hv = to_hv_vcpu(vcpu);

    // PERF_GLOBAL_CTRL has a quirk where some Windows guests may fail to
    // boot if a PV CPUID feature flag is not also set. Treat the fields as
    // unsupported if the flag is not set in guest CPUID.
    if WARN_ON_ONCE(hv_vcpu.is_null()) {
        return false;
    }

    ((*hv_vcpu).cpuid_cache.nested_ebx & HV_X64_NESTED_EVMCS1_PERF_GLOBAL_CTRL) != 0
}

pub unsafe fn nested_evmcs_filter_control_msr(
    vcpu: *mut kvm_vcpu,
    msr_index: u32,
    pdata: *mut u64,
) {
    let mut ctl_low = *pdata as u32;
    let mut ctl_high = (*pdata >> 32) as u32;
    let mut supported_ctrls: u32;

    match msr_index {
        MSR_IA32_VMX_EXIT_CTLS | MSR_IA32_VMX_TRUE_EXIT_CTLS => {
            supported_ctrls = evmcs_get_supported_ctls(evmcs_ctrl_type::EVMCS_EXIT_CTRLS);
            if !evmcs_has_perf_global_ctrl(vcpu) {
                supported_ctrls &= !VM_EXIT_LOAD_IA32_PERF_GLOBAL_CTRL;
            }
            ctl_high &= supported_ctrls;
        }
        MSR_IA32_VMX_ENTRY_CTLS | MSR_IA32_VMX_TRUE_ENTRY_CTLS => {
            supported_ctrls = evmcs_get_supported_ctls(evmcs_ctrl_type::EVMCS_ENTRY_CTRLS);
            if !evmcs_has_perf_global_ctrl(vcpu) {
                supported_ctrls &= !VM_ENTRY_LOAD_IA32_PERF_GLOBAL_CTRL;
            }
            ctl_high &= supported_ctrls;
        }
        MSR_IA32_VMX_PROCBASED_CTLS | MSR_IA32_VMX_TRUE_PROCBASED_CTLS => {
            ctl_high &= evmcs_get_supported_ctls(evmcs_ctrl_type::EVMCS_EXEC_CTRL);
        }
        MSR_IA32_VMX_PROCBASED_CTLS2 => {
            ctl_high &= evmcs_get_supported_ctls(evmcs_ctrl_type::EVMCS_2NDEXEC);
        }
        MSR_IA32_VMX_TRUE_PINBASED_CTLS | MSR_IA32_VMX_PINBASED_CTLS => {
            ctl_high &= evmcs_get_supported_ctls(evmcs_ctrl_type::EVMCS_PINCTRL);
        }
        MSR_IA32_VMX_VMFUNC => {
            ctl_low &= evmcs_get_supported_ctls(evmcs_ctrl_type::EVMCS_VMFUNC);
        }
        _ => {}
    }

    *pdata = ctl_low as u64 | ((ctl_high as u64) << 32);
}

unsafe fn nested_evmcs_is_valid_controls(ctrl_type: evmcs_ctrl_type, val: u32) -> bool {
    (val & !evmcs_get_supported_ctls(ctrl_type)) == 0
}

pub unsafe fn nested_evmcs_check_controls(vmcs12: *mut vmcs12) -> i32 {
    if CC && !nested_evmcs_is_valid_controls(
        evmcs_ctrl_type::EVMCS_PINCTRL,
        (*vmcs12).pin_based_vm_exec_control,
    ) {
        return -EINVAL;
    }
    if CC && !nested_evmcs_is_valid_controls(
        evmcs_ctrl_type::EVMCS_EXEC_CTRL,
        (*vmcs12).cpu_based_vm_exec_control,
    ) {
        return -EINVAL;
    }
    if CC && !nested_evmcs_is_valid_controls(
        evmcs_ctrl_type::EVMCS_2NDEXEC,
        (*vmcs12).secondary_vm_exec_control,
    ) {
        return -EINVAL;
    }
    if CC && !nested_evmcs_is_valid_controls(
        evmcs_ctrl_type::EVMCS_EXIT_CTRLS,
        (*vmcs12).vm_exit_controls,
    ) {
        return -EINVAL;
    }
    if CC && !nested_evmcs_is_valid_controls(
        evmcs_ctrl_type::EVMCS_ENTRY_CTRLS,
        (*vmcs12).vm_entry_controls,
    ) {
        return -EINVAL;
    }

    // VM-Func controls are 64-bit, but KVM currently doesn't support any
    // controls in bits 63:32; dropping those bits on the consistency check
    // is intentional.
    if WARN_ON_ONCE((*vmcs12).vm_function_control >> 32 != 0) {
        return -EINVAL;
    }
    if CC && !nested_evmcs_is_valid_controls(
        evmcs_ctrl_type::EVMCS_VMFUNC,
        (*vmcs12).vm_function_control as u32,
    ) {
        return -EINVAL;
    }
    0
}

pub unsafe fn nested_enable_evmcs(vcpu: *mut kvm_vcpu, vmcs_version: *mut u16) -> i32 {
    let vmx: *mut vcpu_vmx = to_vmx(vcpu);
    (*vmx).nested.enlightened_vmcs_enabled = true;

    if !vmcs_version.is_null() {
        *vmcs_version = nested_get_evmcs_version(vcpu);
    }
    0
}

pub unsafe fn nested_evmcs_l2_tlb_flush_enabled(vcpu: *mut kvm_vcpu) -> bool {
    let hv_vcpu: *mut kvm_vcpu_hv = to_hv_vcpu(vcpu);
    let vmx: *mut vcpu_vmx = to_vmx(vcpu);
    let evmcs: *mut hv_enlightened_vmcs = (*vmx).nested.hv_evmcs;

    if hv_vcpu.is_null() || evmcs.is_null() {
        return false;
    }
    if !(*evmcs).hv_enlightenments_control.nested_flush_hypercall {
        return false;
    }
    (*hv_vcpu).vp_assist_page.nested_control.features.directhypercall
}

pub unsafe fn vmx_hv_inject_synthetic_vmexit_post_tlb_flush(vcpu: *mut kvm_vcpu) {
    nested_vmx_vmexit(vcpu, HV_VMX_SYNTHETIC_EXIT_REASON_TRAP_AFTER_FLUSH, 0, 0);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
