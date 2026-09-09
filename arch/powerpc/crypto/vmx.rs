// SPDX-License-Identifier: GPL-2.0-only
/*
 * Routines supporting VMX instructions on the Power 8
 *
 * Copyright (C) 2015 International Business Machines Inc.
 *
 * Author: Marcelo Henrique Cerri <mhcerri@br.ibm.com>
 */

// Linux kernel and architecture headers, and "aesp8-ppc.h", provide these
// declarations and the module-registration facilities in the C source.

use core::ffi::c_void;

extern "C" {
    static mut p8_aes_cbc_alg: c_void;
    static mut p8_aes_ctr_alg: c_void;
    static mut p8_aes_xts_alg: c_void;

    fn crypto_register_skcipher(alg: *mut c_void) -> i32;
    fn crypto_unregister_skcipher(alg: *mut c_void);
}

unsafe fn p8_init() -> i32 {
    let mut ret: i32;

    ret = crypto_register_skcipher(&raw mut p8_aes_cbc_alg);
    if ret != 0 {
        return ret;
    }

    ret = crypto_register_skcipher(&raw mut p8_aes_ctr_alg);
    if ret != 0 {
        crypto_unregister_skcipher(&raw mut p8_aes_cbc_alg);
        return ret;
    }

    ret = crypto_register_skcipher(&raw mut p8_aes_xts_alg);
    if ret != 0 {
        crypto_unregister_skcipher(&raw mut p8_aes_ctr_alg);
        crypto_unregister_skcipher(&raw mut p8_aes_cbc_alg);
        return ret;
    }

    0
}

unsafe fn p8_exit() {
    crypto_unregister_skcipher(&raw mut p8_aes_xts_alg);
    crypto_unregister_skcipher(&raw mut p8_aes_ctr_alg);
    crypto_unregister_skcipher(&raw mut p8_aes_cbc_alg);
}

// module_cpu_feature_match(PPC_MODULE_FEATURE_VEC_CRYPTO, p8_init);
// module_exit(p8_exit);
// MODULE_AUTHOR("Marcelo Henrique Cerri<mhcerri@br.ibm.com>");
// MODULE_DESCRIPTION("IBM VMX cryptographic acceleration instructions support on Power 8");
// MODULE_LICENSE("GPL");
// MODULE_VERSION("1.0.0");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
