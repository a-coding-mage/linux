/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding kernel translation: linux/types.h.

#[repr(C)]
pub struct qcom_rpm {
    _private: [u8; 0],
}

pub const QCOM_RPM_ACTIVE_STATE: i32 = 0;
pub const QCOM_RPM_SLEEP_STATE: i32 = 1;

extern "C" {
    pub fn qcom_rpm_write(
        rpm: *mut qcom_rpm,
        state: i32,
        resource: i32,
        buf: *mut u32,
        count: usize,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
