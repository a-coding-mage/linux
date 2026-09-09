/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct generic_pm_domain {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

// CONFIG_DT_IDLE_GENPD
#[cfg(feature = "CONFIG_DT_IDLE_GENPD")]
extern "C" {
    pub fn dt_idle_pd_free(pd: *mut generic_pm_domain);

    pub fn dt_idle_pd_alloc(
        np: *mut device_node,
        parse_state: Option<unsafe extern "C" fn(*mut device_node, *mut u32) -> i32>,
    ) -> *mut generic_pm_domain;

    pub fn dt_idle_pd_init_topology(np: *mut device_node) -> i32;

    pub fn dt_idle_pd_remove_topology(np: *mut device_node) -> i32;

    pub fn dt_idle_attach_cpu(cpu: i32, name: *const core::ffi::c_char) -> *mut device;

    pub fn dt_idle_detach_cpu(dev: *mut device);
}

#[cfg(not(feature = "CONFIG_DT_IDLE_GENPD"))]
#[inline]
pub unsafe fn dt_idle_pd_free(_pd: *mut generic_pm_domain) {}

#[cfg(not(feature = "CONFIG_DT_IDLE_GENPD"))]
#[inline]
pub unsafe fn dt_idle_pd_alloc(
    _np: *mut device_node,
    _parse_state: Option<unsafe extern "C" fn(*mut device_node, *mut u32) -> i32>,
) -> *mut generic_pm_domain {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_DT_IDLE_GENPD"))]
#[inline]
pub unsafe fn dt_idle_pd_init_topology(_np: *mut device_node) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_DT_IDLE_GENPD"))]
#[inline]
pub unsafe fn dt_idle_pd_remove_topology(_np: *mut device_node) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_DT_IDLE_GENPD"))]
#[inline]
pub unsafe fn dt_idle_attach_cpu(_cpu: i32, _name: *const core::ffi::c_char) -> *mut device {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_DT_IDLE_GENPD"))]
#[inline]
pub unsafe fn dt_idle_detach_cpu(_dev: *mut device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
