/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ALSA SoC TPA6130A2 amplifier driver
 *
 * Copyright (C) Nokia Corporation
 *
 * Author: Peter Ujfalusi <peter.ujfalusi@ti.com>
 */

/* Register addresses */
pub const TPA6130A2_REG_CONTROL: u32 = 0x01;
pub const TPA6130A2_REG_VOL_MUTE: u32 = 0x02;
pub const TPA6130A2_REG_OUT_IMPEDANCE: u32 = 0x03;
pub const TPA6130A2_REG_VERSION: u32 = 0x04;

/* Register bits */
/* TPA6130A2_REG_CONTROL (0x01) */
pub const TPA6130A2_SWS_SHIFT: u32 = 0;
pub const TPA6130A2_SWS: u32 = 0x01 << TPA6130A2_SWS_SHIFT;
pub const TPA6130A2_TERMAL: u32 = 0x01 << 1;
pub const fn TPA6130A2_MODE(x: u32) -> u32 {
    x << 4
}
pub const TPA6130A2_MODE_STEREO: u32 = 0x00;
pub const TPA6130A2_MODE_DUAL_MONO: u32 = 0x01;
pub const TPA6130A2_MODE_BRIDGE: u32 = 0x02;
pub const TPA6130A2_MODE_MASK: u32 = 0x03;
pub const TPA6130A2_HP_EN_R_SHIFT: u32 = 6;
pub const TPA6130A2_HP_EN_R: u32 = 0x01 << TPA6130A2_HP_EN_R_SHIFT;
pub const TPA6130A2_HP_EN_L_SHIFT: u32 = 7;
pub const TPA6130A2_HP_EN_L: u32 = 0x01 << TPA6130A2_HP_EN_L_SHIFT;

/* TPA6130A2_REG_VOL_MUTE (0x02) */
pub const fn TPA6130A2_VOLUME(x: u32) -> u32 {
    (x & 0x3f) << 0
}
pub const TPA6130A2_MUTE_R: u32 = 0x01 << 6;
pub const TPA6130A2_MUTE_L: u32 = 0x01 << 7;

/* TPA6130A2_REG_OUT_IMPEDANCE (0x03) */
pub const TPA6130A2_HIZ_R: u32 = 0x01 << 0;
pub const TPA6130A2_HIZ_L: u32 = 0x01 << 1;

/* TPA6130A2_REG_VERSION (0x04) */
pub const TPA6130A2_VERSION_MASK: u32 = 0x0f;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
