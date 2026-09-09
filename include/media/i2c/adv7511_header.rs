/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Analog Devices ADV7511 HDMI Transmitter Device Driver
 *
 * Copyright 2013 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
 */

/* notify events */
pub const ADV7511_MONITOR_DETECT: i32 = 0;
pub const ADV7511_EDID_DETECT: i32 = 1;

#[repr(C)]
pub struct adv7511_monitor_detect {
    pub present: i32,
}

#[repr(C)]
pub struct adv7511_edid_detect {
    pub present: i32,
    pub segment: i32,
    pub phys_addr: u16,
}

#[repr(C)]
pub struct adv7511_platform_data {
    pub i2c_edid: u8,
    pub i2c_cec: u8,
    pub i2c_pktmem: u8,
    pub cec_clk: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
