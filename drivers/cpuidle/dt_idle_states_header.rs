/* SPDX-License-Identifier: GPL-2.0 */

// The following opaque types are supplied by the corresponding dependencies.
#[repr(C)]
pub struct cpuidle_driver {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_device_id {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn dt_init_idle_driver(
        drv: *mut cpuidle_driver,
        matches: *const of_device_id,
        start_idx: u32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
