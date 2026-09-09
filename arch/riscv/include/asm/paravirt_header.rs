/* SPDX-License-Identifier: GPL-2.0 */

/* CONFIG_PARAVIRT */
#[cfg(CONFIG_PARAVIRT)]
extern "C" {
    pub fn pv_time_init() -> ::core::ffi::c_int;
}

#[cfg(not(CONFIG_PARAVIRT))]
#[inline]
pub fn pv_time_init() {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
