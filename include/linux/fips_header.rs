/* SPDX-License-Identifier: GPL-2.0 */

/* CONFIG_CRYPTO_FIPS */
#[cfg(feature = "CONFIG_CRYPTO_FIPS")]
extern "C" {
    pub static mut fips_enabled: ::core::ffi::c_int;
    pub static mut fips_fail_notif_chain: atomic_notifier_head;

    pub fn fips_fail_notify();
}

#[cfg(not(feature = "CONFIG_CRYPTO_FIPS"))]
pub const fips_enabled: ::core::ffi::c_int = 0;

#[cfg(not(feature = "CONFIG_CRYPTO_FIPS"))]
#[inline]
pub fn fips_fail_notify() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
