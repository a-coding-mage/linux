/* SPDX-License-Identifier: GPL-2.0 */
/*
 * coupler.h -- SoC Regulator support, coupler API.
 *
 * Regulator Coupler Interface.
 */

/* C dependencies: linux/errno.h, linux/types.h, and linux/suspend.h. */

#[repr(C)]
pub struct list_head;

pub type suspend_state_t = i32;

#[repr(C)]
pub struct regulator_coupler {
    pub list: list_head,

    pub attach_regulator: Option<unsafe extern "C" fn(
        coupler: *mut regulator_coupler,
        rdev: *mut regulator_dev,
    ) -> i32>,
    pub detach_regulator: Option<unsafe extern "C" fn(
        coupler: *mut regulator_coupler,
        rdev: *mut regulator_dev,
    ) -> i32>,
    pub balance_voltage: Option<unsafe extern "C" fn(
        coupler: *mut regulator_coupler,
        rdev: *mut regulator_dev,
        state: suspend_state_t,
    ) -> i32>,
}

#[repr(C)]
pub struct regulator_dev;

/* CONFIG_REGULATOR selects the external implementations below. */
#[cfg(feature = "CONFIG_REGULATOR")]
extern "C" {
    pub fn regulator_coupler_register(coupler: *mut regulator_coupler) -> i32;
    pub fn regulator_check_consumers(
        rdev: *mut regulator_dev,
        min_uV: *mut i32,
        max_uV: *mut i32,
        state: suspend_state_t,
    ) -> i32;
    pub fn regulator_check_voltage(
        rdev: *mut regulator_dev,
        min_uV: *mut i32,
        max_uV: *mut i32,
    ) -> i32;
    pub fn regulator_get_voltage_rdev(rdev: *mut regulator_dev) -> i32;
    pub fn regulator_set_voltage_rdev(
        rdev: *mut regulator_dev,
        min_uV: i32,
        max_uV: i32,
        state: suspend_state_t,
    ) -> i32;
    pub fn regulator_do_balance_voltage(
        rdev: *mut regulator_dev,
        state: suspend_state_t,
        skip_coupled: bool,
    ) -> i32;
}

#[cfg(not(feature = "CONFIG_REGULATOR"))]
#[inline]
pub unsafe fn regulator_coupler_register(_coupler: *mut regulator_coupler) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_REGULATOR"))]
#[inline]
pub unsafe fn regulator_check_consumers(
    _rdev: *mut regulator_dev,
    _min_uV: *mut i32,
    _max_uV: *mut i32,
    _state: suspend_state_t,
) -> i32 {
    -22 /* -EINVAL */
}

#[cfg(not(feature = "CONFIG_REGULATOR"))]
#[inline]
pub unsafe fn regulator_check_voltage(
    _rdev: *mut regulator_dev,
    _min_uV: *mut i32,
    _max_uV: *mut i32,
) -> i32 {
    -22 /* -EINVAL */
}

#[cfg(not(feature = "CONFIG_REGULATOR"))]
#[inline]
pub unsafe fn regulator_get_voltage_rdev(_rdev: *mut regulator_dev) -> i32 {
    -22 /* -EINVAL */
}

#[cfg(not(feature = "CONFIG_REGULATOR"))]
#[inline]
pub unsafe fn regulator_set_voltage_rdev(
    _rdev: *mut regulator_dev,
    _min_uV: i32,
    _max_uV: i32,
    _state: suspend_state_t,
) -> i32 {
    -22 /* -EINVAL */
}

#[cfg(not(feature = "CONFIG_REGULATOR"))]
#[inline]
pub unsafe fn regulator_do_balance_voltage(
    _rdev: *mut regulator_dev,
    _state: suspend_state_t,
    _skip_coupled: bool,
) -> i32 {
    -22 /* -EINVAL */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
