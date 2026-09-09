// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2014 - 2020 Intel Corporation */

// Dependencies supplied by the surrounding kernel/driver translation.
extern "C" {
    fn adf_init_misc_wq() -> i32;
    fn adf_init_aer() -> i32;
    fn adf_init_pf_wq() -> i32;
    fn adf_init_vf_wq() -> i32;
    fn qat_crypto_register() -> i32;
    fn qat_compression_register() -> i32;
    fn qat_crypto_unregister();
    fn qat_compression_unregister();
    fn adf_exit_misc_wq();
    fn adf_exit_aer();
    fn adf_exit_pf_wq();
    fn adf_exit_vf_wq();
    fn adf_clean_vf_map(reset: bool);
}

// Linux kernel errno value for EFAULT.
const EFAULT: i32 = 14;

unsafe fn adf_register_module() -> i32 {
    if adf_init_misc_wq() != 0 {
        return -EFAULT;
    }

    if adf_init_aer() != 0 {
        adf_exit_misc_wq();
        return -EFAULT;
    }

    if adf_init_pf_wq() != 0 {
        adf_exit_aer();
        adf_exit_misc_wq();
        return -EFAULT;
    }

    if adf_init_vf_wq() != 0 {
        adf_exit_pf_wq();
        adf_exit_aer();
        adf_exit_misc_wq();
        return -EFAULT;
    }

    if qat_crypto_register() != 0 {
        adf_exit_vf_wq();
        adf_exit_pf_wq();
        adf_exit_aer();
        adf_exit_misc_wq();
        return -EFAULT;
    }

    if qat_compression_register() != 0 {
        qat_crypto_unregister();
        adf_exit_vf_wq();
        adf_exit_pf_wq();
        adf_exit_aer();
        adf_exit_misc_wq();
        return -EFAULT;
    }

    return 0;
}

unsafe fn adf_unregister_module() {
    adf_exit_misc_wq();
    adf_exit_aer();
    adf_exit_vf_wq();
    adf_exit_pf_wq();
    qat_crypto_unregister();
    qat_compression_unregister();
    adf_clean_vf_map(false);
}

// module_init(adf_register_module);
// module_exit(adf_unregister_module);
// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_AUTHOR("Intel");
// MODULE_DESCRIPTION("Intel(R) QuickAssist Technology");
// MODULE_ALIAS_CRYPTO("intel_qat");
// MODULE_IMPORT_NS("CRYPTO_INTERNAL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
