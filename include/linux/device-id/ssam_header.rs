/* SPDX-License-Identifier: GPL-2.0 */

/* Surface System Aggregator Module */

pub const SSAM_MATCH_TARGET: u32 = 0x1;
pub const SSAM_MATCH_INSTANCE: u32 = 0x2;
pub const SSAM_MATCH_FUNCTION: u32 = 0x4;

#[repr(C)]
pub struct ssam_device_id {
    pub match_flags: u8,

    pub domain: u8,
    pub category: u8,
    pub target: u8,
    pub instance: u8,
    pub function: u8,

    pub driver_data: usize,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
