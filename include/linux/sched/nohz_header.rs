/* SPDX-License-Identifier: GPL-2.0 */

/*
 * This is the interface between the scheduler and nohz/dynticks:
 *
 * The CONFIG_NO_HZ_COMMON conditionals mirror the source build-time
 * configuration.  `rq` is supplied by the scheduler dependency.
 */

#[cfg(feature = "CONFIG_NO_HZ_COMMON")]
unsafe extern "C" {
    pub fn nohz_balance_enter_idle(cpu: ::core::ffi::c_int);
    pub fn get_nohz_timer_target() -> ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_NO_HZ_COMMON"))]
#[inline]
pub fn nohz_balance_enter_idle(_cpu: ::core::ffi::c_int) {}

#[cfg(feature = "CONFIG_NO_HZ_COMMON")]
unsafe extern "C" {
    pub fn calc_load_nohz_start();
    pub fn calc_load_nohz_remote(rq: *mut rq);
    pub fn calc_load_nohz_stop();
}

#[cfg(not(feature = "CONFIG_NO_HZ_COMMON"))]
#[inline]
pub fn calc_load_nohz_start() {}

#[cfg(not(feature = "CONFIG_NO_HZ_COMMON"))]
#[inline]
pub fn calc_load_nohz_remote(_rq: *mut rq) {}

#[cfg(not(feature = "CONFIG_NO_HZ_COMMON"))]
#[inline]
pub fn calc_load_nohz_stop() {}

#[cfg(feature = "CONFIG_NO_HZ_COMMON")]
unsafe extern "C" {
    pub fn wake_up_nohz_cpu(cpu: ::core::ffi::c_int);
}

#[cfg(not(feature = "CONFIG_NO_HZ_COMMON"))]
#[inline]
pub fn wake_up_nohz_cpu(_cpu: ::core::ffi::c_int) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
