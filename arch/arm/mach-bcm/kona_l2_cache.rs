// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2012-2014 Broadcom Corporation

// Dependencies supplied by the surrounding kernel translation:
// bcm_kona_smc_init, bcm_kona_smc, l2x0_of_init, SSAPI_ENABLE_L2_CACHE,
// SEC_ROM_RET_OK, pr_info, and pr_err.

extern "C" {
    fn bcm_kona_smc_init() -> ::core::ffi::c_int;
    fn bcm_kona_smc(
        api: ::core::ffi::c_uint,
        arg1: ::core::ffi::c_uint,
        arg2: ::core::ffi::c_uint,
        arg3: ::core::ffi::c_uint,
        arg4: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_uint;
    fn l2x0_of_init(aux_val: ::core::ffi::c_uint, aux_mask: ::core::ffi::c_uint)
        -> ::core::ffi::c_int;
}

extern "C" {
    static SSAPI_ENABLE_L2_CACHE: ::core::ffi::c_uint;
    static SEC_ROM_RET_OK: ::core::ffi::c_uint;
}

// The kernel's __init annotation has no direct Rust equivalent.
pub unsafe extern "C" fn kona_l2_cache_init() {
    let result: ::core::ffi::c_uint;
    let ret: ::core::ffi::c_int;

    ret = bcm_kona_smc_init();
    if ret != 0 {
        pr_info(
            b"Secure API not available (%d). Skipping L2 init.\n\0".as_ptr(),
            ret,
        );
        return;
    }

    result = bcm_kona_smc(
        SSAPI_ENABLE_L2_CACHE,
        0,
        0,
        0,
        0,
    );
    if result != SEC_ROM_RET_OK {
        pr_err(
            b"Secure Monitor call failed (%u)! Skipping L2 init.\n\0".as_ptr(),
            result,
        );
        return;
    }

    /*
     * The aux_val and aux_mask have no effect since L2 cache is already
     * enabled.  Pass 0s for aux_val and 1s for aux_mask for default value.
     */
    ret = l2x0_of_init(0, !0);
    if ret != 0 {
        pr_err(
            b"Couldn't enable L2 cache: %d\n\0".as_ptr(),
            ret,
        );
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
