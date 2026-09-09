/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * i2c.h - i.MX I2C driver header file
 *
 * Copyright (c) 2008, Darius Augulis <augulis.darius@gmail.com>
 */

/* C header guard: __ASM_ARCH_I2C_H_ */

/**
 * struct imxi2c_platform_data - structure of platform data for MXC I2C driver
 * @bitrate: Bus speed measured in Hz
 */
#[repr(C)]
pub struct imxi2c_platform_data {
    pub bitrate: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
