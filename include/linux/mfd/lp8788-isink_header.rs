/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * TI LP8788 MFD - common definitions for current sinks
 *
 * Copyright 2012 Texas Instruments
 *
 * Author: Milo(Woogyom) Kim <milo.kim@ti.com>
 */

/* register address */
pub const LP8788_ISINK_CTRL: u8 = 0x99;
pub const LP8788_ISINK12_IOUT: u8 = 0x9A;
pub const LP8788_ISINK3_IOUT: u8 = 0x9B;
pub const LP8788_ISINK1_PWM: u8 = 0x9C;
pub const LP8788_ISINK2_PWM: u8 = 0x9D;
pub const LP8788_ISINK3_PWM: u8 = 0x9E;

/* mask bits */
pub const LP8788_ISINK1_IOUT_M: u8 = 0x0F; /* Addr 9Ah */
pub const LP8788_ISINK2_IOUT_M: u8 = 0xF0;
pub const LP8788_ISINK3_IOUT_M: u8 = 0x0F; /* Addr 9Bh */

/* 6 bits used for PWM code : Addr 9C ~ 9Eh */
pub const LP8788_ISINK_MAX_PWM: u8 = 63;
pub const LP8788_ISINK_SCALE_OFFSET: u8 = 3;

const lp8788_iout_addr: [u8; 3] = [
    LP8788_ISINK12_IOUT,
    LP8788_ISINK12_IOUT,
    LP8788_ISINK3_IOUT,
];

const lp8788_iout_mask: [u8; 3] = [
    LP8788_ISINK1_IOUT_M,
    LP8788_ISINK2_IOUT_M,
    LP8788_ISINK3_IOUT_M,
];

const lp8788_pwm_addr: [u8; 3] = [
    LP8788_ISINK1_PWM,
    LP8788_ISINK2_PWM,
    LP8788_ISINK3_PWM,
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
