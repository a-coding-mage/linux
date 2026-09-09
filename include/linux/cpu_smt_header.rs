/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpuhp_smt_control {
    CPU_SMT_ENABLED,
    CPU_SMT_DISABLED,
    CPU_SMT_FORCE_DISABLED,
    CPU_SMT_NOT_SUPPORTED,
    CPU_SMT_NOT_IMPLEMENTED,
}

// Corresponds to: #if defined(CONFIG_SMP) && defined(CONFIG_HOTPLUG_SMT)
#[cfg(all(feature = "CONFIG_SMP", feature = "CONFIG_HOTPLUG_SMT"))]
extern "C" {
    pub static mut cpu_smt_control: cpuhp_smt_control;
    pub static mut cpu_smt_num_threads: core::ffi::c_uint;
    pub fn cpu_smt_disable(force: bool);
    pub fn cpu_smt_set_num_threads(
        num_threads: core::ffi::c_uint,
        max_threads: core::ffi::c_uint,
    );
    pub fn cpu_smt_possible() -> bool;
    pub fn cpuhp_smt_enable() -> core::ffi::c_int;
    pub fn cpuhp_smt_disable(ctrlval: cpuhp_smt_control) -> core::ffi::c_int;
}

// Fallback declarations corresponding to the !CONFIG_SMP || !CONFIG_HOTPLUG_SMT branch.
#[cfg(not(all(feature = "CONFIG_SMP", feature = "CONFIG_HOTPLUG_SMT")))]
pub const cpu_smt_control: cpuhp_smt_control = cpuhp_smt_control::CPU_SMT_NOT_IMPLEMENTED;

#[cfg(not(all(feature = "CONFIG_SMP", feature = "CONFIG_HOTPLUG_SMT")))]
pub const cpu_smt_num_threads: core::ffi::c_uint = 1;

#[cfg(not(all(feature = "CONFIG_SMP", feature = "CONFIG_HOTPLUG_SMT")))]
pub fn cpu_smt_disable(_force: bool) {}

#[cfg(not(all(feature = "CONFIG_SMP", feature = "CONFIG_HOTPLUG_SMT")))]
pub fn cpu_smt_set_num_threads(
    _num_threads: core::ffi::c_uint,
    _max_threads: core::ffi::c_uint,
) {
}

#[cfg(not(all(feature = "CONFIG_SMP", feature = "CONFIG_HOTPLUG_SMT")))]
pub fn cpu_smt_possible() -> bool {
    false
}

#[cfg(not(all(feature = "CONFIG_SMP", feature = "CONFIG_HOTPLUG_SMT")))]
pub fn cpuhp_smt_enable() -> core::ffi::c_int {
    0
}

#[cfg(not(all(feature = "CONFIG_SMP", feature = "CONFIG_HOTPLUG_SMT")))]
pub fn cpuhp_smt_disable(_ctrlval: cpuhp_smt_control) -> core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
