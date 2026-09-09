/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Microchip SAMA7 SFRBU registers offsets and bit definitions.
 *
 * Copyright (C) [2020] Microchip Technology Inc. and its subsidiaries
 *
 * Author: Claudu Beznea <claudiu.beznea@microchip.com>
 */

/* C source condition: CONFIG_SOC_SAMA7 */
#[cfg(feature = "CONFIG_SOC_SAMA7")]
pub const AT91_SFRBU_PSWBU: u32 = 0x00; /* SFRBU Power Switch BU Control Register */
#[cfg(feature = "CONFIG_SOC_SAMA7")]
pub const AT91_SFRBU_PSWBU_PSWKEY: u32 = 0x4BD20C << 8; /* Specific value mandatory to allow writing of other register bits */
#[cfg(feature = "CONFIG_SOC_SAMA7")]
pub const AT91_SFRBU_PSWBU_STATE: u32 = 1 << 2; /* Power switch BU state */
#[cfg(feature = "CONFIG_SOC_SAMA7")]
pub const AT91_SFRBU_PSWBU_SOFTSWITCH: u32 = 1 << 1; /* Power switch BU source selection */
#[cfg(feature = "CONFIG_SOC_SAMA7")]
pub const AT91_SFRBU_PSWBU_CTRL: u32 = 1 << 0; /* Power switch BU control */

#[cfg(feature = "CONFIG_SOC_SAMA7")]
pub const AT91_FRBU_DDRPWR: u32 = 0x10; /* SFRBU DDR Power Control Register */
#[cfg(feature = "CONFIG_SOC_SAMA7")]
pub const AT91_FRBU_DDRPWR_STATE: u32 = 1 << 0; /* DDR Power Mode State */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
