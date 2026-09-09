// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel translation unit:
// capabilities.h, vmx_onhyperv.h

// Corresponds to DEFINE_STATIC_KEY_FALSE(__kvm_is_using_evmcs).
extern "C" {
    pub static mut __kvm_is_using_evmcs: static_key_false;
}

/*
 * KVM on Hyper-V always uses the latest known eVMCSv1 revision, the assumption
 * is: in case a feature has corresponding fields in eVMCS described and it was
 * exposed in VMX feature MSRs, KVM is free to use it. Warn if KVM meets a
 * feature which has no corresponding eVMCS field, this likely means that KVM
 * needs to be updated.
 */
macro_rules! evmcs_check_vmcs_conf {
    ($vmcs_conf:expr, $field:ident, $supported:expr) => {{
        let unsupported = unsafe {
            (*$vmcs_conf).$field & !$supported
        };
        if unsupported != 0 {
            pr_warn_once!(
                concat!(stringify!($field), " unsupported with eVMCS: 0x%llx\n"),
                unsupported as u64
            );
            unsafe {
                (*$vmcs_conf).$field &= $supported;
            }
        }
    }};
}

pub unsafe fn evmcs_sanitize_exec_ctrls(vmcs_conf: *mut vmcs_config) {
    evmcs_check_vmcs_conf!(vmcs_conf, cpu_based_exec_ctrl, EVMCS1_SUPPORTED_EXEC_CTRL);
    evmcs_check_vmcs_conf!(vmcs_conf, pin_based_exec_ctrl, EVMCS1_SUPPORTED_PINCTRL);
    evmcs_check_vmcs_conf!(vmcs_conf, cpu_based_2nd_exec_ctrl, EVMCS1_SUPPORTED_2NDEXEC);
    evmcs_check_vmcs_conf!(vmcs_conf, cpu_based_3rd_exec_ctrl, EVMCS1_SUPPORTED_3RDEXEC);
    evmcs_check_vmcs_conf!(vmcs_conf, vmentry_ctrl, EVMCS1_SUPPORTED_VMENTRY_CTRL);
    evmcs_check_vmcs_conf!(vmcs_conf, vmexit_ctrl, EVMCS1_SUPPORTED_VMEXIT_CTRL);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
