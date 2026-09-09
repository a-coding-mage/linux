/* SPDX-License-Identifier: GPL-2.0 */

/**
 * struct mips_cdmm_device_id - identifies devices in MIPS CDMM bus
 * @type:\tDevice type identifier.
 */
#[repr(C)]
pub struct mips_cdmm_device_id {
    pub r#type: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
