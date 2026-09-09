/* SPDX-License-Identifier: GPL-2.0 */

// Opaque declaration corresponding to `struct task_struct`.
#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[cfg(CONFIG_GENERIC_SMP_IDLE_THREAD)]
extern "C" {
    pub fn idle_thread_get(cpu: ::core::ffi::c_uint) -> *mut task_struct;
    pub fn idle_thread_set_boot_cpu();
    pub fn idle_threads_init();
}

#[cfg(not(CONFIG_GENERIC_SMP_IDLE_THREAD))]
#[inline]
pub unsafe fn idle_thread_get(_cpu: ::core::ffi::c_uint) -> *mut task_struct {
    ::core::ptr::null_mut()
}

#[cfg(not(CONFIG_GENERIC_SMP_IDLE_THREAD))]
#[inline]
pub unsafe fn idle_thread_set_boot_cpu() {}

#[cfg(not(CONFIG_GENERIC_SMP_IDLE_THREAD))]
#[inline]
pub unsafe fn idle_threads_init() {}

extern "C" {
    pub fn smpboot_create_threads(cpu: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn smpboot_park_threads(cpu: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn smpboot_unpark_threads(cpu: ::core::ffi::c_uint) -> ::core::ffi::c_int;

    // `__init` is a kernel build annotation with no direct Rust equivalent.
    pub fn cpuhp_threads_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
