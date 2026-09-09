// SPDX-License-Identifier: GPL-2.0-only

// Dependency equivalent of <linux/types.h> for u32.

#[repr(C)]
pub struct qcom_adm_peripheral_config {
    pub crci: u32,
    pub mux: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
