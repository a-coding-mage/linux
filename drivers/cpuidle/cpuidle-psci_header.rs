/* SPDX-License-Identifier: GPL-2.0 */

/* Header guard: __CPUIDLE_PSCI_H */

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct generic_pm_domain {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn psci_set_domain_state(
        pd: *mut generic_pm_domain,
        state_idx: ::core::ffi::c_uint,
        state: u32,
    );

    pub fn psci_dt_parse_state_node(np: *mut device_node, state: *mut u32) -> ::core::ffi::c_int;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
