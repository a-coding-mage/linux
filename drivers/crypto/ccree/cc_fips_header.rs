/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2012-2019 ARM Limited (or its affiliates). */

/* CONFIG_CRYPTO_FIPS is represented here by the corresponding Rust feature. */

#[cfg(feature = "CONFIG_CRYPTO_FIPS")]
#[repr(C)]
pub enum cc_fips_status {
    CC_FIPS_SYNC_MODULE_OK = 0x0,
    CC_FIPS_SYNC_MODULE_ERROR = 0x1,
    CC_FIPS_SYNC_REE_STATUS = 0x4,
    CC_FIPS_SYNC_TEE_STATUS = 0x8,
    CC_FIPS_SYNC_STATUS_RESERVE32B = i32::MAX,
}

#[cfg(feature = "CONFIG_CRYPTO_FIPS")]
extern "C" {
    pub fn cc_fips_init(p_drvdata: *mut cc_drvdata) -> ::std::os::raw::c_int;
    pub fn cc_fips_fini(drvdata: *mut cc_drvdata);
    pub fn fips_handler(drvdata: *mut cc_drvdata);
    pub fn cc_set_ree_fips_status(drvdata: *mut cc_drvdata, ok: bool);
    pub fn cc_tee_handle_fips_error(p_drvdata: *mut cc_drvdata);
}

#[cfg(not(feature = "CONFIG_CRYPTO_FIPS"))]
#[inline]
pub unsafe fn cc_fips_init(_p_drvdata: *mut cc_drvdata) -> ::std::os::raw::c_int {
    0
}

#[cfg(not(feature = "CONFIG_CRYPTO_FIPS"))]
#[inline]
pub unsafe fn cc_fips_fini(_drvdata: *mut cc_drvdata) {}

#[cfg(not(feature = "CONFIG_CRYPTO_FIPS"))]
#[inline]
pub unsafe fn cc_set_ree_fips_status(_drvdata: *mut cc_drvdata, _ok: bool) {}

#[cfg(not(feature = "CONFIG_CRYPTO_FIPS"))]
#[inline]
pub unsafe fn fips_handler(_drvdata: *mut cc_drvdata) {}

#[cfg(not(feature = "CONFIG_CRYPTO_FIPS"))]
#[inline]
pub unsafe fn cc_tee_handle_fips_error(_p_drvdata: *mut cc_drvdata) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
