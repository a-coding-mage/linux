// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2018 IBM Corporation

// Dependencies: linux/module.h, linux/ima.h, linux/secure_boot.h

// secureboot arch rules
// Build-time conditionals:
// - !CONFIG_KEXEC_SIG includes the KEXEC_KERNEL_CHECK appraise rule
// - !CONFIG_MODULE_SIG includes the MODULE_CHECK appraise rule
// - CONFIG_INTEGRITY_MACHINE_KEYRING && CONFIG_IMA_KEYRINGS_PERMIT_SIGNED_BY_BUILTIN_OR_SECONDARY
//   selects POLICY_CHECK; otherwise CRITICAL_DATA is selected
static SB_ARCH_RULES: [*const u8; 7] = [
    // #if !IS_ENABLED(CONFIG_KEXEC_SIG)
    b"appraise func=KEXEC_KERNEL_CHECK appraise_type=imasig\0".as_ptr(),
    // #endif
    b"measure func=KEXEC_KERNEL_CHECK\0".as_ptr(),
    // #if !IS_ENABLED(CONFIG_MODULE_SIG)
    b"appraise func=MODULE_CHECK appraise_type=imasig\0".as_ptr(),
    // #endif
    // #if IS_ENABLED(CONFIG_INTEGRITY_MACHINE_KEYRING) && IS_ENABLED(CONFIG_IMA_KEYRINGS_PERMIT_SIGNED_BY_BUILTIN_OR_SECONDARY)
    b"appraise func=POLICY_CHECK appraise_type=imasig\0".as_ptr(),
    // #else
    b"measure func=CRITICAL_DATA label=ima_policy\0".as_ptr(),
    // #endif
    b"measure func=MODULE_CHECK\0".as_ptr(),
    core::ptr::null(),
];

extern "C" {
    fn arch_get_secureboot() -> i32;
    fn set_module_sig_enforced();
    fn set_kexec_sig_enforced();
}

pub extern "C" fn arch_get_ima_policy() -> *const *const u8 {
    // Requires CONFIG_IMA_ARCH_POLICY to be enabled.
    if cfg!(CONFIG_IMA_ARCH_POLICY) && unsafe { arch_get_secureboot() != 0 } {
        unsafe {
            set_module_sig_enforced();
            set_kexec_sig_enforced();
        }
        SB_ARCH_RULES.as_ptr()
    } else {
        core::ptr::null()
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
