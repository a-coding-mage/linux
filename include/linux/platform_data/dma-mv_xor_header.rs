/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Marvell XOR platform device data definition file.
 */

// Dependency intent from <linux/dmaengine.h> and <linux/mbus.h>.

pub const MV_XOR_NAME: &str = "mv_xor";

#[repr(C)]
pub struct mv_xor_channel_data {
    pub cap_mask: dma_cap_mask_t,
}

#[repr(C)]
pub struct mv_xor_platform_data {
    pub channels: *mut mv_xor_channel_data,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
