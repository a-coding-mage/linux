// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2018 IBM Corporation

// Dependencies: linux/module.h, linux/ima.h, linux/secure_boot.h

// secureboot arch rules
// Compile-time conditionals:
// - CONFIG_KEXEC_SIG: determines if KEXEC_KERNEL_CHECK appraise rule is included
// - CONFIG_MODULE_SIG: determines if MODULE_CHECK appraise rule is included
// - CONFIG_INTEGRITY_MACHINE_KEYRING && CONFIG_IMA_KEYRINGS_PERMIT_SIGNED_BY_BUILTIN_OR_SECONDARY:
//   determines POLICY_CHECK vs CRITICAL_DATA rule
// - CONFIG_IMA_ARCH_POLICY: determines if this policy is used
static SB_ARCH_RULES: &[&str] = &[
    // #if !IS_ENABLED(CONFIG_KEXEC_SIG)
    "appraise func=KEXEC_KERNEL_CHECK appraise_type=imasig",
    // #endif
    "measure func=KEXEC_KERNEL_CHECK",
    // #if !IS_ENABLED(CONFIG_MODULE_SIG)
    "appraise func=MODULE_CHECK appraise_type=imasig",
    // #endif
    // #if IS_ENABLED(CONFIG_INTEGRITY_MACHINE_KEYRING) && IS_ENABLED(CONFIG_IMA_KEYRINGS_PERMIT_SIGNED_BY_BUILTIN_OR_SECONDARY)
    "appraise func=POLICY_CHECK appraise_type=imasig",
    // #else
    "measure func=CRITICAL_DATA label=ima_policy",
    // #endif
    "measure func=MODULE_CHECK",
];

extern "C" {
    fn arch_get_secureboot() -> i32;
    fn set_module_sig_enforced();
    fn set_kexec_sig_enforced();
}

pub extern "C" fn arch_get_ima_policy() -> *const *const u8 {
    // Requires CONFIG_IMA_ARCH_POLICY to be enabled
    if cfg!(CONFIG_IMA_ARCH_POLICY) && unsafe { arch_get_secureboot() != 0 } {
        unsafe {
            set_module_sig_enforced();
            set_kexec_sig_enforced();
        }
        SB_ARCH_RULES.as_ptr() as *const *const u8
    } else {
        std::ptr::null()
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
