/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright(c) 2021 Intel Corporation. All rights rsvd. */

// CONFIG_CRYPTO_DEV_IAA_CRYPTO_STATS selects the externally implemented
// statistics interface below; otherwise the inline no-op implementations are
// used.

#[cfg(feature = "CONFIG_CRYPTO_DEV_IAA_CRYPTO_STATS")]
extern "C" {
    pub fn iaa_crypto_debugfs_init() -> ::core::ffi::c_int;
    pub fn iaa_crypto_debugfs_cleanup();

    pub fn update_total_comp_calls();
    pub fn update_total_comp_bytes_out(n: ::core::ffi::c_int);
    pub fn update_total_decomp_calls();
    pub fn update_total_sw_comp_calls();
    pub fn update_total_sw_decomp_calls();
    pub fn update_total_decomp_bytes_in(n: ::core::ffi::c_int);
    pub fn update_completion_einval_errs();
    pub fn update_completion_timeout_errs();
    pub fn update_completion_comp_buf_overflow_errs();

    pub fn update_wq_comp_calls(idxd_wq: *mut idxd_wq);
    pub fn update_wq_comp_bytes(idxd_wq: *mut idxd_wq, n: ::core::ffi::c_int);
    pub fn update_wq_decomp_calls(idxd_wq: *mut idxd_wq);
    pub fn update_wq_decomp_bytes(idxd_wq: *mut idxd_wq, n: ::core::ffi::c_int);
}

#[cfg(not(feature = "CONFIG_CRYPTO_DEV_IAA_CRYPTO_STATS"))]
#[inline]
pub fn iaa_crypto_debugfs_init() -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_CRYPTO_DEV_IAA_CRYPTO_STATS"))]
#[inline]
pub fn iaa_crypto_debugfs_cleanup() {}

#[cfg(not(feature = "CONFIG_CRYPTO_DEV_IAA_CRYPTO_STATS"))]
#[inline]
pub fn update_total_comp_calls() {}

#[cfg(not(feature = "CONFIG_CRYPTO_DEV_IAA_CRYPTO_STATS"))]
#[inline]
pub fn update_total_comp_bytes_out(_n: ::core::ffi::c_int) {}

#[cfg(not(feature = "CONFIG_CRYPTO_DEV_IAA_CRYPTO_STATS"))]
#[inline]
pub fn update_total_decomp_calls() {}

#[cfg(not(feature = "CONFIG_CRYPTO_DEV_IAA_CRYPTO_STATS"))]
#[inline]
pub fn update_total_sw_comp_calls() {}

#[cfg(not(feature = "CONFIG_CRYPTO_DEV_IAA_CRYPTO_STATS"))]
#[inline]
pub fn update_total_sw_decomp_calls() {}

#[cfg(not(feature = "CONFIG_CRYPTO_DEV_IAA_CRYPTO_STATS"))]
#[inline]
pub fn update_total_decomp_bytes_in(_n: ::core::ffi::c_int) {}

#[cfg(not(feature = "CONFIG_CRYPTO_DEV_IAA_CRYPTO_STATS"))]
#[inline]
pub fn update_completion_einval_errs() {}

#[cfg(not(feature = "CONFIG_CRYPTO_DEV_IAA_CRYPTO_STATS"))]
#[inline]
pub fn update_completion_timeout_errs() {}

#[cfg(not(feature = "CONFIG_CRYPTO_DEV_IAA_CRYPTO_STATS"))]
#[inline]
pub fn update_completion_comp_buf_overflow_errs() {}

#[cfg(not(feature = "CONFIG_CRYPTO_DEV_IAA_CRYPTO_STATS"))]
#[inline]
pub fn update_wq_comp_calls(_idxd_wq: *mut idxd_wq) {}

#[cfg(not(feature = "CONFIG_CRYPTO_DEV_IAA_CRYPTO_STATS"))]
#[inline]
pub fn update_wq_comp_bytes(_idxd_wq: *mut idxd_wq, _n: ::core::ffi::c_int) {}

#[cfg(not(feature = "CONFIG_CRYPTO_DEV_IAA_CRYPTO_STATS"))]
#[inline]
pub fn update_wq_decomp_calls(_idxd_wq: *mut idxd_wq) {}

#[cfg(not(feature = "CONFIG_CRYPTO_DEV_IAA_CRYPTO_STATS"))]
#[inline]
pub fn update_wq_decomp_bytes(_idxd_wq: *mut idxd_wq, _n: ::core::ffi::c_int) {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
