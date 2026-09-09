/* SPDX-License-Identifier: GPL-2.0 */
/*
 * cpuidle.h - The internal header file
 */

use core::ffi::{c_char, c_int, c_void};

/* C header guard: __DRIVER_CPUIDLE_H */

/* Opaque types supplied by other headers/dependencies. */
#[repr(C)]
pub struct cpuidle_governor { _private: [u8; 0] }
#[repr(C)]
pub struct list_head { _private: [u8; 0] }
#[repr(C)]
pub struct mutex { _private: [u8; 0] }
#[repr(C)]
pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)]
pub struct cpuidle_device { _private: [u8; 0] }
#[repr(C)]
pub struct cpuidle_driver { _private: [u8; 0] }
#[repr(C)]
pub struct device { _private: [u8; 0] }

/* For internal use only */
extern "C" {
    pub static mut param_governor: [c_char; 0];
    pub static mut cpuidle_curr_governor: *mut cpuidle_governor;
    pub static mut cpuidle_prev_governor: *mut cpuidle_governor;
    pub static mut cpuidle_governors: list_head;
    pub static mut cpuidle_detected_devices: list_head;
    pub static mut cpuidle_lock: mutex;
    pub static mut cpuidle_driver_lock: spinlock_t;
    pub fn cpuidle_disabled() -> c_int;
    pub fn cpuidle_enter_state(
        dev: *mut cpuidle_device,
        drv: *mut cpuidle_driver,
        next_state: c_int,
    ) -> c_int;

    /* idle loop */
    pub fn cpuidle_install_idle_handler();
    pub fn cpuidle_uninstall_idle_handler();

    /* governors */
    pub fn cpuidle_find_governor(str_: *const c_char) -> *mut cpuidle_governor;
    pub fn cpuidle_switch_governor(gov: *mut cpuidle_governor) -> c_int;

    /* sysfs */
    pub fn cpuidle_add_interface() -> c_int;
    pub fn cpuidle_remove_interface(dev: *mut device);
    pub fn cpuidle_add_device_sysfs(device: *mut cpuidle_device) -> c_int;
    pub fn cpuidle_remove_device_sysfs(device: *mut cpuidle_device);
    pub fn cpuidle_add_sysfs(dev: *mut cpuidle_device) -> c_int;
    pub fn cpuidle_remove_sysfs(dev: *mut cpuidle_device);
}

/* CONFIG_ARCH_NEEDS_CPU_IDLE_COUPLED selects the external implementations. */
#[cfg(CONFIG_ARCH_NEEDS_CPU_IDLE_COUPLED)]
extern "C" {
    pub fn cpuidle_state_is_coupled(drv: *mut cpuidle_driver, state: c_int) -> bool;
    pub fn cpuidle_coupled_state_verify(drv: *mut cpuidle_driver) -> c_int;
    pub fn cpuidle_enter_state_coupled(
        dev: *mut cpuidle_device,
        drv: *mut cpuidle_driver,
        next_state: c_int,
    ) -> c_int;
    pub fn cpuidle_coupled_register_device(dev: *mut cpuidle_device) -> c_int;
    pub fn cpuidle_coupled_unregister_device(dev: *mut cpuidle_device);
}

#[cfg(not(CONFIG_ARCH_NEEDS_CPU_IDLE_COUPLED))]
#[inline]
pub unsafe fn cpuidle_state_is_coupled(_drv: *mut cpuidle_driver, _state: c_int) -> bool {
    false
}

#[cfg(not(CONFIG_ARCH_NEEDS_CPU_IDLE_COUPLED))]
#[inline]
pub unsafe fn cpuidle_coupled_state_verify(_drv: *mut cpuidle_driver) -> c_int {
    0
}

#[cfg(not(CONFIG_ARCH_NEEDS_CPU_IDLE_COUPLED))]
#[inline]
pub unsafe fn cpuidle_enter_state_coupled(
    _dev: *mut cpuidle_device,
    _drv: *mut cpuidle_driver,
    _next_state: c_int,
) -> c_int {
    -1
}

#[cfg(not(CONFIG_ARCH_NEEDS_CPU_IDLE_COUPLED))]
#[inline]
pub unsafe fn cpuidle_coupled_register_device(_dev: *mut cpuidle_device) -> c_int {
    0
}

#[cfg(not(CONFIG_ARCH_NEEDS_CPU_IDLE_COUPLED))]
#[inline]
pub unsafe fn cpuidle_coupled_unregister_device(_dev: *mut cpuidle_device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
