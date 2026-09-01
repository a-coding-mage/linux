/* SPDX-License-Identifier: GPL-2.0 */

unsafe extern "C" {
    pub fn cpuidle_is_state_disabled(cpu: ::std::os::raw::c_uint, idlestate: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
    pub fn cpuidle_state_disable(
        cpu: ::std::os::raw::c_uint,
        idlestate: ::std::os::raw::c_uint,
        disable: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
    pub fn cpuidle_state_latency(
        cpu: ::std::os::raw::c_uint,
        idlestate: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_ulong;
    pub fn cpuidle_state_residency(
        cpu: ::std::os::raw::c_uint,
        idlestate: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_ulong;
    pub fn cpuidle_state_usage(
        cpu: ::std::os::raw::c_uint,
        idlestate: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_ulong;
    pub fn cpuidle_state_time(
        cpu: ::std::os::raw::c_uint,
        idlestate: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_ulonglong;
    pub fn cpuidle_state_name(
        cpu: ::std::os::raw::c_uint,
        idlestate: ::std::os::raw::c_uint,
    ) -> *mut ::std::os::raw::c_char;
    pub fn cpuidle_state_desc(
        cpu: ::std::os::raw::c_uint,
        idlestate: ::std::os::raw::c_uint,
    ) -> *mut ::std::os::raw::c_char;
    pub fn cpuidle_state_count(cpu: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint;

    pub fn cpuidle_get_governor() -> *mut ::std::os::raw::c_char;
    pub fn cpuidle_get_driver() -> *mut ::std::os::raw::c_char;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
