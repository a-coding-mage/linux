/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  linux/include/linux/cpu_cooling.h
 *
 *  Copyright (C) 2012 Samsung Electronics Co., Ltd(http://www.samsung.com)
 *  Copyright (C) 2012 Amit Daniel <amit.kachhap@linaro.org>
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */

// Dependencies supplied by other translation units: linux/of.h, linux/thermal.h.

#[repr(C)]
pub struct cpufreq_policy {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thermal_cooling_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpuidle_driver {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_CPU_FREQ_THERMAL")]
extern "C" {
    /// cpufreq_cooling_register - function to create cpufreq cooling device.
    /// @policy: cpufreq policy.
    pub fn cpufreq_cooling_register(
        policy: *mut cpufreq_policy,
    ) -> *mut thermal_cooling_device;

    /// cpufreq_cooling_unregister - function to remove cpufreq cooling device.
    /// @cdev: thermal cooling device pointer.
    pub fn cpufreq_cooling_unregister(cdev: *mut thermal_cooling_device);

    /// of_cpufreq_cooling_register - create cpufreq cooling device based on DT.
    /// @policy: cpufreq policy.
    pub fn of_cpufreq_cooling_register(
        policy: *mut cpufreq_policy,
    ) -> *mut thermal_cooling_device;
}

#[cfg(not(feature = "CONFIG_CPU_FREQ_THERMAL"))]
#[inline]
pub unsafe fn cpufreq_cooling_register(
    _policy: *mut cpufreq_policy,
) -> *mut thermal_cooling_device {
    // C equivalent: ERR_PTR(-ENOSYS). ERR_PTR is supplied by another dependency.
    (-38isize) as *mut thermal_cooling_device
}

#[cfg(not(feature = "CONFIG_CPU_FREQ_THERMAL"))]
#[inline]
pub unsafe fn cpufreq_cooling_unregister(_cdev: *mut thermal_cooling_device) {}

#[cfg(not(feature = "CONFIG_CPU_FREQ_THERMAL"))]
#[inline]
pub unsafe fn of_cpufreq_cooling_register(
    _policy: *mut cpufreq_policy,
) -> *mut thermal_cooling_device {
    core::ptr::null_mut()
}

#[cfg(feature = "CONFIG_CPU_IDLE_THERMAL")]
extern "C" {
    pub fn cpuidle_cooling_register(drv: *mut cpuidle_driver);
}

#[cfg(not(feature = "CONFIG_CPU_IDLE_THERMAL"))]
#[inline]
pub unsafe fn cpuidle_cooling_register(_drv: *mut cpuidle_driver) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
