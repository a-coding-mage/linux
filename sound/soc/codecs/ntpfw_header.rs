/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ntpfw.h - Firmware helper functions for Neofidelity codecs
 *
 * Copyright (c) 2024, SaluteDevices. All Rights Reserved.
 */

// C dependencies: <linux/i2c.h>, <linux/firmware.h>

use core::ffi::{c_char, c_int};

extern "C" {
    /**
     * ntpfw_load - load firmware to amplifier over i2c interface.
     *
     * @i2c:	Pointer to amplifier's I2C client.
     * @name:	Firmware file name.
     * @magic:	Magic number to validate firmware.
     *
     * Returns:	0 or error code upon error.
     */
    pub fn ntpfw_load(i2c: *mut i2c_client, name: *const c_char, magic: u32) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
