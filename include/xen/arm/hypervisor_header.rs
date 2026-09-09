/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency supplied by the surrounding Xen/Linux translation. */
#[repr(C)]
pub struct shared_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct start_info {
    _private: [u8; 0],
}

extern "C" {
    pub static mut HYPERVISOR_shared_info: *mut shared_info;
    pub static mut xen_start_info: *mut start_info;
}

/* C __init annotation has no direct Rust equivalent. */
#[cfg(CONFIG_XEN)]
extern "C" {
    pub fn xen_early_init();
}

#[cfg(not(CONFIG_XEN))]
#[inline]
pub fn xen_early_init() {
    return;
}

#[cfg(CONFIG_HOTPLUG_CPU)]
#[inline]
pub fn xen_arch_register_cpu(_num: i32) {
}

#[cfg(CONFIG_HOTPLUG_CPU)]
#[inline]
pub fn xen_arch_unregister_cpu(_num: i32) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
